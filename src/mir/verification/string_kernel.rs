use crate::mir::string_corridor::{StringPublishReason, StringPublishReprPolicy};
use crate::mir::string_kernel_plan::{
    StringKernelPlan, StringKernelPlanBorrowContract, StringKernelPlanCarrier,
    StringKernelPlanPublicationBoundary, StringKernelPlanPublicationContract,
    StringKernelPlanTextConsumer, StringKernelPlanVerifierOwner,
};
use crate::mir::verification_types::VerificationError;
use crate::mir::{infer_string_kernel_text_consumer, MirFunction, ValueId};

fn push_string_kernel_plan_violation(
    errors: &mut Vec<VerificationError>,
    value: ValueId,
    reason: impl Into<String>,
) {
    errors.push(VerificationError::StringKernelPlanViolation {
        value,
        reason: reason.into(),
    });
}

fn publication_metadata_pair(
    value: ValueId,
    plan: &StringKernelPlan,
    errors: &mut Vec<VerificationError>,
) -> Option<(StringPublishReason, StringPublishReprPolicy)> {
    match (plan.publish_reason, plan.publish_repr_policy) {
        (Some(reason), Some(repr)) => Some((reason, repr)),
        (None, None) => None,
        (Some(reason), None) => {
            push_string_kernel_plan_violation(
                errors,
                value,
                format!(
                    "publish.text metadata is partial: reason={} without repr policy",
                    reason
                ),
            );
            None
        }
        (None, Some(repr)) => {
            push_string_kernel_plan_violation(
                errors,
                value,
                format!(
                    "publish.text metadata is partial: repr policy={} without reason",
                    repr
                ),
            );
            None
        }
    }
}

fn verify_publication_boundary_contract(
    value: ValueId,
    plan: &StringKernelPlan,
    errors: &mut Vec<VerificationError>,
) {
    let publish_pair = publication_metadata_pair(value, plan, errors);

    match (plan.publish_repr_policy, plan.stable_view_provenance) {
        (Some(StringPublishReprPolicy::StableView), Some(_)) => {}
        (Some(StringPublishReprPolicy::StableView), None) => {
            push_string_kernel_plan_violation(
                errors,
                value,
                "stable_view repr request requires stable_view_provenance; lowering must downgrade to stable_owned unless StableView legality is verifier-visible",
            );
        }
        (Some(StringPublishReprPolicy::StableOwned) | None, Some(provenance)) => {
            push_string_kernel_plan_violation(
                errors,
                value,
                format!(
                    "stable_view_provenance={} requires publish_repr_policy=stable_view",
                    provenance
                ),
            );
        }
        (Some(StringPublishReprPolicy::StableOwned) | None, None) => {}
    }

    if publish_pair.is_some() {
        if plan.borrow_contract != Some(StringKernelPlanBorrowContract::BorrowTextFromObject) {
            push_string_kernel_plan_violation(
                errors,
                value,
                "publish.text requires borrow_contract=borrow_text_from_obj on the current string direct-kernel lane",
            );
        }
        if plan.source_root.is_none() {
            push_string_kernel_plan_violation(
                errors,
                value,
                "publish.text requires source_root for verifier-visible borrow provenance",
            );
        }
        if plan.publication_contract
            != Some(
                StringKernelPlanPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
            )
        {
            push_string_kernel_plan_violation(
                errors,
                value,
                "publish.text requires publication_contract=publish_now_not_required_before_first_external_boundary to keep freeze.str separated from publish.text",
            );
        }
    }

    if publish_pair.is_some() && plan.publication_boundary.is_none() {
        push_string_kernel_plan_violation(
            errors,
            value,
            "publish.text metadata requires publication_boundary on the direct-kernel lane",
        );
    }

    if matches!(
        publish_pair,
        Some((StringPublishReason::ExplicitApiReplay, _))
    ) && !matches!(
        plan.text_consumer,
        Some(StringKernelPlanTextConsumer::ExplicitColdPublish)
    ) {
        push_string_kernel_plan_violation(
            errors,
            value,
            "explicit_api_replay publish reason requires explicit_cold_publish consumer",
        );
    }

    if !matches!(
        plan.text_consumer,
        Some(StringKernelPlanTextConsumer::ExplicitColdPublish)
    ) {
        return;
    }

    if plan.publication_boundary != Some(StringKernelPlanPublicationBoundary::FirstExternalBoundary)
    {
        push_string_kernel_plan_violation(
            errors,
            value,
            "explicit_cold_publish requires publication_boundary=first_external_boundary",
        );
    }

    if plan.publication_contract
        != Some(
            StringKernelPlanPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        )
    {
        push_string_kernel_plan_violation(
            errors,
            value,
            "explicit_cold_publish requires publication_contract=publish_now_not_required_before_first_external_boundary",
        );
    }

    match publish_pair {
        Some((StringPublishReason::ExplicitApiReplay, StringPublishReprPolicy::StableOwned)) => {}
        Some((StringPublishReason::ExplicitApiReplay, StringPublishReprPolicy::StableView)) => {}
        Some((reason, repr)) => {
            push_string_kernel_plan_violation(
                errors,
                value,
                format!(
                    "explicit_cold_publish currently requires publish.text(explicit_api_replay, stable_owned), got publish.text({}, {})",
                    reason, repr
                ),
            );
        }
        None => {
            push_string_kernel_plan_violation(
                errors,
                value,
                "explicit_cold_publish requires publish.text metadata",
            );
        }
    }
}

pub fn check_string_kernel_plans(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();

    for (value, plan) in &function.metadata.string_kernel_plans {
        if plan.verifier_owner != Some(StringKernelPlanVerifierOwner::LoweringDirectKernelEntry) {
            continue;
        }

        let expected_consumer = infer_string_kernel_text_consumer(function, *value);
        if expected_consumer != plan.text_consumer {
            errors.push(VerificationError::StringKernelPlanViolation {
                value: *value,
                reason: format!(
                    "slot-capable consumer rule mismatch: expected {:?}, got {:?}",
                    expected_consumer, plan.text_consumer
                ),
            });
        }

        let legality = plan.legality();
        if !legality.requires_kernel_text_slot {
            continue;
        }

        match plan.carrier {
            Some(StringKernelPlanCarrier::KernelTextSlot) => {}
            Some(StringKernelPlanCarrier::RegistryBackedHandle) => {
                if legality.rejects_early_stable_box_now {
                    errors.push(VerificationError::StringKernelPlanViolation {
                        value: *value,
                        reason: "early StableBoxNow is illegal before the first external boundary"
                            .to_string(),
                    });
                }
                if legality.rejects_early_fresh_registry_handle {
                    errors.push(VerificationError::StringKernelPlanViolation {
                        value: *value,
                        reason:
                            "early FreshRegistryHandle is illegal before the first external boundary"
                                .to_string(),
                    });
                }
                if legality.rejects_registry_backed_carrier {
                    errors.push(VerificationError::StringKernelPlanViolation {
                        value: *value,
                        reason:
                            "registry-backed carrier is illegal for same-corridor unpublished outcome"
                                .to_string(),
                    });
                }
            }
            None => {
                let expected = match plan.text_consumer {
                    Some(StringKernelPlanTextConsumer::SlotText) => "slot_text",
                    Some(StringKernelPlanTextConsumer::ExplicitColdPublish) => {
                        "explicit_cold_publish"
                    }
                    None => "none",
                };
                errors.push(VerificationError::StringKernelPlanViolation {
                    value: *value,
                    reason: format!(
                        "direct-kernel text lane requires caller-owned KernelTextSlot carrier for {}",
                        expected
                    ),
                });
            }
        }

        verify_publication_boundary_contract(*value, plan, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::string_corridor::{
        StringPublishReason, StringPublishReprPolicy, StringStableViewProvenance,
    };
    use crate::mir::string_corridor_placement::{
        StringCorridorCandidateProof, StringCorridorCandidateState,
    };
    use crate::mir::string_kernel_plan::{
        StringKernelPlanConsumer, StringKernelPlanFamily, StringKernelPlanReadAliasFacts,
        StringKernelPlanRetainedForm,
    };
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirInstruction, MirType, ValueId,
    };

    fn slot_text_function() -> MirFunction {
        fn method_call(
            dst: ValueId,
            receiver: ValueId,
            method: &str,
            args: Vec<ValueId>,
        ) -> MirInstruction {
            MirInstruction::Call {
                dst: Some(dst),
                func: ValueId::INVALID,
                callee: Some(crate::mir::Callee::Method {
                    box_name: "RuntimeDataBox".to_string(),
                    method: method.to_string(),
                    receiver: Some(receiver),
                    certainty: TypeCertainty::Known,
                    box_kind: CalleeBoxKind::RuntimeData,
                }),
                args,
                effects: EffectMask::PURE,
            }
        }

        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Box("RuntimeDataBox".to_string()),
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::String("xx".to_string()),
            },
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Integer(1),
            },
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::Integer(5),
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(10)),
                func: ValueId::INVALID,
                callee: Some(crate::mir::Callee::Extern(
                    "nyash.string.substring_concat3_hhhii".to_string(),
                )),
                args: vec![
                    ValueId::new(0),
                    ValueId::new(1),
                    ValueId::new(0),
                    ValueId::new(2),
                    ValueId::new(3),
                ],
                effects: EffectMask::PURE,
            },
            method_call(
                ValueId::new(11),
                ValueId::new(10),
                "substring",
                vec![ValueId::new(2), ValueId::new(3)],
            ),
        ]);
        block.instruction_spans.extend(vec![Span::unknown(); 5]);
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(11)),
        });
        function
    }

    fn explicit_publish_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Box("RuntimeDataBox".to_string()),
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::String("xx".to_string()),
            },
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Integer(1),
            },
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::Integer(5),
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(10)),
                func: ValueId::INVALID,
                callee: Some(crate::mir::Callee::Extern(
                    "nyash.string.substring_concat3_hhhii".to_string(),
                )),
                args: vec![
                    ValueId::new(0),
                    ValueId::new(1),
                    ValueId::new(0),
                    ValueId::new(2),
                    ValueId::new(3),
                ],
                effects: EffectMask::PURE,
            },
        ]);
        block.instruction_spans.extend(vec![Span::unknown(); 4]);
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(10)),
        });
        function
    }

    fn base_slot_plan(carrier: StringKernelPlanCarrier) -> StringKernelPlan {
        StringKernelPlan {
            plan_value: ValueId::new(10),
            version: 1,
            family: StringKernelPlanFamily::ConcatTripletWindow,
            corridor_root: ValueId::new(10),
            source_root: Some(ValueId::new(0)),
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
            direct_kernel_entry: Some(StringCorridorCandidateState::Candidate),
            consumer: Some(StringKernelPlanConsumer::DirectKernelEntry),
            text_consumer: Some(StringKernelPlanTextConsumer::SlotText),
            carrier: Some(carrier),
            verifier_owner: Some(StringKernelPlanVerifierOwner::LoweringDirectKernelEntry),
            read_alias: StringKernelPlanReadAliasFacts {
                same_receiver: true,
                source_window: true,
                followup_substring: true,
                piecewise_subrange: true,
                direct_set_consumer: false,
                shared_receiver: false,
            },
            slot_hop_substring: None,
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(0)),
                left_source: ValueId::new(0),
                left_start: ValueId::new(2),
                left_end: ValueId::new(2),
                middle: ValueId::new(1),
                right_value: Some(ValueId::new(0)),
                right_source: ValueId::new(0),
                right_start: ValueId::new(2),
                right_end: ValueId::new(3),
                shared_source: true,
            },
            middle_literal: Some("xx".to_string()),
            loop_payload: None,
        }
    }

    #[test]
    fn verifier_rejects_registry_backed_string_lane_carrier() {
        let mut function = slot_text_function();
        function.metadata.string_kernel_plans.insert(
            ValueId::new(10),
            base_slot_plan(StringKernelPlanCarrier::RegistryBackedHandle),
        );

        let errors = check_string_kernel_plans(&function).expect_err("expected violations");
        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::StringKernelPlanViolation { reason, .. }
                if reason.contains("StableBoxNow")
        )));
        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::StringKernelPlanViolation { reason, .. }
                if reason.contains("FreshRegistryHandle")
        )));
        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::StringKernelPlanViolation { reason, .. }
                if reason.contains("registry-backed carrier")
        )));
    }

    #[test]
    fn verifier_rejects_slot_consumer_rule_mismatch() {
        let mut function = slot_text_function();
        let mut plan = base_slot_plan(StringKernelPlanCarrier::KernelTextSlot);
        plan.text_consumer = Some(StringKernelPlanTextConsumer::ExplicitColdPublish);
        function
            .metadata
            .string_kernel_plans
            .insert(ValueId::new(10), plan);

        let errors = check_string_kernel_plans(&function).expect_err("expected mismatch");
        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::StringKernelPlanViolation { reason, .. }
                if reason.contains("slot-capable consumer rule mismatch")
        )));
    }

    #[test]
    fn verifier_rejects_explicit_cold_publish_without_boundary_metadata() {
        let mut function = slot_text_function();
        let mut plan = base_slot_plan(StringKernelPlanCarrier::KernelTextSlot);
        plan.text_consumer = Some(StringKernelPlanTextConsumer::ExplicitColdPublish);
        plan.publication_boundary = None;
        plan.publish_reason = Some(StringPublishReason::ExplicitApiReplay);
        plan.publish_repr_policy = Some(StringPublishReprPolicy::StableOwned);
        function
            .metadata
            .string_kernel_plans
            .insert(ValueId::new(10), plan);

        let errors = check_string_kernel_plans(&function).expect_err("expected boundary failure");
        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::StringKernelPlanViolation { reason, .. }
                if reason.contains("publication_boundary=first_external_boundary")
        )));
    }

    #[test]
    fn verifier_rejects_partial_publish_text_metadata() {
        let mut function = slot_text_function();
        let mut plan = base_slot_plan(StringKernelPlanCarrier::KernelTextSlot);
        plan.publish_reason = Some(StringPublishReason::StableObjectDemand);
        plan.publish_repr_policy = None;
        function
            .metadata
            .string_kernel_plans
            .insert(ValueId::new(10), plan);

        let errors =
            check_string_kernel_plans(&function).expect_err("expected publish metadata failure");
        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::StringKernelPlanViolation { reason, .. }
                if reason.contains("publish.text metadata is partial")
        )));
    }

    #[test]
    fn verifier_rejects_publish_text_without_borrow_provenance() {
        let mut function = explicit_publish_function();
        let mut plan = base_slot_plan(StringKernelPlanCarrier::KernelTextSlot);
        plan.text_consumer = Some(StringKernelPlanTextConsumer::ExplicitColdPublish);
        plan.borrow_contract = None;
        plan.source_root = None;
        plan.publish_reason = Some(StringPublishReason::ExplicitApiReplay);
        plan.publish_repr_policy = Some(StringPublishReprPolicy::StableOwned);
        function
            .metadata
            .string_kernel_plans
            .insert(ValueId::new(10), plan);

        let errors = check_string_kernel_plans(&function).expect_err("expected provenance failure");
        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::StringKernelPlanViolation { reason, .. }
                if reason.contains("borrow_contract=borrow_text_from_obj")
        )));
        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::StringKernelPlanViolation { reason, .. }
                if reason.contains("source_root for verifier-visible borrow provenance")
        )));
    }

    #[test]
    fn verifier_rejects_publish_text_without_freeze_publish_separation_contract() {
        let mut function = explicit_publish_function();
        let mut plan = base_slot_plan(StringKernelPlanCarrier::KernelTextSlot);
        plan.text_consumer = Some(StringKernelPlanTextConsumer::ExplicitColdPublish);
        plan.publication_contract = None;
        plan.publish_reason = Some(StringPublishReason::ExplicitApiReplay);
        plan.publish_repr_policy = Some(StringPublishReprPolicy::StableOwned);
        function
            .metadata
            .string_kernel_plans
            .insert(ValueId::new(10), plan);

        let errors = check_string_kernel_plans(&function).expect_err("expected separation failure");
        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::StringKernelPlanViolation { reason, .. }
                if reason.contains("keep freeze.str separated from publish.text")
        )));
    }

    #[test]
    fn verifier_rejects_unproven_stable_view_publish_request() {
        let mut function = slot_text_function();
        let mut plan = base_slot_plan(StringKernelPlanCarrier::KernelTextSlot);
        plan.text_consumer = Some(StringKernelPlanTextConsumer::ExplicitColdPublish);
        plan.publish_reason = Some(StringPublishReason::ExplicitApiReplay);
        plan.publish_repr_policy = Some(StringPublishReprPolicy::StableView);
        function
            .metadata
            .string_kernel_plans
            .insert(ValueId::new(10), plan);

        let errors =
            check_string_kernel_plans(&function).expect_err("expected stable_view failure");
        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::StringKernelPlanViolation { reason, .. }
                if reason.contains("stable_view repr request requires stable_view_provenance")
        )));
    }

    #[test]
    fn verifier_accepts_proven_stable_view_publish_request() {
        let mut function = explicit_publish_function();
        let mut plan = base_slot_plan(StringKernelPlanCarrier::KernelTextSlot);
        plan.text_consumer = Some(StringKernelPlanTextConsumer::ExplicitColdPublish);
        plan.publish_reason = Some(StringPublishReason::ExplicitApiReplay);
        plan.publish_repr_policy = Some(StringPublishReprPolicy::StableView);
        plan.stable_view_provenance = Some(StringStableViewProvenance::AlreadyStable);
        function
            .metadata
            .string_kernel_plans
            .insert(ValueId::new(10), plan);

        check_string_kernel_plans(&function).expect("proven stable_view request should pass");
    }

    #[test]
    fn verifier_rejects_stable_view_provenance_without_stable_view_request() {
        let mut function = explicit_publish_function();
        let mut plan = base_slot_plan(StringKernelPlanCarrier::KernelTextSlot);
        plan.text_consumer = Some(StringKernelPlanTextConsumer::ExplicitColdPublish);
        plan.publish_reason = Some(StringPublishReason::ExplicitApiReplay);
        plan.publish_repr_policy = Some(StringPublishReprPolicy::StableOwned);
        plan.stable_view_provenance = Some(StringStableViewProvenance::AlreadyStable);
        function
            .metadata
            .string_kernel_plans
            .insert(ValueId::new(10), plan);

        let errors =
            check_string_kernel_plans(&function).expect_err("expected stray provenance failure");
        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::StringKernelPlanViolation { reason, .. }
                if reason.contains("stable_view_provenance=already_stable requires publish_repr_policy=stable_view")
        )));
    }
}
