use super::*;
use crate::kernels::RawActivation;
use crate::kernels::portable::quant::{Activation, p9_q4_scalar, p9_q8_scalar};
use crate::kernels::portable::reference::p9_ref_matvec_multi;

#[test]
fn embedded_block_dots_match_portable() {
    for seed in 0..1024usize {
        let weights: Vec<u8> = (0..33).map(|i| (seed * 193 + i * 71) as u8).collect();
        let q = std::array::from_fn(|i| (seed * 61 + i * 137) as i8);
        let a = RawActivation {
            d: 1.0,
            sum: q.iter().map(|&v| i32::from(v)).sum(),
            q,
        };
        let ap = (&a as *const RawActivation).cast();
        let portable = (&a as *const RawActivation).cast::<Activation>();
        // SAFETY: weights cover 32 bytes (Q4 needs 16); both activation
        // types have the same C layout.
        unsafe {
            let w = weights.as_ptr().add(1);
            assert_eq!(p9_q4_power9(w, ap), p9_q4_scalar(w, portable));
            assert_eq!(p9_q8_power9(w, ap), p9_q8_scalar(w, portable));
        }
    }
}

#[test]
fn embedded_router_matches_portable() {
    let values = [
        0, 0x80000000, 1, 0x80000001, 0x00800000, 0x80800000, 0x3f800000, 0xbf800000, 0x3f800001,
        0x7f7fffff, 0xff7fffff, 0x35000001,
    ];
    for inputs in [0, 1, 2, 3, 7, 8, 9, 16, 17] {
        for rows in [0, 1, 7, 8, 9, 16, 19] {
            for cols in [0, 1, 2, 7, 8, 9, 17, 33, 128] {
                let matrix: Vec<f32> = (0..rows * cols)
                    .map(|i| f32::from_bits(values[(i * 7 + rows) % values.len()]))
                    .collect();
                let x: Vec<f32> = (0..inputs * cols)
                    .map(|i| f32::from_bits(values[(i * 5 + inputs) % values.len()]))
                    .collect();
                let mut blocked = vec![0.0; rows * cols];
                for row in (0..rows / 8 * 8).step_by(8) {
                    for col in 0..cols {
                        for r in 0..8 {
                            blocked[row * cols + col * 8 + r] = matrix[(row + r) * cols + col];
                        }
                    }
                }
                let sentinel = f32::from_bits(0x7fc00123);
                let mut asm = vec![sentinel; inputs * rows + 3];
                let mut portable = asm.clone();
                let guard = f64::from_bits(0x7ff8000000000123);
                let mut scratch = vec![guard; 8 * cols + 2];
                // SAFETY: matrices/inputs cover the requested dimensions, outputs
                // include guards and scratch covers 8*cols doubles. Empty shapes
                // take the no-read paths. Scratch starts 8 bytes unaligned.
                unsafe {
                    p9_ref_matvec_multi_power9(
                        matrix.as_ptr(),
                        blocked.as_ptr(),
                        x.as_ptr(),
                        inputs,
                        rows,
                        cols,
                        asm.as_mut_ptr().add(1),
                        scratch.as_mut_ptr().add(1),
                    );
                    if inputs * rows > 0 {
                        p9_ref_matvec_multi(
                            matrix.as_ptr(),
                            x.as_ptr(),
                            inputs,
                            rows,
                            cols,
                            portable.as_mut_ptr().add(1),
                        );
                    }
                }
                for i in 0..asm.len() {
                    assert_eq!(
                        asm[i].to_bits(),
                        portable[i].to_bits(),
                        "portable: inputs={inputs} rows={rows} cols={cols} cell={i}"
                    );
                }
                assert_eq!(scratch[0].to_bits(), guard.to_bits());
                assert_eq!(scratch[scratch.len() - 1].to_bits(), guard.to_bits());
            }
        }
    }
}

#[test]
fn embedded_slice_probe_counts_exactly() {
    for rounds in [0, 1, 2, 3, 7, 16, 101, 1000] {
        // SAFETY: the probe reads/writes registers only, with bounded loop counts.
        unsafe {
            assert_eq!(p9_slice_probe(rounds), (264 + 2880 * rounds) as u32);
        }
    }
}
