//! T0-R0-C0 proof for the associated-input If-condition tail owner.

use super::cond_lowering_if_plan::lower_cond_to_if_plans;
use super::cond_lowering_if_plan_port::lower_cond_expr_to_if_plans_input;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::test_support::{
    with_default_and_strict_modes, GenericLoopTestModeV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::VerifiedLocatedGenericLoopBodyRepresentationV1;
use crate::mir::builder::control_flow::plan::{
    CoreCallSourceV1, CoreEffectPlan, CoreIfJoin, CorePlan, LocatedLoopPlanExpressionPortV1,
    RawLoopPlanExpressionPortV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedCallableResultLegacySourceViewV1,
};
use crate::mir::MirType;
use std::collections::BTreeMap;

#[test]
fn raw_if_condition_facade_matches_explicit_raw_port_core() {
    let cases = [
        binary(BinaryOperator::Add, int(1), int(2)),
        binary(BinaryOperator::Less, int(1), int(2)),
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand: Box::new(binary(BinaryOperator::Equal, int(3), int(4))),
            span: Span::unknown(),
        },
        binary(
            BinaryOperator::And,
            binary(BinaryOperator::Less, int(1), int(2)),
            binary(
                BinaryOperator::Or,
                binary(BinaryOperator::Equal, int(3), int(3)),
                binary(BinaryOperator::Greater, int(5), int(4)),
            ),
        ),
    ];
    for condition in cases {
        let then_plans = vec![CorePlan::Seq(Vec::new())];
        let else_plans = Some(vec![CorePlan::Seq(Vec::new())]);
        let mut facade_builder = MirBuilder::new();
        let facade = lower_cond_to_if_plans(
            &mut facade_builder,
            &BTreeMap::new(),
            &CondBlockView::from_expr(&condition),
            then_plans.clone(),
            else_plans.clone(),
            Vec::new(),
            "T0-R0-C0 raw facade",
        )
        .expect("raw facade lowers");

        let port = RawLoopPlanExpressionPortV1::new();
        let mut core_builder = MirBuilder::new();
        let core = lower_cond_expr_to_if_plans_input(
            &port,
            port.expr(&condition),
            &mut core_builder,
            &BTreeMap::new(),
            then_plans,
            else_plans,
            Vec::new(),
            "T0-R0-C0 raw core",
        )
        .expect("raw associated-input core lowers");

        assert_eq!(format!("{facade:?}"), format!("{core:?}"));
        assert_eq!(
            facade_builder.type_ctx.value_types,
            core_builder.type_ctx.value_types
        );
    }
}

#[test]
fn raw_join_bearing_and_or_facade_matches_explicit_port_core() {
    let condition = binary(
        BinaryOperator::And,
        binary(BinaryOperator::Less, int(1), int(2)),
        binary(
            BinaryOperator::Or,
            binary(BinaryOperator::Equal, int(3), int(3)),
            binary(BinaryOperator::Greater, int(5), int(4)),
        ),
    );
    let mut facade_builder = MirBuilder::new();
    let facade_join = seeded_join(&mut facade_builder);
    let facade = lower_cond_to_if_plans(
        &mut facade_builder,
        &BTreeMap::new(),
        &CondBlockView::from_expr(&condition),
        vec![CorePlan::Seq(Vec::new())],
        Some(vec![CorePlan::Seq(Vec::new())]),
        vec![facade_join],
        "T0-R0-C0 join facade",
    )
    .expect("join-bearing raw facade lowers");

    let port = RawLoopPlanExpressionPortV1::new();
    let mut core_builder = MirBuilder::new();
    let core_join = seeded_join(&mut core_builder);
    let core = lower_cond_expr_to_if_plans_input(
        &port,
        port.expr(&condition),
        &mut core_builder,
        &BTreeMap::new(),
        vec![CorePlan::Seq(Vec::new())],
        Some(vec![CorePlan::Seq(Vec::new())]),
        vec![core_join],
        "T0-R0-C0 join core",
    )
    .expect("join-bearing associated-input core lowers");

    assert_eq!(format!("{facade:?}"), format!("{core:?}"));
    assert_eq!(
        facade_builder.type_ctx.value_types,
        core_builder.type_ctx.value_types
    );
}

#[test]
fn borrowed_located_loop_condition_preserves_exact_call_sites() {
    with_default_and_strict_modes(|mode| {
        if mode != GenericLoopTestModeV1::Default {
            return;
        }
        let plan = actual_parser_add_fixture::plan();
        let caller = actual_parser_add_fixture::caller(&plan);
        let source = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller)
            .expect("located source view");
        let root = source.root_body();
        let loop_root = source.body_stmt(&root, 4).expect("actual Loop at Body(4)");
        let port = LocatedLoopPlanExpressionPortV1::new(source);
        let representation =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, loop_root)
                .expect("strict O0 representation");
        let bound = representation
            .bind_lowering_port(&port)
            .expect("same port binds");
        let condition = bound.condition();

        let mut builder = MirBuilder::new();
        let text = builder.alloc_typed(MirType::String);
        let pos = builder.alloc_typed(MirType::Integer);
        let bindings = BTreeMap::from([("text".to_owned(), text), ("pos".to_owned(), pos)]);
        let plans = lower_cond_expr_to_if_plans_input(
            &port,
            condition,
            &mut builder,
            &bindings,
            vec![CorePlan::Seq(Vec::new())],
            Some(vec![CorePlan::Seq(Vec::new())]),
            Vec::new(),
            "T0-R0-C0 located condition",
        )
        .expect("located exact condition lowers");
        let mut sources = Vec::new();
        collect_call_sources(&plans, &mut sources);
        assert_eq!(sources.len(), 3);
        assert!(sources
            .iter()
            .all(|source| matches!(source, CoreCallSourceV1::LocatedMethodCall(_))));
    });
}

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn seeded_join(builder: &mut MirBuilder) -> CoreIfJoin {
    let pre = builder.alloc_typed(MirType::Integer);
    let then_value = builder.alloc_typed(MirType::Integer);
    let else_value = builder.alloc_typed(MirType::Integer);
    let dst = builder.alloc_typed(MirType::Integer);
    CoreIfJoin {
        name: "value".to_owned(),
        dst,
        pre_val: Some(pre),
        then_val: then_value,
        else_val: else_value,
    }
}

fn collect_call_sources<'a>(plans: &'a [CorePlan], out: &mut Vec<&'a CoreCallSourceV1>) {
    for plan in plans {
        match plan {
            CorePlan::Seq(children) => collect_call_sources(children, out),
            CorePlan::If(node) => {
                collect_call_sources(&node.then_plans, out);
                if let Some(children) = &node.else_plans {
                    collect_call_sources(children, out);
                }
            }
            CorePlan::Effect(effect) => match effect {
                CoreEffectPlan::MethodCall { source, .. }
                | CoreEffectPlan::GlobalCall { source, .. }
                | CoreEffectPlan::ValueCall { source, .. }
                | CoreEffectPlan::ExternCall { source, .. } => out.push(source),
                _ => {}
            },
            CorePlan::Loop(_) | CorePlan::BranchN(_) | CorePlan::Exit(_) => {}
        }
    }
}
