use super::*;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::resolved_semantics::SourcePathSegmentV1;

#[test]
fn direct_scalar_admission_requires_exact_i64() {
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
    for (representation, direct_i64) in [
        (
            VerifiedCallableResultRepresentationV1::ExactBool,
            false,
        ),
        (VerifiedCallableResultRepresentationV1::ExactI64, true),
        (
            VerifiedCallableResultRepresentationV1::ExactString,
            false,
        ),
        (
            VerifiedCallableResultRepresentationV1::ExactNominalBox { nullable: false,
                box_name: "Page".into(),
            },
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
    }
}
