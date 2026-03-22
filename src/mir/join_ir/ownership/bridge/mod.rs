//! Bridge helpers that adapt ownership analysis results to lowering/runtime checks.
//!
//! Keep this sub-box separate from the analysis core:
//! - `analyzer.rs` / `ast_analyzer/*` decide ownership facts
//! - `bridge/*` turns those facts into lowering inputs or validator checks
//!
//! This is still inside the JoinIR review lane and is not a standalone crate
//! boundary yet.

mod plan_to_lowering;
mod plan_validator;

pub use plan_to_lowering::*;
pub use plan_validator::*;
