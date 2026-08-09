use crate::ast::{ASTNode, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    /// Recognize `release root` only when both contextual words occupy the
    /// same source line. Parenthesized and newline-separated spellings remain
    /// in the ordinary identifier/call grammar.
    pub(super) fn is_release_statement_start(&self) -> bool {
        let Some(prefix) = self.tokens.get(self.current) else {
            return false;
        };
        let TokenType::IDENTIFIER(name) = &prefix.token_type else {
            return false;
        };
        if name != "release" {
            return false;
        }
        let Some(root) = self.tokens.get(self.current + 1) else {
            return false;
        };
        matches!(
            root.token_type,
            TokenType::IDENTIFIER(_) | TokenType::ME | TokenType::THIS
        ) && root.line == prefix.line
    }

    pub(super) fn parse_release_statement(&mut self) -> Result<ASTNode, ParseError> {
        let prefix = self.current_token().clone();
        let root_token = self.tokens.get(self.current + 1).cloned().ok_or_else(|| {
            ParseError::UnexpectedToken {
                found: TokenType::EOF,
                expected: "[freeze:contract][parser/release_exact_root_required] exact identifier root after `release`".to_string(),
                line: prefix.line,
            }
        })?;
        let terminal = self
            .tokens
            .get(self.current + 2)
            .map(|token| &token.token_type)
            .unwrap_or(&TokenType::EOF);
        if !matches!(
            terminal,
            TokenType::NEWLINE | TokenType::SEMICOLON | TokenType::RBRACE | TokenType::EOF
        ) {
            return Err(ParseError::UnexpectedToken {
                found: terminal.clone(),
                expected: "[freeze:contract][parser/release_exact_root_required] statement end after exact identifier root".to_string(),
                line: prefix.line,
            });
        }
        let TokenType::IDENTIFIER(root) = root_token.token_type else {
            return Err(ParseError::UnexpectedToken {
                found: root_token.token_type,
                expected: "[freeze:contract][parser/release_exact_root_required] exact identifier root after `release`".to_string(),
                line: prefix.line,
            });
        };

        self.advance();
        self.advance();
        Ok(ASTNode::Release {
            root,
            span: Span::new(0, 0, prefix.line, prefix.column),
        })
    }
}
