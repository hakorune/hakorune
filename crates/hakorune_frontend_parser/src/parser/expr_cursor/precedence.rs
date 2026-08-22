use super::ExprParserWithCursor;
use crate::ast::{ASTNode, BinaryOperator, Span};
use crate::parser::cursor::TokenCursor;
use crate::parser::ParseError;
use crate::tokenizer::TokenType;

impl ExprParserWithCursor {
    /// OR式をパース
    pub(crate) fn parse_or_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        let mut left = Self::parse_and_expr(cursor)?;

        while cursor.match_token(&TokenType::OR) {
            let op_line = cursor.current().line;
            cursor.advance();
            let right = Self::parse_and_expr(cursor)?;
            left = ASTNode::BinaryOp {
                operator: BinaryOperator::Or,
                left: Box::new(left),
                right: Box::new(right),
                span: Span::new(op_line, 0, op_line, 0),
            };
        }

        Ok(left)
    }

    /// AND式をパース
    pub(crate) fn parse_and_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        let mut left = Self::parse_comparison_expr(cursor)?;

        while cursor.match_token(&TokenType::AND) {
            let op_line = cursor.current().line;
            cursor.advance();
            let right = Self::parse_comparison_expr(cursor)?;
            left = ASTNode::BinaryOp {
                operator: BinaryOperator::And,
                left: Box::new(left),
                right: Box::new(right),
                span: Span::new(op_line, 0, op_line, 0),
            };
        }

        Ok(left)
    }

    /// 比較式をパース
    pub(crate) fn parse_comparison_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        let mut left = Self::parse_additive_expr(cursor)?;

        while let Some(op) = Self::match_comparison_op(cursor) {
            let op_line = cursor.current().line;
            cursor.advance();
            let right = Self::parse_additive_expr(cursor)?;
            left = ASTNode::BinaryOp {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                span: Span::new(op_line, 0, op_line, 0),
            };
        }

        Ok(left)
    }

    /// 比較演算子をチェック
    fn match_comparison_op(cursor: &TokenCursor) -> Option<BinaryOperator> {
        match &cursor.current().token_type {
            TokenType::EQUALS => Some(BinaryOperator::Equal),
            TokenType::NotEquals => Some(BinaryOperator::NotEqual),
            TokenType::LESS => Some(BinaryOperator::Less),
            TokenType::LessEquals => Some(BinaryOperator::LessEqual),
            TokenType::GREATER => Some(BinaryOperator::Greater),
            TokenType::GreaterEquals => Some(BinaryOperator::GreaterEqual),
            _ => None,
        }
    }

    /// 加算式をパース
    pub(crate) fn parse_additive_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        let mut left = Self::parse_multiplicative_expr(cursor)?;

        while let Some(op) = Self::match_additive_op(cursor) {
            let op_line = cursor.current().line;
            cursor.advance();
            let right = Self::parse_multiplicative_expr(cursor)?;
            left = ASTNode::BinaryOp {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                span: Span::new(op_line, 0, op_line, 0),
            };
        }

        Ok(left)
    }

    /// 加算演算子をチェック
    fn match_additive_op(cursor: &TokenCursor) -> Option<BinaryOperator> {
        match &cursor.current().token_type {
            TokenType::PLUS => Some(BinaryOperator::Add),
            TokenType::MINUS => Some(BinaryOperator::Subtract),
            _ => None,
        }
    }

    /// 乗算式をパース
    pub(crate) fn parse_multiplicative_expr(
        cursor: &mut TokenCursor,
    ) -> Result<ASTNode, ParseError> {
        let mut left = Self::parse_unary_expr(cursor)?;

        while let Some(op) = Self::match_multiplicative_op(cursor) {
            let op_line = cursor.current().line;
            cursor.advance();
            let right = Self::parse_unary_expr(cursor)?;
            left = ASTNode::BinaryOp {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                span: Span::new(op_line, 0, op_line, 0),
            };
        }

        Ok(left)
    }

    /// 乗算演算子をチェック
    fn match_multiplicative_op(cursor: &TokenCursor) -> Option<BinaryOperator> {
        match &cursor.current().token_type {
            TokenType::MULTIPLY => Some(BinaryOperator::Multiply),
            TokenType::DIVIDE => Some(BinaryOperator::Divide),
            TokenType::MODULO => Some(BinaryOperator::Modulo),
            _ => None,
        }
    }

    /// 単項演算子（- / not）
    pub(crate) fn parse_unary_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        // match式は旧系にあるが、ここでは単項の最小対応に限定
        match &cursor.current().token_type {
            TokenType::WEAK => {
                let op_line = cursor.current().line;
                cursor.advance();
                if cursor.match_token(&TokenType::LPAREN) {
                    return Err(ParseError::UnexpectedToken {
                        found: TokenType::LPAREN,
                        expected: "[freeze:contract][parser/weak_paren_call_rejected] grammar profile rejects weak_paren_expr".to_owned(),
                        line: cursor.current().line,
                    });
                }
                let operand = Self::parse_unary_expr(cursor)?;
                Ok(ASTNode::UnaryOp {
                    operator: crate::ast::UnaryOperator::Weak,
                    operand: Box::new(operand),
                    span: Span::new(op_line, 0, op_line, 0),
                })
            }
            TokenType::MINUS => {
                let op_line = cursor.current().line;
                cursor.advance();
                let operand = Self::parse_unary_expr(cursor)?;
                Ok(ASTNode::UnaryOp {
                    operator: crate::ast::UnaryOperator::Minus,
                    operand: Box::new(operand),
                    span: Span::new(op_line, 0, op_line, 0),
                })
            }
            TokenType::NOT => {
                let op_line = cursor.current().line;
                cursor.advance();
                let operand = Self::parse_unary_expr(cursor)?;
                Ok(ASTNode::UnaryOp {
                    operator: crate::ast::UnaryOperator::Not,
                    operand: Box::new(operand),
                    span: Span::new(op_line, 0, op_line, 0),
                })
            }
            TokenType::BitNot => {
                let op_line = cursor.current().line;
                cursor.advance();
                let operand = Self::parse_unary_expr(cursor)?;
                Ok(ASTNode::UnaryOp {
                    operator: crate::ast::UnaryOperator::BitNot,
                    operand: Box::new(operand),
                    span: Span::new(op_line, 0, op_line, 0),
                })
            }
            TokenType::AWAIT => {
                let op_line = cursor.current().line;
                cursor.advance();
                let expression = Self::parse_unary_expr(cursor)?;
                Ok(ASTNode::AwaitExpression {
                    expression: Box::new(expression),
                    span: Span::new(op_line, 0, op_line, 0),
                })
            }
            _ => Self::parse_postfix_expr(cursor),
        }
    }

    /// 後置（フィールドアクセス・関数/メソッド呼び出し）をパース
    pub(crate) fn parse_postfix_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        let mut expr = Self::parse_primary_expr(cursor)?;

        loop {
            if matches!(expr, ASTNode::Variable { .. })
                && Self::current_lbrace_starts_record_literal(cursor)
            {
                if let ASTNode::Variable { name, .. } = expr {
                    expr = Self::parse_record_literal(cursor, name)?;
                    continue;
                }
            }
            if let ASTNode::Variable { name, .. } = &expr {
                if cursor.match_token(&TokenType::LBRACE)
                    && name
                        .chars()
                        .next()
                        .map(|ch| ch.is_ascii_uppercase())
                        .unwrap_or(false)
                {
                    let line = cursor.current().line;
                    return Err(ParseError::UnexpectedToken {
                        found: TokenType::LBRACE,
                        expected: "[record-literal] field COLON".to_string(),
                        line,
                    });
                }
            }

            if Self::current_is_contextual_with(cursor) {
                expr = Self::parse_record_update(cursor, expr)?;
                continue;
            }

            if cursor.match_token(&TokenType::DOT) {
                cursor.advance();
                if cursor.match_token(&TokenType::BIRTH) {
                    let line = cursor.current().line;
                    return Err(crate::parser::lifecycle::direct_birth_call_error(
                        cursor.current().token_type.clone(),
                        line,
                    ));
                }
                let field = match &cursor.current().token_type {
                    TokenType::IDENTIFIER(s) => {
                        let v = s.clone();
                        cursor.advance();
                        v
                    }
                    other => {
                        let line = cursor.current().line;
                        return Err(ParseError::UnexpectedToken {
                            found: other.clone(),
                            expected: "identifier after '.'".to_string(),
                            line,
                        });
                    }
                };
                expr = ASTNode::FieldAccess {
                    object: Box::new(expr),
                    field,
                    span: Span::unknown(),
                };
                continue;
            }

            if let ASTNode::Variable { name, .. } = &expr {
                if name == "externcall" {
                    if let TokenType::STRING(target) = &cursor.current().token_type {
                        if !matches!(cursor.peek_nth_token(1), TokenType::LPAREN) {
                            return Err(ParseError::UnexpectedToken {
                                found: cursor.peek_nth_token(1).clone(),
                                expected: "[parser/explicit_externcall_shape] '(' after target"
                                    .to_owned(),
                                line: cursor.current().line,
                            });
                        } else {
                            let target = target.clone();
                            cursor.advance();
                            cursor.consume(TokenType::LPAREN)?;
                            let mut arguments = Vec::new();
                            while !cursor.match_token(&TokenType::RPAREN) && !cursor.is_at_end() {
                                arguments.push(Self::parse_expression(cursor)?);
                                if cursor.match_token(&TokenType::COMMA) {
                                    cursor.advance();
                                }
                            }
                            cursor.consume(TokenType::RPAREN)?;
                            expr = ASTNode::ExplicitExternCall {
                                target,
                                arguments,
                                span: Span::unknown(),
                            };
                            continue;
                        }
                    }
                }
            }

            if cursor.match_token(&TokenType::LPAREN) {
                cursor.advance();
                let mut args: Vec<ASTNode> = Vec::new();
                while !cursor.match_token(&TokenType::RPAREN) && !cursor.is_at_end() {
                    let a = Self::parse_expression(cursor)?;
                    args.push(a);
                    if cursor.match_token(&TokenType::COMMA) {
                        cursor.advance();
                    }
                }
                cursor.consume(TokenType::RPAREN)?;

                expr = match expr {
                    ASTNode::Variable { name, .. } => ASTNode::FunctionCall {
                        name,
                        arguments: args,
                        span: Span::unknown(),
                    },
                    ASTNode::FieldAccess { object, field, .. } => ASTNode::MethodCall {
                        object,
                        method: field,
                        arguments: args,
                        span: Span::unknown(),
                    },
                    callee => ASTNode::Call {
                        callee: Box::new(callee),
                        arguments: args,
                        span: Span::unknown(),
                    },
                };
                continue;
            }

            if cursor.match_token(&TokenType::LBRACK) {
                cursor.advance();
                let index_expr = Self::parse_expression(cursor)?;
                cursor.consume(TokenType::RBRACK)?;
                expr = ASTNode::Index {
                    target: Box::new(expr),
                    index: Box::new(index_expr),
                    span: Span::unknown(),
                };
                continue;
            }

            break;
        }

        Ok(expr)
    }
}

#[cfg(test)]
mod weak_grammar_parity_tests {
    use super::*;
    use crate::tokenizer::Token;

    fn token(token_type: TokenType, column: usize) -> Token {
        Token {
            token_type,
            line: 1,
            column,
        }
    }

    #[test]
    fn weak_unary_emits_one_weak_ast_node() {
        let tokens = [
            token(TokenType::WEAK, 1),
            token(TokenType::IDENTIFIER("value".to_owned()), 6),
            token(TokenType::EOF, 11),
        ];
        let mut cursor = TokenCursor::new(&tokens);
        let parsed = ExprParserWithCursor::parse_expression(&mut cursor).unwrap();
        assert!(matches!(
            parsed,
            ASTNode::UnaryOp {
                operator: crate::ast::UnaryOperator::Weak,
                operand,
                ..
            } if matches!(operand.as_ref(), ASTNode::Variable { name, .. } if name == "value")
        ));
    }

    #[test]
    fn weak_parenthesized_rejects_before_operand_parse() {
        let tokens = [
            token(TokenType::WEAK, 1),
            token(TokenType::LPAREN, 5),
            token(TokenType::IDENTIFIER("missing".to_owned()), 6),
            token(TokenType::RPAREN, 13),
            token(TokenType::EOF, 14),
        ];
        let mut cursor = TokenCursor::new(&tokens);
        let error = ExprParserWithCursor::parse_expression(&mut cursor).unwrap_err();
        assert!(format!("{error}").contains("parser/weak_paren_call_rejected"));
        assert!(cursor.match_token(&TokenType::LPAREN));
    }
}
