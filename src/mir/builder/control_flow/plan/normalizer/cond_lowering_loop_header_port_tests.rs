//! T0-C0 raw-facade parity for the shared loop-condition port core.

use super::cond_lowering_loop_header::{lower_loop_header_cond, LoopHeaderCondResult};
use super::cond_lowering_loop_header_port::lower_loop_header_cond_input;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, RawLoopPlanExpressionPortV1};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

#[test]
fn raw_loop_header_facade_matches_explicit_raw_port_core() {
    let final_comparison_span = Span::new(30, 31, 7, 9);
    let condition = binary(
        BinaryOperator::And,
        binary(BinaryOperator::Less, int(1), int(2)),
        binary(
            BinaryOperator::Or,
            binary(BinaryOperator::Greater, int(3), int(2)),
            binary_at(BinaryOperator::Equal, int(4), int(4), final_comparison_span),
        ),
    );
    let mut facade_builder = MirBuilder::new();
    let mut core_builder = MirBuilder::new();
    let (facade_header, facade_body, facade_after) = blocks(&mut facade_builder);
    let (core_header, core_body, core_after) = blocks(&mut core_builder);

    let facade = lower_loop_header_cond(
        &mut facade_builder,
        &BTreeMap::new(),
        &CondBlockView::from_expr(&condition),
        facade_header,
        facade_body,
        facade_after,
        empty_carriers_args(),
        empty_carriers_args(),
        "T0-C0 raw facade",
    )
    .expect("raw facade lowers");

    let port = RawLoopPlanExpressionPortV1::new();
    let core = lower_loop_header_cond_input(
        &mut core_builder,
        &BTreeMap::new(),
        &port,
        port.expr(&condition),
        core_header,
        core_body,
        core_after,
        empty_carriers_args(),
        empty_carriers_args(),
        "T0-C0 explicit raw port",
    )
    .expect("raw port core lowers");

    assert_eq!(snapshot(&facade), snapshot(&core));
    assert_eq!(facade.preds_to(facade_body), core.preds_to(core_body));
    assert_eq!(facade.preds_to(facade_after), core.preds_to(core_after));
    assert_lazy_and_or_shape(&facade, facade_header, facade_body, facade_after);
    assert_eq!(
        facade_builder.metadata_ctx.current_span(),
        final_comparison_span
    );
    assert_eq!(
        core_builder.metadata_ctx.current_span(),
        final_comparison_span
    );
}

fn blocks(builder: &mut MirBuilder) -> (BasicBlockId, BasicBlockId, BasicBlockId) {
    (
        builder.next_block_id(),
        builder.next_block_id(),
        builder.next_block_id(),
    )
}

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    binary_at(operator, left, right, Span::unknown())
}

fn binary_at(operator: BinaryOperator, left: ASTNode, right: ASTNode, span: Span) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span,
    }
}

fn assert_lazy_and_or_shape(
    result: &LoopHeaderCondResult,
    header: BasicBlockId,
    body: BasicBlockId,
    after: BasicBlockId,
) {
    assert_eq!(result.branches.len(), 3);
    let and_rhs = result.branches[0].then_target;
    let or_rhs = result.branches[1].else_target;
    assert_ne!(and_rhs, body);
    assert_ne!(and_rhs, after);
    assert_ne!(or_rhs, body);
    assert_ne!(or_rhs, after);
    assert_ne!(and_rhs, or_rhs);

    assert_eq!(result.branches[0].from, header);
    assert_eq!(result.branches[0].then_target, and_rhs);
    assert_eq!(result.branches[0].else_target, after);
    assert_eq!(result.branches[1].from, and_rhs);
    assert_eq!(result.branches[1].then_target, body);
    assert_eq!(result.branches[1].else_target, or_rhs);
    assert_eq!(result.branches[2].from, or_rhs);
    assert_eq!(result.branches[2].then_target, body);
    assert_eq!(result.branches[2].else_target, after);

    assert_eq!(result.block_effects.len(), 3);
    assert!(result.block_effects.contains_key(&header));
    assert!(result.block_effects.contains_key(&and_rhs));
    assert!(result.block_effects.contains_key(&or_rhs));
    assert_eq!(result.preds_to(body).len(), 2);
    assert_eq!(result.preds_to(after).len(), 2);
}

#[derive(Debug, PartialEq, Eq)]
struct HeaderSnapshot {
    first_cond: ValueId,
    blocks: Vec<(BasicBlockId, Vec<String>)>,
    branches: Vec<String>,
}

fn snapshot(result: &LoopHeaderCondResult) -> HeaderSnapshot {
    HeaderSnapshot {
        first_cond: result.first_cond,
        blocks: result
            .block_effects
            .iter()
            .map(|(block, effects)| {
                (
                    *block,
                    effects.iter().map(effect_snapshot).collect::<Vec<_>>(),
                )
            })
            .collect(),
        branches: result
            .branches
            .iter()
            .map(|branch| {
                format!(
                    "{:?}:%{}->{:?}/{:?}",
                    branch.from, branch.cond.0, branch.then_target, branch.else_target
                )
            })
            .collect(),
    }
}

fn effect_snapshot(effect: &CoreEffectPlan) -> String {
    match effect {
        CoreEffectPlan::Const { dst, value } => format!("const:%{}:{value:?}", dst.0),
        CoreEffectPlan::Compare { dst, lhs, op, rhs } => {
            format!("compare:%{}:%{}:{op:?}:%{}", dst.0, lhs.0, rhs.0)
        }
        other => panic!("unexpected T0-C0 parity effect: {other:?}"),
    }
}
