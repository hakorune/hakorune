use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::builder::VerifiedSameModuleCallableDeclarationV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;

use super::expression_proof::{ExpressionProofContextV1, I64ExpressionFactV1};
use super::requirements::{union_requirements, RequirementSetV1};
use super::{CallableResultCatalogErrorV1, CallableResultUnavailableReasonV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FunctionProofOutcomeV1 {
    Exact(RequirementSetV1),
    Unavailable(CallableResultUnavailableReasonV1),
}

type EnvironmentV1 = BTreeMap<String, I64ExpressionFactV1>;

#[derive(Debug)]
struct FlowV1 {
    fallthrough: Option<EnvironmentV1>,
    breaks: Vec<EnvironmentV1>,
    continues: Vec<EnvironmentV1>,
}

impl FlowV1 {
    fn fallthrough(environment: EnvironmentV1) -> Self {
        Self {
            fallthrough: Some(environment),
            breaks: Vec::new(),
            continues: Vec::new(),
        }
    }
}

pub(super) fn prove_function(
    declaration: &VerifiedSameModuleCallableDeclarationV1,
) -> Result<FunctionProofOutcomeV1, CallableResultCatalogErrorV1> {
    let key = declaration.key();
    if let Some(result) = declaration.return_type_name() {
        return Ok(if ExactTrivialScalarAbiV1::classify(result).is_some() {
            FunctionProofOutcomeV1::Exact(BTreeSet::new())
        } else {
            FunctionProofOutcomeV1::Unavailable(
                CallableResultUnavailableReasonV1::DeclaredNonI64Result,
            )
        });
    }
    if declaration.body().iter().any(contains_grouped_assignment) {
        return Ok(FunctionProofOutcomeV1::Unavailable(
            CallableResultUnavailableReasonV1::UnsupportedExpressionKind,
        ));
    }

    let mut context = ExpressionProofContextV1::new(key, declaration.params())?;
    let mut returns = Vec::new();
    let mut fatal = None;
    let flow = analyze_statements(
        &mut context,
        declaration.body(),
        0,
        &mut returns,
        &mut fatal,
    )?;
    if let Some(reason) = fatal {
        return Ok(FunctionProofOutcomeV1::Unavailable(reason));
    }
    if flow.fallthrough.is_some() {
        return Ok(FunctionProofOutcomeV1::Unavailable(
            CallableResultUnavailableReasonV1::MissingReturn,
        ));
    }
    summarize_returns(returns)
}

fn contains_grouped_assignment(root: &ASTNode) -> bool {
    let mut worklist = vec![root];
    while let Some(node) = worklist.pop() {
        if matches!(node, ASTNode::GroupedAssignmentExpr { .. }) {
            return true;
        }
        node.for_each_child(&mut |child| worklist.push(child));
    }
    false
}

fn analyze_statements(
    context: &mut ExpressionProofContextV1,
    statements: &[ASTNode],
    loop_depth: usize,
    returns: &mut Vec<I64ExpressionFactV1>,
    fatal: &mut Option<CallableResultUnavailableReasonV1>,
) -> Result<FlowV1, CallableResultCatalogErrorV1> {
    let mut flow = FlowV1::fallthrough(context.bindings().clone());
    for statement in statements {
        let Some(environment) = flow.fallthrough.take() else {
            break;
        };
        context.replace_bindings(environment);
        match statement {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                if variables.len() != initial_values.len() {
                    *fatal = Some(CallableResultUnavailableReasonV1::UnsupportedStatementKind);
                    break;
                }
                for (name, initial) in variables.iter().zip(initial_values) {
                    if context.contains_binding(name) {
                        *fatal = Some(CallableResultUnavailableReasonV1::DuplicateLocal);
                        break;
                    }
                    let fact = match initial {
                        Some(initial) => context.prove_expression(initial)?,
                        None => I64ExpressionFactV1::Unknown(
                            CallableResultUnavailableReasonV1::UnknownExpression,
                        ),
                    };
                    context.publish_binding(name.clone(), fact);
                }
                flow.fallthrough = Some(context.bindings().clone());
            }
            ASTNode::Assignment { target, value, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    *fatal = Some(CallableResultUnavailableReasonV1::UnsupportedAssignmentTarget);
                    break;
                };
                if !context.contains_binding(name) {
                    *fatal = Some(CallableResultUnavailableReasonV1::UnboundLocal);
                    break;
                }
                let fact = context.prove_expression(value)?;
                context.publish_binding(name.clone(), fact);
                flow.fallthrough = Some(context.bindings().clone());
            }
            ASTNode::Return {
                value: Some(value), ..
            } => {
                returns.push(context.prove_expression(value)?);
            }
            ASTNode::Return { value: None, .. } => {
                returns.push(I64ExpressionFactV1::Unknown(
                    CallableResultUnavailableReasonV1::NoValueReturn,
                ));
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let before_condition = context.bindings().clone();
                let _ = context.prove_expression(condition)?;
                if context.bindings() != &before_condition {
                    *fatal = Some(CallableResultUnavailableReasonV1::UnsupportedExpressionKind);
                    break;
                }
                let base = context.bindings().clone();
                context.replace_bindings(base.clone());
                let then_flow = analyze_statements(context, then_body, loop_depth, returns, fatal)?;
                context.replace_bindings(base.clone());
                let else_flow = if let Some(else_body) = else_body {
                    analyze_statements(context, else_body, loop_depth, returns, fatal)?
                } else {
                    FlowV1::fallthrough(base)
                };
                flow.fallthrough =
                    merge_optional_environments(then_flow.fallthrough, else_flow.fallthrough);
                flow.breaks.extend(then_flow.breaks);
                flow.breaks.extend(else_flow.breaks);
                flow.continues.extend(then_flow.continues);
                flow.continues.extend(else_flow.continues);
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                if loop_depth != 0 {
                    *fatal = Some(CallableResultUnavailableReasonV1::NestedLoopUnsupported);
                    break;
                }
                let before_condition = context.bindings().clone();
                let _ = context.prove_expression(condition)?;
                if context.bindings() != &before_condition {
                    *fatal = Some(CallableResultUnavailableReasonV1::UnsupportedExpressionKind);
                    break;
                }
                let loop_flow = analyze_loop(context, body, returns, fatal)?;
                flow.fallthrough = loop_flow.fallthrough;
            }
            ASTNode::Break { .. } if loop_depth != 0 => {
                flow.breaks.push(context.bindings().clone());
            }
            ASTNode::Continue { .. } if loop_depth != 0 => {
                flow.continues.push(context.bindings().clone());
            }
            ASTNode::FunctionCall { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::GroupedAssignmentExpr { .. } => {
                let _ = context.prove_expression(statement)?;
                flow.fallthrough = Some(context.bindings().clone());
            }
            _ => {
                *fatal = Some(CallableResultUnavailableReasonV1::UnsupportedStatementKind);
                break;
            }
        }
        if fatal.is_some() {
            break;
        }
    }
    Ok(flow)
}

fn analyze_loop(
    context: &mut ExpressionProofContextV1,
    body: &[ASTNode],
    returns: &mut Vec<I64ExpressionFactV1>,
    fatal: &mut Option<CallableResultUnavailableReasonV1>,
) -> Result<FlowV1, CallableResultCatalogErrorV1> {
    let entry = context.bindings().clone();
    let mut invariant = entry.clone();
    let budget = 32usize
        .saturating_add(entry.len())
        .saturating_add(body.len() * 4);
    let mut final_breaks = Vec::new();

    for _ in 0..budget {
        context.replace_bindings(invariant.clone());
        let body_flow = analyze_statements(context, body, 1, returns, fatal)?;
        if fatal.is_some() {
            return Ok(FlowV1 {
                fallthrough: None,
                breaks: Vec::new(),
                continues: Vec::new(),
            });
        }
        let mut backedges = body_flow.continues.clone();
        if let Some(fallthrough) = body_flow.fallthrough {
            backedges.push(fallthrough);
        }
        let mut next = entry.clone();
        for backedge in backedges {
            next = merge_environments(&next, &backedge);
        }
        final_breaks = body_flow.breaks;
        if next == invariant {
            let mut exits = vec![entry.clone(), invariant.clone()];
            exits.extend(final_breaks);
            let exit = exits
                .into_iter()
                .reduce(|left, right| merge_environments(&left, &right))
                .unwrap_or(entry);
            return Ok(FlowV1::fallthrough(exit));
        }
        invariant = next;
    }

    *fatal = Some(CallableResultUnavailableReasonV1::LoopInvariantUnavailable);
    Ok(FlowV1 {
        fallthrough: None,
        breaks: final_breaks,
        continues: Vec::new(),
    })
}

fn merge_optional_environments(
    left: Option<EnvironmentV1>,
    right: Option<EnvironmentV1>,
) -> Option<EnvironmentV1> {
    match (left, right) {
        (Some(left), Some(right)) => Some(merge_environments(&left, &right)),
        (Some(environment), None) | (None, Some(environment)) => Some(environment),
        (None, None) => None,
    }
}

fn merge_environments(left: &EnvironmentV1, right: &EnvironmentV1) -> EnvironmentV1 {
    left.iter()
        .filter_map(|(name, left_fact)| {
            right.get(name).map(|right_fact| {
                (
                    name.clone(),
                    I64ExpressionFactV1::merge_paths(left_fact, right_fact),
                )
            })
        })
        .collect()
}

fn summarize_returns(
    returns: Vec<I64ExpressionFactV1>,
) -> Result<FunctionProofOutcomeV1, CallableResultCatalogErrorV1> {
    if returns.is_empty() {
        return Ok(FunctionProofOutcomeV1::Unavailable(
            CallableResultUnavailableReasonV1::MissingReturn,
        ));
    }

    let mut requirements = BTreeSet::new();
    let mut saw_exact = false;
    let mut saw_non_i64 = false;
    for fact in returns {
        match fact {
            I64ExpressionFactV1::Exact(current) => {
                saw_exact = true;
                requirements = union_requirements(&requirements, &current);
            }
            I64ExpressionFactV1::KnownNonI64 => saw_non_i64 = true,
            I64ExpressionFactV1::Unknown(reason) => {
                return Ok(FunctionProofOutcomeV1::Unavailable(reason));
            }
            I64ExpressionFactV1::Conflict => {
                return Ok(FunctionProofOutcomeV1::Unavailable(
                    CallableResultUnavailableReasonV1::ConflictingReturnRepresentations,
                ));
            }
        }
    }
    if saw_non_i64 {
        return Ok(FunctionProofOutcomeV1::Unavailable(if saw_exact {
            CallableResultUnavailableReasonV1::ConflictingReturnRepresentations
        } else {
            CallableResultUnavailableReasonV1::KnownNonI64Return
        }));
    }
    Ok(FunctionProofOutcomeV1::Exact(requirements))
}
