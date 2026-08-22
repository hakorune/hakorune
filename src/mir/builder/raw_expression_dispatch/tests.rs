use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::{MirBuilder, MirInstruction, MirType, ValueId};
use crate::parser::NyashParser;
use hakorune_mir_builder::BoxCompilationContext;

use super::input_view::{RawLegacyBodyInputV1, RawLegacyStatementInputV1};
use crate::mir::builder::builder_build::PreparedRawNewExpressionV1;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::builder::stmts::block_stmt::{
    build_block, build_block_input_view_with_port_v1, build_statement,
    build_statement_input_view_with_port_v1,
};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

fn parsed_box(source: &str) -> ASTNode {
    let ASTNode::Program { mut statements, .. } =
        NyashParser::parse_from_string(source).expect("static Box fixture must parse")
    else {
        panic!("parser must return Program");
    };
    assert_eq!(statements.len(), 1);
    statements.remove(0)
}

fn seeded_static_box_caller() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.prepare_module().expect("module shell");
    builder.root_is_app_mode = Some(false);
    builder.enter_function_for_test("static_box_parent/0".to_string());
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("caller".to_string(), ValueId(41));
    builder
        .function_state
        .type_ctx
        .set_type(ValueId(42), MirType::Integer);
    let mut slots = FunctionSlotRegistry::new();
    slots.ensure_slot("caller", Some(MirType::Integer));
    builder.comp_ctx.current_slot_registry = Some(slots);
    let mut box_context = BoxCompilationContext::new();
    box_context
        .variable_map
        .insert("caller".to_string(), ValueId(43));
    builder.comp_ctx.compilation_context = Some(box_context);
    builder
}

fn assert_static_box_caller_restored(builder: &MirBuilder) {
    assert_eq!(
        builder
            .function_state
            .variable_ctx
            .variable_map
            .get("caller"),
        Some(&ValueId(41))
    );
    assert_eq!(
        builder.function_state.type_ctx.get_type(ValueId(42)),
        Some(&MirType::Integer)
    );
    assert!(builder
        .comp_ctx
        .current_slot_registry
        .as_ref()
        .is_some_and(|slots| slots.get_slot("caller").is_some()));
    assert_eq!(
        builder
            .comp_ctx
            .compilation_context
            .as_ref()
            .and_then(|context| context.variable_map.get("caller")),
        Some(&ValueId(43))
    );
}

#[test]
fn prepared_integer_new_consumes_one_const_route() {
    crate::test_support::with_env_var("NYASH_MIR_CORE13_PURE", "off", || {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("prepared_integer_new/0".to_owned());
        let prepared = PreparedRawNewExpressionV1::prepare(
            &builder,
            "IntegerBox".to_owned(),
            vec![integer(9)],
            Vec::new(),
        )
        .unwrap();
        let mut port = RawLegacyChildLoweringPortV1;
        let dst = builder
            .lower_prepared_raw_new_expression_with_port_v1(&mut port, prepared)
            .unwrap();
        let instructions = instructions(&builder);
        assert_eq!(
            instructions
                .iter()
                .filter(|instruction| matches!(
                    instruction,
                    MirInstruction::Const {
                        dst: actual,
                        value: crate::mir::ConstValue::Integer(9),
                    } if *actual == dst
                ))
                .count(),
            1
        );
        assert!(!instructions
            .iter()
            .any(|instruction| matches!(instruction, MirInstruction::NewBox { .. })));
    });
}

#[test]
fn legacy_body_and_statement_facades_preserve_input_view_parity() {
    let body = vec![integer(1), add(integer(2), integer(3))];

    let mut body_facade = MirBuilder::new();
    body_facade.enter_function_for_test("raw_input_body/0".to_string());
    let facade_body_value = build_block(&mut body_facade, body.clone()).unwrap();

    let mut body_view = MirBuilder::new();
    body_view.enter_function_for_test("raw_input_body/0".to_string());
    let mut body_port = RawLegacyChildLoweringPortV1;
    let view_body_value = build_block_input_view_with_port_v1(
        &mut body_view,
        &mut body_port,
        RawLegacyBodyInputV1::new(body),
    )
    .unwrap();

    assert_eq!(view_body_value, facade_body_value);
    assert_eq!(instructions(&body_view), instructions(&body_facade));

    let statement = add(integer(4), integer(5));
    let mut statement_facade = MirBuilder::new();
    statement_facade.enter_function_for_test("raw_input_statement/0".to_string());
    let facade_statement_value = build_statement(&mut statement_facade, statement.clone()).unwrap();

    let mut statement_view = MirBuilder::new();
    statement_view.enter_function_for_test("raw_input_statement/0".to_string());
    let mut statement_port = RawLegacyChildLoweringPortV1;
    let view_statement_value = build_statement_input_view_with_port_v1(
        &mut statement_view,
        &mut statement_port,
        RawLegacyStatementInputV1::new(statement),
    )
    .unwrap();

    assert_eq!(view_statement_value, facade_statement_value);
    assert_eq!(
        instructions(&statement_view),
        instructions(&statement_facade)
    );
}

#[test]
fn raw_nonmain_static_box_success_restores_four_state_caller() {
    let mut builder = seeded_static_box_caller();
    let mut port = RawLegacyChildLoweringPortV1;
    let result = builder
        .build_expression_impl_with_port_v1(
            &mut port,
            parsed_box("static box Helpers { alpha() { return 1 } beta() { return 2 } }"),
        )
        .unwrap();

    assert_static_box_caller_restored(&builder);
    assert_eq!(
        builder.function_state.type_ctx.get_type(result),
        Some(&MirType::Void)
    );
    let module = builder.current_module.as_ref().expect("module shell");
    assert!(module.functions.contains_key("Helpers.alpha/0"));
    assert!(module.functions.contains_key("Helpers.beta/0"));
}

#[test]
fn raw_nonmain_static_box_app_mode_is_void_without_registration() {
    let mut builder = seeded_static_box_caller();
    builder.root_is_app_mode = Some(true);
    let mut port = RawLegacyChildLoweringPortV1;
    let result = builder
        .build_expression_impl_with_port_v1(
            &mut port,
            parsed_box("static box Helpers { alpha() { return 1 } }"),
        )
        .expect("App-mode static Box is a no-op");

    assert_eq!(
        builder.function_state.type_ctx.get_type(result),
        Some(&MirType::Void)
    );
    assert!(!builder.comp_ctx.user_defined_boxes.contains_key("Helpers"));
    let module = builder.current_module.as_ref().expect("module shell");
    assert!(!module.functions.contains_key("Helpers.alpha/0"));
}

#[test]
fn raw_nonmain_static_box_undecided_mode_freezes_before_registration() {
    let mut builder = seeded_static_box_caller();
    builder.root_is_app_mode = None;
    let before_functions = builder
        .current_module
        .as_ref()
        .expect("module shell")
        .functions
        .len();
    let mut port = RawLegacyChildLoweringPortV1;

    let error = builder
        .build_expression_impl_with_port_v1(
            &mut port,
            parsed_box("static box Helpers { alpha() { return 1 } }"),
        )
        .expect_err("unset root mode must freeze before lifecycle effects");

    assert_eq!(error, "[freeze:contract][mir/root-app-mode/undecided]");
    assert!(!builder.comp_ctx.user_defined_boxes.contains_key("Helpers"));
    assert_eq!(
        builder
            .current_module
            .as_ref()
            .expect("module shell")
            .functions
            .len(),
        before_functions
    );
    assert_static_box_caller_restored(&builder);
}

#[test]
fn normal_nonmain_static_box_undecided_mode_freezes_before_registration() {
    let mut builder = seeded_static_box_caller();
    builder.root_is_app_mode = None;
    let mut port = RawLegacyChildLoweringPortV1;

    let error = super::PreparedRawNonMainStaticBoxLifecycleV1::prepare(
        "Helpers".to_owned(),
        match parsed_box("static box Helpers { alpha() { return 1 } }") {
            ASTNode::BoxDeclaration { methods, .. } => methods,
            other => panic!("expected static box, got {other:?}"),
        },
    )
    .lower_normal_with_port_v1(&mut builder, &mut port)
    .expect_err("unset root mode must freeze before normal lifecycle effects");

    assert_eq!(error, "[freeze:contract][mir/root-app-mode/undecided]");
    assert!(!builder.comp_ctx.user_defined_boxes.contains_key("Helpers"));
    assert_static_box_caller_restored(&builder);
}

#[test]
fn raw_nonmain_static_box_failure_keeps_inner_state_and_primary_error() {
    let mut builder = seeded_static_box_caller();
    let mut port = RawLegacyChildLoweringPortV1;
    let error = builder
        .build_expression_impl_with_port_v1(&mut port, parsed_box(
            "static box Broken { alpha() { return 1 } beta() { return missing } gamma() { return 3 } }",
        ))
        .unwrap_err();

    assert!(error.contains("Undefined variable: missing"), "{error}");
    assert!(!builder
        .function_state
        .variable_ctx
        .variable_map
        .contains_key("caller"));
    assert_eq!(builder.function_state.type_ctx.get_type(ValueId(42)), None);
    assert!(builder.comp_ctx.current_slot_registry.is_none());
    assert!(builder
        .comp_ctx
        .compilation_context
        .as_ref()
        .is_some_and(BoxCompilationContext::is_empty));
    assert!(builder.comp_ctx.user_defined_boxes.contains_key("Broken"));
    let module = builder.current_module.as_ref().expect("module shell");
    assert!(module.functions.contains_key("Broken.alpha/0"));
    assert!(!module.functions.contains_key("Broken.gamma/0"));
}

#[test]
fn raw_lambda_dispatches_once_with_source_ordered_captures() {
    let mut builder = MirBuilder::new();
    builder.prepare_module().expect("module shell");
    builder.enter_function_for_test("raw_lambda_dispatch/0".to_string());
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("second".into(), ValueId(12));
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("first".into(), ValueId(11));
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("me".into(), ValueId(99));

    let lambda = ASTNode::Lambda {
        params: vec![],
        body: vec![add(variable("first"), variable("second"))],
        span: Span::unknown(),
    };
    let mut port = RawLegacyChildLoweringPortV1;
    let dst = builder
        .build_expression_impl_with_port_v1(&mut port, lambda)
        .unwrap();

    let closure = instructions(&builder)
        .into_iter()
        .find(|instruction| matches!(instruction, MirInstruction::NewClosure { dst: actual, .. } if *actual == dst))
        .expect("one closure instruction");
    assert!(matches!(
        closure,
        MirInstruction::NewClosure {
            body_id: Some(0),
            captures,
            me: None,
            ..
        } if captures == vec![("first".into(), ValueId(11)), ("second".into(), ValueId(12))]
    ));
    assert_eq!(
        builder
            .current_module
            .as_ref()
            .unwrap()
            .metadata
            .closure_bodies
            .len(),
        1
    );
}
