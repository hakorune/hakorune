//! Original expression-to-physical correspondence in the existing prefix pool.
//! This packet never issues fields, calls, input permissions or runtime owners.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use crate::mir::instruction::{StringInvokeOperand as Operand, StringInvokeOperation as Operation};
use crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(crate) struct EmittedPrintExpressionProjectionV1 {
    frame: Rc<CheckedPrintExpressionFrameV1>,
    values: Box<[Option<Operand>]>,
    calls: Box<[Rc<EmittedLexicalCallProjectionV1>]>,
    path_reads: Box<[crate::mir::builder::ExactLexicalReadV1]>,
    reference_reads: std::collections::BTreeMap<usize, super::expression_projection::ExpressionReferenceReadV1>,
    entry: Binding,
    print: Binding,
    continuation: Binding,
    original: Vec<Binding>,
}

#[cfg(test)]
#[path = "print_expression_projection_tests.rs"]
mod tests;

impl EmittedPrintExpressionProjectionV1 {
    pub(crate) fn frame(&self) -> &Rc<CheckedPrintExpressionFrameV1> {
        &self.frame
    }
    pub(crate) fn calls(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        self.calls.iter().map(Rc::as_ref)
    }

    /// Observations must come from the sole writer. Even a literal Print owes
    /// its exact checked Print and a real continuing edge, never a fake Call.
    pub(in crate::mir) fn new(
        frame: Rc<CheckedPrintExpressionFrameV1>,
        values: Vec<Option<Operand>>,
        calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
        path_reads: Vec<crate::mir::builder::ExactLexicalReadV1>,
        reference_reads: std::collections::BTreeMap<usize, super::expression_projection::ExpressionReferenceReadV1>,
        entry: Binding,
        print: Binding,
        continuation: Binding,
        original: Vec<Binding>,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        let packet = Self {
            frame,
            values: values.into_boxed_slice(),
            calls: calls.into_boxed_slice(),
            path_reads: path_reads.into_boxed_slice(),
            reference_reads,
            entry,
            print,
            continuation,
            original,
        };
        packet.validate_source(CallValidationContextV1::Lowering(ledger))?;
        packet.validate_recorded(&packet.original, None)?;
        let covered = packet.validate_paths(ledger, function, None)?;
        packet.validate_original_scope(ledger, function, &covered)?;
        Ok(packet)
    }

    fn expression(&self) -> super::expression_projection::ExpressionPhysicalProjectionRefV1<'_> {
        super::expression_projection::ExpressionPhysicalProjectionRefV1::new(
            self.frame.evaluation(), &self.values, &self.calls, &self.path_reads, &self.reference_reads, &self.original,
        )
    }

    fn validate_source(&self, context: CallValidationContextV1<'_>) -> Result<(), String> {
        self.expression().validate_source(context)?;
        if !matches!((&self.print.1, self.values.first()),
            (MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Print { value }), .. }, Some(Some(root))) if value == root)
            || !matches!(self.continuation.1, MirInstruction::Jump { .. })
        {
            return Err(freeze("print-expression/print-or-continuation"));
        }
        Ok(())
    }

    fn ordered_bindings(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<Vec<Binding>, String> {
        self.ordered_bindings_in_context(CallValidationContextV1::Lowering(ledger))
    }

    fn ordered_bindings_in_context(&self, context: CallValidationContextV1<'_>) -> Result<Vec<Binding>, String> {
        let mut ordered = self.expression().ordered_bindings_in_context(context)?;
        ordered.push(self.print.clone());
        Ok(ordered)
    }

    fn validate_original_scope(
        &self, ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
        covered: &std::collections::BTreeSet<BasicBlockId>,
    ) -> Result<(), String> {
        self.expression().validate_original_scope(ledger, function, covered,
            &self.entry, &self.print, &self.continuation, &self.ordered_bindings(ledger)?)
    }

    pub(super) fn validate_paths(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_paths_in_context(CallValidationContextV1::Lowering(ledger), function, finishing)
    }

    pub(super) fn validate_paths_in_context(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction, finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_source(context)?;
        self.expression().validate_paths_to_terminal(context, function, finishing, &self.print, &self.continuation, None)
    }

    pub(super) fn validate_recorded(
        &self, bindings: &[Binding],
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.expression().validate_recorded(bindings, finishing)?;
        if !self.original.contains(&self.entry)
            || !self.original.contains(&self.print)
            || !self.original.contains(&self.continuation)
        {
            return Err(freeze("print-expression/terminal-membership"));
        }
        Ok(())
    }
}
