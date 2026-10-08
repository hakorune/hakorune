use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};

fn package(received: bool, opaque: bool) -> VerifiedNormalCallableSemanticPackageV1 {
    let formal = if opaque { "size" } else { "size: i64" };
    let relay = if received {
        "local item = me.make(7) local sibling = new Token() return item"
    } else {
        "local first = new Token() return me.make(7)"
    };
    issue_with_brand_catalog(&format!("box Token {{}} box Maker {{ make({formal}) {{ return new Token() }} relay() {{ {relay} }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
}
fn value(package: &VerifiedNormalCallableSemanticPackageV1) -> TerminalValueReturnV1 {
    let ledger = &package.ordinary_new_claim_ledger;
    let map = ledger.normal_return_dispositions.as_ref().unwrap();
    let (owner, exit) = map.keys().next().unwrap();
    let Some(TerminalRelationV1::Value(value)) =
        ledger.terminal_relation_for_owner_at(*owner, exit)
    else {
        panic!("value")
    };
    value.clone()
}
fn reseal(package: &mut VerifiedNormalCallableSemanticPackageV1) -> Result<(), String> {
    Rc::get_mut(&mut package.ordinary_new_claim_ledger)
        .unwrap()
        .seal_object_return_dispositions_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
}

#[test]
fn normal_return_factory_installs_typed_opaque_direct_received_proofs_without_artifact_grant() {
    for opaque in [false, true] {
        for received in [false, true] {
            let package = package(received, opaque);
            let value = value(&package);
            let ledger = &package.ordinary_new_claim_ledger;
            let map = ledger.normal_return_dispositions.as_ref().unwrap();
            let NormalReturnDispositionV1::Verified { proof } =
                &map[&(value.owner(), value.return_site().clone())]
            else {
                panic!("proof")
            };
            let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
                panic!("owned")
            };
            assert_eq!(proof.acquisition().original(), original.as_ref());
            assert_eq!(map.len(), 1);
            assert!(ledger.has_pending_object_return_v1(value.owner()));
            assert!(ledger.validate_no_pending_object_returns_v1().is_err());
        }
    }
}

#[test]
fn normal_return_duplicate_nonempty_seal_preserves_original_proof() {
    let mut package = package(false, false);
    let value = value(&package);
    let identity = (value.owner(), value.return_site().clone());
    let NormalReturnDispositionV1::Verified { proof } = &package
        .ordinary_new_claim_ledger
        .normal_return_dispositions
        .as_ref()
        .unwrap()[&identity]
    else {
        panic!("proof")
    };
    let original = Rc::clone(proof);
    assert!(reseal(&mut package)
        .unwrap_err()
        .contains("normal-duplicate-seal"));
    let NormalReturnDispositionV1::Verified { proof } = &package
        .ordinary_new_claim_ledger
        .normal_return_dispositions
        .as_ref()
        .unwrap()[&identity]
    else {
        panic!("proof")
    };
    assert!(Rc::ptr_eq(&original, proof));
}

#[test]
fn normal_return_empty_factory_map_is_sealed_and_rejects_reissuance() {
    let mut package = issue_with_brand_catalog("static box Main { main() { return 0 } }").unwrap();
    assert!(package
        .ordinary_new_claim_ledger
        .normal_return_dispositions
        .as_ref()
        .unwrap()
        .is_empty());
    assert!(reseal(&mut package)
        .unwrap_err()
        .contains("normal-duplicate-seal"));
    assert!(package
        .ordinary_new_claim_ledger
        .normal_return_dispositions
        .as_ref()
        .unwrap()
        .is_empty());
}

#[test]
fn normal_return_conflicting_root_index_terminal_leaves_map_uninstalled() {
    let mut package = package(true, false);
    let value = value(&package);
    let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
        panic!("owned")
    };
    assert_ne!(value.value_site(), original.qualification().call().site());
    let conflict = value.with_value_site_for_test(original.qualification().call().site().clone());
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.normal_return_dispositions = None;
    ledger.terminal_relation.insert(
        value.return_site().clone(),
        TerminalRelationV1::Value(conflict),
    );
    assert!(reseal(&mut package)
        .unwrap_err()
        .contains("normal-exit-identity"));
    assert!(package
        .ordinary_new_claim_ledger
        .normal_return_dispositions
        .is_none());
}

#[test]
fn normal_return_missing_input_retains_unavailable_disposition_and_pending_gate() {
    let mut package = package(false, false);
    let value = value(&package);
    let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
        panic!("owned")
    };
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.normal_return_dispositions = None;
    assert!(ledger
        .borrowed_formal_actuals
        .remove(original.qualification().call())
        .is_some());
    reseal(&mut package).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    assert!(matches!(
        ledger.normal_return_dispositions.as_ref().unwrap()
            [&(value.owner(), value.return_site().clone())],
        NormalReturnDispositionV1::Unavailable(ObjectReturnHandoffUnavailableV1::Acquisition)
    ));
    assert!(ledger.validate_no_pending_object_returns_v1().is_err());
}

#[test]
fn normal_return_late_receiver_error_never_installs_earlier_successes() {
    let mut package = issue_with_brand_catalog("box Token {} box Maker { make(size: i64) { return new Token() } left() { local item = me.make(7) return item } right() { local item = me.make(8) return item } } static box Main { main() { return 0 } }").unwrap();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let map = ledger.normal_return_dispositions.take().unwrap();
    assert_eq!(map.len(), 2);
    let identities: Vec<_> = map.keys().cloned().collect();
    let calls: Vec<_> = identities
        .iter()
        .map(|(owner, exit)| {
            let Some(TerminalRelationV1::Value(value)) =
                ledger.terminal_relation_for_owner_at(*owner, exit)
            else {
                panic!("value")
            };
            let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
                panic!("owned")
            };
            original.qualification().call().clone()
        })
        .collect();
    let replacement = ledger.receiver_call_observation(&calls[0]).unwrap().clone();
    ledger
        .receiver_call_observations
        .insert(calls[1].clone(), replacement);
    assert!(reseal(&mut package)
        .unwrap_err()
        .contains("acquisition-receiver-identity"));
    assert!(package
        .ordinary_new_claim_ledger
        .normal_return_dispositions
        .is_none());
}
