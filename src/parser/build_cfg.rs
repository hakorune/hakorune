//! Parser support for AST-level build conditionals.
//!
//! `gate` is intentionally parser-contextual instead of a tokenizer keyword so
//! existing source may keep ordinary identifiers named `gate`.

use crate::ast::{ASTNode, BuildPredicate, Span};
use crate::parser::common::ParserUtils;
use crate::parser::statements::helpers::AnnotationSite;
use crate::parser::{BuildMode, NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildGateExplainReport {
    pub output_contract: &'static str,
    pub conditional_group_count: usize,
    pub active_branch_count: usize,
    pub inactive_branch_count: usize,
    pub inactive_branch_mir_count: usize,
}

impl BuildGateExplainReport {
    pub const OUTPUT_CONTRACT: &'static str = "hakorune-build-cfg-explain-v0";

    pub fn new() -> Self {
        Self {
            output_contract: Self::OUTPUT_CONTRACT,
            conditional_group_count: 0,
            active_branch_count: 0,
            inactive_branch_count: 0,
            inactive_branch_mir_count: 0,
        }
    }

    pub fn to_kv_lines(&self) -> Vec<String> {
        vec![
            format!("output_contract={}", self.output_contract),
            format!("conditional_group_count={}", self.conditional_group_count),
            format!("active_branch_count={}", self.active_branch_count),
            format!("inactive_branch_count={}", self.inactive_branch_count),
            format!(
                "inactive_branch_mir_count={}",
                self.inactive_branch_mir_count
            ),
            "summary=ok".to_string(),
        ]
    }
}

impl NyashParser {
    pub(super) fn prune_build_gate_program(&self, ast: ASTNode) -> Result<ASTNode, ParseError> {
        match ast {
            ASTNode::Program { statements, span } => Ok(ASTNode::Program {
                statements: self.prune_build_gate_items(statements)?,
                span,
            }),
            other => Ok(other),
        }
    }

    fn prune_build_gate_items(&self, items: Vec<ASTNode>) -> Result<Vec<ASTNode>, ParseError> {
        let mut out = Vec::new();
        for item in items {
            match item {
                ASTNode::BuildGate {
                    predicate,
                    then_items,
                    else_items,
                    span,
                } => {
                    let selected = if self.eval_build_predicate(&predicate, span)? {
                        then_items
                    } else {
                        else_items.unwrap_or_default()
                    };
                    out.extend(self.prune_build_gate_items(selected)?);
                }
                ASTNode::Program { statements, span } => {
                    out.push(ASTNode::Program {
                        statements: self.prune_build_gate_items(statements)?,
                        span,
                    });
                }
                ASTNode::ScopeBox { body, span } => {
                    out.push(ASTNode::ScopeBox {
                        body: self.prune_build_gate_items(body)?,
                        span,
                    });
                }
                ASTNode::TaskScope {
                    body,
                    source_keyword,
                    span,
                } => {
                    out.push(ASTNode::TaskScope {
                        body: self.prune_build_gate_items(body)?,
                        source_keyword,
                        span,
                    });
                }
                ASTNode::ContextScope {
                    name,
                    declared_type_name,
                    value,
                    body,
                    source_keyword,
                    span,
                } => {
                    out.push(ASTNode::ContextScope {
                        name,
                        declared_type_name,
                        value,
                        body: self.prune_build_gate_items(body)?,
                        source_keyword,
                        span,
                    });
                }
                ASTNode::If {
                    condition,
                    then_body,
                    else_body,
                    span,
                } => {
                    out.push(ASTNode::If {
                        condition,
                        then_body: self.prune_build_gate_items(then_body)?,
                        else_body: else_body
                            .map(|body| self.prune_build_gate_items(body))
                            .transpose()?,
                        span,
                    });
                }
                ASTNode::Loop {
                    condition,
                    body,
                    span,
                } => {
                    out.push(ASTNode::Loop {
                        condition,
                        body: self.prune_build_gate_items(body)?,
                        span,
                    });
                }
                ASTNode::LoopRange {
                    var_name,
                    start,
                    end,
                    body,
                    span,
                } => {
                    out.push(ASTNode::LoopRange {
                        var_name,
                        start,
                        end,
                        body: self.prune_build_gate_items(body)?,
                        span,
                    });
                }
                ASTNode::Return { value, span } => {
                    out.push(ASTNode::Return { value, span });
                }
                ASTNode::BoxDeclaration {
                    name,
                    fields,
                    field_decls,
                    public_fields,
                    private_fields,
                    methods,
                    constructors,
                    init_fields,
                    weak_fields,
                    delegates,
                    invariants,
                    transitions,
                    is_interface,
                    is_record,
                    extends,
                    implements,
                    type_parameters,
                    is_sync,
                    is_static,
                    static_init,
                    attrs,
                    span,
                } => {
                    let methods = methods
                        .into_iter()
                        .map(|(key, method)| Ok((key, self.prune_build_gate_node(method)?)))
                        .collect::<Result<HashMap<_, _>, ParseError>>()?;
                    let constructors = constructors
                        .into_iter()
                        .map(|(key, ctor)| Ok((key, self.prune_build_gate_node(ctor)?)))
                        .collect::<Result<HashMap<_, _>, ParseError>>()?;
                    let static_init = static_init
                        .map(|body| self.prune_build_gate_items(body))
                        .transpose()?;
                    out.push(ASTNode::BoxDeclaration {
                        name,
                        fields,
                        field_decls,
                        public_fields,
                        private_fields,
                        methods,
                        constructors,
                        init_fields,
                        weak_fields,
                        delegates,
                        invariants,
                        transitions,
                        is_interface,
                        is_record,
                        extends,
                        implements,
                        type_parameters,
                        is_sync,
                        is_static,
                        static_init,
                        attrs,
                        span,
                    });
                }
                ASTNode::FunctionDeclaration {
                    name,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    contracts,
                    is_static,
                    is_override,
                    attrs,
                    span,
                } => {
                    out.push(ASTNode::FunctionDeclaration {
                        name,
                        params,
                        param_decls,
                        return_type_name,
                        body: self.prune_build_gate_items(body)?,
                        uses,
                        contracts,
                        is_static,
                        is_override,
                        attrs,
                        span,
                    });
                }
                ASTNode::Lambda { params, body, span } => {
                    out.push(ASTNode::Lambda {
                        params,
                        body: self.prune_build_gate_items(body)?,
                        span,
                    });
                }
                ASTNode::TryCatch {
                    try_body,
                    catch_clauses,
                    finally_body,
                    span,
                } => {
                    let catch_clauses = catch_clauses
                        .into_iter()
                        .map(|clause| {
                            Ok(crate::ast::CatchClause {
                                exception_type: clause.exception_type,
                                variable_name: clause.variable_name,
                                body: self.prune_build_gate_items(clause.body)?,
                                span: clause.span,
                            })
                        })
                        .collect::<Result<Vec<_>, ParseError>>()?;
                    let finally_body = finally_body
                        .map(|body| self.prune_build_gate_items(body))
                        .transpose()?;
                    out.push(ASTNode::TryCatch {
                        try_body: self.prune_build_gate_items(try_body)?,
                        catch_clauses,
                        finally_body,
                        span,
                    });
                }
                ASTNode::BlockExpr {
                    prelude_stmts,
                    tail_expr,
                    span,
                } => {
                    out.push(ASTNode::BlockExpr {
                        prelude_stmts: self.prune_build_gate_items(prelude_stmts)?,
                        tail_expr,
                        span,
                    });
                }
                ASTNode::GlobalVar { name, value, span } => {
                    out.push(ASTNode::GlobalVar { name, value, span });
                }
                other => out.push(other),
            }
        }
        Ok(out)
    }

    fn prune_build_gate_node(&self, node: ASTNode) -> Result<ASTNode, ParseError> {
        match node {
            ASTNode::Program { statements, span } => Ok(ASTNode::Program {
                statements: self.prune_build_gate_items(statements)?,
                span,
            }),
            ASTNode::ScopeBox { body, span } => Ok(ASTNode::ScopeBox {
                body: self.prune_build_gate_items(body)?,
                span,
            }),
            ASTNode::TaskScope {
                body,
                source_keyword,
                span,
            } => Ok(ASTNode::TaskScope {
                body: self.prune_build_gate_items(body)?,
                source_keyword,
                span,
            }),
            ASTNode::ContextScope {
                name,
                declared_type_name,
                value,
                body,
                source_keyword,
                span,
            } => Ok(ASTNode::ContextScope {
                name,
                declared_type_name,
                value,
                body: self.prune_build_gate_items(body)?,
                source_keyword,
                span,
            }),
            ASTNode::If {
                condition,
                then_body,
                else_body,
                span,
            } => Ok(ASTNode::If {
                condition,
                then_body: self.prune_build_gate_items(then_body)?,
                else_body: else_body
                    .map(|body| self.prune_build_gate_items(body))
                    .transpose()?,
                span,
            }),
            ASTNode::Loop {
                condition,
                body,
                span,
            } => Ok(ASTNode::Loop {
                condition,
                body: self.prune_build_gate_items(body)?,
                span,
            }),
            ASTNode::LoopRange {
                var_name,
                start,
                end,
                body,
                span,
            } => Ok(ASTNode::LoopRange {
                var_name,
                start,
                end,
                body: self.prune_build_gate_items(body)?,
                span,
            }),
            ASTNode::Return { value, span } => Ok(ASTNode::Return { value, span }),
            ASTNode::BoxDeclaration {
                name,
                fields,
                field_decls,
                public_fields,
                private_fields,
                methods,
                constructors,
                init_fields,
                weak_fields,
                delegates,
                invariants,
                transitions,
                is_interface,
                is_record,
                extends,
                implements,
                type_parameters,
                is_sync,
                is_static,
                static_init,
                attrs,
                span,
            } => {
                let methods = methods
                    .into_iter()
                    .map(|(key, method)| Ok((key, self.prune_build_gate_node(method)?)))
                    .collect::<Result<HashMap<_, _>, ParseError>>()?;
                let constructors = constructors
                    .into_iter()
                    .map(|(key, ctor)| Ok((key, self.prune_build_gate_node(ctor)?)))
                    .collect::<Result<HashMap<_, _>, ParseError>>()?;
                let static_init = static_init
                    .map(|body| self.prune_build_gate_items(body))
                    .transpose()?;
                Ok(ASTNode::BoxDeclaration {
                    name,
                    fields,
                    field_decls,
                    public_fields,
                    private_fields,
                    methods,
                    constructors,
                    init_fields,
                    weak_fields,
                    delegates,
                    invariants,
                    transitions,
                    is_interface,
                    is_record,
                    extends,
                    implements,
                    type_parameters,
                    is_sync,
                    is_static,
                    static_init,
                    attrs,
                    span,
                })
            }
            ASTNode::FunctionDeclaration {
                name,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                contracts,
                is_static,
                is_override,
                attrs,
                span,
            } => Ok(ASTNode::FunctionDeclaration {
                name,
                params,
                param_decls,
                return_type_name,
                body: self.prune_build_gate_items(body)?,
                uses,
                contracts,
                is_static,
                is_override,
                attrs,
                span,
            }),
            ASTNode::Lambda { params, body, span } => Ok(ASTNode::Lambda {
                params,
                body: self.prune_build_gate_items(body)?,
                span,
            }),
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                span,
            } => {
                let catch_clauses = catch_clauses
                    .into_iter()
                    .map(|clause| {
                        Ok(crate::ast::CatchClause {
                            exception_type: clause.exception_type,
                            variable_name: clause.variable_name,
                            body: self.prune_build_gate_items(clause.body)?,
                            span: clause.span,
                        })
                    })
                    .collect::<Result<Vec<_>, ParseError>>()?;
                let finally_body = finally_body
                    .map(|body| self.prune_build_gate_items(body))
                    .transpose()?;
                Ok(ASTNode::TryCatch {
                    try_body: self.prune_build_gate_items(try_body)?,
                    catch_clauses,
                    finally_body,
                    span,
                })
            }
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                span,
            } => Ok(ASTNode::BlockExpr {
                prelude_stmts: self.prune_build_gate_items(prelude_stmts)?,
                tail_expr,
                span,
            }),
            ASTNode::GlobalVar { name, value, span } => {
                Ok(ASTNode::GlobalVar { name, value, span })
            }
            other => Ok(other),
        }
    }

    pub(super) fn is_build_gate_head(&self) -> bool {
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

    pub(super) fn parse_build_gate_item(&mut self) -> Result<ASTNode, ParseError> {
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

    pub(super) fn consume_build_gate_head(&mut self) -> Result<(), ParseError> {
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

    pub(super) fn parse_build_predicate(&mut self) -> Result<BuildPredicate, ParseError> {
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

    pub(super) fn parse_build_predicate_list(
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

    pub(super) fn explain_build_gate_program(
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

    pub(super) fn eval_build_predicate(
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
