//! Standalone Rust engine. Model parsing and kernel execution are separate
//! boundaries; ggml is not used as an execution runtime.

pub mod attention;
pub mod chat;
pub mod completion;
pub mod diffusion;
pub mod execution;
pub mod generation;
pub mod model;
pub mod operators;
pub mod output;
pub mod stats;
pub mod text;
pub mod tokenizer;
pub mod weights;

pub use crate::{gguf, kernels};

/// CPU placement policy for this machine. GPU-only NUMA nodes are excluded.
/// This is configuration, not yet an allocator or worker-pinning implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuPlacement {
    pub numa_nodes: Vec<u32>,
}

impl Default for CpuPlacement {
    fn default() -> Self {
        Self {
            numa_nodes: vec![0, 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_cpu_policy_excludes_gpu_nodes() {
        assert_eq!(CpuPlacement::default().numa_nodes, [0, 8]);
    }
}
