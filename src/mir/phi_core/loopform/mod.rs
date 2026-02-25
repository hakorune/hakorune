//! LoopForm PHI Construction System
//!
//! This module implements the LoopForm Meta-Box approach to PHI construction,
//! solving the ValueId circular dependency problem by treating loop structure
//! as a "Meta-Box" with explicit separation of carriers vs. pinned variables.
//!
//! # Architecture: 4-Pass PHI Generation Pipeline
//!
//! The LoopForm system constructs SSA PHI nodes in 4 distinct passes:
//!
//! ## Pass 1: prepare_structure() - Variable Discovery & ValueId Allocation
//! - Classify variables as carriers (modified in loop) or pinned (loop-invariant)
//! - Allocate ValueIds upfront for preheader_copy and header_phi
//! - Ensure counter is after all existing ValueIds
//! - **Key Innovation**: All ValueIds allocated BEFORE any MIR emission
//!
//! ## Pass 2: emit_preheader() - Preheader Copy Generation
//! - Generate copy instructions in preheader block
//! - Order: pinned variables first, then carrier variables (deterministic)
//! - Jump to header block
//!
//! ## Pass 3: emit_header_phis() - Header PHI Construction
//! - Generate incomplete PHI nodes in header block
//! - First input: preheader_copy (known)
//! - Second input: latch value (unknown at this point)
//!
//! ## Pass 4: seal_phis() - PHI Completion
//! - Complete PHI nodes by finding latch values
//! - Separate handling for pinned (seal_pinned_phis) and carrier (seal_carrier_phis)
//! - Update PHI inputs with actual latch values
//!
//! # Module Structure
//!
//! - `context`: ValueId management and counter operations
//! - `variable_models`: CarrierVariable, PinnedVariable, LoopBypassFlags types
//! - `utils`: Debug and bypass utilities
//! - `exit_phi`: Exit PHI builder for loop exits
//! - `builder_core`: Core LoopFormBuilder implementation
//! - `passes`: 4-pass architecture implementation
//!   - `pass1_discovery`: Variable discovery and classification
//!   - `pass2_header_phi`: Header PHI construction
//!   - `pass3_latch`: Latch processing
//!   - `pass4_exit_phi`: Exit PHI construction
//!
//! # Example Usage
//!
//! ```ignore
//! let mut builder = LoopFormBuilder::new(preheader, header);
//! builder.prepare_structure(&mut ops, &current_vars)?;
//! builder.emit_preheader(&mut ops)?;
//! builder.emit_header_phis(&mut ops)?;
//! // ... loop body construction ...
//! builder.seal_phis(&mut ops, &latch_vars)?;
//! ```
//!
//! # Status
//!
//! Phase: 25.1b prototype → 25.1m stabilization
//! Status: Always-on (NYASH_LOOPFORM_PHI_V2 for compatibility only)
//! Phase 191: Modularized into separate submodules
//! Phase 193: Complete 4-pass architecture explicit structure

// Core modules
pub mod builder_core;
pub mod context;
pub mod exit_phi;
pub mod passes;
pub mod utils;
pub mod variable_models;

// Re-export main types for backward compatibility
pub use builder_core::{LoopFormBuilder, LoopFormOps};
pub use context::LoopFormContext;
pub use variable_models::{CarrierVariable, LoopBypassFlags, PinnedVariable};

// Re-export utility functions
pub use utils::{
    get_loop_bypass_flags, is_joinir_exit_bypass_target, is_loopform_debug_enabled,
    joinir_exit_bypass_enabled,
};

// Re-export exit PHI function for backward compatibility
pub fn build_exit_phis_for_control<O: LoopFormOps>(
    loopform: &LoopFormBuilder,
    ops: &mut O,
    form: &crate::mir::control_form::ControlForm,
    exit_snapshots: &[(
        crate::mir::BasicBlockId,
        std::collections::BTreeMap<String, crate::mir::ValueId>,
    )],
    branch_source_block: crate::mir::BasicBlockId,
) -> Result<(), String> {
    exit_phi::ExitPhiBuilder::build_exit_phis_for_control(
        loopform,
        ops,
        form,
        exit_snapshots,
        branch_source_block,
    )
}
