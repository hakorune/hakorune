/*!
 * Static declaration parsing
 * Handles both static functions and static boxes
 */

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    /// 静的宣言をパース - 🔥 static function / static box 記法  
    pub fn parse_static_declaration(&mut self) -> Result<ASTNode, ParseError> {
        self.consume(TokenType::STATIC)?;

        // 次のトークンで分岐: function か box か
        match &self.current_token().token_type {
            TokenType::FUNCTION => self.parse_static_function(),
            TokenType::BOX => crate::parser::declarations::static_def::parse_static_box(self),
            TokenType::IDENTIFIER(word) if word == "const" => self.parse_static_const_table(),
            _ => {
                let line = self.current_token().line;
                Err(ParseError::UnexpectedToken {
                    found: self.current_token().token_type.clone(),
                    expected: "function, box, or const after static".to_string(),
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

        Ok(ASTNode::StaticConstTable {
            name,
            element_type: "u16".to_string(),
            values,
            span: Span::unknown(),
        })
    }

    /// 静的関数宣言をパース - static function Name() { ... }
    fn parse_static_function(&mut self) -> Result<ASTNode, ParseError> {
        self.consume(TokenType::FUNCTION)?;
        let attrs = self.take_pending_runes_for_static_function()?;

        // 関数名を取得（Box名.関数名の形式をサポート）
        let name = if let TokenType::IDENTIFIER(first_part) = &self.current_token().token_type {
            let mut full_name = first_part.clone();
            self.advance();

            // ドット記法をチェック（例：Math.min）
            if self.match_token(&TokenType::DOT) {
                self.advance(); // DOTを消費

                if let TokenType::IDENTIFIER(method_name) = &self.current_token().token_type {
                    full_name = format!("{}.{}", full_name, method_name);
                    self.advance();
                } else {
                    let line = self.current_token().line;
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "method name after dot".to_string(),
                        line,
                    });
                }
            }

            full_name
        } else {
            let line = self.current_token().line;
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "static function name".to_string(),
                line,
            });
        };

        // パラメータリストをパース
        self.consume(TokenType::LPAREN)?;

        let param_decls =
            crate::parser::common::params::parse_param_decl_list(self, "static method")?;
        let params = crate::ast::ParamDecl::names(&param_decls);

        self.consume(TokenType::RPAREN)?;
        let return_type_name =
            crate::parser::common::params::parse_optional_return_type_annotation(
                self,
                "static method",
            )?;
        let (uses, contracts) = self.parse_signature_metadata_until_body()?;

        // 関数本体をパース（共通ブロックヘルパー）
        let body = self.parse_block_statements()?;

        Ok(ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            return_type_name,
            body,
            contracts,
            uses,
            is_static: true,    // 🔥 静的関数フラグを設定
            is_override: false, // デフォルトは非オーバーライド
            attrs,
            span: Span::unknown(),
        })
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
