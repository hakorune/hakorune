// MirOptimizer-coupled built-in pass composition layer.
// All passes take `&mut MirOptimizer` as the first argument and are consumed
// only by the optimizer pipeline (optimizer/core.rs). Not for external reuse.
// Boundary: see OPTIMIZER_REGISTRY.md

pub mod boxfield;
pub mod diagnostics;
pub mod intrinsics;
pub mod normalize;
pub mod normalize_core13_pure;
pub mod reorder;
