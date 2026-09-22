use super::*;
use crate::mir::builder::normal_callable_loop_source_route::{
    CallableLoopSourceItemBindingV1, CallableLoopSourceTargetProbeV1,
    CallableLoopSourceTargetRelationV1, CallableLoopSourceTargetRequirementV1,
};
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};

fn source_site() -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]))
}

fn source_item(site: SourceExprSiteV1) -> CallableLoopSourceItemBindingV1 {
    CallableLoopSourceItemBindingV1::for_test(site)
}

fn relation(
    site: SourceExprSiteV1,
    representation: VerifiedCallableResultRepresentationV1,
) -> CallableLoopSourceTargetRelationV1 {
    CallableLoopSourceTargetRelationV1::new(
        site,
        CanonicalSameModuleCallableKeyV1::test_static_box_method("ArrayBox", "push", 1),
        Some(CallableLoopSourceTargetRequirementV1::for_test(
            representation,
        )),
    )
}

#[test]
fn source_evidence_rejects_missing_selected_target_before_disposition() {
    let site = source_site();
    let error = CallableLoopSourceTargetProbeV1::from_parts(
        Box::new([]),
        vec![site.clone()].into_boxed_slice(),
        false,
    )
    .into_item_dispositions(&[source_item(site.clone())])
    .expect_err("missing selected source target must stay named");
    assert!(matches!(
        error,
        CallableLoopSourceRouteRejectV1::SourceTargetUnselected { call_sites }
            if call_sites.as_ref() == [site]
    ));
}

#[test]
fn source_evidence_rejects_duplicate_selected_target_before_consumption() {
    let site = source_site();
    let selected = relation(
        site.clone(),
        VerifiedCallableResultRepresentationV1::ExactI64,
    );
    let error = CallableLoopSourceTargetProbeV1::from_parts(
        vec![selected.clone(), selected].into_boxed_slice(),
        Box::new([]),
        false,
    )
    .into_item_dispositions(&[source_item(site.clone())])
    .expect_err("duplicate selected source target must stay named");
    assert!(matches!(
        error,
        CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site }
            if call_site == site
    ));
}

#[test]
fn source_evidence_rejects_value_demanded_target_without_scalar_contract() {
    let site = source_site();
    let error = CallableLoopSourceTargetProbeV1::from_parts(
        vec![relation(
            site.clone(),
            VerifiedCallableResultRepresentationV1::ExactNominalBox {
                box_name: "Text".into(),
            },
        )]
        .into_boxed_slice(),
        Box::new([]),
        false,
    )
    .into_item_dispositions(&[source_item(site)])
    .expect_err("value-demanded target must not pass a scalar source contract");
    assert_eq!(
        error,
        CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch
    );
}
