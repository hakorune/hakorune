//! Disconnected P0c-F-DX0a finite direct-call preflight fixtures.

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, ParamDecl, Span};

use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::{CanonicalLoweringErrorV1, VerifiedResolvedCallableProgramV1};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn call(name: &str, argument: ASTNode) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.to_string(),
        arguments: vec![argument],
        span: Span::unknown(),
    }
}

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn function(name: &str, result: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_string(),
        params: vec!["n".to_string()],
        param_decls: vec![ParamDecl {
            name: "n".to_string(),
            declared_type_name: Some("i64".to_string()),
        }],
        return_type_name: Some("i64".to_string()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(result)),
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

fn input_for<'a>(
    source: &'a VerifiedResolvedCallableProgramV1,
    name: &str,
) -> ResolvedFunctionLoweringInputV1<'a> {
    let key = source
        .module()
        .functions_by_key()
        .keys()
        .find(|key| key.name() == name)
        .unwrap();
    source.module().function_input(key).unwrap()
}

fn finite_profile(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> crate::mir::resolved_value_profile::product::VerifiedTrivialCanonicalOwnerV1 {
    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) =
        CanonicalLoweringPreflightV1::verify_function_with_finite_direct_calls_v1(input).unwrap()
    else {
        panic!("finite exact calls must select the trivial Binding-SSA plan")
    };
    let (_, _, _, profile, _) = plan.into_parts();
    profile
}

#[test]
fn finite_preflight_accepts_multiple_targets_but_exact_one_stays_closed() {
    let caller = function(
        "caller",
        add(call("left", variable("n")), call("right", variable("n"))),
    );
    let source = VerifiedResolvedCallableProgramV1::resolve(program(vec![
        caller,
        function("left", variable("n")),
        function("right", variable("n")),
    ]))
    .unwrap();
    let input = input_for(&source, "caller");

    assert!(matches!(
        CanonicalLoweringPreflightV1::verify_function(input),
        Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
            reason: "direct_call_cardinality_not_activated",
            ..
        })
    ));

    let profile = finite_profile(input);
    let symbols = profile
        .direct_calls()
        .iter()
        .map(|row| row.target().symbol().as_mir_name())
        .collect::<Vec<_>>();
    assert_eq!(symbols, ["left/1", "right/1"]);
}

#[test]
fn finite_preflight_accepts_nested_calls_in_child_before_parent_order() {
    let caller = function("caller", call("step", call("step", variable("n"))));
    let source = VerifiedResolvedCallableProgramV1::resolve(program(vec![
        caller,
        function("step", variable("n")),
    ]))
    .unwrap();
    let input = input_for(&source, "caller");

    assert!(CanonicalLoweringPreflightV1::verify_function(input).is_err());
    let profile = finite_profile(input);
    let [inner, outer] = profile.direct_calls() else {
        panic!("expected nested inner and outer rows")
    };
    assert_eq!(inner.target().symbol().as_mir_name(), "step/1");
    assert_eq!(outer.target().symbol().as_mir_name(), "step/1");
    assert_eq!(outer.arguments(), [inner.site().clone()]);
}
