use super::*;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

struct Fixture(PathBuf);
impl Fixture {
    fn new(bytes: &[u8]) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "power-gguf-{}-{}.gguf",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        file.write_all(bytes).unwrap();
        Self(path)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn string(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u64).to_le_bytes());
    bytes.extend_from_slice(value.as_bytes());
}

// Independently encoded GGUF v3 fixture, with one 32-element F32 tensor.
fn model() -> Vec<u8> {
    let mut bytes = b"GGUF".to_vec();
    bytes.extend_from_slice(&3u32.to_le_bytes());
    bytes.extend_from_slice(&1u64.to_le_bytes()); // tensors
    bytes.extend_from_slice(&4u64.to_le_bytes()); // metadata
    string(&mut bytes, "general.architecture");
    bytes.extend_from_slice(&8u32.to_le_bytes());
    string(&mut bytes, "test-model");
    string(&mut bytes, "test-model.block_count");
    bytes.extend_from_slice(&4u32.to_le_bytes());
    bytes.extend_from_slice(&2u32.to_le_bytes());
    string(&mut bytes, "test.bool");
    bytes.extend_from_slice(&7u32.to_le_bytes());
    bytes.push(1);
    string(&mut bytes, "test.array");
    bytes.extend_from_slice(&9u32.to_le_bytes());
    bytes.extend_from_slice(&4u32.to_le_bytes()); // u32 elements
    bytes.extend_from_slice(&2u64.to_le_bytes());
    bytes.extend_from_slice(&42u32.to_le_bytes());
    bytes.extend_from_slice(&99u32.to_le_bytes());
    string(&mut bytes, "test.weight");
    bytes.extend_from_slice(&1u32.to_le_bytes()); // dimensions
    bytes.extend_from_slice(&32u64.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes()); // F32
    bytes.extend_from_slice(&0u64.to_le_bytes()); // relative offset
    bytes.resize(bytes.len().div_ceil(32) * 32, 0);
    bytes.extend_from_slice(&[0; 128]);
    bytes
}

#[test]
fn reads_metadata_and_tensor_offsets_without_loading_weights() {
    let fixture = Fixture::new(&model());
    let inventory = inspect(&fixture.0).unwrap();
    assert_eq!(inventory.version, 3);
    assert_eq!(inventory.architecture(), Some("test-model"));
    assert_eq!(
        inventory.metadata["test-model.block_count"],
        MetadataValue::Unsigned(2)
    );
    assert_eq!(inventory.metadata["test.bool"], MetadataValue::Bool(true));
    assert!(matches!(
        inventory.metadata["test.array"],
        MetadataValue::Array { len: 2, .. }
    ));
    match &inventory.metadata["test.array"] {
        MetadataValue::Array { values, .. } => {
            assert_eq!(values, &ArrayValues::Unsigned(vec![42, 99]))
        }
        _ => panic!("array missing"),
    }
    assert_eq!(inventory.tensors.len(), 1);
    let t = &inventory.tensors[0];
    assert_eq!(t.name, "test.weight");
    assert_eq!(t.shape, [32, 1, 1, 1]);
    assert_eq!(t.type_name, "f32");
    assert_eq!(t.size_bytes, 128);
    assert_eq!(t.file_offset, inventory.data_offset);
    assert_eq!(inventory.quantization_summary()["f32"], (1, 128));
}

#[test]
fn copies_every_primitive_array_type_and_strings() {
    let mut payloads: Vec<(u32, Vec<u8>, ArrayValues)> = vec![
        (0, vec![0, 255], ArrayValues::Unsigned(vec![0, 255])),
        (1, vec![128, 127], ArrayValues::Signed(vec![-128, 127])),
        (
            2,
            [0u16, 65535].iter().flat_map(|n| n.to_le_bytes()).collect(),
            ArrayValues::Unsigned(vec![0, 65535]),
        ),
        (
            3,
            [-32768i16, 32767]
                .iter()
                .flat_map(|n| n.to_le_bytes())
                .collect(),
            ArrayValues::Signed(vec![-32768, 32767]),
        ),
        (
            4,
            [0u32, u32::MAX]
                .iter()
                .flat_map(|n| n.to_le_bytes())
                .collect(),
            ArrayValues::Unsigned(vec![0, u32::MAX.into()]),
        ),
        (
            5,
            [i32::MIN, i32::MAX]
                .iter()
                .flat_map(|n| n.to_le_bytes())
                .collect(),
            ArrayValues::Signed(vec![i32::MIN.into(), i32::MAX.into()]),
        ),
        (
            6,
            [-0.25f32, 1.5]
                .iter()
                .flat_map(|n| n.to_le_bytes())
                .collect(),
            ArrayValues::Float(vec![-0.25, 1.5]),
        ),
        (7, vec![0, 1], ArrayValues::Bool(vec![false, true])),
        (
            10,
            [0u64, u64::MAX]
                .iter()
                .flat_map(|n| n.to_le_bytes())
                .collect(),
            ArrayValues::Unsigned(vec![0, u64::MAX]),
        ),
        (
            11,
            [i64::MIN, i64::MAX]
                .iter()
                .flat_map(|n| n.to_le_bytes())
                .collect(),
            ArrayValues::Signed(vec![i64::MIN, i64::MAX]),
        ),
        (
            12,
            [-0.25f64, 1.5]
                .iter()
                .flat_map(|n| n.to_le_bytes())
                .collect(),
            ArrayValues::Float(vec![-0.25, 1.5]),
        ),
    ];
    let mut strings = Vec::new();
    string(&mut strings, "hello");
    string(&mut strings, "世界");
    payloads.push((
        8,
        strings,
        ArrayValues::String(vec!["hello".into(), "世界".into()]),
    ));
    let mut bytes = b"GGUF".to_vec();
    bytes.extend_from_slice(&3u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(&(payloads.len() as u64).to_le_bytes());
    for (kind, payload, _) in &payloads {
        string(&mut bytes, &format!("array.{kind}"));
        bytes.extend_from_slice(&9u32.to_le_bytes());
        bytes.extend_from_slice(&kind.to_le_bytes());
        bytes.extend_from_slice(&2u64.to_le_bytes());
        bytes.extend_from_slice(payload);
    }
    let fixture = Fixture::new(&bytes);
    let model = inspect(&fixture.0).unwrap();
    drop(fixture); // returned metadata owns its storage
    for (kind, _, expected) in payloads {
        match &model.metadata[&format!("array.{kind}")] {
            MetadataValue::Array { len: 2, values, .. } => assert_eq!(values, &expected),
            _ => panic!("wrong array {kind}"),
        }
    }
}

#[test]
fn rejects_truncated_weight_blob_even_with_valid_metadata() {
    let mut bytes = model();
    bytes.pop();
    let fixture = Fixture::new(&bytes);
    assert!(matches!(inspect(&fixture.0), Err(Error::InvalidLayout(_))));
}

#[test]
fn invalid_header_and_missing_file_return_errors() {
    let fixture = Fixture::new(b"NOT A GGUF FILE");
    assert!(matches!(inspect(&fixture.0), Err(Error::ParseFailed)));
    let path = fixture.0.clone();
    drop(fixture);
    assert!(matches!(inspect(path), Err(Error::Io(_))));
}

/// The Rust parser against the pinned ggml reader: identical inventories or
/// identical errors.
#[cfg(feature = "reference-oracle")]
mod ggml_twin {
    use super::*;

    unsafe extern "C" {
        fn ggml_log_set(
            callback: Option<
                unsafe extern "C" fn(i32, *const std::ffi::c_char, *mut std::ffi::c_void),
            >,
            user: *mut std::ffi::c_void,
        );
    }
    unsafe extern "C" fn quiet(_: i32, _: *const std::ffi::c_char, _: *mut std::ffi::c_void) {}

    fn both(path: &Path) -> (String, String) {
        // SAFETY: a no-op logger; ggml only calls it during parsing.
        unsafe { ggml_log_set(Some(quiet), std::ptr::null_mut()) };
        (
            format!("{:?}", inspect(path)),
            format!("{:?}", ggml::inspect(path)),
        )
    }

    /// Every GGUF model under POWER_LLM_GGUF_DIRS (colon-separated, default
    /// `$HOME/models`); absent directories are skipped.
    #[test]
    fn real_models_match_ggml() {
        let dirs = std::env::var("POWER_LLM_GGUF_DIRS")
            .unwrap_or_else(|_| format!("{}/models", std::env::var("HOME").unwrap_or_default()));
        let mut files = Vec::new();
        let mut stack: Vec<PathBuf> = dirs.split(':').map(PathBuf::from).collect();
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "gguf") {
                    files.push(path);
                }
            }
        }
        files.sort();
        for path in &files {
            let (rust, ggml) = both(path);
            assert!(rust == ggml, "{}: Rust and ggml differ", path.display());
            eprintln!(
                "{} identical ({} bytes of Debug)",
                path.display(),
                rust.len()
            );
        }
        eprintln!("{} GGUF files compared", files.len());
    }

    /// A richer fixture: several tensors and types, padding, string arrays,
    /// every scalar type and an explicit alignment.
    fn rich() -> Vec<u8> {
        let mut b = b"GGUF".to_vec();
        b.extend_from_slice(&3u32.to_le_bytes());
        b.extend_from_slice(&3u64.to_le_bytes());
        b.extend_from_slice(&14u64.to_le_bytes());
        string(&mut b, "general.alignment");
        b.extend_from_slice(&4u32.to_le_bytes());
        b.extend_from_slice(&64u32.to_le_bytes());
        for (i, (kind, width)) in [
            (0u32, 1),
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 4),
            (5, 4),
            (6, 4),
            (7, 1),
            (10, 8),
            (11, 8),
            (12, 8),
        ]
        .into_iter()
        .enumerate()
        {
            string(&mut b, &format!("k.scalar{i}"));
            b.extend_from_slice(&kind.to_le_bytes());
            b.extend((0..width).map(|j| (i * 37 + j * 11) as u8));
        }
        string(&mut b, "k.strings");
        b.extend_from_slice(&9u32.to_le_bytes());
        b.extend_from_slice(&8u32.to_le_bytes());
        b.extend_from_slice(&3u64.to_le_bytes());
        for s in ["a", "", "token\u{2581}x"] {
            string(&mut b, s);
        }
        string(&mut b, "k.f32s");
        b.extend_from_slice(&9u32.to_le_bytes());
        b.extend_from_slice(&6u32.to_le_bytes());
        b.extend_from_slice(&2u64.to_le_bytes());
        b.extend_from_slice(&1.5f32.to_le_bytes());
        b.extend_from_slice(&(-0.0f32).to_le_bytes());
        // (name, dims, type, bytes): q4_0 [64, 2], f32 [3], q8_0 [32, 1, 2].
        let tensors: [(&str, &[u64], u32, u64); 3] = [
            ("blk.0.w", &[64, 2], 2, 72),
            ("blk.0.b", &[3], 0, 12),
            ("blk.1.w", &[32, 1, 2], 8, 68),
        ];
        let mut offset = 0u64;
        for (name, dims, kind, bytes) in tensors {
            string(&mut b, name);
            b.extend_from_slice(&(dims.len() as u32).to_le_bytes());
            for d in dims {
                b.extend_from_slice(&d.to_le_bytes());
            }
            b.extend_from_slice(&kind.to_le_bytes());
            b.extend_from_slice(&offset.to_le_bytes());
            offset += bytes.div_ceil(64) * 64;
        }
        b.resize(b.len().div_ceil(64) * 64, 0);
        b.extend(std::iter::repeat_n(0x11, offset as usize));
        b
    }

    /// Element-count overflow boundary (ggml v0.26 rule): shapes whose
    /// product is exactly representable are fine, zero-element tensors with
    /// overflowing leading dimensions are rejected.
    #[test]
    fn element_count_overflow_matches_ggml() {
        let shapes: [[i64; 4]; 8] = [
            [i64::MAX, 1, 1, 1],
            [1 << 40, 1 << 40, 0, 1],
            [1 << 40, 1 << 40, 1, 1],
            [1 << 31, 1 << 31, 1, 1],
            [0, 1 << 40, 1 << 40, 1],
            [1 << 62, 2, 1, 1],
            [1 << 62, 2, 0, 1],
            [3, 3074457345618258602, 1, 1],
        ];
        for ne in shapes {
            let mut b = b"GGUF".to_vec();
            b.extend_from_slice(&3u32.to_le_bytes());
            b.extend_from_slice(&1u64.to_le_bytes());
            b.extend_from_slice(&0u64.to_le_bytes());
            string(&mut b, "t");
            b.extend_from_slice(&4u32.to_le_bytes());
            for d in ne {
                b.extend_from_slice(&d.to_le_bytes());
            }
            b.extend_from_slice(&0u32.to_le_bytes()); // F32
            b.extend_from_slice(&0u64.to_le_bytes()); // offset
            b.resize(b.len().div_ceil(32) * 32 + 64, 0);
            let (rust, ggml) = both(&Fixture::new(&b).0);
            assert_eq!(rust, ggml, "shape {ne:?}");
        }
    }

    /// Random byte, word and length mutations of both fixtures.
    #[test]
    fn mutated_files_match_ggml() {
        let seeds = [model(), rich()];
        let mut state = 0x9e3779b97f4a7c15u64;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let (mut accepted, mut cases) = (0, 0);
        for seed in &seeds {
            let (rust, ggml) = both(&Fixture::new(seed).0);
            assert_eq!(rust, ggml);
            assert!(rust.starts_with("Ok("), "{rust}");
            for _ in 0..6000 {
                let mut b = seed.clone();
                for _ in 0..1 + next() % 3 {
                    let at = (next() as usize) % b.len();
                    match next() % 6 {
                        0 => b[at] = next() as u8,
                        1 => b[at] ^= 1 << (next() % 8),
                        2 => {
                            // Plausible small or extreme little-endian words.
                            let word = [
                                0u64,
                                1,
                                2,
                                3,
                                4,
                                5,
                                8,
                                9,
                                12,
                                13,
                                31,
                                32,
                                63,
                                64,
                                255,
                                1 << 30,
                                (1 << 30) + 1,
                                u64::MAX,
                                i64::MAX as u64,
                                u64::MAX - 7,
                            ][next() as usize % 20];
                            let end = (at + 8).min(b.len());
                            b[at..end].copy_from_slice(&word.to_le_bytes()[..end - at]);
                        }
                        3 => b.truncate(at),
                        4 => b
                            .splice(at..at, (0..next() % 9).map(|_| next() as u8))
                            .for_each(drop),
                        _ => {
                            let end = (at + 1 + next() as usize % 8).min(b.len());
                            b.drain(at..end);
                        }
                    }
                    if b.is_empty() {
                        break;
                    }
                }
                let fixture = Fixture::new(&b);
                let (rust, ggml) = both(&fixture.0);
                assert!(
                    rust == ggml,
                    "mutation differs:\nrust {rust}\nggml {ggml}\nbytes {b:?}"
                );
                cases += 1;
                accepted += rust.starts_with("Ok(") as usize;
            }
        }
        eprintln!("{cases} mutated files, {accepted} accepted by both");
        assert!(accepted > 100 && accepted < cases - 100);
    }
}
