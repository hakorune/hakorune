#![cfg(feature = "vm-reference")]

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::backend::{MirInterpreter, VMValue};
use crate::mir::compiler::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use crate::mir::compiler::{MirCompileResult, VerifiedResolvedSourceUnitV1};
use crate::mir::{ConstValue, MirInstruction, MirType};

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn void() -> ASTNode {
    literal(LiteralValue::Void)
}

fn null() -> ASTNode {
    literal(LiteralValue::Null)
}

fn boolean(value: bool) -> ASTNode {
    literal(LiteralValue::Bool(value))
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
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

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn block_expr(prelude: Vec<ASTNode>, tail: ASTNode) -> ASTNode {
    ASTNode::BlockExpr {
        prelude_stmts: prelude,
        tail_expr: Box::new(tail),
        span: Span::unknown(),
    }
}

fn if_(condition: ASTNode, then_body: Vec<ASTNode>, else_body: Option<Vec<ASTNode>>) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: Span::unknown(),
    }
}

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn returning(name: &str, mut body: Vec<ASTNode>, value: ASTNode) -> ASTNode {
    body.push(ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    });
    function(name, body)
}

fn compile(root: ASTNode) -> MirCompileResult {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    assert!(matches!(
        plan,
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(_)
    ));
    crate::mir::MirCompiler::with_options(false)
        .compile_resolved(unit.lowering_input(), Some("explicit_void.hako"))
        .unwrap()
}

fn execute(root: ASTNode, function_name: &str) -> (MirCompileResult, VMValue) {
    let result = compile(root);
    let value = MirInterpreter::new()
        .execute_function_with_args(&result.module, function_name, &[])
        .unwrap();
    (result, value)
}

fn assert_a_plus(root: ASTNode) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    assert!(matches!(
        plan,
        CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(_)
    ));
}

#[test]
fn explicit_void_blockexpr_and_homogeneous_if_use_void_typed_ssa() {
    let root = returning(
        "void_two_sided",
        vec![
            local(
                "x",
                block_expr(vec![local("inner", void())], variable("inner")),
            ),
            if_(
                boolean(true),
                vec![assignment("x", void())],
                Some(vec![assignment("x", void())]),
            ),
        ],
        variable("x"),
    );
    let (result, value) = execute(root, "void_two_sided/0");
    assert!(matches!(value, VMValue::Void));
    assert!(result.verification_result.is_ok());

    let function = &result.module.functions["void_two_sided/0"];
    let mut void_values = Vec::new();
    let mut phi_values = Vec::new();
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
    {
        match instruction {
            MirInstruction::Const {
                dst,
                value: ConstValue::Void,
            } => void_values.push(*dst),
            MirInstruction::Phi { dst, .. } => phi_values.push(*dst),
            MirInstruction::ReleaseStrong { .. }
            | MirInstruction::CopyOwned { .. }
            | MirInstruction::DestroyOwned { .. } => {
                panic!("ExplicitVoidValue emitted ownership instruction: {instruction:?}")
            }
            _ => {}
        }
    }
    assert!(!void_values.is_empty());
    assert!(!phi_values.is_empty());
    for value in void_values.into_iter().chain(phi_values) {
        assert_eq!(
            function.metadata.value_types.get(&value),
            Some(&MirType::Void)
        );
    }
}

#[test]
fn void_comparison_nested_flow_and_no_value_completion_execute_exactly() {
    let cases = [
        (
            returning(
                "void_equal",
                Vec::new(),
                binary(BinaryOperator::Equal, void(), void()),
            ),
            "void_equal/0",
            VMValue::Bool(true),
        ),
        (
            returning(
                "void_not_equal",
                Vec::new(),
                binary(BinaryOperator::NotEqual, void(), void()),
            ),
            "void_not_equal/0",
            VMValue::Bool(false),
        ),
        (
            returning(
                "void_nested",
                vec![
                    local("x", void()),
                    if_(
                        boolean(true),
                        vec![if_(boolean(false), vec![assignment("x", void())], None)],
                        Some(vec![assignment("x", void())]),
                    ),
                ],
                variable("x"),
            ),
            "void_nested/0",
            VMValue::Void,
        ),
        (
            function(
                "void_empty_return",
                vec![ASTNode::Return {
                    value: None,
                    span: Span::unknown(),
                }],
            ),
            "void_empty_return/0",
            VMValue::Void,
        ),
        (
            function("void_implicit", vec![void()]),
            "void_implicit/0",
            VMValue::Void,
        ),
    ];

    for (root, name, expected) in cases {
        let (_, actual) = execute(root, name);
        assert_eq!(actual, expected);
    }
}

#[test]
fn outbox_null_mixing_void_condition_and_arithmetic_remain_a_plus() {
    assert_a_plus(returning("null_return", Vec::new(), null()));
    assert_a_plus(returning(
        "mixed_void_null",
        vec![
            local("x", void()),
            if_(
                boolean(true),
                vec![assignment("x", null())],
                Some(vec![assignment("x", void())]),
            ),
        ],
        variable("x"),
    ));
    assert_a_plus(function(
        "void_condition",
        vec![if_(void(), Vec::new(), None)],
    ));
    assert_a_plus(returning(
        "void_arithmetic",
        Vec::new(),
        binary(BinaryOperator::Add, void(), void()),
    ));
    assert_a_plus(function(
        "outbox_stays_aplus",
        vec![ASTNode::Outbox {
            variables: vec!["result".into()],
            initial_values: vec![None],
            span: Span::unknown(),
        }],
    ));
}
