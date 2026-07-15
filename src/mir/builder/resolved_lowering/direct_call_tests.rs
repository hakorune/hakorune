#![cfg(feature = "vm-reference")]

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::backend::{MirInterpreter, VMValue};
use crate::mir::compiler::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use crate::mir::compiler::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use crate::mir::{Callee, Effect, MirInstruction, ValueId};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
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

fn call(name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.to_string(),
        arguments,
        span: Span::unknown(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
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

fn return_(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}

fn function(name: &str, parameters: &[&str], body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_string(),
        params: parameters.iter().map(|name| (*name).to_string()).collect(),
        param_decls: parameters
            .iter()
            .map(|name| ParamDecl {
                name: (*name).to_string(),
                declared_type_name: Some("i64".to_string()),
            })
            .collect(),
        return_type_name: Some("i64".to_string()),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn countdown(name: &str) -> ASTNode {
    function(
        name,
        &["n"],
        vec![
            local("result", variable("n")),
            ASTNode::If {
                condition: Box::new(binary(BinaryOperator::Greater, variable("n"), integer(0))),
                then_body: vec![assignment(
                    "result",
                    call(
                        name,
                        vec![binary(BinaryOperator::Subtract, variable("n"), integer(1))],
                    ),
                )],
                else_body: None,
                span: Span::unknown(),
            },
            return_(variable("result")),
        ],
    )
}

fn compile(root: ASTNode) -> crate::mir::compiler::MirCompileResult {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function_with_root_callable(root).unwrap();
    assert!(matches!(
        CanonicalLoweringPreflightV1::verify(&unit).unwrap(),
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(_)
    ));
    crate::mir::MirCompiler::with_options(false)
        .compile_resolved(unit.lowering_input(), Some("canonical_self_call.hako"))
        .unwrap()
}

fn calls(function: &crate::mir::MirFunction) -> Vec<&MirInstruction> {
    function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
        .collect()
}

#[test]
fn exact_self_call_executes_recursively_with_one_sealed_capability() {
    let result = compile(countdown("countdown"));
    let function = &result.module.functions["countdown/1"];
    assert!(result.verification_result.is_ok());
    assert_eq!(calls(function).len(), 1);
    let MirInstruction::Call {
        dst: Some(dst),
        func,
        callee,
        args,
        effects,
    } = calls(function)[0]
    else {
        unreachable!()
    };
    assert_ne!(*dst, ValueId::INVALID);
    assert_eq!(*func, ValueId::INVALID);
    assert_eq!(callee, &Some(Callee::Global("countdown/1".to_string())));
    assert_eq!(args.len(), 1);
    assert!(effects.contains(Effect::Barrier));
    assert!(!effects.contains(Effect::Pure));
    assert_eq!(
        function
            .metadata
            .canonical_direct_static_call_capabilities
            .len(),
        1
    );
    assert!(function
        .blocks
        .values()
        .all(
            |block| block.instructions.iter().all(|instruction| !matches!(
                instruction,
                MirInstruction::CopyOwned { .. }
                    | MirInstruction::DestroyOwned { .. }
                    | MirInstruction::ReleaseStrong { .. }
            ))
        ));

    for (input, expected) in [(0, 0), (1, 0), (6, 0)] {
        let value = MirInterpreter::new()
            .execute_function_with_args(&result.module, "countdown/1", &[VMValue::Integer(input)])
            .unwrap();
        assert_eq!(value, VMValue::Integer(expected));
    }
    let wrong_type = MirInterpreter::new()
        .execute_function_with_args(
            &result.module,
            "countdown/1",
            &[VMValue::String("bad".to_string())],
        )
        .unwrap_err()
        .to_string();
    assert!(wrong_type.contains("[type/parameter_contract_violation]"));
}

#[test]
fn post_if_phi_argument_and_local_call_result_use_binding_ssa() {
    let name = "countdown_phi";
    let root = function(
        name,
        &["n"],
        vec![
            local("result", variable("n")),
            ASTNode::If {
                condition: Box::new(binary(BinaryOperator::Greater, variable("n"), integer(0))),
                then_body: vec![
                    local("next", variable("n")),
                    ASTNode::If {
                        condition: Box::new(binary(
                            BinaryOperator::Greater,
                            variable("n"),
                            integer(1),
                        )),
                        then_body: vec![assignment(
                            "next",
                            binary(BinaryOperator::Subtract, variable("n"), integer(1)),
                        )],
                        else_body: Some(vec![assignment("next", integer(0))]),
                        span: Span::unknown(),
                    },
                    local("recursive", call(name, vec![variable("next")])),
                    assignment("result", variable("recursive")),
                ],
                else_body: None,
                span: Span::unknown(),
            },
            return_(variable("result")),
        ],
    );
    let result = compile(root);
    let function = &result.module.functions["countdown_phi/1"];
    let phi_values = function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .filter_map(|instruction| match instruction {
            MirInstruction::Phi { dst, .. } => Some(*dst),
            _ => None,
        })
        .collect::<Vec<_>>();
    let MirInstruction::Call { args, .. } = calls(function)[0] else {
        unreachable!()
    };
    assert!(phi_values.contains(&args[0]));
    assert_eq!(
        MirInterpreter::new()
            .execute_function_with_args(&result.module, "countdown_phi/1", &[VMValue::Integer(5)],)
            .unwrap(),
        VMValue::Integer(0)
    );
}

#[test]
fn call_result_can_be_the_final_return_without_a_fresh_return_value() {
    let result = compile(function(
        "return_self",
        &["n"],
        vec![return_(call("return_self", vec![variable("n")]))],
    ));
    let function = &result.module.functions["return_self/1"];
    let call_dst = match calls(function)[0] {
        MirInstruction::Call { dst: Some(dst), .. } => *dst,
        _ => unreachable!(),
    };
    let returned = function
        .blocks
        .values()
        .find_map(|block| match block.terminator.as_ref() {
            Some(MirInstruction::Return { value: Some(value) }) => Some(*value),
            _ => None,
        })
        .unwrap();
    assert_eq!(returned, call_dst);
}

#[test]
fn unsupported_call_shapes_reject_before_the_builder_session() {
    for root in [
        function(
            "two_calls",
            &["n"],
            vec![
                local("a", call("two_calls", vec![variable("n")])),
                return_(call("two_calls", vec![variable("a")])),
            ],
        ),
        function(
            "nested_call",
            &["n"],
            vec![return_(call(
                "nested_call",
                vec![call("nested_call", vec![variable("n")])],
            ))],
        ),
    ] {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function_with_root_callable(root).unwrap();
        assert!(matches!(
            CanonicalLoweringPreflightV1::verify(&unit),
            Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
                reason: "direct_call_cardinality_not_activated",
                ..
            })
        ));
        let mut compiler = crate::mir::MirCompiler::with_options(false);
        assert!(compiler
            .compile_resolved(unit.lowering_input(), None)
            .is_err());
        let good = VerifiedResolvedSourceUnitV1::resolve_function_with_root_callable(countdown(
            "after_reject",
        ))
        .unwrap();
        assert!(compiler
            .compile_resolved(good.lowering_input(), None)
            .is_ok());
    }

    let zero = function(
        "zero_parameter",
        &[],
        vec![return_(call("zero_parameter", Vec::new()))],
    );
    assert!(matches!(
        VerifiedResolvedSourceUnitV1::resolve_function_with_root_callable(zero),
        Err(CanonicalLoweringErrorV1::SourceUnitResolution { .. })
    ));
}

#[test]
fn source_resolution_and_backend_capability_never_fallback() {
    for root in [
        function(
            "expected",
            &["n"],
            vec![return_(call("other", vec![variable("n")]))],
        ),
        function(
            "expected",
            &["n"],
            vec![return_(call("expected/1", vec![variable("n")]))],
        ),
        function(
            "expected",
            &["n"],
            vec![return_(call("expected", Vec::new()))],
        ),
    ] {
        assert!(matches!(
            VerifiedResolvedSourceUnitV1::resolve_function_with_root_callable(root),
            Err(CanonicalLoweringErrorV1::SourceUnitResolution { .. })
        ));
    }

    let body_only = VerifiedResolvedSourceUnitV1::resolve_function(countdown("body_only")).unwrap();
    assert!(matches!(
        CanonicalLoweringPreflightV1::verify(&body_only),
        Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
            reason: "expression_not_in_first_family",
            ..
        })
    ));

    let result = compile(countdown("backend_gate"));
    assert!(
        crate::mir::canonical_direct_static_call_backend_capability::enforce(
            &result.module,
            "mir-interpreter"
        )
        .is_ok()
    );
    for backend in [
        "pyvm-harness",
        "ny-llvmc-exe",
        "llvmlite-obj",
        "wasm",
        "wasm-v2",
    ] {
        let error = crate::mir::canonical_direct_static_call_backend_capability::enforce(
            &result.module,
            backend,
        )
        .unwrap_err();
        assert!(error.contains("[backend/canonical_direct_static_call_v1_unsupported]"));
        assert!(error.contains("silent_fallback_allowed=false"));
    }
}
