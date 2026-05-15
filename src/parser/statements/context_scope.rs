//! Structured ambient context statement parsing.
//!
//! `context` is canonical and `scoped` is a compatibility spelling. Both are
//! contextual identifiers; the tokenizer does not reserve either word.

use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    pub(super) fn is_context_scope_statement_start(&self) -> bool {
        matches!(
            (&self.current_token().token_type, self.peek_token()),
            (TokenType::IDENTIFIER(name), TokenType::IDENTIFIER(_))
                if name == "context" || name == "scoped"
        )
    }

    pub(super) fn parse_context_scope_statement(&mut self) -> Result<ASTNode, ParseError> {
        let source_keyword = match &self.current_token().token_type {
            TokenType::IDENTIFIER(name) if name == "context" || name == "scoped" => name.clone(),
            other => {
                return Err(ParseError::UnexpectedToken {
                    found: other.clone(),
                    expected: "`context` binding".to_string(),
                    line: self.current_token().line,
                });
            }
        };
        self.advance();

        let name = match &self.current_token().token_type {
            TokenType::IDENTIFIER(name) => {
                let value = name.clone();
                self.advance();
                value
            }
            other => {
                return Err(ParseError::UnexpectedToken {
                    found: other.clone(),
                    expected: "context binding name".to_string(),
                    line: self.current_token().line,
                });
            }
        };

        let declared_type_name = if self.match_token(&TokenType::COLON) {
            self.advance();
            Some(crate::parser::common::type_refs::parse_type_ref_text(
                self,
                "context binding type",
            )?)
        } else {
            None
        };

        self.consume(TokenType::ASSIGN)?;
        let value = Box::new(self.parse_expression()?);
        let body = self.parse_block_statements()?;

        Ok(ASTNode::ContextScope {
            name,
            declared_type_name,
            value,
            body,
            source_keyword,
            span: Span::unknown(),
        })
    }
}
