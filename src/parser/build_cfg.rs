//! Parser support for AST-level build conditionals.
//!
//! `gate` is intentionally parser-contextual instead of a tokenizer keyword so
//! existing source may keep ordinary identifiers named `gate`.

mod predicate;
mod prune;

pub use hakorune_frontend_parser::parser::BuildGateExplainReport;
