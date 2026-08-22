//! Direct Length Call/result canary for the common V2 session.
//!
//! This child module keeps the canonical session transport wrapper below the
//! source-size split while retaining the parent session as the sole mutable
//! owner. It emits no publication or production edge.

use crate::mir::core_method_result_kind::CoreMethodEffectV1;
use crate::mir::resolved_semantics::{
    CoreMethodHomeExecutionPolicyV1, CoreMethodHomeReceiverRelationV1,
    CoreMethodHomeResultRelationV1,
};
use crate::mir::{Callee, MirInstruction};

use super::super::super::calls::call_target::CallTarget;
use super::super::super::calls::unified_emitter::UnifiedCallEmitterBox;
use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum LengthCallDirectEmitterRejectV1 {
    AlreadyIssued,
    TargetPlan(LengthCallTargetPlanRejectV1),
    SegmentAllocation(String),
    SegmentScopeMismatch,
    ConditionTarget(ConditionBlockTargetRejectV1),
    TargetShapeMismatch,
    PhysicalValue(String),
    Receiver(LengthReceiverPhysicalOperandRejectV1),
}

/// One unpublished generic `StringBox.length` Call and its canonical I64
/// destination. The exclusive session borrow is intentional: a later
/// materializer must be reached through this receipt, never by handing its
/// metadata back to another session.
pub(in crate::mir::builder) struct CanonicalLengthCallResultReceiptV1<'call, 'source, 'envelope> {
    _session: &'call mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    condition_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    call_item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    physical_block: crate::mir::BasicBlockId,
    receiver: crate::mir::ValueId,
    destination: crate::mir::ValueId,
    segment_brand: super::super::common_v2_segment_block_allocation::SegmentBlockAllocationBrandV1,
}

impl<'call, 'source, 'envelope> CanonicalLengthCallResultReceiptV1<'call, 'source, 'envelope> {
    pub(in crate::mir::builder) const fn owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn condition_block(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopBlockKeyV1 {
        self.condition_block
    }

    pub(in crate::mir::builder) const fn call_item(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopItemKeyV1 {
        self.call_item
    }

    pub(in crate::mir::builder) const fn result(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.result
    }

    pub(in crate::mir::builder) const fn physical_block(&self) -> crate::mir::BasicBlockId {
        self.physical_block
    }

    pub(in crate::mir::builder) const fn receiver(&self) -> crate::mir::ValueId {
        self.receiver
    }

    pub(in crate::mir::builder) const fn destination(&self) -> crate::mir::ValueId {
        self.destination
    }

    pub(in crate::mir::builder) fn segment_brand(
        &self,
    ) -> super::super::common_v2_segment_block_allocation::SegmentBlockAllocationBrandV1 {
        self.segment_brand.clone()
    }

    pub(in crate::mir::builder) fn into_condition_bool_parts(
        self,
    ) -> (
        &'call mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
        crate::mir::resolved_semantics::FunctionOwnerIdV1,
        crate::mir::loop_recipe_contract::LoopBlockKeyV1,
        crate::mir::BasicBlockId,
        crate::mir::ValueId,
        super::super::common_v2_segment_block_allocation::SegmentBlockAllocationBrandV1,
    ) {
        (
            self._session,
            self.owner,
            self.condition_block,
            self.physical_block,
            self.destination,
            self.segment_brand,
        )
    }
}

impl<'source, 'envelope> CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    /// Compatibility canary: allocate one private segment scope and emit the
    /// Length result through it. The shared branch path uses the explicit
    /// scope method below so later consumers cannot re-pair a second receipt.
    pub(in crate::mir::builder) fn emit_length_call_result<'call>(
        &'call mut self,
        builder: &mut crate::mir::builder::MirBuilder,
    ) -> Result<
        CanonicalLengthCallResultReceiptV1<'call, 'source, 'envelope>,
        LengthCallDirectEmitterRejectV1,
    > {
        let segment_receipt = self.allocate_v2_segment_blocks(builder).map_err(|error| {
            LengthCallDirectEmitterRejectV1::SegmentAllocation(format!("{error:?}"))
        })?;
        self.emit_length_call_result_from_segment_receipt(builder, &segment_receipt)
    }

    /// Emit Length from the exact session-owned segment allocation. This is
    /// the only API that can feed the shared Bool -> Return-read transition.
    pub(in crate::mir::builder) fn emit_length_call_result_from_scope<'call>(
        &'call mut self,
        builder: &mut crate::mir::builder::MirBuilder,
        scope: &super::super::common_v2_segment_block_allocation::CommonV2SharedSegmentScopeV1,
    ) -> Result<
        CanonicalLengthCallResultReceiptV1<'call, 'source, 'envelope>,
        LengthCallDirectEmitterRejectV1,
    > {
        self.emit_length_call_result_from_segment_receipt(builder, scope.receipt())
    }

    fn emit_length_call_result_from_segment_receipt<'call>(
        &'call mut self,
        builder: &mut crate::mir::builder::MirBuilder,
        segment_receipt:
            &super::super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1,
    ) -> Result<
        CanonicalLengthCallResultReceiptV1<'call, 'source, 'envelope>,
        LengthCallDirectEmitterRejectV1,
    > {
        if self.length_call_direct_issued {
            return Err(LengthCallDirectEmitterRejectV1::AlreadyIssued);
        }
        if !self.session.owns_segment_receipt(segment_receipt) {
            return Err(LengthCallDirectEmitterRejectV1::SegmentScopeMismatch);
        }
        let plan = self
            .issue_length_call_target_plan()
            .map_err(LengthCallDirectEmitterRejectV1::TargetPlan)?;
        if plan.receiver() != CoreMethodHomeReceiverRelationV1::StringBoxReceiver
            || plan.result_relation() != CoreMethodHomeResultRelationV1::I64ToCaller
            || plan.effect() != CoreMethodEffectV1::PureRead
            || plan.execution_policy() != CoreMethodHomeExecutionPolicyV1::NonSuspendingNonControl
            || plan.box_name() != "StringBox"
            || plan.method_name() != "length"
        {
            return Err(LengthCallDirectEmitterRejectV1::TargetShapeMismatch);
        }

        let condition_target = self
            .condition_block_target_from_receipt(&segment_receipt)
            .map_err(LengthCallDirectEmitterRejectV1::ConditionTarget)?;
        let condition_physical_block = condition_target.physical_block();
        let condition_block = condition_target.logical_block();
        let stamp_owner = condition_target.stamp_owner();
        if plan.owner() != self.session.owner()
            || plan.block() != condition_block
            || stamp_owner != self.session.owner()
        {
            return Err(LengthCallDirectEmitterRejectV1::TargetShapeMismatch);
        }
        drop(condition_target);

        // Reserve the destination through the canonical session before the
        // generic emitter. Any later failure is discarded by the outer
        // unpublished transaction and cannot be retried in this session.
        let destination = self
            .session
            .issue_physical_value_id(builder)
            .map_err(LengthCallDirectEmitterRejectV1::PhysicalValue)?;
        let emitted = self
            .with_length_receiver_operand(builder, &segment_receipt, |builder, receiver| {
                if receiver.owner() != plan.owner()
                    || receiver.physical_block() != condition_physical_block
                    || receiver.stamp_owner() != stamp_owner
                {
                    return Err("receiver/condition target drift".to_owned());
                }
                let emission = UnifiedCallEmitterBox::emit_unified_value_call_with_external_result_publication_receipt_v1(
                    builder,
                    destination,
                    CallTarget::Method {
                        box_type: Some(plan.box_name().to_owned()),
                        method: plan.method_name().to_owned(),
                        receiver: receiver.physical_value(),
                    },
                    Vec::new(),
                )
                .map_err(|error| format!("{error:?}"))?;
                if emission.final_destination() != destination {
                    return Err("generic Call destination drift".to_owned());
                }
                let instructions = builder.current_function_instructions();
                let Some((call_destination, box_name, method, call_receiver, first_arg, effects)) =
                    instructions.iter().rev().find_map(|instruction| match instruction {
                        MirInstruction::Call {
                            dst: Some(call_destination),
                            callee: Some(Callee::Method {
                                box_name,
                                method,
                                receiver: Some(call_receiver),
                                ..
                            }),
                            args,
                            effects,
                            ..
                        } => Some((
                            *call_destination,
                            box_name.as_str(),
                            method.as_str(),
                            *call_receiver,
                            args.first().copied(),
                            *effects,
                        )),
                        _ => None,
                    })
                else {
                    return Err("generic Call receipt missing final instruction".to_owned());
                };
                if call_destination != destination
                    || box_name != plan.box_name()
                    || method != plan.method_name()
                    || effects != crate::mir::EffectMask::READ
                    || first_arg != Some(call_receiver)
                {
                    return Err("generic Call shape drift".to_owned());
                }
                Ok((call_receiver, receiver.physical_block()))
            })
            .map_err(LengthCallDirectEmitterRejectV1::Receiver)?;

        self.session
            .publish_physical_value_type(builder, destination, crate::mir::MirType::Integer)
            .map_err(LengthCallDirectEmitterRejectV1::PhysicalValue)?;
        self.length_call_direct_issued = true;
        let owner = self.session.owner();
        Ok(CanonicalLengthCallResultReceiptV1 {
            _session: self,
            owner,
            condition_block,
            call_item: plan.item(),
            result: plan.result(),
            physical_block: emitted.1,
            receiver: emitted.0,
            destination,
            segment_brand: segment_receipt.brand(),
        })
    }
}
