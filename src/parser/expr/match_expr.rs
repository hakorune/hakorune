use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;

#[path = "match_expr_impl.rs"]
mod match_expr_impl;

#[derive(Debug, Clone)]
struct RecordPatternBinding {
    field_name: String,
    binding_name: String,
}

#[derive(Debug, Clone)]
enum ParsedMatchArm {
    Lit {
        lits: Vec<LiteralValue>,
        guard: Option<ASTNode>,
        body: ASTNode,
        line: usize,
    },
    Named {
        name: String,
        binding: Option<String>,
        record_bindings: Option<Vec<RecordPatternBinding>>,
        tuple_bindings: Option<Vec<String>>,
        guard: Option<ASTNode>,
        body: ASTNode,
        line: usize,
    },
    Default {
        body: ASTNode,
        line: usize,
    },
}

impl NyashParser {
    /// match式: match <expr> { lit[ '|' lit ]* => <expr|block>, ..., _ => <expr|block> }
    /// MVP:
    /// - literal arms keep the existing MatchExpr path
    /// - type-pattern arms still lower to nested if-chains
    /// - known-enum shorthand arms (`Some(v)`, `None`) resolve only when the parsed arm set
    ///   fits a known enum inventory from the current source file
    pub(crate) fn expr_parse_match(&mut self) -> Result<ASTNode, ParseError> {
        self.advance(); // consume 'match'
        let scrutinee = self.parse_expression()?;
        self.consume(TokenType::LBRACE)?;

        let mut arms_any: Vec<ParsedMatchArm> = Vec::new();
        let mut saw_default = false;

        while !self.match_token(&TokenType::RBRACE) && !self.is_at_end() {
            while self.match_token(&TokenType::COMMA) || self.match_token(&TokenType::NEWLINE) {
                self.advance();
            }
            if self.match_token(&TokenType::RBRACE) {
                break;
            }

            let arm_line = self.current_token().line;
            let is_default =
                matches!(self.current_token().token_type, TokenType::IDENTIFIER(ref s) if s == "_");

            if is_default {
                if saw_default {
                    return Err(ParseError::InvalidMatchPattern {
                        detail: "duplicate `_` default arm".to_string(),
                        line: arm_line,
                    });
                }
                saw_default = true;
                self.advance(); // consume '_'
                if self.match_token(&TokenType::IF) {
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "'=>' (guard is not allowed for default arm)".to_string(),
                        line: self.current_token().line,
                    });
                }
                self.consume(TokenType::FatArrow)?;
                let body = self.parse_match_arm_body()?;
                arms_any.push(ParsedMatchArm::Default {
                    body,
                    line: arm_line,
                });
            } else if let Some((name, binding, record_bindings, tuple_bindings)) =
                self.parse_named_match_head()?
            {
                let guard = if self.match_token(&TokenType::IF) {
                    self.advance();
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                self.consume(TokenType::FatArrow)?;
                let body = self.parse_match_arm_body()?;
                arms_any.push(ParsedMatchArm::Named {
                    name,
                    binding,
                    record_bindings,
                    tuple_bindings,
                    guard,
                    body,
                    line: arm_line,
                });
            } else {
                let mut lits = vec![self.lit_only_for_match()?];
                while self.match_token(&TokenType::BitOr) {
                    self.advance(); // consume '|'
                    lits.push(self.lit_only_for_match()?);
                }
                let guard = if self.match_token(&TokenType::IF) {
                    self.advance();
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                self.consume(TokenType::FatArrow)?;
                let body = self.parse_match_arm_body()?;
                arms_any.push(ParsedMatchArm::Lit {
                    lits,
                    guard,
                    body,
                    line: arm_line,
                });
            }

            while self.match_token(&TokenType::COMMA) || self.match_token(&TokenType::NEWLINE) {
                self.advance();
            }
        }

        self.consume(TokenType::RBRACE)?;

        if let Some(enum_name) = self.resolve_known_enum_match(&scrutinee, &arms_any) {
            return self.build_known_enum_match_ast(scrutinee, enum_name, arms_any);
        }

        let default_expr = arms_any
            .iter()
            .find_map(|arm| match arm {
                ParsedMatchArm::Default { body, .. } => Some(body.clone()),
                _ => None,
            })
            .ok_or(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "_ => <expr> in match".to_string(),
                line: self.current_token().line,
            })?;

        let saw_named = arms_any
            .iter()
            .any(|arm| matches!(arm, ParsedMatchArm::Named { .. }));
        let saw_guard = arms_any.iter().any(|arm| {
            matches!(
                arm,
                ParsedMatchArm::Lit { guard: Some(_), .. }
                    | ParsedMatchArm::Named { guard: Some(_), .. }
            )
        });

        if !saw_named && !saw_guard {
            let mut lit_arms: Vec<(LiteralValue, ASTNode)> = Vec::new();
            for arm in arms_any.into_iter() {
                match arm {
                    ParsedMatchArm::Lit { lits, body, .. } => {
                        for lit in lits {
                            lit_arms.push((lit, body.clone()));
                        }
                    }
                    ParsedMatchArm::Default { .. } => {}
                    ParsedMatchArm::Named { .. } => unreachable!(),
                }
            }
            return Ok(ASTNode::MatchExpr {
                scrutinee: Box::new(scrutinee),
                arms: lit_arms,
                else_expr: Box::new(default_expr),
                span: Span::unknown(),
            });
        }

        let scr_var = format!("__ny_match_scrutinee_{}", self.current());
        let scr_local = ASTNode::Local {
            variables: vec![scr_var.clone()],
            initial_values: vec![Some(Box::new(scrutinee))],
            span: Span::unknown(),
        };

        let mut else_node = ASTNode::Program {
            statements: vec![default_expr],
            span: Span::unknown(),
        };

        for arm in arms_any.into_iter().rev() {
            match arm {
                ParsedMatchArm::Default { .. } => {}
                ParsedMatchArm::Lit {
                    lits, guard, body, ..
                } => {
                    let mut cond: Option<ASTNode> = None;
                    for lit in lits {
                        let eq = ASTNode::BinaryOp {
                            operator: BinaryOperator::Equal,
                            left: Box::new(ASTNode::Variable {
                                name: scr_var.clone(),
                                span: Span::unknown(),
                            }),
                            right: Box::new(ASTNode::Literal {
                                value: lit,
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        };
                        cond = Some(match cond {
                            None => eq,
                            Some(prev) => ASTNode::BinaryOp {
                                operator: BinaryOperator::Or,
                                left: Box::new(prev),
                                right: Box::new(eq),
                                span: Span::unknown(),
                            },
                        });
                    }
                    let else_statements = match else_node.clone() {
                        ASTNode::Program { statements, .. } => statements,
                        other => vec![other],
                    };
                    let then_body_statements = if let Some(g) = guard {
                        let guard_if = ASTNode::If {
                            condition: Box::new(g),
                            then_body: vec![body],
                            else_body: Some(else_statements.clone()),
                            span: Span::unknown(),
                        };
                        vec![guard_if]
                    } else {
                        vec![body]
                    };
                    else_node = ASTNode::If {
                        condition: Box::new(
                            cond.expect("literal arm must have at least one literal"),
                        ),
                        then_body: then_body_statements,
                        else_body: Some(else_statements),
                        span: Span::unknown(),
                    };
                }
                ParsedMatchArm::Named {
                    name,
                    binding,
                    record_bindings,
                    tuple_bindings,
                    guard,
                    body,
                    line,
                } => {
                    if record_bindings.is_some() || tuple_bindings.is_some() {
                        let surface = if record_bindings.is_some() {
                            "record"
                        } else {
                            "tuple"
                        };
                        return Err(ParseError::InvalidMatchPattern {
                            detail: format!(
                                "{} arm `{}` requires a known enum shorthand context",
                                surface, name
                            ),
                            line,
                        });
                    }
                    let Some(bind) = binding else {
                        return Err(ParseError::InvalidMatchPattern {
                            detail: format!(
                                "bare arm `{}` requires a known enum shorthand context",
                                name
                            ),
                            line,
                        });
                    };
                    let is_call = ASTNode::MethodCall {
                        object: Box::new(ASTNode::Variable {
                            name: scr_var.clone(),
                            span: Span::unknown(),
                        }),
                        method: "is".to_string(),
                        arguments: vec![ASTNode::Literal {
                            value: LiteralValue::String(name.clone()),
                            span: Span::unknown(),
                        }],
                        span: Span::unknown(),
                    };
                    let cast = ASTNode::MethodCall {
                        object: Box::new(ASTNode::Variable {
                            name: scr_var.clone(),
                            span: Span::unknown(),
                        }),
                        method: "as".to_string(),
                        arguments: vec![ASTNode::Literal {
                            value: LiteralValue::String(name.clone()),
                            span: Span::unknown(),
                        }],
                        span: Span::unknown(),
                    };
                    let bind_local = ASTNode::Local {
                        variables: vec![bind.clone()],
                        initial_values: vec![Some(Box::new(cast))],
                        span: Span::unknown(),
                    };
                    let else_statements = match else_node.clone() {
                        ASTNode::Program { statements, .. } => statements,
                        other => vec![other],
                    };
                    let then_body_statements = if let Some(g) = guard {
                        let guard_if = ASTNode::If {
                            condition: Box::new(g),
                            then_body: vec![body],
                            else_body: Some(else_statements.clone()),
                            span: Span::unknown(),
                        };
                        vec![bind_local, guard_if]
                    } else {
                        vec![bind_local, body]
                    };
                    else_node = ASTNode::If {
                        condition: Box::new(is_call),
                        then_body: then_body_statements,
                        else_body: Some(else_statements),
                        span: Span::unknown(),
                    };
                }
            }
        }

        Ok(ASTNode::Program {
            statements: vec![scr_local, else_node],
            span: Span::unknown(),
        })
    }
}
