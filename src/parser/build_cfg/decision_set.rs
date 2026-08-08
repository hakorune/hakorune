//! Parser-private BuildGate decision inventory for the I0-C postpass slice.
//!
//! The parser issues one observation when it constructs each AST BuildGate.
//! This module co-seals those observations with the final AST shape and
//! evaluates every predicate exactly once.  Consumers are intentionally not
//! wired yet; the later projection row will consume this product for prune,
//! source-path rebasing, and explain.

use crate::ast::{ASTNode, BuildPredicate, Span};
use crate::parser::build_gate_selection::BuildGateSelectionOutcomeV1;
use crate::parser::source_authority::{ParserInvocationBrandV1, SourceBuildGateIdV1};
use crate::parser::source_path::SourceBuildGatePathV1;
use crate::parser::{NyashParser, ParseError};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BuildGateObservationV1 {
    pub(super) brand: ParserInvocationBrandV1,
    pub(super) coordinate: u32,
    pub(super) gate_id: Option<SourceBuildGateIdV1>,
    pub(super) source_path: Option<SourceBuildGatePathV1>,
    pub(super) predicate: BuildPredicate,
    pub(super) span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BuildGateReachabilityV1 {
    Reachable,
    InactiveSubtree,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BuildGateDecisionRowV1 {
    pub(super) brand: ParserInvocationBrandV1,
    pub(super) coordinate: u32,
    pub(super) gate_id: Option<SourceBuildGateIdV1>,
    pub(super) source_path: Option<SourceBuildGatePathV1>,
    pub(super) predicate: BuildPredicate,
    pub(super) span: Span,
    pub(super) selected_branch: BuildGateSelectionOutcomeV1,
    pub(super) reachability: BuildGateReachabilityV1,
}

/// The only S0 owner for parser-visible AST BuildGate decisions.
///
/// It is deliberately non-Clone and has no public constructor.  A future
/// projection may consume it; no resolver, Recipe, or physical layer sees it.
#[derive(Debug)]
pub(crate) struct PreparedBuildGateDecisionSetV1 {
    brand: ParserInvocationBrandV1,
    rows: Box<[BuildGateDecisionRowV1]>,
}

impl PreparedBuildGateDecisionSetV1 {
    pub(crate) fn issue(
        parser: &NyashParser,
        ast: &ASTNode,
        observations: Vec<BuildGateObservationV1>,
    ) -> Result<Self, ParseError> {
        let mut ast_gates = Vec::new();
        collect_ast_gates(ast, &mut ast_gates);
        if observations.len() != ast_gates.len() {
            return Err(build_cfg_error(
                format!(
                    "BuildGate observation coverage mismatch: observations={}, ast={}",
                    observations.len(),
                    ast_gates.len()
                ),
                ast.span().line,
            ));
        }

        let brand = parser.source_invocation_brand();
        let mut rows = Vec::with_capacity(observations.len());
        for (observation, (predicate, span, has_else)) in
            observations.into_iter().zip(ast_gates.iter())
        {
            if observation.brand != brand {
                return Err(build_cfg_error(
                    "foreign parser brand in BuildGate observation".to_owned(),
                    observation.span.line,
                ));
            }
            if observation.predicate != *predicate || observation.span.line != span.line {
                return Err(build_cfg_error(
                    format!(
                        "BuildGate observation/AST mismatch at coordinate {}",
                        observation.coordinate
                    ),
                    span.line,
                ));
            }
            validate_predicate_configuration(parser, predicate, span)?;
            let selected = parser.eval_build_predicate(predicate, *span)?;
            let selected_branch = if selected {
                BuildGateSelectionOutcomeV1::Then
            } else if *has_else {
                BuildGateSelectionOutcomeV1::Else
            } else {
                BuildGateSelectionOutcomeV1::NoElse
            };
            rows.push(BuildGateDecisionRowV1 {
                brand: observation.brand,
                coordinate: observation.coordinate,
                gate_id: observation.gate_id,
                source_path: observation.source_path,
                predicate: observation.predicate,
                span: observation.span,
                selected_branch,
                reachability: BuildGateReachabilityV1::InactiveSubtree,
            });
        }

        let mut cursor = 0usize;
        assign_reachability(ast, &mut cursor, true, &mut rows)?;
        if cursor != rows.len() {
            return Err(build_cfg_error(
                "BuildGate decision traversal did not consume every row".to_owned(),
                0,
            ));
        }
        Ok(Self {
            brand,
            rows: rows.into_boxed_slice(),
        })
    }

    pub(super) fn brand(&self) -> &ParserInvocationBrandV1 {
        &self.brand
    }

    pub(super) fn rows(&self) -> &[BuildGateDecisionRowV1] {
        &self.rows
    }
}

impl NyashParser {
    pub(super) fn issue_build_gate_observation(
        &mut self,
        predicate: BuildPredicate,
        span: Span,
        gate_id: Option<SourceBuildGateIdV1>,
        source_path: Option<SourceBuildGatePathV1>,
    ) -> Result<(), ParseError> {
        let coordinate = u32::try_from(self.build_gate_observations.len()).map_err(|_| {
            build_cfg_error(
                "BuildGate observation coordinate exceeds u32".to_owned(),
                span.line,
            )
        })?;
        self.build_gate_observations.push(BuildGateObservationV1 {
            brand: self.source_invocation_brand(),
            coordinate,
            gate_id,
            source_path,
            predicate,
            span,
        });
        Ok(())
    }

    pub(crate) fn issue_pending_build_gate_observation(
        &mut self,
        predicate: BuildPredicate,
        span: Span,
    ) -> Result<(), ParseError> {
        let source_path = self
            .active_source_declaration_path
            .as_ref()
            .and_then(SourceBuildGatePathV1::from_box_path);
        self.issue_build_gate_observation(predicate, span, None, source_path)
    }

    pub(crate) fn issue_build_gate_decision_set(
        &mut self,
        ast: &ASTNode,
    ) -> Result<PreparedBuildGateDecisionSetV1, ParseError> {
        let observations = std::mem::take(&mut self.build_gate_observations);
        PreparedBuildGateDecisionSetV1::issue(self, ast, observations)
    }
}

fn collect_ast_gates(node: &ASTNode, out: &mut Vec<(BuildPredicate, Span, bool)>) {
    match node {
        ASTNode::BuildGate {
            predicate,
            then_items,
            else_items,
            span,
        } => {
            out.push((predicate.clone(), *span, else_items.is_some()));
            for item in then_items {
                collect_ast_gates(item, out);
            }
            if let Some(items) = else_items {
                for item in items {
                    collect_ast_gates(item, out);
                }
            }
        }
        _ => node.for_each_child(&mut |child| collect_ast_gates(child, out)),
    }
}

fn assign_reachability(
    node: &ASTNode,
    cursor: &mut usize,
    reachable: bool,
    output: &mut [BuildGateDecisionRowV1],
) -> Result<(), ParseError> {
    match node {
        ASTNode::BuildGate {
            then_items,
            else_items,
            ..
        } => {
            let index = *cursor;
            let selected_branch = output
                .get(index)
                .ok_or_else(|| {
                    build_cfg_error("BuildGate reachability cursor exceeded rows".to_owned(), 0)
                })?
                .selected_branch;
            output[index].reachability = if reachable {
                BuildGateReachabilityV1::Reachable
            } else {
                BuildGateReachabilityV1::InactiveSubtree
            };
            *cursor += 1;
            let then_reachable =
                reachable && matches!(selected_branch, BuildGateSelectionOutcomeV1::Then);
            for item in then_items {
                assign_reachability(item, cursor, then_reachable, output)?;
            }
            if let Some(items) = else_items {
                let else_reachable =
                    reachable && matches!(selected_branch, BuildGateSelectionOutcomeV1::Else);
                for item in items {
                    assign_reachability(item, cursor, else_reachable, output)?;
                }
            }
        }
        _ => {
            let mut child_error = None;
            node.for_each_child(&mut |child| {
                if child_error.is_none() {
                    child_error = assign_reachability(child, cursor, reachable, output).err();
                }
            });
            if let Some(error) = child_error {
                return Err(error);
            }
        }
    }
    Ok(())
}

fn validate_predicate_configuration(
    parser: &NyashParser,
    predicate: &BuildPredicate,
    span: &Span,
) -> Result<(), ParseError> {
    match predicate {
        BuildPredicate::Feature(name) => {
            if !parser.build_config.known_features.contains(name) {
                return Err(build_cfg_error(
                    format!("unknown feature '{}'", name),
                    span.line,
                ));
            }
        }
        BuildPredicate::TargetEq { key, .. } if !matches!(key.as_str(), "os" | "arch") => {
            return Err(build_cfg_error(
                format!("unsupported Target key '{}'", key),
                span.line,
            ));
        }
        BuildPredicate::BackendEq { key, .. } if key != "kind" => {
            return Err(build_cfg_error(
                format!("unsupported Backend key '{}'", key),
                span.line,
            ));
        }
        BuildPredicate::Not(inner) => validate_predicate_configuration(parser, inner, span)?,
        BuildPredicate::All(items) | BuildPredicate::Any(items) => {
            for item in items {
                validate_predicate_configuration(parser, item, span)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn build_cfg_error(message: String, line: usize) -> ParseError {
    ParseError::BuildCfg { message, line }
}

#[cfg(test)]
#[path = "decision_set_tests.rs"]
mod tests;
