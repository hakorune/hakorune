use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::instruction::MemOpKind;
use crate::mir::{Callee, MirBuilder, MirInstruction};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.into()),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn method(receiver: &str, name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(variable(receiver)),
        method: name.into(),
        arguments,
        span: Span::unknown(),
    }
}

fn instructions(builder: &MirBuilder) -> impl Iterator<Item = &MirInstruction> {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("current function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
}

#[test]
fn selected_mir_debug_route_preserves_debug_payload() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("mir_debug/0".into());
    let result = builder
        .build_expression(method("__mir__", "log", vec![string("value"), integer(7)]))
        .unwrap();

    assert!(instructions(&builder).any(|instruction| {
        matches!(
            instruction,
            MirInstruction::Debug { value, message }
                if *value != result && message == "value"
        )
    }));
}

#[test]
fn selected_mir_debug_zero_argument_failure_is_stable() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("mir_debug_fail/0".into());
    assert_eq!(
        builder
            .build_expression(method("__mir__", "mark", vec![]))
            .unwrap_err(),
        "__mir__.log/__mir__.mark requires at least a label argument"
    );
    assert!(!instructions(&builder)
        .any(|instruction| matches!(instruction, MirInstruction::Debug { .. })));
}

#[test]
fn selected_repl_route_preserves_extern_call() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("repl/0".into());
    builder
        .build_expression(method("__repl", "get", vec![string("name")]))
        .unwrap();

    assert!(instructions(&builder).any(|instruction| {
        matches!(
            instruction,
            MirInstruction::Call {
                callee: Some(Callee::Extern(name)),
                ..
            } if name == "__repl.get"
        )
    }));
}

#[test]
fn selected_repl_unsupported_method_failure_is_stable() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("repl_fail/0".into());
    assert_eq!(
        builder
            .build_expression(method("__repl", "other", vec![]))
            .unwrap_err(),
        "__repl.other is not supported. Only __repl.get and __repl.set are allowed."
    );
}

#[test]
fn selected_fastmem_method_route_preserves_memop_lowering() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_method/0".into());
    let body = vec![ASTNode::FastMemRegion {
        contract: "PageMapV0".into(),
        body: vec![ASTNode::Local {
            variables: vec!["address".into()],
            initial_values: vec![Some(Box::new(method("mem", "addr", vec![integer(4096)])))],
            declared_type_names: Vec::new(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }];
    crate::mir::builder::stmts::block_stmt::build_block(&mut builder, body).unwrap();

    assert!(instructions(&builder).any(|instruction| {
        matches!(
            instruction,
            MirInstruction::MemOp {
                kind: MemOpKind::AddrOf,
                ..
            }
        )
    }));
}
