use super::common_v2_initial_index_seed::{
    issue_s6c_v2_initial_index_seed_relation_v1, InitialIndexSeedRelationRejectV1,
};
use super::s6c_prephysical_ingress::issue_s6c_prephysical_ingress_v2;
use super::s6c_scan_with_init_tests::issue_facts;
use super::{issue_s6c_scan_with_init_logical_output_v1, produce_s6c_scan_with_init_recipe_v2};
use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

#[test]
fn initial_index_seed_rejects_foreign_owner_before_effect() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 979))
            .expect("exact S6C Recipe product"),
    )
    .expect("logical output rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("prephysical ingress");
    let mut owners = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
    let foreign = owners.issue().expect("foreign owner");

    ingress
        .with_ingress(|view| {
            assert_ne!(view.source_owner(), foreign);
            assert!(matches!(
                issue_s6c_v2_initial_index_seed_relation_v1(view, foreign),
                Err(InitialIndexSeedRelationRejectV1::ForeignOwner)
            ));
            Ok(())
        })
        .expect("foreign owner is rejected without effect");
}
