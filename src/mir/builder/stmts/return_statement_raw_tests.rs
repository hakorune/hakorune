use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::{ConstValue, MirBuilder, MirInstruction};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn type_check(receiver: ASTNode) -> ASTNode {
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

fn value_return(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}

fn void_return() -> ASTNode {
    ASTNode::Return {
        value: None,
        span: Span::unknown(),
    }
}

fn accepted_match() -> ASTNode {
    ASTNode::MatchExpr {
        scrutinee: Box::new(integer(2)),
        arms: vec![
            (LiteralValue::Integer(1), integer(10)),
            (LiteralValue::Integer(2), integer(20)),
        ],
        else_expr: Box::new(integer(30)),
        span: Span::unknown(),
    }
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("current RET0-I0 function")
        .blocks
        .values()
        .flat_map(|block| {
            block
                .instructions
                .iter()
                .chain(block.terminator.iter())
                .cloned()
        })
        .collect()
}

fn current_terminator(builder: &MirBuilder) -> Option<MirInstruction> {
    let block = builder.current_block.expect("current RET0-I0 block");
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("current RET0-I0 function")
        .blocks
        .get(&block)
        .expect("current RET0-I0 block body")
        .terminator
        .clone()
}

fn return_count(builder: &MirBuilder) -> usize {
    instructions(builder)
        .iter()
        .filter(|row| matches!(row, MirInstruction::Return { .. }))
        .count()
}

#[test]
fn raw_value_return_selects_owned_descent_for_actual_method_call() {
    let mut builder = builder("ret0_i0_method_call/0");

    let result = builder
        .build_expression(value_return(type_check(integer(8))))
        .unwrap();

    assert!(instructions(&builder)
        .iter()
        .any(|row| matches!(row, MirInstruction::TypeOp { dst, .. } if *dst == result)));
    assert!(matches!(
        current_terminator(&builder),
        Some(MirInstruction::Return { value: Some(value) }) if value == result
    ));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn raw_void_return_stays_on_legacy_facade() {
    let mut builder = builder("ret0_i0_void/0");

    let result = builder.build_expression(void_return()).unwrap();

    assert!(instructions(&builder).iter().any(|row| matches!(
        row,
        MirInstruction::Const {
            dst,
            value: ConstValue::Void,
        } if *dst == result
    )));
    assert!(matches!(
        current_terminator(&builder),
        Some(MirInstruction::Return { value: Some(value) }) if value == result
    ));
}

#[test]
fn raw_match_return_keeps_existing_selection_owner_without_second_completion() {
    let mut builder = builder("ret0_i0_match/0");

    let result = builder
        .build_expression(value_return(accepted_match()))
        .unwrap();
    let rows = instructions(&builder);

    assert!(rows.iter().any(|row| matches!(
        row,
        MirInstruction::Const {
            dst,
            value: ConstValue::Integer(30),
        } if *dst == result
    )));
    assert_eq!(return_count(&builder), 3);
    assert!(!rows
        .iter()
        .any(|row| matches!(row, MirInstruction::Phi { .. })));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn raw_configured_defer_keeps_exact_copy_jump_completion() {
    let mut builder = builder("ret0_i0_defer/0");
    let slot = builder.next_value_id();
    let target = builder.next_block_id();
    builder.return_defer_active = true;
    builder.return_defer_slot = Some(slot);
    builder.return_defer_target = Some(target);

    let result = builder.build_expression(value_return(integer(7))).unwrap();
    let rows = instructions(&builder);

    assert!(builder.return_deferred_emitted);
    assert_eq!(return_count(&builder), 0);
    assert_eq!(
        rows.iter()
            .filter(|row| matches!(row, MirInstruction::Copy { dst, src } if *dst == slot && *src == result))
            .count(),
        1
    );
    assert_eq!(
        rows.iter()
            .filter(|row| matches!(row, MirInstruction::Jump { target: row_target, .. } if *row_target == target))
            .count(),
        1
    );
    assert!(matches!(
        current_terminator(&builder),
        Some(MirInstruction::Jump {
            target: row_target,
            ..
        }) if row_target == target
    ));
}

#[test]
fn raw_cleanup_and_child_failures_leave_no_terminator_then_reuse() {
    let mut cleanup = builder("ret0_i0_cleanup/0");
    cleanup.in_cleanup_block = true;
    cleanup.cleanup_allow_return = false;

    let error = cleanup
        .build_expression(value_return(type_check(integer(8))))
        .unwrap_err();
    assert!(error.contains("return is not allowed inside cleanup block"));
    assert!(instructions(&cleanup).is_empty());
    assert!(current_terminator(&cleanup).is_none());
    assert_eq!(cleanup.recursion_depth, 0);

    cleanup.in_cleanup_block = false;
    cleanup.build_expression(value_return(integer(1))).unwrap();
    assert_eq!(return_count(&cleanup), 1);

    let mut child = builder("ret0_i0_child_failure/0");
    let error = child
        .build_expression(value_return(variable("missing")))
        .unwrap_err();
    assert!(error.contains("Undefined variable: missing"));
    assert!(current_terminator(&child).is_none());
    assert_eq!(child.recursion_depth, 0);

    let result = child.build_expression(value_return(integer(2))).unwrap();
    assert!(matches!(
        current_terminator(&child),
        Some(MirInstruction::Return { value: Some(value) }) if value == result
    ));
}
