//! Gated DeltaNet head update (Qwen3.5/3.6 linear attention), binary32.
//!
//! For every state row `m[j]` (`m[j][i] = S[i][j]`, `n` columns):
//! `m[j] *= decay`, `sk = dot8(m[j], k)`, `delta = (v[j] - sk) * beta`,
//! `m[j] += k * delta`, `out[j] = dot8(m[j], q) * scale`, where `dot8` keeps
//! 8 partial sums (column mod 8) added in column order and combines them as
//! `((p0+p1)+(p2+p3)) + ((p4+p5)+(p6+p7))`. No operation is fused; the POWER9
//! assembly (`asm/deltanet/deltanet_rows_power9.s`) gives the same bits.

/// Arguments of the assembly (field offsets are its ABI).
#[cfg(all(feature = "power9", target_arch = "powerpc64", target_os = "linux"))]
#[repr(C)]
struct Args {
    state: *mut f32,
    rows: usize,
    n: usize,
    k: *const f32,
    q: *const f32,
    v: *const f32,
    out: *mut f32,
    params: *const f32,
}

#[cfg(all(feature = "power9", target_arch = "powerpc64", target_os = "linux"))]
#[unsafe(naked)]
unsafe extern "C" fn deltanet_rows_power9(_args: *const Args) {
    core::arch::naked_asm!(
        ".machine push\n.machine power9",
        include_str!("asm/deltanet/deltanet_rows_power9.s"),
        ".machine pop",
    )
}

/// Updates one head's `state` (`v.len()` rows of `k.len()` columns) and
/// writes its read-out to `out`.
#[allow(clippy::too_many_arguments)]
pub fn head_update(
    state: &mut [f32],
    k: &[f32],
    q: &[f32],
    v: &[f32],
    decay: f32,
    beta: f32,
    scale: f32,
    out: &mut [f32],
) {
    let (n, rows) = (k.len(), v.len());
    assert!(q.len() == n && out.len() == rows && state.len() == rows * n);
    #[cfg(all(feature = "power9", target_arch = "powerpc64", target_os = "linux"))]
    if n % 8 == 0 && rows % 4 == 0 && rows > 0 && n > 0 && native() {
        let params = [decay, beta, scale];
        let args = Args {
            state: state.as_mut_ptr(),
            rows,
            n,
            k: k.as_ptr(),
            q: q.as_ptr(),
            v: v.as_ptr(),
            out: out.as_mut_ptr(),
            params: params.as_ptr(),
        };
        // SAFETY: POWER9 is present; the slices have the sizes asserted
        // above and rows / n meet the kernel's multiples.
        unsafe { deltanet_rows_power9(&args) };
        return;
    }
    head_update_portable(state, k, q, v, decay, beta, scale, out);
}

/// Whether the assembly runs (`POWER_LLM_DELTANET_ASM=0` keeps Rust).
#[cfg(all(feature = "power9", target_arch = "powerpc64", target_os = "linux"))]
fn native() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        super::portable::quant::have_power9()
            && std::env::var("POWER_LLM_DELTANET_ASM").map_or(true, |v| v != "0")
    })
}

/// The reference implementation of [`head_update`].
#[allow(clippy::too_many_arguments)]
pub fn head_update_portable(
    state: &mut [f32],
    k: &[f32],
    q: &[f32],
    v: &[f32],
    decay: f32,
    beta: f32,
    scale: f32,
    out: &mut [f32],
) {
    let n = k.len();
    for ((row, &vj), o) in state.chunks_exact_mut(n).zip(v).zip(out.iter_mut()) {
        let sk = scale_dot8(row, decay, k);
        let delta = (vj - sk) * beta;
        *o = update_dot8(row, k, delta, q) * scale;
    }
}

/// `row *= decay`, returning `dot8(row, k)` of the result.
fn scale_dot8(row: &mut [f32], decay: f32, k: &[f32]) -> f32 {
    let mut partial = [0.0f32; 8];
    let (r8, r_tail) = row.as_chunks_mut::<8>();
    let (k8, k_tail) = k.as_chunks::<8>();
    for (r, k) in r8.iter_mut().zip(k8) {
        for i in 0..8 {
            r[i] *= decay;
            partial[i] += r[i] * k[i];
        }
    }
    for (i, (r, k)) in r_tail.iter_mut().zip(k_tail).enumerate() {
        *r *= decay;
        partial[i] += *r * k;
    }
    ((partial[0] + partial[1]) + (partial[2] + partial[3]))
        + ((partial[4] + partial[5]) + (partial[6] + partial[7]))
}

/// `row += k * delta`, returning `dot8(row, q)` of the result.
fn update_dot8(row: &mut [f32], k: &[f32], delta: f32, q: &[f32]) -> f32 {
    let mut partial = [0.0f32; 8];
    let (r8, r_tail) = row.as_chunks_mut::<8>();
    let (k8, k_tail) = k.as_chunks::<8>();
    let (q8, q_tail) = q.as_chunks::<8>();
    for ((r, k), q) in r8.iter_mut().zip(k8).zip(q8) {
        for i in 0..8 {
            r[i] += k[i] * delta;
            partial[i] += r[i] * q[i];
        }
    }
    for (i, ((r, k), q)) in r_tail.iter_mut().zip(k_tail).zip(q_tail).enumerate() {
        *r += k * delta;
        partial[i] += *r * q;
    }
    ((partial[0] + partial[1]) + (partial[2] + partial[3]))
        + ((partial[4] + partial[5]) + (partial[6] + partial[7]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(seed: &mut u64, count: usize, spread: f32) -> Vec<f32> {
        (0..count)
            .map(|_| {
                *seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((*seed >> 40) as f32 / (1u64 << 24) as f32 - 0.5) * spread
            })
            .collect()
    }

    /// The dispatched update (assembly on POWER9) equals the reference bit
    /// for bit, state and read-out, over repeated steps.
    #[test]
    fn head_update_matches_portable() {
        for (rows, n) in [(128, 128), (4, 8), (16, 40), (12, 64)] {
            let mut seed = (rows * 1000 + n) as u64;
            let mut a = values(&mut seed, rows * n, 2.0);
            let mut b = a.clone();
            for step in 0..5 {
                let k = values(&mut seed, n, 0.3);
                let q = values(&mut seed, n, 0.3);
                let v = values(&mut seed, rows, 4.0);
                let (decay, beta, scale) = (0.9 + 0.01 * step as f32, 0.37, 0.088);
                let (mut oa, mut ob) = (vec![0.0f32; rows], vec![0.0f32; rows]);
                head_update(&mut a, &k, &q, &v, decay, beta, scale, &mut oa);
                head_update_portable(&mut b, &k, &q, &v, decay, beta, scale, &mut ob);
                let bits = |x: &[f32]| x.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
                assert_eq!(bits(&oa), bits(&ob), "out {rows}x{n} step {step}");
                assert_eq!(bits(&a), bits(&b), "state {rows}x{n} step {step}");
            }
        }
    }
}
