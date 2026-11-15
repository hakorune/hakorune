use crate::mir::builder::{
    NormalCallableSemanticAdmissionV1, VerifiedNormalCallableSemanticSourceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::mir::source_call_target::{
    DynamicMemberSourceIssueV1, DynamicMemberSourceRejectV1, VerifiedSourceCallTargetCatalogV1,
    VerifiedStaticImportAliasViewV1,
};
use crate::parser::NyashParser;
use super::*;

fn seal_source<'source>(
    program: &'source crate::ast::ASTNode,
    catalog: &VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedNormalCallableSemanticSourceV1<'source> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let NormalCallableSemanticAdmissionV1::Complete(source) =
        VerifiedNormalCallableSemanticSourceV1::seal(
            program,
            catalog.selected_source_inventory(),
            false,
            &mut resolver,
        )
        .unwrap()
    else {
        panic!("semantic source fixture must be complete")
    };
    source
}

fn empty_targets<'catalog>(
    catalog: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedSourceCallTargetCatalogV1<'catalog> {
    let imports = VerifiedStaticImportAliasViewV1::seal(catalog, std::iter::empty()).unwrap();
    VerifiedSourceCallTargetCatalogV1::seal_qualified(&imports, std::iter::empty()).unwrap()
}


#[test]
fn language_wide_dynamic_contract_preserves_effect_home_and_lifecycle() {
    let contract = dynamic_invocation_execution_envelope_v1();
    assert_eq!(
        contract.effect(),
        DynamicInvocationEffectV1::OpaqueObservable
    );
    assert_eq!(
        contract.ordering(),
        DynamicInvocationOrderingV1::SynchronousNonDetached
    );
    assert_eq!(
        contract.suspension(),
        DynamicInvocationSuspensionV1::MaySuspend
    );
    assert_eq!(
        contract.outcome(),
        DynamicInvocationOutcomeV1::NormalSelfContainedDynamicCarrierOrFault
    );
    assert_eq!(
        contract.control(),
        DynamicInvocationControlV1::CallableBounded
    );
    assert_eq!(
        contract.input_home(),
        DynamicInvocationInputHomeV1::BorrowedNoEscapeForInvocation
    );
    assert_eq!(
        contract.result_lifecycle(),
        crate::mir::dynamic_carrier_contract::DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded
    );
}





#[test]
fn duplicate_dynamic_source_cannot_reach_a_second_envelope_row() {
    let program = NyashParser::parse_from_string(
        "static box Calls { run(src) { return src.substring(0, 1) } }",
    )
    .unwrap();
    let declarations =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let source = seal_source(&program, &declarations);
    let targets = empty_targets(&declarations)
        .extend_complete_dynamic_sources(&source)
        .unwrap();

    assert!(matches!(
        targets.extend_complete_dynamic_sources(&source),
        Err(DynamicMemberSourceIssueV1::Rejected(
            DynamicMemberSourceRejectV1::DuplicateOrCollidingTarget { .. }
        ))
    ));
}

#[test]
fn carrier_lifecycle_vocabulary_has_one_neutral_owner() {
    let invocation_model = include_str!("model.rs");
    let carrier_model = include_str!("../dynamic_carrier_contract/model.rs");

    let retired_name = concat!("DynamicInvocationResult", "LifecycleV1");
    assert!(!invocation_model.contains(retired_name));
    assert_eq!(
        carrier_model
            .matches("enum DynamicCarrierLifecycleObligationV1")
            .count(),
        1
    );
    assert_eq!(
        carrier_model
            .matches("EndExactlyOnceUnlessForwarded")
            .count(),
        1
    );
    for forbidden in ["Home", "runtime tag", "provider", "BasicBlockId", "ValueId"] {
        assert!(
            !carrier_model.contains(forbidden),
            "neutral carrier vocabulary must not contain {forbidden}"
        );
    }
}
