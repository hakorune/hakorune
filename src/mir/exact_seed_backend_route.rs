/*!
 * Function-level backend route tags for temporary exact seed bridges.
 *
 * Exact seed payload routes still own their detailed proof fields. This layer
 * only chooses which already-proven exact backend route the C boundary should
 * try first, so the function entry does not have to rediscover the ladder.
 */

use super::{MirFunction, MirModule};
use crate::mir::{
    StringKernelPlanConsumer, StringKernelPlanFamily, StringKernelPlanRetainedForm, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactSeedBackendRouteKind {
    ArrayStringStoreMicro,
    ConcatConstSuffixMicro,
    SubstringViewsOnlyMicro,
    SubstringConcatLoopAscii,
}

impl ExactSeedBackendRouteKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArrayStringStoreMicro => "array_string_store_micro",
            Self::ConcatConstSuffixMicro => "concat_const_suffix_micro",
            Self::SubstringViewsOnlyMicro => "substring_views_only_micro",
            Self::SubstringConcatLoopAscii => "substring_concat_loop_ascii",
        }
    }

    pub fn source_route_field(self) -> &'static str {
        match self {
            Self::ArrayStringStoreMicro => "array_string_store_micro_seed_route",
            Self::ConcatConstSuffixMicro => "concat_const_suffix_micro_seed_route",
            Self::SubstringViewsOnlyMicro => "substring_views_micro_seed_route",
            Self::SubstringConcatLoopAscii => "string_kernel_plans.loop_payload",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactSeedBackendRoute {
    pub tag: ExactSeedBackendRouteKind,
    pub source_route: String,
    pub proof: String,
    pub selected_value: Option<ValueId>,
}

pub fn refresh_module_exact_seed_backend_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_exact_seed_backend_route(function);
    }
}

pub fn refresh_function_exact_seed_backend_route(function: &mut MirFunction) {
    function.metadata.exact_seed_backend_route = match_exact_seed_backend_route(function);
}

fn match_exact_seed_backend_route(function: &MirFunction) -> Option<ExactSeedBackendRoute> {
    if let Some(route) = function
        .metadata
        .array_string_store_micro_seed_route
        .as_ref()
    {
        return Some(ExactSeedBackendRoute {
            tag: ExactSeedBackendRouteKind::ArrayStringStoreMicro,
            source_route: ExactSeedBackendRouteKind::ArrayStringStoreMicro
                .source_route_field()
                .to_string(),
            proof: route.proof.to_string(),
            selected_value: None,
        });
    }

    function
        .metadata
        .concat_const_suffix_micro_seed_route
        .as_ref()
        .map(|route| ExactSeedBackendRoute {
            tag: ExactSeedBackendRouteKind::ConcatConstSuffixMicro,
            source_route: ExactSeedBackendRouteKind::ConcatConstSuffixMicro
                .source_route_field()
                .to_string(),
            proof: route.proof.to_string(),
            selected_value: None,
        })
        .or_else(|| {
            function
                .metadata
                .substring_views_micro_seed_route
                .as_ref()
                .map(|route| ExactSeedBackendRoute {
                    tag: ExactSeedBackendRouteKind::SubstringViewsOnlyMicro,
                    source_route: ExactSeedBackendRouteKind::SubstringViewsOnlyMicro
                        .source_route_field()
                        .to_string(),
                    proof: route.proof.to_string(),
                    selected_value: None,
                })
        })
        .or_else(|| {
            selected_substring_concat_loop_plan(function).map(|selected_value| {
                ExactSeedBackendRoute {
                    tag: ExactSeedBackendRouteKind::SubstringConcatLoopAscii,
                    source_route: ExactSeedBackendRouteKind::SubstringConcatLoopAscii
                        .source_route_field()
                        .to_string(),
                    proof: "string_kernel_plan_concat_triplet_loop_payload".to_string(),
                    selected_value: Some(selected_value),
                }
            })
        })
}

fn selected_substring_concat_loop_plan(function: &MirFunction) -> Option<ValueId> {
    function
        .metadata
        .string_kernel_plans
        .iter()
        .find_map(|(value, plan)| {
            (plan.family == StringKernelPlanFamily::ConcatTripletWindow
                && plan.corridor_root == *value
                && plan.consumer == Some(StringKernelPlanConsumer::DirectKernelEntry)
                && plan.retained_form == StringKernelPlanRetainedForm::BorrowedText
                && plan.loop_payload.is_some())
            .then_some(*value)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::string_kernel_plan::StringKernelPlanLoopPayload;
    use crate::mir::{
        ArrayStringStoreMicroSeedProof, ArrayStringStoreMicroSeedRoute,
        ConcatConstSuffixMicroSeedProof, ConcatConstSuffixMicroSeedRoute, EffectMask,
        FunctionSignature, MirType, StringKernelPlan, StringKernelPlanBorrowContract,
        StringKernelPlanConsumer, StringKernelPlanFamily, StringKernelPlanPublicationBoundary,
        StringKernelPlanPublicationContract, StringKernelPlanRetainedForm,
        StringKernelPlanVerifierOwner, SubstringViewsMicroSeedProof, SubstringViewsMicroSeedRoute,
    };
    use hakorune_mir_core::BasicBlockId;

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn exact_seed_backend_route_selects_array_string_store_metadata() {
        let mut function = make_function();
        function.metadata.array_string_store_micro_seed_route =
            Some(ArrayStringStoreMicroSeedRoute {
                seed: "line-seed-abcdef".to_string(),
                seed_len: 16,
                size: 128,
                ops: 800000,
                suffix: "xy".to_string(),
                store_len: 18,
                next_text_window_start: 2,
                next_text_window_len: 16,
                proof: ArrayStringStoreMicroSeedProof::KiloMicroArrayStringStore8Block,
            });

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "array_string_store_micro");
        assert_eq!(route.source_route, "array_string_store_micro_seed_route");
        assert_eq!(route.proof, "kilo_micro_array_string_store_8block");
        assert_eq!(route.selected_value, None);
    }

    #[test]
    fn exact_seed_backend_route_selects_concat_const_suffix_metadata() {
        let mut function = make_function();
        function.metadata.concat_const_suffix_micro_seed_route =
            Some(ConcatConstSuffixMicroSeedRoute {
                seed: "line-seed-abcdef".to_string(),
                seed_len: 16,
                suffix: "xy".to_string(),
                suffix_len: 2,
                ops: 600000,
                result_len: 18,
                proof: ConcatConstSuffixMicroSeedProof::KiloMicroConcatConstSuffix5Block,
            });

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "concat_const_suffix_micro");
        assert_eq!(route.source_route, "concat_const_suffix_micro_seed_route");
        assert_eq!(route.proof, "kilo_micro_concat_const_suffix_5block");
        assert_eq!(route.selected_value, None);
    }

    #[test]
    fn exact_seed_backend_route_selects_substring_views_metadata() {
        let mut function = make_function();
        function.metadata.substring_views_micro_seed_route = Some(SubstringViewsMicroSeedRoute {
            source: "line-seed-abcdef".to_string(),
            source_len: 16,
            loop_bound: 300000,
            proof: SubstringViewsMicroSeedProof::KiloMicroSubstringViewsOnly5Block,
        });

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "substring_views_only_micro");
        assert_eq!(route.source_route, "substring_views_micro_seed_route");
        assert_eq!(route.proof, "kilo_micro_substring_views_only_5block");
        assert_eq!(route.selected_value, None);
    }

    #[test]
    fn exact_seed_backend_route_selects_substring_concat_loop_payload() {
        let mut function = make_function();
        function.metadata.string_kernel_plans.insert(
            ValueId::new(35),
            substring_concat_loop_plan(ValueId::new(35)),
        );

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "substring_concat_loop_ascii");
        assert_eq!(route.source_route, "string_kernel_plans.loop_payload");
        assert_eq!(
            route.proof,
            "string_kernel_plan_concat_triplet_loop_payload"
        );
        assert_eq!(route.selected_value, Some(ValueId::new(35)));
    }

    #[test]
    fn exact_seed_backend_route_stays_absent_without_seed_route() {
        let mut function = make_function();

        refresh_function_exact_seed_backend_route(&mut function);

        assert!(function.metadata.exact_seed_backend_route.is_none());
    }

    fn substring_concat_loop_plan(plan_value: ValueId) -> StringKernelPlan {
        StringKernelPlan {
            plan_value,
            version: 1,
            family: StringKernelPlanFamily::ConcatTripletWindow,
            corridor_root: plan_value,
            source_root: Some(ValueId::new(20)),
            borrow_contract: Some(StringKernelPlanBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            known_length: Some(2),
            retained_form: StringKernelPlanRetainedForm::BorrowedText,
            publication_boundary: Some(StringKernelPlanPublicationBoundary::FirstExternalBoundary),
            publication_contract: Some(
                StringKernelPlanPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
            ),
            publication: None,
            materialization: None,
            direct_kernel_entry: Some(crate::mir::StringCorridorCandidateState::Candidate),
            consumer: Some(StringKernelPlanConsumer::DirectKernelEntry),
            text_consumer: None,
            carrier: None,
            verifier_owner: Some(StringKernelPlanVerifierOwner::LoweringDirectKernelEntry),
            read_alias: Default::default(),
            slot_hop_substring: None,
            proof: crate::mir::string_corridor_placement::StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(25)),
                left_source: ValueId::new(20),
                left_start: ValueId::new(46),
                left_end: ValueId::new(47),
                middle: ValueId::new(66),
                right_value: Some(ValueId::new(26)),
                right_source: ValueId::new(20),
                right_start: ValueId::new(47),
                right_end: ValueId::new(5),
                shared_source: true,
            },
            middle_literal: Some("xx".to_string()),
            loop_payload: Some(StringKernelPlanLoopPayload {
                seed_value: ValueId::new(3),
                seed_literal: "line-seed-abcdef".to_string(),
                seed_length: 16,
                loop_bound: 300000,
                split_length: 8,
            }),
        }
    }
}
