#![cfg(feature = "vm-reference")]

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::backend::{MirInterpreter, VMValue};
use crate::mir::compiler::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use crate::mir::compiler::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use crate::mir::{MirInstruction, MirType, ValueId};

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

fn return_(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}

fn typed_function(
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
        .compile_resolved(unit.lowering_input(), Some("parameter_entry.hako"))
        .unwrap()
}

#[test]
fn exact_i64_parameter_adopts_reserved_value_and_executes() {
    let root = typed_function(
        "parameter_plus_one",
        &[Some("i64")],
        None,
        vec![return_(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable("p0")),
            right: Box::new(integer(1)),
            span: Span::unknown(),
        })],
    );
    let result = compile(root);
    assert!(result.verification_result.is_ok());
    let function = &result.module.functions["parameter_plus_one/1"];

    assert_eq!(function.signature.params, vec![MirType::Integer]);
    assert_eq!(function.params, vec![ValueId::new(0)]);
    assert_eq!(function.metadata.declared_param_decls.len(), 1);
    assert_eq!(
        function.metadata.declared_param_decls[0]
            .declared_type_name
            .as_deref(),
        Some("i64")
    );
    assert_eq!(function.metadata.parameter_entry_contracts.len(), 1);
    assert_eq!(
        function.metadata.parameter_entry_contracts[0].parameter_value_id,
        ValueId::new(0)
    );
    for value in function
        .blocks
        .values()
        .flat_map(|block| block.defined_values())
    {
        assert!(
            value.0 >= 1,
            "fresh definition overlaps parameter: {value:?}"
        );
    }
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
    {
        assert!(!matches!(
            instruction,
            MirInstruction::ReleaseStrong { .. }
                | MirInstruction::CopyOwned { .. }
                | MirInstruction::DestroyOwned { .. }
        ));
    }

    let value = MirInterpreter::new()
        .execute_function_with_args(
            &result.module,
            "parameter_plus_one/1",
            &[VMValue::Integer(41)],
        )
        .unwrap();
    assert_eq!(value, VMValue::Integer(42));
}

#[test]
fn parameter_assignment_and_if_use_the_same_binding_ssa_owner() {
    let root = typed_function(
        "parameter_if",
        &[Some("i64"), Some("i64")],
        None,
        vec![
            ASTNode::If {
                condition: Box::new(boolean(true)),
                then_body: vec![assignment("p0", variable("p1"))],
                else_body: Some(vec![assignment("p0", integer(7))]),
                span: Span::unknown(),
            },
            return_(variable("p0")),
        ],
    );
    let result = compile(root);
    let value = MirInterpreter::new()
        .execute_function_with_args(
            &result.module,
            "parameter_if/2",
            &[VMValue::Integer(1), VMValue::Integer(9)],
        )
        .unwrap();
    assert_eq!(value, VMValue::Integer(9));
    assert!(result.module.functions["parameter_if/2"]
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .any(|instruction| matches!(instruction, MirInstruction::Phi { .. })));
}

#[test]
fn final_callee_rejects_wrong_type_and_arity_before_execution() {
    let result = compile(typed_function(
        "parameter_identity",
        &[Some("i64")],
        None,
        vec![return_(variable("p0"))],
    ));
    let wrong_type = MirInterpreter::new()
        .execute_function_with_args(
            &result.module,
            "parameter_identity/1",
            &[VMValue::String("bad".to_string())],
        )
        .unwrap_err()
        .to_string();
    assert!(wrong_type.contains("[type/parameter_contract_violation]"));
    for args in [Vec::new(), vec![VMValue::Integer(1), VMValue::Integer(2)]] {
        let error = MirInterpreter::new()
            .execute_function_with_args(&result.module, "parameter_identity/1", &args)
            .unwrap_err()
            .to_string();
        assert!(error.contains("[type/parameter_arity_mismatch]"), "{error}");
    }
}

#[test]
fn unsupported_typed_parameter_shapes_fail_before_lowering() {
    for parameter_types in [
        vec![Some("bool")],
        vec![Some("f64")],
        vec![Some("usize")],
        vec![Some("i64"), None],
    ] {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(typed_function(
            "unsupported_parameter",
            &parameter_types,
            None,
            vec![return_(integer(1))],
        ))
        .unwrap();
        assert!(matches!(
            CanonicalLoweringPreflightV1::verify(&unit),
            Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
                reason: "typed_parameter_profile_not_activated",
                ..
            })
        ));
    }

    let typed_return = VerifiedResolvedSourceUnitV1::resolve_function(typed_function(
        "typed_return",
        &[Some("i64")],
        Some("i64"),
        vec![return_(variable("p0"))],
    ))
    .unwrap();
    assert!(matches!(
        CanonicalLoweringPreflightV1::verify(&typed_return),
        Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
            reason: "typed_signature_not_activated",
            ..
        })
    ));
}

#[test]
fn contracted_parameter_module_keeps_non_vm_backends_fail_fast() {
    let result = compile(typed_function(
        "parameter_backend",
        &[Some("i64")],
        None,
        vec![return_(variable("p0"))],
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
            error.contains("[type/backend_parameter_contract_capability_missing]"),
            "{error}"
        );
    }
}
