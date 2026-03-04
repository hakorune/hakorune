//! Instruction Rewriter - Modularized
//!
//! Phase 260 P0.1: Boxed modularization of instruction_rewriter.rs
//! Split large file (1477 lines) into focused, single-responsibility modules.
//!
//! ## SSOT: stages/* as Entry Point (Phase 287)
//!
//! **Primary entry point**: `stages/` (2-stage Plan→Apply pipeline)
//! - Phase 287 P6: Removed Scan stage (Plan→Apply only)
//! - Phase 287 P5: Unified API through `stages/mod.rs` facade
//!
//! ## Module Structure
//!
//! - `stages/`: Pipeline SSOT (plan_rewrites, apply_rewrites)
//! - `plan_box`: Data structures (RewrittenBlocks)
//! - `terminator`: Terminator remapping (Branch/Jump/Return conversion)
//! - `helpers`: Small pure functions (is_skippable_continuation)
//!
//! ## Public API (re-exported)
//!
//! - `is_skippable_continuation`: Structural check for skippable continuation functions
//! - `merge_and_rewrite`: Main orchestrator for JoinIR→MIR merge

// Phase 260 P0.1 Step 1: Forward all declarations to parent instruction_rewriter.rs
// This allows gradual migration without breaking existing code.
//
// Modules (extracted):
// - helpers: Small pure functions (is_skippable_continuation) ✅
// - terminator: Jump/Branch remapping ✅

// Future modules (pending):
// - k_exit_handling: k_exit special handling
// - tail_call: Parameter binding + latch incoming
//
// For now, re-export from parent to maintain compatibility.

pub(super) mod carrier_inputs_collector; // Phase 286C-5 Step 1: DRY carrier_inputs collection ✅
pub(super) mod helpers; // Phase 260 P0.1 Step 3: Helpers extracted ✅
pub(super) mod instruction_filter_box; // Phase 286C-2 Step 2: Skip judgment logic extracted ✅
pub(super) mod latch_incoming_recorder; // Phase 287 P2: latch recording SSOT ✅
pub(super) mod return_converter_box; // Phase 286C-2 Step 2: Return → Jump conversion helpers ✅
pub(super) mod rewrite_context; // Phase 286C-3: State consolidation ✅
pub(super) mod tail_call_policy; // Phase 29ae: Tail call policy SSOT ✅
pub(super) mod terminator; // Phase 260 P0.1 Step 5: Terminator remapping extracted ✅

// Phase 287 P7: Removed unused Box scaffolding (apply_box, tail_call_detector_box, parameter_binding_box)
// - apply_box.rs: Stub for Stage 3 (actual logic in stages/apply.rs)
// - tail_call_detector_box.rs: Test-only helpers
// - parameter_binding_box.rs: Test-only helpers

// Phase 286C-2.1: pipeline (Plan → Apply)
pub(super) mod plan_box; // Stage 2: Data structures (RewrittenBlocks - SSOT)
pub(super) mod plan_helpers; // Helper functions for plan_rewrites()

// Phase 287 P3: Pipeline functions extracted to separate files
pub(super) mod stages; // scan_blocks(), plan_rewrites(), apply_rewrites()

// Re-export public API
 // Phase 260 P0.1 Step 3: From helpers ✅
pub(super) use super::instruction_rewriter::merge_and_rewrite; // Still in parent (TODO: extract)
