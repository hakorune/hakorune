use super::ExprParserWithCursor;
use crate::ast::{ASTNode, CheckItem, LiteralValue, Span};
use crate::parser::cursor::TokenCursor;
use crate::parser::ParseError;
use crate::tokenizer::TokenType;
impl ExprParserWithCursor {
    /// プライマリ式をパース
    pub(crate) fn parse_primary_expr(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        match &cursor.current().token_type.clone() {
            TokenType::LBRACK => {
                cursor.advance();
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
            TokenType::LBRACE => Self::parse_object_literal(cursor),
            TokenType::NEW => {
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

                let mut arguments = Vec::new();
                if cursor.match_token(&TokenType::LPAREN) {
                    cursor.advance();
                    while !cursor.match_token(&TokenType::RPAREN) && !cursor.is_at_end() {
                        let arg = Self::parse_expression(cursor)?;
                        arguments.push(arg);
                        if cursor.match_token(&TokenType::COMMA) {
                            cursor.advance();
                        }
                    }
                    cursor.consume(TokenType::RPAREN)?;
                }
                let field_initializers = Self::parse_box_field_initializers(cursor)?;
                Ok(ASTNode::New {
                    class,
                    arguments,
                    field_initializers,
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

    fn parse_box_field_initializers(
        cursor: &mut TokenCursor,
    ) -> Result<Vec<(String, ASTNode)>, ParseError> {
        let mut fields = Vec::new();
        if !cursor.match_token(&TokenType::LBRACE) {
            return Ok(fields);
        }
        cursor.advance();
        while !cursor.match_token(&TokenType::RBRACE) && !cursor.is_at_end() {
            let name = if let TokenType::IDENTIFIER(name) = &cursor.current().token_type {
                let name = name.clone();
                cursor.advance();
                name
            } else {
                return Err(ParseError::UnexpectedToken {
                    found: cursor.current().token_type.clone(),
                    expected: "field name".to_string(),
                    line: cursor.current().line,
                });
            };
            cursor.consume(TokenType::COLON)?;
            let expr = Self::parse_expression(cursor)?;
            fields.push((name, expr));
            if cursor.match_token(&TokenType::COMMA) {
                cursor.advance();
            }
        }
        cursor.consume(TokenType::RBRACE)?;
        Ok(fields)
    }

    fn parse_check_or_variable(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        cursor.advance();

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
            if cursor.match_token(&TokenType::COMMA) || cursor.match_token(&TokenType::NEWLINE) {
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

    fn parse_object_literal(cursor: &mut TokenCursor) -> Result<ASTNode, ParseError> {
        cursor.consume(TokenType::LBRACE)?;
        let mut entries = Vec::new();

        while !cursor.match_token(&TokenType::RBRACE) && !cursor.is_at_end() {
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
}
