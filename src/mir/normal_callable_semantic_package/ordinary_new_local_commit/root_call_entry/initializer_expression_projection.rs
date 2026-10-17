//! Source selection for the same Local initializer physical consumer.
use super::*;
use std::rc::Rc;
use crate::mir::instruction::StringInvokeOperand as Operand;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1;
type Binding = (BasicBlockId, MirInstruction);


impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn initializer_expression_frame_v1(
        &self, owner: FunctionOwnerIdV1, statement: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1>>, String> {
        let Some(flow) = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow()) else { return Ok(None) };
        let mut frames = flow.expression_plans().iter().filter_map(|plan| plan.initializer_frame())
            .filter(|frame| frame.source().root().statement().node() == statement);
        let frame = frames.next().cloned();
        if frames.next().is_some() { return Err(freeze("text-initializer/duplicate-source-frame")); }
        if frame.as_ref().is_some_and(|frame| frame.source().owner() != owner) { return Err(freeze("text-initializer/source-owner")); }
        Ok(frame)
    }

}

/// The completed Local receipt and the sole original evaluation share one pool.
#[derive(Debug)]
pub(crate) struct EmittedInitializerExpressionProjectionV1 {
    frame: Rc<CheckedInitializerExpressionFrameV1>,
    value: ValueId,
    local: ValueId,
    copy: Option<Binding>,
    values: Box<[Option<Operand>]>,
    calls: Box<[Rc<EmittedLexicalCallProjectionV1>]>,
    path_reads: Box<[crate::mir::builder::ExactLexicalReadV1]>,
    reference_reads: std::collections::BTreeMap<usize, super::expression_projection::ExpressionReferenceReadV1>,
    entry: Binding,
    terminal: Binding,
    continuation: Binding,
    original: Vec<Binding>,
}
impl EmittedInitializerExpressionProjectionV1 {
    pub(crate) fn frame(&self) -> &Rc<CheckedInitializerExpressionFrameV1> { &self.frame }
    pub(crate) fn value(&self) -> ValueId { self.value }
    pub(crate) fn calls(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> { self.calls.iter().map(Rc::as_ref) }
    pub(in crate::mir) fn new(frame: Rc<CheckedInitializerExpressionFrameV1>, value: ValueId, local: ValueId, copy: Option<Binding>,
        values: Vec<Option<Operand>>, calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
        path_reads: Vec<crate::mir::builder::ExactLexicalReadV1>,
        reference_reads: std::collections::BTreeMap<usize, super::expression_projection::ExpressionReferenceReadV1>,
        entry: Binding, terminal: Binding, continuation: Binding, original: Vec<Binding>,
        ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<Self, String> {
        let packet = Self { frame, value, local, copy, values: values.into_boxed_slice(), calls: calls.into_boxed_slice(),
            path_reads: path_reads.into_boxed_slice(), reference_reads, entry, terminal, continuation, original };
        packet.validate_source(CallValidationContextV1::Lowering(ledger))?;
        let covered = packet.validate_paths(CallValidationContextV1::Lowering(ledger), function, None)?;
        let mut ordered = packet.expression().ordered_bindings_in_context(CallValidationContextV1::Lowering(ledger))?;
        ordered.push(packet.terminal.clone());
        packet.expression().validate_original_scope(ledger, function, &covered, &packet.entry, &packet.terminal,
            &packet.continuation, &ordered)?;
        Ok(packet)
    }
    fn expression(&self) -> super::expression_projection::ExpressionPhysicalProjectionRefV1<'_> {
        super::expression_projection::ExpressionPhysicalProjectionRefV1::new(self.frame.evaluation(), &self.values,
            &self.calls, &self.path_reads, &self.reference_reads, &self.original)
    }
    fn validate_source(&self, context: CallValidationContextV1<'_>) -> Result<(), String> {
        let selected = context.ledger().initializer_expression_frame_v1(self.frame.source().owner(), self.frame.source().root().statement().node())?
            .ok_or_else(|| freeze("initializer-expression/source-missing"))?;
        if !Rc::ptr_eq(&selected, &self.frame) || !self.frame.is_i64()
            || self.values.first() != Some(&Some(Operand::NormalInteger(self.value)))
            || !matches!(self.terminal.1, MirInstruction::Jump { edge_args: None, .. })
            || !matches!(self.continuation.1, MirInstruction::Jump { edge_args: None, .. }) {
            return Err(freeze("initializer-expression/source-or-result"));
        }
        if self.copy.as_ref().is_none_or(|(_, row)| !matches!(row, MirInstruction::Copy { dst, src }
            if *dst == self.local && *src == self.value)) { return Err(freeze("initializer-expression/local-copy-receipt")); }
        self.expression().validate_source(context)
    }
    pub(super) fn validate_recorded(&self, bindings: &[Binding], finishing: Option<&super::super::physical_boundary::FinishedBindings>) -> Result<(), String> {
        self.expression().validate_recorded(bindings, finishing)?;
        for expected in [&self.entry, &self.terminal, &self.continuation] {
            if !self.original.contains(expected) { return Err(freeze("initializer-expression/structural-membership")); }
        }
        if finishing.is_none() && self.copy.as_ref().is_some_and(|copy| bindings.iter().filter(|row| *row == copy).count() != 1) {
            return Err(freeze("initializer-expression/local-copy-membership"));
        }
        Ok(())
    }
    pub(super) fn validate_paths(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_source(context)?;
        if let Some(copy) = &self.copy {
            let actual = match (finishing, context) {
                (Some(boundary), CallValidationContextV1::Finalized(_)) => boundary.relation_binding(function, copy)?,
                (Some(boundary), CallValidationContextV1::Lowering(_)) => {
                    if boundary.captured_definition(self.local)? != copy { return Err(freeze("initializer-expression/local-copy-capture")); }
                    None
                }
                (None, _) => Some(copy.clone()),
            };
            if let Some((block, instruction)) = actual {
                let mut definitions = function.blocks.iter().flat_map(|(id, body)| body.all_instructions()
                    .map(move |row| (*id, row))).filter(|(_, row)| row.dst_value() == Some(self.local));
                if function.params.contains(&self.local) || definitions.next() != Some((block, &instruction)) || definitions.next().is_some() {
                    return Err(freeze("initializer-expression/local-definition-identity"));
                }
                if !super::super::physical_boundary::check_binding(function, None, block, &instruction)? {
                    return Err(freeze("initializer-expression/local-copy-missing"));
                }
            }
        }
        self.expression().validate_scalar_continuation_v1(context, function, finishing, &self.entry, &self.terminal, &self.continuation)
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn prepare_initializer_expression_bindings(&self, packet: Rc<EmittedInitializerExpressionProjectionV1>,
        bindings: Vec<Binding>,
    ) -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        self.prepare_local_call_binding_group(packet.frame.source().owner(), RootLifecyclePrefixBindingGroupV1::new_initializer(packet, bindings)?)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn validate_initializer_expression_source_v1(&self, frame: &Rc<CheckedInitializerExpressionFrameV1>) -> Result<(), String> {
        let sources = self.expected_local_call_binding_sources(frame.source().owner())?;
        let count = self.root_local_call_bindings.borrow().get(&frame.source().owner()).map_or(0, Vec::len);
        if !matches!(sources.get(count), Some(LifecyclePrefixSourceV1::InitializerExpression { frame: original, .. }) if Rc::ptr_eq(original, frame)) {
            return Err(freeze("initializer-expression/source-prefix-order"));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "initializer_expression_projection_tests.rs"]
mod tests;
