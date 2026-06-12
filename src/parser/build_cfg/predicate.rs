use crate::ast::{ASTNode, BuildPredicate, Span};
use crate::parser::common::ParserUtils;
use crate::parser::statements::helpers::AnnotationSite;
use crate::parser::{BuildMode, NyashParser, ParseError};
use crate::tokenizer::TokenType;

use super::BuildGateExplainReport;

impl NyashParser {
    pub(crate) fn is_build_gate_head(&self) -> bool {
        if !matches!(
            &self.current_token().token_type,
            TokenType::IDENTIFIER(name) if name == "gate"
        ) {
            return false;
        }
        let Some(next) = self.tokens.get(self.current + 1) else {
            return false;
        };
        matches!(
            &next.token_type,
            TokenType::IDENTIFIER(name)
                if matches!(
                    name.as_str(),
                    "Build" | "Feature" | "Target" | "Backend" | "all" | "any"
                )
        ) || matches!(next.token_type, TokenType::NOT)
    }

    pub(crate) fn parse_build_gate_item(&mut self) -> Result<ASTNode, ParseError> {
        let line = self.current_token().line;
        self.consume_build_gate_head()?;
        let predicate = self.parse_build_predicate()?;
        let then_items = self.parse_build_gate_item_block()?;

        let else_items = if self.match_token(&TokenType::ELSE) {
            self.advance();
            if self.is_build_gate_head() {
                Some(vec![self.parse_build_gate_item()?])
            } else {
                Some(self.parse_build_gate_item_block()?)
            }
        } else {
            None
        };

        Ok(ASTNode::BuildGate {
            predicate,
            then_items,
            else_items,
            span: Span::new(0, 0, line, 1),
        })
    }

    pub(crate) fn consume_build_gate_head(&mut self) -> Result<(), ParseError> {
        if self.is_build_gate_head() {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "gate".to_string(),
                line: self.current_token().line,
            })
        }
    }

    fn parse_build_gate_item_block(&mut self) -> Result<Vec<ASTNode>, ParseError> {
        self.consume(TokenType::LBRACE)?;
        let mut items = Vec::new();

        while !self.is_at_end() {
            self.skip_statement_separators();
            if self.match_token(&TokenType::RBRACE) {
                break;
            }
            if self.maybe_parse_opt_annotation_noop(AnnotationSite::TopLevel)? {
                continue;
            }

            let mut item = if self.is_build_gate_head() {
                self.parse_build_gate_item()?
            } else {
                self.parse_statement()?
            };
            self.attach_pending_runes_to_declaration(&mut item)?;
            items.push(item);
        }

        self.consume(TokenType::RBRACE)?;
        Ok(items)
    }

    fn skip_statement_separators(&mut self) {
        let allow_sc = std::env::var("NYASH_PARSER_ALLOW_SEMICOLON")
            .ok()
            .map(|v| {
                let lv = v.to_ascii_lowercase();
                !(lv == "0" || lv == "false" || lv == "off")
            })
            .unwrap_or(true);

        while self.match_token(&TokenType::NEWLINE)
            || (allow_sc && self.match_token(&TokenType::SEMICOLON))
        {
            self.advance();
        }
    }

    pub(crate) fn parse_build_predicate(&mut self) -> Result<BuildPredicate, ParseError> {
        let name = self.consume_identifier_like("build predicate head")?;
        match name.as_str() {
            "Build" => {
                self.consume(TokenType::DOT)?;
                let flag = self.consume_identifier_like("Build predicate flag")?;
                match flag.as_str() {
                    "test" | "debug" | "release" => Ok(BuildPredicate::BuildFlag(flag)),
                    _ => Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "Build.test, Build.debug, or Build.release".to_string(),
                        line: self.current_token().line,
                    }),
                }
            }
            "Feature" => {
                self.consume(TokenType::LPAREN)?;
                let feature = self.consume_string_literal("Feature name")?;
                self.consume(TokenType::RPAREN)?;
                Ok(BuildPredicate::Feature(feature))
            }
            "Target" => {
                self.consume(TokenType::DOT)?;
                let key = self.consume_identifier_like("Target predicate key")?;
                match key.as_str() {
                    "os" | "arch" => {}
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            found: self.current_token().token_type.clone(),
                            expected: "Target.os or Target.arch".to_string(),
                            line: self.current_token().line,
                        })
                    }
                }
                self.consume(TokenType::EQUALS)?;
                let value = self.consume_identifier_like("Target predicate value")?;
                Ok(BuildPredicate::TargetEq { key, value })
            }
            "Backend" => {
                self.consume(TokenType::DOT)?;
                let key = self.consume_identifier_like("Backend predicate key")?;
                if key != "kind" {
                    return Err(ParseError::UnexpectedToken {
                        found: self.current_token().token_type.clone(),
                        expected: "Backend.kind".to_string(),
                        line: self.current_token().line,
                    });
                }
                self.consume(TokenType::EQUALS)?;
                let value = self.consume_identifier_like("Backend predicate value")?;
                Ok(BuildPredicate::BackendEq { key, value })
            }
            "not" => {
                self.consume(TokenType::LPAREN)?;
                let inner = self.parse_build_predicate()?;
                self.consume(TokenType::RPAREN)?;
                Ok(BuildPredicate::Not(Box::new(inner)))
            }
            "all" => self.parse_build_predicate_list(BuildPredicate::All),
            "any" => self.parse_build_predicate_list(BuildPredicate::Any),
            _ => Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: "Build, Feature, Target, Backend, not, all, or any".to_string(),
                line: self.current_token().line,
            }),
        }
    }

    pub(crate) fn parse_build_predicate_list(
        &mut self,
        build: fn(Vec<BuildPredicate>) -> BuildPredicate,
    ) -> Result<BuildPredicate, ParseError> {
        self.consume(TokenType::LPAREN)?;
        let mut predicates = Vec::new();
        predicates.push(self.parse_build_predicate()?);
        while self.match_token(&TokenType::COMMA) {
            self.advance();
            predicates.push(self.parse_build_predicate()?);
        }
        self.consume(TokenType::RPAREN)?;
        Ok(build(predicates))
    }

    fn consume_identifier_like(&mut self, expected: &str) -> Result<String, ParseError> {
        let token = self.current_token().token_type.clone();
        let out = match token {
            TokenType::IDENTIFIER(name) => Some(name),
            TokenType::NOT => Some("not".to_string()),
            _ => None,
        };
        if let Some(name) = out {
            self.advance();
            Ok(name)
        } else {
            Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: expected.to_string(),
                line: self.current_token().line,
            })
        }
    }

    fn consume_string_literal(&mut self, expected: &str) -> Result<String, ParseError> {
        let token = self.current_token().token_type.clone();
        if let TokenType::STRING(value) = token {
            self.advance();
            Ok(value)
        } else {
            Err(ParseError::UnexpectedToken {
                found: self.current_token().token_type.clone(),
                expected: expected.to_string(),
                line: self.current_token().line,
            })
        }
    }

    pub(crate) fn explain_build_gate_program(
        &self,
        ast: &ASTNode,
    ) -> Result<BuildGateExplainReport, ParseError> {
        let mut report = BuildGateExplainReport::new();
        self.collect_build_gate_explain(ast, &mut report)?;
        Ok(report)
    }

    fn collect_build_gate_explain(
        &self,
        node: &ASTNode,
        report: &mut BuildGateExplainReport,
    ) -> Result<(), ParseError> {
        match node {
            ASTNode::BuildGate {
                predicate,
                then_items,
                else_items,
                span,
            } => {
                report.conditional_group_count += 1;
                if self.eval_build_predicate(predicate, *span)? {
                    report.active_branch_count += 1;
                    if else_items.is_some() {
                        report.inactive_branch_count += 1;
                    }
                    for item in then_items {
                        self.collect_build_gate_explain(item, report)?;
                    }
                } else if let Some(else_items) = else_items {
                    report.active_branch_count += 1;
                    report.inactive_branch_count += 1;
                    for item in else_items {
                        self.collect_build_gate_explain(item, report)?;
                    }
                } else {
                    report.inactive_branch_count += 1;
                }
            }
            _ => {
                let mut child_result: Result<(), ParseError> = Ok(());
                node.for_each_child(&mut |child| {
                    if child_result.is_ok() {
                        child_result = self.collect_build_gate_explain(child, report);
                    }
                });
                child_result?;
            }
        }
        Ok(())
    }

    pub(crate) fn eval_build_predicate(
        &self,
        predicate: &BuildPredicate,
        span: Span,
    ) -> Result<bool, ParseError> {
        match predicate {
            BuildPredicate::BuildFlag(flag) => Ok(match (flag.as_str(), &self.build_config.mode) {
                ("test", BuildMode::Test) => true,
                ("debug", BuildMode::Debug) => true,
                ("release", BuildMode::Release) => true,
                _ => false,
            }),
            BuildPredicate::Feature(name) => {
                if !self.build_config.known_features.contains(name) {
                    return Err(ParseError::BuildCfg {
                        message: format!("unknown feature '{}'", name),
                        line: span.line,
                    });
                }
                Ok(self.build_config.enabled_features.contains(name))
            }
            BuildPredicate::TargetEq { key, value } => Ok(match key.as_str() {
                "os" => &self.build_config.target_os == value,
                "arch" => &self.build_config.target_arch == value,
                _ => {
                    return Err(ParseError::BuildCfg {
                        message: format!("unsupported Target key '{}'", key),
                        line: span.line,
                    })
                }
            }),
            BuildPredicate::BackendEq { key, value } => {
                if key != "kind" {
                    return Err(ParseError::BuildCfg {
                        message: format!("unsupported Backend key '{}'", key),
                        line: span.line,
                    });
                }
                Ok(&self.build_config.backend_kind == value)
            }
            BuildPredicate::Not(inner) => Ok(!self.eval_build_predicate(inner, span)?),
            BuildPredicate::All(items) => {
                for item in items {
                    if !self.eval_build_predicate(item, span)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            BuildPredicate::Any(items) => {
                for item in items {
                    if self.eval_build_predicate(item, span)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
        }
    }
}
