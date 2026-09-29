//! Focused tests for the bounded return-position `new` claim family:
//! `return new <class>(...)` mints a destination-less claim keyed on the
//! exact `new` site, co-seals its destination-less result Home prefix and
//! argument observations, and keeps non-return positions outside the lane.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::resolved_semantics::home_new_prefix::{
    TerminalRelationV1, TerminalReturnedSourceV1,
};

/// Covered obligations admit install; the physical execution boundary
/// stays downstream at lowering.
fn assert_install_admits(package: super::VerifiedNormalCallableSemanticPackageV1) {
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

fn work_contract<'a>(
    package: &'a super::VerifiedNormalCallableSemanticPackageV1,
) -> super::result_contract::CallableResultContractRefV1<'a> {
    let owner = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == 1)
        .expect("Work::make declaration")
        .owner();
    package
        .result_contracts
        .rows()
        .find(|row| row.borrow().owner() == owner)
        .expect("Work::make contract row")
        .borrow()
}

#[test]
fn return_position_new_mints_result_claim_and_construction_relation() {
    let package = issue(
        "box Point { x: i64 y: i64 birth(x, y) { me.x = x me.y = y } }
         static box Work { make(args) { return new Point(1, 2) } }
         static box Main { main() { return 30 } }",
    )
    .expect("return-position new package");
    let contract = work_contract(&package);
    let Some(TerminalRelationV1::Value(relation)) = contract.sole_terminal_relation() else {
        panic!("return-position `new` issues a Value relation");
    };
    let TerminalReturnedSourceV1::Construction(site) = relation.returned() else {
        panic!("return-position `new` classifies as returned Construction");
    };
    assert_eq!(site.owner(), contract.owner());
    let result_claims = package
        .ordinary_new_claim_ledger
        .pending_result_claims_for_test();
    let claim = result_claims
        .get(site)
        .expect("return-position `new` mints a result claim at the exact site");
    assert_eq!(claim.class(), "Point");
    assert_eq!(claim.arity(), 2);
    assert!(
        claim.home_prefix().is_ok(),
        "trivial integer arguments issue a covered result prefix, got {:?}",
        claim.home_prefix()
    );
    assert!(
        claim.argument_rows().is_ok(),
        "trivial integer arguments co-seal their observations"
    );
    // The claim lane — not the destination-less birth index — owns the
    // site's `Birth` call edge.
    assert!(
        package
            .ordinary_new_claim_ledger
            .take_birth_site_recipe(site, "Point", 2)
            .expect("birth-site lookup")
            .is_none(),
        "result claims stay out of the destination-less birth index"
    );
    drop(result_claims);
    assert_install_admits(package);
}

#[test]
fn return_position_new_with_unavailable_arguments_stays_retained() {
    let package = issue(
        "box Point { x: i64 birth(x) { me.x = x } }
         static box Work { make(args) { return new Point(1 + 1) } }
         static box Main { main() { return 30 } }",
    )
    .expect("unavailable return-position new package");
    let result_claims = package
        .ordinary_new_claim_ledger
        .pending_result_claims_for_test();
    let claim = result_claims
        .values()
        .next()
        .expect("result claim is still minted");
    assert_eq!(claim.class(), "Point");
    assert!(
        claim.home_prefix().is_err() || claim.argument_rows().is_err(),
        "non-trivial argument keeps the claim truthfully unavailable"
    );
}

#[test]
fn retained_result_position_commit_completes_when_its_expression_is_recorded() {
    // A `return new` whose argument co-seal is retained still emits through
    // the raw lane; a destination-less row has no install step, so recording
    // the emitted expression value is its reachable terminal. A row whose
    // expression never ran stays incomplete — `IncompleteOrdinaryNewCoverage`
    // keeps that real gap.
    let package = issue(
        "box Point { x: i64 birth(x) { me.x = x } }
         static box Work { make(args) { return new Point(1 + 1) } }
         static box Main { main() { return 30 } }",
    )
    .expect("retained result-position package");
    let ledger = &package.ordinary_new_claim_ledger;
    let site = {
        let result_claims = ledger.pending_result_claims_for_test();
        result_claims.keys().next().expect("result claim").clone()
    };
    let claim = ledger
        .try_take_result(&site, "Point", 1)
        .expect("result take")
        .expect("result claim present");
    assert!(
        !ledger.prepare_result_new_emission(&claim).unwrap(),
        "non-trivial argument retains the claim"
    );
    assert!(
        !ledger.local_commit_complete_for_test(&site),
        "PendingExpression is still a coverage gap"
    );
    ledger
        .complete_new_expression(&site, "Point", crate::mir::ValueId(7))
        .expect("raw lane records the emitted expression");
    assert!(
        ledger.local_commit_complete_for_test(&site),
        "a retained result row completes at expression completion"
    );
}

#[test]
fn argument_position_new_mints_no_result_claim() {
    let package = issue(
        "box Point { x: i64 birth(x) { me.x = x } }
         static box Work { take(p) { return 0 } make(args) { local r = Work.take(new Point(1)) return r } }
         static box Main { main() { return 30 } }",
    )
    .expect("argument-position new package");
    assert!(
        package
            .ordinary_new_claim_ledger
            .pending_result_claims_for_test()
            .is_empty(),
        "argument-position `new` stays outside the result claim family"
    );
}

#[test]
fn field_position_new_mints_no_result_claim() {
    let package = issue(
        "box Inner { v: i64 birth(v) { me.v = v } }
         box Outer { inner: Inner birth() { me.inner = new Inner(7) } }
         static box Main { main() { local o = new Outer() return 0 } }",
    )
    .expect("field-position new package");
    assert!(
        package
            .ordinary_new_claim_ledger
            .pending_result_claims_for_test()
            .is_empty(),
        "field-position `new` stays outside the result claim family"
    );
}

#[test]
fn nested_if_return_position_new_claims_cover_each_exit() {
    // The per-exit home-flow scan walks branch bodies: each `return` site
    // names its own exit row, so a `new` nested under an `if` is covered
    // for its own exit instead of falling back to owner-level coverage.
    let package = issue(
        "box Point { x: i64 birth(x) { me.x = x } }
         static box Work { make(args) { if args { return new Point(1) } return new Point(2) } }
         static box Main { main() { return 30 } }",
    )
    .expect("nested return-position new package");
    let claims = package
        .ordinary_new_claim_ledger
        .pending_result_claims_for_test();
    assert_eq!(claims.len(), 2, "both `return` sites mint claims");
    for claim in claims.values() {
        assert!(
            claim.home_prefix().is_ok(),
            "each exit's `new` is covered at its own return site"
        );
    }
}

#[test]
fn assignment_position_new_mints_no_result_claim() {
    let package = issue(
        "box Point { x: i64 birth(x) { me.x = x } }
         static box Work { make(args) { local p = 0 p = new Point(1) return 0 } }
         static box Main { main() { return 30 } }",
    )
    .expect("assignment-position new package");
    assert!(
        package
            .ordinary_new_claim_ledger
            .pending_result_claims_for_test()
            .is_empty(),
        "assignment `Value` positions are not return membership"
    );
}
