//! Entry-only instance Home loan: the sole Home ABI issuer lends verified
//! receiver/parameter demands to source Home Flow for the exact admitted
//! `InstanceBoxMethod` cohort only. These tests pin both directions through
//! the real issuer and the real `scan_new_home_flow` consumer: a loan for
//! the exact declaration admits the entry; absent or foreign evidence keeps
//! the named `EntryDemandMissing` unavailability.

use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::ast::ASTNode;
use crate::mir::callable_parameter_contract::issue_callable_parameter_contract_v1;
use crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    issue_new_home_prefixes_v1, HomePrefixUnavailableV1,
};
use crate::mir::resolved_semantics::{
    CallableHomeAbiIssuerV1, OwnedExprSiteV1, VerifiedInstanceEntryHomeLoanV1,
    VerifiedInstanceEntryHomeParameterV1,
};
use std::collections::BTreeMap;

/// One instance box: `birth()` plus `make(n)` whose body installs a local
/// `new`. The static `main` gives the batch a non-instance sibling.
const SOURCE: &str = "box Node {
    value: i64
    make(n) { local child = new Node()
return 0 }
}
box Other {
    value: i64
    touch(k) { me.value = k }
}
static box Main { main() { local n = new Node()
return 0 } }";

fn entry_loans(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
) -> crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1 {
    let catalog =
        issue_callable_parameter_contract_v1(package.batch()).expect("common parameter catalog");
    CallableHomeAbiIssuerV1::issue_source_entry_home_catalog_v1(package.batch(), &catalog)
        .expect("exact instance entry cohort")
}

fn make_slot(package: &super::VerifiedNormalCallableSemanticPackageV1) -> u32 {
    package
        .batch()
        .declarations()
        .find(|row| {
            row.mode() == ResolvedCallableDeclarationModeV1::InstanceBoxMethod
                && row.parameter_count() == 1
        })
        .map(|row| row.batch_slot())
        .expect("make declaration")
}

fn static_slot(package: &super::VerifiedNormalCallableSemanticPackageV1) -> u32 {
    package
        .batch()
        .declarations()
        .find(|row| row.mode() == ResolvedCallableDeclarationModeV1::StaticBoxMethod)
        .map(|row| row.batch_slot())
        .expect("static main declaration")
}

/// Per-site `new` prefixes for one batch slot under the given entry loan.
fn new_prefixes(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
    batch_slot: u32,
    entry_home: Option<&VerifiedInstanceEntryHomeLoanV1>,
) -> BTreeMap<
    OwnedExprSiteV1,
    Result<
        crate::mir::resolved_semantics::home_new_prefix::CallerNewHomePrefixV1,
        HomePrefixUnavailableV1,
    >,
> {
    package
        .batch()
        .with_lowering_input(batch_slot, |input| {
            let owner = input.owner();
            let function = input.function();
            let mut new_sites = BTreeMap::new();
            for initializer in function.expression_source().initializers() {
                let Some(initializer_site) = initializer.initializer_site() else {
                    continue;
                };
                if !matches!(
                    initializer_site.node().segments(),
                    [
                        crate::mir::resolved_semantics::SourcePathSegmentV1::Body(_),
                        crate::mir::resolved_semantics::SourcePathSegmentV1::Initializer(_)
                    ]
                ) {
                    continue;
                }
                let site = OwnedExprSiteV1::new(owner, initializer_site.clone());
                let Ok(located) = input.source().expr_at(&site) else {
                    continue;
                };
                if matches!(located.node(), ASTNode::New { .. }) {
                    new_sites.insert(site, initializer.binding());
                }
            }
            issue_new_home_prefixes_v1(input, &new_sites, entry_home)
        })
        .expect("batch slot lowering input")
}

fn assert_entry_demand_missing(
    prefixes: &BTreeMap<
        OwnedExprSiteV1,
        Result<
            crate::mir::resolved_semantics::home_new_prefix::CallerNewHomePrefixV1,
            HomePrefixUnavailableV1,
        >,
    >,
) {
    assert!(!prefixes.is_empty(), "fixture must carry a `new` site");
    assert!(
        prefixes
            .values()
            .all(|row| matches!(row, Err(HomePrefixUnavailableV1::EntryDemandMissing))),
        "entry demand stays unavailable without exact loan evidence: {prefixes:?}"
    );
}

#[test]
fn exact_instance_entry_loan_lends_receiver_and_parameter_demands() {
    let package = issue(SOURCE).expect("instance entry package");
    let loans = entry_loans(&package);
    let slot = make_slot(&package);
    let loan = loans.for_batch_slot(slot).expect("make entry loan");
    let prefixes = new_prefixes(&package, slot, Some(loan));
    assert!(!prefixes.is_empty(), "fixture must carry a `new` site");
    assert!(
        prefixes.values().all(|row| row.is_ok()),
        "lent receiver/parameter demands admit the entry home flow: {prefixes:?}"
    );
}

#[test]
fn missing_entry_evidence_keeps_receiver_demand_unavailable() {
    let package = issue(SOURCE).expect("instance entry package");
    assert_entry_demand_missing(&new_prefixes(&package, make_slot(&package), None));
}

#[test]
fn static_and_top_level_declarations_receive_no_entry_loan() {
    let package = issue(SOURCE).expect("instance entry package");
    let loans = entry_loans(&package);
    assert!(loans.for_batch_slot(static_slot(&package)).is_none());
}

#[test]
fn foreign_cohort_loan_is_rejected_by_the_exact_declaration() {
    let package = issue(SOURCE).expect("instance entry package");
    let loans = entry_loans(&package);
    let slot = make_slot(&package);
    // `Other.touch` is the sibling `InstanceBoxMethod` row: its loan is real
    // cohort evidence but foreign to `make`'s declaration.
    let other_slot = package
        .batch()
        .declarations()
        .find(|row| {
            row.mode() == ResolvedCallableDeclarationModeV1::InstanceBoxMethod
                && row.parameter_count() == 1
                && row.batch_slot() != slot
        })
        .map(|row| row.batch_slot())
        .expect("sibling instance declaration");
    let foreign = loans
        .for_batch_slot(other_slot)
        .expect("sibling instance entry loan");
    assert!(foreign.owner() != loans.for_batch_slot(slot).unwrap().owner());
    assert_entry_demand_missing(&new_prefixes(&package, slot, Some(foreign)));
}

#[test]
fn foreign_owner_metadata_is_rejected_by_the_exact_declaration() {
    let package = issue(SOURCE).expect("instance entry package");
    let loans = entry_loans(&package);
    let slot = make_slot(&package);
    let loan = loans.for_batch_slot(slot).expect("make entry loan");
    let foreign_owner = package
        .batch()
        .declarations()
        .find(|row| row.owner() != loan.owner())
        .map(|row| row.owner())
        .expect("foreign owner");
    let forged = VerifiedInstanceEntryHomeLoanV1::issue(
        slot,
        loan.identity().clone(),
        foreign_owner,
        loan.mode(),
        loan.receiver(),
        loan.parameters()
            .iter()
            .map(|row| {
                VerifiedInstanceEntryHomeParameterV1::issue(
                    row.ordinal(),
                    row.binding(),
                    row.kind(),
                    row.demand(),
                )
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    assert_entry_demand_missing(&new_prefixes(&package, slot, Some(&forged)));
}

#[test]
fn wrong_receiver_binding_is_rejected_by_the_exact_declaration() {
    let package = issue(SOURCE).expect("instance entry package");
    let loans = entry_loans(&package);
    let slot = make_slot(&package);
    let loan = loans.for_batch_slot(slot).expect("make entry loan");
    let parameter_binding = loan.parameters()[0].binding();
    let forged = VerifiedInstanceEntryHomeLoanV1::issue(
        slot,
        loan.identity().clone(),
        loan.owner(),
        loan.mode(),
        parameter_binding,
        loan.parameters()
            .iter()
            .map(|row| {
                VerifiedInstanceEntryHomeParameterV1::issue(
                    row.ordinal(),
                    row.binding(),
                    row.kind(),
                    row.demand(),
                )
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    assert_entry_demand_missing(&new_prefixes(&package, slot, Some(&forged)));
}

#[test]
fn duplicate_and_incomplete_parameter_rows_are_rejected() {
    let package = issue(SOURCE).expect("instance entry package");
    let loans = entry_loans(&package);
    let slot = make_slot(&package);
    let loan = loans.for_batch_slot(slot).expect("make entry loan");
    let row = &loan.parameters()[0];
    for parameters in [
        vec![
            VerifiedInstanceEntryHomeParameterV1::issue(
                row.ordinal(),
                row.binding(),
                row.kind(),
                row.demand(),
            ),
            VerifiedInstanceEntryHomeParameterV1::issue(
                row.ordinal(),
                row.binding(),
                row.kind(),
                row.demand(),
            ),
        ],
        vec![],
    ] {
        let forged = VerifiedInstanceEntryHomeLoanV1::issue(
            slot,
            loan.identity().clone(),
            loan.owner(),
            loan.mode(),
            loan.receiver(),
            parameters.into_boxed_slice(),
        );
        assert_entry_demand_missing(&new_prefixes(&package, slot, Some(&forged)));
    }
}
