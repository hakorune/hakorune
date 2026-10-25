//! Normal completion of the same physical Call: a value projection or Unit.
//! A Unit continuation never creates a ValueId or an InvokeNormalResult.
use super::*;
use super::super::expression_projection::{ExpressionCallArgumentSourceV1, ExpressionCallTransientPrefixV1};
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;

/// Physical delimiters and Normal End observations in the SAME Call packet.
/// Ownership is always rederived from original argument frames, never stored here.
#[derive(Debug)]
pub(super) struct ArgumentCallCleanupV1 {
    pub(super) ingress: Binding,
    pub(super) continuation: Binding,
    pub(super) normal: Vec<Binding>,
    pub(super) root_fault_suffix: Option<BasicBlockId>,
}

#[derive(Debug)]
pub(super) enum NormalCallProjectionV1 {
    Value(Binding),
    Unit { normal_landing: BasicBlockId },
}
impl NormalCallProjectionV1 {
    pub(super) fn value(&self) -> Result<&Binding, String> {
        match self {
            Self::Value(binding) => Ok(binding),
            Self::Unit { .. } => Err(freeze("unit-call-has-no-value")),
        }
    }
    #[cfg(test)]
    pub(super) fn value_mut(&mut self) -> Result<&mut Binding, String> {
        match self {
            Self::Value(binding) => Ok(binding),
            Self::Unit { .. } => Err(freeze("unit-call-has-no-value")),
        }
    }
}

impl EmittedLexicalCallProjectionV1 {
    /// The original disposition must already prove Unit. This retains only
    /// its actual Invoke/Normal edge; it issues no semantic result evidence.
    pub(in crate::mir) fn from_unit_source(
        row: CallPacketSourceV1,
        prepared: PreparedLexicalCallProjectionV1,
        invoke: Binding,
        normal_landing: BasicBlockId,
    ) -> Result<Self, String> {
        if row.loan().result() != Some(InvokeCallResultKind::Unit)
            || !matches!(&invoke.1, MirInstruction::Invoke {
                operation: InvokeOperation::Call { result: InvokeCallResultKind::Unit, .. },
                normal_landing: actual, fault_landing, ..
            } if *actual == normal_landing && normal_landing != invoke.0
                && *fault_landing != normal_landing && *fault_landing != invoke.0)
        {
            return Err(freeze("unit-call/normal-source-drift"));
        }
        Ok(Self {
            row,
            prepared,
            invoke,
            projection: NormalCallProjectionV1::Unit { normal_landing },
            argument_cleanup: None,
        })
    }

    pub(in crate::mir) fn with_argument_cleanup_v1(
        mut self,
        ingress: Binding,
        continuation: Binding,
        normal: Vec<Binding>,
    ) -> Result<Self, String> {
        if self.argument_cleanup.is_some()
            || normal.is_empty()
            || !matches!(
                ingress.1,
                MirInstruction::Jump {
                    edge_args: None,
                    ..
                }
            )
            || !matches!(
                continuation.1,
                MirInstruction::Jump {
                    edge_args: None,
                    ..
                }
            )
        {
            return Err(freeze("call-argument/cleanup-delimiters"));
        }
        self.argument_cleanup = Some(ArgumentCallCleanupV1 {
            ingress,
            continuation,
            normal,
            root_fault_suffix: None,
        });
        Ok(self)
    }

    /// The root writer lends the same prefix delimiters and older Fault entry.
    pub(in crate::mir) fn with_root_argument_cleanup_v1(
        self, ingress: Binding, continuation: Binding, normal: Vec<Binding>,
        fault_suffix: BasicBlockId,
    ) -> Result<Self, String> {
        let mut packet = self.with_argument_cleanup_v1(ingress, continuation, normal)?;
        packet.argument_cleanup.as_mut().unwrap().root_fault_suffix = Some(fault_suffix);
        Ok(packet)
    }

    /// Coordinates only. The argument lifetime validator independently checks
    /// both paths against the original prepared arguments before publication.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn root_cleanup_suffixes_v1(
        &self, finishing: Option<&FinishedBindings>,
    ) -> Result<Option<(BasicBlockId, BasicBlockId)>, String> {
        let Some(cleanup) = &self.argument_cleanup else { return Ok(None); };
        let Some(fault) = cleanup.root_fault_suffix else { return Ok(None); };
        let MirInstruction::Jump { target: normal, edge_args: None } = cleanup.continuation.1
            else { return Err(freeze("call-argument/cleanup-delimiters")); };
        let map = |id| match finishing {
            Some(boundary) => boundary.destination(id).ok_or_else(|| freeze("call-argument/continuation-removed")),
            None => Ok(id),
        };
        Ok(Some((map(normal)?, map(fault)?)))
    }

    pub(super) fn validate_normal_recorded(
        &self,
        bindings: &[Binding],
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        if let Some(cleanup) = &self.argument_cleanup {
            for original in std::iter::once(&cleanup.ingress)
                .chain(&cleanup.normal)
                .chain(std::iter::once(&cleanup.continuation))
            {
                match finishing {
                    Some(boundary)
                        if boundary.binding(original.0, &original.1)?.is_none()
                            && matches!(
                                original.1,
                                MirInstruction::Jump {
                                    edge_args: None,
                                    ..
                                }
                            ) =>
                    {
                        ()
                    }
                    _ => require_recorded(bindings, original, finishing)?,
                }
            }
        }
        match &self.projection {
            NormalCallProjectionV1::Value(binding) => {
                require_recorded(bindings, binding, finishing)
            }
            NormalCallProjectionV1::Unit { .. } => {
                let mapped = finishing
                    .map(|finished| finished.binding(self.invoke.0, &self.invoke.1))
                    .transpose()?
                    .flatten();
                let origin = mapped.as_ref().unwrap_or(&self.invoke).0;
                if bindings.iter().any(|(_, instruction)| matches!(instruction,
                    MirInstruction::InvokeNormalResult { invoke_block, .. } if *invoke_block == origin)) {
                    return Err(freeze("unit-call/unexpected-value-projection"));
                }
                Ok(())
            }
        }
    }

    /// Physical call order: receiver before argument calls, then Invoke/Normal.
    /// Reuse original packets; the postorder node visitor remains unchanged.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn ordered_call_bindings_in_context_v1(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        self.call_in_context(owner, source, context)?;
        let mut ordered = Vec::new();
        if let Some(read) = self.stored_receiver_read_v1() {
            ordered.push(read.clone());
        }
        for (argument, original) in self.prepared.arguments.iter().zip(source) {
            if let LexicalCallArgumentProjectionV1::CallResult(inner) = argument {
                let LocalCallArgumentV1::CallResult(original) = original else {
                    return Err(freeze("lexical-i64/nested-source-missing"));
                };
                ordered.extend(inner.ordered_call_bindings_in_context_v1(
                    owner,
                    original.arguments(),
                    context,
                )?);
            }
        }
        ordered.push(self.invoke.clone());
        if let NormalCallProjectionV1::Value(result) = &self.projection {
            ordered.push(result.clone());
        }
        if let Some(cleanup) = &self.argument_cleanup {
            ordered.extend_from_slice(&cleanup.normal);
        }
        Ok(ordered)
    }

    pub(in crate::mir) fn corroborate_loop_continuation_v2(
        &self,
        source: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        entry: BasicBlockId,
        end: BasicBlockId,
        context: CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        check_coordinates: bool,
    ) -> Result<(), String> {
        use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1 as Class;
        let expected = match source.result() {
            Class::I64 => InvokeCallResultKind::I64,
            Class::Bool => InvokeCallResultKind::Bool,
            Class::Unit => InvokeCallResultKind::Unit,
            Class::Handle => InvokeCallResultKind::Handle,
            Class::Nullable => InvokeCallResultKind::NullableHandle,
            _ => return Err(freeze("source-loop/call-source")),
        };
        if self.call_site() != source.site()
            || self.row.loan().result() != Some(expected)
            || context
                .ledger()
                .local_call_for_owner(source.owner(), source.site().site())
                != Some(source)
        {
            return Err(freeze("source-loop/call-source"));
        }
        let arguments = if matches!(source.result(), Class::Handle | Class::Nullable) {
            context.ledger().original_local_packet_arguments_v1(self)?
        } else { source.arguments() };
        self.normal_result_using(source.owner(), arguments, Some(context))?;
        let MirInstruction::Invoke { normal_landing, .. } = &self.invoke.1 else {
            return Err(freeze("source-loop/call-invoke"));
        };
        let expected = match &self.argument_cleanup {
            Some(cleanup) => match cleanup.continuation.1 {
                MirInstruction::Jump {
                    target,
                    edge_args: None,
                } => target,
                _ => return Err(freeze("source-loop/call-continuation")),
            },
            None => *normal_landing,
        };
        if expected != end {
            return Err(freeze("source-loop/call-continuation"));
        }
        if check_coordinates {
            let dominators = crate::mir::verification::utils::compute_dominators(function);
            let mut writes = self.ordered_call_bindings_in_context_v1(
                source.owner(),
                arguments,
                context,
            )?;
            for argument in &self.prepared.arguments {
                writes.extend_from_slice(argument.expression_bindings_v1());
            }
            if writes
                .iter()
                .filter(|(_, row)| !matches!(row, MirInstruction::FaultFrameEnter { .. }))
                .any(|(block, _)| !dominators.dominates(entry, *block))
            {
                return Err(freeze("source-loop/call-entry"));
            }
        }
        Ok(())
    }

    pub(in crate::mir) fn value_in_context_v1(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        self.value_using(owner, source, Some(context))
    }

    pub(super) fn value_using(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
        context: Option<CallValidationContextV1<'_>>,
    ) -> Result<ValueId, String> {
        self.normal_result_using(owner, source, context)?
            .ok_or_else(|| freeze("unit-call-has-no-value"))
    }

    /// Validate the SAME target, receiver and ordered arguments for both
    /// result forms. Nested arguments still require an actual value.
    pub(super) fn normal_result_using(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
        context: Option<CallValidationContextV1<'_>>,
    ) -> Result<Option<ValueId>, String> {
        let ledger = context.map(CallValidationContextV1::ledger);
        if let CallPacketSourceLoanV1::Instance(row) = self.row.loan() {
            if row
                .source_target()
                .borrowed_nullable_local_source()
                .is_some()
            {
                ledger
                    .ok_or_else(|| freeze("borrowed-nullable-call/ledger-missing"))?
                    .corroborate_borrowed_nullable_call_v1(row)?;
            }
            if row.source_target().mixed_local_source().is_some() {
                let context = context.ok_or_else(|| freeze("mixed-call/ledger-missing"))?;
                context
                    .ledger()
                    .corroborate_mixed_call_using_v1(row, context)?;
            }
            if row.result() == Some(InvokeCallResultKind::OwnedString) {
                ledger
                    .ok_or_else(|| freeze("borrowed-call/source-missing"))?
                    .corroborate_local_instance_text_call_v1(row)?;
            }
            if row.result() == Some(InvokeCallResultKind::Unit) {
                ledger
                    .ok_or_else(|| freeze("unit-call/ledger-missing"))?
                    .corroborate_unit_discard_v1(row)?;
            }
        }
        let expected =
            self.prepared
                .materialize_in_context(owner, self.row.loan(), source, context)?;
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::Call {
                    call,
                    result: emitted_result,
                },
            normal_landing,
            ..
        } = &self.invoke.1
        else {
            return Err(freeze("lexical-i64/nested-invoke-shape"));
        };
        if self.row.loan().result() != Some(*emitted_result) || *call != expected {
            return Err(freeze("lexical-i64/nested-producer-drift"));
        }
        match &self.projection {
            NormalCallProjectionV1::Value(projection) => {
                let MirInstruction::InvokeNormalResult { invoke_block, dst } = &projection.1 else {
                    return Err(freeze("lexical-i64/nested-projection-shape"));
                };
                if *emitted_result == InvokeCallResultKind::Unit
                    || *invoke_block != self.invoke.0
                    || *normal_landing != projection.0
                {
                    return Err(freeze("lexical-i64/nested-producer-drift"));
                }
                Ok(Some(*dst))
            }
            NormalCallProjectionV1::Unit {
                normal_landing: original,
            } => {
                if *emitted_result != InvokeCallResultKind::Unit || normal_landing != original {
                    return Err(freeze("unit-call/normal-source-drift"));
                }
                Ok(None)
            }
        }
    }
}

impl EmittedLexicalCallProjectionV1 {
    /// Verify the complete Call boundary, including an argument-free nested
    /// Call which still inherits earlier argument acquisitions.
    pub(super) fn validate_argument_lifetime_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
        original: Option<ExpressionCallArgumentSourceV1<'_>>,
        normal_fault_cancellation: &[InvokeOperation],
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        use super::super::super::root_cleanup_graph::ordered_paths as paths;
        // Every wrapper and nested Call uses the same original result projection.
        // An outer expression may corroborate this cancellation, never replace it.
        let result_cancellation = if self.row.loan().result()
            == Some(crate::mir::instruction::InvokeCallResultKind::OwnedString)
        {
            let (block, MirInstruction::InvokeNormalResult { dst, invoke_block }) =
                self.projection.value()?
            else {
                return Err(freeze("call-argument/invoke-shape"));
            };
            if *invoke_block != self.invoke.0
                || !matches!(&self.invoke.1,
                MirInstruction::Invoke { normal_landing, operation: InvokeOperation::Call { result: crate::mir::instruction::InvokeCallResultKind::OwnedString, .. }, .. }
                    if normal_landing == block)
            {
                return Err(freeze("call-argument/invoke-shape"));
            }
            vec![InvokeOperation::String(
                crate::mir::instruction::StringInvokeOperation::End { value: *dst },
            )]
        } else if let CallPacketSourceLoanV1::Instance(row) = self.row.loan() {
            if matches!(row.result(), Some(InvokeCallResultKind::Handle | InvokeCallResultKind::NullableHandle))
                && context.ledger().uses_object_packet_protocol_v1(row)? {
                let packet = context.ledger().borrowed_object_call_packet_v1(row)?;
                let (block, MirInstruction::InvokeNormalResult { dst, invoke_block }) = self.projection.value()? else {
                    return Err(freeze("call-argument/invoke-shape"));
                };
                if *invoke_block != self.invoke.0 || !matches!(&self.invoke.1,
                    MirInstruction::Invoke { normal_landing, operation: InvokeOperation::Call { result, .. }, .. }
                        if normal_landing == block && *result == packet.result()) {
                    return Err(freeze("call-argument/invoke-shape"));
                }
                packet.cancellation_operations_v1(*dst)
            } else { Vec::new() }
        } else { Vec::new() };
        let normal_fault_cancellation = if result_cancellation.is_empty() {
            normal_fault_cancellation
        } else {
            if !normal_fault_cancellation.is_empty()
                && normal_fault_cancellation != result_cancellation
            {
                return Err(freeze("call-argument/duplicate-acquisition"));
            }
            &result_cancellation
        };
        let ends = self.prepared.deferred_argument_ends_v1(context)?;
        if ends.is_empty() && inherited.is_none() {
            if self.argument_cleanup.is_some() {
                return Err(freeze("call-argument/unexpected-cleanup"));
            }
            return Ok(Default::default());
        }
        if matches!(context, CallValidationContextV1::Finalized(_)) {
            finishing
                .ok_or_else(|| freeze("call-argument/finished-boundary-missing"))?
                .validate_finished_sequences_v1(function)?;
        }
        let source = original.ok_or_else(|| freeze("call-argument/original-source"))?;
        let mut suffix = inherited
            .map(|prefix| prefix.unwind_for_call(self.call_site(), source.prior_homes()))
            .transpose()?
            .unwrap_or_default();
        suffix.extend(
            context
                .ledger()
                .prior_home_unwind_for(source.prior_homes())?,
        );
        let operations: Vec<_> = ends.iter().chain(&suffix).cloned().collect();
        let mut seen = std::collections::BTreeSet::new();
        for operation in &operations {
            if let InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End {
                value,
            }) = operation
            {
                if !seen.insert(*value) {
                    return Err(freeze("call-argument/duplicate-acquisition"));
                }
            }
        }
        let mapped = |binding: &Binding| match finishing {
            Some(boundary) => boundary
                .binding(binding.0, &binding.1)?
                .ok_or_else(|| freeze("call-argument/required-binding")),
            None => Ok(binding.clone()),
        };
        let invoke = mapped(&self.invoke)?;
        if !super::super::super::physical_boundary::check_binding(
            function, None, invoke.0, &invoke.1,
        )? {
            return Err(freeze("call-argument/invoke-missing"));
        }
        let MirInstruction::Invoke {
            fault_frame,
            normal_landing,
            fault_landing,
            ..
        } = invoke.1
        else {
            return Err(freeze("call-argument/invoke-shape"));
        };
        let mut covered =
            paths::pending_operations(function, fault_landing, &operations, Some(fault_frame))?;
        covered.insert(invoke.0);
        if let Some((_, suffix)) = self.root_cleanup_suffixes_v1(finishing)? {
            let mut next = fault_landing;
            for operation in &ends {
                next = paths::skip_until(function, next, None)?;
                let (normal, fault) = paths::release(function, next, operation, Some(fault_frame))?;
                if paths::skip_until(function, normal, None)? != paths::skip_until(function, fault, None)? {
                    return Err(freeze("call-argument/cleanup-delimiters"));
                }
                next = normal;
            }
            if paths::skip_until(function, next, None)? != paths::skip_until(function, suffix, None)? {
                return Err(freeze("call-argument/cleanup-delimiters"));
            }
        }
        if ends.is_empty() {
            if self.argument_cleanup.is_some() {
                return Err(freeze("call-argument/unexpected-cleanup"));
            }
            return Ok(covered);
        }
        let cleanup = self
            .argument_cleanup
            .as_ref()
            .ok_or_else(|| freeze("call-argument/cleanup-missing"))?;
        if cleanup.normal.len() != ends.len()
            || !matches!(&self.invoke.1, MirInstruction::Invoke { normal_landing, .. } if *normal_landing == cleanup.ingress.0)
        {
            return Err(freeze("call-argument/cleanup-delimiters"));
        }
        for delimiter in [&cleanup.ingress, &cleanup.continuation] {
            match finishing {
                Some(boundary) => {
                    boundary.structural_jump_binding(
                        function,
                        delimiter,
                        matches!(context, CallValidationContextV1::Finalized(_)),
                    )?;
                }
                None if function
                    .blocks
                    .get(&delimiter.0)
                    .and_then(|block| block.terminator.as_ref())
                    == Some(&delimiter.1) =>
                {
                    ()
                }
                None => return Err(freeze("call-argument/cleanup-delimiters")),
            }
        }
        let stop = match finishing {
            Some(boundary) => boundary
                .destination(cleanup.continuation.0)
                .ok_or_else(|| freeze("call-argument/continuation-removed"))?,
            None => cleanup.continuation.0,
        };
        let mut normal = normal_landing;
        for (index, operation) in ends.iter().enumerate() {
            normal = paths::skip_until(function, normal, Some(stop))?;
            let observed = mapped(&cleanup.normal[index])?;
            if observed.0 != normal
                || !super::super::super::physical_boundary::check_binding(
                    function,
                    None,
                    observed.0,
                    &observed.1,
                )?
            {
                return Err(freeze("call-argument/normal-end-origin"));
            }
            let (next, fault) = paths::release(function, normal, operation, Some(fault_frame))?;
            covered.insert(normal);
            let after_fault: Vec<_> = ends[index + 1..]
                .iter()
                .chain(normal_fault_cancellation)
                .chain(&suffix)
                .cloned()
                .collect();
            covered.extend(paths::pending_operations(
                function,
                fault,
                &after_fault,
                Some(fault_frame),
            )?);
            normal = next;
        }
        if paths::skip_until(function, normal, Some(stop))? != stop {
            return Err(freeze("call-argument/normal-continuation"));
        }
        Ok(covered)
    }
}

impl EmittedLexicalCallProjectionV1 {
    pub(in crate::mir) fn source_loop_argument_bindings_in_context_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let mut rows = Vec::new();
        for argument in &self.prepared.arguments {
            for binding in argument.source_loop_bindings_in_context_v1(context)? {
                if !rows.contains(&binding) {
                    rows.push(binding);
                }
            }
        }
        Ok(rows)
    }

}

impl EmittedLexicalCallProjectionV1 {
    pub(in crate::mir) fn copy_dependencies(
        &self,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<Vec<(crate::mir::resolved_semantics::OwnedExprSiteV1, Binding)>, String> {
        self.copy_dependencies_in_context(CallValidationContextV1::Lowering(ledger))
    }

    pub(in crate::mir) fn copy_dependencies_in_context(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<(crate::mir::resolved_semantics::OwnedExprSiteV1, Binding)>, String> {
        let mut dependencies = Vec::new();
        if let (CallPacketSourceLoanV1::Instance(row), LexicalReceiverProjectionV1::Lexical(read)) =
            (self.row.loan(), &self.prepared.receiver)
        {
            if row.source_target().borrowed_receiver_source_v1().is_some() {
                context.require_borrowed_map_receiver_v1(row, read)?;
                dependencies.extend(
                    read.original_copy_dependencies_v1()?
                        .iter()
                        .map(|copy| (self.call_site().clone(), copy.clone())),
                );
            }
        }
        if let LexicalReceiverProjectionV1::StoredChild { parent, .. } = &self.prepared.receiver {
            dependencies.extend(
                parent
                    .original_copy_dependencies_v1()?
                    .iter()
                    .map(|copy| (self.row.loan().call_site().clone(), copy.clone())),
            );
        }
        for argument in &self.prepared.arguments {
            if let LexicalCallArgumentProjectionV1::Expression(_) = argument {
                dependencies.extend(
                    argument
                        .expression_copy_dependencies_v1(context)?
                        .into_iter()
                        .map(|copy| (self.call_site().clone(), copy)),
                );
                for call in argument.expression_calls_v1() {
                    dependencies.extend(call.copy_dependencies_in_context(context)?);
                }
            } else if let LexicalCallArgumentProjectionV1::CallResult(inner) = argument {
                dependencies.extend(inner.copy_dependencies_in_context(context)?);
            } else {
                for copy in borrowed_projection::copies_in_context(
                    argument,
                    self.row.loan().call_site().owner(),
                    self.row.loan(),
                    context,
                )? {
                    dependencies.push((self.row.loan().call_site().clone(), copy.clone()));
                }
            }
        }
        Ok(dependencies)
    }

}
