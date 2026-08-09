use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationV1,
};
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1, SourcePathV1};
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1;

use super::call_row::CallableResultCallRowsV1;
use super::expression_proof::{
    ExpressionEnvironmentV1, ExpressionProofContextV1, I64ExpressionFactV1,
};
use super::requirements::{union_requirements, RequirementSetV1};
use super::{
    CallableResultCatalogErrorV1, CallableResultUnavailableReasonV1,
    VerifiedCallableResultDispositionV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FunctionProofOutcomeV1 {
    Exact(RequirementSetV1),
    ExactNominalBox(String),
    Unavailable(CallableResultUnavailableReasonV1),
    PendingDependency,
}

pub(super) struct FunctionProofProductV1<'targets> {
    pub(super) outcome: FunctionProofOutcomeV1,
    pub(super) call_rows: CallableResultCallRowsV1<'targets>,
}

#[derive(Debug)]
struct FlowV1 {
    fallthrough: Option<ExpressionEnvironmentV1>,
    breaks: Vec<ExpressionEnvironmentV1>,
    continues: Vec<ExpressionEnvironmentV1>,
}

impl FlowV1 {
    fn fallthrough(environment: ExpressionEnvironmentV1) -> Self {
        Self {
            fallthrough: Some(environment),
            breaks: Vec::new(),
            continues: Vec::new(),
        }
    }
}

pub(super) fn prove_function<'targets, 'catalog>(
    declaration: &'catalog VerifiedSameModuleCallableDeclarationV1,
    targets: &'targets VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    result_rows: &BTreeMap<CanonicalSameModuleCallableKeyV1, VerifiedCallableResultDispositionV1>,
) -> Result<FunctionProofProductV1<'targets>, CallableResultCatalogErrorV1> {
    let key = declaration.key();
    if let Some(result) = declaration.return_type_name() {
        return Ok(FunctionProofProductV1 {
            outcome: if ExactTrivialScalarAbiV1::classify(result).is_some() {
                FunctionProofOutcomeV1::Exact(BTreeSet::new())
            } else {
                FunctionProofOutcomeV1::Unavailable(
                    CallableResultUnavailableReasonV1::DeclaredNonI64Result,
                )
            },
            call_rows: BTreeMap::new(),
        });
    }
    if declaration.body().iter().any(contains_grouped_assignment) {
        return Ok(FunctionProofProductV1 {
            outcome: FunctionProofOutcomeV1::Unavailable(
                CallableResultUnavailableReasonV1::UnsupportedExpressionKind,
            ),
            call_rows: BTreeMap::new(),
        });
    }

    let mut context =
        ExpressionProofContextV1::new(key, declaration.params(), targets, result_rows)?;
    let mut returns = Vec::new();
    let mut fatal = None;
    let paths = root_body_paths(declaration.body().len());
    let flow = analyze_statements(
        &mut context,
        declaration.body(),
        &paths,
        0,
        &mut returns,
        &mut fatal,
    )?;
    let outcome = if let Some(reason) = fatal {
        FunctionProofOutcomeV1::Unavailable(reason)
    } else if flow.fallthrough.is_some() {
        FunctionProofOutcomeV1::Unavailable(CallableResultUnavailableReasonV1::MissingReturn)
    } else {
        summarize_returns(returns)
    };
    Ok(FunctionProofProductV1 {
        outcome,
        call_rows: context.into_call_rows(),
    })
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
    context: &mut ExpressionProofContextV1<'_, '_, '_>,
    statements: &[ASTNode],
    paths: &[SourcePathV1],
    loop_depth: usize,
    returns: &mut Vec<I64ExpressionFactV1>,
    fatal: &mut Option<CallableResultUnavailableReasonV1>,
) -> Result<FlowV1, CallableResultCatalogErrorV1> {
    debug_assert_eq!(statements.len(), paths.len());
    let mut flow = FlowV1::fallthrough(context.environment().clone());
    for (statement, path) in statements.iter().zip(paths) {
        let Some(environment) = flow.fallthrough.take() else {
            break;
        };
        context.replace_environment(environment);
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
                for (index, (name, initial)) in variables.iter().zip(initial_values).enumerate() {
                    if context.contains_binding(name) {
                        *fatal = Some(CallableResultUnavailableReasonV1::DuplicateLocal);
                        break;
                    }
                    let receiver_fact = initial
                        .as_ref()
                        .and_then(|initial| context.core_receiver_fact(initial));
                    let fact = match initial {
                        Some(initial) => context.prove_expression(
                            initial,
                            &expr_child_path(
                                statement,
                                path,
                                ExprChildRoleV1::LocalInitializer(index as u32),
                            ),
                        )?,
                        None => I64ExpressionFactV1::Unknown(
                            CallableResultUnavailableReasonV1::UnknownExpression,
                        ),
                    };
                    context.publish_binding(name, fact);
                    context.publish_core_receiver_binding(name, receiver_fact);
                }
                flow.fallthrough = Some(context.environment().clone());
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
                let receiver_fact = context.core_receiver_fact(value);
                let fact = context.prove_expression(
                    value,
                    &expr_child_path(statement, path, ExprChildRoleV1::AssignmentValue),
                )?;
                context.publish_binding(name, fact);
                context.publish_core_receiver_binding(name, receiver_fact);
                flow.fallthrough = Some(context.environment().clone());
            }
            ASTNode::Return {
                value: Some(value), ..
            } => returns.push(context.prove_expression(
                value,
                &expr_child_path(statement, path, ExprChildRoleV1::ReturnValue),
            )?),
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
                let before_condition = context.environment().clone();
                let _ = context.prove_expression(
                    condition,
                    &expr_child_path(statement, path, ExprChildRoleV1::IfCondition),
                )?;
                if context.environment() != &before_condition {
                    *fatal = Some(CallableResultUnavailableReasonV1::UnsupportedExpressionKind);
                    break;
                }
                let base = context.environment().clone();
                context.replace_environment(base.clone());
                let then_paths =
                    body_child_paths(statement, path, BodyChildRoleV1::IfThen, then_body.len());
                let then_flow = analyze_statements(
                    context,
                    then_body,
                    &then_paths,
                    loop_depth,
                    returns,
                    fatal,
                )?;
                context.replace_environment(base.clone());
                let else_flow = if let Some(else_body) = else_body {
                    let else_paths =
                        body_child_paths(statement, path, BodyChildRoleV1::IfElse, else_body.len());
                    analyze_statements(context, else_body, &else_paths, loop_depth, returns, fatal)?
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
                let before_condition = context.environment().clone();
                let _ = context.prove_expression(
                    condition,
                    &expr_child_path(statement, path, ExprChildRoleV1::LoopCondition),
                )?;
                if context.environment() != &before_condition {
                    *fatal = Some(CallableResultUnavailableReasonV1::UnsupportedExpressionKind);
                    break;
                }
                let body_paths =
                    body_child_paths(statement, path, BodyChildRoleV1::LoopBody, body.len());
                let loop_flow = analyze_loop(context, body, &body_paths, returns, fatal)?;
                flow.fallthrough = loop_flow.fallthrough;
            }
            ASTNode::Break { .. } if loop_depth != 0 => {
                flow.breaks.push(context.environment().clone());
            }
            ASTNode::Continue { .. } if loop_depth != 0 => {
                flow.continues.push(context.environment().clone());
            }
            ASTNode::FunctionCall { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::GroupedAssignmentExpr { .. } => {
                let _ = context.prove_expression(statement, path)?;
                flow.fallthrough = Some(context.environment().clone());
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
    context: &mut ExpressionProofContextV1<'_, '_, '_>,
    body: &[ASTNode],
    body_paths: &[SourcePathV1],
    returns: &mut Vec<I64ExpressionFactV1>,
    fatal: &mut Option<CallableResultUnavailableReasonV1>,
) -> Result<FlowV1, CallableResultCatalogErrorV1> {
    let entry = context.environment().clone();
    let mut invariant = entry.clone();
    let budget = 32usize
        .saturating_add(entry.binding_count())
        .saturating_add(body.len() * 4);
    let mut final_breaks = Vec::new();

    for _ in 0..budget {
        let call_rows_before_iteration = context.call_row_state();
        let returns_before_iteration = returns.len();
        context.replace_environment(invariant.clone());
        let body_flow = analyze_statements(context, body, body_paths, 1, returns, fatal)?;
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
            next = ExpressionEnvironmentV1::merge(&next, &backedge);
        }
        final_breaks = body_flow.breaks;
        if next == invariant {
            let mut exits = vec![entry.clone(), invariant.clone()];
            exits.extend(final_breaks);
            let exit = exits
                .into_iter()
                .reduce(|left, right| ExpressionEnvironmentV1::merge(&left, &right))
                .unwrap_or(entry);
            return Ok(FlowV1::fallthrough(exit));
        }
        context.restore_call_row_state(call_rows_before_iteration);
        returns.truncate(returns_before_iteration);
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
    left: Option<ExpressionEnvironmentV1>,
    right: Option<ExpressionEnvironmentV1>,
) -> Option<ExpressionEnvironmentV1> {
    match (left, right) {
        (Some(left), Some(right)) => Some(ExpressionEnvironmentV1::merge(&left, &right)),
        (Some(environment), None) | (None, Some(environment)) => Some(environment),
        (None, None) => None,
    }
}

fn summarize_returns(returns: Vec<I64ExpressionFactV1>) -> FunctionProofOutcomeV1 {
    if returns.is_empty() {
        return FunctionProofOutcomeV1::Unavailable(
            CallableResultUnavailableReasonV1::MissingReturn,
        );
    }
    if returns
        .iter()
        .any(|fact| matches!(fact, I64ExpressionFactV1::PendingDependency))
    {
        return FunctionProofOutcomeV1::PendingDependency;
    }

    let mut requirements = BTreeSet::new();
    let mut saw_exact = false;
    let mut saw_non_i64 = false;
    let mut exact_box: Option<String> = None;
    for fact in returns {
        match fact {
            I64ExpressionFactV1::Exact(current) => {
                if exact_box.is_some() {
                    return FunctionProofOutcomeV1::Unavailable(
                        CallableResultUnavailableReasonV1::ConflictingReturnRepresentations,
                    );
                }
                saw_exact = true;
                requirements = union_requirements(&requirements, &current);
            }
            I64ExpressionFactV1::ExactNominalBox(box_name) => {
                if saw_exact
                    || exact_box
                        .as_ref()
                        .is_some_and(|existing| existing != &box_name)
                {
                    return FunctionProofOutcomeV1::Unavailable(
                        CallableResultUnavailableReasonV1::ConflictingReturnRepresentations,
                    );
                }
                exact_box = Some(box_name);
            }
            I64ExpressionFactV1::KnownNonI64 => saw_non_i64 = true,
            I64ExpressionFactV1::Unknown(reason) => {
                return FunctionProofOutcomeV1::Unavailable(reason);
            }
            I64ExpressionFactV1::Conflict => {
                return FunctionProofOutcomeV1::Unavailable(
                    CallableResultUnavailableReasonV1::ConflictingReturnRepresentations,
                );
            }
            I64ExpressionFactV1::PendingDependency => unreachable!("handled before reduction"),
        }
    }
    if saw_non_i64 {
        return FunctionProofOutcomeV1::Unavailable(if saw_exact || exact_box.is_some() {
            CallableResultUnavailableReasonV1::ConflictingReturnRepresentations
        } else {
            CallableResultUnavailableReasonV1::KnownNonI64Return
        });
    }
    if let Some(box_name) = exact_box {
        return FunctionProofOutcomeV1::ExactNominalBox(box_name);
    }
    FunctionProofOutcomeV1::Exact(requirements)
}

fn root_body_paths(len: usize) -> Vec<SourcePathV1> {
    (0..len).map(SourcePathV1::root_body).collect()
}

fn expr_child_path(parent: &ASTNode, path: &SourcePathV1, role: ExprChildRoleV1) -> SourcePathV1 {
    path.child(
        role.segment_for(parent)
            .expect("[freeze:contract][source_path/callable_result_stmt_expr_role]"),
    )
}

fn body_child_paths(
    statement: &ASTNode,
    parent: &SourcePathV1,
    role: BodyChildRoleV1,
    len: usize,
) -> Vec<SourcePathV1> {
    let kind = role
        .kind_for(statement)
        .expect("[freeze:contract][source_path/callable_result_stmt_body_role]");
    (0..len)
        .map(|index| parent.child(kind.item_segment(index as u32)))
        .collect()
}
