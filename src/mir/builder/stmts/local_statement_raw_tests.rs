use crate::ast::{ASTNode, BinaryOperator, FieldDecl, LiteralValue, Span};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::{ConstValue, MirBuilder, MirInstruction};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
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

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn local(
    variables: &[&str],
    initial_values: Vec<Option<Box<ASTNode>>>,
    declared_type_names: Vec<Option<&str>>,
) -> ASTNode {
    ASTNode::Local {
        variables: variables.iter().map(|name| (*name).to_string()).collect(),
        initial_values,
        declared_type_names: declared_type_names
            .into_iter()
            .map(|name| name.map(str::to_string))
            .collect(),
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
        .function_state
        .current_function
        .as_ref()
        .expect("current LCL0-I0 function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

#[test]
fn raw_local_selector_preserves_initializer_order_and_binding_completion() {
    let mut builder = builder("lcl0_raw_order/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    builder
        .build_expression(local(
            &["x", "y"],
            vec![Some(Box::new(integer(4))), Some(Box::new(integer(9)))],
            vec![None, None],
        ))
        .unwrap();

    let constants = instructions(&builder)
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Const {
                value: ConstValue::Integer(value),
                ..
            } => Some(*value),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(constants.starts_with(&[4, 9]), "constants={constants:?}");
    assert!(builder.function_state.binding_ctx.contains("x"));
    assert!(builder.function_state.binding_ctx.contains("y"));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn raw_local_preflight_rejects_before_first_initializer_effect() {
    let mut builder = builder("lcl0_raw_preflight/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    let error = builder
        .build_expression(local(
            &["x", "y"],
            vec![Some(Box::new(integer(4))), None],
            vec![None, Some("Array<u8>")],
        ))
        .unwrap_err();

    assert!(error.contains("local_contract_uninitialized_forbidden"));
    assert!(instructions(&builder).is_empty());
    assert!(!builder.function_state.binding_ctx.contains("x"));
    assert!(!builder.function_state.binding_ctx.contains("y"));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn raw_local_child_failure_stops_later_initializer_and_binding_publication() {
    let mut builder = builder("lcl0_raw_child_failure/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    let error = builder
        .build_expression(local(
            &["x", "y"],
            vec![
                Some(Box::new(variable("missing"))),
                Some(Box::new(integer(91))),
            ],
            vec![None, None],
        ))
        .unwrap_err();

    assert!(error.contains("Undefined variable: missing"));
    assert!(instructions(&builder).is_empty());
    assert!(!builder.function_state.binding_ctx.contains("x"));
    assert!(!builder.function_state.binding_ctx.contains("y"));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn raw_local_initializers_reuse_binary_and_short_circuit_spines() {
    let mut builder = builder("lcl0_raw_expression_spines/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    builder
        .build_expression(local(
            &["sum", "flag"],
            vec![
                Some(Box::new(binary(
                    BinaryOperator::Add,
                    integer(2),
                    integer(3),
                ))),
                Some(Box::new(binary(
                    BinaryOperator::And,
                    boolean(true),
                    boolean(false),
                ))),
            ],
            vec![None, None],
        ))
        .unwrap();

    let rows = instructions(&builder);
    assert!(rows
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::BinOp { .. })));
    assert!(rows
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::Phi { .. })));
    assert!(builder.function_state.binding_ctx.contains("sum"));
    assert!(builder.function_state.binding_ctx.contains("flag"));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn raw_local_typed_array_reuses_specialized_claim_before_appends() {
    let mut builder = builder("lcl0_raw_typed_array/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    builder
        .build_expression(local(
            &["xs"],
            vec![Some(Box::new(ASTNode::ArrayLiteral {
                elements: vec![integer(1), integer(2)],
                span: Span::unknown(),
            }))],
            vec![Some("Array<u8>")],
        ))
        .unwrap();

    let rows = instructions(&builder);
    let claim = rows
        .iter()
        .position(|instruction| {
            matches!(instruction, MirInstruction::ArrayStateContractClaim { .. })
        })
        .expect("typed-array claim");
    let append = rows
        .iter()
        .position(|instruction| matches!(instruction, MirInstruction::ArrayElementWrite { .. }))
        .expect("typed-array append");
    assert!(claim < append);
    assert!(builder.function_state.binding_ctx.contains("xs"));
}

#[test]
fn raw_local_record_initializer_reuses_existing_constructor_owner() {
    let mut builder = builder("lcl0_raw_record/0");
    builder.comp_ctx.register_record_decl(
        "Pair".to_string(),
        Vec::new(),
        &[FieldDecl {
            name: "value".to_string(),
            declared_type_name: None,
            is_weak: false,
            default_value: None,
        }],
    );
    let _scope = LexicalScopeGuard::new(&mut builder);

    builder
        .build_expression(local(
            &["pair"],
            vec![Some(Box::new(ASTNode::New {
                class: "Pair".to_string(),
                arguments: vec![integer(7)],
                type_arguments: Vec::new(),
                field_initializers: Vec::new(),
                span: Span::unknown(),
            }))],
            vec![None],
        ))
        .unwrap();

    assert!(builder.function_state.binding_ctx.contains("pair"));
    assert!(instructions(&builder)
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::RecordValuePublish { .. })));
}

#[test]
fn raw_local_untyped_missing_initializer_keeps_existing_null_sugar() {
    let mut builder = builder("lcl0_raw_null/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    builder
        .build_expression(local(&["x"], vec![None], vec![None]))
        .unwrap();

    assert!(builder.function_state.binding_ctx.contains("x"));
    assert!(instructions(&builder).iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Const {
            value: ConstValue::Null,
            ..
        }
    )));
}
