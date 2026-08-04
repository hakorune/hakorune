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
        params: vec!["x".into()],
        param_decls: vec![ParamDecl {
            name: "x".into(),
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

fn program(functions: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements: functions,
        span: Span::unknown(),
    }
}

fn compile(functions: Vec<ASTNode>) -> super::MirCompileResult {
    let source = VerifiedResolvedCallableProgramV1::resolve(program(functions)).unwrap();
    MirCompiler::with_options(false)
        .compile_resolved_callable_module(source.lowering_input(), Some("p0c_f_i1.hako"))
        .unwrap()
}

fn call_count(function: &crate::mir::MirFunction) -> usize {
    function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
        .count()
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
fn nested_repeated_multi_target_and_multi_hop_calls_execute_in_any_declaration_order() {
    let step = || {
        returning(
            "step",
            binary(BinaryOperator::Add, variable("x"), integer(1)),
        )
    };
    let twice = || returning("twice", call("step", call("step", variable("x"))));
    let root = || {
        returning(
            "root",
            binary(
                BinaryOperator::Add,
                call("twice", variable("x")),
                call("step", variable("x")),
            ),
        )
    };

    let mut observed = Vec::new();
    for functions in [vec![root(), twice(), step()], vec![step(), root(), twice()]] {
        let result = compile(functions);
        assert_eq!(execute(&result, "root", 1), VMValue::Integer(5));
        assert_eq!(call_count(&result.module.functions["root/1"]), 2);
        assert_eq!(call_count(&result.module.functions["twice/1"]), 2);
        assert_eq!(call_count(&result.module.functions["step/1"]), 0);
        assert_eq!(
            result.module.functions["root/1"]
                .metadata
                .canonical_direct_static_call_capabilities
                .len(),
            1
        );
        assert_eq!(
            result.module.functions["twice/1"]
                .metadata
                .canonical_direct_static_call_capabilities
                .len(),
            1
        );
        assert!(result.module.functions["step/1"]
            .metadata
            .canonical_direct_static_call_capabilities
            .is_empty());
        observed.push(
            result
                .module
                .function_names()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    assert_eq!(observed[0], observed[1]);
}

#[test]
fn calls_in_both_fallthrough_if_arms_execute() {
    let branch = function(
        "branch",
        vec![
            local("result", variable("x")),
            ASTNode::If {
                condition: Box::new(binary(BinaryOperator::Greater, variable("x"), integer(0))),
                then_body: vec![assignment("result", call("left", variable("x")))],
                else_body: Some(vec![assignment("result", call("right", variable("x")))]),
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(variable("result"))),
                span: Span::unknown(),
            },
        ],
    );
    let result = compile(vec![
        branch,
        returning(
            "left",
            binary(BinaryOperator::Add, variable("x"), integer(1)),
        ),
        returning(
            "right",
            binary(BinaryOperator::Add, variable("x"), integer(2)),
        ),
    ]);
    assert_eq!(execute(&result, "branch", 1), VMValue::Integer(2));
    assert_eq!(execute(&result, "branch", -1), VMValue::Integer(1));
}

#[test]
fn one_call_rhs_branch_preserves_call_phi_and_parity() {
    let branch = function(
        "branch_call_rhs",
        vec![
            local("result", variable("x")),
            ASTNode::If {
                condition: Box::new(binary(BinaryOperator::Greater, variable("x"), integer(0))),
                then_body: vec![assignment("result", call("left", variable("x")))],
                else_body: Some(vec![assignment(
                    "result",
                    binary(BinaryOperator::Add, variable("x"), integer(2)),
                )]),
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(variable("result"))),
                span: Span::unknown(),
            },
        ],
    );
    let result = compile(vec![
        branch,
        returning(
            "left",
            binary(BinaryOperator::Add, variable("x"), integer(1)),
        ),
    ]);
    let branch_function = &result.module.functions["branch_call_rhs/1"];
    let calls = branch_function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .filter_map(|instruction| match instruction {
            MirInstruction::Call {
                dst: Some(dst),
                callee: Some(crate::mir::Callee::Global(target)),
                args,
                ..
            } => Some((*dst, target.as_str(), args.as_slice())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(calls.len(), 1);
    let (call_result, target, call_args) = calls[0];
    assert_eq!(target, "left/1");
    assert_eq!(call_args.len(), 1);
    assert_eq!(
        branch_function
            .metadata
            .canonical_direct_static_call_capabilities
            .len(),
        1
    );

    let phis = branch_function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            MirInstruction::Phi { dst, inputs, .. } => Some((*dst, inputs.clone())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(phis.len(), 1);
    let (_phi, inputs) = &phis[0];
    assert_eq!(inputs.len(), 2);
    assert!(inputs.iter().any(|(_, value)| *value == call_result));
    let merge = branch_function
        .blocks
        .values()
        .find(|block| {
            block.instructions.iter().any(|instruction| {
                matches!(instruction, MirInstruction::Phi { dst, .. } if *dst == phis[0].0)
            })
        })
        .expect("Call-RHS merge block");
    let input_blocks = inputs
        .iter()
        .map(|(block, _)| *block)
        .collect::<std::collections::BTreeSet<_>>();
    let predecessors = merge
        .predecessors
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(input_blocks, predecessors);

    assert_eq!(execute(&result, "branch_call_rhs", 1), VMValue::Integer(2));
    assert_eq!(execute(&result, "branch_call_rhs", -1), VMValue::Integer(1));
}

#[test]
fn zero_call_and_recursive_graphs_reject_without_poisoning_the_compiler() {
    for rejected in [
        vec![returning("a", variable("x")), returning("b", variable("x"))],
        vec![
            returning("a", call("b", variable("x"))),
            returning("b", call("a", variable("x"))),
        ],
    ] {
        let source = VerifiedResolvedCallableProgramV1::resolve(program(rejected)).unwrap();
        let mut compiler = MirCompiler::with_options(false);
        let error = compiler
            .compile_resolved_callable_module(source.lowering_input(), None)
            .unwrap_err()
            .to_string();
        assert!(error.contains("[freeze:contract][canonical_callable_module/acyclic_activation]"));

        let valid = VerifiedResolvedCallableProgramV1::resolve(program(vec![
            returning("caller", call("callee", variable("x"))),
            returning("callee", variable("x")),
        ]))
        .unwrap();
        assert!(compiler
            .compile_resolved_callable_module(valid.lowering_input(), None)
            .is_ok());
    }
}

#[test]
fn acyclic_module_keeps_vm_only_and_zero_ownership_operations() {
    let result = compile(vec![
        returning("caller", call("callee", variable("x"))),
        returning("callee", variable("x")),
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
    assert!(
        crate::mir::canonical_direct_static_call_backend_capability::enforce(
            &result.module,
            "mir-interpreter"
        )
        .is_ok()
    );
    let error = crate::mir::canonical_direct_static_call_backend_capability::enforce(
        &result.module,
        "wasm",
    )
    .unwrap_err();
    assert!(error.contains("[backend/canonical_direct_static_call_v1_unsupported]"));
    assert!(error.contains("silent_fallback_allowed=false"));
}
