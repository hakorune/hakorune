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

fn null() -> ASTNode {
    literal(LiteralValue::Null)
}

fn void() -> ASTNode {
    literal(LiteralValue::Void)
}

fn boolean(value: bool) -> ASTNode {
    literal(LiteralValue::Bool(value))
}

fn integer(value: i64) -> ASTNode {
    literal(LiteralValue::Integer(value))
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn equal(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
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

fn function(name: &str, mut body: Vec<ASTNode>, result: ASTNode) -> ASTNode {
    body.push(ASTNode::Return {
        value: Some(Box::new(result)),
        span: Span::unknown(),
    });
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

fn compile(root: ASTNode) -> MirCompileResult {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    assert!(matches!(
        plan,
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(_)
    ));
    crate::mir::MirCompiler::with_options(false)
        .compile_resolved(unit.lowering_input(), Some("null_sentinel.hako"))
        .unwrap()
}

fn execute_bool(root: ASTNode, function_name: &str) -> (MirCompileResult, bool) {
    let result = compile(root);
    let value = MirInterpreter::new()
        .execute_function_with_args(&result.module, function_name, &[])
        .unwrap();
    let VMValue::Bool(value) = value else {
        panic!("expected Bool result, got {value:?}")
    };
    (result, value)
}

#[test]
fn null_blockexpr_and_homogeneous_if_execute_without_ownership_ops() {
    let root = function(
        "null_two_sided",
        vec![
            local(
                "x",
                block_expr(vec![local("inner", null())], variable("inner")),
            ),
            if_(
                boolean(true),
                vec![assignment("x", null())],
                Some(vec![assignment("x", null())]),
            ),
        ],
        equal(variable("x"), null()),
    );
    let (result, value) = execute_bool(root, "null_two_sided/0");
    assert!(value);
    assert!(result.verification_result.is_ok());

    let function = &result.module.functions["null_two_sided/0"];
    let mut null_values = Vec::new();
    let mut phi_values = Vec::new();
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
    {
        match instruction {
            MirInstruction::Const {
                dst,
                value: ConstValue::Null,
            } => null_values.push(*dst),
            MirInstruction::Phi { dst, .. } => phi_values.push(*dst),
            MirInstruction::ReleaseStrong { .. }
            | MirInstruction::CopyOwned { .. }
            | MirInstruction::DestroyOwned { .. } => {
                panic!("NullSentinel route emitted ownership instruction: {instruction:?}")
            }
            _ => {}
        }
    }
    assert!(!null_values.is_empty());
    assert!(!phi_values.is_empty());
    for value in null_values.into_iter().chain(phi_values) {
        assert_eq!(
            function.metadata.value_types.get(&value),
            Some(&MirType::Void)
        );
    }
}

#[test]
fn one_sided_and_nested_null_flows_remain_binding_ssa() {
    let cases = [
        function(
            "null_one_sided",
            vec![
                local("x", null()),
                if_(boolean(true), vec![assignment("x", null())], None),
            ],
            equal(variable("x"), null()),
        ),
        function(
            "null_nested",
            vec![
                local("x", null()),
                if_(
                    boolean(true),
                    vec![if_(
                        boolean(false),
                        vec![assignment("x", null())],
                        Some(vec![assignment("x", null())]),
                    )],
                    Some(vec![assignment("x", null())]),
                ),
            ],
            equal(variable("x"), null()),
        ),
    ];

    for (root, name) in cases.into_iter().zip(["null_one_sided/0", "null_nested/0"]) {
        let (_, value) = execute_bool(root, name);
        assert!(value);
    }
}

#[test]
fn null_return_void_outbox_and_mixed_merge_stay_whole_unit_a_plus() {
    let fixtures = [
        function("null_return", Vec::new(), null()),
        function("void_return", Vec::new(), void()),
        function(
            "mixed_null",
            vec![
                local("x", null()),
                if_(
                    boolean(true),
                    vec![assignment("x", integer(1))],
                    Some(vec![assignment("x", null())]),
                ),
            ],
            equal(variable("x"), null()),
        ),
        function(
            "outbox_stays_aplus",
            vec![ASTNode::Outbox {
                variables: vec!["result".into()],
                initial_values: vec![None],
                span: Span::unknown(),
            }],
            boolean(true),
        ),
    ];

    for fixture in fixtures {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(fixture).unwrap();
        let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();
        assert!(matches!(
            plan,
            CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(_)
        ));
    }
}
