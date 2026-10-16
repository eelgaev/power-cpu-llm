//! Portable decoding of the on-disk types present in the target GGUFs.
//! This is a correctness path, not an optimized quantized matrix kernel.

use std::{ffi::c_int, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Encoding {
    F32 = 0,
    /// IEEE binary16 (E2B mobile `per_layer_model_proj`); exact paths only.
    F16 = 1,
    Q4_0 = 2,
    Q5_0 = 6,
    /// 32 values q * d + m, q in 0..31 (Nemotron 3.5 experts); exact paths only.
    Q5_1 = 7,
    Q8_0 = 8,
    Q4K = 12,
    /// 256 values `d * sc * q - dmin * m`, q in 0..31 (Q4_K plus a high
    /// bit); exact paths only.
    Q5K = 13,
    Q6K = 14,
    /// Ternary: 256 values (q - 1) * d, q in 0..3 (E2B mobile MLPs).
    TQ2_0 = 35,
    /// bfloat16; exact paths only.
    BF16 = 30,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    UnsupportedType(i32),
    InvalidElementCount,
    WrongByteCount { expected: usize, actual: usize },
    NativeDecodeFailed,
    NonFiniteValue,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

unsafe extern "C" {
    fn p9_decode_row(
        kind: c_int,
        input: *const u8,
        size: usize,
        out: *mut f32,
        count: usize,
    ) -> bool;
}

impl TryFrom<i32> for Encoding {
    type Error = Error;
    fn try_from(id: i32) -> Result<Self, Error> {
        match id {
            0 => Ok(Self::F32),
            1 => Ok(Self::F16),
            35 => Ok(Self::TQ2_0),
            2 => Ok(Self::Q4_0),
            6 => Ok(Self::Q5_0),
            7 => Ok(Self::Q5_1),
            30 => Ok(Self::BF16),
            8 => Ok(Self::Q8_0),
            12 => Ok(Self::Q4K),
            13 => Ok(Self::Q5K),
            14 => Ok(Self::Q6K),
            _ => Err(Error::UnsupportedType(id)),
        }
    }
}

impl Encoding {
    pub fn block_layout(self) -> (usize, usize) {
        match self {
            Self::F32 => (1, 4),
            Self::F16 => (1, 2),
            Self::TQ2_0 => (256, 66),
            Self::Q4_0 => (32, 18),
            Self::Q5_0 => (32, 22),
            Self::Q5_1 => (32, 24),
            Self::BF16 => (1, 2),
            Self::Q8_0 => (32, 34),
            Self::Q4K => (256, 144),
            Self::Q5K => (256, 176),
            Self::Q6K => (256, 210),
        }
    }
    pub fn row_bytes(self, count: usize) -> Result<usize, Error> {
        let (elements, bytes) = self.block_layout();
        if !count.is_multiple_of(elements) {
            return Err(Error::InvalidElementCount);
        }
        (count / elements)
            .checked_mul(bytes)
            .ok_or(Error::InvalidElementCount)
    }
    pub fn decode(self, bytes: &[u8], count: usize) -> Result<Vec<f32>, Error> {
        let expected = self.row_bytes(count)?;
        if count > isize::MAX as usize / size_of::<f32>() {
            return Err(Error::InvalidElementCount);
        }
        if bytes.len() != expected {
            return Err(Error::WrongByteCount {
                expected,
                actual: bytes.len(),
            });
        }
        let mut output = vec![0.0; count];
        // SAFETY: exact encoded size, valid type, non-overlapping input/output,
        // and count initialized F32 elements. Decoder supports unaligned bytes.
        if !unsafe {
            p9_decode_row(
                self as c_int,
                bytes.as_ptr(),
                bytes.len(),
                output.as_mut_ptr(),
                count,
            )
        } {
            return Err(Error::NativeDecodeFailed);
        }
        if output.iter().any(|x| !x.is_finite()) {
            return Err(Error::NonFiniteValue);
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_is_little_endian_and_checked() {
        let bytes: Vec<u8> = [1.0_f32, -2.5, 0.0]
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .collect();
        assert_eq!(Encoding::F32.decode(&bytes, 3).unwrap(), [1.0, -2.5, 0.0]);
        assert_eq!(
            Encoding::F32
                .decode(&f32::NAN.to_le_bytes(), 1)
                .unwrap_err(),
            Error::NonFiniteValue
        );
        assert!(Encoding::Q4K.decode(&[0; 144], 32).is_err());
        assert!(Encoding::Q8_0.decode(&[0; 33], 32).is_err());
        assert!(Encoding::try_from(999).is_err());
        assert_eq!(Encoding::Q6K.decode(&[], 0).unwrap(), Vec::<f32>::new());
    }

    #[test]
    fn half_scales_cover_subnormals_sign_and_nonfinite() {
        for (half, expected) in [
            (0x3c00_u16, 1.0),
            (0xbc00, -1.0),
            (0x0001, 2.0_f32.powi(-24)),
        ] {
            let mut bytes = vec![1; 34];
            bytes[..2].copy_from_slice(&half.to_le_bytes());
            assert_eq!(Encoding::Q8_0.decode(&bytes, 32).unwrap(), [expected; 32]);
        }
        let mut bytes = vec![1; 34];
        bytes[..2].copy_from_slice(&0x7c00_u16.to_le_bytes());
        assert_eq!(
            Encoding::Q8_0.decode(&bytes, 32).unwrap_err(),
            Error::NonFiniteValue
        );
    }

    #[test]
    fn q5_high_bits_and_split_nibbles_are_independently_indexed() {
        let mut bytes = vec![0; 22];
        bytes[..2].copy_from_slice(&0x3c00_u16.to_le_bytes());
        bytes[2..6].copy_from_slice(&((1u32 << 0) | (1u32 << 31)).to_le_bytes());
        bytes[6..].fill(0x72);
        let decoded = Encoding::Q5_0.decode(&bytes, 32).unwrap();
        for (i, &x) in decoded.iter().enumerate() {
            let high = if i == 0 || i == 31 { 16 } else { 0 };
            let nibble = if i < 16 { 2 } else { 7 };
            assert_eq!(x, (nibble + high - 16) as f32);
        }
    }

    #[test]
    fn q5_1_adds_the_block_minimum_and_bf16_is_the_high_half() {
        let mut bytes = vec![0; 24];
        bytes[..2].copy_from_slice(&0x3c00_u16.to_le_bytes()); // d=1
        bytes[2..4].copy_from_slice(&0xb800_u16.to_le_bytes()); // m=-0.5
        bytes[4..8].copy_from_slice(&((1u32 << 1) | (1u32 << 30)).to_le_bytes());
        bytes[8..].fill(0x93);
        let decoded = Encoding::Q5_1.decode(&bytes, 32).unwrap();
        for (i, &x) in decoded.iter().enumerate() {
            let high = if i == 1 || i == 30 { 16 } else { 0 };
            let nibble = if i < 16 { 3 } else { 9 };
            assert_eq!(x, (nibble + high) as f32 - 0.5);
        }
        let bytes = [0x80, 0x3f, 0x20, 0xc0];
        assert_eq!(Encoding::BF16.decode(&bytes, 2).unwrap(), [1.0, -2.5]);
    }

    #[test]
    fn q4_k_scale_and_min_are_applied_per_32_element_group() {
        let mut bytes = vec![0; 144];
        bytes[..2].copy_from_slice(&0x3c00_u16.to_le_bytes()); // d=1
        bytes[2..4].copy_from_slice(&0x3800_u16.to_le_bytes()); // dmin=0.5
        // All eight 6-bit scales/mins are 1/2. Last four packed into nibbles.
        bytes[4..8].fill(1);
        bytes[8..12].fill(2);
        bytes[12..16].fill(0x21);
        bytes[16..].fill(0x73);
        let decoded = Encoding::Q4K.decode(&bytes, 256).unwrap();
        for (i, &x) in decoded.iter().enumerate() {
            assert_eq!(x, if (i / 32) % 2 == 0 { 2.0 } else { 6.0 });
        }
    }

    #[test]
    fn q6_k_signed_scales_and_quarter_order() {
        let mut bytes = vec![0; 210];
        bytes[..128].fill(0x52);
        bytes[128..192].fill(0b11_10_01_00);
        for (i, x) in bytes[192..208].iter_mut().enumerate() {
            *x = (i as i8 - 8) as u8;
        }
        bytes[208..].copy_from_slice(&0x3c00_u16.to_le_bytes());
        let decoded = Encoding::Q6K.decode(&bytes, 256).unwrap();
        for (i, &x) in decoded.iter().enumerate() {
            let group = (i % 128) / 32;
            let low = if group < 2 { 2 } else { 5 };
            let q = low + (group as i32 * 16) - 32;
            let scale = i as i32 / 16 - 8;
            assert_eq!(x, (q * scale) as f32);
        }
    }
}
