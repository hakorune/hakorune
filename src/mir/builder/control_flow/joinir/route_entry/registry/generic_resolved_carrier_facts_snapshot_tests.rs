//! D3-S2-P2: neutral Generic carrier facts snapshot, test-only.
//!
//! The snapshot consumes one P1 provenance product and adds no Generic route,
//! policy, Recipe, Builder, MIR, or physical ownership meaning.

use super::generic_nested_carrier_bindingref_tests::{SHADOWING_SOURCE, SOURCE};
use super::generic_resolved_carrier_provenance_product_tests::handoff;
use crate::mir::loop_structural_facts::{
    issue_generic_resolved_carrier_facts_v1, ResolvedCarrierDispositionV1,
};
use crate::mir::resolved_semantics::generic_resolved_carrier_provenance::
    issue_resolved_carrier_provenance_v1;

#[test]
fn generic_d3_s2_p2_snapshot_consumes_one_p1_product() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let provenance = issue_resolved_carrier_provenance_v1(handoff(SOURCE))
        .expect("natural Both source must seal P1 provenance");
    let facts = issue_generic_resolved_carrier_facts_v1(provenance);
    assert_eq!(
        facts.disposition(),
        ResolvedCarrierDispositionV1::NestedWriteWithPostLoopRead
    );
}
#[test]
fn generic_d3_s2_p2_snapshot_preserves_p1_reject_boundary() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let result = issue_resolved_carrier_provenance_v1(handoff(SHADOWING_SOURCE))
        .map(issue_generic_resolved_carrier_facts_v1);
    assert!(result.is_err(), "shadowing must reject before snapshot issue");
}
