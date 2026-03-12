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

pub(super) mod conditional_update;
mod dispatch;
pub(in crate::mir::builder) mod entry;
pub(super) mod exit;
pub(super) mod exit_branch;
pub(super) mod exit_kind_depth_view;
pub(super) mod if_;
pub(super) mod if_exit;
pub(super) mod if_general;
pub(super) mod join_scope;
mod loop_;
pub(super) mod stmt;
mod var_map_scope;
pub(super) mod verify;

pub(in crate::mir::builder) use loop_::LoopBodyContractKind;

#[cfg(test)]
mod wiring_tests;
