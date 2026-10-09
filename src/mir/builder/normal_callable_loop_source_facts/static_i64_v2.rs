//! Source-bound V2 semantics for a callable I64 search Loop.
//!
//! The resolver and original Static loans own source identity. This producer
//! alone assigns Recipe keys, then uses the common V2 verifier and JoinSig.
//! It issues no MIR, Home, actual ValueId, or physical permission.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::loop_recipe_contract::{
    issue_sole_root_carrier_join_closure_v2, LoopBinaryI64OpV2, LoopBindingKeyV1, LoopBlockKeyV1,
    LoopCarrierKeyV1, LoopCompareI64OpV2, LoopConditionV2, LoopExitKeyV1, LoopExitKindV2,
    LoopItemKeyV1, LoopNodeKeyV1, LoopNodeV2, LoopOperationV2, LoopRecipeBindingV2,
    LoopRecipeBlockV2, LoopRecipeCarrierV2, LoopRecipeExitV2, LoopRecipeItemRowV2,
    LoopRecipeItemV2, LoopRecipeV2, LoopRecipeValueV2, LoopRecipeVerifierV2, LoopValueClassV2,
    LoopValueKeyV1, VerifiedLoopJoinClosureV2, VerifiedLoopRecipeV2,
};
use crate::mir::normal_callable_semantic_package::{
    LoopEntryStaticI64SourceLoanV1, LoopStaticSourceCallLoanV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, OwnedExprSiteV1, ResolvedAssignmentTargetV1, ResolvedBinaryOperatorV1,
    ResolvedLexicalRefV1, ResolvedLoopPlacementV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1 as Segment, SourceStmtSiteV1,
};

/// Checked source-to-Recipe roles for the sole future physical consumer.
#[derive(Debug)]
pub(in crate::mir) struct StaticI64LoopRolesV2 {
    pub loop_site: SourceStmtSiteV1,
    pub n_binding: BindingRefV1,
    pub bin_binding: BindingRefV1,
    pub bin_declaration: SourceBindingSiteV1,
    pub bin_initializer: SourceExprSiteV1,
    pub return_site: SourceStmtSiteV1,
    pub assignment_target: SourceExprSiteV1,
    pub n_input: LoopValueKeyV1,
    pub bin_input: LoopValueKeyV1,
    pub header_bin_read: LoopItemKeyV1,
    pub header_call: LoopItemKeyV1,
    pub body_actual_read: LoopItemKeyV1,
    pub body_call: LoopItemKeyV1,
    pub return_bin_read: LoopItemKeyV1,
    pub return_exit: LoopItemKeyV1,
    pub backedge_bin_read: LoopItemKeyV1,
    pub backedge_write: LoopItemKeyV1,
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedStaticI64LoopSemanticV2 {
    recipe: VerifiedLoopRecipeV2,
    join: VerifiedLoopJoinClosureV2,
    roles: StaticI64LoopRolesV2,
    entry: LoopEntryStaticI64SourceLoanV1,
    header: LoopStaticSourceCallLoanV1,
    body: LoopStaticSourceCallLoanV1,
}

impl VerifiedStaticI64LoopSemanticV2 {
    pub(in crate::mir) fn recipe(&self) -> &VerifiedLoopRecipeV2 {
        &self.recipe
    }
    pub(in crate::mir) fn join(&self) -> &VerifiedLoopJoinClosureV2 {
        &self.join
    }
    pub(in crate::mir) fn roles(&self) -> &StaticI64LoopRolesV2 {
        &self.roles
    }
    pub(in crate::mir) fn source_calls(
        &self,
    ) -> (
        &LoopEntryStaticI64SourceLoanV1,
        &LoopStaticSourceCallLoanV1,
        &LoopStaticSourceCallLoanV1,
    ) {
        (&self.entry, &self.header, &self.body)
    }
}

/// The selected source shape is general in binding and selector names: only
/// operators, statement order, exact resolver sites and source loan identities
/// decide admission. A changed source stops before any physical operation.
pub(in crate::mir) fn produce_static_i64_loop_semantic_v2(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_site: &SourceStmtSiteV1,
    completion: &VerifiedFunctionCompletionV1,
    entry: LoopEntryStaticI64SourceLoanV1,
    header: LoopStaticSourceCallLoanV1,
    body_call: LoopStaticSourceCallLoanV1,
) -> Result<VerifiedStaticI64LoopSemanticV2, String> {
    let reject = || "[freeze:contract][callable-loop/static-i64-v2/source]".to_owned();
    if completion.owner() != input.owner()
        || entry.loop_site() != loop_site
        || header.loop_site() != loop_site
        || body_call.loop_site() != loop_site
        || header.placement() != &ResolvedLoopPlacementV1::Condition
        || body_call.placement() != &ResolvedLoopPlacementV1::Body
    {
        return Err(reject());
    }
    input
        .function()
        .loop_region_bundle(loop_site)
        .map_err(|_| reject())?;
    let located = input.source().exact_stmt(loop_site).map_err(|_| reject())?;
    let ASTNode::Loop {
        condition, body, ..
    } = located.node()
    else {
        return Err(reject());
    };
    if body.len() != 2 {
        return Err(reject());
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left: head_left,
        right: head_right,
        ..
    } = condition.as_ref()
    else {
        return Err(reject());
    };
    let ASTNode::Variable { name: bin_name, .. } = head_left.as_ref() else {
        return Err(reject());
    };
    let ASTNode::MethodCall {
        object: head_receiver,
        method: head_method,
        arguments: head_args,
        ..
    } = head_right.as_ref()
    else {
        return Err(reject());
    };
    if !matches!(head_receiver.as_ref(), ASTNode::Me { .. }) || !head_args.is_empty() {
        return Err(reject());
    }
    let ASTNode::If {
        condition: if_condition,
        then_body,
        else_body,
        ..
    } = &body[0]
    else {
        return Err(reject());
    };
    if else_body.is_some() || then_body.len() != 1 {
        return Err(reject());
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left: if_left,
        right: if_right,
        ..
    } = if_condition.as_ref()
    else {
        return Err(reject());
    };
    let ASTNode::Variable { name: n_name, .. } = if_left.as_ref() else {
        return Err(reject());
    };
    let ASTNode::MethodCall {
        object: body_receiver,
        method: body_method,
        arguments: body_args,
        ..
    } = if_right.as_ref()
    else {
        return Err(reject());
    };
    if !matches!(body_receiver.as_ref(), ASTNode::Me { .. })
        || body_args.len() != 1
        || !matches!(&body_args[0], ASTNode::Variable { name, .. } if name == bin_name)
    {
        return Err(reject());
    }
    let ASTNode::Return {
        value: Some(ret), ..
    } = &then_body[0]
    else {
        return Err(reject());
    };
    if !matches!(ret.as_ref(), ASTNode::Variable { name, .. } if name == bin_name) {
        return Err(reject());
    }
    let ASTNode::Assignment { target, value, .. } = &body[1] else {
        return Err(reject());
    };
    if !matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == bin_name) {
        return Err(reject());
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: step_left,
        right: step_right,
        ..
    } = value.as_ref()
    else {
        return Err(reject());
    };
    if !matches!(step_left.as_ref(), ASTNode::Variable { name, .. } if name == bin_name)
        || !matches!(
            step_right.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                ..
            }
        )
    {
        return Err(reject());
    }

    let at = |path: &[Segment]| expression_site(loop_site, path);
    let head_left_site = at(&[Segment::LoopCondition, Segment::Lhs]);
    let head_call_site = at(&[Segment::LoopCondition, Segment::Rhs]);
    let if_site = statement_site(loop_site, &[Segment::LoopBody(0)]);
    let if_left_site = at(&[Segment::LoopBody(0), Segment::IfCondition, Segment::Lhs]);
    let body_call_site = at(&[Segment::LoopBody(0), Segment::IfCondition, Segment::Rhs]);
    let body_arg_site = at(&[
        Segment::LoopBody(0),
        Segment::IfCondition,
        Segment::Rhs,
        Segment::Argument(0),
    ]);
    let return_site = statement_site(loop_site, &[Segment::LoopBody(0), Segment::IfThen(0)]);
    let return_value_site = at(&[Segment::LoopBody(0), Segment::IfThen(0), Segment::Value]);
    let assignment_target = at(&[Segment::LoopBody(1), Segment::Target]);
    let step_left_site = at(&[Segment::LoopBody(1), Segment::Value, Segment::Lhs]);
    let step_right_site = at(&[Segment::LoopBody(1), Segment::Value, Segment::Rhs]);
    let source = input.function();
    source.if_region_bundle(&if_site).map_err(|_| reject())?;
    if !completion.explicit_sites().contains(&return_site) {
        return Err(reject());
    }
    let binding = match source.variable_ref(&head_left_site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => return Err(reject()),
    };
    let n_binding = match source.variable_ref(&if_left_site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => return Err(reject()),
    };
    let entry_relation = source
        .expression_source()
        .initializer(entry.declaration())
        .ok_or_else(reject)?;
    if n_binding == binding
        || !before_same_body(entry.declaration(), loop_site)
        || entry_relation.binding() != n_binding
        || entry.call_site().site() != entry_relation.initializer_site().ok_or_else(reject)?
        || !matches!(source.variable_ref(&body_arg_site), Some(ResolvedLexicalRefV1::Local(actual)) if actual == binding)
        || !matches!(source.variable_ref(&return_value_site), Some(ResolvedLexicalRefV1::Local(actual)) if actual == binding)
        || !matches!(source.variable_ref(&step_left_site), Some(ResolvedLexicalRefV1::Local(actual)) if actual == binding)
        || !matches!(source.assignment_target(&assignment_target), Some(ResolvedAssignmentTargetV1::BindingRebind(actual)) if *actual == binding)
    {
        return Err(reject());
    }
    let head_condition_site = at(&[Segment::LoopCondition]);
    let if_condition_site = at(&[Segment::LoopBody(0), Segment::IfCondition]);
    let step_value = at(&[Segment::LoopBody(1), Segment::Value]);
    let has_binary = |site, op, lhs, rhs| {
        source.expression_source().binaries().any(|row| {
            row.site() == site && row.operator() == op && row.lhs() == lhs && row.rhs() == rhs
        })
    };
    if !has_binary(
        &head_condition_site,
        ResolvedBinaryOperatorV1::LessEqual,
        &head_left_site,
        &head_call_site,
    ) || !has_binary(
        &if_condition_site,
        ResolvedBinaryOperatorV1::LessEqual,
        &if_left_site,
        &body_call_site,
    ) || !has_binary(
        &step_value,
        ResolvedBinaryOperatorV1::Add,
        &step_left_site,
        &step_right_site,
    ) {
        return Err(reject());
    }
    let bin_initializers: Vec<_> = source
        .expression_source()
        .initializers()
        .filter(|row| row.binding() == binding)
        .collect();
    let [bin_initializer] = bin_initializers.as_slice() else {
        return Err(reject());
    };
    if !before_same_body(bin_initializer.declaration_site(), loop_site) {
        return Err(reject());
    }
    let bin_initial_site = bin_initializer.initializer_site().ok_or_else(reject)?;
    if !matches!(
        input
            .source()
            .expr_at(&OwnedExprSiteV1::new(
                input.owner(),
                bin_initial_site.clone()
            ))
            .map_err(|_| reject())?
            .node(),
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            ..
        }
    ) {
        return Err(reject());
    }
    if header.call_site().site() != &head_call_site
        || body_call.call_site().site() != &body_call_site
        || !header.argument_sites().is_empty()
        || body_call.argument_sites() != [body_arg_site]
        || source
            .method_calls()
            .find(|(site, _)| *site == &head_call_site)
            .is_none_or(|(_, call)| call.selector() != head_method)
        || source
            .method_calls()
            .find(|(site, _)| *site == &body_call_site)
            .is_none_or(|(_, call)| call.selector() != body_method)
        || n_name == bin_name
    {
        return Err(reject());
    }

    let recipe = LoopRecipeVerifierV2::verify(build_recipe(bin_name)).map_err(|error| {
        format!("[freeze:contract][callable-loop/static-i64-v2/recipe] {error:?}")
    })?;
    let join = issue_sole_root_carrier_join_closure_v2(&recipe).map_err(|error| {
        format!("[freeze:contract][callable-loop/static-i64-v2/join] {error:?}")
    })?;
    Ok(VerifiedStaticI64LoopSemanticV2 {
        recipe,
        join,
        roles: StaticI64LoopRolesV2 {
            loop_site: loop_site.clone(),
            n_binding,
            bin_binding: binding,
            bin_declaration: bin_initializer.declaration_site().clone(),
            bin_initializer: bin_initial_site.clone(),
            return_site,
            assignment_target,
            n_input: LoopValueKeyV1::new(0),
            bin_input: LoopValueKeyV1::new(1),
            header_bin_read: LoopItemKeyV1::new(0),
            header_call: LoopItemKeyV1::new(1),
            body_actual_read: LoopItemKeyV1::new(3),
            body_call: LoopItemKeyV1::new(4),
            return_bin_read: LoopItemKeyV1::new(7),
            return_exit: LoopItemKeyV1::new(8),
            backedge_bin_read: LoopItemKeyV1::new(9),
            backedge_write: LoopItemKeyV1::new(12),
        },
        entry,
        header,
        body: body_call,
    })
}

fn before_same_body(declaration: &SourceBindingSiteV1, loop_site: &SourceStmtSiteV1) -> bool {
    let SourceBindingSiteV1::Local { statement, .. } = declaration else {
        return false;
    };
    let Some((Segment::Body(declaration_index), declaration_parent)) =
        statement.node().segments().split_last()
    else {
        return false;
    };
    let Some((Segment::Body(loop_index), loop_parent)) = loop_site.node().segments().split_last()
    else {
        return false;
    };
    declaration_parent == loop_parent && declaration_index < loop_index
}

fn expression_site(loop_site: &SourceStmtSiteV1, relative: &[Segment]) -> SourceExprSiteV1 {
    let mut path = loop_site.node().segments().to_vec();
    path.extend_from_slice(relative);
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(path))
}

fn statement_site(loop_site: &SourceStmtSiteV1, relative: &[Segment]) -> SourceStmtSiteV1 {
    let mut path = loop_site.node().segments().to_vec();
    path.extend_from_slice(relative);
    SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(path))
}

fn build_recipe(label: &str) -> LoopRecipeV2 {
    let b = LoopBindingKeyV1::new(0);
    let v = LoopValueKeyV1::new;
    let i = LoopItemKeyV1::new;
    let op = |raw, operation| LoopRecipeItemRowV2 {
        key: i(raw),
        item: LoopRecipeItemV2::Operation { operation },
    };
    let block = |raw, items: &[u32]| LoopRecipeBlockV2 {
        key: LoopBlockKeyV1::new(raw),
        owner_loop: LoopNodeKeyV1::new(0),
        items: items.iter().copied().map(i).collect(),
    };
    LoopRecipeV2 {
        root_loop: LoopNodeKeyV1::new(0),
        loops: vec![LoopNodeV2 {
            key: LoopNodeKeyV1::new(0),
            parent: None,
            condition: LoopConditionV2::Predicate {
                block: LoopBlockKeyV1::new(0),
                value: v(4),
            },
            body: LoopBlockKeyV1::new(1),
        }],
        blocks: vec![
            block(0, &[0, 1, 2]),
            block(1, &[3, 4, 5, 6, 9, 10, 11, 12]),
            block(2, &[7, 8]),
        ],
        items: vec![
            op(
                0,
                LoopOperationV2::ReadBinding {
                    binding: b,
                    result: v(2),
                },
            ),
            op(
                1,
                LoopOperationV2::CallSlot {
                    receiver: None,
                    args: vec![],
                    result: Some(v(3)),
                },
            ),
            op(
                2,
                LoopOperationV2::CompareI64 {
                    op: LoopCompareI64OpV2::LessEqual,
                    left: v(2),
                    right: v(3),
                    result: v(4),
                },
            ),
            op(
                3,
                LoopOperationV2::ReadBinding {
                    binding: b,
                    result: v(5),
                },
            ),
            op(
                4,
                LoopOperationV2::CallSlot {
                    receiver: None,
                    args: vec![v(5)],
                    result: Some(v(6)),
                },
            ),
            op(
                5,
                LoopOperationV2::CompareI64 {
                    op: LoopCompareI64OpV2::LessEqual,
                    left: v(0),
                    right: v(6),
                    result: v(7),
                },
            ),
            LoopRecipeItemRowV2 {
                key: i(6),
                item: LoopRecipeItemV2::If {
                    condition: v(7),
                    then_block: LoopBlockKeyV1::new(2),
                    else_block: None,
                },
            },
            op(
                7,
                LoopOperationV2::ReadBinding {
                    binding: b,
                    result: v(8),
                },
            ),
            LoopRecipeItemRowV2 {
                key: i(8),
                item: LoopRecipeItemV2::Exit {
                    exit: LoopExitKeyV1::new(0),
                },
            },
            op(
                9,
                LoopOperationV2::ReadBinding {
                    binding: b,
                    result: v(9),
                },
            ),
            op(
                10,
                LoopOperationV2::ConstI64 {
                    result: v(10),
                    value: 1,
                },
            ),
            op(
                11,
                LoopOperationV2::BinaryI64 {
                    op: LoopBinaryI64OpV2::Add,
                    left: v(9),
                    right: v(10),
                    result: v(11),
                },
            ),
            op(
                12,
                LoopOperationV2::WriteBinding {
                    binding: b,
                    value: v(11),
                },
            ),
        ],
        bindings: vec![LoopRecipeBindingV2 {
            key: b,
            label: label.to_owned(),
            class: LoopValueClassV2::I64,
        }],
        values: (0..12)
            .map(|raw| LoopRecipeValueV2 {
                key: v(raw),
                class: if raw == 4 || raw == 7 {
                    LoopValueClassV2::Bool
                } else {
                    LoopValueClassV2::I64
                },
            })
            .collect(),
        inputs: vec![v(0), v(1)],
        carriers: vec![LoopRecipeCarrierV2 {
            key: LoopCarrierKeyV1::new(0),
            owner_loop: LoopNodeKeyV1::new(0),
            binding: b,
            class: LoopValueClassV2::I64,
            entry_value: v(1),
        }],
        exits: vec![LoopRecipeExitV2 {
            key: LoopExitKeyV1::new(0),
            owner_loop: LoopNodeKeyV1::new(0),
            kind: LoopExitKindV2::Return { value: Some(v(8)) },
        }],
    }
}
