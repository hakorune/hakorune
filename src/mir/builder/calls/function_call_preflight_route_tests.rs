use super::{
    lower_prepared_raw_function_preflight_with_port_v1, PreparedRawExplicitExternCallV1,
    PreparedRawFunctionPreflightRouteV1, PreparedRawFunctionPreflightV1,
    PreparedRawOrdinaryFunctionCompletionV1,
};
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::recursive_child_lowering::{
    RawFunctionHeaderLookupPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{MirInstruction, MirType, TypeOpKind, ValueId};

#[derive(Default)]
struct RecordingPortV1 {
    expression_count: usize,
    events: Vec<&'static str>,
    fail_expression: bool,
}

impl RecursiveChildLoweringPortV1 for RecordingPortV1 {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        unreachable!("FunctionCall route test does not lower a body")
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        unreachable!("FunctionCall route test does not lower a statement")
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        _input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        self.expression_count += 1;
        self.events.push("child");
        if self.fail_expression {
            return Err("direct str child failed".to_owned());
        }
        crate::mir::builder::emission::constant::emit_integer(builder, 7)
    }
}

impl RawFunctionHeaderLookupPortV1 for RecordingPortV1 {
    fn with_function_headers<R>(
        &mut self,
        observe: impl for<'headers> FnOnce(
                Option<
                    &'headers dyn crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1,
                >,
            ) -> R,
    ) -> R {
        self.events.push("header");
        observe(None)
    }
}

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    literal(LiteralValue::Integer(value))
}

fn new_box(name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::New {
        class: name.to_string(),
        type_arguments: Vec::new(),
        arguments,
        field_initializers: Vec::new(),
        span: Span::unknown(),
    }
}

#[test]
fn direct_function_preflight_priority_is_total() {
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .register_brand_decl("sin".to_string(), "Integer".to_string());
    builder
        .comp_ctx
        .register_brand_decl("isType".to_string(), "Integer".to_string());
    builder
        .comp_ctx
        .register_brand_decl("mem.addr".to_string(), "Integer".to_string());
    builder
        .comp_ctx
        .register_brand_decl("str".to_string(), "Integer".to_string());
    builder.push_fastmem_region(FastMemRegionId::new(6));

    let weak =
        PreparedRawFunctionPreflightV1::prepare(&builder, "weak".to_string(), vec![integer(1)]);
    assert!(matches!(
        weak.route,
        PreparedRawFunctionPreflightRouteV1::WeakReject
    ));

    let explicit = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "externcall".to_string(),
        vec![integer(1)],
    );
    assert!(matches!(
        explicit.route,
        PreparedRawFunctionPreflightRouteV1::ExplicitExtern(_)
    ));

    let brand =
        PreparedRawFunctionPreflightV1::prepare(&builder, "sin".to_string(), vec![integer(1)]);
    assert!(matches!(
        brand.route,
        PreparedRawFunctionPreflightRouteV1::Brand(_)
    ));

    for (name, arguments) in [
        (
            "isType",
            vec![
                integer(1),
                literal(LiteralValue::String("Integer".to_string())),
            ],
        ),
        ("mem.addr", vec![integer(1)]),
        ("str", vec![integer(1)]),
    ] {
        let collision =
            PreparedRawFunctionPreflightV1::prepare(&builder, name.to_string(), arguments);
        assert!(matches!(
            collision.route,
            PreparedRawFunctionPreflightRouteV1::Brand(_)
        ));
    }

    let mut builder = MirBuilder::new();
    let typeop = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "isType".to_string(),
        vec![
            integer(1),
            literal(LiteralValue::String("Integer".to_string())),
        ],
    );
    assert!(matches!(
        typeop.route,
        PreparedRawFunctionPreflightRouteV1::TypeOp { .. }
    ));

    let malformed_typeop = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "isType".to_string(),
        vec![integer(1), integer(2)],
    );
    assert!(matches!(
        malformed_typeop.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary { .. }
    ));

    let math =
        PreparedRawFunctionPreflightV1::prepare(&builder, "sqrt".to_string(), vec![integer(4)]);
    assert!(matches!(
        math.route,
        PreparedRawFunctionPreflightRouteV1::Math { .. }
    ));

    let inactive_fastmem =
        PreparedRawFunctionPreflightV1::prepare(&builder, "mem.addr".to_string(), vec![integer(1)]);
    assert!(matches!(
        inactive_fastmem.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary { .. }
    ));

    builder.push_fastmem_region(FastMemRegionId::new(7));
    let fastmem =
        PreparedRawFunctionPreflightV1::prepare(&builder, "mem.addr".to_string(), vec![integer(1)]);
    assert!(matches!(
        fastmem.route,
        PreparedRawFunctionPreflightRouteV1::FastMem { .. }
    ));

    let ordinary = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "user_function".to_string(),
        vec![integer(1)],
    );
    assert!(matches!(
        ordinary.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary { .. }
    ));

    let str_one =
        PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_string(), vec![integer(1)]);
    assert!(matches!(
        str_one.route,
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: PreparedRawOrdinaryFunctionCompletionV1::StrNormalization { .. }
        }
    ));
    for arguments in [Vec::new(), vec![integer(1), integer(2)]] {
        let wrong_arity =
            PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_string(), arguments);
        assert!(matches!(
            wrong_arity.route,
            PreparedRawFunctionPreflightRouteV1::Ordinary {
                completion: PreparedRawOrdinaryFunctionCompletionV1::Resolved { .. }
            }
        ));
    }
}

#[test]
fn rejecting_routes_precede_children_and_typeop_uses_one_child() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("direct_preflight_order/0".to_string());
    builder
        .comp_ctx
        .register_brand_decl("Meter".to_string(), "Integer".to_string());
    let mut port = RecordingPortV1::default();

    for (name, arguments) in [
        ("externcall", vec![integer(1)]),
        ("Meter", Vec::new()),
        ("Meter", vec![integer(1), integer(2)]),
    ] {
        let prepared =
            PreparedRawFunctionPreflightV1::prepare(&builder, name.to_string(), arguments);
        assert!(lower_prepared_raw_function_preflight_with_port_v1(
            &mut builder,
            &mut port,
            prepared,
        )
        .is_err());
        assert_eq!(port.expression_count, 0);
    }

    let typeop = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "asType".to_string(),
        vec![
            integer(1),
            literal(LiteralValue::String("Integer".to_string())),
        ],
    );
    lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, typeop).unwrap();
    assert_eq!(port.expression_count, 1);

    let malformed_typeop = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "isType".to_string(),
        vec![integer(1), integer(2)],
    );
    let _ = lower_prepared_raw_function_preflight_with_port_v1(
        &mut builder,
        &mut port,
        malformed_typeop,
    );
    assert_eq!(port.expression_count, 3);

    let inactive_fastmem =
        PreparedRawFunctionPreflightV1::prepare(&builder, "mem.addr".to_string(), vec![integer(1)]);
    let _ = lower_prepared_raw_function_preflight_with_port_v1(
        &mut builder,
        &mut port,
        inactive_fastmem,
    );
    assert_eq!(port.expression_count, 4);

    builder.push_fastmem_region(FastMemRegionId::new(8));
    let unknown_fastmem = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "mem.unknown".to_string(),
        vec![integer(1)],
    );
    let error = lower_prepared_raw_function_preflight_with_port_v1(
        &mut builder,
        &mut port,
        unknown_fastmem,
    )
    .unwrap_err();
    assert!(error.contains("[fastmem/forbidden_call]"));
    assert_eq!(port.expression_count, 4);

    let wrong_arity = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "mem.addr".to_string(),
        vec![integer(1), integer(2)],
    );
    let error =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, wrong_arity)
            .unwrap_err();
    assert!(error.contains("[fastmem/arity] call=mem.addr expected=1 actual=2"));
    assert_eq!(port.expression_count, 4);
}

#[test]
fn explicit_extern_preflight_defers_rejection_and_preserves_stringbox_target() {
    assert!(matches!(
        PreparedRawExplicitExternCallV1::prepare(Vec::new()),
        PreparedRawExplicitExternCallV1::MissingTarget
    ));
    assert!(matches!(
        PreparedRawExplicitExternCallV1::prepare(vec![integer(1)]),
        PreparedRawExplicitExternCallV1::TargetMustBeString
    ));

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("direct_extern_preflight/0".to_string());
    let mut port = RecordingPortV1::default();
    for arguments in [Vec::new(), vec![integer(1), integer(2)]] {
        let prepared =
            PreparedRawFunctionPreflightV1::prepare(&builder, "externcall".to_string(), arguments);
        assert!(lower_prepared_raw_function_preflight_with_port_v1(
            &mut builder,
            &mut port,
            prepared,
        )
        .is_err());
        assert_eq!(port.expression_count, 0);
    }

    let target = new_box(
        "StringBox",
        vec![literal(LiteralValue::String("hako_mem_alloc".to_string()))],
    );
    let prepared = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "externcall".to_string(),
        vec![target, integer(7)],
    );
    let value =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, prepared)
            .unwrap();
    assert_eq!(port.expression_count, 1);
    assert_eq!(
        builder.function_state.type_ctx.value_types.get(&value),
        Some(&MirType::Integer)
    );
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|instruction| matches!(
            instruction,
            MirInstruction::Call {
                dst: Some(dst),
                callee: Some(crate::mir::Callee::Extern(_)),
                ..
            } if *dst == value
        )));
}

#[test]
fn selected_math_and_ordinary_str_keep_child_and_completion_order() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("direct_preflight_completion/0".to_string());
    let mut port = RecordingPortV1::default();

    let math = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "sqrt".to_string(),
        vec![new_box("IntegerBox", vec![integer(9)])],
    );
    lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, math).unwrap();
    assert_eq!(port.expression_count, 1);
    assert_eq!(port.events, vec!["child"]);
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|instruction| matches!(
            instruction,
            MirInstruction::TypeOp {
                op: TypeOpKind::Cast,
                ..
            }
        )));

    let string =
        PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_string(), vec![integer(1)]);
    lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, string).unwrap();
    assert_eq!(port.expression_count, 2);
    assert_eq!(port.events, vec!["child", "child"]);

    port.events.clear();
    let ordinary = PreparedRawFunctionPreflightV1::prepare(
        &builder,
        "user_function".to_string(),
        vec![integer(1)],
    );
    let _ = lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, ordinary);
    assert_eq!(port.events, vec!["child", "header"]);
}

#[test]
fn direct_str_child_failure_does_not_retry_or_observe_headers_and_reuses_builder() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("direct_str_failure_reuse/0".to_owned());
    let mut port = RecordingPortV1 {
        fail_expression: true,
        ..Default::default()
    };

    let failing =
        PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_owned(), vec![integer(1)]);
    let error =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, failing)
            .unwrap_err();
    assert_eq!(error, "direct str child failed");
    assert_eq!(port.events, vec!["child"]);

    port.fail_expression = false;
    port.events.clear();
    let succeeding =
        PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_owned(), vec![integer(2)]);
    lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, succeeding)
        .unwrap();
    assert_eq!(port.events, vec!["child"]);
    assert_eq!(port.expression_count, 2);
}
