use crate::mir::verification_types::VerificationError;
use crate::mir::{
    infer_string_kernel_text_consumer, MirFunction, StringKernelPlanCarrier,
    StringKernelPlanTextConsumer, StringKernelPlanVerifierOwner,
};

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
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirInstruction, MirType,
        StringCorridorCandidateProof, StringCorridorCandidateState, StringKernelPlan,
        StringKernelPlanConsumer, StringKernelPlanFamily, StringKernelPlanPublicationBoundary,
        StringKernelPlanPublicationContract, StringKernelPlanRetainedForm, ValueId,
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

    fn base_slot_plan(carrier: StringKernelPlanCarrier) -> StringKernelPlan {
        StringKernelPlan {
            plan_value: ValueId::new(10),
            version: 1,
            family: StringKernelPlanFamily::ConcatTripletWindow,
            corridor_root: ValueId::new(10),
            source_root: Some(ValueId::new(0)),
            borrow_contract: Some(crate::mir::StringKernelPlanBorrowContract::BorrowTextFromObject),
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
}
