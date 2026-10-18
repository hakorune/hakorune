//! Common Normal/Fault temporary cleanup correspondence for expression purposes.
use super::*;
impl ExpressionPhysicalProjectionRefV1<'_> {
    pub(in super::super) fn validate_paths_to_terminal(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        terminal: &Binding, continuation: &Binding, residence: Option<&Binding>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        let ledger = context.ledger();
        use super::super::root_cleanup_graph::ordered_paths as paths;
        self.validate_source(context)?;
        self.validate_reference_dependencies(context, function, finishing)?;
        let mapped = |original: &Binding| match finishing {
            Some(projection) => projection
                .binding(original.0, &original.1)?
                .ok_or_else(|| freeze("print-expression/required-producer-removed")),
            None => Ok(original.clone()),
        };
        for read in self.path_reads {
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
        let mut original_order = self.ordered_bindings_in_context(context)?;
        let (mut normal, fault_frame) = match &terminal.1 {
            MirInstruction::Invoke { fault_frame, .. } => {
                original_order.push(terminal.clone());
                let (_, current) = mapped(terminal)?;
                let MirInstruction::Invoke { normal_landing, .. } = current else { return Err(freeze("expression/terminal-invoke")); };
                (normal_landing, *fault_frame)
            }
            MirInstruction::Jump { target, edge_args: None } => {
                let frame = self.original.iter().find_map(|(_, row)| match row {
                    MirInstruction::FaultFrameEnter { dst, .. } => Some(*dst), _ => None,
                }).ok_or_else(|| freeze("expression/local-fault-frame"))?;
                let normal = match finishing { Some(projection) => projection.destination(*target)
                    .ok_or_else(|| freeze("expression/local-normal-target"))?, None => *target };
                (normal, frame)
            }
            _ => return Err(freeze("expression/terminal-shape")),
        };
        if let Some(install) = residence { original_order.push(install.clone()); }
        let ordered = original_order.iter().map(mapped).collect::<Result<Vec<_>, _>>()?;
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
        let (mut live, mut covered) = self.validate_acquisition_paths_v1(
            function, &ordered, &homes, fault_frame,
        )?;
        let installed = if let Some(original) = residence {
            let MirInstruction::StringResidenceInstall { value } = original.1 else { return Err(freeze("expression/residence-install-shape")); };
            if self.values.first() != Some(&Some(Operand::OwnedString(value))) || live.pop() != Some(value) {
                return Err(freeze("expression/residence-original-root"));
            }
            let actual = mapped(original)?;
            if !super::super::physical_boundary::check_binding(function, None, actual.0, &actual.1)? {
                return Err(freeze("expression/residence-install-missing"));
            }
            Some(value)
        } else { None };
        let end_sequence = |live: &[ValueId]| {
            live.iter().rev().copied().chain(installed).map(|value|
                InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value }))
                .chain(homes.iter().cloned()).collect::<Vec<_>>()
        };
        let stop = match finishing {
            Some(projection) => projection
                .destination(continuation.0)
                .ok_or_else(|| freeze("print-expression/continuation-removed"))?,
            None => continuation.0,
        };
        while let Some(value) = live.pop() {
            normal = paths::skip_until(function, normal, Some(stop))?;
            let (next, fault) = paths::release(
                function,
                normal,
                &InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value }),
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
        for original in self.original {
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
}

impl ExpressionPhysicalProjectionRefV1<'_> {
    /// Shared physical operation/Fault correspondence. The purpose retains its
    /// terminal and Normal lifetime policy; this walk issues no source rights.
    fn validate_acquisition_paths_v1(
        &self, function: &MirFunction, ordered: &[Binding], homes: &[InvokeOperation],
        fault_frame: ValueId,
    ) -> Result<(Vec<ValueId>, std::collections::BTreeSet<BasicBlockId>), String> {
        use super::super::root_cleanup_graph::ordered_paths as paths;
        use std::collections::BTreeSet;
        let mut live = Vec::new();
        let mut covered = BTreeSet::new();
        let end_sequence = |live: &[ValueId]| {
            live.iter()
                .rev()
                .map(|value| InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value: *value }))
                .chain(homes.iter().cloned())
                .collect::<Vec<_>>()
        };
        for (id, instruction) in ordered {
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
        Ok((live, covered))
    }
}

impl ExpressionPhysicalProjectionRefV1<'_> {
    pub(in crate::mir) fn validate_scalar_continuation_v1(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>, entry: &Binding, terminal: &Binding, continuation: &Binding,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_local_continuation_v1(context, function, finishing, entry, terminal, continuation, None)
    }
    pub(in crate::mir) fn validate_local_continuation_v1(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>, entry: &Binding, terminal: &Binding, continuation: &Binding,
        residence: Option<&Binding>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        for original in [entry, terminal, continuation] {
            match finishing {
                Some(boundary) => { boundary.structural_jump_binding(function, original,
                    matches!(context, CallValidationContextV1::Finalized(_)))?; }
                None => {
                    if function.blocks.get(&original.0).and_then(|block| block.terminator.as_ref()) != Some(&original.1) {
                        return Err(freeze("initializer-expression/structural-jump-drift"));
                    }
                }
            }
        }
        self.validate_paths_to_terminal(context, function, finishing, terminal, continuation, residence)
    }
}
