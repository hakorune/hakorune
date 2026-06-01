//! Parser support for AST-level build conditionals.
//!
//! LANG-CFG-001 owns parser transport only. Build-config evaluation and
//! inactive branch pruning are later slices.

use crate::ast::{ASTNode, BuildPredicate, Span};
use crate::parser::common::ParserUtils;
use crate::parser::statements::helpers::AnnotationSite;
use crate::parser::{BuildMode, NyashParser, ParseError, ParserBuildConfig};
use crate::tokenizer::TokenType;

impl NyashParser {
    pub(super) fn prune_build_when_program(&self, ast: ASTNode) -> Result<ASTNode, ParseError> {
        let ASTNode::Program { statements, span } = ast else {
            return Ok(ast);
        };
        let statements = self.prune_build_when_items(statements)?;
        Ok(ASTNode::Program { statements, span })
    }

    fn prune_build_when_items(&self, items: Vec<ASTNode>) -> Result<Vec<ASTNode>, ParseError> {
        let mut out = Vec::new();
        for item in items {
            match item {
                ASTNode::BuildWhen {
                    predicate,
                    then_items,
                    else_items,
                    span,
                } => {
                    let selected = if eval_build_predicate(&predicate, &self.build_config, span)? {
                        then_items
                    } else {
                        else_items.unwrap_or_default()
                    };
                    out.extend(self.prune_build_when_items(selected)?);
                }
                other => out.push(other),
            }
        }
        Ok(out)
    }

    pub(super) fn parse_build_when_item(&mut self) -> Result<ASTNode, ParseError> {
        let line = self.current_token().line;
        self.consume(TokenType::WHEN)?;
        let predicate = self.parse_build_predicate()?;
        let then_items = self.parse_build_when_item_block()?;

        let else_items = if self.match_token(&TokenType::ELSE) {
            self.advance();
            if self.match_token(&TokenType::WHEN) {
                Some(vec![self.parse_build_when_item()?])
            } else {
                Some(self.parse_build_when_item_block()?)
            }
        } else {
            None
        };

        Ok(ASTNode::BuildWhen {
            predicate,
            then_items,
            else_items,
            span: Span::new(0, 0, line, 1),
        })
    }

    fn parse_build_when_item_block(&mut self) -> Result<Vec<ASTNode>, ParseError> {
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

            let mut item = if self.match_token(&TokenType::WHEN) {
                self.parse_build_when_item()?
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

    fn parse_build_predicate(&mut self) -> Result<BuildPredicate, ParseError> {
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

    fn parse_build_predicate_list(
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
}

fn eval_build_predicate(
    predicate: &BuildPredicate,
    config: &ParserBuildConfig,
    span: Span,
) -> Result<bool, ParseError> {
    match predicate {
        BuildPredicate::BuildFlag(flag) => Ok(match (flag.as_str(), &config.mode) {
            ("test", BuildMode::Test) => true,
            ("debug", BuildMode::Debug) => true,
            ("release", BuildMode::Release) => true,
            _ => false,
        }),
        BuildPredicate::Feature(name) => {
            if !config.known_features.contains(name) {
                return Err(ParseError::BuildCfg {
                    message: format!("unknown feature '{}'", name),
                    line: span.line,
                });
            }
            Ok(config.enabled_features.contains(name))
        }
        BuildPredicate::TargetEq { key, value } => Ok(match key.as_str() {
            "os" => &config.target_os == value,
            "arch" => &config.target_arch == value,
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
            Ok(&config.backend_kind == value)
        }
        BuildPredicate::Not(inner) => Ok(!eval_build_predicate(inner, config, span)?),
        BuildPredicate::All(items) => {
            for item in items {
                if !eval_build_predicate(item, config, span)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        BuildPredicate::Any(items) => {
            for item in items {
                if eval_build_predicate(item, config, span)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    }
}
