//! Common Normal/Fault temporary cleanup correspondence for expression purposes.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{LocalCallArgumentV1, LocalCallObservationV1, ArgumentCallObservationV1, CheckedArgumentExpressionFrameV1};
use crate::mir::normal_callable_semantic_package::LexicalCallArgumentProjectionV1;
/// Borrow original ordered arguments and live Homes across Call purposes;
/// an expression result never gains a synthetic local destination.
#[derive(Clone, Copy)]
pub(in crate::mir) struct ExpressionCallArgumentSourceV1<'a> {
    site: &'a OwnedExprSiteV1,
    arguments: &'a [LocalCallArgumentV1],
    prior_homes: &'a [LiveHomeV1],
}
impl<'a> ExpressionCallArgumentSourceV1<'a> {
    pub(in crate::mir) fn from_local(call: &'a LocalCallObservationV1) -> Self {
        Self { site: call.site(), arguments: call.arguments(), prior_homes: call.prior_homes() }
    }
    pub(in crate::mir) fn from_nested(call: &'a ArgumentCallObservationV1) -> Self {
        Self { site: call.site(), arguments: call.arguments(), prior_homes: call.prior_homes() }
    }
    pub(in crate::mir) fn for_owned_array_candidate_v1(
        ledger: &'a OrdinaryNewClaimLedgerV1,
        source: &'a crate::mir::normal_callable_semantic_package::OwnedArrayAppendSourceLoanV1,
        row: &'a crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1,
    ) -> Result<Self, String> {
        source.check_candidate_v1(row)?;
        if source.candidate() != *row.call_site() {
            return Err(freeze("argument-expression/prefix-original-call"));
        }
        Ok(Self { site: row.call_site(), arguments: ledger.object_packet_arguments_v1(row)?,
            prior_homes: source.prior_homes()? })
    }
    pub(in crate::mir) fn site(self) -> &'a OwnedExprSiteV1 { self.site }
    pub(in crate::mir) fn arguments(self) -> &'a [LocalCallArgumentV1] { self.arguments }
    pub(in crate::mir) fn prior_homes(self) -> &'a [LiveHomeV1] { self.prior_homes }
}
/// A synchronous loan of the parent's already checked live stack at one Call.
/// It carries no source permission and never survives into a child packet.
pub(in crate::mir) struct ExpressionCallTransientPrefixV1<'a> {
    origin: TransientPrefixOriginV1<'a>,
    parent: Option<&'a ExpressionCallTransientPrefixV1<'a>>,
}
enum TransientPrefixOriginV1<'a> {
    Emitted { call: &'a EmittedLexicalCallProjectionV1, live: &'a [ValueId],
        prior_homes: &'a [LiveHomeV1] },
    Arguments { call: ExpressionCallArgumentSourceV1<'a>, ordinal: usize,
        earlier: &'a [LexicalCallArgumentProjectionV1], context: CallValidationContextV1<'a> },
    Evaluation { frame: &'a CheckedExpressionEvaluationV1,
        call: &'a crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        acquired: &'a [(usize, ValueId)] },
}
impl<'a> ExpressionCallTransientPrefixV1<'a> {
    pub(in crate::mir) fn from_evaluation(
        frame: &'a CheckedExpressionEvaluationV1,
        call: &'a crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        acquired: &'a [(usize, ValueId)], values: &[Option<Operand>], function: &MirFunction,
        parent: Option<&'a Self>,
    ) -> Result<Self, String> {
        validate_expression_acquisition_prefix_v1(frame, Some(call), acquired, values, function)?;
        if let Some(parent) = parent { parent.for_argument(frame)?; }
        Ok(Self { origin: TransientPrefixOriginV1::Evaluation { frame, call, acquired }, parent })
    }
    pub(in crate::mir) fn for_prepared_argument(
        frame: &'a std::rc::Rc<CheckedArgumentExpressionFrameV1>,
        earlier: &'a [LexicalCallArgumentProjectionV1], context: CallValidationContextV1<'a>,
        parent: Option<&'a Self>,
    ) -> Result<Self, String> {
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal, .. } = frame.source().root()
            else { return Err(freeze("argument-expression/transient-purpose")) };
        let site = OwnedExprSiteV1::new(frame.source().owner(), call.site().clone());
        let original = context.ledger().qualified_argument_expression_frame_v1(&site, *ordinal,
            frame.source().nodes().first().ok_or_else(|| freeze("argument-expression/root-missing"))?.site())?;
        if earlier.len() != *ordinal as usize || !std::rc::Rc::ptr_eq(&original, frame) {
            return Err(freeze("argument-expression/prefix-ordinal-frame"));
        }
        let call = context.ledger().local_call_for_owner(site.owner(), site.site())
            .ok_or_else(|| freeze("argument-expression/prefix-original-call"))?;
        Self::for_prepared_call_argument(ExpressionCallArgumentSourceV1::from_local(call), *ordinal as usize, earlier, context, parent)
    }
    /// The original Call/position also covers a CallResult argument, which has
    /// no expression frame. Callers lend the same source observation used by
    /// their packet; this synchronous view never becomes stored authority.
    pub(in crate::mir) fn for_prepared_call_argument(
        call: ExpressionCallArgumentSourceV1<'a>, ordinal: usize,
        earlier: &'a [LexicalCallArgumentProjectionV1], context: CallValidationContextV1<'a>,
        parent: Option<&'a Self>,
    ) -> Result<Self, String> {
        if ordinal >= call.arguments().len() || earlier.len() != ordinal {
            return Err(freeze("argument-expression/prefix-ordinal-frame"));
        }
        for (position, packet) in earlier.iter().enumerate() {
            packet.require_argument_frame_position_v1(call.site(), position, call.prior_homes(), context)?;
            packet.deferred_text_values_v1(context)?;
        }
        if let Some(parent) = parent { parent.unwind_for_call(call.site(), call.prior_homes())?; }
        let prefix = Self { origin: TransientPrefixOriginV1::Arguments { call, ordinal, earlier, context }, parent };
        prefix.operations()?;
        Ok(prefix)
    }
    fn contains(&self, site: &OwnedExprSiteV1) -> Result<bool, String> {
        fn in_arguments(arguments: &[LocalCallArgumentV1], site: &OwnedExprSiteV1) -> bool {
            arguments.iter().any(|argument| matches!(argument, LocalCallArgumentV1::CallResult(inner)
                if inner.site() == site || in_arguments(inner.arguments(), site)))
        }
        Ok(match &self.origin {
            TransientPrefixOriginV1::Emitted { call, .. } => call.retained_node_at_v1(site)?.is_some(),
            TransientPrefixOriginV1::Evaluation { call, .. } => call.site() == site || in_arguments(call.arguments(), site),
            TransientPrefixOriginV1::Arguments { call, ordinal, .. } =>
                call.site() == site || matches!(call.arguments().get(*ordinal),
                    Some(LocalCallArgumentV1::CallResult(inner))
                        if inner.site() == site || in_arguments(inner.arguments(), site)),
        })
    }
    fn prior_homes(&self) -> &[LiveHomeV1] {
        match &self.origin {
            TransientPrefixOriginV1::Emitted { prior_homes, .. } => prior_homes,
            TransientPrefixOriginV1::Evaluation { frame, .. } => frame.prior_homes(),
            TransientPrefixOriginV1::Arguments { call, .. } => call.prior_homes(),
        }
    }
    pub(in super::super) fn require_call(&self, call: &EmittedLexicalCallProjectionV1) -> Result<(), String> {
        if !self.contains(call.call_site())? || matches!(&self.origin,
            TransientPrefixOriginV1::Emitted { call: original, .. }
                if !original.retained_node_at_v1(call.call_site())?.is_some_and(|packet| std::ptr::eq(packet, call))) {
            return Err(freeze("argument-expression/transient-call"));
        }
        Ok(())
    }
    fn operations(&self) -> Result<Vec<InvokeOperation>, String> {
        let end = |value| InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value });
        let mut operations: Vec<_> = match &self.origin {
            TransientPrefixOriginV1::Emitted { live, .. } => live.iter().rev().map(|value| end(*value)).collect(),
            TransientPrefixOriginV1::Evaluation { acquired, .. } => acquired.iter().rev().map(|(_, value)| end(*value)).collect(),
            TransientPrefixOriginV1::Arguments { earlier, context, .. } => {
                let mut operations = Vec::new();
                for argument in earlier.iter().rev() {
                    operations.extend(argument.deferred_text_values_v1(*context)?.into_iter().rev().map(end));
                }
                operations
            },
        };
        if let Some(parent) = self.parent { operations.extend(parent.operations()?); }
        // Every acquisition is affine across the complete borrowed prefix,
        // including different arguments and ancestors. Never deduplicate an
        // overlapping stack: that would hide a double owner and alter faults.
        let mut seen = std::collections::BTreeSet::new();
        for operation in &operations {
            if let InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value }) = operation {
                if !seen.insert(*value) { return Err(freeze("argument-expression/prefix-duplicate-acquisition")); }
            }
        }
        Ok(operations)
    }
    pub(in crate::mir) fn unwind_for_call(&self, site: &OwnedExprSiteV1,
        homes: &[LiveHomeV1]) -> Result<Vec<InvokeOperation>, String> {
        if self.prior_homes() != homes || !self.contains(site)? {
            return Err(freeze("expression-call/context-drift"));
        }
        self.operations()
    }
    pub(in crate::mir) fn for_argument(&self, frame: &CheckedExpressionEvaluationV1)
        -> Result<Vec<InvokeOperation>, String> {
        if let TransientPrefixOriginV1::Arguments { call, ordinal, context, .. } = &self.origin {
            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call: original, ordinal: position, .. } = frame.source().root()
                else { return Err(freeze("argument-expression/transient-purpose")) };
            let site = frame.source().nodes().first().ok_or_else(|| freeze("argument-expression/root-missing"))?.site();
            let canonical = context.ledger().qualified_argument_expression_frame_v1(call.site(), *position, site)?;
            if call.site().owner() != frame.source().owner() || call.site().site() != original.site()
                || *ordinal != *position as usize || !std::ptr::eq(canonical.evaluation(), frame) {
                return Err(freeze("argument-expression/prefix-current-frame"));
            }
        }
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, .. } = frame.source().root()
            else { return Err(freeze("argument-expression/transient-purpose")) };
        self.unwind_for_call(&OwnedExprSiteV1::new(frame.source().owner(), call.site().clone()), frame.prior_homes())
    }
}

impl ExpressionPhysicalProjectionRefV1<'_> {
    pub(in super::super) fn validate_paths_to_terminal(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        terminal: &Binding, continuation: &Binding, residence: Option<&Binding>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_paths_with_policy(context, function, finishing, terminal, continuation, residence, inherited, false, None, None)
    }
    fn validate_paths_with_policy(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        terminal: &Binding, continuation: &Binding, residence: Option<&Binding>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>, defer_owned_argument: bool, displaced: Option<&InvokeOperation>,
        purpose_acquisition: Option<&[Binding; 2]>,
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
        if let Some(pair) = purpose_acquisition { original_order.extend(pair.iter().cloned()); }
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
        let mut homes = inherited.map(|prefix| prefix.for_argument(self.frame)).transpose()?.unwrap_or_default();
        homes.extend(ledger.prior_home_unwind_for(self.frame.prior_homes())?);
        let (mut live, mut covered) = self.validate_acquisition_paths_v1(
            context, function, finishing, &ordered, &homes, fault_frame, inherited,
        )?;
        if defer_owned_argument {
            let root = purpose_acquisition.and_then(|pair| pair[1].1.dst_value())
                .or_else(|| match self.values.first() { Some(Some(Operand::OwnedString(value))) => Some(*value), _ => None })
                .ok_or_else(|| freeze("argument-expression/deferred-root"))?;
            if residence.is_some() || (purpose_acquisition.is_none() && !self.frame.owned_result_nodes().contains(&0))
                || live.last() != Some(&root)
                || !matches!(self.frame.source().root(),
                    crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { .. }) {
                return Err(freeze("argument-expression/deferred-source-acquisition"));
            }
        }
        let installed = if let Some(original) = residence {
            let MirInstruction::StringResidenceInstall { value, replaces } = original.1 else { return Err(freeze("expression/residence-install-shape")); };
            let expected_previous = match displaced {
                Some(InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value })) => Some(*value),
                Some(_) => return Err(freeze("expression/residence-replacement-kind")),
                None => None,
            };
            let effective = purpose_acquisition.and_then(|pair| pair[1].1.dst_value())
                .map(Operand::OwnedString).or_else(|| self.values.first().cloned().flatten());
            if replaces != expected_previous || effective != Some(Operand::OwnedString(value)) || live.pop() != Some(value) {
                return Err(freeze("expression/residence-original-root"));
            }
            let actual = mapped(original)?;
            if !super::super::physical_boundary::check_binding(function, None, actual.0, &actual.1)? {
                return Err(freeze("expression/residence-install-missing"));
            }
            Some(value)
        } else { None };
        let full: Vec<_> = live.iter().rev().copied().chain(installed).map(|value|
            InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value }))
            .chain(homes.iter().cloned()).collect();
        let mut normal_indices: Vec<_> = if defer_owned_argument { Vec::new() } else { (0..live.len()).collect() };
        if let Some(old) = displaced {
            let index = full.iter().position(|operation| operation == old)
                .ok_or_else(|| freeze("rebind-expression/old-home-missing"))?;
            if residence.is_none() || full.iter().filter(|operation| *operation == old).count() != 1
                || normal_indices.contains(&index) { return Err(freeze("rebind-expression/old-home-duplicate")); }
            normal_indices.push(index);
        }
        let stop = match finishing {
            Some(projection) => projection.destination(continuation.0)
                .ok_or_else(|| freeze("print-expression/continuation-removed"))?,
            None => continuation.0,
        };
        for (step, index) in normal_indices.iter().enumerate() {
            normal = paths::skip_until(function, normal, Some(stop))?;
            let (next, fault) = paths::release(function, normal, &full[*index], Some(fault_frame))?;
            covered.insert(normal);
            let residual = super::super::fault_indices_after_normal_step_v2(full.len(), &normal_indices, step)?
                .into_iter().map(|index| full[index].clone()).collect::<Vec<_>>();
            covered.extend(paths::pending_operations(function, fault, &residual, Some(fault_frame))?);
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
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        ordered: &[Binding], homes: &[InvokeOperation], fault_frame: ValueId,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
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
        let calls = self.frame.calls().iter().zip(self.calls()).map(|(source, call)| {
            let effects = call.ordered_call_bindings_in_context_v1(self.frame.source().owner(), source.arguments(), context)?
                .into_iter().filter(|(_, instruction)| matches!(instruction, MirInstruction::Invoke { .. }))
                .map(|binding| match finishing {
                    Some(projection) => projection.binding(binding.0, &binding.1)?
                        .ok_or_else(|| freeze("argument-expression/enclosing-invoke-removed")),
                    None => Ok(binding),
                }).collect::<Result<Vec<_>, String>>()?;
            Ok((effects, call))
        }).collect::<Result<Vec<_>, String>>()?;
        let mut validated_calls = BTreeSet::new();
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
            if let Some((effects, call)) = calls.iter().find(|(effects, _)| effects.iter().any(|binding| binding == &(*id, instruction.clone()))) {
                if validated_calls.insert(call.invoke_binding().0) {
                    let prefix = ExpressionCallTransientPrefixV1 { origin: TransientPrefixOriginV1::Emitted { call, live: &live, prior_homes: self.frame.prior_homes() }, parent: inherited };
                    covered.extend(call.validate_expression_arguments_with_prefix_v1(context, function,
                        finishing, self.original, None, Some(&prefix))?);
                }
                if !effects.iter().all(|binding| covered.contains(&binding.0)) {
                    return Err(freeze("argument-expression/call-effect-uncovered"));
                }
                continue;
            }
            covered.insert(*id);
            covered.extend(paths::pending_operations(function, *fault_landing, &end_sequence(&live), Some(fault_frame))?);
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
                if live.contains(&value) || homes.iter().any(|operation| matches!(operation,
                    InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value: prior }) if *prior == value)) {
                    return Err(freeze("argument-expression/prefix-duplicate-acquisition"));
                }
                live.push(value);
            }
        }
        Ok((live, covered))
    }
}

impl ExpressionPhysicalProjectionRefV1<'_> {
    pub(in crate::mir) fn validate_scalar_continuation_v1(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>, entry: &Binding, terminal: &Binding, continuation: &Binding,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_local_continuation_v1(context, function, finishing, entry, terminal, continuation, None, inherited)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_rebind_continuation_v1(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>, entry: &Binding, terminal: &Binding, continuation: &Binding,
        residence: Option<&Binding>, displaced: Option<&InvokeOperation>, purpose_acquisition: Option<&[Binding; 2]>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_local_delimiters_v1(context, function, finishing, entry, terminal, continuation)?;
        self.validate_paths_with_policy(context, function, finishing, terminal, continuation, residence, None, false, displaced, purpose_acquisition)
    }
    pub(in crate::mir) fn validate_local_continuation_v1(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>, entry: &Binding, terminal: &Binding, continuation: &Binding,
        residence: Option<&Binding>, inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_local_delimiters_v1(context, function, finishing, entry, terminal, continuation)?;
        self.validate_paths_to_terminal(context, function, finishing, terminal, continuation, residence, inherited)
    }
    pub(in super::super) fn validate_deferred_argument_continuation_v1(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        entry: &Binding, terminal: &Binding, continuation: &Binding,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>, purpose_acquisition: Option<&[Binding; 2]>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_local_delimiters_v1(context, function, finishing, entry, terminal, continuation)?;
        self.validate_paths_with_policy(context, function, finishing, terminal, continuation, None, inherited, true, None, purpose_acquisition)
    }
    fn validate_local_delimiters_v1(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        entry: &Binding, terminal: &Binding, continuation: &Binding,
    ) -> Result<(), String> {
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
        Ok(())
    }

    #[cfg(test)]
    pub(in crate::mir) fn assert_original_deferred_argument_paths_for_test(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
        entry: &Binding, terminal: &Binding, continuation: &Binding,
    ) {
        self.validate_deferred_argument_continuation_v1(context, function, None,
            entry, terminal, continuation, None, None).unwrap();
        assert!(self.validate_scalar_continuation_v1(context, function, None,
            entry, terminal, continuation, None).is_err(), "scalar completion cannot skip its own Ends");
        let mut changed = function.clone();
        changed.blocks.get_mut(&terminal.0).unwrap().terminator = Some(MirInstruction::Jump {
            target: BasicBlockId(999_995), edge_args: None,
        });
        assert!(self.validate_deferred_argument_continuation_v1(context, &changed, None,
            entry, terminal, continuation, None, None).is_err(), "deferred delimiter must remain original");
    }

}
