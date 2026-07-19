use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::{BindingId, ConstValue, MirBuilder, MirInstruction, ValueId};

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

fn binary(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn assignment(target: ASTNode, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(target),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn grouped_assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::GroupedAssignmentExpr {
        lhs: name.to_string(),
        rhs: Box::new(value),
        span: Span::unknown(),
    }
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn declare(builder: &mut MirBuilder, name: &str, value: ValueId, binding: u32) {
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert(name.to_string(), value);
    builder
        .function_state
        .binding_ctx
        .insert(name.to_string(), BindingId::new(binding));
}

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current ASN0 raw function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

#[test]
fn raw_variable_assignment_selects_owned_descent_and_recursive_rhs() {
    let mut builder = builder("asn0_i0_raw/0");
    let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
    declare(&mut builder, "x", old, 0);

    let result = builder
        .build_expression(assignment(variable("x"), binary(integer(2), integer(3))))
        .unwrap();

    assert_eq!(
        builder.function_state.variable_ctx.variable_map.get("x"),
        Some(&result)
    );
    assert_ne!(result, old);
    let rows = instructions(&builder);
    assert_eq!(
        rows.iter()
            .filter(|row| matches!(row, MirInstruction::BinOp { .. }))
            .count(),
        1
    );
    assert_eq!(
        rows.iter()
            .filter(|row| matches!(row, MirInstruction::ReleaseStrong { .. }))
            .count(),
        1
    );
}

#[test]
fn raw_undeclared_target_rejects_before_rhs_effects() {
    let mut builder = builder("asn0_i0_undeclared/0");

    let error = builder
        .build_expression(assignment(
            variable("missing"),
            binary(integer(40), integer(2)),
        ))
        .unwrap_err();

    assert!(error.contains("Undefined variable: missing"));
    assert!(instructions(&builder).is_empty());
}

#[test]
fn raw_rhs_failure_keeps_old_binding_and_fresh_retry_succeeds() {
    let mut builder = builder("asn0_i0_reuse/0");
    let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
    declare(&mut builder, "x", old, 0);

    let error = builder
        .build_expression(assignment(variable("x"), variable("missing_rhs")))
        .unwrap_err();
    assert!(error.contains("Undefined variable: missing_rhs"));
    assert_eq!(
        builder.function_state.variable_ctx.variable_map.get("x"),
        Some(&old)
    );
    assert!(!instructions(&builder)
        .iter()
        .any(|row| matches!(row, MirInstruction::ReleaseStrong { .. })));

    let result = builder
        .build_expression(assignment(variable("x"), integer(9)))
        .unwrap();
    assert_eq!(
        builder.function_state.variable_ctx.variable_map.get("x"),
        Some(&result)
    );
    assert_eq!(
        instructions(&builder)
            .iter()
            .filter_map(|row| match row {
                MirInstruction::Const {
                    value: ConstValue::Integer(value),
                    ..
                } => Some(*value),
                _ => None,
            })
            .last(),
        Some(9)
    );
}

#[test]
fn field_target_stays_on_field_owner_before_rhs_descent() {
    let mut builder = builder("asn0_i0_field_control/0");
    let field_target = ASTNode::FieldAccess {
        object: Box::new(variable("missing_object")),
        field: "value".to_string(),
        span: Span::unknown(),
    };

    let error = builder
        .build_expression(assignment(field_target, integer(99)))
        .unwrap_err();

    assert!(error.contains("Undefined variable: missing_object"));
    assert!(!instructions(&builder).iter().any(|row| matches!(
        row,
        MirInstruction::Const {
            value: ConstValue::Integer(99),
            ..
        }
    )));
}

#[test]
fn grouped_assignment_remains_on_its_legacy_facade() {
    let mut builder = builder("asn0_i0_grouped_control/0");
    let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
    declare(&mut builder, "x", old, 0);

    let result = builder
        .build_expression(grouped_assignment("x", integer(11)))
        .unwrap();

    assert_eq!(
        builder.function_state.variable_ctx.variable_map.get("x"),
        Some(&result)
    );
    assert_ne!(result, old);
    assert_eq!(
        instructions(&builder)
            .iter()
            .filter(|row| matches!(row, MirInstruction::ReleaseStrong { .. }))
            .count(),
        1
    );
}
