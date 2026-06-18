//! Passive frontend parser/tokenizer crate scaffold.
//!
//! This crate is the future owner for parser/tokenizer modules. The initial
//! scaffold intentionally preserves root module names used by the existing
//! parser/tokenizer code, but it does not move active parser files yet.
//!
//! Boundary:
//! - may depend on `hakorune-frontend-ast` and `hakorune-frontend-grammar`
//! - must not depend on the main crate, runtime, MIR, backend, or Box impls
//! - must not change parser/tokenizer behavior while it is scaffold-only

pub const FRONTEND_PARSER_CRATE_READY: bool = true;

pub mod ast;
pub mod frontend_env;
pub mod frontend_host;
pub mod frontend_log;
pub mod grammar;
pub mod parser;
pub mod tokenizer;

#[macro_export]
macro_rules! must_advance {
    ($parser:expr, $unused:ident, $context:expr) => {{
        let before = $parser.current;
        let _ = $parser.advance();
        if $parser.current == before {
            return Err(crate::parser::ParseError::UnexpectedToken {
                expected: format!("progress while {}", $context),
                found: format!("{:?}", $parser.current_token()),
                line: $parser.current_token().line,
            });
        }
    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrontendParserBoundary;

impl FrontendParserBoundary {
    pub const fn name(self) -> &'static str {
        "hakorune-frontend-parser"
    }
}
