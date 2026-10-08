use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};

fn package(nullable: bool) -> VerifiedNormalCallableSemanticPackageV1 {
    let body = if nullable {
        "if size > 0 { return new Token() } return null"
    } else {
        "return new Token()"
    };
    issue_with_brand_catalog(&format!("box Token {{}} box Maker {{ make(size: i64) {{ {body} }} relay() {{ return me.make(7) }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
}
fn input(
    package: &VerifiedNormalCallableSemanticPackageV1,
) -> (
    LexicalInstanceCallSourceTargetV1,
    ObjectReturnCallQualificationV1,
) {
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Maker", "relay", 0,
    );
    let ledger = &package.ordinary_new_claim_ledger;
    let value = ledger.callable_result_classes.outcomes(&key).unwrap()[0].site();
    let loan = ledger
        .callable_result_classes
        .object_return_qualification(value)
        .unwrap();
    let target = source
        .object_source_target_at_v1(loan.call())
        .unwrap()
        .unwrap()
        .clone();
    (target, loan)
}
#[test]
fn completed_object_callee_covers_all_fresh_and_nullable_exits_with_original_edges() {
    for nullable in [false, true] {
        let package = package(nullable);
        let (target, loan) = input(&package);
        let ledger = &package.ordinary_new_claim_ledger;
        let rows = ledger
            .checked_completed_object_callee_exits_v1(&target, &loan)
            .unwrap()
            .unwrap();
        assert_eq!(rows.len(), if nullable { 2 } else { 1 });
        for ((witness, terminal), original) in rows.iter().zip(loan.witnesses()) {
            let ResultWitnessStepV1::Call { callee, .. } = original.step() else {
                panic!("call")
            };
            assert!(Rc::ptr_eq(witness, callee));
            assert_eq!(
                witness.site(),
                &OwnedExprSiteV1::new(terminal.owner(), terminal.value_site().clone())
            );
        }
    }
}
#[test]
fn completed_object_callee_missing_index_never_retries_root_completion() {
    let mut package = package(false);
    let (target, loan) = input(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let completion = ledger
        .completion_index
        .remove(&target.callee_owner())
        .unwrap();
    ledger.root_completion = Some(completion);
    assert!(ledger
        .checked_completed_object_callee_exits_v1(&target, &loan)
        .unwrap()
        .is_none());
}
#[test]
fn completed_object_callee_rejected_index_preserves_original_cause() {
    let mut package = package(false);
    let (target, loan) = input(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.completion_index.insert(target.callee_owner(), Err(crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::SourceNavigation("callee-original-error".into())));
    assert!(ledger
        .checked_completed_object_callee_exits_v1(&target, &loan)
        .unwrap_err()
        .contains("callee-original-error"));
}
#[test]
fn completed_object_callee_missing_terminal_preserves_incomplete_whole_coverage() {
    let mut package = package(true);
    let (target, loan) = input(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let index = ledger
        .terminal_relation_index
        .get_mut(&target.callee_owner())
        .unwrap();
    let index = Rc::make_mut(index);
    let exit = index.keys().next().unwrap().clone();
    index.remove(&exit);
    assert!(ledger
        .checked_completed_object_callee_exits_v1(&target, &loan)
        .unwrap()
        .is_none());
}
#[test]
fn completed_object_callee_foreign_facts_qualification_refuses() {
    let foreign = package(true);
    let package = package(false);
    let (target, _) = input(&package);
    let (_, loan) = input(&foreign);
    assert!(package
        .ordinary_new_claim_ledger
        .checked_completed_object_callee_exits_v1(&target, &loan)
        .unwrap_err()
        .contains("callee-call-qualification"));
}

#[test]
fn completed_object_callee_shared_target_projection_retains_same_order_and_rc() {
    for nullable in [false, true] {
        let package = package(nullable);
        let (target, loan) = input(&package);
        let ledger = &package.ordinary_new_claim_ledger;
        let qualified = ledger
            .checked_completed_object_callee_exits_v1(&target, &loan)
            .unwrap()
            .unwrap();
        let target_only = ledger
            .checked_completed_object_callee_target_exits_v1(&target)
            .unwrap()
            .unwrap();
        assert_eq!(qualified.len(), target_only.len());
        for ((a, at), (b, bt)) in qualified.iter().zip(target_only.iter()) {
            assert!(Rc::ptr_eq(a, b));
            assert_eq!(at, bt);
        }
    }
}
