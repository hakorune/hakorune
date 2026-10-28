//! Original expression-to-physical correspondence in the existing prefix pool.
//! This packet never issues fields, calls, input permissions or runtime owners.
use super::*;
use crate::mir::instruction::StringInvokeOperation as Operation;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(crate) struct EmittedPrintExpressionProjectionV1 {
    frame: Rc<CheckedPrintExpressionFrameV1>,
    values: Box<[Option<EvaluatedExpressionValueV1>]>,
    calls: Box<[Rc<EmittedLexicalCallProjectionV1>]>,
    path_reads: Box<[std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>]>,
    reference_reads:
        std::collections::BTreeMap<usize, super::expression_projection::ExpressionReferenceReadV1>,
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
        values: Vec<Option<EvaluatedExpressionValueV1>>,
        calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
        path_reads: Vec<std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>>,
        reference_reads: std::collections::BTreeMap<
            usize,
            super::expression_projection::ExpressionReferenceReadV1,
        >,
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

    pub(super) fn expression(&self) -> super::expression_projection::ExpressionPhysicalProjectionRefV1<'_> {
        super::expression_projection::ExpressionPhysicalProjectionRefV1::new(
            self.frame.evaluation(),
            &self.values,
            &self.calls,
            &self.path_reads,
            &self.reference_reads,
            &self.original,
        )
    }

    fn validate_source(&self, context: CallValidationContextV1<'_>) -> Result<(), String> {
        self.expression().validate_source(context)?;
        let sink = if self.frame.is_discard() {
            matches!(self.print.1, MirInstruction::Jump { edge_args: None, .. })
        } else {
            matches!((&self.print.1, self.values.first()),
                (MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Print { value }), .. }, Some(Some(root))) if root.as_operand().is_ok_and(|root| value == root))
        };
        if !sink || !matches!(self.continuation.1, MirInstruction::Jump { .. })
        {
            return Err(freeze("print-expression/print-or-continuation"));
        }
        Ok(())
    }

    fn ordered_bindings(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<Vec<Binding>, String> {
        self.ordered_bindings_in_context(CallValidationContextV1::Lowering(ledger))
    }

    fn ordered_bindings_in_context(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let mut ordered = self.expression().ordered_bindings_in_context(context)?;
        if !self.frame.is_discard() {
            ordered.push(self.print.clone());
        }
        Ok(ordered)
    }

    fn validate_original_scope(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        covered: &std::collections::BTreeSet<BasicBlockId>,
    ) -> Result<(), String> {
        self.expression().validate_original_scope(
            ledger,
            function,
            covered,
            &self.entry,
            &self.print,
            &self.continuation,
            &self.ordered_bindings(ledger)?,
        )
    }

    pub(super) fn validate_paths(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_paths_in_context(
            CallValidationContextV1::Lowering(ledger),
            function,
            finishing,
        )
    }

    pub(super) fn validate_paths_in_context(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_source(context)?;
        self.expression().validate_paths_to_terminal(
            context,
            function,
            finishing,
            &self.print,
            &self.continuation,
            None,
            None,
        )
    }

    pub(super) fn validate_recorded(
        &self,
        bindings: &[Binding],
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

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn print_expression_frame_v1(
        &self,
        owner: FunctionOwnerIdV1,
        statement: &SourceStmtSiteV1,
    ) -> Result<
        Option<
            std::rc::Rc<
                crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1,
            >,
        >,
        String,
    > {
        let Some(flow) = self
            .completion_for_owner(owner)
            .and_then(|completion| completion.cleanup().root_flow())
        else {
            return Ok(None);
        };
        let mut frames = flow
            .expression_plans()
            .iter()
            .filter_map(|plan| plan.print_frame())
            .filter(|frame| frame.source().root().statement() == statement);
        let frame = frames.next().cloned();
        if frame.is_none() && flow.statement_prefix_unavailable(statement) {
            return Err(freeze("print-expression/source-unavailable"));
        }
        if frames.next().is_some() {
            return Err(freeze("print-expression/duplicate-source-frame"));
        }
        Ok(frame)
    }

    /// The retained frame of a bare statement call that drops an owned result.
    /// Absent for every other statement, which never reports unavailability here.
    pub(crate) fn discard_expression_frame_v1(
        &self,
        owner: FunctionOwnerIdV1,
        statement: &SourceStmtSiteV1,
    ) -> Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1>> {
        let flow = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow())?;
        flow.expression_plans().iter().filter_map(|plan| plan.print_frame())
            .find(|frame| frame.is_discard() && frame.source().root().statement() == statement)
            .cloned()
    }

    pub(crate) fn validate_print_expression_source_v1(
        &self,
        frame: &std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1,
        >,
    ) -> Result<(), String> {
        let owner = frame.source().owner();
        let sources = self.expected_local_call_binding_sources(owner)?;
        let groups = self.root_local_call_bindings.borrow();
        let count = groups.get(&owner).map_or(0, Vec::len);
        if !matches!(sources.get(count), Some(LifecyclePrefixSourceV1::PrintExpression { frame: expected, .. })
            if std::rc::Rc::ptr_eq(frame, expected))
        {
            return Err(freeze("print-expression/source-prefix-order"));
        }
        Ok(())
    }

    /// The same pool owns a complete expression prefix. Source/frame identity
    /// and ordering are checked before this pending group can be published.
    pub(crate) fn prepare_print_expression_bindings(
        &self,
        owner: FunctionOwnerIdV1,
        packet: std::rc::Rc<EmittedPrintExpressionProjectionV1>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        self.prepare_local_call_binding_group(
            owner,
            RootLifecyclePrefixBindingGroupV1::new_print(packet, bindings)?,
        )
    }

}
