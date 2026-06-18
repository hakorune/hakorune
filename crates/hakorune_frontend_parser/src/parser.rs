//! Parser compatibility root scaffold.
//!
//! Active parser implementation still lives in the main crate. This module
//! exists so future file-move rows can preserve `crate::parser::*` paths inside
//! the extracted crate.

pub mod build_cfg;
pub use build_cfg::BuildGateExplainReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserBoundary;

impl ParserBoundary {
    pub const fn name(self) -> &'static str {
        "parser"
    }
}
