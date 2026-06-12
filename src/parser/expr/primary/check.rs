use crate::ast::{ASTNode, CheckItem};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    pub(crate) fn parse_check_expr(&mut self) -> Result<ASTNode, ParseError> {
        let start_span = self.current_span();
        self.advance();

        let name = match &self.current_token().token_type {
            TokenType::STRING(label) => {
                let label = label.clone();
                self.advance();
                Some(label)
            }
            _ => None,
        };

        self.consume(TokenType::LBRACE)?;
        let mut items = Vec::new();

        while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
            if self.match_token(&TokenType::COMMA) || self.match_token(&TokenType::NEWLINE) {
                self.advance();
                continue;
            }

            let label = match &self.current_token().token_type {
                TokenType::STRING(label) if matches!(self.peek_nth_token(1), TokenType::COLON) => {
                    let label = label.clone();
                    self.advance();
                    self.consume(TokenType::COLON)?;
                    Some(label)
                }
                _ => None,
            };

            let expression = self.parse_expression()?;
            items.push(CheckItem { label, expression });

            if self.match_token(&TokenType::COMMA) {
                self.advance();
            }
        }

        self.consume(TokenType::RBRACE)?;
        Ok(ASTNode::CheckExpr {
            name,
            items,
            span: start_span,
        })
    }
}
