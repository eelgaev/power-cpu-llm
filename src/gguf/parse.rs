//! GGUF metadata parsing in Rust, accepting exactly the files the pinned ggml
//! reader (`vendor/ggml` 353b63b, `gguf_init_from_reader`) accepts and
//! returning the same values. The `reference-oracle` tests compare both on
//! real models and on mutated files.

use crate::gguf::{ArrayValues, Error, MetadataValue, TensorInfo};
use std::collections::{BTreeMap, HashSet};
use std::io::{BufReader, Read, Seek, SeekFrom};

const MAX_STRING: u64 = 1 << 30;
const MAX_ARRAY: u64 = 1 << 30;
const MAX_NAME: usize = 64;
const MAX_DIMS: u32 = 4;
const VERSION: u32 = 3;
const DEFAULT_ALIGNMENT: u64 = 32;

/// ggml's type table: (name, elements per block, bytes per block); a zero
/// block size marks a removed type, which no tensor may use.
const TYPES: [(&str, u64, u64); 43] = [
    ("f32", 1, 4),
    ("f16", 1, 2),
    ("q4_0", 32, 18),
    ("q4_1", 32, 20),
    ("DEPRECATED", 0, 0),
    ("DEPRECATED", 0, 0),
    ("q5_0", 32, 22),
    ("q5_1", 32, 24),
    ("q8_0", 32, 34),
    ("q8_1", 32, 36),
    ("q2_K", 256, 84),
    ("q3_K", 256, 110),
    ("q4_K", 256, 144),
    ("q5_K", 256, 176),
    ("q6_K", 256, 210),
    ("q8_K", 256, 292),
    ("iq2_xxs", 256, 66),
    ("iq2_xs", 256, 74),
    ("iq3_xxs", 256, 98),
    ("iq1_s", 256, 50),
    ("iq4_nl", 32, 18),
    ("iq3_s", 256, 110),
    ("iq2_s", 256, 82),
    ("iq4_xs", 256, 136),
    ("i8", 1, 1),
    ("i16", 1, 2),
    ("i32", 1, 4),
    ("i64", 1, 8),
    ("f64", 1, 8),
    ("iq1_m", 256, 56),
    ("bf16", 1, 2),
    (
        "TYPE_Q4_0_4_4 REMOVED, use Q4_0 with runtime repacking",
        0,
        0,
    ),
    (
        "TYPE_Q4_0_4_8 REMOVED, use Q4_0 with runtime repacking",
        0,
        0,
    ),
    (
        "TYPE_Q4_0_8_8 REMOVED, use Q4_0 with runtime repacking",
        0,
        0,
    ),
    ("tq1_0", 256, 54),
    ("tq2_0", 256, 66),
    (
        "TYPE_IQ4_NL_4_4 REMOVED, use IQ4_NL with runtime repacking",
        0,
        0,
    ),
    (
        "TYPE_IQ4_NL_4_8 REMOVED, use IQ4_NL with runtime repacking",
        0,
        0,
    ),
    (
        "TYPE_IQ4_NL_8_8 REMOVED, use IQ4_NL with runtime repacking",
        0,
        0,
    ),
    ("mxfp4", 32, 17),
    ("nvfp4", 64, 36),
    ("q1_0", 128, 18),
    ("q2_0", 64, 18),
];

/// Metadata value type names (`gguf_type_name`), by type id.
const VALUE_TYPES: [&str; 13] = [
    "u8", "i8", "u16", "i16", "u32", "i32", "f32", "bool", "str", "arr", "u64", "i64", "f64",
];

/// ggml's name for a tensor type id (`ggml_type_name`).
pub fn type_name(kind: i32) -> Option<&'static str> {
    TYPES.get(usize::try_from(kind).ok()?).map(|t| t.0)
}

pub(crate) struct Parsed {
    pub version: u32,
    pub alignment: usize,
    pub data_offset: u64,
    pub metadata: BTreeMap<String, MetadataValue>,
    pub tensors: Vec<TensorInfo>,
}

struct Reader<R> {
    inner: R,
    pos: u64,
    end: u64,
}

impl<R: Read + Seek> Reader<R> {
    fn remain(&self) -> u64 {
        self.end - self.pos
    }

    fn bytes(&mut self, n: u64) -> Result<Vec<u8>, Error> {
        if n > self.remain() {
            return Err(Error::ParseFailed);
        }
        let mut buf = vec![0; n as usize];
        self.inner.read_exact(&mut buf)?;
        self.pos += n;
        Ok(buf)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        if (N as u64) > self.remain() {
            return Err(Error::ParseFailed);
        }
        let mut buf = [0; N];
        self.inner.read_exact(&mut buf)?;
        self.pos += N as u64;
        Ok(buf)
    }

    fn u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_le_bytes(self.array()?))
    }

    fn i32(&mut self) -> Result<i32, Error> {
        Ok(i32::from_le_bytes(self.array()?))
    }

    fn u64(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(self.array()?))
    }

    fn i64(&mut self) -> Result<i64, Error> {
        Ok(i64::from_le_bytes(self.array()?))
    }

    /// Raw string bytes (ggml's `std::string`, which may hold NULs).
    fn string(&mut self) -> Result<Vec<u8>, Error> {
        let n = self.u64()?;
        if n > MAX_STRING {
            return Err(Error::ParseFailed);
        }
        self.bytes(n)
    }

    fn seek(&mut self, to: u64) -> Result<(), Error> {
        if to > self.end {
            return Err(Error::ParseFailed);
        }
        self.inner.seek(SeekFrom::Start(to))?;
        self.pos = to;
        Ok(())
    }
}

/// Strings as the C API returns them: up to the first NUL, lossy UTF-8.
fn c_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

fn width(kind: u32) -> Option<u64> {
    Some(match kind {
        0 | 1 | 7 => 1,
        2 | 3 => 2,
        4..=6 => 4,
        10..=12 => 8,
        _ => return None,
    })
}

fn scalar<R: Read + Seek>(r: &mut Reader<R>, kind: u32) -> Result<MetadataValue, Error> {
    Ok(match kind {
        0 => MetadataValue::Unsigned(r.array::<1>()?[0].into()),
        1 => MetadataValue::Signed((r.array::<1>()?[0] as i8).into()),
        2 => MetadataValue::Unsigned(u16::from_le_bytes(r.array()?).into()),
        3 => MetadataValue::Signed(i16::from_le_bytes(r.array()?).into()),
        4 => MetadataValue::Unsigned(r.u32()?.into()),
        5 => MetadataValue::Signed(r.i32()?.into()),
        6 => MetadataValue::Float(f32::from_le_bytes(r.array()?).into()),
        7 => MetadataValue::Bool(r.array::<1>()?[0] != 0),
        8 => MetadataValue::String(c_string(&r.string()?)),
        10 => MetadataValue::Unsigned(r.u64()?),
        11 => MetadataValue::Signed(r.i64()?),
        12 => MetadataValue::Float(f64::from_le_bytes(r.array()?)),
        _ => return Err(Error::ParseFailed),
    })
}

fn array<R: Read + Seek>(r: &mut Reader<R>, kind: u32, n: u64) -> Result<ArrayValues, Error> {
    if n > MAX_ARRAY {
        return Err(Error::ParseFailed);
    }
    if kind == 8 {
        if r.remain() < n * 8 {
            return Err(Error::ParseFailed);
        }
        let mut out = Vec::with_capacity(n as usize);
        for _ in 0..n {
            out.push(c_string(&r.string()?));
        }
        return Ok(ArrayValues::String(out));
    }
    let size = width(kind).ok_or(Error::ParseFailed)?;
    let raw = r.bytes(n * size)?;
    let mut chunks = raw.chunks_exact(size as usize);
    let mut fixed = || -> [u8; 8] {
        let mut b = [0; 8];
        let c = chunks.next().unwrap();
        b[..c.len()].copy_from_slice(c);
        b
    };
    let n = n as usize;
    Ok(match kind {
        0 => ArrayValues::Unsigned((0..n).map(|_| fixed()[0].into()).collect()),
        1 => ArrayValues::Signed((0..n).map(|_| (fixed()[0] as i8).into()).collect()),
        2 => ArrayValues::Unsigned(
            (0..n)
                .map(|_| u16::from_le_bytes(fixed()[..2].try_into().unwrap()).into())
                .collect(),
        ),
        3 => ArrayValues::Signed(
            (0..n)
                .map(|_| i16::from_le_bytes(fixed()[..2].try_into().unwrap()).into())
                .collect(),
        ),
        4 => ArrayValues::Unsigned(
            (0..n)
                .map(|_| u32::from_le_bytes(fixed()[..4].try_into().unwrap()).into())
                .collect(),
        ),
        5 => ArrayValues::Signed(
            (0..n)
                .map(|_| i32::from_le_bytes(fixed()[..4].try_into().unwrap()).into())
                .collect(),
        ),
        6 => ArrayValues::Float(
            (0..n)
                .map(|_| f32::from_le_bytes(fixed()[..4].try_into().unwrap()).into())
                .collect(),
        ),
        7 => ArrayValues::Bool((0..n).map(|_| fixed()[0] != 0).collect()),
        10 => ArrayValues::Unsigned((0..n).map(|_| u64::from_le_bytes(fixed())).collect()),
        11 => ArrayValues::Signed((0..n).map(|_| i64::from_le_bytes(fixed())).collect()),
        12 => ArrayValues::Float((0..n).map(|_| f64::from_le_bytes(fixed())).collect()),
        _ => unreachable!(),
    })
}

/// `ggml_nbytes` of a contiguous tensor, with its wrapping size_t arithmetic.
fn nbytes(ne: [i64; 4], block: u64, size: u64) -> u64 {
    if ne.iter().any(|&d| d <= 0) {
        return 0;
    }
    let ne = ne.map(|d| d as u64);
    let mut nb = [size, size.wrapping_mul(ne[0] / block), 0, 0];
    nb[2] = nb[1].wrapping_mul(ne[1]);
    nb[3] = nb[2].wrapping_mul(ne[2]);
    let first = if block == 1 {
        size.wrapping_add((ne[0] - 1).wrapping_mul(nb[0]))
    } else {
        ne[0].wrapping_mul(nb[0]) / block
    };
    (1..4).fold(first, |n, i| {
        n.wrapping_add((ne[i] - 1).wrapping_mul(nb[i]))
    })
}

fn pad(x: u64, n: u64) -> Option<u64> {
    Some(x.checked_add(n - 1)? & !(n - 1))
}

/// Parse the header, metadata and tensor table of a GGUF stream of `len` bytes.
pub(crate) fn parse<R: Read + Seek>(inner: R, len: u64) -> Result<Parsed, Error> {
    let mut r = Reader {
        inner: BufReader::with_capacity(1 << 20, inner),
        pos: 0,
        end: len,
    };
    if r.array::<4>()? != *b"GGUF" {
        return Err(Error::ParseFailed);
    }
    let version = r.u32()?;
    if version == 0 || version & 0xffff == 0 || version == 1 || version > VERSION {
        return Err(Error::ParseFailed);
    }
    let n_tensors = r.i64()?;
    let n_kv = r.i64()?;
    if n_tensors < 0 || n_kv < 0 {
        return Err(Error::ParseFailed);
    }

    let mut metadata = BTreeMap::new();
    let mut raw_keys = HashSet::new();
    // gguf_find_key: the first key whose C string is "general.alignment".
    let mut alignment = None;
    for _ in 0..n_kv {
        let key = r.string()?;
        if key.is_empty() || !raw_keys.insert(key.clone()) {
            return Err(Error::ParseFailed);
        }
        let mut kind = r.u32()?;
        let is_array = kind == 9;
        let value = if is_array {
            kind = r.u32()?;
            let n = r.u64()?;
            let values = array(&mut r, kind, n)?;
            MetadataValue::Array {
                element_type: VALUE_TYPES[kind as usize].to_string(),
                len: n as usize,
                values,
            }
        } else {
            scalar(&mut r, kind)?
        };
        // The C API sees keys up to their first NUL; later ids win in the map.
        let name = c_string(&key);
        if name == "general.alignment" && alignment.is_none() {
            alignment = Some(match value {
                MetadataValue::Unsigned(v) if kind == 4 && !is_array => Some(v),
                _ => None,
            });
        }
        metadata.insert(name, value);
    }
    let alignment = match alignment {
        None => DEFAULT_ALIGNMENT,
        Some(Some(v)) if v != 0 && v.is_power_of_two() => v,
        Some(_) => return Err(Error::ParseFailed),
    };

    let mut tensors = Vec::new();
    let mut names = HashSet::new();
    let mut offsets = Vec::new();
    for _ in 0..n_tensors {
        let raw = r.string()?;
        if raw.len() >= MAX_NAME {
            return Err(Error::ParseFailed);
        }
        // ggml_set_name copies a C string: names compare up to the first NUL.
        let name = c_string(&raw);
        let c_name = raw[..raw.iter().position(|&b| b == 0).unwrap_or(raw.len())].to_vec();
        if !names.insert(c_name) {
            return Err(Error::ParseFailed);
        }
        let n_dims = r.u32()?;
        if n_dims > MAX_DIMS {
            return Err(Error::ParseFailed);
        }
        let mut ne = [1i64; 4];
        for (j, d) in ne.iter_mut().enumerate() {
            if (j as u32) < n_dims {
                *d = r.i64()?;
            }
            if *d < 0 {
                return Err(Error::ParseFailed);
            }
        }
        let elements = ne.iter().fold(1i64, |a, &d| a.wrapping_mul(d));
        // ggml v0.26: reject only if the running element count would exceed
        // INT64_MAX when multiplied by a dimension greater than one.
        let mut running = ne[0];
        for &d in &ne[1..] {
            if d > 1 && i64::MAX / d < running {
                return Err(Error::ParseFailed);
            }
            running = running.wrapping_mul(d);
        }
        let type_id = r.i32()?;
        let &(type_name, block, size) = usize::try_from(type_id)
            .ok()
            .and_then(|t| TYPES.get(t))
            .ok_or(Error::ParseFailed)?;
        if block == 0 || !(ne[0] as u64).is_multiple_of(block) {
            return Err(Error::ParseFailed);
        }
        if (elements / block as i64) as u64 > u64::MAX / size {
            return Err(Error::ParseFailed);
        }
        let size_bytes = nbytes(ne, block, size);
        offsets.push(r.u64()?);
        tensors.push(TensorInfo {
            name,
            type_id,
            type_name: type_name.to_string(),
            shape: ne.map(|d| d as u64),
            file_offset: 0,
            size_bytes,
        });
    }
    if n_tensors > 0 {
        let start = pad(r.pos, alignment).ok_or(Error::ParseFailed)?;
        r.seek(start)?;
    }
    let data_offset = r.pos;
    let mut expected = 0u64;
    for (tensor, &offset) in tensors.iter_mut().zip(&offsets) {
        if offset != expected {
            return Err(Error::ParseFailed);
        }
        let padded = pad(tensor.size_bytes, alignment).ok_or(Error::ParseFailed)?;
        expected = expected.checked_add(padded).ok_or(Error::ParseFailed)?;
        // Relative to the data section until lib.rs validates the layout.
        tensor.file_offset = offset;
    }
    Ok(Parsed {
        version,
        alignment: alignment as usize,
        data_offset,
        metadata,
        tensors,
    })
}
