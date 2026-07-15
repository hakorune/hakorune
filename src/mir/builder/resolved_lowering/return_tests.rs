#![cfg(feature = "vm-reference")]

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::backend::{MirInterpreter, VMValue};
use crate::mir::compiler::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use crate::mir::compiler::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use crate::mir::function::{ReturnExitContractKind, ReturnExitContractOwner, ReturnExitVoidPolicy};
use crate::mir::{ConstValue, MirInstruction, MirType, ValueId};

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

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
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

fn return_(value: Option<ASTNode>) -> ASTNode {
    ASTNode::Return {
        value: value.map(Box::new),
        span: Span::unknown(),
    }
}

fn function(
    name: &str,
    parameter_types: &[Option<&str>],
    return_type_name: Option<&str>,
    body: Vec<ASTNode>,
) -> ASTNode {
    let params = parameter_types
        .iter()
        .enumerate()
        .map(|(index, _)| format!("p{index}"))
        .collect::<Vec<_>>();
    let param_decls = params
        .iter()
        .zip(parameter_types)
        .map(|(name, declared_type_name)| ParamDecl {
            name: name.clone(),
            declared_type_name: declared_type_name.map(str::to_string),
        })
        .collect();
    ASTNode::FunctionDeclaration {
        name: name.to_string(),
        params,
        param_decls,
        return_type_name: return_type_name.map(str::to_string),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn compile(root: ASTNode) -> crate::mir::compiler::MirCompileResult {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    assert!(matches!(
        CanonicalLoweringPreflightV1::verify(&unit).unwrap(),
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(_)
    ));
    crate::mir::MirCompiler::with_options(false)
        .compile_resolved(unit.lowering_input(), Some("exact_i64_return.hako"))
        .unwrap()
}

#[test]
fn exact_i64_return_installs_one_verified_callable_boundary() {
    let result = compile(function(
        "answer",
        &[],
        Some("i64"),
        vec![return_(Some(integer(42)))],
    ));
    let function = &result.module.functions["answer/0"];
    assert_eq!(function.signature.return_type, MirType::Integer);
    assert_eq!(
        function.metadata.declared_return_type_name.as_deref(),
        Some("i64")
    );
    let contract = function.metadata.return_exit_contract.as_ref().unwrap();
    assert_eq!(contract.contract_kind, ReturnExitContractKind::ExactNumeric);
    assert_eq!(
        contract.owner,
        ReturnExitContractOwner::FunctionReturnContract
    );
    assert_eq!(contract.void_policy, ReturnExitVoidPolicy::RejectVoid);
    assert!(contract.runtime_check_required);
    assert!(result.verification_result.is_ok());
    assert_eq!(
        MirInterpreter::new()
            .execute_function_with_args(&result.module, "answer/0", &[])
            .unwrap(),
        VMValue::Integer(42)
    );
}

#[test]
fn exact_i64_parameter_phi_and_return_share_binding_ssa() {
    let result = compile(function(
        "choose",
        &[Some("i64"), Some("i64")],
        Some("i64"),
        vec![
            ASTNode::If {
                condition: Box::new(boolean(true)),
                then_body: vec![assignment("p0", variable("p1"))],
                else_body: Some(vec![assignment("p0", integer(7))]),
                span: Span::unknown(),
            },
            return_(Some(variable("p0"))),
        ],
    ));
    let function = &result.module.functions["choose/2"];
    let phi_dst = function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .find_map(|instruction| match instruction {
            MirInstruction::Phi { dst, .. } => Some(*dst),
            _ => None,
        })
        .expect("Binding SSA must define the joined parameter value");
    let return_value = function
        .blocks
        .values()
        .find_map(|block| match block.terminator.as_ref() {
            Some(MirInstruction::Return { value: Some(value) }) => Some(*value),
            _ => None,
        })
        .expect("typed function must return one value");
    assert_eq!(return_value, phi_dst, "blocks={:?}", function.blocks);
    assert_eq!(function.metadata.parameter_entry_contracts.len(), 2);
    assert!(function.metadata.return_exit_contract.is_some());
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
    assert_eq!(
        MirInterpreter::new()
            .execute_function_with_args(
                &result.module,
                "choose/2",
                &[VMValue::Integer(1), VMValue::Integer(9)],
            )
            .unwrap(),
        VMValue::Integer(9)
    );
}

#[test]
fn unsupported_typed_returns_fail_before_builder_and_never_retry() {
    for root in [
        function(
            "spelling",
            &[],
            Some("Integer"),
            vec![return_(Some(integer(1)))],
        ),
        function(
            "wrong_value",
            &[],
            Some("i64"),
            vec![return_(Some(boolean(true)))],
        ),
        function("empty", &[], Some("i64"), vec![return_(None)]),
        function("implicit", &[], Some("i64"), Vec::new()),
    ] {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
        assert!(matches!(
            CanonicalLoweringPreflightV1::verify(&unit),
            Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
                reason: "typed_return_profile_not_activated",
                ..
            })
        ));
        let mut compiler = crate::mir::MirCompiler::with_options(false);
        assert!(matches!(
            compiler.compile_resolved(unit.lowering_input(), None),
            Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
                reason: "typed_return_profile_not_activated",
                ..
            })
        ));
    }
}

#[test]
fn contracted_return_keeps_non_vm_backends_fail_fast() {
    let result = compile(function(
        "backend",
        &[],
        Some("i64"),
        vec![return_(Some(integer(1)))],
    ));
    assert!(
        crate::mir::backend_capability::enforce_mir_backend_supported(
            &result.module,
            "mir-interpreter"
        )
        .is_ok()
    );
    for backend in ["pyvm-harness", "ny-llvmc-exe", "llvmlite-obj", "wasm"] {
        let error =
            crate::mir::backend_capability::enforce_mir_backend_supported(&result.module, backend)
                .unwrap_err();
        assert!(
            error.contains("[type/backend_return_contract_capability_missing]"),
            "{error}"
        );
    }
}

#[test]
fn runtime_contract_rejects_wrong_final_value_before_observation() {
    let mut result = compile(function(
        "runtime_mismatch",
        &[],
        Some("i64"),
        vec![return_(Some(integer(1)))],
    ));
    let function = result
        .module
        .functions
        .get_mut("runtime_mismatch/0")
        .unwrap();
    let constant = function
        .blocks
        .values_mut()
        .flat_map(|block| &mut block.instructions)
        .find(|instruction| matches!(instruction, MirInstruction::Const { .. }))
        .unwrap();
    let MirInstruction::Const { dst, value } = constant else {
        unreachable!()
    };
    assert_ne!(*dst, ValueId::INVALID);
    *value = ConstValue::String("bad".to_string());
    let error = MirInterpreter::new()
        .execute_function_with_args(&result.module, "runtime_mismatch/0", &[])
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("[type/return_contract_violation]"),
        "{error}"
    );
}
