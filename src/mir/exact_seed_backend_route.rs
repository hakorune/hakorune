/*!
 * Function-level backend route tags for temporary exact seed bridges.
 *
 * Exact seed payload routes still own their detailed proof fields. This layer
 * only chooses which already-proven exact backend route the C boundary should
 * try first, so the function entry does not have to rediscover the ladder.
 */

use super::{MirFunction, MirModule};
use crate::mir::{
    StringKernelPlanConsumer, StringKernelPlanFamily, StringKernelPlanRetainedForm,
    UserBoxLocalScalarSeedKind, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExactSeedBackendRouteKind {
    ArrayStringStoreMicro,
    ArrayGetSetMicro,
    ArrayRmwAdd1Leaf,
    ConcatConstSuffixMicro,
    SubstringViewsOnlyMicro,
    SubstringConcatLoopAscii,
    SumVariantTagLocal,
    SumVariantProjectLocal,
    UserBoxPointLocalScalar,
    UserBoxFlagPointFLocalScalar,
    UserBoxLoopMicro,
    UserBoxKnownReceiverMethodSeed,
}

impl ExactSeedBackendRouteKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::ArrayStringStoreMicro => "array_string_store_micro",
            Self::ArrayGetSetMicro => "array_getset_micro",
            Self::ArrayRmwAdd1Leaf => "array_rmw_add1_leaf",
            Self::ConcatConstSuffixMicro => "concat_const_suffix_micro",
            Self::SubstringViewsOnlyMicro => "substring_views_only_micro",
            Self::SubstringConcatLoopAscii => "substring_concat_loop_ascii",
            Self::SumVariantTagLocal => "sum_variant_tag_local",
            Self::SumVariantProjectLocal => "sum_variant_project_local",
            Self::UserBoxPointLocalScalar => "userbox_point_local_scalar",
            Self::UserBoxFlagPointFLocalScalar => "userbox_flag_pointf_local_scalar",
            Self::UserBoxLoopMicro => "userbox_loop_micro",
            Self::UserBoxKnownReceiverMethodSeed => "userbox_known_receiver_method_seed",
        }
    }

    fn source_route_field(self) -> &'static str {
        match self {
            Self::ArrayStringStoreMicro => "array_string_store_micro_seed_route",
            Self::ArrayGetSetMicro => "array_getset_micro_seed_route",
            Self::ArrayRmwAdd1Leaf => "array_rmw_add1_leaf_seed_route",
            Self::ConcatConstSuffixMicro => "concat_const_suffix_micro_seed_route",
            Self::SubstringViewsOnlyMicro => "substring_views_micro_seed_route",
            Self::SubstringConcatLoopAscii => "string_kernel_plans.loop_payload",
            Self::SumVariantTagLocal => "sum_variant_tag_seed_route",
            Self::SumVariantProjectLocal => "sum_variant_project_seed_route",
            Self::UserBoxPointLocalScalar => "userbox_local_scalar_seed_route",
            Self::UserBoxFlagPointFLocalScalar => "userbox_local_scalar_seed_route",
            Self::UserBoxLoopMicro => "userbox_loop_micro_seed_route",
            Self::UserBoxKnownReceiverMethodSeed => "userbox_known_receiver_method_seed_route",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactSeedBackendRoute {
    tag: ExactSeedBackendRouteKind,
    source_route: String,
    proof: String,
    selected_value: Option<ValueId>,
}

impl ExactSeedBackendRoute {
    pub fn tag(&self) -> &'static str {
        self.tag.as_str()
    }

    pub fn source_route(&self) -> &str {
        &self.source_route
    }

    pub fn proof(&self) -> &str {
        &self.proof
    }

    pub fn selected_value(&self) -> Option<ValueId> {
        self.selected_value
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::{ExactSeedBackendRoute, ExactSeedBackendRouteKind};
    use crate::mir::ValueId;

    fn fixture(
        tag: ExactSeedBackendRouteKind,
        proof: &str,
        selected_value: Option<ValueId>,
    ) -> ExactSeedBackendRoute {
        ExactSeedBackendRoute {
            tag,
            source_route: tag.source_route_field().to_string(),
            proof: proof.to_string(),
            selected_value,
        }
    }

    pub(crate) fn array_string_store_micro() -> ExactSeedBackendRoute {
        fixture(
            ExactSeedBackendRouteKind::ArrayStringStoreMicro,
            "kilo_micro_array_string_store_8block",
            None,
        )
    }

    pub(crate) fn concat_const_suffix_micro() -> ExactSeedBackendRoute {
        fixture(
            ExactSeedBackendRouteKind::ConcatConstSuffixMicro,
            "kilo_micro_concat_const_suffix_5block",
            None,
        )
    }

    pub(crate) fn array_rmw_add1_leaf() -> ExactSeedBackendRoute {
        fixture(
            ExactSeedBackendRouteKind::ArrayRmwAdd1Leaf,
            "kilo_leaf_array_rmw_add1_7block",
            None,
        )
    }

    pub(crate) fn sum_variant_tag_local() -> ExactSeedBackendRoute {
        fixture(
            ExactSeedBackendRouteKind::SumVariantTagLocal,
            "sum_variant_tag_local_aggregate_seed",
            None,
        )
    }

    pub(crate) fn sum_variant_project_local() -> ExactSeedBackendRoute {
        fixture(
            ExactSeedBackendRouteKind::SumVariantProjectLocal,
            "sum_variant_project_local_aggregate_seed",
            None,
        )
    }

    pub(crate) fn userbox_point_local_scalar() -> ExactSeedBackendRoute {
        fixture(
            ExactSeedBackendRouteKind::UserBoxPointLocalScalar,
            "userbox_point_field_local_scalar_seed",
            None,
        )
    }

    pub(crate) fn userbox_flag_pointf_local_scalar() -> ExactSeedBackendRoute {
        fixture(
            ExactSeedBackendRouteKind::UserBoxFlagPointFLocalScalar,
            "userbox_pointf_field_local_scalar_seed",
            None,
        )
    }

    pub(crate) fn substring_views_only_micro() -> ExactSeedBackendRoute {
        fixture(
            ExactSeedBackendRouteKind::SubstringViewsOnlyMicro,
            "kilo_micro_substring_views_only_5block",
            None,
        )
    }

    pub(crate) fn substring_concat_loop_ascii(selected_value: ValueId) -> ExactSeedBackendRoute {
        fixture(
            ExactSeedBackendRouteKind::SubstringConcatLoopAscii,
            "string_kernel_plan_concat_triplet_loop_payload",
            Some(selected_value),
        )
    }
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
            proof: route.proof().to_string(),
            selected_value: None,
        });
    }

    if let Some(route) = function.metadata.array_getset_micro_seed_route.as_ref() {
        return Some(ExactSeedBackendRoute {
            tag: ExactSeedBackendRouteKind::ArrayGetSetMicro,
            source_route: ExactSeedBackendRouteKind::ArrayGetSetMicro
                .source_route_field()
                .to_string(),
            proof: route.proof().to_string(),
            selected_value: None,
        });
    }

    if let Some(route) = function.metadata.array_rmw_add1_leaf_seed_route.as_ref() {
        return Some(ExactSeedBackendRoute {
            tag: ExactSeedBackendRouteKind::ArrayRmwAdd1Leaf,
            source_route: ExactSeedBackendRouteKind::ArrayRmwAdd1Leaf
                .source_route_field()
                .to_string(),
            proof: route.proof().to_string(),
            selected_value: None,
        });
    }

    if let Some(route) = function.metadata.sum_variant_tag_seed_route.as_ref() {
        return Some(ExactSeedBackendRoute {
            tag: ExactSeedBackendRouteKind::SumVariantTagLocal,
            source_route: ExactSeedBackendRouteKind::SumVariantTagLocal
                .source_route_field()
                .to_string(),
            proof: route.proof().to_string(),
            selected_value: None,
        });
    }

    if let Some(route) = function.metadata.sum_variant_project_seed_route.as_ref() {
        return Some(ExactSeedBackendRoute {
            tag: ExactSeedBackendRouteKind::SumVariantProjectLocal,
            source_route: ExactSeedBackendRouteKind::SumVariantProjectLocal
                .source_route_field()
                .to_string(),
            proof: route.proof().to_string(),
            selected_value: None,
        });
    }

    if let Some(route) = function.metadata.userbox_local_scalar_seed_route.as_ref() {
        let tag = match route.kind() {
            UserBoxLocalScalarSeedKind::PointLocalI64
            | UserBoxLocalScalarSeedKind::PointCopyLocalI64 => {
                ExactSeedBackendRouteKind::UserBoxPointLocalScalar
            }
            UserBoxLocalScalarSeedKind::FlagLocalBool
            | UserBoxLocalScalarSeedKind::FlagCopyLocalBool
            | UserBoxLocalScalarSeedKind::PointFLocalF64
            | UserBoxLocalScalarSeedKind::PointFCopyLocalF64 => {
                ExactSeedBackendRouteKind::UserBoxFlagPointFLocalScalar
            }
        };
        return Some(ExactSeedBackendRoute {
            tag,
            source_route: tag.source_route_field().to_string(),
            proof: route.proof().to_string(),
            selected_value: None,
        });
    }

    if let Some(route) = function.metadata.userbox_loop_micro_seed_route.as_ref() {
        let tag = ExactSeedBackendRouteKind::UserBoxLoopMicro;
        return Some(ExactSeedBackendRoute {
            tag,
            source_route: tag.source_route_field().to_string(),
            proof: route.proof.to_string(),
            selected_value: None,
        });
    }

    if let Some(route) = function
        .metadata
        .userbox_known_receiver_method_seed_route
        .as_ref()
    {
        let tag = ExactSeedBackendRouteKind::UserBoxKnownReceiverMethodSeed;
        return Some(ExactSeedBackendRoute {
            tag,
            source_route: tag.source_route_field().to_string(),
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
            proof: route.proof().to_string(),
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
                    proof: route.proof().to_string(),
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
        EffectMask, FunctionSignature, MirType, StringKernelPlan, StringKernelPlanBorrowContract,
        StringKernelPlanConsumer, StringKernelPlanFamily, StringKernelPlanPublicationBoundary,
        StringKernelPlanPublicationContract, StringKernelPlanRetainedForm,
        StringKernelPlanVerifierOwner, UserBoxKnownReceiverMethodSeedKind,
        UserBoxKnownReceiverMethodSeedPayload, UserBoxKnownReceiverMethodSeedProof,
        UserBoxKnownReceiverMethodSeedRoute, UserBoxLoopMicroSeedKind, UserBoxLoopMicroSeedProof,
        UserBoxLoopMicroSeedRoute,
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
        function.metadata.array_string_store_micro_seed_route = Some(
            crate::mir::array_string_store_micro_seed_plan::test_support::kilo_micro_array_string_store_8block(),
        );

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
        function.metadata.concat_const_suffix_micro_seed_route = Some(
            crate::mir::concat_const_suffix_micro_seed_plan::test_support::kilo_micro_concat_const_suffix_5block(),
        );

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
    fn exact_seed_backend_route_selects_array_rmw_add1_leaf_metadata() {
        let mut function = make_function();
        function.metadata.array_rmw_add1_leaf_seed_route = Some(
            crate::mir::array_rmw_add1_leaf_seed_plan::test_support::kilo_leaf_array_rmw_add1_7block(),
        );

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "array_rmw_add1_leaf");
        assert_eq!(route.source_route, "array_rmw_add1_leaf_seed_route");
        assert_eq!(route.proof, "kilo_leaf_array_rmw_add1_7block");
        assert_eq!(route.selected_value, None);
    }

    #[test]
    fn exact_seed_backend_route_selects_array_getset_micro_metadata() {
        let mut function = make_function();
        function.metadata.array_getset_micro_seed_route = Some(
            crate::mir::array_getset_micro_seed_plan::test_support::kilo_micro_array_getset_7block(
            ),
        );

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "array_getset_micro");
        assert_eq!(route.source_route, "array_getset_micro_seed_route");
        assert_eq!(route.proof, "kilo_micro_array_getset_7block");
        assert_eq!(route.selected_value, None);
    }

    #[test]
    fn exact_seed_backend_route_selects_sum_variant_tag_metadata() {
        let mut function = make_function();
        function.metadata.sum_variant_tag_seed_route =
            Some(crate::mir::sum_variant_tag_seed_plan::test_support::local_i64_result_ok());

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "sum_variant_tag_local");
        assert_eq!(route.source_route, "sum_variant_tag_seed_route");
        assert_eq!(route.proof, "sum_variant_tag_local_aggregate_seed");
        assert_eq!(route.selected_value, None);
    }

    #[test]
    fn exact_seed_backend_route_selects_sum_variant_project_metadata() {
        let mut function = make_function();
        function.metadata.sum_variant_project_seed_route = Some(
            crate::mir::sum_variant_project_seed_plan::test_support::local_i64_result_int_ok(),
        );

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "sum_variant_project_local");
        assert_eq!(route.source_route, "sum_variant_project_seed_route");
        assert_eq!(route.proof, "sum_variant_project_local_aggregate_seed");
        assert_eq!(route.selected_value, None);
    }

    #[test]
    fn exact_seed_backend_route_selects_userbox_point_local_scalar_metadata() {
        let mut function = make_function();
        function.metadata.userbox_local_scalar_seed_route =
            Some(crate::mir::userbox_local_scalar_seed_plan::test_support::point_local_i64());

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "userbox_point_local_scalar");
        assert_eq!(route.source_route, "userbox_local_scalar_seed_route");
        assert_eq!(route.proof, "userbox_point_field_local_scalar_seed");
        assert_eq!(route.selected_value, None);
    }

    #[test]
    fn exact_seed_backend_route_selects_userbox_flag_pointf_local_scalar_metadata() {
        let mut function = make_function();
        function.metadata.userbox_local_scalar_seed_route =
            Some(crate::mir::userbox_local_scalar_seed_plan::test_support::flag_copy_local_bool());

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "userbox_flag_pointf_local_scalar");
        assert_eq!(route.source_route, "userbox_local_scalar_seed_route");
        assert_eq!(route.proof, "userbox_flag_field_local_scalar_seed");
        assert_eq!(route.selected_value, None);
    }

    #[test]
    fn exact_seed_backend_route_selects_userbox_loop_micro_metadata() {
        let mut function = make_function();
        function.metadata.userbox_loop_micro_seed_route = Some(UserBoxLoopMicroSeedRoute {
            kind: UserBoxLoopMicroSeedKind::FlagToggleMicro,
            box_name: "Flag".to_string(),
            block_count: 4,
            newbox_block: BasicBlockId::new(0),
            newbox_instruction_index: 0,
            box_value: ValueId::new(9),
            ops: 2_000_000,
            flip_at: Some(1_000_000),
            field_get_count: 2,
            field_set_count: 2,
            compare_lt_count: 2,
            compare_eq_count: 2,
            binop_count: 3,
            proof: UserBoxLoopMicroSeedProof::FlagToggleLoopMicroSeed,
        });

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "userbox_loop_micro");
        assert_eq!(route.source_route, "userbox_loop_micro_seed_route");
        assert_eq!(route.proof, "userbox_flag_toggle_loop_micro_seed");
        assert_eq!(route.selected_value, None);
    }

    #[test]
    fn exact_seed_backend_route_selects_userbox_known_receiver_method_metadata() {
        let mut function = make_function();
        function.metadata.userbox_known_receiver_method_seed_route =
            Some(UserBoxKnownReceiverMethodSeedRoute {
                kind: UserBoxKnownReceiverMethodSeedKind::CounterStepCopyLocalI64,
                box_name: "Counter".to_string(),
                method: "step".to_string(),
                method_function: "Counter.step/1".to_string(),
                block_count: 1,
                method_block_count: 1,
                block: BasicBlockId::new(0),
                method_block: BasicBlockId::new(1),
                newbox_instruction_index: 1,
                copy_instruction_index: Some(3),
                call_instruction_index: 4,
                box_value: ValueId::new(2),
                copy_value: Some(ValueId::new(3)),
                result_value: ValueId::new(4),
                proof: UserBoxKnownReceiverMethodSeedProof::CounterStepLocalI64Seed,
                payload: UserBoxKnownReceiverMethodSeedPayload::CounterStepI64 {
                    base_i64: 41,
                    delta_i64: 2,
                },
            });

        refresh_function_exact_seed_backend_route(&mut function);

        let route = function
            .metadata
            .exact_seed_backend_route
            .expect("exact seed backend route");
        assert_eq!(route.tag.as_str(), "userbox_known_receiver_method_seed");
        assert_eq!(
            route.source_route,
            "userbox_known_receiver_method_seed_route"
        );
        assert_eq!(route.proof, "userbox_counter_step_local_i64_seed");
        assert_eq!(route.selected_value, None);
    }

    #[test]
    fn exact_seed_backend_route_selects_substring_views_metadata() {
        let mut function = make_function();
        function.metadata.substring_views_micro_seed_route = Some(
            crate::mir::substring_views_micro_seed_plan::test_support::kilo_micro_substring_views_only_5block(),
        );

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
