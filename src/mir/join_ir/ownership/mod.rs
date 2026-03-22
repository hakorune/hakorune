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
//! - Phase 57: Analyzer implemented
//! - Phase 58-59: ownership/bridge/plan_to_lowering helper for P2/P3
//! - Phase 71-Pre: ownership/bridge/plan_validator box (reusable validation)

mod analyzer;
mod ast_analyzer;
mod bridge;
mod types;

pub use analyzer::*;
pub use ast_analyzer::*;
pub use bridge::*;
pub use types::*;
