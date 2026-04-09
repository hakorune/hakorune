use crate::ast::{ASTNode, LiteralValue, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::{BTreeMap, BTreeSet};

impl NyashParser {
    pub(crate) fn expr_parse_primary(&mut self) -> Result<ASTNode, ParseError> {
        match &self.current_token().token_type {
            TokenType::LBRACK => {
                let sugar_on = crate::parser::sugar_gate::is_enabled()
                    || std::env::var("NYASH_ENABLE_ARRAY_LITERAL").ok().as_deref() == Some("1");
                if !sugar_on {
                    let line = self.current_token().line;
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "enable NYASH_SYNTAX_SUGAR_LEVEL=basic|full or NYASH_ENABLE_ARRAY_LITERAL=1".to_string(),
                        line,
                    });
                }
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
                    || std::env::var("NYASH_ENABLE_MAP_LITERAL").ok().as_deref() == Some("1");
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
            TokenType::THIS => {
                let line = self.current_token().line;
                let column = self.current_token().column;
                if std::env::var("NYASH_DEPRECATE_THIS").ok().as_deref() == Some("1") {
                    crate::runtime::get_global_ring0().log.warn(&format!(
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
                    self.consume(TokenType::LPAREN)?;
                    let mut arguments = Vec::new();
                    while !self.match_token(&TokenType::RPAREN) && !self.is_at_end() {
                        crate::must_advance!(self, _unused, "new expression argument parsing");
                        arguments.push(self.parse_expression()?);
                        if self.match_token(&TokenType::COMMA) {
                            self.advance();
                        }
                    }
                    self.consume(TokenType::RPAREN)?;
                    Ok(ASTNode::New {
                        class,
                        arguments,
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

    fn parse_known_enum_record_ctor_arguments(
        &mut self,
        enum_name: &str,
        variant_name: &str,
    ) -> Result<Vec<ASTNode>, ParseError> {
        let line = self.current_token().line;
        let variant_decl = self
            .known_enums
            .get(enum_name)
            .and_then(|variants| variants.iter().find(|variant| variant.name == variant_name))
            .cloned()
            .ok_or_else(|| ParseError::UnexpectedToken {
                found: TokenType::LBRACE,
                expected: format!("known enum variant `{}` for `{}`", variant_name, enum_name),
                line,
            })?;
        if !variant_decl.is_record_payload() {
            return Err(ParseError::UnexpectedToken {
                found: TokenType::LBRACE,
                expected: format!("tuple constructor `{}::{}(...)`", enum_name, variant_name),
                line,
            });
        }

        self.consume(TokenType::LBRACE)?;
        let mut values = BTreeMap::new();
        let mut seen = BTreeSet::new();
        while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
            if self.match_token(&TokenType::COMMA) || self.match_token(&TokenType::NEWLINE) {
                self.advance();
                continue;
            }

            let field_name = match &self.current_token().token_type {
                TokenType::IDENTIFIER(name) => {
                    let name = name.clone();
                    self.advance();
                    name
                }
                other => {
                    return Err(ParseError::UnexpectedToken {
                        found: other.clone(),
                        expected: "record field name".to_string(),
                        line: self.current_token().line,
                    });
                }
            };
            if !seen.insert(field_name.clone()) {
                return Err(ParseError::InvalidMatchPattern {
                    detail: format!(
                        "duplicate field `{}` in record enum constructor {}::{}",
                        field_name, enum_name, variant_name
                    ),
                    line: self.current_token().line,
                });
            }
            if !variant_decl
                .record_field_decls
                .iter()
                .any(|field| field.name == field_name)
            {
                return Err(ParseError::InvalidMatchPattern {
                    detail: format!(
                        "unknown field `{}` in record enum constructor {}::{}",
                        field_name, enum_name, variant_name
                    ),
                    line: self.current_token().line,
                });
            }

            self.consume(TokenType::COLON)?;
            let value = self.parse_expression()?;
            values.insert(field_name, value);

            if self.match_token(&TokenType::COMMA) {
                self.advance();
            }
        }
        self.consume(TokenType::RBRACE)?;

        let mut ordered = Vec::with_capacity(variant_decl.record_field_decls.len());
        let mut missing = Vec::new();
        for field in &variant_decl.record_field_decls {
            if let Some(value) = values.remove(&field.name) {
                ordered.push(value);
            } else {
                missing.push(field.name.clone());
            }
        }
        if !missing.is_empty() {
            return Err(ParseError::InvalidMatchPattern {
                detail: format!(
                    "record enum constructor {}::{} is missing field(s): {}",
                    enum_name,
                    variant_name,
                    missing.join(", ")
                ),
                line,
            });
        }
        Ok(ordered)
    }

    /// Check if current position looks like legacy map literal: { "key" : ...
    /// Current token should be LBRACE when called.
    fn peek_is_legacy_map_literal(&self) -> bool {
        // Look ahead: { STRING COLON ...
        // Note: peek_nth_token(1) is relative to current position
        let next = self.peek_nth_token(1);
        let after = self.peek_nth_token(2);
        matches!(next, TokenType::STRING(_)) && matches!(after, TokenType::COLON)
    }

    /// Parse BlockExpr: { prelude_stmts; tail_expr }
    fn parse_block_expr(&mut self) -> Result<ASTNode, ParseError> {
        let start_span = self.current_span();
        self.consume(TokenType::LBRACE)?;

        let mut stmts: Vec<ASTNode> = Vec::new();
        while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
            stmts.push(self.parse_statement()?);
        }

        let end_span = self.current_span();
        self.consume(TokenType::RBRACE)?;

        // Last stmt must be expression (becomes tail_expr)
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
