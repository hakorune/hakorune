//! Parser compatibility root scaffold.
//!
//! Active parser implementation still lives in the main crate. This module
//! exists so future file-move rows can preserve `crate::parser::*` paths inside
//! the extracted crate.

pub mod build_cfg;
pub mod build_config;
pub mod cursor;
pub mod error;
pub mod expr_cursor;
pub mod lifecycle;
pub use build_cfg::BuildGateExplainReport;
pub use build_config::{BuildMode, ParserBuildConfig};
pub use cursor::{NewlineMode, TokenCursor};
pub use error::ParseError;
pub use expr_cursor::ExprParserWithCursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserBoundary;

impl ParserBoundary {
    pub const fn name(self) -> &'static str {
        "parser"
    }
}
