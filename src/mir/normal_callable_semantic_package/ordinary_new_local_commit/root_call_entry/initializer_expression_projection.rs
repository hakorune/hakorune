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
    install: Option<Binding>,
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
    pub(super) fn corroborate_loop_frame_v2(&self, entry: BasicBlockId, end: BasicBlockId, value: ValueId) -> Result<(), String> {
        if self.entry.0 != entry || self.value() != value
            || !matches!(self.continuation.1, MirInstruction::Jump { target, edge_args: None } if target == end) {
            return Err(freeze("source-loop/binding-frame-correspondence"));
        }
        Ok(())
    }
    pub(crate) fn frame(&self) -> &Rc<CheckedInitializerExpressionFrameV1> { &self.frame }
    pub(crate) fn value(&self) -> ValueId { self.value }
    pub(in crate::mir::normal_callable_semantic_package) fn local_value(&self) -> ValueId { self.local }
    pub(in crate::mir::normal_callable_semantic_package) fn install_binding(&self) -> Option<&Binding> { self.install.as_ref() }
    pub(in crate::mir::normal_callable_semantic_package) fn has_text_home(&self) -> bool {
        self.frame.is_text_residence() && self.value == self.local && self.copy.is_none()
            && self.install.as_ref().is_some_and(|(_, row)| matches!(row,
                MirInstruction::StringResidenceInstall { value, replaces: None } if *value == self.value))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn root_acquisition_binding_v1(&self) -> Result<&Binding, String> {
        if !self.has_text_home() { return Err(freeze("initializer-expression/no-text-home")); }
        self.expression().require_unique(|row| matches!(row,
            MirInstruction::InvokeNormalResult { dst, .. } if *dst == self.value))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn receipt_copy_binding_v1(&self) -> Option<&Binding> { self.copy.as_ref() }
    /// Only this completed receipt can borrow the existing source-Copy omission
    /// law. Finalized validate_paths still requires sealed relation_binding.
    pub(super) fn validates_receipt_copy_v1(&self, observed: &Binding, function: &MirFunction,
        boundary: &super::super::physical_boundary::FinishedBindings, projected: bool,
    ) -> Result<bool, String> {
        let Some(copy) = &self.copy else { return Ok(false) };
        let expected = if projected { boundary.binding(copy.0, &copy.1)? } else { Some(copy.clone()) };
        if expected.as_ref() != Some(observed) { return Ok(false); }
        if boundary.captured_definition(self.local)? != copy
            || boundary.source_local_copy_binding_v1(function, self.local, self.value)?.as_ref() != Some(copy) {
            return Err(freeze("initializer-expression/local-copy-correspondence"));
        }
        Ok(true)
    }
    pub(crate) fn calls(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> { self.calls.iter().map(Rc::as_ref) }
    pub(in crate::mir) fn new(frame: Rc<CheckedInitializerExpressionFrameV1>, value: ValueId, local: ValueId, copy: Option<Binding>,
        values: Vec<Option<Operand>>, calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
        path_reads: Vec<crate::mir::builder::ExactLexicalReadV1>,
        reference_reads: std::collections::BTreeMap<usize, super::expression_projection::ExpressionReferenceReadV1>,
        entry: Binding, terminal: Binding, continuation: Binding, original: Vec<Binding>,
        ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<Self, String> {
        let install = original.iter().find(|(_, row)| matches!(row, MirInstruction::StringResidenceInstall { .. })).cloned();
        let packet = Self { frame, value, local, copy, install, values: values.into_boxed_slice(), calls: calls.into_boxed_slice(),
            path_reads: path_reads.into_boxed_slice(), reference_reads, entry, terminal, continuation, original };
        packet.validate_source(CallValidationContextV1::Lowering(ledger))?;
        let covered = packet.validate_paths(CallValidationContextV1::Lowering(ledger), function, None)?;
        let mut ordered = packet.expression().ordered_bindings_in_context(CallValidationContextV1::Lowering(ledger))?;
        if let Some(install) = &packet.install { ordered.push(install.clone()); }
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
        let expected = if self.frame.is_i64() { Operand::NormalInteger(self.value) }
            else if self.has_text_home() { Operand::OwnedString(self.value) }
            else { return Err(freeze("initializer-expression/source-or-result")); };
        if !Rc::ptr_eq(&selected, &self.frame) || self.values.first() != Some(&Some(expected))
            || !matches!(self.terminal.1, MirInstruction::Jump { edge_args: None, .. })
            || !matches!(self.continuation.1, MirInstruction::Jump { edge_args: None, .. }) {
            return Err(freeze("initializer-expression/source-or-result"));
        }
        let installs = self.original.iter().filter(|(_, row)| matches!(row, MirInstruction::StringResidenceInstall { .. })).count();
        if installs != usize::from(self.has_text_home()) { return Err(freeze("initializer-expression/install-census")); }
        if self.frame.is_i64() && self.copy.as_ref().is_none_or(|(_, row)| !matches!(row, MirInstruction::Copy { dst, src }
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
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_paths(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
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
        self.expression().validate_local_continuation_v1(context, function, finishing, &self.entry, &self.terminal, &self.continuation, self.install.as_ref(), None)
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
        let site = OwnedExprSiteV1::new(frame.source().owner(), frame.source().nodes()[0].site().clone());
        let source = self.next_prefix_source_v2(frame.source().owner(), &site)?;
        if !matches!(source.as_ref(), Some(LifecyclePrefixSourceV1::InitializerExpression { frame: original, .. }) if Rc::ptr_eq(original, frame)) {
            return Err(freeze("initializer-expression/source-prefix-order"));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "initializer_expression_projection_tests.rs"]
mod tests;
