/*!
 * Control Flow Statement Parsers
 *
 * Handles parsing of control flow statements:
 * - if/else statements
 * - loop statements
 * - break/continue statements
 * - return statements
 */

use crate::ast::{ASTNode, EnumMatchArm, EnumVariantDecl, LiteralValue, Span, UnaryOperator};
use crate::parser::common::ParserUtils;
use crate::parser::cursor::TokenCursor;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

impl NyashParser {
    /// Parse control flow statement dispatch
    pub(super) fn parse_control_flow_statement(&mut self) -> Result<ASTNode, ParseError> {
        let stage3 = Self::is_stage3_enabled();

        match &self.current_token().token_type {
            TokenType::IF => self.parse_if(),
            TokenType::GUARD => self.parse_guard_else(),
            // Stage-3: while
            TokenType::WHILE if stage3 => self.parse_while_stage3(),
            // Stage-3 legacy for-range compatibility. Canonical surface is loop i in a..b.
            TokenType::FOR if stage3 => self.parse_legacy_for_range_stage3(),
            // Legacy loop
            TokenType::LOOP => self.parse_loop(),
            TokenType::BREAK => self.parse_break(),
            TokenType::CONTINUE => self.parse_continue(),
            TokenType::RETURN => self.parse_return(),
            _ => Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "control flow statement".to_string(),
                line: self.current_token().line,
            }),
        }
    }

    /// Parse if statement: if (condition) { body } else if ... else { body }
    pub(super) fn parse_if(&mut self) -> Result<ASTNode, ParseError> {
        // Thin-adapt statement start when Cursor route is enabled
        if super::helpers::cursor_enabled() {
            let mut cursor = TokenCursor::new(&self.tokens);
            cursor.set_position(self.current);
            cursor.with_stmt_mode(|c| c.skip_newlines());
            self.current = cursor.position();
        }
        self.advance(); // consume 'if'

        if self.match_token(&TokenType::SOME) {
            return self.parse_if_some_sugar();
        }

        // Parse condition
        let condition = Box::new(self.parse_expression()?);

        // Parse then body
        let then_body = self.parse_block_statements()?;

        // Parse else if/else
        let else_body = if self.match_token(&TokenType::ELSE) {
            self.advance(); // consume 'else'

            if self.match_token(&TokenType::IF) {
                // else if - parse as nested if
                let nested_if = self.parse_if()?;
                Some(vec![nested_if])
            } else {
                // plain else
                Some(self.parse_block_statements()?)
            }
        } else {
            None
        };

        Ok(ASTNode::If {
            condition,
            then_body,
            else_body,
            span: Span::unknown(),
        })
    }

    /// Parse guard statement: guard condition else { early-exit body }
    ///
    /// C200 keeps `guard` as source sugar only. It lowers to:
    /// `if !(condition) { early-exit body }`
    pub(super) fn parse_guard_else(&mut self) -> Result<ASTNode, ParseError> {
        if super::helpers::cursor_enabled() {
            let mut cursor = TokenCursor::new(&self.tokens);
            cursor.set_position(self.current);
            cursor.with_stmt_mode(|c| c.skip_newlines());
            self.current = cursor.position();
        }
        self.advance(); // consume 'guard'

        if self.current_is_contextual_let() {
            return self.parse_guard_let_after_guard();
        }

        let condition = self.parse_expression()?;

        if !self.match_token(&TokenType::ELSE) {
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "else after guard condition".to_string(),
                line: self.current_token().line,
            });
        }
        self.advance(); // consume 'else'

        let then_body = self.parse_block_statements()?;

        Ok(ASTNode::If {
            condition: Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand: Box::new(condition),
                span: Span::unknown(),
            }),
            then_body,
            else_body: None,
            span: Span::unknown(),
        })
    }

    fn current_is_contextual_let(&self) -> bool {
        matches!(
            &self.current_token().token_type,
            TokenType::IDENTIFIER(name) if name == "let"
        )
    }

    fn parse_guard_let_after_guard(&mut self) -> Result<ASTNode, ParseError> {
        let let_line = self.current_token().line;
        let let_column = self.current_token().column;
        self.advance(); // consume contextual `let`

        let enum_name = self.consume_identifier("enum name in `guard let Type::Variant(...)`")?;
        self.consume(TokenType::DoubleColon)?;
        let variant_name =
            self.consume_identifier("variant name in `guard let Type::Variant(...)`")?;
        self.consume(TokenType::LPAREN)?;
        let binding_name = self.consume_identifier("binding identifier in `guard let`")?;
        self.consume(TokenType::RPAREN)?;
        self.consume(TokenType::ASSIGN)?;
        let scrutinee = self.parse_expression()?;

        if !self.match_token(&TokenType::ELSE) {
            return Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "else after guard let pattern".to_string(),
                line: self.current_token().line,
            });
        }
        self.advance(); // consume 'else'
        let else_body = self.parse_block_statements()?;

        let variants = self.known_enums.get(&enum_name).ok_or_else(|| {
            ParseError::InvalidMatchPattern {
                detail: format!("unknown enum `{}` in guard let pattern", enum_name),
                line: let_line,
            }
        })?;
        let variant = variants
            .iter()
            .find(|decl| decl.name == variant_name)
            .ok_or_else(|| ParseError::InvalidMatchPattern {
                detail: format!(
                    "unknown variant `{}` for enum `{}` in guard let pattern",
                    variant_name, enum_name
                ),
                line: let_line,
            })?;
        if variant.is_record_payload()
            || variant.is_multi_payload_tuple()
            || variant.payload_arity() != 1
        {
            return Err(ParseError::InvalidMatchPattern {
                detail: format!(
                    "guard let MVP supports single-payload enum variants only; got {}::{}",
                    enum_name, variant_name
                ),
                line: let_line,
            });
        }

        let temp_name = format!(
            "__ny_guard_let_subject_{}_{}_{}",
            let_line, let_column, self.current
        );
        let condition =
            enum_variant_failure_match_expr(&enum_name, &variant_name, &temp_name, variants);
        let binding_local = enum_variant_binding_local(
            &enum_name,
            &variant_name,
            &binding_name,
            &temp_name,
            variants,
        );

        Ok(ASTNode::ScopeBox {
            body: vec![
                ASTNode::Local {
                    variables: vec![temp_name.clone()],
                    initial_values: vec![Some(Box::new(scrutinee))],
                    declared_type_names: Vec::new(),
                    span: Span::unknown(),
                },
                ASTNode::If {
                    condition: Box::new(condition),
                    then_body: else_body,
                    else_body: None,
                    span: Span::unknown(),
                },
                binding_local,
            ],
            span: Span::unknown(),
        })
    }

    fn consume_identifier(&mut self, expected: &'static str) -> Result<String, ParseError> {
        match &self.current_token().token_type {
            TokenType::IDENTIFIER(name) => {
                let value = name.clone();
                self.advance();
                Ok(value)
            }
            other => Err(ParseError::UnexpectedToken {
                found: other.clone(),
                expected: expected.to_string(),
                line: self.current_token().line,
            }),
        }
    }

    fn parse_if_some_sugar(&mut self) -> Result<ASTNode, ParseError> {
        let some_line = self.current_token().line;
        let some_column = self.current_token().column;
        self.advance(); // consume 'some'

        let binding_name = match &self.current_token().token_type {
            TokenType::IDENTIFIER(name) => {
                let value = name.clone();
                self.advance();
                value
            }
            other => {
                return Err(ParseError::UnexpectedToken {
                    found: other.clone(),
                    expected: "binding identifier after `if some`".to_string(),
                    line: self.current_token().line,
                });
            }
        };

        self.consume(TokenType::ASSIGN)?;
        let scrutinee = self.parse_expression()?;
        let then_body = self.parse_block_statements()?;

        let else_body = if self.match_token(&TokenType::ELSE) {
            self.advance();
            if self.match_token(&TokenType::IF) {
                let nested_if = self.parse_if()?;
                Some(vec![nested_if])
            } else {
                Some(self.parse_block_statements()?)
            }
        } else {
            None
        };

        let temp_name = format!(
            "__ny_option_some_subject_{}_{}_{}",
            some_line, some_column, self.current
        );
        let condition = option_some_presence_match_expr(&temp_name);

        let mut then_body_with_binding = Vec::with_capacity(then_body.len() + 1);
        then_body_with_binding.push(option_some_binding_local(&binding_name, &temp_name));
        then_body_with_binding.extend(then_body);

        Ok(ASTNode::ScopeBox {
            body: vec![
                ASTNode::Local {
                    variables: vec![temp_name.clone()],
                    initial_values: vec![Some(Box::new(scrutinee))],
                    declared_type_names: Vec::new(),
                    span: Span::unknown(),
                },
                ASTNode::If {
                    condition: Box::new(condition),
                    then_body: then_body_with_binding,
                    else_body,
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        })
    }

    /// Parse loop statement.
    ///
    /// Supported header shapes:
    /// - `loop { ... }`
    /// - `loop(cond) { ... }`
    /// - `loop cond { ... }`
    /// - `loop i in start..end { ... }`
    /// - `loop(i in start..end) { ... }`
    pub(super) fn parse_loop(&mut self) -> Result<ASTNode, ParseError> {
        if super::helpers::cursor_enabled() {
            let mut cursor = TokenCursor::new(&self.tokens);
            cursor.set_position(self.current);
            cursor.with_stmt_mode(|c| c.skip_newlines());
            self.current = cursor.position();
        }
        self.advance(); // consume 'loop'

        if self.match_token(&TokenType::LBRACE) {
            let body = self.parse_block_statements()?;
            return Ok(ASTNode::Loop {
                condition: Box::new(ASTNode::Literal {
                    value: crate::ast::LiteralValue::Bool(true),
                    span: Span::unknown(),
                }),
                body,
                span: Span::unknown(),
            });
        }

        if self.match_token(&TokenType::LPAREN) {
            self.advance(); // consume '('
            if self.current_loop_range_header_starts() {
                let (var_name, start, end) =
                    self.parse_range_header("loop range index identifier")?;
                self.consume(TokenType::RPAREN)?;
                let body = self.parse_block_statements()?;
                return Ok(ASTNode::LoopRange {
                    var_name,
                    start,
                    end,
                    body,
                    span: Span::unknown(),
                });
            }
            let condition = Box::new(self.parse_expression()?);
            self.consume(TokenType::RPAREN)?;
            let body = self.parse_block_statements()?;
            return Ok(ASTNode::Loop {
                condition,
                body,
                span: Span::unknown(),
            });
        }

        if self.current_loop_range_header_starts() {
            let (var_name, start, end) =
                self.parse_range_header("loop range index identifier")?;
            let body = self.parse_block_statements()?;
            return Ok(ASTNode::LoopRange {
                var_name,
                start,
                end,
                body,
                span: Span::unknown(),
            });
        }

        let condition = Box::new(self.parse_expression()?);
        let body = self.parse_block_statements()?;

        Ok(ASTNode::Loop {
            condition,
            body,
            span: Span::unknown(),
        })
    }

    fn current_loop_range_header_starts(&self) -> bool {
        matches!(self.current_token().token_type, TokenType::IDENTIFIER(_))
            && matches!(self.peek_token(), TokenType::IN)
    }

    fn parse_range_header(
        &mut self,
        expected_identifier: &'static str,
    ) -> Result<(String, Box<ASTNode>, Box<ASTNode>), ParseError> {
        let var_name = match &self.current_token().token_type {
            TokenType::IDENTIFIER(name) => {
                let value = name.clone();
                self.advance();
                value
            }
            other => {
                return Err(ParseError::UnexpectedToken {
                    found: other.clone(),
                    expected: expected_identifier.to_string(),
                    line: self.current_token().line,
                });
            }
        };

        self.consume(TokenType::IN)?;
        let start = Box::new(self.expr_parse_term()?);
        self.consume(TokenType::RANGE)?;
        let end = Box::new(self.expr_parse_term()?);
        Ok((var_name, start, end))
    }

    /// Stage-3: while <cond> { body }
    fn parse_while_stage3(&mut self) -> Result<ASTNode, ParseError> {
        // Normalize cursor at statement start (skip leading newlines etc.)
        if super::helpers::cursor_enabled() {
            let mut cursor = TokenCursor::new(&self.tokens);
            cursor.set_position(self.current);
            cursor.with_stmt_mode(|c| c.skip_newlines());
            self.current = cursor.position();
        }
        // consume 'while'
        self.advance();

        // condition expression (no parentheses required in MVP)
        let condition = Box::new(self.parse_expression()?);

        // body block
        let body = self.parse_block_statements()?;

        // LOOPCLEAN-002: legacy while sugar normalizes to canonical Loop at
        // parser output. Keep the token accepted as Stage-3 compatibility, but
        // do not propagate a second loop-shaped AST for new parser output.
        Ok(ASTNode::Loop {
            condition,
            body,
            span: Span::unknown(),
        })
    }

    /// Stage-3 legacy for-range compatibility parser.
    ///
    /// Canonical Hakorune source uses `loop i in start..end { ... }`. This
    /// parser branch keeps older `for i in start..end { ... }` input readable
    /// and routes it through the same range-header metadata shape. It must not
    /// grow independent lowering semantics.
    fn parse_legacy_for_range_stage3(&mut self) -> Result<ASTNode, ParseError> {
        // Normalize cursor at statement start
        if super::helpers::cursor_enabled() {
            let mut cursor = TokenCursor::new(&self.tokens);
            cursor.set_position(self.current);
            cursor.with_stmt_mode(|c| c.skip_newlines());
            self.current = cursor.position();
        }
        // consume 'for'
        self.advance();
        let (var_name, start, end) = self.parse_range_header("for range index identifier")?;
        let body = self.parse_block_statements()?;
        Ok(ASTNode::LoopRange {
            var_name,
            start,
            end,
            body,
            span: Span::unknown(),
        })
    }

    /// Helper: env-gated Stage-3 enable check.
    fn is_stage3_enabled() -> bool {
        crate::config::env::parser_stage3_enabled()
    }

    /// Parse break statement
    pub(super) fn parse_break(&mut self) -> Result<ASTNode, ParseError> {
        self.advance(); // consume 'break'
        Ok(ASTNode::Break {
            span: Span::unknown(),
        })
    }

    /// Parse continue statement
    pub(super) fn parse_continue(&mut self) -> Result<ASTNode, ParseError> {
        self.advance(); // consume 'continue'
        Ok(ASTNode::Continue {
            span: Span::unknown(),
        })
    }

    /// Parse return statement
    pub(super) fn parse_return(&mut self) -> Result<ASTNode, ParseError> {
        if super::helpers::cursor_enabled() {
            let mut cursor = TokenCursor::new(&self.tokens);
            cursor.set_position(self.current);
            cursor.with_stmt_mode(|c| c.skip_newlines());
            self.current = cursor.position();
        }
        self.advance(); // consume 'return'

        // Check if there's a return value
        let value = if self.is_at_end() || self.match_token(&TokenType::RBRACE) {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };

        Ok(ASTNode::Return {
            value,
            span: Span::unknown(),
        })
    }
}

fn option_some_presence_match_expr(temp_name: &str) -> ASTNode {
    ASTNode::EnumMatchExpr {
        enum_name: "Option".to_string(),
        scrutinee: Box::new(ASTNode::Variable {
            name: temp_name.to_string(),
            span: Span::unknown(),
        }),
        arms: vec![
            EnumMatchArm {
                variant_name: "Some".to_string(),
                binding_name: None,
                body: ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: Span::unknown(),
                },
            },
            EnumMatchArm {
                variant_name: "None".to_string(),
                binding_name: None,
                body: ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    span: Span::unknown(),
                },
            },
        ],
        else_expr: None,
        span: Span::unknown(),
    }
}

fn enum_variant_failure_match_expr(
    enum_name: &str,
    variant_name: &str,
    temp_name: &str,
    variants: &[EnumVariantDecl],
) -> ASTNode {
    ASTNode::EnumMatchExpr {
        enum_name: enum_name.to_string(),
        scrutinee: Box::new(ASTNode::Variable {
            name: temp_name.to_string(),
            span: Span::unknown(),
        }),
        arms: variants
            .iter()
            .map(|variant| EnumMatchArm {
                variant_name: variant.name.clone(),
                binding_name: None,
                body: ASTNode::Literal {
                    value: LiteralValue::Bool(variant.name != variant_name),
                    span: Span::unknown(),
                },
            })
            .collect(),
        else_expr: None,
        span: Span::unknown(),
    }
}

fn enum_variant_binding_local(
    enum_name: &str,
    variant_name: &str,
    binding_name: &str,
    temp_name: &str,
    variants: &[EnumVariantDecl],
) -> ASTNode {
    ASTNode::Local {
        variables: vec![binding_name.to_string()],
        initial_values: vec![Some(Box::new(ASTNode::EnumMatchExpr {
            enum_name: enum_name.to_string(),
            scrutinee: Box::new(ASTNode::Variable {
                name: temp_name.to_string(),
                span: Span::unknown(),
            }),
            arms: variants
                .iter()
                .map(|variant| EnumMatchArm {
                    variant_name: variant.name.clone(),
                    binding_name: if variant.name == variant_name {
                        Some(binding_name.to_string())
                    } else {
                        None
                    },
                    body: if variant.name == variant_name {
                        ASTNode::Variable {
                            name: binding_name.to_string(),
                            span: Span::unknown(),
                        }
                    } else {
                        ASTNode::Literal {
                            value: LiteralValue::Null,
                            span: Span::unknown(),
                        }
                    },
                })
                .collect(),
            else_expr: None,
            span: Span::unknown(),
        }))],
        declared_type_names: Vec::new(),
        span: Span::unknown(),
    }
}

fn option_some_binding_local(binding_name: &str, temp_name: &str) -> ASTNode {
    ASTNode::Local {
        variables: vec![binding_name.to_string()],
        initial_values: vec![Some(Box::new(ASTNode::EnumMatchExpr {
            enum_name: "Option".to_string(),
            scrutinee: Box::new(ASTNode::Variable {
                name: temp_name.to_string(),
                span: Span::unknown(),
            }),
            arms: vec![
                EnumMatchArm {
                    variant_name: "Some".to_string(),
                    binding_name: Some(binding_name.to_string()),
                    body: ASTNode::Variable {
                        name: binding_name.to_string(),
                        span: Span::unknown(),
                    },
                },
                EnumMatchArm {
                    variant_name: "None".to_string(),
                    binding_name: None,
                    body: ASTNode::Literal {
                        value: LiteralValue::Null,
                        span: Span::unknown(),
                    },
                },
            ],
            else_expr: None,
            span: Span::unknown(),
        }))],
        declared_type_names: Vec::new(),
        span: Span::unknown(),
    }
}
