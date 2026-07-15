#![cfg(feature = "vm-reference")]

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::backend::{MirInterpreter, VMValue};
use crate::mir::{Callee, MirInstruction};

use super::{MirCompiler, VerifiedResolvedCallableProgramV1};

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

fn call(name: &str) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.to_string(),
        arguments: vec![variable("n")],
        span: Span::unknown(),
    }
}

fn function(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_string(),
        params: vec!["n".to_string()],
        param_decls: vec![ParamDecl {
            name: "n".to_string(),
            declared_type_name: Some("i64".to_string()),
        }],
        return_type_name: Some("i64".to_string()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(value)),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn program(functions: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements: functions,
        span: Span::unknown(),
    }
}

fn valid_program(order: bool) -> ASTNode {
    let caller = function("caller", call("callee"));
    let callee = function(
        "callee",
        binary(BinaryOperator::Add, variable("n"), integer(1)),
    );
    if order {
        program(vec![caller, callee])
    } else {
        program(vec![callee, caller])
    }
}

fn compile(program: ASTNode) -> super::MirCompileResult {
    let source = VerifiedResolvedCallableProgramV1::resolve(program).unwrap();
    MirCompiler::with_options(false)
        .compile_resolved_callable_module(
            source.lowering_input(),
            Some("canonical_sibling_call.hako"),
        )
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
fn exact_sibling_call_is_order_independent_and_executes() {
    let mut observed_symbols = Vec::new();
    for order in [true, false] {
        let result = compile(valid_program(order));
        let caller = &result.module.functions["caller/1"];
        let callee = &result.module.functions["callee/1"];
        let [MirInstruction::Call {
            callee: Some(Callee::Global(target)),
            ..
        }] = calls(caller).as_slice()
        else {
            panic!("caller must contain one exact global call")
        };

        assert_eq!(target, "callee/1");
        assert!(calls(callee).is_empty());
        assert_eq!(
            caller
                .metadata
                .canonical_direct_static_call_capabilities
                .len(),
            1
        );
        assert!(callee
            .metadata
            .canonical_direct_static_call_capabilities
            .is_empty());
        assert!(result
            .module
            .functions
            .values()
            .all(|function| function
                .blocks
                .values()
                .all(
                    |block| block.instructions.iter().all(|instruction| !matches!(
                        instruction,
                        MirInstruction::CopyOwned { .. }
                            | MirInstruction::DestroyOwned { .. }
                            | MirInstruction::ReleaseStrong { .. }
                    ))
                )));
        assert_eq!(
            MirInterpreter::new()
                .execute_function_with_args(&result.module, "caller/1", &[VMValue::Integer(41)],)
                .unwrap(),
            VMValue::Integer(42)
        );
        observed_symbols.push(
            result
                .module
                .function_names()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    assert_eq!(observed_symbols[0], observed_symbols[1]);
}

#[test]
fn activation_rejects_zero_self_or_multiple_edges_without_poisoning_compiler() {
    for rejected in [
        program(vec![
            function("first", variable("n")),
            function("second", variable("n")),
        ]),
        program(vec![
            function("first", call("first")),
            function("second", variable("n")),
        ]),
        program(vec![
            function("first", call("second")),
            function("second", call("first")),
        ]),
    ] {
        let source = VerifiedResolvedCallableProgramV1::resolve(rejected).unwrap();
        let mut compiler = MirCompiler::with_options(false);
        let error = compiler
            .compile_resolved_callable_module(source.lowering_input(), None)
            .unwrap_err()
            .to_string();
        assert!(error.contains("[freeze:contract][canonical_sibling_call/sibling_activation]"));

        let valid = VerifiedResolvedCallableProgramV1::resolve(valid_program(true)).unwrap();
        assert!(compiler
            .compile_resolved_callable_module(valid.lowering_input(), None)
            .is_ok());
    }
}

#[test]
fn sibling_call_keeps_the_vm_only_backend_capability() {
    let result = compile(valid_program(true));
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
