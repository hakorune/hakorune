use crate::ast::ASTNode;
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    pub(crate) fn parse_block_expr(&mut self) -> Result<ASTNode, ParseError> {
        let start_span = self.current_span();
        self.consume(TokenType::LBRACE)?;

        let mut stmts: Vec<ASTNode> = Vec::new();
        while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
            stmts.push(self.parse_statement()?);
        }

        let end_span = self.current_span();
        self.consume(TokenType::RBRACE)?;

        let tail_expr = match stmts.pop() {
            Some(expr) => {
                if !expr.is_expression() {
                    return Err(ParseError::UnexpectedToken {
                        found: TokenType::RBRACE,
                        expected: "BlockExpr must end with an expression, not a statement"
                            .to_string(),
                        line: end_span.line,
                    });
                }
                expr
            }
            None => {
                return Err(ParseError::UnexpectedToken {
                    found: TokenType::RBRACE,
                    expected: "BlockExpr requires at least one expression".to_string(),
                    line: start_span.line,
                });
            }
        };

        Ok(ASTNode::BlockExpr {
            prelude_stmts: stmts,
            tail_expr: Box::new(tail_expr),
            span: start_span.merge(end_span),
        })
    }
}
