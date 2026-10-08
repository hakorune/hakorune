//! Corroborate the same original publication and executable input packet.
use super::*;
use crate::mir::callable_result_representation::{
    StaticCallResultPublicationTakeV1, VerifiedSameModuleCallableResultCatalogV1,
    VerifiedStaticCallResultPublicationOwnerV1,
};
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use crate::mir::source_call_target::{
    VerifiedStaticImportAliasViewV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
};
use std::rc::Rc;
const TEXT: &str = "static box Layout { pick(p) { return 0 } } static box Main { main() { local a = Layout.pick(8) return 0 } }";
fn package() -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1
{
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        TEXT,
    )
    .unwrap()
}
fn source(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
) -> Rc<StaticIncomingSourceV1> {
    let ledger = &package.ordinary_new_claim_ledger;
    let site = ledger
        .borrowed_static_source_sites
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .clone();
    ledger
        .selected_static_local_source_v1(&site)
        .unwrap()
        .unwrap()
}
fn handoff(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
    original: &StaticIncomingSourceV1,
) -> crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1 {
    let declarations = package.catalog.catalog();
    let imports = VerifiedStaticImportAliasViewV1::seal(declarations, []).unwrap();
    let targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, &imports)
        .unwrap()
        .into_targets();
    let results =
        VerifiedSameModuleCallableResultCatalogV1::verify(declarations, &targets).unwrap();
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(declarations, &targets, &results)
            .unwrap();
    let StaticCallResultPublicationTakeV1::Selected(handoff) = owner
        .take_for_source(declarations, original.caller(), original.call_site().site())
        .unwrap()
    else {
        panic!("original handoff");
    };
    assert!(
        owner
            .take_for_source(declarations, original.caller(), original.call_site().site())
            .is_err(),
        "affine publication taken exactly once"
    );
    handoff
}
fn bindings(original: &StaticIncomingSourceV1) -> (Binding, Binding) {
    (
        (
            BasicBlockId(10),
            MirInstruction::Invoke {
                operation: InvokeOperation::Call {
                    call: MirCall::new(
                        None,
                        crate::mir::Callee::Global(
                            original.target().canonical_global_target_v1().unwrap(),
                        ),
                        vec![ValueId(78)],
                    ),
                    result: InvokeCallResultKind::I64,
                },
                fault_frame: ValueId(4),
                normal_landing: BasicBlockId(11),
                fault_landing: BasicBlockId(12),
            },
        ),
        (
            BasicBlockId(11),
            MirInstruction::InvokeNormalResult {
                invoke_block: BasicBlockId(10),
                dst: ValueId(79),
            },
        ),
    )
}
#[test]
fn static_packet_corroborates_borrowed_literal_and_original_global_projection() {
    let package = package();
    let ledger = &package.ordinary_new_claim_ledger;
    let original = source(&package);
    let actual = &ledger
        .borrowed_static_packet_actuals_v1(&original)
        .unwrap()
        .unwrap()[0];
    let argument = LexicalCallArgumentProjectionV1::BorrowedLiteral {
        ordinal: 0,
        site: actual.site.clone(),
        formal: actual.formal,
        binding: (
            BasicBlockId(10),
            MirInstruction::Const {
                dst: ValueId(78),
                value: crate::mir::ConstValue::Integer(8),
            },
        ),
    };
    let (invoke, projection) = bindings(&original);
    let packet = EmittedLexicalCallProjectionV1::new_static(
        original.clone(),
        handoff(&package, &original),
        vec![argument],
        invoke,
        projection,
        ledger,
    )
    .unwrap();
    let observation = ledger
        .local_call_for_owner(original.call_site().owner(), original.call_site().site())
        .unwrap();
    assert_eq!(
        packet
            .value_with_ledger(observation.owner(), observation.arguments(), ledger)
            .unwrap(),
        ValueId(79)
    );
    assert!(packet
        .original_row()
        .unwrap_err()
        .contains("instance-source-required"));
    assert!(packet
        .value_for_source(observation.owner(), observation.arguments())
        .unwrap_err()
        .contains("ledger-missing"));
}
#[test]
fn static_packet_refuses_foreign_brand_raw_payload_and_physical_drift() {
    for mutation in 0..7 {
        let package = package();
        let ledger = &package.ordinary_new_claim_ledger;
        let original = source(&package);
        let actual = &ledger
            .borrowed_static_packet_actuals_v1(&original)
            .unwrap()
            .unwrap()[0];
        let binding = (
            BasicBlockId(10),
            MirInstruction::Const {
                dst: ValueId(78),
                value: crate::mir::ConstValue::Integer(if mutation == 2 { 9 } else { 8 }),
            },
        );
        let argument = if mutation == 1 {
            LexicalCallArgumentProjectionV1::Integer(binding)
        } else {
            LexicalCallArgumentProjectionV1::BorrowedLiteral {
                ordinal: if mutation == 3 { 1 } else { 0 },
                site: actual.site.clone(),
                formal: actual.formal,
                binding,
            }
        };
        let foreign = self::package();
        let publication = if mutation == 0 {
            handoff(&foreign, &source(&foreign))
        } else {
            handoff(&package, &original)
        };
        let (mut invoke, mut projection) = bindings(&original);
        if mutation == 4 {
            projection.0 = BasicBlockId(99);
        }
        if let MirInstruction::Invoke {
            operation: InvokeOperation::Call { call, result },
            ..
        } = &mut invoke.1
        {
            if mutation == 5 {
                *result = InvokeCallResultKind::NullableHandle;
            }
            if mutation == 6 {
                call.callee = crate::mir::Callee::SameModuleInstance {
                    key: original.target().clone(),
                    receiver: ValueId(78),
                };
            }
        }
        let error = EmittedLexicalCallProjectionV1::new_static(
            original,
            publication,
            vec![argument],
            invoke,
            projection,
            ledger,
        )
        .unwrap_err();
        assert!(
            error.starts_with("[freeze:contract]"),
            "mutation {mutation}: {error}"
        );
    }
}

#[test]
fn static_zero_packet_preserves_current_source_and_original_affine_handoff() {
    const ZERO: &str = "static box Layout { word() { return 8 } run() { local a = me.word() return 0 } } static box Main { main() { return 0 } }";
    for foreign in [false, true] {
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(ZERO).unwrap();
        let original = source(&package);
        assert!(original.current_owner_source().is_some());
        assert!(!original.is_qualified());
        let other = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(ZERO).unwrap();
        let publication = if foreign {
            handoff(&other, &source(&other))
        } else {
            handoff(&package, &original)
        };
        let (mut invoke, projection) = bindings(&original);
        if let MirInstruction::Invoke {
            operation: InvokeOperation::Call { call, .. },
            ..
        } = &mut invoke.1
        {
            call.args.clear();
        }
        let packet = EmittedLexicalCallProjectionV1::new_static(
            original.clone(),
            publication,
            vec![],
            invoke,
            projection,
            &package.ordinary_new_claim_ledger,
        );
        if foreign {
            assert!(packet.unwrap_err().contains("original-source-drift"));
        } else {
            let packet = packet.unwrap();
            assert_eq!(
                packet
                    .value_with_ledger(
                        original.call_site().owner(),
                        &[],
                        &package.ordinary_new_claim_ledger
                    )
                    .unwrap(),
                ValueId(79)
            );
        }
    }
}
