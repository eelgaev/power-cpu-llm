//! Validated, positional reads of immutable local model tensor rows.
//! No mmap yet: small reads avoid unsafe mapping/lifetime assumptions.

use crate::gguf::{Inventory, TensorInfo};
use crate::kernels::encoded::Encoding;
#[cfg(unix)]
use std::os::unix::fs::FileExt;
use std::{collections::BTreeMap, fmt, fs::File, path::Path};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Gguf(crate::gguf::Error),
    Encoding(crate::kernels::encoded::Error),
    UnsupportedArchitecture,
    TensorMissing(String),
    InvalidLayout(String),
    RowOutOfRange,
    ReadLimit,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<crate::gguf::Error> for Error {
    fn from(e: crate::gguf::Error) -> Self {
        Self::Gguf(e)
    }
}
impl From<crate::kernels::encoded::Error> for Error {
    fn from(e: crate::kernels::encoded::Error) -> Self {
        Self::Encoding(e)
    }
}

#[derive(Debug)]
pub struct EncodedRow {
    pub encoding: Encoding,
    pub elements: usize,
    pub bytes: Vec<u8>,
}
impl EncodedRow {
    pub fn decode(&self) -> Result<Vec<f32>, Error> {
        Ok(self.encoding.decode(&self.bytes, self.elements)?)
    }
}

/// Owned bounded contiguous encoded row tile. Rows flatten dimensions 1..3,
/// so this also reads a selected range inside an expert tensor.
#[derive(Debug)]
pub struct EncodedTile {
    pub encoding: Encoding,
    pub elements: usize,
    pub rows: usize,
    pub bytes: Vec<u8>,
}

type CacheMap = std::collections::HashMap<String, std::sync::Arc<dyn std::any::Any + Send + Sync>>;

/// Read-only shared mapping of the whole (immutable) model file.
struct Mapping {
    ptr: *const u8,
    len: usize,
}
// SAFETY: a read-only mapping of plain bytes; never written, unmapped on drop.
unsafe impl Send for Mapping {}
unsafe impl Sync for Mapping {}
impl Mapping {
    fn new(file: &File, len: u64) -> Option<Self> {
        use std::os::fd::AsRawFd;
        let len = usize::try_from(len).ok().filter(|&n| n > 0)?;
        if std::env::var_os("POWER_LLM_NO_MMAP").is_some() {
            return None;
        }
        // SAFETY: valid fd and length; PROT_READ/MAP_SHARED creates no aliasing writes.
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                len,
                libc::PROT_READ,
                libc::MAP_SHARED,
                file.as_raw_fd(),
                0,
            )
        };
        (ptr != libc::MAP_FAILED).then(|| Self {
            ptr: ptr.cast(),
            len,
        })
    }
    fn bytes(&self) -> &[u8] {
        // SAFETY: the mapping covers `len` readable bytes until drop.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}
impl Drop for Mapping {
    fn drop(&mut self) {
        // SAFETY: exactly the region returned by mmap.
        unsafe { libc::munmap(self.ptr.cast_mut().cast(), self.len) };
    }
}

/// Encoded rows borrowed from the file mapping (or owned when unmapped).
#[derive(Debug)]
pub struct RowsView<'f> {
    pub encoding: Encoding,
    pub elements: usize,
    pub rows: usize,
    pub bytes: std::borrow::Cow<'f, [u8]>,
}

pub struct ModelFile {
    map: Option<Mapping>,
    file: File,
    inventory: Inventory,
    tensor_ids: BTreeMap<String, usize>,
    /// Derived immutable data (decoded small tensors, routers), built once per
    /// file and shared; weights never change, so this is pure memoization.
    cache: std::sync::Mutex<CacheMap>,
    /// Unique per opened file (never reused), for caches keyed by address.
    id: u64,
}
impl ModelFile {
    /// A process-unique id of this opened file.
    pub fn id(&self) -> u64 {
        self.id
    }
    /// File must be a completed, immutable local download. Concurrent content
    /// changes cannot be fully detected; truncated reads produce I/O errors.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file = File::open(path.as_ref())?;
        let inventory = crate::gguf::inspect(path)?;
        if file.metadata()?.len() != inventory.file_size {
            return Err(Error::InvalidLayout(
                "file changed size during inspection".into(),
            ));
        }
        if !matches!(
            inventory.architecture(),
            Some(
                "diffusion-gemma"
                    | "gemma4"
                    | crate::engine::nemotron::ARCHITECTURE
                    | crate::engine::assistant::ARCHITECTURE
                    | crate::engine::qwen35::ARCHITECTURE
            )
        ) {
            return Err(Error::UnsupportedArchitecture);
        }
        let mut tensor_ids = BTreeMap::new();
        for (index, tensor) in inventory.tensors.iter().enumerate() {
            validate_layout(tensor)?;
            if tensor_ids.insert(tensor.name.clone(), index).is_some() {
                return Err(Error::InvalidLayout("duplicate tensor name".into()));
            }
        }
        let map = Mapping::new(&file, inventory.file_size);
        Ok(Self {
            map,
            file,
            inventory,
            tensor_ids,
            cache: std::sync::Mutex::new(CacheMap::new()),
            id: {
                static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            },
        })
    }
    /// Like [`Self::read_rows`] (same validation), but borrows the bytes from
    /// the file mapping instead of copying them when the file is mapped. The
    /// model file must not change while it is open (as for `read_rows`).
    pub fn view_rows(
        &self,
        name: &str,
        first: u64,
        rows: usize,
        max_bytes: usize,
    ) -> Result<RowsView<'_>, Error> {
        let Some(map) = &self.map else {
            let tile = self.read_rows(name, first, rows, max_bytes)?;
            return Ok(RowsView {
                encoding: tile.encoding,
                elements: tile.elements,
                rows: tile.rows,
                bytes: std::borrow::Cow::Owned(tile.bytes),
            });
        };
        let (encoding, elements, offset, size) = self.locate(name, first, rows, max_bytes)?;
        let start = usize::try_from(offset).map_err(|_| Error::ReadLimit)?;
        let bytes = start
            .checked_add(size)
            .and_then(|end| map.bytes().get(start..end))
            .ok_or(Error::ReadLimit)?;
        Ok(RowsView {
            encoding,
            elements,
            rows,
            bytes: std::borrow::Cow::Borrowed(bytes),
        })
    }
    /// Memoized `make()` under `key` for the lifetime of this file. `make`
    /// runs without the lock held; concurrent first calls may both build (the
    /// first stored value wins). Errors are not cached.
    pub fn cached<T: Send + Sync + 'static, E>(
        &self,
        key: &str,
        make: impl FnOnce() -> Result<T, E>,
    ) -> Result<std::sync::Arc<T>, E> {
        let lookup = |cache: &CacheMap| {
            cache
                .get(key)
                .and_then(|v| std::sync::Arc::clone(v).downcast::<T>().ok())
        };
        if let Some(found) = lookup(&self.cache.lock().unwrap_or_else(|e| e.into_inner())) {
            return Ok(found);
        }
        let value = std::sync::Arc::new(make()?);
        let mut cache = self.cache.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(found) = lookup(&cache) {
            return Ok(found);
        }
        cache.insert(key.to_owned(), value.clone());
        Ok(value)
    }
    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }
    pub fn tensor(&self, name: &str) -> Option<&TensorInfo> {
        self.tensor_ids
            .get(name)
            .map(|&id| &self.inventory.tensors[id])
    }
    /// Rows flatten dimensions 1..3 in GGUF order. For expert tensors the
    /// expert index is part of this flattened row index; dimension 0 is width.
    pub fn read_row(&self, name: &str, row: u64) -> Result<EncodedRow, Error> {
        let tile = self.read_rows(name, row, 1, usize::MAX)?;
        Ok(EncodedRow {
            encoding: tile.encoding,
            elements: tile.elements,
            bytes: tile.bytes,
        })
    }
    /// Validate the complete range and byte budget before allocating/reading.
    /// One positional read, no per-row seek, decoding, mmap or retained pointer.
    pub fn read_rows(
        &self,
        name: &str,
        first: u64,
        rows: usize,
        max_bytes: usize,
    ) -> Result<EncodedTile, Error> {
        let (encoding, elements, offset, size) = self.locate(name, first, rows, max_bytes)?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(size)
            .map_err(|_| Error::ReadLimit)?;
        bytes.resize(size, 0);
        self.file.read_exact_at(&mut bytes, offset)?;
        Ok(EncodedTile {
            encoding,
            elements,
            rows,
            bytes,
        })
    }
    /// Validated (encoding, elements per row, file offset, byte size) of rows
    /// `first..first + rows` of `name` within `max_bytes`.
    fn locate(
        &self,
        name: &str,
        first: u64,
        rows: usize,
        max_bytes: usize,
    ) -> Result<(Encoding, usize, u64, usize), Error> {
        let tensor = self
            .tensor(name)
            .ok_or_else(|| Error::TensorMissing(name.into()))?;
        let (encoding, elements, bytes_per_row, row_count) = validate_layout(tensor)?;
        if rows == 0
            || u64::try_from(rows)
                .ok()
                .and_then(|n| first.checked_add(n))
                .filter(|&n| n <= row_count)
                .is_none()
        {
            return Err(Error::RowOutOfRange);
        }
        let size = rows
            .checked_mul(bytes_per_row)
            .filter(|&n| n <= max_bytes && n <= isize::MAX as usize)
            .ok_or(Error::ReadLimit)?;
        let offset = first
            .checked_mul(bytes_per_row as u64)
            .and_then(|n| tensor.file_offset.checked_add(n))
            .ok_or_else(|| Error::InvalidLayout("row offset overflow".into()))?;
        Ok((encoding, elements, offset, size))
    }
}

pub fn row_count(tensor: &TensorInfo) -> Result<u64, Error> {
    tensor.shape[1..]
        .iter()
        .try_fold(1u64, |n, &dimension| n.checked_mul(dimension))
        .ok_or_else(|| Error::InvalidLayout("row count overflow".into()))
}

fn validate_layout(tensor: &TensorInfo) -> Result<(Encoding, usize, usize, u64), Error> {
    if tensor.shape.contains(&0) {
        return Err(Error::InvalidLayout("zero tensor dimension".into()));
    }
    let encoding = Encoding::try_from(tensor.type_id)?;
    let elements = usize::try_from(tensor.shape[0])
        .map_err(|_| Error::InvalidLayout("row width overflow".into()))?;
    let bytes_per_row = encoding.row_bytes(elements)?;
    let rows = row_count(tensor)?;
    let size = rows
        .checked_mul(bytes_per_row as u64)
        .ok_or_else(|| Error::InvalidLayout("tensor size overflow".into()))?;
    if size != tensor.size_bytes
        || bytes_per_row > isize::MAX as usize
        || elements > isize::MAX as usize / size_of::<f32>()
    {
        return Err(Error::InvalidLayout(format!(
            "tensor byte size mismatch: {}",
            tensor.name
        )));
    }
    Ok((encoding, elements, bytes_per_row, rows))
}

#[cfg(test)]
mod tests;
