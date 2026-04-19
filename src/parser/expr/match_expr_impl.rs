use super::{ParsedMatchArm, RecordPatternBinding};
use crate::ast::{ASTNode, EnumMatchArm, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::BTreeSet;

impl NyashParser {
    pub(super) fn resolve_known_enum_match(
        &self,
        scrutinee: &ASTNode,
        arms: &[ParsedMatchArm],
    ) -> Option<String> {
        let has_named = arms
            .iter()
            .any(|arm| matches!(arm, ParsedMatchArm::Named { .. }));
        if !has_named
            || arms
                .iter()
                .any(|arm| matches!(arm, ParsedMatchArm::Lit { .. }))
        {
            return None;
        }

        if let ASTNode::FromCall { parent, .. } = scrutinee {
            if self.named_arms_could_belong_to_enum(parent, arms) {
                return Some(parent.clone());
            }
        }

        let mut candidates = self
            .known_enums
            .keys()
            .filter(|enum_name| self.named_arms_could_belong_to_enum(enum_name, arms))
            .cloned()
            .collect::<Vec<_>>();
        if candidates.len() == 1 {
            candidates.pop()
        } else {
            None
        }
    }

    pub(super) fn named_arms_could_belong_to_enum(
        &self,
        enum_name: &str,
        arms: &[ParsedMatchArm],
    ) -> bool {
        let Some(variants) = self.known_enums.get(enum_name) else {
            return false;
        };
        arms.iter().all(|arm| match arm {
            ParsedMatchArm::Named {
                name,
                binding,
                record_bindings,
                tuple_bindings,
                ..
            } => variants
                .iter()
                .find(|variant| variant.name == *name)
                .map(|variant| match (record_bindings, tuple_bindings, binding) {
                    (Some(_), None, None) => variant.is_record_payload(),
                    (None, Some(tuple_bindings), None) => {
                        variant.is_multi_payload_tuple()
                            && tuple_bindings.len() == variant.payload_arity()
                    }
                    (None, None, Some(_)) => {
                        !variant.is_record_payload()
                            && !variant.is_multi_payload_tuple()
                            && variant.payload_arity() == 1
                    }
                    (None, None, None) => !variant.has_payload(),
                    _ => false,
                })
                .unwrap_or(false),
            ParsedMatchArm::Default { .. } => true,
            ParsedMatchArm::Lit { .. } => false,
        })
    }

    pub(super) fn build_known_enum_match_ast(
        &self,
        scrutinee: ASTNode,
        enum_name: String,
        arms: Vec<ParsedMatchArm>,
    ) -> Result<ASTNode, ParseError> {
        let Some(variants) = self.known_enums.get(&enum_name) else {
            return Err(ParseError::InvalidMatchPattern {
                detail: format!("unknown enum `{}`", enum_name),
                line: 0,
            });
        };

        let expected_variants = variants
            .iter()
            .map(|variant| variant.name.clone())
            .collect::<BTreeSet<_>>();
        let mut covered_variants = BTreeSet::new();
        let mut enum_arms = Vec::new();
        let mut else_expr = None;
        let mut anchor_line = 0usize;

        for arm in arms {
            match arm {
                ParsedMatchArm::Named {
                    name,
                    binding,
                    record_bindings,
                    tuple_bindings,
                    guard,
                    body,
                    line,
                } => {
                    if anchor_line == 0 {
                        anchor_line = line;
                    }
                    if guard.is_some() {
                        return Err(ParseError::InvalidMatchPattern {
                            detail: format!(
                                "guarded enum shorthand arm `{}` is not supported in the MVP",
                                name
                            ),
                            line,
                        });
                    }
                    let Some(variant) = variants.iter().find(|variant| variant.name == name) else {
                        return Err(ParseError::InvalidMatchPattern {
                            detail: format!("unknown variant `{}` for enum `{}`", name, enum_name),
                            line,
                        });
                    };
                    if let Some(record_bindings) = record_bindings {
                        if !variant.is_record_payload() {
                            return Err(ParseError::InvalidMatchPattern {
                                detail: format!(
                                    "tuple/unit variant `{}` for `{}` does not accept a record pattern",
                                    name, enum_name
                                ),
                                line,
                            });
                        }
                        validate_record_pattern_fields(
                            &enum_name,
                            &name,
                            variant,
                            &record_bindings,
                            line,
                        )?;
                        let payload_binding =
                            format!("__ny_enum_record_payload_{}_{}", enum_arms.len(), name);
                        let wrapped_body = wrap_compat_payload_enum_arm_body(
                            &payload_binding,
                            &record_bindings,
                            body,
                            line,
                        )?;
                        if !covered_variants.insert(name.clone()) {
                            return Err(ParseError::InvalidMatchPattern {
                                detail: format!(
                                    "duplicate enum variant arm `{}` in match for `{}`",
                                    name, enum_name
                                ),
                                line,
                            });
                        }
                        enum_arms.push(EnumMatchArm {
                            variant_name: name,
                            binding_name: Some(payload_binding),
                            body: wrapped_body,
                        });
                        continue;
                    }
                    if let Some(tuple_bindings) = tuple_bindings {
                        if !variant.is_multi_payload_tuple() {
                            return Err(ParseError::InvalidMatchPattern {
                                detail: format!(
                                    "unit/single-payload/record variant `{}` for `{}` does not accept a tuple pattern",
                                    name, enum_name
                                ),
                                line,
                            });
                        }
                        let payload_binding =
                            format!("__ny_enum_tuple_payload_{}_{}", enum_arms.len(), name);
                        let wrapped_body = wrap_compat_payload_enum_arm_body(
                            &payload_binding,
                            &tuple_pattern_bindings(variant, &tuple_bindings, line)?,
                            body,
                            line,
                        )?;
                        if !covered_variants.insert(name.clone()) {
                            return Err(ParseError::InvalidMatchPattern {
                                detail: format!(
                                    "duplicate enum variant arm `{}` in match for `{}`",
                                    name, enum_name
                                ),
                                line,
                            });
                        }
                        enum_arms.push(EnumMatchArm {
                            variant_name: name,
                            binding_name: Some(payload_binding),
                            body: wrapped_body,
                        });
                        continue;
                    }
                    if variant.is_record_payload() {
                        return Err(ParseError::InvalidMatchPattern {
                            detail: format!(
                                "record variant `{}` for `{}` requires a record pattern",
                                name, enum_name
                            ),
                            line,
                        });
                    }
                    if variant.is_multi_payload_tuple() {
                        return Err(ParseError::InvalidMatchPattern {
                            detail: format!(
                                "enum variant `{}` for `{}` requires exactly {} tuple binding(s)",
                                name,
                                enum_name,
                                variant.payload_arity()
                            ),
                            line,
                        });
                    }
                    if variant.has_payload() != binding.is_some() {
                        let detail = if variant.payload_type_name.is_some() {
                            format!(
                                "enum variant `{}` for `{}` requires exactly one binding",
                                name, enum_name
                            )
                        } else {
                            format!(
                                "unit variant `{}` for `{}` must not bind a payload",
                                name, enum_name
                            )
                        };
                        return Err(ParseError::InvalidMatchPattern { detail, line });
                    }
                    if !covered_variants.insert(name.clone()) {
                        return Err(ParseError::InvalidMatchPattern {
                            detail: format!(
                                "duplicate enum variant arm `{}` in match for `{}`",
                                name, enum_name
                            ),
                            line,
                        });
                    }
                    enum_arms.push(EnumMatchArm {
                        variant_name: name,
                        binding_name: binding,
                        body,
                    });
                }
                ParsedMatchArm::Default { body, line } => {
                    if anchor_line == 0 {
                        anchor_line = line;
                    }
                    if else_expr.replace(Box::new(body)).is_some() {
                        return Err(ParseError::InvalidMatchPattern {
                            detail: "duplicate `_` default arm".to_string(),
                            line,
                        });
                    }
                }
                ParsedMatchArm::Lit { line, .. } => {
                    return Err(ParseError::InvalidMatchPattern {
                        detail: format!(
                            "literal arms cannot mix with enum shorthand match for `{}`",
                            enum_name
                        ),
                        line,
                    });
                }
            }
        }

        let missing = expected_variants
            .difference(&covered_variants)
            .cloned()
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            let suffix = if else_expr.is_some() {
                " (`_` does not satisfy known-enum exhaustiveness)"
            } else {
                ""
            };
            return Err(ParseError::InvalidMatchPattern {
                detail: format!(
                    "non-exhaustive enum match for `{}`; missing variant(s): {}{}",
                    enum_name,
                    missing.join(", "),
                    suffix
                ),
                line: anchor_line,
            });
        }

        Ok(ASTNode::EnumMatchExpr {
            enum_name,
            scrutinee: Box::new(scrutinee),
            arms: enum_arms,
            else_expr,
            span: Span::unknown(),
        })
    }

    pub(super) fn parse_named_match_head(
        &mut self,
    ) -> Result<
        Option<(
            String,
            Option<String>,
            Option<Vec<RecordPatternBinding>>,
            Option<Vec<String>>,
        )>,
        ParseError,
    > {
        let TokenType::IDENTIFIER(name) = self.current_token().token_type.clone() else {
            return Ok(None);
        };

        if self.peek_token() == &TokenType::LPAREN {
            self.advance(); // TypeName / VariantName
            self.consume(TokenType::LPAREN)?;
            let mut bindings = Vec::new();
            loop {
                let binding = match self.current_token().token_type.clone() {
                    TokenType::IDENTIFIER(binding) => {
                        self.advance();
                        binding
                    }
                    other => {
                        return Err(ParseError::UnexpectedToken {
                            found: other,
                            expected: "identifier".to_string(),
                            line: self.current_token().line,
                        });
                    }
                };
                bindings.push(binding);
                if self.match_token(&TokenType::COMMA) {
                    self.advance();
                    continue;
                }
                break;
            }
            self.consume(TokenType::RPAREN)?;
            if bindings.len() == 1 {
                return Ok(Some((name, bindings.pop(), None, None)));
            }
            return Ok(Some((name, None, None, Some(bindings))));
        }

        if self.peek_token() == &TokenType::LBRACE {
            self.advance(); // VariantName
            let record_bindings = self.parse_record_match_bindings()?;
            return Ok(Some((name, None, Some(record_bindings), None)));
        }

        if matches!(self.peek_token(), &TokenType::IF | &TokenType::FatArrow) {
            self.advance(); // bare unit shorthand / unresolved bare head
            return Ok(Some((name, None, None, None)));
        }

        Ok(None)
    }

    pub(super) fn parse_record_match_bindings(
        &mut self,
    ) -> Result<Vec<RecordPatternBinding>, ParseError> {
        let line = self.current_token().line;
        self.consume(TokenType::LBRACE)?;
        let mut bindings = Vec::new();
        while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
            if self.match_token(&TokenType::COMMA) || self.match_token(&TokenType::NEWLINE) {
                self.advance();
                continue;
            }

            let field_name = match self.current_token().token_type.clone() {
                TokenType::IDENTIFIER(name) => {
                    self.advance();
                    name
                }
                other => {
                    return Err(ParseError::UnexpectedToken {
                        found: other,
                        expected: "record pattern field name".to_string(),
                        line: self.current_token().line,
                    });
                }
            };
            let binding_name = if self.match_token(&TokenType::COLON) {
                self.advance();
                match self.current_token().token_type.clone() {
                    TokenType::IDENTIFIER(binding) => {
                        self.advance();
                        binding
                    }
                    other => {
                        return Err(ParseError::UnexpectedToken {
                            found: other,
                            expected: "record pattern binding name".to_string(),
                            line: self.current_token().line,
                        });
                    }
                }
            } else {
                field_name.clone()
            };
            bindings.push(RecordPatternBinding {
                field_name,
                binding_name,
            });

            if self.match_token(&TokenType::COMMA) {
                self.advance();
            }
        }
        self.consume(TokenType::RBRACE)?;
        if bindings.is_empty() {
            return Err(ParseError::InvalidMatchPattern {
                detail: "record enum pattern requires at least one field".to_string(),
                line,
            });
        }
        Ok(bindings)
    }

    pub(super) fn parse_match_arm_body(&mut self) -> Result<ASTNode, ParseError> {
        if self.match_token(&TokenType::LBRACE) {
            if self.is_object_literal() {
                self.parse_expression()
            } else {
                self.advance(); // consume '{'
                let mut statements = Vec::new();
                while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
                    if !self.match_token(&TokenType::RBRACE) {
                        statements.push(self.parse_statement()?);
                    }
                }
                self.consume(TokenType::RBRACE)?;
                Ok(ASTNode::Program {
                    statements,
                    span: Span::unknown(),
                })
            }
        } else {
            self.parse_expression()
        }
    }

    /// オブジェクトリテラル判定: { IDENTIFIER : または { STRING : の場合はtrue
    pub(super) fn is_object_literal(&self) -> bool {
        if !matches!(self.current_token().token_type, TokenType::LBRACE) {
            return false;
        }
        let mut lookahead_idx = 1;
        while matches!(self.peek_nth_token(lookahead_idx), TokenType::NEWLINE) {
            lookahead_idx += 1;
        }
        match self.peek_nth_token(lookahead_idx) {
            TokenType::IDENTIFIER(_) | TokenType::STRING(_) => {
                lookahead_idx += 1;
                while matches!(self.peek_nth_token(lookahead_idx), TokenType::NEWLINE) {
                    lookahead_idx += 1;
                }
                matches!(self.peek_nth_token(lookahead_idx), TokenType::COLON)
            }
            _ => false,
        }
    }

    // match 用の最小リテラルパーサ（式は受け付けない）
    pub(super) fn lit_only_for_match(&mut self) -> Result<crate::ast::LiteralValue, ParseError> {
        match &self.current_token().token_type {
            TokenType::STRING(s) => {
                let v = crate::ast::LiteralValue::String(s.clone());
                self.advance();
                Ok(v)
            }
            TokenType::NUMBER(n) => {
                let v = crate::ast::LiteralValue::Integer(*n);
                self.advance();
                Ok(v)
            }
            TokenType::FLOAT(f) => {
                let v = crate::ast::LiteralValue::Float(*f);
                self.advance();
                Ok(v)
            }
            TokenType::TRUE => {
                self.advance();
                Ok(crate::ast::LiteralValue::Bool(true))
            }
            TokenType::FALSE => {
                self.advance();
                Ok(crate::ast::LiteralValue::Bool(false))
            }
            TokenType::NULL => {
                self.advance();
                Ok(crate::ast::LiteralValue::Null)
            }
            TokenType::VOID => {
                self.advance();
                Ok(crate::ast::LiteralValue::Void)
            }
            _ => {
                let line = self.current_token().line;
                Err(ParseError::UnexpectedToken {
                    found: self.current_token().token_type.clone(),
                    expected: "literal".to_string(),
                    line,
                })
            }
        }
    }
}

fn validate_record_pattern_fields(
    enum_name: &str,
    variant_name: &str,
    variant: &crate::ast::EnumVariantDecl,
    record_bindings: &[RecordPatternBinding],
    line: usize,
) -> Result<(), ParseError> {
    let mut actual = BTreeSet::new();
    for binding in record_bindings {
        if !actual.insert(binding.field_name.clone()) {
            return Err(ParseError::InvalidMatchPattern {
                detail: format!(
                    "duplicate record field `{}` in enum pattern {}::{}",
                    binding.field_name, enum_name, variant_name
                ),
                line,
            });
        }
    }

    let expected = variant
        .record_field_decls
        .iter()
        .map(|field| field.name.clone())
        .collect::<BTreeSet<_>>();
    let missing = expected.difference(&actual).cloned().collect::<Vec<_>>();
    let unknown = actual.difference(&expected).cloned().collect::<Vec<_>>();
    if !missing.is_empty() || !unknown.is_empty() {
        let mut pieces = Vec::new();
        if !missing.is_empty() {
            pieces.push(format!("missing field(s): {}", missing.join(", ")));
        }
        if !unknown.is_empty() {
            pieces.push(format!("unknown field(s): {}", unknown.join(", ")));
        }
        return Err(ParseError::InvalidMatchPattern {
            detail: format!(
                "record enum pattern for {}::{} must bind the declared field set exactly ({})",
                enum_name,
                variant_name,
                pieces.join("; ")
            ),
            line,
        });
    }
    Ok(())
}

fn wrap_compat_payload_enum_arm_body(
    payload_binding: &str,
    record_bindings: &[RecordPatternBinding],
    body: ASTNode,
    line: usize,
) -> Result<ASTNode, ParseError> {
    if matches!(body, ASTNode::Program { .. }) {
        return Err(ParseError::InvalidMatchPattern {
            detail: "payload-box enum shorthand block bodies are outside the first compat cut"
                .to_string(),
            line,
        });
    }

    let prelude_stmts = record_bindings
        .iter()
        .map(|binding| ASTNode::Local {
            variables: vec![binding.binding_name.clone()],
            initial_values: vec![Some(Box::new(ASTNode::FieldAccess {
                object: Box::new(ASTNode::Variable {
                    name: payload_binding.to_string(),
                    span: Span::unknown(),
                }),
                field: binding.field_name.clone(),
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        })
        .collect();

    Ok(ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr: Box::new(body),
        span: Span::unknown(),
    })
}

fn tuple_pattern_bindings(
    variant: &crate::ast::EnumVariantDecl,
    tuple_bindings: &[String],
    line: usize,
) -> Result<Vec<RecordPatternBinding>, ParseError> {
    let field_decls = variant.compat_payload_field_decls();
    if field_decls.len() != tuple_bindings.len() {
        return Err(ParseError::InvalidMatchPattern {
            detail: format!(
                "tuple enum pattern requires exactly {} binding(s)",
                field_decls.len()
            ),
            line,
        });
    }
    Ok(field_decls
        .into_iter()
        .zip(tuple_bindings.iter())
        .map(|(field_decl, binding_name)| RecordPatternBinding {
            field_name: field_decl.name,
            binding_name: binding_name.clone(),
        })
        .collect())
}
