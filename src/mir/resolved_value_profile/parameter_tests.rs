use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::compiler::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_control_flow::if_control::verify_resolved_function_if_control_v1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::SourceBindingSiteV1;

use super::error::TrivialProfileStopReasonV1;
use super::product::{
    TrivialProfileCoverageSubjectV1, TrivialRepresentationV1, TrivialTerminalProfileV1,
    VerifiedTrivialCanonicalOwnerV1,
};
use super::{
    analyze_trivial_canonical_owner_v1, TrivialCanonicalOwnerAnalysisV1,
    TrivialProfileConsumptionV1,
};

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

fn if_(then_body: Vec<ASTNode>, else_body: Vec<ASTNode>) -> ASTNode {
    ASTNode::If {
        condition: Box::new(boolean(true)),
        then_body,
        else_body: Some(else_body),
        span: Span::unknown(),
    }
}

fn typed_function(parameter_types: &[Option<&str>], body: Vec<ASTNode>) -> ASTNode {
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
        name: "parameter_profile_fixture".to_string(),
        params,
        param_decls,
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn return_value(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
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

fn admitted(root: ASTNode) -> VerifiedTrivialCanonicalOwnerV1 {
    let TrivialCanonicalOwnerAnalysisV1::Admitted(product) = analyze(root) else {
        panic!("expected exact i64 parameter profile")
    };
    product
}

#[test]
fn exact_i64_parameters_are_sealed_before_body_subjects() {
    let product = admitted(typed_function(
        &[Some("i64"), Some("i64")],
        vec![return_value(variable("p0"))],
    ));

    let [first, second] = product.parameter_entries() else {
        panic!("expected two exact parameter entries")
    };
    assert_eq!(first.formal_index(), 0);
    assert_eq!(first.site(), &SourceBindingSiteV1::Parameter { index: 0 });
    assert_eq!(first.source_name(), "p0");
    assert_eq!(first.declared_type_name(), "i64");
    assert_eq!(first.representation(), TrivialRepresentationV1::InlineI64);
    assert_eq!(
        first
            .abi()
            .mir_param_decl(first.source_name())
            .declared_type_name
            .as_deref(),
        Some("i64")
    );
    assert_eq!(second.formal_index(), 1);
    assert_eq!(second.source_name(), "p1");

    let subjects = product.coverage().ordered_subjects();
    assert!(matches!(
        subjects.first(),
        Some(TrivialProfileCoverageSubjectV1::Definition {
            origin: super::product::TrivialBindingDefinitionOriginV1::Declaration(
                SourceBindingSiteV1::Parameter { index: 0 }
            ),
            ..
        })
    ));
    assert!(matches!(
        subjects.get(1),
        Some(TrivialProfileCoverageSubjectV1::Definition {
            origin: super::product::TrivialBindingDefinitionOriginV1::Declaration(
                SourceBindingSiteV1::Parameter { index: 1 }
            ),
            ..
        })
    ));
    assert!(matches!(
        product.terminal(),
        TrivialTerminalProfileV1::ExplicitValue {
            representation: TrivialRepresentationV1::InlineI64,
            ..
        }
    ));
}

#[test]
fn parameter_profile_consumption_uses_one_global_ordered_ledger() {
    let root = typed_function(&[Some("i64")], vec![return_value(variable("p0"))]);
    let mut wrong_order = TrivialProfileConsumptionV1::new(admitted(root.clone()));
    assert!(wrong_order.claim_parameter_entry(1).is_err());

    let mut duplicate = TrivialProfileConsumptionV1::new(admitted(root.clone()));
    duplicate.claim_parameter_entry(0).unwrap();
    assert!(duplicate.claim_parameter_entry(0).is_err());

    let product = admitted(root);
    let (statement, value) = match product.terminal() {
        TrivialTerminalProfileV1::ExplicitValue {
            statement, value, ..
        } => (statement.clone(), value.clone()),
        _ => panic!("expected explicit value terminal"),
    };
    let mut ledger = TrivialProfileConsumptionV1::new(product);
    assert_eq!(ledger.parameter_entry_count(), 1);
    ledger.claim_parameter_entry(0).unwrap();
    assert_eq!(
        ledger.claim_value(&value).unwrap(),
        TrivialRepresentationV1::InlineI64
    );
    ledger
        .claim_terminal_explicit_value(&statement, &value)
        .unwrap();
    ledger.finish().unwrap();
}

#[test]
fn parameter_rebind_and_if_merge_reuse_the_existing_profile_environment() {
    let product = admitted(typed_function(
        &[Some("i64")],
        vec![
            assignment("p0", integer(1)),
            if_(
                vec![assignment("p0", integer(2))],
                vec![assignment("p0", integer(3))],
            ),
            return_value(variable("p0")),
        ],
    ));

    assert_eq!(product.parameter_entries().len(), 1);
    assert_eq!(product.merge_profiles().len(), 1);
    assert_eq!(
        product.merge_profiles()[0].representation(),
        TrivialRepresentationV1::InlineI64
    );
    assert!(product
        .definitions()
        .iter()
        .all(|row| row.representation() == TrivialRepresentationV1::InlineI64));
}

#[test]
fn unsupported_parameter_types_and_untyped_parameters_do_not_admit() {
    for declared in [
        None,
        Some("bool"),
        Some("f64"),
        Some("usize"),
        Some("String"),
    ] {
        let TrivialCanonicalOwnerAnalysisV1::NotAdmitted(stop) =
            analyze(typed_function(&[declared], vec![return_value(integer(1))]))
        else {
            panic!("unsupported parameter representation must not admit")
        };
        let expected = if declared.is_none() {
            TrivialProfileStopReasonV1::ParameterRepresentationUnavailable
        } else {
            TrivialProfileStopReasonV1::TypedSignatureOutsideProfile
        };
        assert_eq!(stop.reason(), expected);
    }
}

#[test]
fn exact_parameter_profile_selects_production_binding_ssa_route() {
    let root = typed_function(&[Some("i64")], vec![return_value(variable("p0"))]);
    assert!(matches!(
        analyze(root.clone()),
        TrivialCanonicalOwnerAnalysisV1::Admitted(_)
    ));
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    assert!(matches!(
        CanonicalLoweringPreflightV1::verify(&unit).unwrap(),
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(_)
    ));
}
