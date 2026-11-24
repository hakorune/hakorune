//! Parts - Lowerer-only dispatch entry
//!
//! SSOT: docs/development/current/main/design/recipe-tree-and-parts-ssot.md
//!
//! Phase: M5 active (SSOT for stmt/exit/if/verify lowering)
//!
//! Implemented modules:
//! - stmt.rs: Stmt lowering (return prelude)
//! - exit.rs: Exit lowering (Return/Break/Continue with PHI args)
//! - if_.rs: If lowering (exit-if tree)
//! - verify.rs: RecipeVerifier
//! - dispatch/: RecipeBlock dispatch (M5m-2) - modularized
//!   - block.rs: Core block lowering
//!   - if_join.rs: If-join lowering
//!   - if_exit_only.rs: Exit-only if lowering
//!
//! Future modules:
//! - seq.rs: Seq lowering
//! - loop.rs: Loop skeleton assembly
//! - join.rs: JoinPayload generation

pub(super) mod associated_source;
mod dispatch;
pub(in crate::mir::builder) mod entry;
pub(super) mod exit;
pub(super) mod exit_branch;
pub(super) mod exit_kind_depth_view;
pub(super) mod if_;
pub(super) mod if_exit;
pub(super) mod join_scope;
mod loop_;

#[cfg(test)]
pub(crate) fn reset_loop_physical_effect_probe() {
    loop_::test_probe::reset();
}

#[cfg(test)]
pub(crate) fn take_loop_physical_effect_probe() -> usize {
    loop_::test_probe::take()
}
pub(super) mod stmt;
mod stmt_shape;
pub(in crate::mir::builder::control_flow::plan) mod var_map_scope;
pub(super) mod verify;
mod verify_refs;
mod verify_shape;

pub(in crate::mir::builder) use associated_source::callable_loop_source::CallableLoopSourcePartsBlockV1;
pub(in crate::mir::builder) use associated_source::callable_loop_source_lowering::lower_callable_loop_source_parts_block;
pub(in crate::mir::builder) use associated_source::dispatch::PartsAssociatedBlockModeV1;

#[cfg(test)]
mod wiring_tests;
