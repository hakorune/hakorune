use super::*;
use crate::mir::resolved_semantics::SourcePathSegmentV1;

#[test]
fn composite_scalar_admission_keeps_direct_i64_and_rejects_string_box() {
    let site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]));
    let item = CallableLoopSourceItemBindingV1 {
        call_site: site.clone(),
        receiver_site: site.clone(),
        argument_sites: Box::new([]),
        result_site: site.clone(),
        selector: "predicate".into(),
        arity: 0,
    };
    for (representation, accepted, direct_i64) in [
        (
            VerifiedCallableResultRepresentationV1::ExactBool,
            true,
            false,
        ),
        (VerifiedCallableResultRepresentationV1::ExactI64, true, true),
        (
            VerifiedCallableResultRepresentationV1::ExactString,
            false,
            false,
        ),
        (
            VerifiedCallableResultRepresentationV1::ExactNominalBox {
                box_name: "Page".into(),
            },
            false,
            false,
        ),
    ] {
        let relation = CallableLoopSourceTargetRelationV1::new(
            site.clone(),
            CanonicalSameModuleCallableKeyV1::test_static_box_method("Source", "predicate", 0),
            Some(CallableLoopSourceTargetRequirementV1 {
                representation,
                required_callee_i64_arguments: Box::new([]),
            }),
        );
        assert_eq!(relation.has_exact_i64_result(), direct_i64);
        let direct = CallableLoopSourceTargetProbeV1::from_parts(
            vec![relation.clone()].into(),
            Box::new([]),
            false,
        )
        .into_selected_relations(&[item.clone()]);
        assert_eq!(direct.is_ok(), direct_i64);
        let composite =
            CallableLoopSourceTargetProbeV1::from_parts(vec![relation].into(), Box::new([]), false)
                .into_item_dispositions(&[item.clone()]);
        if accepted {
            assert!(composite.is_ok());
        } else {
            assert!(matches!(
                composite,
                Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch)
            ));
        }
    }
}
