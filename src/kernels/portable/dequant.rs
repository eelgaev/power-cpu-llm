//! Byte-oriented GGUF reference decoding (formerly `native/dequant.c`). No
//! aligned struct casts; format definitions follow the pinned MIT-licensed
//! ggml source (`vendor/ggml/src/ggml-common.h`). The frozen POWER9 decoder
//! calls `p9_decode_row` for formats it does not vectorize.

use std::ffi::c_int;

pub(crate) fn u16le(p: &[u8]) -> u16 {
    u16::from(p[0]) | (u16::from(p[1]) << 8)
}

pub(crate) fn u32le(p: &[u8]) -> u32 {
    u32::from_le_bytes([p[0], p[1], p[2], p[3]])
}

/// IEEE binary16 to binary32, exact.
pub(crate) fn half(p: &[u8]) -> f32 {
    let h = u32::from(u16le(p));
    let sign = (h & 0x8000) << 16;
    let exponent = (h >> 10) & 31;
    let mut mantissa = h & 1023;
    let bits = if exponent == 0 {
        if mantissa == 0 {
            sign
        } else {
            let mut e = -14i32;
            while mantissa & 1024 == 0 {
                mantissa <<= 1;
                e -= 1;
            }
            sign | (((e + 127) as u32) << 23) | ((mantissa & 1023) << 13)
        }
    } else if exponent == 31 {
        sign | 0x7f80_0000 | (mantissa << 13)
    } else {
        sign | ((exponent + 112) << 23) | (mantissa << 13)
    };
    f32::from_bits(bits)
}

pub(crate) fn signed_byte(x: u8) -> i32 {
    i32::from(x as i8)
}

/// (elements, bytes) per block of a decodable GGUF type id.
pub(crate) fn layout(kind: c_int) -> Option<(usize, usize)> {
    Some(match kind {
        0 => (1, 4),
        1 => (1, 2),
        2 => (32, 18),
        6 => (32, 22),
        8 => (32, 34),
        12 => (256, 144),
        14 => (256, 210),
        35 => (256, 66),
        _ => return None,
    })
}

/// Decode whole blocks of `kind` from `input` into `out` (sizes checked).
pub(crate) fn decode(kind: c_int, input: &[u8], out: &mut [f32]) {
    let (elements, bytes) = layout(kind).expect("checked type");
    for (p, y) in input
        .chunks_exact(bytes)
        .zip(out.chunks_exact_mut(elements))
    {
        match kind {
            0 => y[0] = f32::from_bits(u32le(p)),
            1 => y[0] = half(p),
            35 => {
                // ggml dequantize_row_tq2_0: qs[64] (four 2-bit values per
                // byte), then d; element (j * 4 + l * 32 + m) = (q - 1) * d.
                let d = half(&p[64..]);
                for j in [0, 32] {
                    for l in 0..4 {
                        for m in 0..32 {
                            let q = i32::from((p[j + m] >> (2 * l)) & 3);
                            y[j * 4 + l * 32 + m] = (q - 1) as f32 * d;
                        }
                    }
                }
            }
            8 => {
                let d = half(p);
                for i in 0..32 {
                    y[i] = d * signed_byte(p[2 + i]) as f32;
                }
            }
            2 => {
                let d = half(p);
                for i in 0..32 {
                    let nibble = if i < 16 {
                        p[2 + i] & 15
                    } else {
                        p[2 + i - 16] >> 4
                    };
                    y[i] = d * (i32::from(nibble) - 8) as f32;
                }
            }
            6 => {
                let d = half(p);
                let high = u32le(&p[2..]);
                for i in 0..32 {
                    let nibble = u32::from(if i < 16 {
                        p[6 + i] & 15
                    } else {
                        p[6 + i - 16] >> 4
                    });
                    let q = (nibble | (((high >> i) & 1) << 4)) as i32 - 16;
                    y[i] = d * q as f32;
                }
            }
            12 => {
                let scales = &p[4..16];
                let q = &p[16..];
                let (d, dmin) = (half(p), half(&p[2..]));
                for group in 0..8 {
                    let (scale, min) = if group < 4 {
                        (scales[group] & 63, scales[group + 4] & 63)
                    } else {
                        (
                            (scales[group + 4] & 15) | ((scales[group - 4] >> 6) << 4),
                            (scales[group + 4] >> 4) | ((scales[group] >> 6) << 4),
                        )
                    };
                    let ds = d * f32::from(scale);
                    let dm = dmin * f32::from(min);
                    for i in 0..32 {
                        let packed = q[(group / 2) * 32 + i];
                        let value = if group % 2 == 0 {
                            packed & 15
                        } else {
                            packed >> 4
                        };
                        y[group * 32 + i] = ds * f32::from(value) - dm;
                    }
                }
            }
            14 => {
                let d = half(&p[208..]);
                for h in 0..2 {
                    for group in 0..4 {
                        let low_shift = if group < 2 { 0 } else { 4 };
                        let high_shift = group * 2;
                        for sub in 0..2 {
                            let low = h * 64 + (group % 2) * 32 + sub * 16;
                            let high = 128 + h * 32 + sub * 16;
                            let ds = d * signed_byte(p[192 + h * 8 + group * 2 + sub]) as f32;
                            let dst = h * 128 + group * 32 + sub * 16;
                            for lane in 0..16 {
                                let ql = (p[low + lane] >> low_shift) & 15;
                                let qh = (p[high + lane] >> high_shift) & 3;
                                y[dst + lane] = ds * (i32::from(ql | (qh << 4)) - 32) as f32;
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

/// Portable reference decoder for little-endian GGUF rows. Validates exact
/// input size and block divisibility before writing.
///
/// # Safety
/// `input` holds `input_size` bytes and `out` `count` floats, not overlapping.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p9_decode_row(
    kind: c_int,
    input: *const u8,
    input_size: usize,
    out: *mut f32,
    count: usize,
) -> bool {
    let Some((elements, bytes)) = layout(kind) else {
        return false;
    };
    if !count.is_multiple_of(elements)
        || count / elements > usize::MAX / bytes
        || count / elements * bytes != input_size
    {
        return false;
    }
    if count == 0 {
        return true;
    }
    // SAFETY: caller contract above.
    let (input, out) = unsafe {
        (
            std::slice::from_raw_parts(input, input_size),
            std::slice::from_raw_parts_mut(out, count),
        )
    };
    decode(kind, input, out);
    true
}
