use crate::ast::{ASTNode, BinaryOperator, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    pub(crate) fn expr_parse_factor(&mut self) -> Result<ASTNode, ParseError> {
        let mut expr = self.parse_unary()?;
        while self.match_token(&TokenType::MULTIPLY)
            || self.match_token(&TokenType::DIVIDE)
            || self.match_token(&TokenType::MODULO)
        {
            let operator = match &self.current_token().token_type {
                TokenType::MULTIPLY => BinaryOperator::Multiply,
                TokenType::DIVIDE => BinaryOperator::Divide,
                TokenType::MODULO => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary()?;
            if crate::parser::env::grammar_diff() {
                let name = match operator {
                    BinaryOperator::Multiply => "mul",
                    BinaryOperator::Divide => "div",
                    _ => "mod",
                };
                let ok = hakorune_frontend_grammar::engine::get().syntax_is_allowed_binop(name);
                if !ok {
                    crate::parser::log::warn(&format!(
                        "[GRAMMAR-DIFF][Parser] binop '{}' not allowed by syntax rules",
                        name
                    ));
                }
            }
            expr = ASTNode::BinaryOp {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
                span: Span::unknown(),
            };
        }
        Ok(expr)
    }
}
