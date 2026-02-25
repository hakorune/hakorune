//! LoopForm 4-Pass PHI Generation Pipeline
//!
//! This module implements the 4-pass architecture for PHI node construction
//! in loops. Each pass has a specific responsibility in the SSA construction
//! process.
//!
//! # The 4 Passes
//!
//! ## Pass 1: Variable Discovery & ValueId Allocation (`pass1_discovery`)
//! - **Purpose**: Identify all variables in the loop and allocate ValueIds upfront
//! - **Responsibility**: Classify variables as carriers (modified) or pinned (invariant)
//! - **Key Innovation**: All ValueIds allocated BEFORE any MIR emission
//! - **Function**: `prepare_structure()`
//!
//! ## Pass 2: Preheader Copy Generation (`pass2_preheader`)
//! - **Purpose**: Generate copy instructions in preheader block
//! - **Responsibility**: Emit deterministic copy instructions
//! - **Order**: Pinned variables first, then carrier variables
//! - **Function**: `emit_preheader()`
//!
//! ## Pass 3: Header PHI Construction (`pass3_header_phi`)
//! - **Purpose**: Generate incomplete PHI nodes in header block
//! - **Responsibility**: Create PHI nodes with preheader input only
//! - **Note**: Latch input will be added in Pass 4
//! - **Function**: `emit_header_phis()`
//!
//! ## Pass 4: PHI Sealing (`pass4_seal`)
//! - **Purpose**: Complete PHI nodes by finding latch values
//! - **Responsibility**: Update PHI inputs with actual latch values
//! - **Handles**: Both pinned and carrier variables
//! - **Function**: `seal_phis()`
//!
//! # Usage Example
//!
//! ```ignore
//! // Pass 1: Variable discovery
//! builder.prepare_structure(&mut ops, &current_vars)?;
//!
//! // Pass 2: Preheader copies
//! builder.emit_preheader(&mut ops)?;
//!
//! // Pass 3: Header PHIs (incomplete)
//! builder.emit_header_phis(&mut ops)?;
//!
//! // ... loop body construction ...
//!
//! // Pass 4: Seal PHIs with latch values
//! builder.seal_phis(&mut ops, latch_id, &continue_snapshots, &writes, false)?;
//! ```
//!
//! # Architecture Benefits
//!
//! - **Explicit separation**: Each pass has clear responsibilities
//! - **Testability**: Each pass can be tested independently
//! - **Maintainability**: Easy to understand and modify
//! - **Determinism**: Predictable ValueId allocation and PHI construction

pub mod pass1_discovery;
pub mod pass2_preheader;
pub mod pass3_header_phi;
pub mod pass4_seal;

// Re-export main functions for convenience
pub use pass1_discovery::prepare_structure;
pub use pass2_preheader::emit_preheader;
pub use pass3_header_phi::emit_header_phis;
pub use pass4_seal::seal_phis;
