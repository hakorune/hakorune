use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::canonical_direct_call_contract::VerifiedDirectCallEffectV1;
use crate::mir::compiler::{VerifiedResolvedCallableProgramV1, VerifiedResolvedSourceUnitV1};
use crate::mir::resolved_control_flow::if_control::verify_resolved_function_if_control_with_direct_call_v1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::error::{TrivialProfileContractErrorV1, TrivialProfileStopReasonV1};
use super::product::{
    TrivialProfileCoverageSubjectV1, TrivialRepresentationV1, TrivialTerminalProfileV1,
    VerifiedTrivialCanonicalOwnerV1,
};
use super::{
    analyze_trivial_canonical_with_mode_v1, TrivialCanonicalAnalysisModeV1,
    TrivialCanonicalOwnerAnalysisV1, TrivialProfileConsumptionV1,
};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn call(arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionCall {
        name: "countdown".into(),
        arguments,
        span: Span::unknown(),
    }
}

fn function(parameter_count: usize, result: ASTNode) -> ASTNode {
    let params = (0..parameter_count)
        .map(|index| format!("p{index}"))
        .collect::<Vec<_>>();
    let param_decls = params
        .iter()
        .map(|name| ParamDecl {
            name: name.clone(),
            declared_type_name: Some("i64".into()),
        })
        .collect();
    ASTNode::FunctionDeclaration {
        name: "countdown".into(),
        params,
        param_decls,
        return_type_name: Some("i64".into()),
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

fn analyze(
    root: ASTNode,
    allow_call: bool,
) -> Result<TrivialCanonicalOwnerAnalysisV1, TrivialProfileContractErrorV1> {
    if allow_call {
        let source = VerifiedResolvedCallableProgramV1::resolve(ASTNode::Program {
            statements: vec![root],
            span: Span::unknown(),
        })
        .unwrap();
        let key = source.module().functions_by_key().keys().next().unwrap();
        let input = source.module().function_input(key).unwrap();
        let completion = verify_function_completion_v1(input).unwrap();
        let if_control =
            verify_resolved_function_if_control_with_direct_call_v1(input, &completion).unwrap();
        analyze_trivial_canonical_with_mode_v1(
            input,
            &completion,
            &if_control,
            TrivialCanonicalAnalysisModeV1::OrdinaryFiniteDirectCalls,
        )
    } else {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
        let input = unit.root_function_input().unwrap();
        let completion = verify_function_completion_v1(input).unwrap();
        let if_control =
            verify_resolved_function_if_control_with_direct_call_v1(input, &completion).unwrap();
        analyze_trivial_canonical_with_mode_v1(
            input,
            &completion,
            &if_control,
            TrivialCanonicalAnalysisModeV1::OrdinaryClosed,
        )
    }
}

fn admitted(root: ASTNode) -> VerifiedTrivialCanonicalOwnerV1 {
    let TrivialCanonicalOwnerAnalysisV1::Admitted(product) = analyze(root, true).unwrap() else {
        panic!("expected disconnected direct-call profile")
    };
    product
}

fn admitted_finite(root: ASTNode) -> VerifiedTrivialCanonicalOwnerV1 {
    admitted(root)
}

#[test]
fn co_seals_target_ordered_i64_arguments_result_effect_and_coverage() {
    let product = admitted(function(2, call(vec![variable("p0"), variable("p1")])));
    let [row] = product.direct_calls() else {
        panic!("expected exactly one direct-call row")
    };
    assert_eq!(row.target().callable().owner(), product.owner());
    assert_eq!(row.target().symbol().as_mir_name(), "countdown/2");
    assert_eq!(row.target().signature().arity(), 2);
    assert_eq!(row.arguments().len(), 2);
    assert!(matches!(
        row.arguments()[0].node().segments().last(),
        Some(SourcePathSegmentV1::Argument(0))
    ));
    assert!(matches!(
        row.arguments()[1].node().segments().last(),
        Some(SourcePathSegmentV1::Argument(1))
    ));
    assert_eq!(row.result(), TrivialRepresentationV1::InlineI64);
    assert_eq!(
        row.effect(),
        VerifiedDirectCallEffectV1::ConservativeBarrier
    );
    assert_eq!(
        product.representation_at(row.site()),
        Some(TrivialRepresentationV1::InlineI64)
    );

    let subjects = product.coverage().ordered_subjects();
    assert_eq!(
        subjects
            .iter()
            .filter(|subject| matches!(subject, TrivialProfileCoverageSubjectV1::DirectCall(_)))
            .count(),
        1
    );
    assert!(!subjects.iter().any(|subject| matches!(
        subject,
        TrivialProfileCoverageSubjectV1::Value(site) if site == row.site()
    )));
}

#[test]
fn consumption_claims_arguments_then_one_whole_direct_call_row() {
    let product = admitted(function(1, call(vec![variable("p0")])));
    let row = product.direct_calls()[0].clone();
    let argument = row.arguments()[0].clone();
    let (statement, value) = match product.terminal() {
        TrivialTerminalProfileV1::ExplicitValue {
            statement, value, ..
        } => (statement.clone(), value.clone()),
        _ => panic!("expected explicit value terminal"),
    };

    let mut wrong_order =
        TrivialProfileConsumptionV1::new(admitted(function(1, call(vec![variable("p0")]))));
    assert!(wrong_order.claim_direct_call(row.site()).is_err());

    let mut ledger = TrivialProfileConsumptionV1::new(product);
    ledger.claim_parameter_entry(0).unwrap();
    ledger.claim_value(&argument).unwrap();
    assert_eq!(ledger.claim_direct_call(row.site()).unwrap(), row);
    assert!(ledger.claim_direct_call(row.site()).is_err());
    ledger
        .claim_terminal_explicit_value(&statement, &value)
        .unwrap();
    ledger.finish().unwrap();
}

#[test]
fn rejects_non_i64_arguments_before_profile_publication() {
    for argument in [
        literal(LiteralValue::Bool(true)),
        literal(LiteralValue::Float(1.5)),
    ] {
        let TrivialCanonicalOwnerAnalysisV1::NotAdmitted(stop) =
            analyze(function(1, call(vec![argument])), true).unwrap()
        else {
            panic!("unsupported direct-call argument must stop")
        };
        assert!(matches!(
            stop.reason(),
            TrivialProfileStopReasonV1::BinaryOperandsNotExact
                | TrivialProfileStopReasonV1::ExpressionOutsideProfile
        ));
    }
}

#[test]
fn production_analyzer_remains_call_disabled() {
    let TrivialCanonicalOwnerAnalysisV1::NotAdmitted(stop) =
        analyze(function(1, call(vec![variable("p0")])), false).unwrap()
    else {
        panic!("body-only profile must remain call-disabled")
    };
    assert_eq!(
        stop.reason(),
        TrivialProfileStopReasonV1::ExpressionOutsideProfile
    );
}

#[test]
fn finite_profile_records_sequential_calls() {
    let two_calls = ASTNode::BinaryOp {
        operator: crate::ast::BinaryOperator::Add,
        left: Box::new(call(vec![variable("p0")])),
        right: Box::new(call(vec![variable("p0")])),
        span: Span::unknown(),
    };
    let product = admitted_finite(function(1, two_calls));
    assert_eq!(product.direct_calls().len(), 2);
}

#[test]
fn finite_profile_records_nested_call_child_before_parent() {
    let product = admitted_finite(function(1, call(vec![call(vec![variable("p0")])])));
    let [inner, outer] = product.direct_calls() else {
        panic!("expected inner and outer call rows")
    };
    assert_eq!(outer.arguments(), [inner.site().clone()]);
    assert!(matches!(
        inner.site().node().segments().last(),
        Some(SourcePathSegmentV1::Argument(0))
    ));
}
