use crate::kernels::portable::dequant::p9_decode_row;

unsafe extern "C" {
    fn p9_decode_row_vsx(
        ty: i32,
        input: *const u8,
        bytes: usize,
        out: *mut f32,
        count: usize,
    ) -> bool;
    fn p9_all_finite_vsx(values: *const f32, count: usize) -> i32;
}

#[test]
fn embedded_decoder_matches_portable() {
    for (ty, elements, block_bytes, scales) in [
        (0, 1, 4, &[][..]),
        (1, 1, 2, &[][..]),
        (2, 32, 18, &[0][..]),
        (6, 32, 22, &[0][..]),
        (8, 32, 34, &[0][..]),
        (12, 256, 144, &[0, 2][..]),
        (14, 256, 210, &[208][..]),
        (35, 256, 66, &[0][..]),
    ] {
        for blocks in [0, 1, 2, 3, 17] {
            for seed in 0..32usize {
                let bytes = blocks * block_bytes;
                let count = blocks * elements;
                let mut input: Vec<u8> = (0..bytes + 1)
                    .map(|i| (seed * 131 + i * 73) as u8)
                    .collect();
                for block in 0..blocks {
                    for &offset in scales {
                        // Keep scales finite; cover wide exponents, signs, and subnormals.
                        input[1 + block * block_bytes + offset + 1] &= 0x7b;
                    }
                }
                let guard = f32::from_bits(0x7fc01234);
                let mut asm = vec![guard; count + 2];
                let mut clang = asm.clone();
                let mut portable = asm.clone();
                // SAFETY: input includes every block; outputs include guards.
                unsafe {
                    let p = input.as_ptr().add(1);
                    assert!(p9_decode_row_vsx(
                        ty,
                        p,
                        bytes,
                        asm.as_mut_ptr().add(1),
                        count
                    ));
                    assert!(p9_decode_row(
                        ty,
                        p,
                        bytes,
                        clang.as_mut_ptr().add(1),
                        count
                    ));
                    assert!(p9_decode_row(
                        ty,
                        p,
                        bytes,
                        portable.as_mut_ptr().add(1),
                        count
                    ));
                }
                for i in 0..asm.len() {
                    assert_eq!(
                        asm[i].to_bits(),
                        clang[i].to_bits(),
                        "clang type={ty} blocks={blocks} seed={seed} cell={i}"
                    );
                    assert_eq!(
                        asm[i].to_bits(),
                        portable[i].to_bits(),
                        "portable type={ty} blocks={blocks} seed={seed} cell={i}"
                    );
                }
                if bytes != 0 {
                    let before: Vec<u32> = asm.iter().map(|v| v.to_bits()).collect();
                    // SAFETY: malformed size is rejected before any buffer access.
                    unsafe {
                        assert!(!p9_decode_row_vsx(
                            ty,
                            input.as_ptr().add(1),
                            bytes - 1,
                            asm.as_mut_ptr().add(1),
                            count
                        ));
                    }
                    assert_eq!(before, asm.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
                }
            }
        }
    }
    let mut output = [f32::from_bits(0x7fc01234); 4];
    // SAFETY: invalid types and block shapes reject before accessing null input.
    unsafe {
        for ty in [-1, 3, 99] {
            assert!(!p9_decode_row_vsx(
                ty,
                core::ptr::null(),
                0,
                output.as_mut_ptr(),
                0
            ));
        }
        assert!(!p9_decode_row_vsx(
            2,
            core::ptr::null(),
            0,
            output.as_mut_ptr(),
            31
        ));
        assert!(!p9_decode_row_vsx(
            2,
            core::ptr::null(),
            0,
            output.as_mut_ptr(),
            usize::MAX - 31
        ));
    }
}

#[test]
fn embedded_decoder_preserves_every_half_pattern() {
    for h in 0..=u16::MAX {
        let mut input = [0u8; 18];
        input[..2].copy_from_slice(&h.to_le_bytes());
        for (i, v) in input[2..].iter_mut().enumerate() {
            *v = (i * 17) as u8;
        }
        let mut asm = [0.0; 32];
        let mut clang = [0.0; 32];
        // SAFETY: one complete Q4_0 block, including nonfinite half scales.
        unsafe {
            assert!(p9_decode_row_vsx(
                2,
                input.as_ptr(),
                18,
                asm.as_mut_ptr(),
                32
            ));
            assert!(p9_decode_row(2, input.as_ptr(), 18, clang.as_mut_ptr(), 32));
        }
        assert_eq!(
            asm.map(f32::to_bits),
            clang.map(f32::to_bits),
            "half={h:04x}"
        );
    }
}

#[test]
fn embedded_finite_scan_matches_rust() {
    let bits = [
        0, 0x80000000, 1, 0x80000001, 0x00800000, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000,
        0x7fc00123, 0x7f800001, 0xff800001,
    ];
    for count in [0, 1, 2, 3, 4, 5, 7, 8, 9, 31, 32, 33, 257] {
        for bad in 0..=count {
            for &b in &bits {
                let mut values = vec![1.0; count + 1];
                if bad < count {
                    values[bad + 1] = f32::from_bits(b);
                }
                let expected = i32::from(values[1..].iter().all(|v| v.is_finite()));
                // SAFETY: offset buffer covers count floats, including zero length.
                unsafe {
                    let p = values.as_ptr().add(1);
                    assert_eq!(p9_all_finite_vsx(p, count), expected);
                }
            }
        }
    }
}
