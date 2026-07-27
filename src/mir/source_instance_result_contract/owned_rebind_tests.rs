use std::sync::Arc;

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::VerifiedSourceMethodCallSiteV1;

use super::{
    rebind_nested_instance_result_contract_v1, NestedInstanceResultRebindErrorV1,
    NestedInstanceResultRebindStageV1, OwnedNestedInstanceResultRebindWitnessV1,
};
use crate::mir::source_instance_result_contract::{
    seal_nested_instance_result_contract, VerifiedCurrentOwnerInstanceResultTargetV1,
};

struct OwnedFixture {
    catalog: Arc<VerifiedSameModuleCallableDeclarationCatalogV1>,
    caller: CanonicalSameModuleCallableKeyV1,
    sites: [SourceExprSiteV1; 2],
    witness: OwnedNestedInstanceResultRebindWitnessV1,
}

fn owned_fixture() -> OwnedFixture {
    let (catalog, (caller, sites, witness)) =
        actual_parser_add_fixture::with_owned_stageb_carrier_correspondence_inputs(
            |catalog, caller, _outer_site, sites, _targets, results| {
                let call =
                    VerifiedSourceMethodCallSiteV1::verify(catalog, caller, sites[0].clone())
                        .expect("selected nested MethodCall");
                let target = VerifiedCurrentOwnerInstanceResultTargetV1::seal(&call)
                    .expect("same-owner instance target");
                let proof = results
                    .issue_unannotated_body_proof(target.target())
                    .expect("exact Integer proof");
                let contract = seal_nested_instance_result_contract(target, proof)
                    .expect("sealed nested Integer contract");
                (
                    caller.clone(),
                    sites.clone(),
                    contract.into_owned_rebind_witness(),
                )
            },
        );
    OwnedFixture {
        catalog,
        caller,
        sites,
        witness,
    }
}

fn verified_call<'catalog>(
    catalog: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    site: SourceExprSiteV1,
) -> VerifiedSourceMethodCallSiteV1<'catalog> {
    VerifiedSourceMethodCallSiteV1::verify(catalog, caller, site)
        .expect("exact rebound source MethodCall")
}

#[test]
fn rebinds_after_the_result_catalog_and_original_borrows_are_gone() {
    let fixture = owned_fixture();
    let call = verified_call(
        fixture.catalog.as_ref(),
        &fixture.caller,
        fixture.sites[0].clone(),
    );
    let contract =
        rebind_nested_instance_result_contract_v1(fixture.witness, &fixture.catalog, &call)
            .expect("same-allocation rebind");
    assert!(contract.result_is_integer());
    assert_eq!(contract.target().call().site(), &fixture.sites[0]);
}

#[test]
fn rejects_an_equal_looking_foreign_catalog_allocation() {
    let primary = owned_fixture();
    let foreign = owned_fixture();
    let call = verified_call(
        foreign.catalog.as_ref(),
        &primary.caller,
        primary.sites[0].clone(),
    );
    let rejected =
        rebind_nested_instance_result_contract_v1(primary.witness, &foreign.catalog, &call)
            .expect_err("foreign allocation");
    assert_eq!(
        rejected.stage(),
        NestedInstanceResultRebindStageV1::CatalogAllocation
    );
    assert_eq!(
        rejected.cause(),
        &NestedInstanceResultRebindErrorV1::ForeignCatalog
    );
    rejected.discard();
}

#[test]
fn rejects_caller_target_and_site_drift_without_new_inference() {
    let mut caller_drift = owned_fixture();
    caller_drift.witness.caller = caller_drift.witness.target.clone();
    let call = verified_call(
        caller_drift.catalog.as_ref(),
        &caller_drift.caller,
        caller_drift.sites[0].clone(),
    );
    let rejected = rebind_nested_instance_result_contract_v1(
        caller_drift.witness,
        &caller_drift.catalog,
        &call,
    )
    .expect_err("caller drift");
    assert_eq!(rejected.stage(), NestedInstanceResultRebindStageV1::Caller);
    assert_eq!(
        rejected.cause(),
        &NestedInstanceResultRebindErrorV1::CallerMismatch
    );
    rejected.discard();

    let mut target_drift = owned_fixture();
    target_drift.witness.target = target_drift.witness.caller.clone();
    let call = verified_call(
        target_drift.catalog.as_ref(),
        &target_drift.caller,
        target_drift.sites[0].clone(),
    );
    let rejected = rebind_nested_instance_result_contract_v1(
        target_drift.witness,
        &target_drift.catalog,
        &call,
    )
    .expect_err("target drift");
    assert_eq!(rejected.stage(), NestedInstanceResultRebindStageV1::Target);
    assert_eq!(
        rejected.cause(),
        &NestedInstanceResultRebindErrorV1::TargetMismatch
    );
    rejected.discard();

    let site_drift = owned_fixture();
    let call = verified_call(
        site_drift.catalog.as_ref(),
        &site_drift.caller,
        site_drift.sites[1].clone(),
    );
    let rejected =
        rebind_nested_instance_result_contract_v1(site_drift.witness, &site_drift.catalog, &call)
            .expect_err("site drift");
    assert_eq!(rejected.stage(), NestedInstanceResultRebindStageV1::Site);
    assert_eq!(
        rejected.cause(),
        &NestedInstanceResultRebindErrorV1::SiteMismatch
    );
    rejected.discard();
}
