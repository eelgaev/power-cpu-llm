//! Metadata-only GGUF reader in Rust, accepting exactly what the pinned ggml
//! reader accepts (`parse.rs`). No weight blob is allocated, mapped, or
//! evaluated. Use trusted local model files: this parser is not an
//! upload/security sandbox.

mod parse;
pub use parse::type_name;

#[cfg(feature = "reference-oracle")]
mod ffi;
#[cfg(feature = "reference-oracle")]
pub mod ggml;
#[cfg(feature = "reference-oracle")]
mod metadata;

#[cfg(feature = "reference-oracle")]
pub mod reference;

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidPath,
    ParseFailed,
    InvalidLayout(String),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::InvalidPath => write!(f, "model path contains a NUL byte"),
            Self::ParseFailed => write!(f, "could not parse GGUF metadata"),
            Self::InvalidLayout(e) => write!(f, "invalid GGUF layout: {e}"),
        }
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// Integer/float widths are widened without losing their original array type
/// name in MetadataValue. Buffers are copied, never borrowed from native state.
#[derive(Debug, Clone, PartialEq)]
pub enum ArrayValues {
    Unsigned(Vec<u64>),
    Signed(Vec<i64>),
    Float(Vec<f64>),
    Bool(Vec<bool>),
    String(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetadataValue {
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Bool(bool),
    String(String),
    /// Owned contents; the native context can be freed immediately.
    Array {
        element_type: String,
        len: usize,
        values: ArrayValues,
    },
}

impl MetadataValue {
    /// Avoid dumping hundreds of thousands of tokenizer entries in inspection.
    pub fn summary(&self) -> String {
        match self {
            Self::Array {
                element_type,
                len,
                values,
            } if *len <= 64 => format!("Array<{element_type}>[{len}] {values:?}"),
            Self::Array {
                element_type, len, ..
            } => format!("Array<{element_type}>[{len}]"),
            _ => format!("{self:?}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TensorInfo {
    pub name: String,
    pub type_id: i32,
    pub type_name: String,
    /// ggml's normalized dimensions: trailing unused dimensions are 1.
    pub shape: [u64; 4],
    /// Absolute byte offset from the start of this file.
    pub file_offset: u64,
    pub size_bytes: u64,
}

#[derive(Debug)]
pub struct Inventory {
    pub version: u32,
    pub alignment: usize,
    pub data_offset: u64,
    pub file_size: u64,
    pub metadata: BTreeMap<String, MetadataValue>,
    pub tensors: Vec<TensorInfo>,
}

impl Inventory {
    pub fn architecture(&self) -> Option<&str> {
        match self.metadata.get("general.architecture") {
            Some(MetadataValue::String(s)) => Some(s),
            _ => None,
        }
    }

    /// Counts and encoded bytes by tensor type, not a statement of execution support.
    pub fn quantization_summary(&self) -> BTreeMap<&str, (usize, u64)> {
        let mut summary = BTreeMap::new();
        for tensor in &self.tensors {
            let entry = summary.entry(tensor.type_name.as_str()).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += tensor.size_bytes;
        }
        summary
    }
}

/// Inspect a single GGUF file. Split-file aggregation is not implemented yet.
/// Validates every tensor range against file length before returning inventory.
pub fn inspect(path: impl AsRef<Path>) -> Result<Inventory, Error> {
    let path = path.as_ref();
    if path.as_os_str().as_encoded_bytes().contains(&0) {
        return Err(Error::InvalidPath);
    }
    let file = std::fs::File::open(path)?;
    let file_size = file.metadata()?.len();
    validate(parse::parse(file, file_size)?, file_size)
}

/// Shared layout checks; tensor offsets arrive relative to the data section.
fn validate(parsed: parse::Parsed, file_size: u64) -> Result<Inventory, Error> {
    let parse::Parsed {
        version,
        alignment,
        data_offset,
        metadata,
        mut tensors,
    } = parsed;
    if data_offset > file_size {
        return Err(Error::InvalidLayout("data offset exceeds file size".into()));
    }
    for tensor in &mut tensors {
        let name = &tensor.name;
        if tensor.shape.contains(&0) {
            return Err(Error::InvalidLayout(format!(
                "non-positive dimension in {name}"
            )));
        }
        tensor.file_offset = data_offset
            .checked_add(tensor.file_offset)
            .ok_or_else(|| Error::InvalidLayout(format!("offset overflow in {name}")))?;
        let end = tensor
            .file_offset
            .checked_add(tensor.size_bytes)
            .ok_or_else(|| Error::InvalidLayout(format!("size overflow in {name}")))?;
        if end > file_size {
            return Err(Error::InvalidLayout(format!(
                "tensor {name} ends beyond file size"
            )));
        }
    }
    Ok(Inventory {
        version,
        alignment,
        data_offset,
        file_size,
        metadata,
        tensors,
    })
}

#[cfg(test)]
mod tests;
