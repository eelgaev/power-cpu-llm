use crate::gguf::{ArrayValues, Error, ffi, ggml::copy_string};

// SAFETY: caller supplies a live ctx and an in-range ARRAY key. T is the
// exact primitive type of its native array storage. Avoid aligned casts:
// ggml internally stores raw arrays in byte vectors.
unsafe fn primitives<T: Copy>(ctx: *const ffi::Context, id: i64) -> Result<Vec<T>, Error> {
    let len = unsafe { ffi::gguf_get_arr_n(ctx, id) };
    if len > isize::MAX as usize / size_of::<T>() {
        return Err(Error::InvalidLayout(
            "metadata array byte size overflows".into(),
        ));
    }
    if len == 0 {
        return Ok(Vec::new());
    }
    let ptr = unsafe { ffi::gguf_get_arr_data(ctx, id) }.cast::<T>();
    if ptr.is_null() {
        return Err(Error::InvalidLayout("NULL metadata array".into()));
    }
    Ok((0..len)
        .map(|i| unsafe { ptr.add(i).read_unaligned() })
        .collect())
}

// SAFETY: ctx is live and id is an ARRAY key; select the matching typed reader
// only after inspecting its actual type. Strings are copied before ctx drops.
pub(crate) unsafe fn copy_array(ctx: *const ffi::Context, id: i64) -> Result<ArrayValues, Error> {
    unsafe {
        Ok(match ffi::gguf_get_arr_type(ctx, id) {
            0 => ArrayValues::Unsigned(
                primitives::<u8>(ctx, id)?
                    .into_iter()
                    .map(u64::from)
                    .collect(),
            ),
            1 => ArrayValues::Signed(
                primitives::<i8>(ctx, id)?
                    .into_iter()
                    .map(i64::from)
                    .collect(),
            ),
            2 => ArrayValues::Unsigned(
                primitives::<u16>(ctx, id)?
                    .into_iter()
                    .map(u64::from)
                    .collect(),
            ),
            3 => ArrayValues::Signed(
                primitives::<i16>(ctx, id)?
                    .into_iter()
                    .map(i64::from)
                    .collect(),
            ),
            4 => ArrayValues::Unsigned(
                primitives::<u32>(ctx, id)?
                    .into_iter()
                    .map(u64::from)
                    .collect(),
            ),
            5 => ArrayValues::Signed(
                primitives::<i32>(ctx, id)?
                    .into_iter()
                    .map(i64::from)
                    .collect(),
            ),
            6 => ArrayValues::Float(
                primitives::<f32>(ctx, id)?
                    .into_iter()
                    .map(f64::from)
                    .collect(),
            ),
            7 => ArrayValues::Bool(
                primitives::<u8>(ctx, id)?
                    .into_iter()
                    .map(|v| v != 0)
                    .collect(),
            ),
            8 => {
                let len = ffi::gguf_get_arr_n(ctx, id);
                if len > isize::MAX as usize / size_of::<String>() {
                    return Err(Error::InvalidLayout("string array size overflows".into()));
                }
                ArrayValues::String(
                    (0..len)
                        .map(|i| copy_string(ffi::gguf_get_arr_str(ctx, id, i)))
                        .collect(),
                )
            }
            10 => ArrayValues::Unsigned(primitives::<u64>(ctx, id)?),
            11 => ArrayValues::Signed(primitives::<i64>(ctx, id)?),
            12 => ArrayValues::Float(primitives::<f64>(ctx, id)?),
            kind => {
                return Err(Error::InvalidLayout(format!(
                    "unsupported array element type {kind}"
                )));
            }
        })
    }
}
