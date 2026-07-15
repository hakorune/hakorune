use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::canonical_direct_call_contract::VerifiedDirectCallEffectV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_control_flow::if_control::verify_resolved_function_if_control_with_direct_call_v1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::error::{TrivialProfileContractErrorV1, TrivialProfileStopReasonV1};
use super::product::{
    TrivialProfileCoverageSubjectV1, TrivialRepresentationV1, TrivialTerminalProfileV1,
    VerifiedTrivialCanonicalOwnerV1,
};
use super::{
    analyze_trivial_canonical_owner_v1, analyze_trivial_canonical_owner_with_direct_call_v1,
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
    let unit = VerifiedResolvedSourceUnitV1::resolve_function_with_root_callable(root).unwrap();
    let input = unit.root_function_input().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let if_control =
        verify_resolved_function_if_control_with_direct_call_v1(input, &completion).unwrap();
    if allow_call {
        analyze_trivial_canonical_owner_with_direct_call_v1(input, &completion, &if_control)
    } else {
        analyze_trivial_canonical_owner_v1(input, &completion, &if_control)
    }
}

fn admitted(root: ASTNode) -> VerifiedTrivialCanonicalOwnerV1 {
    let TrivialCanonicalOwnerAnalysisV1::Admitted(product) = analyze(root, true).unwrap() else {
        panic!("expected disconnected direct-call profile")
    };
    product
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
fn rejects_non_i64_nested_and_second_calls_before_profile_publication() {
    for argument in [
        literal(LiteralValue::Bool(true)),
        literal(LiteralValue::Float(1.5)),
        call(vec![variable("p0")]),
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

    let two_calls = ASTNode::BinaryOp {
        operator: crate::ast::BinaryOperator::Add,
        left: Box::new(call(vec![variable("p0")])),
        right: Box::new(call(vec![variable("p0")])),
        span: Span::unknown(),
    };
    let TrivialCanonicalOwnerAnalysisV1::NotAdmitted(stop) =
        analyze(function(1, two_calls), true).unwrap()
    else {
        panic!("second direct call must stop")
    };
    assert_eq!(
        stop.reason(),
        TrivialProfileStopReasonV1::ExpressionOutsideProfile
    );
}

#[test]
fn call_enabled_entry_requires_exactly_one_row() {
    assert!(matches!(
        analyze(function(1, variable("p0")), true),
        Err(TrivialProfileContractErrorV1::DirectCallCardinality { actual: 0 })
    ));
}

#[test]
fn production_analyzer_remains_call_disabled() {
    let TrivialCanonicalOwnerAnalysisV1::NotAdmitted(stop) =
        analyze(function(1, call(vec![variable("p0")])), false).unwrap()
    else {
        panic!("production profile must remain call-disabled until P0c-I1")
    };
    assert_eq!(
        stop.reason(),
        TrivialProfileStopReasonV1::ExpressionOutsideProfile
    );
}
