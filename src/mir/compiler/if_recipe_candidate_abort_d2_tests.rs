//! D2 candidate-abort proof for the selected If recipe shape.
//!
//! The production path already owns a whole unpublished compile candidate.
//! These tests cover the existing no-call envelope plus the selected Call-RHS
//! candidate-abort proof; they do not add a fault toggle or transaction owner.

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};

use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::{MirCompiler, VerifiedResolvedCallableProgramV1, VerifiedResolvedSourceUnitV1};

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn bool_literal(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_owned()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(literal(value)),
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

fn call(name: &str, argument: ASTNode) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.to_owned(),
        arguments: vec![argument],
        span: Span::unknown(),
    }
}

fn if_stmt(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: Span::unknown(),
    }
}

fn function(else_body: Option<Vec<ASTNode>>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "if_abort_d2".to_owned(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            local("x", literal(0)),
            if_stmt(bool_literal(true), vec![assignment("x", 1)], else_body),
            ASTNode::Return {
                value: Some(Box::new(variable("x"))),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn call_rhs_program() -> ASTNode {
    fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.to_owned(),
            params: vec!["p0".to_owned()],
            param_decls: vec![ParamDecl {
                name: "p0".to_owned(),
                declared_type_name: Some("i64".to_owned()),
            }],
            return_type_name: Some("i64".to_owned()),
            body,
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    let branch = function(
        "if_call_abort_d2",
        vec![
            local("x", literal(0)),
            if_stmt(
                binary(BinaryOperator::Greater, variable("p0"), literal(0)),
                vec![ASTNode::Assignment {
                    target: Box::new(variable("x")),
                    value: Box::new(call("left_call_abort_d2", variable("p0"))),
                    span: Span::unknown(),
                }],
                Some(vec![ASTNode::Assignment {
                    target: Box::new(variable("x")),
                    value: Box::new(binary(
                        BinaryOperator::Add,
                        variable("p0"),
                        literal(2),
                    )),
                    span: Span::unknown(),
                }]),
            ),
            ASTNode::Return {
                value: Some(Box::new(variable("x"))),
                span: Span::unknown(),
            },
        ],
    );
    let left = function(
        "left_call_abort_d2",
        vec![ASTNode::Return {
            value: Some(Box::new(binary(
                BinaryOperator::Add,
                variable("p0"),
                literal(1),
            ))),
            span: Span::unknown(),
        }],
    );
    ASTNode::Program {
        statements: vec![branch, left],
        span: Span::unknown(),
    }
}

fn assert_candidate_abort_reuses_compiler(unit: VerifiedResolvedSourceUnitV1, source_file: &str) {
    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.set_source_file_hint("before.hako");
    compiler.builder.next_value_id();
    compiler.builder.next_block_id();
    let before = compiler.builder.loop_candidate_test_fingerprint();

    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) =
        CanonicalLoweringPreflightV1::verify(&unit).expect("trivial If plan")
    else {
        panic!("If fixture must select TrivialBindingSsa");
    };
    let mut candidate =
        super::module_session::CanonicalModuleLoweringSessionV1::open(&compiler.builder);
    let error = candidate
        .builder_mut()
        .lower_resolved_trivial_function_draft_with_seal_failure_for_test(plan)
        .expect_err("late draft-seal failure must reject after selected If lowering");
    assert!(matches!(
        error,
        super::CanonicalResolvedBuildErrorV1::BuilderContract(detail)
            if detail.contains("DraftSeal") || detail.contains("draft_seal")
    ));
    drop(candidate);

    assert_eq!(compiler.builder.loop_candidate_test_fingerprint(), before);
    assert!(compiler.builder.current_module.is_none());
    assert!(compiler.builder.current_function_name().is_none());
    assert!(compiler.builder.current_function_entry_block().is_none());

    let result = compiler
        .compile_resolved(unit.lowering_input(), Some(source_file))
        .expect("same compiler must accept a fresh If request");
    assert!(result.verification_result.is_ok());
    assert!(result.module.functions.contains_key("if_abort_d2/0"));
}

fn assert_call_candidate_abort_reuses_compiler(
    source: VerifiedResolvedCallableProgramV1,
    source_file: &str,
) {
    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.set_source_file_hint("before.hako");
    compiler.builder.next_value_id();
    compiler.builder.next_block_id();
    let before = compiler.builder.loop_candidate_test_fingerprint();

    let key = source
        .module()
        .functions_by_key()
        .keys()
        .find(|key| key.name() == "if_call_abort_d2")
        .expect("Call-RHS branch key")
        .clone();
    let input = source
        .module()
        .function_input(&key)
        .expect("Call-RHS function input");
    let plan = CanonicalLoweringPreflightV1::verify_function_with_finite_direct_calls_v1(input)
        .expect("finite direct-call If plan");
    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) = plan else {
        panic!("Call-RHS fixture must select TrivialBindingSsa")
    };
    let mut candidate = super::module_session::CanonicalModuleLoweringSessionV1::open(
        &compiler.builder,
    );
    let error = candidate
        .builder_mut()
        .lower_resolved_trivial_function_draft_with_seal_failure_for_test(plan)
        .expect_err("late Call-RHS draft-seal failure must reject after call and PHI work");
    assert!(matches!(
        error,
        super::CanonicalResolvedBuildErrorV1::BuilderContract(detail)
            if detail.contains("DraftSeal") || detail.contains("draft_seal")
    ));
    drop(candidate);

    assert_eq!(compiler.builder.loop_candidate_test_fingerprint(), before);
    assert!(compiler.builder.current_module.is_none());
    assert!(compiler.builder.current_function_name().is_none());
    assert!(compiler.builder.current_function_entry_block().is_none());

    let result = compiler
        .compile_resolved_callable_module(source.lowering_input(), Some(source_file))
        .expect("same compiler must accept a fresh Call-RHS module");
    assert!(result.module.functions.contains_key("if_call_abort_d2/1"));
}

#[test]
fn implicit_if_candidate_discards_after_late_draft_seal_failure() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(None))
        .expect("implicit If fixture resolves");
    assert_candidate_abort_reuses_compiler(unit, "implicit-if-reused.hako");
}

#[test]
fn call_rhs_candidate_discards_after_call_and_phi_seal_failure() {
    let source = VerifiedResolvedCallableProgramV1::resolve(call_rhs_program())
        .expect("Call-RHS abort module resolves");
    assert_call_candidate_abort_reuses_compiler(source, "call-rhs-reused.hako");
}
