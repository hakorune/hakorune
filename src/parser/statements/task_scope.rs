use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    pub(super) fn is_task_scope_statement_start(&self) -> bool {
        let TokenType::IDENTIFIER(name) = &self.current_token().token_type else {
            return false;
        };
        matches!(name.as_str(), "co" | "task_scope")
            && matches!(self.peek_token(), TokenType::LBRACE)
    }

    pub(super) fn parse_task_scope_statement(&mut self) -> Result<ASTNode, ParseError> {
        let source_keyword = match &self.current_token().token_type {
            TokenType::IDENTIFIER(name) if matches!(name.as_str(), "co" | "task_scope") => {
                name.clone()
            }
            other => {
                return Err(ParseError::UnexpectedToken {
                    found: other.clone(),
                    expected: "`co { ... }` or `task_scope { ... }`".to_string(),
                    line: self.current_token().line,
                });
            }
        };
        self.advance();
        let body = self.parse_block_statements()?;
        Ok(ASTNode::TaskScope {
            body,
            source_keyword,
            span: Span::unknown(),
        })
    }
}
