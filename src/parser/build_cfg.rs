//! Parser support for AST-level build conditionals.
//!
//! `gate` is intentionally parser-contextual instead of a tokenizer keyword so
//! existing source may keep ordinary identifiers named `gate`.

pub(super) mod decision_set;
pub(super) mod program_item_slots;
pub(super) mod projection;
pub(super) use projection::project_build_gates;
mod predicate;
mod prune;

pub use hakorune_frontend_parser::parser::BuildGateExplainReport;
