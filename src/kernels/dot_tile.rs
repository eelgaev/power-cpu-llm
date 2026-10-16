//! Binary32 dots of 4 rows x 4 inputs at once, as [`dot_f32`] of each pair:
//! eight partial sums (column mod 8), each a separate product then sum in
//! column order, combined as `((p0+p1)+(p2+p3)) + ((p4+p5)+(p6+p7))`. The
//! POWER9 assembly (`asm/dot/dot_f32_tile4x4_power9.s`) forms the full
//! 8-column chunks with 32 independent accumulators; the same bits.

/// `sum(w[i] x[i])` in binary32 with eight interleaved partial sums added
/// in a fixed order.
pub fn dot_f32(w: &[f32], x: &[f32]) -> f32 {
    let mut partial = [0.0f32; 8];
    let (w8, w_tail) = w.as_chunks::<8>();
    let (x8, x_tail) = x.as_chunks::<8>();
    for (w, x) in w8.iter().zip(x8) {
        for i in 0..8 {
            partial[i] += w[i] * x[i];
        }
    }
    for (i, (w, x)) in w_tail.iter().zip(x_tail).enumerate() {
        partial[i] += w * x;
    }
    reduce(&partial)
}

fn reduce(p: &[f32; 8]) -> f32 {
    ((p[0] + p[1]) + (p[2] + p[3])) + ((p[4] + p[5]) + (p[6] + p[7]))
}

/// Arguments of the assembly (field offsets are its ABI).
#[cfg(all(feature = "power9", target_arch = "powerpc64", target_os = "linux"))]
#[repr(C)]
struct Args {
    w: *const f32,
    w_stride: usize,
    x: [*const f32; 4],
    chunks: usize,
    partial: *mut [[[f32; 8]; 4]; 4],
}

#[cfg(all(feature = "power9", target_arch = "powerpc64", target_os = "linux"))]
#[unsafe(naked)]
unsafe extern "C" fn dot_f32_tile4x4_power9(_args: *const Args) {
    core::arch::naked_asm!(
        ".machine push\n.machine power9",
        include_str!("asm/dot/dot_f32_tile4x4_power9.s"),
        ".machine pop",
    )
}

/// `out[r][p] = dot_f32(matrix row r, x[p])` for 4 consecutive rows of the
/// row-major `matrix` (`cols` = `x[p].len()` columns) starting at `row0`.
pub fn dot_f32_tile(matrix: &[f32], row0: usize, x: [&[f32]; 4]) -> [[f32; 4]; 4] {
    let cols = x[0].len();
    assert!(x.iter().all(|x| x.len() == cols) && matrix.len() >= (row0 + 4) * cols);
    let rows: [&[f32]; 4] = std::array::from_fn(|r| &matrix[(row0 + r) * cols..][..cols]);
    let mut partial = [[[0.0f32; 8]; 4]; 4];
    let full = cols / 8;
    let mut native = false;
    #[cfg(all(feature = "power9", target_arch = "powerpc64", target_os = "linux"))]
    if full > 0 && have_native() {
        let args = Args {
            w: rows[0].as_ptr(),
            w_stride: cols * 4,
            x: x.map(<[f32]>::as_ptr),
            chunks: full,
            partial: &mut partial,
        };
        // SAFETY: POWER9 is present; the 4 rows and 4 inputs each hold
        // `cols` >= 8 * chunks values (asserted above).
        unsafe { dot_f32_tile4x4_power9(&args) };
        native = true;
    }
    let mut out = [[0.0f32; 4]; 4];
    for r in 0..4 {
        for p in 0..4 {
            let partial = &mut partial[r][p];
            let (w8, w_tail) = rows[r].as_chunks::<8>();
            let (x8, x_tail) = x[p].as_chunks::<8>();
            if !native {
                for (w, x) in w8.iter().zip(x8) {
                    for i in 0..8 {
                        partial[i] += w[i] * x[i];
                    }
                }
            }
            for (i, (w, x)) in w_tail.iter().zip(x_tail).enumerate() {
                partial[i] += w * x;
            }
            out[r][p] = reduce(partial);
        }
    }
    out
}

/// Whether the assembly runs (`POWER_LLM_DOT_ASM=0` keeps Rust).
#[cfg(all(feature = "power9", target_arch = "powerpc64", target_os = "linux"))]
fn have_native() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        super::portable::quant::have_power9()
            && std::env::var("POWER_LLM_DOT_ASM").map_or(true, |v| v != "0")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_matches_dot_f32() {
        let mut seed = 7u32;
        let mut next = || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) as f32 / (1 << 24) as f32 - 0.5
        };
        for cols in [3, 8, 13, 64, 2048, 2051] {
            let matrix: Vec<f32> = (0..6 * cols).map(|_| next()).collect();
            let inputs: Vec<Vec<f32>> = (0..4)
                .map(|_| (0..cols).map(|_| next()).collect())
                .collect();
            let x: [&[f32]; 4] = std::array::from_fn(|p| inputs[p].as_slice());
            for row0 in [0, 2] {
                let tile = dot_f32_tile(&matrix, row0, x);
                for r in 0..4 {
                    for p in 0..4 {
                        let want = dot_f32(&matrix[(row0 + r) * cols..][..cols], x[p]);
                        assert_eq!(
                            tile[r][p].to_bits(),
                            want.to_bits(),
                            "cols {cols} r {r} p {p}"
                        );
                    }
                }
            }
        }
    }
}
