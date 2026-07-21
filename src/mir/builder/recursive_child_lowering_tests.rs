use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::{
    BasicBlockId, Effect, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirInstruction,
    MirType, ValueId,
};

use super::recursive_child_lowering::{
    drive_legacy_body_v1, drive_legacy_expression_v1, drive_legacy_statement_v1,
    RawInvocationChildPortV1, RecursiveChildLoweringPortV1,
};
use super::{
    module_draft_collector::{
        DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1,
    },
    module_lowering_invocation::ModuleLoweringInvocationV1,
};

struct BodyTokenV1(i64);
struct StatementTokenV1(i64);
struct ExpressionTokenV1(i64);

#[derive(Default)]
struct CountingPortV1 {
    body_calls: usize,
    statement_calls: usize,
    expression_calls: usize,
    fail_expression: bool,
}

impl RecursiveChildLoweringPortV1 for CountingPortV1 {
    type BodyInput = BodyTokenV1;
    type StatementInput = StatementTokenV1;
    type ExpressionInput = ExpressionTokenV1;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        self.body_calls += 1;
        crate::mir::builder::emission::constant::emit_integer(builder, input.0)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        self.statement_calls += 1;
        crate::mir::builder::emission::constant::emit_integer(builder, input.0)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        self.expression_calls += 1;
        if self.fail_expression {
            return Err("counting-expression-failure".to_string());
        }
        crate::mir::builder::emission::constant::emit_integer(builder, input.0)
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
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

fn typeop(receiver: ASTNode) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(receiver),
        method: "is".to_string(),
        arguments: vec![ASTNode::Literal {
            value: LiteralValue::String("Integer".to_string()),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("current test function");
    function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

fn collected_header(symbol: &str, arity: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_string(),
            params: vec![MirType::Integer; arity],
            return_type: MirType::Void,
            effects: EffectMask::READ.add(Effect::ReadHeap),
        },
        BasicBlockId(0),
    )
}

fn collector_with_header(symbol: &str, arity: usize) -> ModuleDraftCollectorV1 {
    let mut collector = ModuleDraftCollectorV1::default();
    collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol(symbol.to_string()),
            symbol.to_string(),
            arity,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(collected_header(symbol, arity))
        .unwrap()
        .collect();
    collector
}

#[test]
fn associated_inputs_dispatch_each_child_kind_exactly_once() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("recursive_child_inputs/0".to_string());
    let mut port = CountingPortV1::default();

    drive_legacy_body_v1(&mut builder, &mut port, BodyTokenV1(1)).unwrap();
    drive_legacy_statement_v1(&mut builder, &mut port, StatementTokenV1(2)).unwrap();
    drive_legacy_expression_v1(&mut builder, &mut port, ExpressionTokenV1(3)).unwrap();

    assert_eq!(port.body_calls, 1);
    assert_eq!(port.statement_calls, 1);
    assert_eq!(port.expression_calls, 1);
}

#[test]
fn child_driver_propagates_failure_without_retry() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("recursive_child_failure/0".to_string());
    let mut port = CountingPortV1 {
        fail_expression: true,
        ..CountingPortV1::default()
    };

    assert_eq!(
        drive_legacy_expression_v1(&mut builder, &mut port, ExpressionTokenV1(3)).unwrap_err(),
        "counting-expression-failure"
    );
    assert_eq!(port.expression_calls, 1);
}

#[test]
fn raw_invocation_port_reborrows_one_collector_backed_header_view() {
    let mut builder = MirBuilder::new();
    let mut invocation = ModuleLoweringInvocationV1::with_collector(
        &mut builder,
        collector_with_header("Prefix.f/1", 1),
    );

    invocation.with_module_port(|_builder, module_port| {
        let mut root = RawInvocationChildPortV1::new(module_port);
        root.with_headers(|headers| {
            assert_eq!(headers.signature("Prefix.f/1").unwrap().params.len(), 1);
            assert_eq!(headers.symbol_count(), 1);
        });

        let nested = root.reborrow();
        nested.with_headers(|headers| {
            assert!(headers.contains_symbol("Prefix.f/1"));
            assert_eq!(
                headers.signature("Prefix.f/1").unwrap().return_type,
                MirType::Void
            );
        });
    });
}

#[test]
fn selected_raw_expression_port_preserves_nested_mir() {
    let expression = add(integer(1), add(integer(2), integer(3)));
    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("recursive_child_nested/0".to_string());
    let selected_output = selected.build_expression(expression.clone()).unwrap();

    let mut raw_leaf = MirBuilder::new();
    raw_leaf.enter_function_for_test("recursive_child_nested/0".to_string());
    let raw_output = raw_leaf.build_expression_impl(expression).unwrap();

    assert_eq!(selected_output, raw_output);
    assert_eq!(instructions(&selected), instructions(&raw_leaf));
}

#[test]
fn selected_raw_body_and_statement_ports_preserve_order_and_last_value() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("recursive_child_body/0".to_string());

    let output = builder.build_block(vec![integer(4), integer(5)]).unwrap();
    let integer_rows = instructions(&builder)
        .into_iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Const {
                dst,
                value: crate::mir::ConstValue::Integer(value),
            } => Some((dst, value)),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(integer_rows.len(), 2);
    assert_eq!(integer_rows[0].1, 4);
    assert_eq!(integer_rows[1].1, 5);
    assert_eq!(output, integer_rows[1].0);

    let statement_output = builder.build_statement(integer(6)).unwrap();
    assert!(instructions(&builder).iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Const {
            dst,
            value: crate::mir::ConstValue::Integer(6),
        } if *dst == statement_output
    )));
}

#[test]
fn expression_failure_restores_recursion_depth_for_reuse() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("recursive_child_reuse/0".to_string());

    assert!(builder
        .build_expression(ASTNode::Variable {
            name: "missing".to_string(),
            span: Span::unknown(),
        })
        .is_err());
    assert_eq!(builder.recursion_depth, 0);
    builder.build_expression(integer(9)).unwrap();
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn raw_expression_depth_limit_rejects_without_poisoning_the_session() {
    let _ = std::panic::catch_unwind(|| {
        crate::runtime::ring0::init_global_ring0(crate::runtime::ring0::default_ring0())
    });
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("recursive_child_depth_limit/0".to_string());
    builder.recursion_depth = 200;

    let error = builder.build_expression(integer(8)).unwrap_err();
    assert!(error.contains("Recursion depth exceeded: 201"));
    assert_eq!(builder.recursion_depth, 200);

    builder.recursion_depth = 0;
    builder.build_expression(integer(9)).unwrap();
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn typeop_receiver_uses_nested_raw_depth_guard_without_publishing_on_failure() {
    let _ = std::panic::catch_unwind(|| {
        crate::runtime::ring0::init_global_ring0(crate::runtime::ring0::default_ring0())
    });
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("typeop_receiver_depth_limit/0".to_string());

    // Public TypeOp lowering consumes one guard for the MethodCall and one for its receiver.
    builder.recursion_depth = 198;
    builder.build_expression(typeop(integer(8))).unwrap();
    assert_eq!(builder.recursion_depth, 198);
    assert_eq!(
        instructions(&builder)
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::TypeOp { .. }))
            .count(),
        1
    );

    builder.recursion_depth = 199;
    let before = instructions(&builder);
    let error = builder.build_expression(typeop(integer(9))).unwrap_err();
    assert!(error.contains("Recursion depth exceeded: 201"));
    assert_eq!(builder.recursion_depth, 199);
    assert_eq!(instructions(&builder), before);

    builder.recursion_depth = 0;
    builder.build_expression(integer(10)).unwrap();
    assert_eq!(builder.recursion_depth, 0);
}
