//! Retain the sole ObjectFieldGet and its original parent packet through finish.
use super::*;
use crate::mir::builder::ExactLexicalReadV1;
use crate::mir::normal_callable_semantic_package::{CallValidationContextV1, LocalFieldReadSourceV1};
#[path = "field_read_argument_interval.rs"]
pub(super) mod interval;

#[derive(Debug)]
pub(super) struct EmittedFieldReadArgumentV1 {
    source: Rc<LocalFieldReadSourceV1>,
    call: OwnedExprSiteV1,
    receiver: ExactLexicalReadV1,
    read: Binding,
    parent_invoke: Binding,
    parent_normal: Binding,
}
impl EmittedFieldReadArgumentV1 {
    pub(super) fn new(
        source: Rc<LocalFieldReadSourceV1>, call: OwnedExprSiteV1,
        receiver: ExactLexicalReadV1, read: Binding,
        context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<Self, String> {
        let (_, parent_invoke, parent_normal) = context.ledger().returned_field_argument_parent_v1(&source, context)?;
        let result = Self { source, call, receiver, read, parent_invoke, parent_normal };
        let child = result.value(&result.call, &result.source, context)?;
        let parent = result.copies(context).and_then(|_| {
            context.ledger().returned_field_argument_parent_v1(&result.source, context).map(|row| row.0)
        })?;
        let intervals = interval::IntervalViewV1::new(function);
        intervals.require_before(&result.parent_invoke, &result.parent_normal)?;
        intervals.require_before(&result.parent_normal, &result.read)?;
        intervals.read(parent, child, result.source.field())?.check(&result.parent_normal, &result.read)?;
        Ok(result)
    }
    pub(super) fn binding(&self) -> &Binding { &self.read }
    pub(super) fn value(
        &self, call: &OwnedExprSiteV1, source: &Rc<LocalFieldReadSourceV1>,
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        if &self.call != call || !Rc::ptr_eq(source, &self.source) {
            return Err(freeze("field-argument/original-source-identity"));
        }
        let (original_call, _, loan) = source.returned_argument_v1()
            .ok_or_else(|| freeze("field-argument/source-missing"))?;
        if original_call != call { return Err(freeze("field-argument/call-identity")); }
        let (parent, invoke, normal) = context.ledger().returned_field_argument_parent_v1(source, context)?;
        if invoke != self.parent_invoke || normal != self.parent_normal
            || context.ledger().returned_field_argument_binding_v1(source)? != self.read {
            return Err(freeze("field-argument/original-producer-drift"));
        }
        let base = self.receiver.value_for(call.owner(), loan.receiver_site().node(), loan.parent())
            .map_err(|issue| issue.to_string())?;
        self.receiver.loan_forwarded_copies(loan.parent(), parent)?;
        match self.read.1 {
            MirInstruction::ObjectFieldGet { dst, base: actual, field }
                if actual == base && field == source.field() && dst != base => Ok(dst),
            _ => Err(freeze("field-argument/read-operands")),
        }
    }
    pub(super) fn copies(&self, context: CallValidationContextV1<'_>) -> Result<&[Binding], String> {
        let (_, _, loan) = self.source.returned_argument_v1()
            .ok_or_else(|| freeze("field-argument/source-missing"))?;
        let parent = context.ledger().returned_field_argument_parent_v1(&self.source, context)?.0;
        self.receiver.loan_forwarded_copies(loan.parent(), parent)
    }
    pub(super) fn validate_anchor_ends_v1(
        &self, context: CallValidationContextV1<'_>, intervals: &interval::IntervalViewV1<'_>,
        projection: Option<&super::super::physical_boundary::FinishedBindings>, ends: &[Binding],
    ) -> Result<(), String> {
        if ends.is_empty() { return Ok(()); }
        let function = intervals.function();
        let child = self.value(&self.call, &self.source, context)?;
        let parent = context.ledger().returned_field_argument_parent_v1(&self.source, context)?.0;
        let normal = match projection {
            Some(projection) => projection.relation_binding(function, &self.parent_normal)?
                .ok_or_else(|| freeze("field-end/finished-parent-missing"))?,
            None => self.parent_normal.clone(),
        };
        let read = intervals.read(parent, child, self.source.field())?;
        for end in ends {
            intervals.require_before(&normal, end)?;
            read.check(&normal, end)?;
        }
        Ok(())
    }
    pub(super) fn validate_finished(
        &self, function: &MirFunction, projection: &super::super::physical_boundary::FinishedBindings,
        context: CallValidationContextV1<'_>, call: &OwnedExprSiteV1, outer: &Binding,
        coordinate: (BasicBlockId, usize), ordinal: u32,
    ) -> Result<(), String> {
        let child = self.value(call, &self.source, context)?;
        let (_, expected_ordinal, _) = self.source.returned_argument_v1()
            .ok_or_else(|| freeze("field-argument/source-missing"))?;
        if expected_ordinal != ordinal { return Err(freeze("field-argument/ordinal-drift")); }
        let map = |original: &Binding| -> Result<Binding, String> {
            let mapped = projection.binding(original.0, &original.1)?
                .ok_or_else(|| freeze("field-argument/finished-producer-removed"))?;
            if !projection.recorded().contains(&mapped) {
                return Err(freeze("field-argument/finished-unrecorded"));
            }
            Ok(mapped)
        };
        let parent_invoke = map(&self.parent_invoke)?;
        let parent_normal = projection.relation_binding(function, &self.parent_normal)?
            .ok_or_else(|| freeze("field-argument/finished-parent-removed"))?;
        let read = projection.relation_binding(function, &self.read)?
            .ok_or_else(|| freeze("field-argument/finished-read-removed"))?;
        let outer = map(outer)?;
        if interval::locate(function, &outer)? != coordinate {
            return Err(freeze("field-argument/finished-call-coordinate"));
        }
        let MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } = &outer.1 else {
            return Err(freeze("field-argument/finished-call-missing"));
        };
        if call.args.get(ordinal as usize) != Some(&child) {
            return Err(freeze("field-argument/finished-argument-slot"));
        }
        let parent = context.ledger().returned_field_argument_parent_v1(&self.source, context)?.0;
        if !matches!((&parent_invoke.1, &parent_normal.1),
            (MirInstruction::Invoke { normal_landing, operation: InvokeOperation::Call { result: crate::mir::instruction::InvokeCallResultKind::Handle, .. }, .. },
             MirInstruction::InvokeNormalResult { invoke_block, dst })
                if *invoke_block == parent_invoke.0 && *normal_landing == parent_normal.0 && *dst == parent)
        { return Err(freeze("field-argument/finished-parent-link")); }
        let intervals = interval::IntervalViewV1::new(function);
        intervals.require_before(&parent_invoke, &parent_normal)?;
        intervals.require_before(&parent_normal, &read)?;
        intervals.require_before(&read, &outer)?;
        // Copy omission is legal only under the captured boundary's permission.
        for copy in self.copies(context)? { projection.relation_binding(function, copy)?; }
        intervals.read(parent, child, self.source.field())?.check(&parent_normal, &outer)
    }
}
