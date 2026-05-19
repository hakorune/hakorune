use crate::ast::{ASTNode, BinaryOperator, CheckItem, LiteralValue, Span};
use crate::parser::cursor::TokenCursor;
use crate::parser::ParseError;
use crate::tokenizer::TokenType;
use std::collections::BTreeSet;

/// TokenCursorを使用した式パーサー（実験的実装）
pub struct ExprParserWithCursor;

impl ExprParserWithCursor {
    /// 式をパース（TokenCursor版）
    pub fn parse_expression(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        // 式モードで実行（改行を自動的にスキップ）
        cursor.with_expr_mode(|c| {
            c.skip_newlines();
            Self::parse_or_expr(c)
        })
    }

    /// OR式をパース
    fn parse_or_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
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
    fn parse_and_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
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
    fn parse_comparison_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
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
    fn parse_additive_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
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
    fn parse_multiplicative_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
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

    /// 単項演算子（- / not）
    fn parse_unary_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        // match式は旧系にあるが、ここでは単項の最小対応に限定
        match &cursor.current().token_type {
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
    fn parse_postfix_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
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

            // フィールドアクセス obj.field
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

            // 呼び出し (…) — 直前のノードの形に応じて Call/FunctionCall/MethodCall を作る
            if cursor.match_token(&TokenType::LPAREN) {
                // 引数リスト
                cursor.advance(); // consume '('
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

            // 添字アクセス target[index]
            if cursor.match_token(&TokenType::LBRACK) {
                cursor.advance(); // consume '['
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

    /// 乗算演算子をチェック
    fn match_multiplicative_op(cursor: &TokenCursor) -> Option<BinaryOperator> {
        match &cursor.current().token_type {
            TokenType::MULTIPLY => Some(BinaryOperator::Multiply),
            TokenType::DIVIDE => Some(BinaryOperator::Divide),
            TokenType::MODULO => Some(BinaryOperator::Modulo),
            _ => None,
        }
    }

    /// プライマリ式をパース
    fn parse_primary_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        match &cursor.current().token_type.clone() {
            TokenType::LBRACK => {
                // ARRAY-001: parser accepts the literal shape; Stage1 owns typed-context checks.
                cursor.advance(); // consume '['
                let mut elements: Vec<ASTNode> = Vec::new();
                while !cursor.match_token(&TokenType::RBRACK) && !cursor.is_at_end() {
                    let el = Self::parse_expression(cursor)?;
                    elements.push(el);
                    if cursor.match_token(&TokenType::COMMA) {
                        cursor.advance();
                    }
                }
                cursor.consume(TokenType::RBRACK)?;
                Ok(ASTNode::ArrayLiteral {
                    elements,
                    span: Span::unknown(),
                })
            }
            TokenType::NUMBER(n) => {
                let value = *n;
                cursor.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::Integer(value),
                    span: Span::unknown(),
                })
            }
            TokenType::TypedNumber(n, declared_type_name) => {
                let value = *n;
                let declared_type_name = declared_type_name.clone();
                cursor.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::TypedInteger {
                        value,
                        declared_type_name,
                    },
                    span: Span::unknown(),
                })
            }
            TokenType::STRING(s) => {
                let value = s.clone();
                cursor.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::String(value),
                    span: Span::unknown(),
                })
            }
            TokenType::TRUE => {
                cursor.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: Span::unknown(),
                })
            }
            TokenType::FALSE => {
                cursor.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    span: Span::unknown(),
                })
            }
            TokenType::NULL => {
                cursor.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::Null,
                    span: Span::unknown(),
                })
            }
            TokenType::VOID => {
                cursor.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::Void,
                    span: Span::unknown(),
                })
            }
            TokenType::NONE => {
                cursor.advance();
                Ok(ASTNode::FromCall {
                    parent: "Option".to_string(),
                    method: "None".to_string(),
                    arguments: Vec::new(),
                    span: Span::unknown(),
                })
            }
            TokenType::SOME => {
                cursor.advance();
                let payload = Self::parse_expression(cursor)?;
                Ok(ASTNode::FromCall {
                    parent: "Option".to_string(),
                    method: "Some".to_string(),
                    arguments: vec![payload],
                    span: Span::unknown(),
                })
            }
            TokenType::IDENTIFIER(name) => {
                let name = name.clone();
                if name == "check" {
                    return Self::parse_check_or_variable(cursor);
                }
                cursor.advance();
                Ok(ASTNode::Variable {
                    name,
                    span: Span::unknown(),
                })
            }
            TokenType::LPAREN => {
                cursor.advance();
                let expr = Self::parse_expression(cursor)?;
                cursor.consume(TokenType::RPAREN)?;
                Ok(expr)
            }
            TokenType::LBRACE => {
                // オブジェクトリテラル（改行対応済み）
                Self::parse_object_literal(cursor)
            }
            TokenType::NEW => {
                // new ClassName(<args>) with optional type args: <T,U>
                cursor.advance();
                let class = match &cursor.current().token_type {
                    TokenType::IDENTIFIER(s) => {
                        let v = s.clone();
                        cursor.advance();
                        v
                    }
                    other => {
                        let line = cursor.current().line;
                        return Err(ParseError::UnexpectedToken {
                            found: other.clone(),
                            expected: "class identifier after 'new'".to_string(),
                            line,
                        });
                    }
                };

                // Optional type arguments <T, U>
                let mut type_arguments: Vec<String> = Vec::new();
                if cursor.match_token(&TokenType::LESS) {
                    cursor.advance();
                    loop {
                        match &cursor.current().token_type {
                            TokenType::IDENTIFIER(tn) => {
                                type_arguments.push(tn.clone());
                                cursor.advance();
                            }
                            other => {
                                let line = cursor.current().line;
                                return Err(ParseError::UnexpectedToken {
                                    found: other.clone(),
                                    expected: "type identifier".to_string(),
                                    line,
                                });
                            }
                        }
                        if cursor.match_token(&TokenType::COMMA) {
                            cursor.advance();
                            continue;
                        }
                        cursor.consume(TokenType::GREATER)?;
                        break;
                    }
                }

                cursor.consume(TokenType::LPAREN)?;
                let mut arguments = Vec::new();
                while !cursor.match_token(&TokenType::RPAREN) && !cursor.is_at_end() {
                    let arg = Self::parse_expression(cursor)?;
                    arguments.push(arg);
                    if cursor.match_token(&TokenType::COMMA) {
                        cursor.advance();
                    }
                }
                cursor.consume(TokenType::RPAREN)?;
                Ok(ASTNode::New {
                    class,
                    arguments,
                    type_arguments,
                    span: Span::unknown(),
                })
            }
            _ => {
                let line = cursor.current().line;
                Err(ParseError::InvalidExpression { line })
            }
        }
    }

    fn parse_check_or_variable(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        cursor.advance(); // consume `check`

        let name = match &cursor.current().token_type {
            TokenType::STRING(label) => {
                let label = label.clone();
                cursor.advance();
                Some(label)
            }
            TokenType::LBRACE => None,
            _ => {
                return Ok(ASTNode::Variable {
                    name: "check".to_string(),
                    span: Span::unknown(),
                })
            }
        };

        cursor.consume(TokenType::LBRACE)?;
        let mut items = Vec::new();

        while !cursor.match_token(&TokenType::RBRACE) && !cursor.is_at_end() {
            if cursor.match_token(&TokenType::COMMA) {
                cursor.advance();
                continue;
            }

            let label = if let TokenType::STRING(label) = &cursor.current().token_type {
                let candidate = label.clone();
                let pos = cursor.position();
                cursor.advance();
                if cursor.match_token(&TokenType::COLON) {
                    cursor.advance();
                    Some(candidate)
                } else {
                    cursor.set_position(pos);
                    None
                }
            } else {
                None
            };

            let expression = Self::parse_expression(cursor)?;
            items.push(CheckItem { label, expression });

            if cursor.match_token(&TokenType::COMMA) {
                cursor.advance();
            }
        }

        cursor.consume(TokenType::RBRACE)?;
        Ok(ASTNode::CheckExpr {
            name,
            items,
            span: Span::unknown(),
        })
    }

    /// オブジェクトリテラルをパース（TokenCursor版）
    fn parse_object_literal(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        cursor.consume(TokenType::LBRACE)?;
        let mut entries = Vec::new();

        while !cursor.match_token(&TokenType::RBRACE) && !cursor.is_at_end() {
            // キーをパース（STRING or IDENTIFIER）
            let key = match &cursor.current().token_type {
                TokenType::STRING(s) => {
                    let k = s.clone();
                    cursor.advance();
                    k
                }
                TokenType::IDENTIFIER(id) => {
                    let k = id.clone();
                    cursor.advance();
                    k
                }
                _ => {
                    let line = cursor.current().line;
                    return Err(ParseError::UnexpectedToken {
                        found: cursor.current().token_type.clone(),
                        expected: "string or identifier key".to_string(),
                        line,
                    });
                }
            };

            cursor.consume(TokenType::COLON)?;
            let value = Self::parse_expression(cursor)?;
            entries.push((key, value));

            if cursor.match_token(&TokenType::COMMA) {
                cursor.advance();
            }
        }

        cursor.consume(TokenType::RBRACE)?;
        Ok(ASTNode::MapLiteral {
            entries,
            span: Span::unknown(),
        })
    }

    fn parse_record_literal(
        cursor: &mut TokenCursor,
        record_type_name: String,
    ) -> Result<ASTNode, ParseError> {
        cursor.consume(TokenType::LBRACE)?;
        let mut fields = Vec::new();
        let mut seen = BTreeSet::new();

        while !cursor.match_token(&TokenType::RBRACE) && !cursor.is_at_end() {
            if cursor.match_token(&TokenType::COMMA) {
                cursor.advance();
                continue;
            }

            let field_name = match &cursor.current().token_type {
                TokenType::IDENTIFIER(name) => {
                    let name = name.clone();
                    cursor.advance();
                    name
                }
                other => {
                    let line = cursor.current().line;
                    return Err(ParseError::UnexpectedToken {
                        found: other.clone(),
                        expected: "[record-literal] field name".to_string(),
                        line,
                    });
                }
            };
            if !seen.insert(field_name.clone()) {
                let line = cursor.current().line;
                return Err(ParseError::UnexpectedToken {
                    found: TokenType::IDENTIFIER(field_name),
                    expected: "[record-literal] unique field name".to_string(),
                    line,
                });
            }
            cursor.consume(TokenType::COLON)?;
            let value = Self::parse_expression(cursor)?;
            fields.push((field_name, value));

            if cursor.match_token(&TokenType::COMMA) {
                cursor.advance();
            }
        }

        cursor.consume(TokenType::RBRACE)?;
        Ok(ASTNode::RecordLiteral {
            record_type_name,
            fields,
            span: Span::unknown(),
        })
    }

    fn current_is_contextual_with(cursor: &TokenCursor) -> bool {
        matches!(&cursor.current().token_type, TokenType::IDENTIFIER(name) if name == "with")
    }

    fn current_lbrace_starts_record_literal(cursor: &TokenCursor) -> bool {
        if !cursor.match_token(&TokenType::LBRACE) {
            return false;
        }

        let mut offset = 1;
        while matches!(
            cursor.peek_nth_token(offset),
            TokenType::NEWLINE | TokenType::COMMA
        ) {
            offset += 1;
        }
        if !matches!(cursor.peek_nth_token(offset), TokenType::IDENTIFIER(_)) {
            return false;
        }
        offset += 1;
        while matches!(cursor.peek_nth_token(offset), TokenType::NEWLINE) {
            offset += 1;
        }
        matches!(cursor.peek_nth_token(offset), TokenType::COLON)
    }

    fn parse_record_update(cursor: &mut TokenCursor, base: ASTNode) -> Result<ASTNode, ParseError> {
        cursor.advance(); // consume contextual `with`
        cursor.consume(TokenType::LBRACE)?;
        let mut updates = Vec::new();
        let mut seen = BTreeSet::new();

        while !cursor.match_token(&TokenType::RBRACE) && !cursor.is_at_end() {
            if cursor.match_token(&TokenType::COMMA) {
                cursor.advance();
                continue;
            }

            let field_name = match &cursor.current().token_type {
                TokenType::IDENTIFIER(name) => {
                    let name = name.clone();
                    cursor.advance();
                    name
                }
                other => {
                    let line = cursor.current().line;
                    return Err(ParseError::UnexpectedToken {
                        found: other.clone(),
                        expected: "[record-update] field name".to_string(),
                        line,
                    });
                }
            };
            if !seen.insert(field_name.clone()) {
                let line = cursor.current().line;
                return Err(ParseError::UnexpectedToken {
                    found: TokenType::IDENTIFIER(field_name),
                    expected: "[record-update] unique field name".to_string(),
                    line,
                });
            }
            cursor.consume(TokenType::COLON)?;
            let value = Self::parse_expression(cursor)?;
            updates.push((field_name, value));

            if cursor.match_token(&TokenType::COMMA) {
                cursor.advance();
            }
        }

        cursor.consume(TokenType::RBRACE)?;
        Ok(ASTNode::RecordUpdate {
            base: Box::new(base),
            updates,
            span: Span::unknown(),
        })
    }
}
