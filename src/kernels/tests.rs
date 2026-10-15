use super::*;

fn available() -> Vec<Kernels> {
    let mut kernels = vec![Kernels::new(Backend::Scalar).unwrap()];
    if let Ok(native) = Kernels::new(Backend::Power9) {
        kernels.push(native);
    }
    kernels
}

fn reference(w: &[i8; BLOCK_SIZE], a: &[i8; BLOCK_SIZE]) -> i32 {
    w.iter()
        .zip(a)
        .map(|(&w, &a)| i32::from(w) * i32::from(a))
        .sum()
}

#[test]
fn extreme_values_and_bias_corrections() {
    for kernels in available() {
        for av in [-128, -127, -1, 0, 1, 127] {
            let a = [av; BLOCK_SIZE];
            let activation = ActivationBlock::from_quantized(&a, 1.0).unwrap();
            for wv in [-128, -1, 0, 1, 127] {
                let w = [wv; BLOCK_SIZE];
                let packed = Q8WeightBlock::from_quantized(&w, 1.0).unwrap();
                assert_eq!(
                    kernels.q8_block_dot(&packed, &activation),
                    reference(&w, &a)
                );
            }
            for wv in [-8, -1, 0, 1, 7] {
                let w = [wv; BLOCK_SIZE];
                let packed = Q4WeightBlock::from_quantized(&w, 1.0).unwrap();
                assert_eq!(
                    kernels.q4_block_dot(&packed, &activation),
                    reference(&w, &a)
                );
            }
        }
    }
}

#[test]
fn random_blocks_match_independent_reference() {
    let mut state = 0x1234_5678_u32;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        state as u8
    };
    for kernels in available() {
        for _ in 0..10_000 {
            let a = std::array::from_fn(|_| next() as i8);
            let w8 = std::array::from_fn(|_| next() as i8);
            let w4 = std::array::from_fn(|_| (next() & 15) as i8 - 8);
            let activation = ActivationBlock::from_quantized(&a, 1.0).unwrap();
            let q8 = Q8WeightBlock::from_quantized(&w8, 1.0).unwrap();
            let q4 = Q4WeightBlock::from_quantized(&w4, 1.0).unwrap();
            assert_eq!(kernels.q8_block_dot(&q8, &activation), reference(&w8, &a));
            assert_eq!(kernels.q4_block_dot(&q4, &activation), reference(&w4, &a));
        }
    }
}

#[test]
fn row_dots_apply_per_block_scales_and_handle_unaligned_blocks() {
    let mut activations = Vec::new();
    let mut q8 = Vec::new();
    let mut q4 = Vec::new();
    let mut expected = 0.0;
    for b in 0..17 {
        let a = std::array::from_fn(|i| (i as i8).wrapping_mul(7));
        let w = std::array::from_fn(|i| ((i + b) % 16) as i8 - 8);
        let ad = (b + 1) as f32 / 32.0;
        let wd = if b % 2 == 0 { 0.25 } else { -0.125 };
        activations.push(ActivationBlock::from_quantized(&a, ad).unwrap());
        q8.push(Q8WeightBlock::from_quantized(&w, wd).unwrap());
        q4.push(Q4WeightBlock::from_quantized(&w, wd).unwrap());
        expected += reference(&w, &a) as f32 * (ad * wd);
    }
    for k in available() {
        assert_eq!(k.q8_dot(&q8, &activations).unwrap(), expected);
        assert_eq!(k.q4_dot(&q4, &activations).unwrap(), expected);
        assert_eq!(k.q8_dot(&[], &[]).unwrap(), 0.0);
        assert!(k.q8_dot(&q8[..1], &activations).is_err());
        assert!(k.q4_dot(&q4[..1], &activations).is_err());
    }
}

#[test]
fn reject_invalid_blocks_without_exposing_mutation() {
    for bad in [-128, -9, 8, 127] {
        let mut q = [0; BLOCK_SIZE];
        q[31] = bad;
        assert!(matches!(
            Q4WeightBlock::from_quantized(&q, 1.0),
            Err(Error::Q4OutOfRange)
        ));
    }
    for scale in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(ActivationBlock::from_quantized(&[0; BLOCK_SIZE], scale).is_err());
        assert!(Q8WeightBlock::from_quantized(&[0; BLOCK_SIZE], scale).is_err());
        assert!(Q4WeightBlock::from_quantized(&[0; BLOCK_SIZE], scale).is_err());
    }
}

#[test]
fn q4_nibble_order_is_split_half_not_interleaved() {
    let q = std::array::from_fn(|i| (i % 16) as i8 - 8);
    let packed = Q4WeightBlock::from_quantized(&q, 1.0).unwrap();
    for i in 0..16 {
        assert_eq!(packed.0.q[i], (i as u8) | ((i as u8) << 4));
    }
}

#[test]
fn automatic_dispatch_selects_native_when_available() {
    let auto = Kernels::new(Backend::Auto).unwrap();
    let expected = Kernels::new(Backend::Power9).map_or("scalar", |k| k.name());
    assert_eq!(auto.name(), expected);
}
