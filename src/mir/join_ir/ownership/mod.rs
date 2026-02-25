//! Ownership Analysis for JoinIR
//!
//! # Responsibility Boundary
//!
//! This module is responsible for **analysis only**:
//! - Collecting reads/writes from AST/ProgramJSON
//! - Determining variable ownership (owned/relay/capture)
//! - Producing OwnershipPlan for downstream lowering
//!
//! This module does NOT:
//! - Generate MIR instructions
//! - Modify JoinIR structures
//! - Perform lowering transformations
//!
//! # Core Invariants
//!
//! 1. **Ownership Uniqueness**: Each variable has exactly one owner scope
//! 2. **Carrier Locality**: carriers = writes ∩ owned
//! 3. **Relay Propagation**: writes to ancestor-owned → relay up
//! 4. **Capture Read-Only**: captures have no PHI at this scope
//!
//! # Phase Status
//!
//! - Phase 57: Analyzer implemented (dev-only)
//! - Phase 58: plan_to_lowering helper for P2 (analyzer-based testing only)
//! - Phase 59: plan_to_lowering helper for P3 (if-sum patterns)
//! - Phase 71-Pre: plan_validator box (reusable validation)

mod analyzer;
#[cfg(feature = "normalized_dev")]
mod ast_analyzer;
#[cfg(feature = "normalized_dev")]
mod plan_to_lowering;
#[cfg(feature = "normalized_dev")]
mod plan_validator;
mod types;

pub use analyzer::*;
#[cfg(feature = "normalized_dev")]
pub use ast_analyzer::*;
#[cfg(feature = "normalized_dev")]
pub use plan_to_lowering::*;
#[cfg(feature = "normalized_dev")]
pub use plan_validator::*;
pub use types::*;
