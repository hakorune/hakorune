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
pub enum ExactSeedBackendRouteKind {
    ArrayStringStoreMicro,
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
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArrayStringStoreMicro => "array_string_store_micro",
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

    pub fn source_route_field(self) -> &'static str {
        match self {
            Self::ArrayStringStoreMicro => "array_string_store_micro_seed_route",
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

    if let Some(route) = function.metadata.array_rmw_add1_leaf_seed_route.as_ref() {
        return Some(ExactSeedBackendRoute {
            tag: ExactSeedBackendRouteKind::ArrayRmwAdd1Leaf,
            source_route: ExactSeedBackendRouteKind::ArrayRmwAdd1Leaf
                .source_route_field()
                .to_string(),
            proof: route.proof.to_string(),
            selected_value: None,
        });
    }

    if let Some(route) = function.metadata.sum_variant_tag_seed_route.as_ref() {
        return Some(ExactSeedBackendRoute {
            tag: ExactSeedBackendRouteKind::SumVariantTagLocal,
            source_route: ExactSeedBackendRouteKind::SumVariantTagLocal
                .source_route_field()
                .to_string(),
            proof: route.proof.to_string(),
            selected_value: None,
        });
    }

    if let Some(route) = function.metadata.sum_variant_project_seed_route.as_ref() {
        return Some(ExactSeedBackendRoute {
            tag: ExactSeedBackendRouteKind::SumVariantProjectLocal,
            source_route: ExactSeedBackendRouteKind::SumVariantProjectLocal
                .source_route_field()
                .to_string(),
            proof: route.proof.to_string(),
            selected_value: None,
        });
    }

    if let Some(route) = function.metadata.userbox_local_scalar_seed_route.as_ref() {
        let tag = match route.kind {
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
            proof: route.proof.to_string(),
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
        ArrayRmwAdd1LeafSeedProof, ArrayRmwAdd1LeafSeedRoute, ArrayStringStoreMicroSeedProof,
        ArrayStringStoreMicroSeedRoute, ConcatConstSuffixMicroSeedProof,
        ConcatConstSuffixMicroSeedRoute, EffectMask, FunctionSignature, MirType, StringKernelPlan,
        StringKernelPlanBorrowContract, StringKernelPlanConsumer, StringKernelPlanFamily,
        StringKernelPlanPublicationBoundary, StringKernelPlanPublicationContract,
        StringKernelPlanRetainedForm, StringKernelPlanVerifierOwner, SubstringViewsMicroSeedProof,
        SubstringViewsMicroSeedRoute, SumLocalAggregateLayout, SumVariantProjectSeedKind,
        SumVariantProjectSeedPayload, SumVariantProjectSeedProof, SumVariantProjectSeedRoute,
        SumVariantTagSeedKind, SumVariantTagSeedProof, SumVariantTagSeedRoute,
        UserBoxKnownReceiverMethodSeedKind, UserBoxKnownReceiverMethodSeedPayload,
        UserBoxKnownReceiverMethodSeedProof, UserBoxKnownReceiverMethodSeedRoute,
        UserBoxLocalScalarSeedKind, UserBoxLocalScalarSeedPayload, UserBoxLocalScalarSeedProof,
        UserBoxLocalScalarSeedRoute, UserBoxLocalScalarSeedSinglePayload, UserBoxLoopMicroSeedKind,
        UserBoxLoopMicroSeedProof, UserBoxLoopMicroSeedRoute,
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
    fn exact_seed_backend_route_selects_array_rmw_add1_leaf_metadata() {
        let mut function = make_function();
        function.metadata.array_rmw_add1_leaf_seed_route = Some(ArrayRmwAdd1LeafSeedRoute {
            size: 128,
            ops: 2_000_000,
            init_push_count: 1,
            final_get_count: 2,
            selected_rmw_block: BasicBlockId::new(23),
            selected_rmw_instruction_index: 8,
            selected_rmw_set_instruction_index: 13,
            proof: ArrayRmwAdd1LeafSeedProof::KiloLeafArrayRmwAdd1SevenBlock,
            rmw_proof: crate::mir::ArrayRmwWindowProof::ArrayGetAdd1SetSameSlot,
        });

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
    fn exact_seed_backend_route_selects_sum_variant_tag_metadata() {
        let mut function = make_function();
        function.metadata.sum_variant_tag_seed_route = Some(SumVariantTagSeedRoute {
            kind: SumVariantTagSeedKind::LocalI64,
            enum_name: "Result".to_string(),
            variant: "Ok".to_string(),
            subject: "Result::Ok".to_string(),
            layout: SumLocalAggregateLayout::TagI64Payload,
            variant_tag: 0,
            make_block: BasicBlockId::new(0),
            make_instruction_index: 1,
            tag_block: BasicBlockId::new(0),
            tag_instruction_index: 2,
            sum_value: ValueId::new(2),
            tag_value: ValueId::new(3),
            tag_source_value: ValueId::new(2),
            copy_value: None,
            payload_value: Some(ValueId::new(1)),
            proof: SumVariantTagSeedProof::LocalAggregateTagSeed,
        });

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
        function.metadata.sum_variant_project_seed_route = Some(SumVariantProjectSeedRoute {
            kind: SumVariantProjectSeedKind::LocalI64,
            enum_name: "ResultInt".to_string(),
            variant: "Ok".to_string(),
            subject: "ResultInt::Ok".to_string(),
            layout: SumLocalAggregateLayout::TagI64Payload,
            variant_tag: 0,
            make_block: BasicBlockId::new(0),
            make_instruction_index: 1,
            project_block: BasicBlockId::new(0),
            project_instruction_index: 2,
            sum_value: ValueId::new(2),
            project_value: ValueId::new(3),
            project_source_value: ValueId::new(2),
            copy_value: None,
            payload_value: ValueId::new(1),
            payload: SumVariantProjectSeedPayload::I64(73),
            proof: SumVariantProjectSeedProof::LocalAggregateProjectSeed,
        });

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
        function.metadata.userbox_local_scalar_seed_route = Some(UserBoxLocalScalarSeedRoute {
            kind: UserBoxLocalScalarSeedKind::PointLocalI64,
            box_name: "Point".to_string(),
            block: BasicBlockId::new(0),
            newbox_instruction_index: 2,
            box_value: ValueId::new(3),
            copy_value: None,
            result_value: ValueId::new(6),
            proof: UserBoxLocalScalarSeedProof::PointFieldLocalScalarSeed,
            payload: UserBoxLocalScalarSeedPayload::PointI64Pair {
                x_field: "x".to_string(),
                y_field: "y".to_string(),
                set_x_instruction_index: 3,
                set_y_instruction_index: 4,
                get_x_instruction_index: 5,
                get_y_instruction_index: 6,
                x_value: ValueId::new(1),
                y_value: ValueId::new(2),
                get_x_value: ValueId::new(4),
                get_y_value: ValueId::new(5),
                x_i64: 41,
                y_i64: 2,
            },
        });

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
        function.metadata.userbox_local_scalar_seed_route = Some(UserBoxLocalScalarSeedRoute {
            kind: UserBoxLocalScalarSeedKind::FlagCopyLocalBool,
            box_name: "Flag".to_string(),
            block: BasicBlockId::new(0),
            newbox_instruction_index: 1,
            box_value: ValueId::new(2),
            copy_value: Some(ValueId::new(3)),
            result_value: ValueId::new(4),
            proof: UserBoxLocalScalarSeedProof::FlagFieldLocalScalarSeed,
            payload: UserBoxLocalScalarSeedPayload::SingleField {
                field: "enabled".to_string(),
                set_instruction_index: 2,
                get_instruction_index: 4,
                field_value: ValueId::new(1),
                get_field_value: ValueId::new(4),
                payload: UserBoxLocalScalarSeedSinglePayload::I64(1),
            },
        });

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
