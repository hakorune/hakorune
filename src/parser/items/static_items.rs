/*!
 * Static declaration parsing
 * Static boxes and constant tables
 */

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    /// Parse a static box or static constant table.
    pub fn parse_static_declaration(&mut self) -> Result<ASTNode, ParseError> {
        self.consume(TokenType::STATIC)?;

        match &self.current_token().token_type {
            TokenType::FUNCTION => Err(ParseError::function_outside_box(self.current_token().line)),
            TokenType::BOX => crate::parser::declarations::static_def::parse_static_box(self),
            TokenType::IDENTIFIER(word) if word == "const" => self.parse_static_const_table(),
            _ => {
                let line = self.current_token().line;
                Err(ParseError::UnexpectedToken {
                    found: self.current_token().token_type.clone(),
                    expected: "box or const after static".to_string(),
                    line,
                })
            }
        }
    }

    /// Backend-private static readonly table declaration.
    ///
    /// Accepted M11b-decl shape:
    /// `static const NAME: u16[] = [1, 2, 3]`
    ///
    /// Accepted M11b-eval first shape:
    /// `static const NAME: u16[] = [8 + 8, 3 * 8, 1 << 5]`
    fn parse_static_const_table(&mut self) -> Result<ASTNode, ParseError> {
        self.advance(); // consume identifier `const`

        let name = if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
            let name = name.clone();
            self.advance();
            name
        } else {
            let line = self.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "[static-const/declaration] table name".to_string(),
                line,
            });
        };

        self.consume(TokenType::COLON)?;
        let type_text =
            crate::parser::common::type_refs::parse_type_ref_text(self, "static const table")?;
        if type_text != "u16[]" {
            let line = self.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "[static-const/unsupported-element] u16[]".to_string(),
                line,
            });
        }

        self.consume(TokenType::ASSIGN)?;
        self.consume(TokenType::LBRACK)?;
        let mut values = Vec::new();
        while !self.match_token(&TokenType::RBRACK) && !self.is_at_end() {
            let expr_line = self.current_token().line;
            let expr = self.parse_expression()?;
            values.push(eval_static_const_u16_expr(&expr, expr_line)?);

            if self.match_token(&TokenType::COMMA) {
                self.advance();
                continue;
            }
            if self.match_token(&TokenType::RBRACK) {
                break;
            }
            let line = self.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "[static-const/declaration] comma or closing bracket".to_string(),
                line,
            });
        }
        self.consume(TokenType::RBRACK)?;

        let node = ASTNode::StaticConstTable {
            name,
            element_type: "u16".to_string(),
            values,
            span: Span::unknown(),
        };

        self.wrap_with_pending_build_gate(node)
    }
}

fn eval_static_const_u16_expr(expr: &ASTNode, line: usize) -> Result<u64, ParseError> {
    let value = eval_static_const_i128_expr(expr, line)?;
    if !(0..=u16::MAX as i128).contains(&value) {
        return Err(ParseError::UnexpectedToken {
            found: TokenType::NUMBER(value as i64),
            expected: "[static-const/value-out-of-range] 0..65535".to_string(),
            line,
        });
    }
    Ok(value as u64)
}

fn eval_static_const_i128_expr(expr: &ASTNode, line: usize) -> Result<i128, ParseError> {
    match expr {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Ok(*value as i128),
        ASTNode::Literal {
            value: LiteralValue::TypedInteger { value, .. },
            ..
        } => Ok(*value as i128),
        ASTNode::UnaryOp {
            operator: UnaryOperator::Minus,
            operand,
            ..
        } => eval_static_const_i128_expr(operand, line)?
            .checked_neg()
            .ok_or_else(|| static_const_unsupported_expr(line, "unary -")),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            let lhs = eval_static_const_i128_expr(left, line)?;
            let rhs = eval_static_const_i128_expr(right, line)?;
            eval_static_const_binary(operator, lhs, rhs, line)
        }
        _ => Err(ParseError::UnexpectedToken {
            found: TokenType::IDENTIFIER(expr.info()),
            expected: "[static-const/unsupported-initializer] integer const expression".to_string(),
            line,
        }),
    }
}

fn eval_static_const_binary(
    operator: &BinaryOperator,
    lhs: i128,
    rhs: i128,
    line: usize,
) -> Result<i128, ParseError> {
    let unsupported = || static_const_unsupported_expr(line, operator.to_string());
    match operator {
        BinaryOperator::Add => lhs.checked_add(rhs).ok_or_else(unsupported),
        BinaryOperator::Subtract => lhs.checked_sub(rhs).ok_or_else(unsupported),
        BinaryOperator::Multiply => lhs.checked_mul(rhs).ok_or_else(unsupported),
        BinaryOperator::Divide => {
            if rhs == 0 {
                return Err(unsupported());
            }
            lhs.checked_div(rhs).ok_or_else(unsupported)
        }
        BinaryOperator::Modulo => {
            if rhs == 0 {
                return Err(unsupported());
            }
            lhs.checked_rem(rhs).ok_or_else(unsupported)
        }
        BinaryOperator::Shl => {
            if lhs < 0 {
                return Err(unsupported());
            }
            let shift = static_const_shift_amount(rhs, line)?;
            lhs.checked_shl(shift).ok_or_else(unsupported)
        }
        BinaryOperator::Shr => {
            if lhs < 0 {
                return Err(unsupported());
            }
            let shift = static_const_shift_amount(rhs, line)?;
            lhs.checked_shr(shift).ok_or_else(unsupported)
        }
        BinaryOperator::BitAnd | BinaryOperator::BitOr | BinaryOperator::BitXor
            if lhs >= 0 && rhs >= 0 =>
        {
            match operator {
                BinaryOperator::BitAnd => Ok(lhs & rhs),
                BinaryOperator::BitOr => Ok(lhs | rhs),
                BinaryOperator::BitXor => Ok(lhs ^ rhs),
                _ => unreachable!(),
            }
        }
        _ => Err(unsupported()),
    }
}

fn static_const_unsupported_expr(line: usize, found: impl ToString) -> ParseError {
    ParseError::UnexpectedToken {
        found: TokenType::IDENTIFIER(found.to_string()),
        expected: "[static-const/unsupported-initializer] integer const expression".to_string(),
        line,
    }
}

fn static_const_shift_amount(value: i128, line: usize) -> Result<u32, ParseError> {
    if !(0..=63).contains(&value) {
        return Err(ParseError::UnexpectedToken {
            found: TokenType::NUMBER(value as i64),
            expected: "[static-const/unsupported-initializer] shift amount 0..63".to_string(),
            line,
        });
    }
    Ok(value as u32)
}
