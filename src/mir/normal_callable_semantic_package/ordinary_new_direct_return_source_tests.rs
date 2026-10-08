use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};
use std::rc::Rc;

fn package(
    received: bool,
    opaque: bool,
    nullable: bool,
) -> VerifiedNormalCallableSemanticPackageV1 {
    let formal = if opaque { "size" } else { "size: i64" };
    let body = if nullable {
        "if size > 0 { return new Token() } return null"
    } else {
        "return new Token()"
    };
    let relay = if received {
        "local item = me.make(7) return item"
    } else {
        "local prior = new Token() return me.make(7)"
    };
    issue_with_brand_catalog(&format!("box Token {{}} box Maker {{ make({formal}) {{ {body} }} relay() {{ {relay} }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
}
fn identity(
    package: &VerifiedNormalCallableSemanticPackageV1,
) -> (FunctionOwnerIdV1, SourceStmtSiteV1) {
    package
        .ordinary_new_claim_ledger
        .normal_return_dispositions
        .as_ref()
        .unwrap()
        .keys()
        .next()
        .unwrap()
        .clone()
}
#[test]
fn direct_return_source_original_packet_lends_typed_opaque_handle_nullable() {
    for opaque in [false, true] {
        for nullable in [false, true] {
            let package = package(false, opaque, nullable);
            let (owner, exit) = identity(&package);
            let ledger = &package.ordinary_new_claim_ledger;
            let view = ledger
                .verified_direct_object_return_source_v1(owner, &exit)
                .unwrap()
                .unwrap();
            assert_eq!(
                view.result(),
                if nullable {
                    InvokeCallResultKind::NullableHandle
                } else {
                    InvokeCallResultKind::Handle
                }
            );
            assert_eq!(view.class(), "Token");
            let (_, teardown_nullable) = view.teardown().expect("exact Fresh teardown");
            assert_eq!(teardown_nullable, nullable);
            assert_eq!(view.target().call_site().owner(), owner);
            assert_eq!(view.arguments().len(), 1);
            let NormalReturnDispositionV1::Verified { proof } =
                &ledger.normal_return_dispositions.as_ref().unwrap()[&(owner, exit.clone())]
            else {
                panic!("verified")
            };
            assert_eq!(
                view.arguments().as_ptr(),
                proof.acquisition().arguments().as_ptr()
            );
            assert_eq!(
                ledger
                    .borrowed_terminal_arguments_v1(owner, &exit)
                    .unwrap()
                    .unwrap()
                    .as_ref(),
                view.arguments()
            );
            assert!(ledger.validate_no_pending_object_returns_v1().is_err());
        }
    }
}
#[test]
fn direct_return_source_received_is_not_an_outer_call_packet() {
    let package = package(true, false, false);
    let (owner, exit) = identity(&package);
    assert!(package
        .ordinary_new_claim_ledger
        .verified_direct_object_return_source_v1(owner, &exit)
        .unwrap()
        .is_none());
    assert!(package
        .ordinary_new_claim_ledger
        .borrowed_terminal_arguments_v1(owner, &exit)
        .unwrap()
        .is_none());
}
#[test]
fn direct_return_source_missing_index_and_proof_do_not_borrow_root_copies() {
    for missing_index in [false, true] {
        let mut package = package(false, false, false);
        let (owner, exit) = identity(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        if missing_index {
            ledger.root_completion = Some(ledger.completion_index.remove(&owner).unwrap());
        } else {
            ledger.normal_return_dispositions = None;
        }
        assert!(ledger
            .verified_direct_object_return_source_v1(owner, &exit)
            .unwrap()
            .is_none());
        assert!(ledger
            .borrowed_terminal_arguments_v1(owner, &exit)
            .unwrap()
            .is_none());
    }
}
#[test]
fn direct_return_source_foreign_saved_proof_refuses() {
    let mut package = package(false, false, false);
    let mut foreign = self::package(false, false, false);
    let (owner, exit) = identity(&package);
    let (foreign_owner, foreign_exit) = identity(&foreign);
    let proof = Rc::get_mut(&mut foreign.ordinary_new_claim_ledger)
        .unwrap()
        .normal_return_dispositions
        .as_mut()
        .unwrap()
        .remove(&(foreign_owner, foreign_exit))
        .unwrap();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger
        .normal_return_dispositions
        .as_mut()
        .unwrap()
        .insert((owner, exit.clone()), proof);
    assert!(ledger
        .verified_direct_object_return_source_v1(owner, &exit)
        .unwrap_err()
        .contains("projection-proof-identity"));
}

#[test]
fn direct_return_source_zero_argument_nested_calls_keep_original_packets() {
    let package = issue_with_brand_catalog("box Token {} box Maker { make() { return new Token() } relay() { return me.make() } outer() { return me.relay() } } static box Main { main() { return 0 } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let rows = ledger.normal_return_dispositions.as_ref().unwrap();
    assert_eq!(rows.len(), 2);
    for (owner, exit) in rows.keys() {
        let view = ledger
            .verified_direct_object_return_source_v1(*owner, exit)
            .unwrap()
            .unwrap();
        assert!(view.arguments().is_empty());
        assert_eq!(view.result(), InvokeCallResultKind::Handle);
        assert_eq!(view.class(), "Token");
        assert!(ledger
            .borrowed_terminal_arguments_v1(*owner, exit)
            .unwrap()
            .unwrap()
            .is_empty());
    }
}

#[test]
fn direct_return_source_missing_terminal_index_cannot_borrow_root_relation() {
    let mut package = package(false, false, false);
    let (owner, exit) = identity(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let relation = ledger.terminal_relation_index.remove(&owner).unwrap()[&exit].clone();
    Rc::make_mut(&mut ledger.terminal_relation).insert(exit.clone(), relation);
    assert!(ledger
        .verified_direct_object_return_source_v1(owner, &exit)
        .unwrap()
        .is_none());
    assert!(ledger
        .borrowed_terminal_arguments_v1(owner, &exit)
        .unwrap()
        .is_none());
}

#[test]
fn direct_return_source_taken_row_matches_original_result_and_target() {
    for opaque in [false, true] {
        for nullable in [false, true] {
            let package = package(false, opaque, nullable);
            let (owner, exit) = identity(&package);
            let ledger = &package.ordinary_new_claim_ledger;
            let call = ledger
                .verified_direct_object_return_source_v1(owner, &exit)
                .unwrap()
                .unwrap()
                .target()
                .call_site()
                .clone();
            let row = ledger
                .take_lexical_instance_call(owner, call.site())
                .unwrap()
                .unwrap();
            let view = ledger
                .checked_direct_object_return_row_v1(owner, &exit, &row)
                .unwrap()
                .unwrap();
            assert_eq!(view.target(), row.source_target());
            assert_eq!(Some(view.result()), row.result());
            assert!(ledger
                .take_lexical_instance_call(owner, call.site())
                .is_err());
            assert!(ledger.validate_no_pending_object_returns_v1().is_err());
        }
    }
}

#[test]
fn direct_return_source_taken_row_missing_wrong_result_and_foreign_target_refuse() {
    for result in [
        None,
        Some(InvokeCallResultKind::I64),
        Some(InvokeCallResultKind::NullableHandle),
    ] {
        let package = package(false, false, false);
        let (owner, exit) = identity(&package);
        let ledger = &package.ordinary_new_claim_ledger;
        let call = ledger
            .verified_direct_object_return_source_v1(owner, &exit)
            .unwrap()
            .unwrap()
            .target()
            .call_site()
            .clone();
        let row = ledger
            .take_lexical_instance_call(owner, call.site())
            .unwrap()
            .unwrap()
            .with_result_for_test(result);
        assert!(ledger
            .checked_direct_object_return_row_v1(owner, &exit, &row)
            .unwrap_err()
            .contains("taken-row-drift"));
        assert!(ledger
            .take_lexical_instance_call(owner, call.site())
            .is_err());
    }
    let package = package(false, false, false);
    let foreign = self::package(false, false, false);
    let (owner, exit) = identity(&package);
    let (foreign_owner, foreign_exit) = identity(&foreign);
    let foreign_ledger = &foreign.ordinary_new_claim_ledger;
    let call = foreign_ledger
        .verified_direct_object_return_source_v1(foreign_owner, &foreign_exit)
        .unwrap()
        .unwrap()
        .target()
        .call_site()
        .clone();
    let row = foreign_ledger
        .take_lexical_instance_call(foreign_owner, call.site())
        .unwrap()
        .unwrap();
    assert!(package
        .ordinary_new_claim_ledger
        .checked_direct_object_return_row_v1(owner, &exit, &row)
        .unwrap_err()
        .contains("taken-row-drift"));
}

#[test]
fn direct_return_source_taken_object_row_never_retries_missing_proof_as_i64() {
    let mut package = package(false, false, false);
    let (owner, exit) = identity(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let call = ledger
        .verified_direct_object_return_source_v1(owner, &exit)
        .unwrap()
        .unwrap()
        .target()
        .call_site()
        .clone();
    let row = ledger
        .take_lexical_instance_call(owner, call.site())
        .unwrap()
        .unwrap();
    ledger.normal_return_dispositions = None;
    assert!(ledger
        .checked_direct_object_return_row_v1(owner, &exit, &row)
        .unwrap_err()
        .contains("taken-row-proof-missing"));
    assert!(ledger
        .take_lexical_instance_call(owner, call.site())
        .is_err());
}

impl OrdinaryNewClaimLedgerV1 {
    /// Builder negative fixture: take the original packet, then corrupt only its outcome.
    pub(in crate::mir) fn take_direct_return_row_for_test(
        &self,
        owner: FunctionOwnerIdV1,
        result: Option<InvokeCallResultKind>,
    ) -> (SourceStmtSiteV1, LexicalInstanceCallDispositionRowV1) {
        let exit = self
            .normal_return_dispositions
            .as_ref()
            .unwrap()
            .keys()
            .find(|(candidate, _)| *candidate == owner)
            .unwrap()
            .1
            .clone();
        let call = self
            .verified_direct_object_return_source_v1(owner, &exit)
            .unwrap()
            .unwrap()
            .target()
            .call_site()
            .clone();
        let row = self
            .take_lexical_instance_call(owner, call.site())
            .unwrap()
            .unwrap();
        (exit, row.with_result_for_test(result))
    }
}
