use crate::ast::{ASTNode, LiteralValue, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

mod block;
mod check;
mod record;

impl NyashParser {
    pub(crate) fn expr_parse_primary(&mut self) -> Result<ASTNode, ParseError> {
        match &self.current_token().token_type {
            TokenType::LBRACK => {
                // ARRAY-001: parser accepts the literal shape; Stage1 owns typed-context checks.
                self.advance();
                let mut elems: Vec<ASTNode> = Vec::new();
                while !self.match_token(&TokenType::RBRACK) && !self.is_at_end() {
                    crate::must_advance!(self, _unused, "array literal element parsing");
                    let el = self.parse_expression()?;
                    elems.push(el);
                    if self.match_token(&TokenType::COMMA) {
                        self.advance();
                    }
                }
                self.consume(TokenType::RBRACK)?;
                Ok(ASTNode::ArrayLiteral {
                    elements: elems,
                    span: Span::unknown(),
                })
            }
            TokenType::PercentLBrace => {
                let sugar_on = crate::parser::sugar_gate::is_enabled()
                    || crate::parser::env::enable_map_literal();
                if !sugar_on {
                    let line = self.current_token().line;
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected:
                            "enable NYASH_SYNTAX_SUGAR_LEVEL=basic|full or NYASH_ENABLE_MAP_LITERAL=1"
                                .to_string(),
                        line,
                    });
                }
                self.advance();
                let mut entries: Vec<(String, ASTNode)> = Vec::new();
                while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
                    let key = match &self.current_token().token_type {
                        TokenType::STRING(s) => {
                            let v = s.clone();
                            self.advance();
                            v
                        }
                        _ => {
                            let line = self.current_token().line;
                            return Err(ParseError::UnexpectedToken {
                                found: self.current_token().token_type.clone(),
                                expected: "string key in `%{...}` map literal".to_string(),
                                line,
                            });
                        }
                    };
                    if self.match_token(&TokenType::COLON) {
                        let line = self.current_token().line;
                        return Err(ParseError::UnexpectedToken {
                            found: self.current_token().token_type.clone(),
                            expected: "`%{...}` uses `=>` (legacy map literal is `{ \"k\": v }`)"
                                .to_string(),
                            line,
                        });
                    }
                    self.consume(TokenType::FatArrow)?;
                    let value_expr = self.parse_expression()?;
                    entries.push((key, value_expr));
                    if self.match_token(&TokenType::COMMA) {
                        self.advance();
                    }
                }
                self.consume(TokenType::RBRACE)?;
                Ok(ASTNode::MapLiteral {
                    entries,
                    span: Span::unknown(),
                })
            }
            TokenType::LBRACE => {
                // B2-2: Parse {...} as BlockExpr (Phase B2)
                // Check for legacy map literal {"key": value} - provide helpful error
                if self.peek_is_legacy_map_literal() {
                    let line = self.current_token().line;
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "Legacy map literal `{\"key\": value}` is no longer supported. Use `%{\"key\" => value}` instead.".to_string(),
                        line,
                    });
                }
                // Parse as BlockExpr: { prelude_stmts; tail_expr }
                self.parse_block_expr()
            }
            TokenType::STRING(s) => {
                let line = self.current_token().line;
                let column = self.current_token().column;
                let value = s.clone();
                self.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::String(value),
                    span: Span::new(0, 0, line, column),
                })
            }
            TokenType::NUMBER(n) => {
                let line = self.current_token().line;
                let column = self.current_token().column;
                let value = *n;
                self.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::Integer(value),
                    span: Span::new(0, 0, line, column),
                })
            }
            TokenType::TypedNumber(n, declared_type_name) => {
                let line = self.current_token().line;
                let column = self.current_token().column;
                let value = *n;
                let declared_type_name = declared_type_name.clone();
                self.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::TypedInteger {
                        value,
                        declared_type_name,
                    },
                    span: Span::new(0, 0, line, column),
                })
            }
            TokenType::FLOAT(f) => {
                let line = self.current_token().line;
                let column = self.current_token().column;
                let value = *f;
                self.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::Float(value),
                    span: Span::new(0, 0, line, column),
                })
            }
            TokenType::TRUE => {
                let line = self.current_token().line;
                let column = self.current_token().column;
                self.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: Span::new(0, 0, line, column),
                })
            }
            TokenType::FALSE => {
                let line = self.current_token().line;
                let column = self.current_token().column;
                self.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    span: Span::new(0, 0, line, column),
                })
            }
            TokenType::NULL => {
                let line = self.current_token().line;
                let column = self.current_token().column;
                self.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::Null,
                    span: Span::new(0, 0, line, column),
                })
            }
            TokenType::VOID => {
                let line = self.current_token().line;
                let column = self.current_token().column;
                self.advance();
                Ok(ASTNode::Literal {
                    value: LiteralValue::Void,
                    span: Span::new(0, 0, line, column),
                })
            }
            TokenType::NONE => {
                self.advance();
                Ok(ASTNode::FromCall {
                    parent: "Option".to_string(),
                    method: "None".to_string(),
                    arguments: Vec::new(),
                    span: Span::unknown(),
                })
            }
            TokenType::SOME => {
                self.advance();
                let payload = self.parse_expression()?;
                Ok(ASTNode::FromCall {
                    parent: "Option".to_string(),
                    method: "Some".to_string(),
                    arguments: vec![payload],
                    span: Span::unknown(),
                })
            }
            TokenType::THIS => {
                let line = self.current_token().line;
                let column = self.current_token().column;
                if crate::parser::env::deprecate_this_enabled() {
                    crate::parser::log::warn(&format!(
                        "[deprecate:this] 'this' is deprecated; use 'me' instead (line {})",
                        self.current_token().line
                    ));
                }
                self.advance();
                Ok(ASTNode::Me {
                    span: Span::new(0, 0, line, column),
                })
            }
            TokenType::ME => {
                let line = self.current_token().line;
                let column = self.current_token().column;
                self.advance();
                Ok(ASTNode::Me {
                    span: Span::new(0, 0, line, column),
                })
            }
            TokenType::NEW => {
                self.advance();
                if let TokenType::IDENTIFIER(class_name) = &self.current_token().token_type {
                    let class = class_name.clone();
                    self.advance();
                    let mut type_arguments: Vec<String> = Vec::new();
                    if self.match_token(&TokenType::LESS) {
                        self.advance();
                        loop {
                            if let TokenType::IDENTIFIER(tn) = &self.current_token().token_type {
                                type_arguments.push(tn.clone());
                                self.advance();
                            } else {
                                let line = self.current_token().line;
                                return Err(ParseError::UnexpectedToken {
                                    found: self.current_token().token_type.clone(),
                                    expected: "type argument".to_string(),
                                    line,
                                });
                            }
                            if self.match_token(&TokenType::COMMA) {
                                self.advance();
                                continue;
                            }
                            self.consume(TokenType::GREATER)?;
                            break;
                        }
                    }
                    let mut arguments = Vec::new();
                    if self.match_token(&TokenType::LPAREN) {
                        self.advance();
                        while !self.match_token(&TokenType::RPAREN) && !self.is_at_end() {
                            crate::must_advance!(self, _unused, "new expression argument parsing");
                            arguments.push(self.parse_expression()?);
                            if self.match_token(&TokenType::COMMA) {
                                self.advance();
                            }
                        }
                        self.consume(TokenType::RPAREN)?;
                    }
                    let field_initializers = self.parse_box_field_initializers()?;
                    Ok(ASTNode::New {
                        class,
                        arguments,
                        field_initializers,
                        type_arguments,
                        span: Span::unknown(),
                    })
                } else {
                    let line = self.current_token().line;
                    Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "class name".to_string(),
                        line,
                    })
                }
            }
            TokenType::FROM => self.parse_from_call(),
            TokenType::IDENTIFIER(name) => {
                if name == "check"
                    && matches!(
                        self.peek_nth_token(1),
                        TokenType::STRING(_) | TokenType::LBRACE
                    )
                {
                    return self.parse_check_expr();
                }

                let parent = name.clone();
                self.advance();
                if self.match_token(&TokenType::DoubleColon) {
                    self.advance();
                    let method = match &self.current_token().token_type {
                        TokenType::IDENTIFIER(m) => {
                            let s = m.clone();
                            self.advance();
                            s
                        }
                        TokenType::INIT => {
                            self.advance();
                            "init".to_string()
                        }
                        TokenType::PACK => {
                            self.advance();
                            "pack".to_string()
                        }
                        TokenType::BIRTH => {
                            self.advance();
                            "birth".to_string()
                        }
                        _ => {
                            let line = self.current_token().line;
                            return Err(ParseError::UnexpectedToken {
                                found: self.current_token().token_type.clone(),
                                expected: "method name".to_string(),
                                line,
                            });
                        }
                    };
                    let arguments = if self.match_token(&TokenType::LPAREN) {
                        self.parse_parent_colon_arguments()?
                    } else if self.match_token(&TokenType::LBRACE) {
                        self.parse_known_enum_record_ctor_arguments(&parent, &method)?
                    } else if self
                        .known_enums
                        .get(&parent)
                        .and_then(|variants| variants.iter().find(|variant| variant.name == method))
                        .is_some_and(|variant| variant.payload_arity() == 0)
                    {
                        Vec::new()
                    } else {
                        let line = self.current_token().line;
                        return Err(ParseError::UnexpectedToken {
                            found: self.current_token().token_type.clone(),
                            expected: "`(` or `{` after `Type::Variant`".to_string(),
                            line,
                        });
                    };
                    Ok(ASTNode::FromCall {
                        parent,
                        method,
                        arguments,
                        span: Span::unknown(),
                    })
                } else if self.current_lbrace_starts_record_literal() {
                    self.parse_record_literal(parent)
                } else if self.match_token(&TokenType::LBRACE)
                    && parent
                        .chars()
                        .next()
                        .map(|ch| ch.is_ascii_uppercase())
                        .unwrap_or(false)
                {
                    Err(ParseError::UnexpectedToken {
                        found: TokenType::LBRACE,
                        expected: "[record-literal] field COLON".to_string(),
                        line: self.current_token().line,
                    })
                } else {
                    Ok(ASTNode::Variable {
                        name: parent,
                        span: Span::unknown(),
                    })
                }
            }
            TokenType::LPAREN => {
                // Phase 152-A: Try grouped assignment first (Stage-3 only)
                if let Some(assignment) = self.try_parse_grouped_assignment()? {
                    return Ok(assignment);
                }

                // Fallback: normal grouped expression
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenType::RPAREN)?;
                Ok(expr)
            }
            TokenType::FN => {
                self.advance();
                let mut params: Vec<String> = Vec::new();
                if self.match_token(&TokenType::LPAREN) {
                    self.advance();
                    while !self.match_token(&TokenType::RPAREN) && !self.is_at_end() {
                        if let TokenType::IDENTIFIER(p) = &self.current_token().token_type {
                            params.push(p.clone());
                            self.advance();
                            if self.match_token(&TokenType::COMMA) {
                                self.advance();
                            }
                        } else {
                            let line = self.current_token().line;
                            return Err(ParseError::UnexpectedToken {
                                found: self.current_token().token_type.clone(),
                                expected: "parameter name".to_string(),
                                line,
                            });
                        }
                    }
                    self.consume(TokenType::RPAREN)?;
                }
                self.consume(TokenType::LBRACE)?;
                let mut body: Vec<ASTNode> = Vec::new();
                while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
                    if !self.match_token(&TokenType::RBRACE) {
                        body.push(self.parse_statement()?);
                    }
                }
                self.consume(TokenType::RBRACE)?;
                Ok(ASTNode::Lambda {
                    params,
                    body,
                    span: Span::unknown(),
                })
            }
            _ => {
                let line = self.current_token().line;
                Err(ParseError::InvalidExpression { line })
            }
        }
    }

    fn parse_box_field_initializers(&mut self) -> Result<Vec<(String, ASTNode)>, ParseError> {
        let mut fields = Vec::new();
        if !self.match_token(&TokenType::LBRACE) {
            return Ok(fields);
        }
        self.advance();
        while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
            let name = if let TokenType::IDENTIFIER(name) = &self.current_token().token_type {
                let name = name.clone();
                self.advance();
                name
            } else {
                return Err(ParseError::UnexpectedToken {
                    found: self.current_token().token_type.clone(),
                    expected: "field name".to_string(),
                    line: self.current_token().line,
                });
            };
            self.consume(TokenType::COLON)?;
            let expr = self.parse_expression()?;
            fields.push((name, expr));
            if self.match_token(&TokenType::COMMA) {
                self.advance();
            }
        }
        self.consume(TokenType::RBRACE)?;
        Ok(fields)
    }

    fn parse_parent_colon_arguments(&mut self) -> Result<Vec<ASTNode>, ParseError> {
        self.consume(TokenType::LPAREN)?;
        let mut arguments = Vec::new();
        while !self.match_token(&TokenType::RPAREN) && !self.is_at_end() {
            crate::must_advance!(self, _unused, "Parent::method call argument parsing");
            arguments.push(self.parse_expression()?);
            if self.match_token(&TokenType::COMMA) {
                self.advance();
            }
        }
        self.consume(TokenType::RPAREN)?;
        Ok(arguments)
    }
}
