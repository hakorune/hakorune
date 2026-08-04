//! D2 candidate-abort proof for the selected If recipe shape.
//!
//! The production path already owns a whole unpublished compile candidate.
//! These tests cover the existing no-call envelope plus the selected one-call
//! and two-call RHS candidate-abort proofs; they do not add a fault toggle or
//! transaction owner.

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

fn call_rhs_program(explicit_else: bool) -> ASTNode {
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
                explicit_else.then(|| {
                    vec![ASTNode::Assignment {
                        target: Box::new(variable("x")),
                        value: Box::new(call("right_call_abort_d2", variable("p0"))),
                        span: Span::unknown(),
                    }]
                }),
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
    let right = function(
        "right_call_abort_d2",
        vec![ASTNode::Return {
            value: Some(Box::new(binary(
                BinaryOperator::Add,
                variable("p0"),
                literal(2),
            ))),
            span: Span::unknown(),
        }],
    );
    ASTNode::Program {
        statements: vec![branch, left, right],
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
    let mut candidate =
        super::module_session::CanonicalModuleLoweringSessionV1::open(&compiler.builder);
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
    let source = VerifiedResolvedCallableProgramV1::resolve(call_rhs_program(true))
        .expect("Call-RHS abort module resolves");
    assert_call_candidate_abort_reuses_compiler(source, "call-rhs-reused.hako");
}

#[test]
fn implicit_call_rhs_candidate_discards_after_call_and_phi_seal_failure() {
    let source = VerifiedResolvedCallableProgramV1::resolve(call_rhs_program(false))
        .expect("implicit Call-RHS abort module resolves");
    assert_call_candidate_abort_reuses_compiler(source, "implicit-call-rhs-reused.hako");
}

#[cfg(feature = "vm-reference")]
#[test]
fn explicit_two_call_rhs_preserves_targets_phi_and_runtime_parity() {
    use std::collections::BTreeSet;

    use crate::backend::{MirInterpreter, VMValue};
    use crate::mir::verification::utils::compute_predecessors;
    use crate::mir::{Callee, MirInstruction, MirType};

    let source = VerifiedResolvedCallableProgramV1::resolve(call_rhs_program(true))
        .expect("two-call parity module resolves");
    let result = MirCompiler::with_options(false)
        .compile_resolved_callable_module(source.lowering_input(), Some("two-call-d2.hako"))
        .expect("two-call parity module lowers");
    assert!(result.verification_result.is_ok());

    let function = &result.module.functions["if_call_abort_d2/1"];
    let calls = function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .filter_map(|instruction| match instruction {
            MirInstruction::Call {
                dst: Some(dst),
                callee: Some(Callee::Global(target)),
                ..
            } => Some((target.clone(), *dst)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(calls.len(), 2);
    assert_eq!(
        calls
            .iter()
            .map(|(target, _)| target.as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["left_call_abort_d2/1", "right_call_abort_d2/1"])
    );
    assert!(calls
        .iter()
        .all(|(_, dst)| function.metadata.value_types.get(dst) == Some(&MirType::Integer)));
    assert_eq!(
        function
            .metadata
            .canonical_direct_static_call_capabilities
            .len(),
        1,
        "capability remains one function-level marker"
    );

    let predecessors = compute_predecessors(function);
    let phi_rows = function
        .blocks
        .iter()
        .flat_map(|(block_id, block)| {
            block
                .phi_instructions()
                .map(move |instruction| (*block_id, instruction))
        })
        .collect::<Vec<_>>();
    assert_eq!(phi_rows.len(), 1, "one shared merge binding has one PHI");
    let (merge_block, instruction) = phi_rows[0];
    let MirInstruction::Phi {
        inputs, type_hint, ..
    } = instruction
    else {
        unreachable!("phi_instructions yields only Phi rows")
    };
    assert_eq!(*type_hint, Some(MirType::Integer));
    assert_eq!(inputs.len(), 2);
    let input_predecessors = inputs
        .iter()
        .map(|(predecessor, _)| *predecessor)
        .collect::<BTreeSet<_>>();
    let input_values = inputs
        .iter()
        .map(|(_, value)| *value)
        .collect::<BTreeSet<_>>();
    let actual_predecessors = predecessors
        .get(&merge_block)
        .into_iter()
        .flatten()
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(input_predecessors, actual_predecessors);
    assert_eq!(
        input_values,
        calls.iter().map(|(_, dst)| *dst).collect::<BTreeSet<_>>(),
        "the shared PHI consumes both direct-call results"
    );

    let mut interpreter = MirInterpreter::new();
    assert_eq!(
        interpreter
            .execute_function_with_args(
                &result.module,
                "if_call_abort_d2/1",
                &[VMValue::Integer(1)],
            )
            .unwrap(),
        VMValue::Integer(2)
    );
    assert_eq!(
        interpreter
            .execute_function_with_args(
                &result.module,
                "if_call_abort_d2/1",
                &[VMValue::Integer(-1)],
            )
            .unwrap(),
        VMValue::Integer(1)
    );
}
