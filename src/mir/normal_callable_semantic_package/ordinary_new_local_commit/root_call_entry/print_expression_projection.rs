//! Original expression-to-physical correspondence in the existing prefix pool.
//! This packet never issues fields, calls, input permissions or runtime owners.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use super::expression_projection::normal_order;
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
    pub(crate) fn new(
        frame: Rc<CheckedPrintExpressionFrameV1>,
        values: Vec<Option<Operand>>,
        calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
        path_reads: Vec<crate::mir::builder::ExactLexicalReadV1>,
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
            self.frame.evaluation(), &self.values, &self.calls, &self.path_reads, &self.original,
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

    fn require_unique(&self, matches: impl Fn(&MirInstruction) -> bool) -> Result<&Binding, String> {
        self.expression().require_unique(matches)
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
        let ledger = context.ledger();
        use super::super::root_cleanup_graph::ordered_paths as paths;
        use std::collections::BTreeSet;
        self.validate_source(context)?;
        let mapped = |original: &Binding| match finishing {
            Some(projection) => projection
                .binding(original.0, &original.1)?
                .ok_or_else(|| freeze("print-expression/required-producer-removed")),
            None => Ok(original.clone()),
        };
        for read in &self.path_reads {
            for original in read.original_copy_dependencies_v1()? {
                let actual = match finishing {
                    Some(projection) => projection.relation_binding(function, original)?,
                    None => Some(original.clone()),
                };
                if let Some((block, instruction)) = actual {
                    if !super::super::physical_boundary::check_binding(function, None, block, &instruction)? {
                        return Err(freeze("print-expression/path-copy-missing"));
                    }
                }
            }
        }
        for node in self.frame.source().nodes() {
            for original in ledger.checked_field_copy_dependencies_v1(
                &OwnedExprSiteV1::new(self.frame.source().owner(), node.site().clone()), context)? {
                let actual = match finishing {
                    Some(projection) => projection.relation_binding(function, &original)?,
                    None => Some(original),
                };
                if let Some((block, instruction)) = actual {
                    if !super::super::physical_boundary::check_binding(function, None, block, &instruction)? {
                        return Err(freeze("print-expression/field-copy-missing"));
                    }
                }
            }
        }
        let ordered = self
            .ordered_bindings_in_context(context)?
            .iter()
            .map(mapped)
            .collect::<Result<Vec<_>, _>>()?;
        let (_, print) = mapped(&self.print)?;
        let MirInstruction::Invoke {
            fault_frame,
            normal_landing: mut normal,
            ..
        } = print
        else {
            return Err(freeze("print-expression/print-invoke"));
        };
        let frame = self.require_unique(
            |row| matches!(row, MirInstruction::FaultFrameEnter { dst, .. } if *dst == fault_frame),
        )?;
        let expected_mode = if ledger.root_owner() == Some(self.frame.source().owner()) {
            crate::mir::instruction::FaultFrameMode::RootOwned
        } else {
            crate::mir::instruction::FaultFrameMode::Borrowed
        };
        if !matches!(frame.1, MirInstruction::FaultFrameEnter { mode, .. } if mode == expected_mode)
        {
            return Err(freeze("print-expression/frame-mode"));
        }
        let homes = ledger.prior_home_unwind_for(self.frame.prior_homes())?;
        let mut live = Vec::new();
        let mut covered = BTreeSet::new();
        let end_sequence = |live: &[ValueId]| {
            live.iter()
                .rev()
                .map(|value| InvokeOperation::String(Operation::End { value: *value }))
                .chain(homes.iter().cloned())
                .collect::<Vec<_>>()
        };
        for (id, instruction) in &ordered {
            let MirInstruction::Invoke {
                operation,
                fault_frame: actual,
                fault_landing,
                ..
            } = instruction
            else {
                continue;
            };
            if *actual != fault_frame {
                return Err(freeze("print-expression/frame-drift"));
            }
            covered.insert(*id);
            covered.extend(paths::pending_operations(
                function,
                *fault_landing,
                &end_sequence(&live),
                Some(fault_frame),
            )?);
            if matches!(operation, InvokeOperation::String(operation) if operation.owns_normal_result()) {
                let value = ordered
                    .iter()
                    .find_map(|(_, row)| match row {
                        MirInstruction::InvokeNormalResult { invoke_block, dst }
                            if invoke_block == id =>
                        {
                            Some(*dst)
                        }
                        _ => None,
                    })
                    .ok_or_else(|| freeze("print-expression/add-result"))?;
                live.push(value);
            }
        }
        let stop = match finishing {
            Some(projection) => projection
                .destination(self.continuation.0)
                .ok_or_else(|| freeze("print-expression/continuation-removed"))?,
            None => self.continuation.0,
        };
        while let Some(value) = live.pop() {
            normal = paths::skip_until(function, normal, Some(stop))?;
            let (next, fault) = paths::release(
                function,
                normal,
                &InvokeOperation::String(Operation::End { value }),
                Some(fault_frame),
            )?;
            covered.insert(normal);
            covered.extend(paths::pending_operations(
                function,
                fault,
                &end_sequence(&live),
                Some(fault_frame),
            )?);
            normal = next;
        }
        if paths::skip_until(function, normal, Some(stop))? != stop {
            return Err(freeze("print-expression/continue-path"));
        }
        // No recorded effect can hide behind a valid named source operation.
        for original in &self.original {
            if matches!(
                original.1,
                MirInstruction::Invoke {
                    operation: InvokeOperation::String(_) | InvokeOperation::Call { .. } | InvokeOperation::FieldGet { .. },
                    ..
                }
            ) {
                let actual = mapped(original)?;
                if !covered.contains(&actual.0) {
                    return Err(freeze("print-expression/unowned-operation"));
                }
            }
        }
        let actual_frame = mapped(frame)?;
        if !super::super::physical_boundary::check_binding(
            function,
            None,
            actual_frame.0,
            &actual_frame.1,
        )? {
            return Err(freeze("print-expression/frame-definition"));
        }
        normal_order(function, &ordered)?;
        Ok(covered)
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
