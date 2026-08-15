use super::s6c_scan_with_init_tests::issue_facts;
use super::{
    issue_s6c_prephysical_ingress_v2, issue_s6c_scan_with_init_logical_output_v1,
    issue_s6c_text_eq_source_binding_v1, produce_s6c_scan_with_init_recipe_v2, TextEqualityLawV1,
};
use crate::mir::callable_semantic_batch::{S6CBinaryRoleV1, S6CLogicalValueClassV1};
use crate::mir::resolved_semantics::{ResolvedBinaryOperatorV1, ResolvedLoopPlacementV1};

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

#[test]
fn text_eq_site_binding_retains_parent_and_lends_exact_site() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 952))
            .expect("exact S6C Recipe product"),
    )
    .expect("logical output rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("prephysical ingress");
    let binding = issue_s6c_text_eq_source_binding_v1(ingress);

    binding.with_site(|site| {
        assert_eq!(site.law(), TextEqualityLawV1::ExactUnicodeScalarSequence);
        assert_eq!(site.item().raw(), 7);
        assert_eq!(site.block().raw(), 1);
        assert_eq!(site.left().raw(), 9);
        assert_eq!(site.right().raw(), 1);
        assert_eq!(site.result().raw(), 10);
        assert_eq!(site.if_item().raw(), 8);
        assert_eq!(site.if_block().raw(), 1);
        assert_eq!(site.if_condition(), site.result());
        assert_eq!(site.if_then_block().raw(), 2);

        let source = site.source();
        assert_eq!(source.role(), S6CBinaryRoleV1::TextEqual);
        assert_eq!(source.placement(), ResolvedLoopPlacementV1::Body);
        assert_eq!(source.source().operator(), ResolvedBinaryOperatorV1::Equal);
        assert_eq!(source.result_class(), S6CLogicalValueClassV1::Bool);
    });

    binding.with_site(|site| {
        assert!(matches!(
            site.source().placement(),
            ResolvedLoopPlacementV1::Body
        ));
    });
}
