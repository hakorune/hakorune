//! Original nonview Mul operands close through the same finished source owner.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use crate::mir::normal_callable_semantic_package::{
    BorrowedMulMaterializationV1, BorrowedMulSourceLoanV1,
};

impl FinalizedRootSourceHandoffV1 {
    pub(super) fn verify_finished_mul_nonviews_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        record: &BorrowedMulMaterializationV1,
        finished: &Binding,
        copies: &[Option<Binding>; 2],
    ) -> Result<(), String> {
        let loan = record.source();
        let children = [record.children().0, record.children().1];
        let views = loan.checked_views();
        self.ledger
            .with_finished_projection(owner, |symbol, projection| {
                if function.signature.name != symbol || loan.owner() != owner {
                    return Err(freeze("borrowed-mul/nonview-owner"));
                }
                for side in 0..2 {
                    if views.iter().any(|(index, _, _)| *index == side) {
                        continue;
                    }
                    let consumer = copies[side].as_ref().unwrap_or(finished);
                    if let Some((site, value)) = loan.integer_literal(side) {
                        if site.owner() != owner {
                            return Err(freeze("borrowed-mul/literal-owner"));
                        }
                        let original = projection.captured_definition(children[side])?;
                        if !matches!(original.1, MirInstruction::Const {
                        value: crate::mir::ConstValue::Integer(actual), ..
                    } if actual == value)
                        {
                            return Err(freeze("borrowed-mul/literal-original"));
                        }
                        let literal = projection
                            .relation_binding(function, original)?
                            .ok_or_else(|| freeze("borrowed-mul/literal-removed"))?;
                        require_before(function, &literal, consumer)?;
                    } else if loan.integer_call_source(side).is_some() {
                        self.verify_mul_call_operand(
                            owner,
                            function,
                            loan,
                            side,
                            children[side],
                            consumer,
                            projection,
                        )?;
                    } else {
                        return Err(freeze("borrowed-mul/nonview-source-missing"));
                    }
                }
                Ok(())
            })
    }

    fn verify_mul_call_operand(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        loan: &BorrowedMulSourceLoanV1,
        side: usize,
        child: ValueId,
        consumer: &Binding,
        projection: &physical_boundary::FinishedBindings,
    ) -> Result<(), String> {
        let source = loan
            .integer_call_source(side)
            .ok_or_else(|| freeze("borrowed-mul/call-source-missing"))?;
        let site = source.call_site();
        if site.owner() != owner {
            return Err(freeze("borrowed-mul/call-owner"));
        }
        let mut packets = self.local_calls.get(&owner).into_iter().flatten()
            .flat_map(|group| group.lexical_packets().map(move |packet| (group, packet)))
            .filter(|(_, packet)| packet.call_site() == site);
        let (group, packet) = packets.next().ok_or_else(|| freeze("borrowed-mul/call-group-missing"))?;
        if packets.next().is_some() { return Err(freeze("borrowed-mul/call-group-duplicate")); }
        let CallPacketSourceLoanV1::Static {
            original,
            observation,
            ..
        } = packet.original_source()
        else {
            return Err(freeze("borrowed-mul/call-packet-source"));
        };
        if !Rc::ptr_eq(original, source) {
            return Err(freeze("borrowed-mul/call-original-identity"));
        }
        let current = self
            .ledger
            .lexical_i64_call_source(site)
            .ok_or_else(|| freeze("borrowed-mul/call-observation-missing"))?;
        if current != observation
            || !current.arguments().is_empty()
            || packet.value_in_context_v1(owner, current.arguments(), CallValidationContextV1::Finalized(self))? != child
        {
            return Err(freeze("borrowed-mul/call-observation-drift"));
        }
        let (invoke, normal) = packet.outer_bindings()?;
        // Invoke has no dst; its existing call loan checks retained membership
        // and recorded projection, including the original Fault landing/frame.
        self.finished_local_call_producer_v1(owner, group.site(), site, invoke, function)?;
        let (_, invoke) = project_recorded(&function.signature.name, projection, invoke)?;
        let normal = projection
            .relation_binding(function, normal)?
            .ok_or_else(|| freeze("borrowed-mul/call-normal-removed"))?;
        if normal.1.dst_value() != Some(child)
            || !matches!((&invoke.1, &normal.1),
                (MirInstruction::Invoke { normal_landing, .. },
                 MirInstruction::InvokeNormalResult { invoke_block, .. })
                if *normal_landing == normal.0 && *invoke_block == invoke.0)
        {
            return Err(freeze("borrowed-mul/call-normal-link"));
        }
        require_before(function, &invoke, &normal)?;
        require_before(function, &normal, consumer)
    }
}

fn require_before(
    function: &MirFunction,
    producer: &Binding,
    consumer: &Binding,
) -> Result<(), String> {
    let producer_position = find_finished_producer(&function.signature.name, producer, function)?;
    let consumer_position = find_finished_producer(&function.signature.name, consumer, function)?;
    let dominators = crate::mir::verification::utils::compute_dominators(function);
    if !dominators.is_reachable(producer.0)
        || !dominators.is_reachable(consumer.0)
        || !dominators.dominates(producer.0, consumer.0)
        || (producer.0 == consumer.0 && producer_position.1 >= consumer_position.1)
    {
        return Err(freeze("borrowed-mul/nonview-order"));
    }
    Ok(())
}
