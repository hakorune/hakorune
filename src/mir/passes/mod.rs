// Reusable MIR transforms / analyses namespace.
// All passes take `&mut MirModule` only (no MirOptimizer coupling); safe for
// external reuse (runner/bin, LLVM backend). Boundary: see OPTIMIZER_REGISTRY.md

pub mod callsite_canonicalize;
pub mod concat3_canonicalize;
pub mod cse;
pub mod dce;
pub mod escape;
pub mod inline_soft_leaf;
pub mod memory_effect;
pub mod method_id_inject;
pub mod placement_effect_transform;
pub mod rc_insertion;
pub mod rc_insertion_helpers;
pub mod semantic_simplification;
pub mod simplify_cfg;
pub mod string_corridor_sink;
pub mod type_hints;
