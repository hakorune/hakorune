//! Common utilities for Normalized shadow (Phase 138+)

pub mod expr_lowerer_box;
pub mod expr_lowering_contract;
pub mod known_intrinsics; // Phase 141 P1.5
pub mod loop_if_exit_contract; // Phase 143 R0: Contract SSOT for loop-if-exit patterns
pub mod normalized_helpers;
pub mod return_value_lowerer_box; // Phase 143.5: Common helper functions for Normalized lowering
pub mod route_function_names;
