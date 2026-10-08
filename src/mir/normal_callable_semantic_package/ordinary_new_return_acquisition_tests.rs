use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};
use std::rc::Rc;

fn package(received: bool, opaque: bool) -> VerifiedNormalCallableSemanticPackageV1 {
    let formal = if opaque { "size" } else { "size: i64" };
    let relay = if received {
        "local item = me.make(7) local sibling = new Token() return item"
    } else {
        "local first = new Token() return me.make(7)"
    };
    issue_with_brand_catalog(&format!("box Token {{}} box Maker {{ make({formal}) {{ return new Token() }} relay() {{ {relay} }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
}
fn terminal(package: &VerifiedNormalCallableSemanticPackageV1) -> TerminalValueReturnV1 {
    let ledger = &package.ordinary_new_claim_ledger;
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Maker", "relay", 0,
    );
    let owner = ledger.callable_result_classes.outcomes(&key).unwrap()[0]
        .site()
        .owner();
    let rows = ledger.terminal_relations_for_owner(owner);
    let [TerminalRelationV1::Value(value)] = rows.as_slice() else {
        panic!("terminal")
    };
    (*value).clone()
}
#[test]
fn completed_object_acquisition_joins_direct_and_received_typed_inputs_without_snapshot_mutation() {
    for received in [false, true] {
        let package = package(received, false);
        let value = terminal(&package);
        let ledger = &package.ordinary_new_claim_ledger;
        let proof = ledger
            .checked_completed_object_acquisition_v1(
                &value,
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
            )
            .unwrap()
            .unwrap();
        assert_eq!(proof.arguments(), &[LocalCallArgumentV1::Integer(7)]);
        let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
            panic!("owned")
        };
        assert_eq!(proof.original(), original.as_ref());
        assert!(ledger
            .lexical_i64_call_source(original.qualification().call())
            .is_none());
        assert!(ledger.has_pending_object_return_v1(value.owner()));
        assert!(ledger.validate_no_pending_object_returns_v1().is_err());
    }
}
#[test]
fn completed_object_acquisition_joins_original_final_opaque_inputs() {
    for received in [false, true] {
        let package = package(received, true);
        let value = terminal(&package);
        let ledger = &package.ordinary_new_claim_ledger;
        let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
            panic!("owned")
        };
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let target = source
            .object_source_target_at_v1(original.qualification().call())
            .unwrap()
            .unwrap();
        assert!(source.contains_definition_for_test(target.callee_owner()));
        let proof = ledger
            .checked_completed_object_acquisition_v1(
                &value,
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
            )
            .unwrap()
            .unwrap();
        assert!(matches!(
            proof.arguments(),
            [LocalCallArgumentV1::BorrowedActual { ordinal: 0, .. }]
        ));
        assert_eq!(proof.original(), original.as_ref());
        assert!(ledger
            .lexical_i64_call_source(original.qualification().call())
            .is_none());
        assert!(ledger.has_pending_object_return_v1(value.owner()));
    }
}
#[test]
fn completed_object_acquisition_missing_index_never_retries_root() {
    let mut package = package(false, false);
    let value = terminal(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.root_completion = Some(ledger.completion_index.remove(&value.owner()).unwrap());
    assert!(ledger
        .checked_completed_object_acquisition_v1(
            &value,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature
        )
        .unwrap()
        .is_none());
}
#[test]
fn completed_object_acquisition_rejected_index_preserves_original_cause() {
    let mut package = package(true, false);
    let value = terminal(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.completion_index.insert(value.owner(), Err(crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::SourceNavigation("original-acquisition-error".into())));
    assert!(ledger
        .checked_completed_object_acquisition_v1(
            &value,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature
        )
        .unwrap_err()
        .contains("original-acquisition-error"));
}

#[test]
fn completed_object_acquisition_preserves_staged_error_in_typed_and_opaque_lanes() {
    for opaque in [false, true] {
        let mut package = package(false, opaque);
        let value = terminal(&package);
        let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
            panic!("owned")
        };
        let site = original.qualification().call().clone();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        ledger
            .borrowed_formal_actuals
            .insert(site, Err("original-staged-acquisition-error".into()));
        assert_eq!(
            ledger
                .checked_completed_object_acquisition_v1(
                    &value,
                    &package.selected,
                    &package.parameter_contracts,
                    &package.physical_signature
                )
                .unwrap_err(),
            "original-staged-acquisition-error"
        );
    }
}

#[test]
fn completed_object_acquisition_received_typed_scalar_keeps_original_binding() {
    let package = issue_with_brand_catalog("box Token {} box Maker { make(size: i64) { return new Token() } relay(size: i64) { local item = me.make(size) return item } } static box Main { main() { return 0 } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Maker", "relay", 1,
    );
    let owner = ledger.callable_result_classes.outcomes(&key).unwrap()[0]
        .site()
        .owner();
    let rows = ledger.terminal_relations_for_owner(owner);
    let [TerminalRelationV1::Value(value)] = rows.as_slice() else {
        panic!("terminal")
    };
    let proof = ledger
        .checked_completed_object_acquisition_v1(
            value,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
        .unwrap()
        .unwrap();
    let [LocalCallArgumentV1::Scalar(binding)] = proof.arguments() else {
        panic!("scalar")
    };
    assert_eq!(binding.owner(), owner);
    assert!(ledger.has_pending_object_return_v1(owner));
}

#[test]
fn completed_object_acquisition_missing_staged_input_stays_unavailable() {
    for received in [false, true] {
        for opaque in [false, true] {
            let mut package = package(received, opaque);
            let value = terminal(&package);
            let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
                panic!("owned")
            };
            let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
            assert!(ledger
                .borrowed_formal_actuals
                .remove(original.qualification().call())
                .is_some());
            assert!(ledger
                .checked_completed_object_acquisition_v1(
                    &value,
                    &package.selected,
                    &package.parameter_contracts,
                    &package.physical_signature,
                )
                .unwrap()
                .is_none());
            assert!(ledger.validate_no_pending_object_returns_v1().is_err());
        }
    }
}

#[test]
fn completed_object_acquisition_missing_receiver_observation_stays_unavailable() {
    for opaque in [false, true] {
        let mut package = package(true, opaque);
        let value = terminal(&package);
        let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
            panic!("owned")
        };
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        assert!(ledger
            .receiver_call_observations
            .remove(original.qualification().call())
            .is_some());
        assert!(ledger
            .checked_completed_object_acquisition_v1(
                &value,
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
            )
            .unwrap()
            .is_none());
        assert!(ledger.validate_no_pending_object_returns_v1().is_err());
    }
}

#[test]
fn completed_object_acquisition_receiver_swap_refuses_foreign_observation() {
    for opaque in [false, true] {
        let formal = if opaque { "size" } else { "size: i64" };
        let foreign = issue_with_brand_catalog(&format!("box Token {{}} box Maker {{ make({formal}) {{ return new Token() }} relay() {{ local item = me.make(8) local sibling = new Token() return item }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap();
        let mut package = package(true, opaque);
        let value = terminal(&package);
        let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
            panic!("owned")
        };
        let site = original.qualification().call();
        let foreign_value = terminal(&foreign);
        let TerminalReturnedSourceV1::OwnedCall(foreign_original) = foreign_value.returned() else {
            panic!("foreign owned")
        };
        let replacement = foreign
            .ordinary_new_claim_ledger
            .receiver_call_observation(foreign_original.qualification().call())
            .unwrap()
            .clone();
        assert_ne!(replacement.destination().owner(), value.owner());
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        ledger
            .receiver_call_observations
            .insert(site.clone(), replacement);
        assert!(ledger
            .checked_completed_object_acquisition_v1(
                &value,
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
            )
            .unwrap_err()
            .contains("acquisition-receiver-identity"));
        assert!(ledger.validate_no_pending_object_returns_v1().is_err());
    }
}

#[test]
fn completed_object_opaque_target_lender_shares_original_storage_and_error_scope() {
    let foreign = package(false, true);
    for received in [false, true] {
        let mut package = package(received, true);
        let value = terminal(&package);
        let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
            panic!("owned")
        };
        let loan = original.qualification();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let target = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .object_source_target_at_v1(loan.call())
            .unwrap()
            .unwrap()
            .clone();
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| row.owner == target.callee_owner())
            .unwrap();
        let qualified = ledger
            .checked_completed_opaque_object_arguments_v1(&target, loan, contract)
            .unwrap()
            .unwrap();
        let generic = ledger
            .checked_completed_opaque_object_target_arguments_v1(&target, contract)
            .unwrap()
            .unwrap();
        assert_eq!(generic.as_ptr(), qualified.as_ptr());
        let foreign_contract = foreign
            .parameter_contracts
            .iter()
            .find(|row| row.parameters.len() == 1)
            .unwrap();
        assert!(ledger
            .checked_completed_opaque_object_target_arguments_v1(&target, foreign_contract)
            .unwrap_err()
            .contains("opaque-input-identity"));
        let retained = ledger.borrowed_formal_actuals.remove(loan.call()).unwrap();
        assert!(ledger
            .checked_completed_opaque_object_target_arguments_v1(&target, contract)
            .unwrap()
            .is_none());
        ledger
            .borrowed_formal_actuals
            .insert(loan.call().clone(), Err("opaque-original-refusal".into()));
        assert_eq!(
            ledger
                .checked_completed_opaque_object_arguments_v1(&target, loan, contract)
                .unwrap_err(),
            "opaque-original-refusal"
        );
        assert_eq!(
            ledger
                .checked_completed_opaque_object_target_arguments_v1(&target, contract)
                .unwrap_err(),
            "opaque-original-refusal"
        );
        ledger
            .borrowed_formal_actuals
            .insert(loan.call().clone(), retained);
    }
}

#[test]
fn completed_object_acquisition_rejects_changed_snapshot_with_matching_terminal_table() {
    for received in [false, true] {
        for opaque in [false, true] {
            for observed in [false, true] {
                let mut package = package(received, opaque);
                let original = terminal(&package);
                let arguments = vec![LocalCallArgumentV1::Integer(99)].into_boxed_slice();
                let support = if observed {
                    ObjectCallSourceSupportV1::Observed(arguments)
                } else {
                    ObjectCallSourceSupportV1::SourceOnly(arguments)
                };
                let changed = original.with_object_arguments_for_test(support);
                let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
                Rc::make_mut(
                    ledger
                        .terminal_relation_index
                        .get_mut(&original.owner())
                        .unwrap(),
                )
                .insert(
                    original.return_site().clone(),
                    TerminalRelationV1::Value(changed.clone()),
                );
                assert!(ledger
                    .checked_completed_object_acquisition_v1(
                        &changed,
                        &package.selected,
                        &package.parameter_contracts,
                        &package.physical_signature,
                    )
                    .unwrap_err()
                    .contains("acquisition-argument-snapshot-drift"));
                assert!(ledger.validate_no_pending_object_returns_v1().is_err());
            }
        }
    }
}

#[test]
fn completed_object_acquisition_raw_only_opaque_definition_cannot_grant_execution() {
    for received in [false, true] {
        let mut package = package(received, true);
        let value = terminal(&package);
        let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
            panic!("owned")
        };
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let source = ledger
            .borrowed_formal_source
            .as_mut()
            .unwrap()
            .as_mut()
            .unwrap();
        let owner = source
            .object_source_target_at_v1(original.qualification().call())
            .unwrap()
            .unwrap()
            .callee_owner();
        source.retain_only_source_definition_for_test(owner);
        assert!(!source.contains_definition_for_test(owner));
        assert!(ledger
            .checked_completed_object_acquisition_v1(
                &value,
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
            )
            .unwrap()
            .is_none());
        assert!(ledger.validate_no_pending_object_returns_v1().is_err());
    }
}
