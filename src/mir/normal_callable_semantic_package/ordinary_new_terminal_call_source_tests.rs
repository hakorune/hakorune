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
        "local out = me.make(7) return out"
    } else {
        "return me.make(7)"
    };
    issue_with_brand_catalog(&format!("box Token {{}} box Maker {{ make({formal}) {{ {body} }} relay() {{ {relay} }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
}
fn exit(
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
fn terminal_call_source_selects_real_direct_exact_kind_and_same_affine_row() {
    for opaque in [false, true] {
        for nullable in [false, true] {
            let package = package(false, opaque, nullable);
            let (owner, exit) = exit(&package);
            let ledger = &package.ordinary_new_claim_ledger;
            let source = ledger
                .verified_terminal_call_source_v1(owner, &exit)
                .unwrap()
                .unwrap();
            assert!(source.legacy_terminal().is_none());
            let expected = if nullable {
                InvokeCallResultKind::NullableHandle
            } else {
                InvokeCallResultKind::Handle
            };
            assert_eq!(source.result(), expected);
            let row = ledger
                .take_borrowed_lexical_call_for_return_v1(owner, &exit)
                .unwrap()
                .unwrap();
            assert_eq!(row.call_site().site(), source.call_site());
            assert_eq!(row.result(), Some(expected));
            source.corroborate_row(&row).unwrap();
            ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap();
            assert!(ledger
                .take_borrowed_lexical_call_for_return_v1(owner, &exit)
                .unwrap_err()
                .contains("already-taken"));
            assert!(ledger.validate_no_pending_object_returns_v1().is_err());
        }
    }
}

#[test]
fn terminal_call_source_received_remains_value_without_terminal_take() {
    for nullable in [false, true] {
        let package = package(true, false, nullable);
        let (owner, exit) = exit(&package);
        let ledger = &package.ordinary_new_claim_ledger;
        assert!(ledger
            .verified_terminal_call_source_v1(owner, &exit)
            .unwrap()
            .is_none());
        assert!(ledger
            .take_borrowed_lexical_call_for_return_v1(owner, &exit)
            .unwrap()
            .is_none());
    }
}

#[test]
fn terminal_call_source_missing_direct_proof_never_consumes_ready_row() {
    let mut package = package(false, false, false);
    let (owner, exit) = exit(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let site = ledger
        .verified_direct_object_return_source_v1(owner, &exit)
        .unwrap()
        .unwrap()
        .target()
        .call_site()
        .clone();
    ledger.normal_return_dispositions.as_mut().unwrap().clear();
    assert!(ledger
        .take_borrowed_lexical_call_for_return_v1(owner, &exit)
        .unwrap()
        .is_none());
    assert!(ledger
        .take_lexical_instance_call(owner, site.site())
        .unwrap()
        .is_some());
}

#[test]
fn terminal_call_source_wrong_result_and_foreign_target_refuse_same_direct_loan() {
    let package = package(false, false, false);
    let foreign = self::package(false, false, false);
    let (owner, exit) = exit(&package);
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger
        .verified_terminal_call_source_v1(owner, &exit)
        .unwrap()
        .unwrap();
    let row = ledger
        .take_borrowed_lexical_call_for_return_v1(owner, &exit)
        .unwrap()
        .unwrap()
        .with_result_for_test(Some(InvokeCallResultKind::I64));
    assert!(source
        .corroborate_row(&row)
        .unwrap_err()
        .contains("row-identity"));
    let (foreign_owner, foreign_exit) = self::exit(&foreign);
    let row = foreign
        .ordinary_new_claim_ledger
        .take_borrowed_lexical_call_for_return_v1(foreign_owner, &foreign_exit)
        .unwrap()
        .unwrap();
    assert!(source.corroborate_row(&row).is_err());
}

#[test]
fn terminal_call_source_legacy_input_refusal_precedes_corrupt_result() {
    use super::super::super::lexical_instance_call::LexicalInstanceCallDispositionSlotV1;
    let mut package = issue_with_brand_catalog("box Transport { birth() {} probe(p): i64 { return 0 } other(q): i64 { return 0 } } static box Main { main() { local recv = new Transport() local ignored = recv.other(8) return recv.probe(7) } }").unwrap();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let owner = ledger.root_owner().unwrap();
    let exit = ledger.completion_for_owner(owner).unwrap().explicit_sites()[0].clone();
    let source = ledger
        .verified_terminal_call_source_v1(owner, &exit)
        .unwrap()
        .unwrap();
    assert!(source.legacy_terminal().is_some());
    let site = source.call_site().clone();
    let row = ledger
        .take_lexical_instance_call(owner, &site)
        .unwrap()
        .unwrap();
    let key = row.call_site().clone();
    ledger.lexical_instance_calls.borrow_mut().insert(
        key.clone(),
        LexicalInstanceCallDispositionSlotV1::Ready(
            row.with_result_for_test(Some(InvokeCallResultKind::Handle)),
        ),
    );
    let unrelated = ledger
        .borrowed_formal_actuals
        .keys()
        .find(|site| **site != key)
        .unwrap()
        .clone();
    ledger
        .borrowed_formal_actuals
        .insert(unrelated, Err("legacy-unrelated-incoming-error".into()));
    assert_eq!(
        ledger
            .take_borrowed_lexical_call_for_return_v1(owner, &exit)
            .unwrap_err(),
        "legacy-unrelated-incoming-error"
    );
}
