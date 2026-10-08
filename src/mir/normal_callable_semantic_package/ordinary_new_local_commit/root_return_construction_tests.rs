//! Real source-index construction admission; artifact permission remains separate.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};
use crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1;
use crate::mir::resolved_semantics::SourcePathSegmentV1;

fn package(received: bool, nullable: bool, owned: bool) -> VerifiedNormalCallableSemanticPackageV1 {
    let fields = if owned {
        "items: ArrayBox = new ArrayBox() tail: ArrayBox = new ArrayBox() birth() {}"
    } else {
        ""
    };
    let body = if nullable {
        "if size > 0 { return new Token() } return null"
    } else {
        "return new Token()"
    };
    let relay = if received {
        "local item = me.make(7) local spare = new Token() return item"
    } else {
        "local spare = new Token() return me.make(7)"
    };
    issue_with_brand_catalog(&format!("box Token {{ {fields} }} box Maker {{ make(size: i64) {{ {body} }} relay() {{ {relay} }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
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
fn function() -> MirFunction {
    MirFunction::new(
        crate::mir::FunctionSignature {
            name: "construction-admission".into(),
            params: vec![],
            return_type: crate::mir::MirType::Box("Token".into()),
            effects: crate::mir::EffectMask::PURE,
        },
        BasicBlockId(0),
    )
}
#[test]
fn root_return_construction_propagates_original_completion_error_before_optional_lookup() {
    for received in [false, true] {
        let mut package = package(received, false, false);
        let (owner, exit) = identity(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        ledger.completion_index.insert(
            owner,
            Err(FunctionCompletionVerificationErrorV1::OwnerClosureMismatch),
        );
        let error = ledger
            .prepare_root_home_exit(owner, exit.node())
            .unwrap_err();
        assert!(error.contains("construction-completion"), "{error}");
        assert!(error.contains("OwnerClosureMismatch"));
        let error = ledger
            .validate_root_home_exit(owner, &function(), None)
            .unwrap_err();
        assert!(error.contains("construction-completion"), "{error}");
        assert!(
            ledger.root_exits.borrow().is_empty(),
            "no progress is issued after refusal"
        );
    }
}
#[test]
fn root_return_construction_missing_original_index_does_not_borrow_root_fallback() {
    let mut package = package(false, false, false);
    let (owner, exit) = identity(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    // Even an exact retained root copy cannot replace the original index.
    ledger.root_completion = Some(Ok(Rc::clone(
        ledger.completion_index[&owner].as_ref().unwrap(),
    )));
    ledger.completion_index.remove(&owner);
    assert!(!ledger.prepare_root_home_exit(owner, exit.node()).unwrap());
    ledger
        .validate_root_home_exit(owner, &function(), None)
        .unwrap();
    assert!(ledger.root_exits.borrow().is_empty());
}
#[test]
fn root_return_construction_rejects_retained_exit_outside_original_completion() {
    let mut package = package(false, false, false);
    let (owner, exit) = identity(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let foreign = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(999),
    ]));
    let rows = ledger.normal_return_dispositions.as_mut().unwrap();
    let retained = rows.remove(&(owner, exit.clone())).unwrap();
    rows.insert((owner, foreign), retained);
    assert!(ledger
        .prepare_root_home_exit(owner, exit.node())
        .unwrap_err()
        .contains("construction-disposition-exit"));
    assert!(ledger
        .validate_root_home_exit(owner, &function(), None)
        .unwrap_err()
        .contains("construction-disposition-exit"));
    assert!(ledger.root_exits.borrow().is_empty());
}
#[test]
fn root_return_construction_supported_sources_are_ready_without_artifact_permission() {
    for received in [false, true] {
        for nullable in [false, true] {
            let package = package(received, nullable, false);
            let (owner, _) = identity(&package);
            assert!(package
                .ordinary_new_claim_ledger
                .object_return_construction_ready_v1(owner)
                .unwrap());
            assert!(package
                .ordinary_new_claim_ledger
                .validate_no_pending_object_returns_v1()
                .is_err());
        }
    }
    let package = package(false, true, true);
    let (owner, _) = identity(&package);
    assert!(
        !package
            .ordinary_new_claim_ledger
            .object_return_construction_ready_v1(owner)
            .unwrap(),
        "nullable owned-field teardown has no accepted envelope"
    );
}
