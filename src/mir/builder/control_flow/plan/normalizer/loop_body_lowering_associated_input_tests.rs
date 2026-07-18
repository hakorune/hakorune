use super::loop_body_lowering;
use super::loop_body_lowering_associated_input as associated;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::{
    CoreCallSourceV1, CoreEffectPlan, CorePlan, RawLoopPlanExpressionPortV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, MirType, ValueId};
use std::collections::BTreeMap;

fn span() -> Span {
    Span::unknown()
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: span(),
    }
}

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

fn assert_const_then_global(effects: &[CoreEffectPlan], target: &str) {
    assert!(matches!(
        effects,
        [
            CoreEffectPlan::Const {
                value: ConstValue::Integer(7),
                ..
            },
            CoreEffectPlan::GlobalCall {
                source: CoreCallSourceV1::Unlocated,
                dst: None,
                func,
                args,
            }
        ] if func == target && args.len() == 1
    ));
}

#[test]
fn raw_function_call_statement_facade_matches_associated_input() {
    let call = ASTNode::FunctionCall {
        name: "Worker.run".to_string(),
        arguments: vec![int(7)],
        span: span(),
    };
    let bindings = BTreeMap::new();
    let mut raw_builder = MirBuilder::new();
    let raw = loop_body_lowering::lower_function_call_stmt(
        &mut raw_builder,
        &bindings,
        &call,
        "raw function call",
    )
    .expect("raw facade");

    let port = RawLoopPlanExpressionPortV1::new();
    let mut associated_builder = MirBuilder::new();
    let via_port = associated::lower_function_call_statement_input(
        &port,
        port.expr(&call),
        &mut associated_builder,
        &bindings,
        "associated function call",
    )
    .expect("associated input");

    assert_const_then_global(&raw, "Worker.run");
    assert_const_then_global(&via_port, "Worker.run");
    assert_eq!(
        raw_builder.type_ctx.value_types,
        associated_builder.type_ctx.value_types
    );
}

#[test]
fn raw_assignment_and_local_facades_match_associated_inputs() {
    let target = var("x");
    let value = int(7);
    let bindings = BTreeMap::new();
    let mut raw_builder = MirBuilder::new();
    let raw_assignment = loop_body_lowering::lower_assignment_stmt(
        &mut raw_builder,
        &bindings,
        &target,
        &value,
        "raw assignment",
    )
    .expect("raw assignment facade");

    let port = RawLoopPlanExpressionPortV1::new();
    let mut associated_builder = MirBuilder::new();
    let associated_assignment = associated::lower_assignment_inputs(
        &port,
        port.expr(&target),
        port.expr(&value),
        &mut associated_builder,
        &bindings,
        "associated assignment",
    )
    .expect("associated assignment");
    assert_eq!(raw_assignment.0, associated_assignment.0);
    assert!(matches!(
        (
            raw_assignment.1.as_slice(),
            associated_assignment.1.as_slice()
        ),
        (
            [CoreEffectPlan::Const {
                value: ConstValue::Integer(7),
                ..
            }],
            [CoreEffectPlan::Const {
                value: ConstValue::Integer(7),
                ..
            }]
        )
    ));

    let variables = vec!["item".to_string()];
    let initial_values = vec![Some(Box::new(int(7)))];
    let raw_local = loop_body_lowering::lower_local_init_values(
        &mut raw_builder,
        &bindings,
        &variables,
        &initial_values,
        "raw local",
    )
    .expect("raw local facade");
    let associated_local = associated::lower_local_initializer_inputs(
        &port,
        &variables,
        vec![Some(
            port.expr(initial_values[0].as_deref().expect("initializer")),
        )],
        &mut associated_builder,
        &bindings,
        "associated local",
    )
    .expect("associated local");
    assert_eq!(raw_local.0.len(), associated_local.0.len());
    assert!(matches!(
        (raw_local.1.as_slice(), associated_local.1.as_slice()),
        (
            [CoreEffectPlan::Const {
                value: ConstValue::Integer(7),
                ..
            }],
            [CoreEffectPlan::Const {
                value: ConstValue::Integer(7),
                ..
            }]
        )
    ));
}

#[test]
fn raw_method_statement_and_associated_return_preserve_statement_semantics() {
    let call = ASTNode::MethodCall {
        object: Box::new(var("obj")),
        method: "touch".to_string(),
        arguments: vec![int(7)],
        span: span(),
    };
    let object = ValueId::new(40);
    let bindings = BTreeMap::from([("obj".to_string(), object)]);
    let mut raw_builder = MirBuilder::new();
    let raw = loop_body_lowering::lower_method_call_stmt(
        &mut raw_builder,
        &bindings,
        &call,
        "raw method",
    )
    .expect("raw method facade");
    let port = RawLoopPlanExpressionPortV1::new();
    let mut associated_builder = MirBuilder::new();
    let via_port = associated::lower_method_call_statement_input(
        &port,
        port.expr(&call),
        &mut associated_builder,
        &bindings,
        "associated method",
    )
    .expect("associated method");
    for effects in [&raw, &via_port] {
        assert!(matches!(
            effects.as_slice(),
            [
                CoreEffectPlan::Const { value: ConstValue::Integer(7), .. },
                CoreEffectPlan::MethodCall {
                    source: CoreCallSourceV1::Unlocated,
                    dst: None,
                    object: actual,
                    method,
                    args,
                    ..
                }
            ] if *actual == object && method == "touch" && args.len() == 1
        ));
    }

    let return_stmt = ASTNode::Return {
        value: Some(Box::new(int(7))),
        span: span(),
    };
    let plans = associated::lower_return_statement_input(
        &port,
        &return_stmt,
        &mut associated_builder,
        &bindings,
        "associated return",
    )
    .expect("associated return");
    assert!(matches!(
        plans.as_slice(),
        [
            CorePlan::Effect(CoreEffectPlan::Const {
                value: ConstValue::Integer(7),
                ..
            }),
            CorePlan::Exit(_)
        ]
    ));
    assert!(associated_builder
        .type_ctx
        .value_types
        .values()
        .any(|ty| *ty == MirType::Integer));
}
