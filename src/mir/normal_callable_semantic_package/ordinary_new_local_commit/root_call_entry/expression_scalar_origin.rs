//! Original scalar producer and binding-copy provenance for expression reads.
use super::*;

pub(in super::super) fn initializer_value(
    source: &Producer,
    read: &BindingValueObservationV2,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    let expected = match source {
        Producer::Integer { binding, .. } => Some(read.loan_root_initializer_v1(*binding)?),
        Producer::Call(call) => {
            if context.ledger().lexical_i64_call_source(call.site()) != Some(call) {
                return Err(freeze("expression/scalar-call-source"));
            }
            call.local_binding()
                .ok_or_else(|| freeze("expression/scalar-call-local"))?;
            let value = context.original_scalar_call_result_v1(call)?;
            Some(value)
        }
        Producer::FieldRead(source) => {
            let (_, instruction) = context
                .ledger()
                .local_scalar_field_read_emission_v1(source.site_v1())?
                .ok_or_else(|| freeze("expression/scalar-field-read-source"))?;
            Some(
                instruction
                    .dst_value()
                    .ok_or_else(|| freeze("expression/scalar-field-read-dst"))?,
            )
        }
        Producer::Operation(source) => {
            let original = context.ledger().scalar_operation_original_v1(source)?;
            Some(
                original
                    .original()
                    .1
                    .dst_value()
                    .ok_or_else(|| freeze("expression/scalar-producer-value"))?,
            )
        }
        Producer::Initializer(frame) => {
            Some(context.with_original_initializer_v1(frame, |packet| Ok(packet.value()))?)
        }
        Producer::SignedEntry(_) => return Err(freeze("expression/scalar-entry-not-initializer")),
        Producer::BranchNormal(_) => return Err(freeze("expression/scalar-producer-kind")),
        Producer::BindingRebind(_) => {
            return Err(freeze("expression/scalar-rebind-not-initializer"))
        }
        Producer::LoopHeader(_) => return Err(freeze("expression/loop-header-physical-pending")),
        Producer::LoopAfter { .. } => return Err(freeze("expression/loop-after-physical-pending")),
    };
    let value = read.loan_forwarded_initializer_v1(expected)?;
    if let Producer::Operation(operation) = source {
        context.ledger().retain_scalar_operation_use_v1(
            operation,
            matches!(context, CallValidationContextV1::Finalized(_)),
        )?;
    }
    Ok(value)
}
pub(in super::super) fn validate_scalar_origin(
    binding: crate::mir::resolved_semantics::BindingRefV1,
    source: &Producer,
    read: &BindingValueObservationV2,
    producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    let value = read
        .value_for(binding.owner(), binding)
        .map_err(|error| error.to_string())?;
    if let Producer::BranchNormal(basis) = source {
        context.ledger().require_branch_scalar_source_v1(basis)?;
        let point = read.loan_scalar_join_v1(source)?;
        let ReferenceProducerV1::Phi(relation) = producer else {
            return Err(freeze("expression/scalar-producer-kind"));
        };
        if !std::rc::Rc::ptr_eq(point.relation(), relation) || point.branch().is_none() {
            return Err(freeze("expression/scalar-producer-kind"));
        }
        relation.require_completed_v1()?;
        return Ok(value);
    }
    if matches!(source, Producer::LoopHeader(_) | Producer::LoopAfter { .. }) {
        let ReferenceProducerV1::Phi(point) = producer else {
            return Err(freeze("expression/loop-point-producer-kind"));
        };
        if !std::rc::Rc::ptr_eq(read.loan_scalar_join_v1(source)?.relation(), point) {
            return Err(freeze("expression/loop-point-original-purpose"));
        }
        context.source_loop_point_product_v2(source)?;
        if matches!(source, Producer::LoopAfter { .. })
            || matches!(context, CallValidationContextV1::Finalized(_))
        {
            point.require_completed_v1()?;
        }
        return Ok(value);
    }
    if let Producer::BindingRebind(frame) = source {
        return context.with_original_rebind_v1(frame, |packet| {
            let assigned = read.loan_checked_rebind_v1(frame)?.0;
            if assigned != packet.value() || producer != packet.root_producer() {
                return Err(freeze("expression/rebind-original-producer"));
            }
            Ok(value)
        });
    }
    match (source, producer) {
        (Producer::Initializer(frame), ReferenceProducerV1::Entry(entry)) => {
            let formal = initializer_signed_entry_v1(frame)
                .ok_or_else(|| freeze("expression/scalar-entry-ordinal-value"))?;
            if !matches!(
                frame.evaluation().references().get(&0),
                Some(Reference::FiniteEntry { .. })
            ) {
                signed_entry_layout(binding, formal, context)?;
            }
            if initializer_value(source, read, context)? != *entry {
                return Err(freeze("expression/scalar-entry-ordinal-value"));
            }
            read.loan_forwarded_copies(formal, *entry)?;
        }
        (Producer::SignedEntry(formal), ReferenceProducerV1::Entry(entry)) => {
            signed_entry_layout(binding, *formal, context)?;
            read.loan_forwarded_copies(*formal, *entry)?;
        }
        (Producer::SignedEntry(_), _) | (_, ReferenceProducerV1::Entry(_)) => {
            return Err(freeze("expression/scalar-producer-kind"))
        }
        (_, ReferenceProducerV1::Phi(_)) => {
            return Err(freeze("expression/loop-point-source-kind"))
        }
        (original, ReferenceProducerV1::Definition(binding)) => {
            if binding.1.dst_value() != Some(initializer_value(source, read, context)?) {
                return Err(freeze("expression/scalar-producer-value"));
            }
            if let Producer::Integer {
                value: expected, ..
            } = original
            {
                if !matches!(binding.1, MirInstruction::Const { value: ConstValue::Integer(actual), .. } if actual == *expected)
                {
                    return Err(freeze("expression/scalar-literal-producer"));
                }
            }
        }
    }
    Ok(value)
}
