#![cfg(feature = "vm-reference")]

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::backend::{MirInterpreter, VMValue};
use crate::mir::MirInstruction;

use super::{MirCompiler, VerifiedResolvedCallableProgramV1};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
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

fn call(name: &str, argument: ASTNode) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.into(),
        arguments: vec![argument],
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

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: vec!["n".into()],
        param_decls: vec![ParamDecl {
            name: "n".into(),
            declared_type_name: Some("i64".into()),
        }],
        return_type_name: Some("i64".into()),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn returning(name: &str, value: ASTNode) -> ASTNode {
    function(
        name,
        vec![ASTNode::Return {
            value: Some(Box::new(value)),
            span: Span::unknown(),
        }],
    )
}

fn recursive_branch(name: &str, base: i64, target: &str) -> ASTNode {
    function(
        name,
        vec![
            local("result", integer(base)),
            ASTNode::If {
                condition: Box::new(binary(BinaryOperator::NotEqual, variable("n"), integer(0))),
                then_body: vec![assignment(
                    "result",
                    call(
                        target,
                        binary(BinaryOperator::Subtract, variable("n"), integer(1)),
                    ),
                )],
                else_body: None,
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(variable("result"))),
                span: Span::unknown(),
            },
        ],
    )
}

fn program(functions: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements: functions,
        span: Span::unknown(),
    }
}

fn compile(functions: Vec<ASTNode>) -> super::MirCompileResult {
    let source = VerifiedResolvedCallableProgramV1::resolve(program(functions)).unwrap();
    MirCompiler::with_options(false)
        .compile_resolved_recursive_callable_module(source.lowering_input(), Some("p0c_mr_i1.hako"))
        .unwrap()
}

fn execute(result: &super::MirCompileResult, name: &str, input: i64) -> VMValue {
    MirInterpreter::new()
        .execute_function_with_args(
            &result.module,
            &format!("{name}/1"),
            &[VMValue::Integer(input)],
        )
        .unwrap()
}

#[test]
fn singleton_program_self_recursion_uses_program_authority_and_both_markers() {
    let result = compile(vec![recursive_branch("countdown", 0, "countdown")]);
    assert_eq!(execute(&result, "countdown", 0), VMValue::Integer(0));
    assert_eq!(execute(&result, "countdown", 7), VMValue::Integer(0));

    let function = &result.module.functions["countdown/1"];
    assert_eq!(
        function
            .metadata
            .canonical_direct_static_call_capabilities
            .len(),
        1
    );
    assert!(result
        .module
        .metadata
        .canonical_recursive_callable_module_capability
        .is_some());
}

#[test]
fn singleton_program_accepts_finite_repeated_and_nested_self_calls() {
    let repeated = returning(
        "repeated",
        binary(
            BinaryOperator::Add,
            call("repeated", variable("n")),
            call("repeated", variable("n")),
        ),
    );
    let nested = returning("nested", call("nested", call("nested", variable("n"))));

    for (function, symbol, expected_calls) in [(repeated, "repeated/1", 2), (nested, "nested/1", 2)]
    {
        let result = compile(vec![function]);
        let call_count = result.module.functions[symbol]
            .blocks
            .values()
            .flat_map(|block| &block.instructions)
            .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
            .count();
        assert_eq!(call_count, expected_calls);
    }
}

#[test]
fn even_odd_and_declaration_reorder_execute_with_one_module_marker() {
    let mut observed = Vec::new();
    for functions in [
        vec![
            recursive_branch("even", 1, "odd"),
            recursive_branch("odd", 0, "even"),
        ],
        vec![
            recursive_branch("odd", 0, "even"),
            recursive_branch("even", 1, "odd"),
        ],
    ] {
        let result = compile(functions);
        observed.push((execute(&result, "even", 8), execute(&result, "odd", 8)));
        assert!(result
            .module
            .metadata
            .canonical_recursive_callable_module_capability
            .is_some());
    }
    assert_eq!(observed[0], (VMValue::Integer(1), VMValue::Integer(0)));
    assert_eq!(observed[0], observed[1]);
}

#[test]
fn three_function_scc_and_outer_dag_caller_execute() {
    let result = compile(vec![
        returning(
            "outer",
            binary(BinaryOperator::Add, call("a", variable("n")), integer(5)),
        ),
        recursive_branch("a", 10, "b"),
        recursive_branch("b", 20, "c"),
        recursive_branch("c", 30, "a"),
    ]);
    assert_eq!(execute(&result, "outer", 3), VMValue::Integer(15));
    assert_eq!(execute(&result, "outer", 4), VMValue::Integer(25));
}

#[test]
fn recursive_ingress_rejects_acyclic_input_without_poisoning_compiler() {
    let acyclic = VerifiedResolvedCallableProgramV1::resolve(program(vec![
        returning("a", call("b", variable("n"))),
        returning("b", variable("n")),
    ]))
    .unwrap();
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_resolved_recursive_callable_module(acyclic.lowering_input(), None)
        .unwrap_err()
        .to_string();
    assert!(error.contains("[freeze:contract][canonical_callable_module/recursive_activation]"));

    let recursive = VerifiedResolvedCallableProgramV1::resolve(program(vec![
        recursive_branch("even", 1, "odd"),
        recursive_branch("odd", 0, "even"),
    ]))
    .unwrap();
    assert!(compiler
        .compile_resolved_recursive_callable_module(recursive.lowering_input(), None)
        .is_ok());
}

#[test]
fn recursive_module_is_vm_only_and_emits_zero_ownership_operations() {
    let result = compile(vec![
        recursive_branch("even", 1, "odd"),
        recursive_branch("odd", 0, "even"),
    ]);
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
    crate::mir::backend_capability::enforce_mir_backend_supported(
        &result.module,
        "mir-interpreter",
    )
    .unwrap();
    let aggregate_error =
        crate::mir::backend_capability::enforce_mir_backend_supported(&result.module, "wasm")
            .unwrap_err();
    assert!(
        aggregate_error.contains("backend=wasm"),
        "{aggregate_error}"
    );
    let error = crate::mir::canonical_recursive_callable_module_backend_capability::enforce(
        &result.module,
        "wasm",
    )
    .unwrap_err();
    assert!(
        error.contains("[backend/canonical_recursive_callable_module_v1_unsupported]"),
        "{error}"
    );
    assert!(error.contains("silent_fallback_allowed=false"));
}

fn recursive_module_with_safe_leaf() -> super::MirCompileResult {
    compile(vec![
        recursive_branch("even", 1, "odd"),
        recursive_branch("odd", 0, "even"),
        returning("safe", variable("n")),
    ])
}

fn assert_reusable_after_error(interpreter: &mut MirInterpreter, module: &crate::mir::MirModule) {
    assert_eq!(
        interpreter
            .execute_function_with_args(module, "safe/1", &[VMValue::Integer(77)])
            .unwrap(),
        VMValue::Integer(77)
    );
}

#[test]
fn max_call_depth_restores_frames_and_interpreter_reuse() {
    let result = compile(vec![
        returning("a", call("b", variable("n"))),
        returning("b", call("a", variable("n"))),
        returning("safe", variable("n")),
    ]);
    let mut interpreter = MirInterpreter::new();
    let error = interpreter
        .execute_function_with_args(&result.module, "a/1", &[VMValue::Integer(1)])
        .unwrap_err()
        .to_string();
    assert!(error.contains("vm call stack depth exceeded"), "{error}");
    assert_reusable_after_error(&mut interpreter, &result.module);
}

#[test]
fn inner_parameter_contract_failure_restores_frames_and_interpreter_reuse() {
    let mut result = recursive_module_with_safe_leaf();
    let function = result.module.functions.get_mut("even/1").unwrap();
    let instruction = function
        .blocks
        .values_mut()
        .flat_map(|block| block.instructions.iter_mut())
        .find(|instruction| {
            matches!(
                instruction,
                MirInstruction::BinOp {
                    op: crate::mir::BinaryOp::Sub,
                    ..
                }
            )
        })
        .unwrap();
    let MirInstruction::BinOp { dst, .. } = instruction else {
        unreachable!()
    };
    *instruction = MirInstruction::Const {
        dst: *dst,
        value: crate::mir::ConstValue::String("bad-argument".to_string()),
    };

    let mut interpreter = MirInterpreter::new();
    let error = interpreter
        .execute_function_with_args(&result.module, "even/1", &[VMValue::Integer(1)])
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("[type/parameter_contract_violation]"),
        "{error}"
    );
    assert_reusable_after_error(&mut interpreter, &result.module);
}

#[test]
fn inner_return_contract_failure_restores_frames_and_interpreter_reuse() {
    let mut result = recursive_module_with_safe_leaf();
    let function = result.module.functions.get_mut("odd/1").unwrap();
    let instruction = function
        .blocks
        .values_mut()
        .flat_map(|block| block.instructions.iter_mut())
        .find(|instruction| {
            matches!(
                instruction,
                MirInstruction::Const {
                    value: crate::mir::ConstValue::Integer(0),
                    ..
                }
            )
        })
        .unwrap();
    let MirInstruction::Const { dst, .. } = instruction else {
        unreachable!()
    };
    *instruction = MirInstruction::Const {
        dst: *dst,
        value: crate::mir::ConstValue::String("bad-return".to_string()),
    };

    let mut interpreter = MirInterpreter::new();
    let error = interpreter
        .execute_function_with_args(&result.module, "even/1", &[VMValue::Integer(1)])
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("[type/return_contract_violation]"),
        "{error}"
    );
    assert_reusable_after_error(&mut interpreter, &result.module);
}
