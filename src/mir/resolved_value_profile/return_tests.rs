use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::compiler::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::resolved_control_flow::if_control::verify_resolved_function_if_control_v1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;

use super::error::TrivialProfileStopReasonV1;
use super::product::{TrivialRepresentationV1, TrivialTerminalProfileV1};
use super::{analyze_trivial_canonical_owner_v1, TrivialCanonicalOwnerAnalysisV1};

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
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
        name: "return_profile_fixture".to_string(),
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

fn analyze(root: ASTNode) -> TrivialCanonicalOwnerAnalysisV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let input = unit.root_function_input().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let if_control = verify_resolved_function_if_control_v1(input, &completion).unwrap();
    analyze_trivial_canonical_owner_v1(input, &completion, &if_control).unwrap()
}

#[test]
fn exact_i64_return_witness_co_seals_with_existing_terminal() {
    let root = function(
        &[],
        Some("i64"),
        vec![return_(Some(literal(LiteralValue::Integer(42))))],
    );
    let TrivialCanonicalOwnerAnalysisV1::Admitted(product) = analyze(root) else {
        panic!("expected disconnected exact i64 return witness")
    };
    assert_eq!(
        product.function_return().unwrap().abi(),
        ExactTrivialReturnAbiV1::I64
    );
    assert!(matches!(
        product.terminal(),
        TrivialTerminalProfileV1::ExplicitValue {
            representation: TrivialRepresentationV1::InlineI64,
            ..
        }
    ));
}

#[test]
fn exact_i64_parameter_and_return_share_one_profile() {
    let root = function(
        &[Some("i64")],
        Some("i64"),
        vec![return_(Some(variable("p0")))],
    );
    let TrivialCanonicalOwnerAnalysisV1::Admitted(product) = analyze(root) else {
        panic!("expected co-sealed parameter and return profile")
    };
    assert_eq!(product.parameter_entries().len(), 1);
    assert!(product.function_return().is_some());
}

#[test]
fn typed_return_requires_exact_spelling_and_inline_i64_terminal() {
    for spelling in ["int", "Integer", "I64", " i64", "i64 "] {
        let root = function(
            &[],
            Some(spelling),
            vec![return_(Some(literal(LiteralValue::Integer(1))))],
        );
        let TrivialCanonicalOwnerAnalysisV1::NotAdmitted(stop) = analyze(root) else {
            panic!("expected non-exact return spelling to stop")
        };
        assert_eq!(
            stop.reason(),
            TrivialProfileStopReasonV1::TypedSignatureOutsideProfile
        );
    }
    for (body, expected_reason) in [
        (
            vec![return_(Some(literal(LiteralValue::Bool(true))))],
            TrivialProfileStopReasonV1::TypedSignatureOutsideProfile,
        ),
        (
            vec![return_(Some(literal(LiteralValue::Float(1.0))))],
            TrivialProfileStopReasonV1::TypedSignatureOutsideProfile,
        ),
        (
            vec![return_(Some(literal(LiteralValue::String("value".into()))))],
            TrivialProfileStopReasonV1::StringRepresentationUnavailable,
        ),
    ] {
        let root = function(&[], Some("i64"), body);
        let TrivialCanonicalOwnerAnalysisV1::NotAdmitted(stop) = analyze(root) else {
            panic!("expected non-i64 terminal to stop")
        };
        assert_eq!(stop.reason(), expected_reason);
    }
}

#[test]
fn exact_i64_return_selects_production_binding_ssa_route() {
    let root = function(
        &[],
        Some("i64"),
        vec![return_(Some(literal(LiteralValue::Integer(42))))],
    );
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    assert!(matches!(
        CanonicalLoweringPreflightV1::verify(&unit).unwrap(),
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(_)
    ));
}
