//! Contract-bound memory fast-path region parsing.
//!
//! `fastmem ContractName { ... }` is a parse-only capsule in the current
//! parser parity lane. Execution and lowering remain owned by later rows.

use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    pub(super) fn is_fastmem_region_statement_start(&self) -> bool {
        matches!(
            &self.current_token().token_type,
            TokenType::IDENTIFIER(name) if name == "fastmem"
        )
    }

    pub(super) fn parse_fastmem_region_statement(&mut self) -> Result<ASTNode, ParseError> {
        let line = self.current_token().line;
        self.advance();

        let contract = match &self.current_token().token_type {
            TokenType::IDENTIFIER(name) => {
                let value = name.clone();
                self.advance();
                value
            }
            other => {
                return Err(ParseError::UnexpectedToken {
                    found: other.clone(),
                    expected: "[freeze:contract][parser/fastmem] contract name after fastmem"
                        .to_string(),
                    line,
                });
            }
        };

        let body = self.parse_block_statements()?;
        Ok(ASTNode::FastMemRegion {
            contract,
            body,
            span: Span::new(0, 0, line, 1),
        })
    }
}
