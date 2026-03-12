//! JoinIR MIR Block Merging Coordinator
//!
//! This module coordinates the merging of JoinIR-generated MIR functions
//! into the host MIR builder. The process is broken into 6 phases:
//!
//! 1. Block ID allocation (block_allocator.rs)
//! 2. Value collection (value_collector.rs)
//! 3. ValueId remapping (uses JoinIrIdRemapper)
//! 4. Instruction rewriting (instruction_rewriter.rs)
//! 5. Exit PHI construction (exit_phi_builder.rs)
//! 6. Boundary reconnection (coordinator.rs)
//!
//! Phase 4 Refactoring: Breaking down 714-line merge_joinir_mir_blocks() into focused modules

mod block_allocator;
mod block_remapper; // Phase 284 P1: Block ID remap SSOT
mod boundary_carrier_layout; // Phase 29af P3: Carrier order SSOT
mod boundary_logging; // Phase 287 P0.5: Boundary logging consolidation
mod carrier_init_builder;
mod config;
mod contract_checks; // Phase 256 P1.5-DBG: Exposed for patterns to access verify_boundary_entry_params
mod coordinator;
mod debug_assertions; // Phase 286C-4.3: Debug-only assertions (split from contract_checks)
mod dev_log; // Phase 29ae: Dev logging SSOT
mod entry_selector; // Phase 287 P0.3: Entry function selection (SSOT)
pub mod exit_args_collector; // Phase 118: Exit args collection box
pub mod exit_line;
mod exit_phi_builder;
mod expr_result_resolver;
mod header_phi_prebuild; // Phase 287 P0.4: Loop header PHI pre-build orchestration
mod header_pred_policy; // Phase 29ae P1: Header pred SSOT
mod instruction_rewriter; // Phase 260 P0.1: Keep for gradual migration
mod loop_header_phi_builder;
mod loop_header_phi_info;
mod merge_result;
mod phi_block_remapper; // Phase 94: Phi block-id remap box
mod rewriter; // Phase 260 P0.1: New modularized rewriter (forwards to instruction_rewriter)
mod tail_call_classifier;
mod value_collector;
mod value_remapper; // Phase 287 P0.2: ValueId remapping helper

#[cfg(test)]
mod tests; // Phase 132-R0 Task 3: Continuation contract tests

use crate::mir::builder::control_flow::joinir::trace;

// Phase 33-17: Re-export for use by other modules
pub(in crate::mir::builder) use contract_checks::run_all_pipeline_checks;
pub use loop_header_phi_builder::LoopHeaderPhiBuilder;
pub use loop_header_phi_info::LoopHeaderPhiInfo;
// Phase 131 P1 Task 1: Re-export MergeContracts for SSOT visibility
pub(in crate::mir::builder) use coordinator::merge_joinir_mir_blocks;
pub use merge_result::MergeContracts;
