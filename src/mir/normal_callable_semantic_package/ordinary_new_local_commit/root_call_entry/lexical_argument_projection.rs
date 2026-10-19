//! Tie an observed argument value to the original lexical argument meaning.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use super::super::expression_projection::{ExpressionCallTransientPrefixV1, ExpressionCallArgumentSourceV1};

impl LexicalCallArgumentProjectionV1 {
    pub(super) fn borrowed_expression_value_v1(
        &self, actual: &crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1,
        call: &OwnedExprSiteV1, ordinal: u32, site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        let crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1::Expression(original) = actual
            else { return Err(freeze("argument-expression/borrowed-source-required")) };
        let Self::Expression(packet) = self else { return Err(freeze("argument-expression/borrowed-packet-required")); };
        let source = packet.frame.source();
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call: outer, ordinal: position, .. } = source.root()
            else { return Err(freeze("argument-expression/source-purpose")) };
        if !std::rc::Rc::ptr_eq(original.frame(), &packet.frame)
            || packet.completion != if packet.frame.is_i64() { ArgumentCompletionV1::ScalarCompleted } else { ArgumentCompletionV1::OwnedTextDeferred }
            || source.owner() != call.owner() || outer.site() != call.site()
            || *position != ordinal || source.nodes().first().map(|node| node.site()) != Some(site) {
            return Err(freeze("argument-expression/borrowed-original-identity"));
        }
        packet.validate_source(context)
    }

    pub(in crate::mir) fn value_for_source_v1(
        &self,
        source: &LocalCallArgumentV1,
        owner: FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        row: Option<CallPacketSourceLoanV1<'_>>,
        context: Option<CallValidationContextV1<'_>>,
    ) -> Result<ValueId, String> {
        Ok(match (self, source) {
            (
                LexicalCallArgumentProjectionV1::Integer((
                    _,
                    MirInstruction::Const {
                        dst,
                        value: crate::mir::ConstValue::Integer(actual),
                    },
                )),
                LocalCallArgumentV1::Integer(expected),
            ) if actual == expected => *dst,
            (
                LexicalCallArgumentProjectionV1::Scalar(read),
                LocalCallArgumentV1::Scalar(binding),
            ) => read
                .value_for(owner, site.node(), *binding)
                .map_err(|error| format!("[freeze:contract][lexical-i64/argument/{error:?}]"))?,
            (
                LexicalCallArgumentProjectionV1::CallResult(emitted),
                LocalCallArgumentV1::CallResult(inner),
            ) if inner.site().owner() == owner
                && inner.site().site() == site
                && emitted.call_site() == inner.site() =>
            {
                emitted.value_using(owner, inner.arguments(), context)?
            }
            (
                projection,
                LocalCallArgumentV1::BorrowedActual {
                    ordinal,
                    site: original,
                },
            ) if original == site => borrowed_projection::value(
                projection,
                owner,
                row.ok_or_else(|| freeze("lexical-i64/borrowed-source-row-missing"))?,
                *ordinal,
                original,
                context.ok_or_else(|| freeze("lexical-i64/borrowed-lender-missing"))?,
            )?,
            (
                Self::Expression(packet),
                LocalCallArgumentV1::Expression {
                    ordinal,
                    site: original,
                },
            ) if original == site => {
                if packet.completion != ArgumentCompletionV1::ScalarCompleted || packet.ordinal()? != *ordinal
                    || packet.frame.source().owner() != owner
                    || packet
                        .frame
                        .source()
                        .nodes()
                        .first()
                        .is_none_or(|root| root.site() != site)
                {
                    return Err(freeze("argument-expression/ordinal-owner"));
                }
                packet.validate_source(
                    context.ok_or_else(|| freeze("argument-expression/context-missing"))?,
                )?
            }
            _ => return Err(freeze("lexical-i64/argument-projection-drift")),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArgumentCompletionV1 { ScalarCompleted, OwnedTextDeferred }

/// Observations remain within the enclosing Call packet, never a second pool.
#[derive(Debug)]
pub(in crate::mir) struct EmittedArgumentExpressionProjectionV1 {
    frame: std::rc::Rc<
        crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
    >,
    values: Vec<Option<crate::mir::instruction::StringInvokeOperand>>,
    calls: Vec<std::rc::Rc<EmittedLexicalCallProjectionV1>>,
    path_reads: Vec<crate::mir::builder::ExactLexicalReadV1>,
    reference_reads: std::collections::BTreeMap<
        usize,
        super::super::expression_projection::ExpressionReferenceReadV1,
    >,
    entry: Binding,
    terminal: Binding,
    continuation: Binding,
    original: Vec<Binding>,
    completion: ArgumentCompletionV1,
}
impl EmittedArgumentExpressionProjectionV1 {
    fn ordinal(&self) -> Result<u32, String> {
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { ordinal, .. } = self.frame.source().root()
            else { return Err(freeze("argument-expression/source-purpose")) };
        Ok(*ordinal)
    }
    fn expression(
        &self,
    ) -> super::super::expression_projection::ExpressionPhysicalProjectionRefV1<'_> {
        super::super::expression_projection::ExpressionPhysicalProjectionRefV1::new(
            self.frame.evaluation(),
            &self.values,
            &self.calls,
            &self.path_reads,
            &self.reference_reads,
            &self.original,
        )
    }
    fn validate_source(&self, context: CallValidationContextV1<'_>) -> Result<ValueId, String> {
        let source = self.frame.source();
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal, .. } = source.root()
            else { return Err(freeze("argument-expression/source-purpose")) };
        let site = source
            .nodes()
            .first()
            .ok_or_else(|| freeze("argument-expression/root-missing"))?
            .site();
        let original = context.ledger().qualified_argument_expression_frame_v1(
            &crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                source.owner(),
                call.site().clone(),
            ),
            *ordinal,
            site,
        )?;
        if !std::rc::Rc::ptr_eq(&original, &self.frame)
            || !matches!(
                self.terminal.1,
                MirInstruction::Jump {
                    edge_args: None,
                    ..
                }
            )
            || !matches!(
                self.continuation.1,
                MirInstruction::Jump {
                    edge_args: None,
                    ..
                }
            )
        {
            return Err(freeze("argument-expression/source-or-result"));
        }
        let value = match (self.completion, self.values.first()) {
            (ArgumentCompletionV1::ScalarCompleted, Some(Some(crate::mir::instruction::StringInvokeOperand::NormalInteger(value))))
                if self.frame.is_i64() => *value,
            (ArgumentCompletionV1::OwnedTextDeferred, Some(Some(crate::mir::instruction::StringInvokeOperand::OwnedString(value))))
                if !self.frame.is_i64() && self.frame.evaluation().owned_result_nodes().contains(&0) => *value,
            _ => return Err(freeze("argument-expression/result")),
        };
        self.expression().validate_source(context)?;
        Ok(value)
    }
    fn validate(
        &self,
        context: CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_source(context)?;
        match self.completion {
            ArgumentCompletionV1::ScalarCompleted => self.expression().validate_scalar_continuation_v1(
                context, function, finishing, &self.entry, &self.terminal, &self.continuation, inherited),
            ArgumentCompletionV1::OwnedTextDeferred => self.expression().validate_deferred_argument_continuation_v1(
                context, function, finishing, &self.entry, &self.terminal, &self.continuation, inherited),
        }
    }
}
impl LexicalCallArgumentProjectionV1 {
    pub(in crate::mir) fn from_expression_v1(
        frame: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
        >,
        values: Vec<Option<crate::mir::instruction::StringInvokeOperand>>,
        calls: Vec<std::rc::Rc<EmittedLexicalCallProjectionV1>>,
        path_reads: Vec<crate::mir::builder::ExactLexicalReadV1>,
        reference_reads: std::collections::BTreeMap<
            usize,
            super::super::expression_projection::ExpressionReferenceReadV1,
        >,
        entry: Binding,
        terminal: Binding,
        continuation: Binding,
        original: Vec<Binding>,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
        function: &crate::mir::MirFunction,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<Self, String> {
        let completion = if frame.is_i64() { ArgumentCompletionV1::ScalarCompleted }
            else if frame.evaluation().owned_result_nodes().contains(&0) { ArgumentCompletionV1::OwnedTextDeferred }
            else { return Err(freeze("argument-expression/deferred-source-root")); };
        let packet = EmittedArgumentExpressionProjectionV1 {
            completion,
            frame,
            values,
            calls,
            path_reads,
            reference_reads,
            entry,
            terminal,
            continuation,
            original,
        };
        let context = CallValidationContextV1::Lowering(ledger);
        let covered = packet.validate(context, function, None, inherited)?;
        let mut ordered = packet.expression().ordered_bindings_in_context(context)?;
        ordered.push(packet.terminal.clone());
        packet.expression().validate_original_scope(
            ledger,
            function,
            &covered,
            &packet.entry,
            &packet.terminal,
            &packet.continuation,
            &ordered,
        )?;
        Ok(Self::Expression(Box::new(packet)))
    }
    /// Source-bound live-out observations, never a second acquisition inventory.
    pub(in crate::mir) fn deferred_text_values_v1(
        &self, context: CallValidationContextV1<'_>,
    ) -> Result<Vec<ValueId>, String> {
        let Self::Expression(packet) = self else { return Ok(Vec::new()) };
        let root = packet.validate_source(context)?;
        if packet.completion == ArgumentCompletionV1::ScalarCompleted { return Ok(Vec::new()); }
        let mut values = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for &node in packet.frame.evaluation().ordered_nodes() {
            if !packet.frame.evaluation().owned_result_nodes().contains(&node) { continue; }
            let Some(Some(crate::mir::instruction::StringInvokeOperand::OwnedString(value))) = packet.values.get(node) else {
                return Err(freeze("argument-expression/deferred-value"));
            };
            if !seen.insert(*value) { return Err(freeze("argument-expression/deferred-duplicate")); }
            values.push(*value);
        }
        if values.last() != Some(&root) {
            return Err(freeze("argument-expression/deferred-root-order"));
        }
        Ok(values)
    }
    /// Ownership-prefix corroboration only. Non-expression arguments add no
    /// retained stack; the enclosing materializer still checks every original
    /// actual tuple, including its formal, site and ordinal.
    pub(in crate::mir) fn require_argument_frame_position_v1(
        &self, call: &OwnedExprSiteV1, ordinal: usize,
        homes: &[LiveHomeV1], context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        let Self::Expression(packet) = self else { return Ok(()) };
        let source = packet.frame.source();
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call: original, ordinal: position, .. } = source.root()
            else { return Err(freeze("argument-expression/source-purpose")) };
        if source.owner() != call.owner() || original.site() != call.site() || *position as usize != ordinal
            || packet.frame.prior_homes() != homes {
            return Err(freeze("argument-expression/prefix-position"));
        }
        packet.validate_source(context).map(drop)
    }
    pub(super) fn validate_expression_recorded_v1(
        &self, bindings: &[Binding], finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        if let Self::Expression(packet) = self { packet.expression().validate_recorded(bindings, finishing)?; }
        Ok(())
    }
    pub(in crate::mir) fn expression_bindings_v1(&self) -> &[Binding] {
        match self {
            Self::Expression(packet) => &packet.original,
            _ => &[],
        }
    }
    pub(in crate::mir) fn validate_expression_physical_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        bindings: &[Binding],
        coordinates: Option<&super::super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_expression_physical_with_prefix_v1(context, function, finishing, bindings, coordinates, None)
    }
    fn validate_expression_physical_with_prefix_v1(
        &self, context: CallValidationContextV1<'_>, function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>, bindings: &[Binding], coordinates: Option<&FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        if let Self::Expression(packet) = self {
            let covered = packet.validate(context, function, finishing, inherited)?;
            packet.expression().validate_recorded(bindings, coordinates)?;
            return Ok(covered);
        }
        Ok(Default::default())
    }

    pub(super) fn expression_copy_dependencies_v1(&self, context: CallValidationContextV1<'_>) -> Result<Vec<Binding>, String> {
        let mut copies = Vec::new();
        if let Self::Expression(packet) = self {
            packet.validate_source(context)?;
            for read in &packet.path_reads { copies.extend_from_slice(read.original_copy_dependencies_v1()?); }
            for read in packet.reference_reads.values() { copies.extend_from_slice(read.original_copy_dependencies_v1()?); }
            for node in packet.frame.source().nodes() {
                copies.extend(context.ledger().checked_field_copy_dependencies_v1(
                    &OwnedExprSiteV1::new(packet.frame.source().owner(), node.site().clone()), context)?);
            }
        }
        Ok(copies)
    }

    /// Child packets remain borrowed from their original evaluation, never
    /// reconstructed from the flattened MIR or promoted to another pool.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn expression_calls_v1(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        match self { Self::Expression(packet) => Some(packet.as_ref()), _ => None }
            .into_iter().flat_map(|packet| packet.calls.iter().map(std::rc::Rc::as_ref))
    }

    pub(super) fn visit_expression_calls_v1(
        &self, owner: FunctionOwnerIdV1, context: CallValidationContextV1<'_>,
        visit: &mut impl FnMut(&EmittedLexicalCallProjectionV1, &[LocalCallArgumentV1]) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Self::Expression(packet) = self {
            packet.validate_source(context)?;
            for (source, call) in packet.frame.evaluation().calls().iter().zip(&packet.calls) {
                call.visit_original_nodes_in_context_v1(owner, source.arguments(), context, visit)?;
            }
        }
        Ok(())
    }
}

impl EmittedLexicalCallProjectionV1 {
    /// The enclosing Call is the common consumer of its source-bound argument
    /// evaluations. Completed scalar children stay outside its live-temp stack.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_expression_arguments_v1(
        &self, context: CallValidationContextV1<'_>, function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>, bindings: &[Binding], coordinates: Option<&FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_expression_arguments_with_prefix_v1(context, function, finishing, bindings, coordinates, None)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_expression_arguments_with_prefix_v1(
        &self, context: CallValidationContextV1<'_>, function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>, bindings: &[Binding], coordinates: Option<&FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        let original = context.ledger().local_call_for_owner(self.call_site().owner(), self.call_site().site())
            .map(ExpressionCallArgumentSourceV1::Local);
        self.validate_expression_arguments_at_source_v1(context, function, finishing, bindings, coordinates, inherited, original)
    }
    fn validate_expression_arguments_at_source_v1(
        &self, context: CallValidationContextV1<'_>, function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>, bindings: &[Binding], coordinates: Option<&FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
        original: Option<ExpressionCallArgumentSourceV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        if let Some(prefix) = inherited { prefix.require_call(self)?; }
        if original.is_some_and(|source| source.site() != self.call_site()
            || source.arguments().len() != self.prepared.arguments.len()) {
            return Err(freeze("argument-expression/prefix-original-call"));
        }
        if finishing.is_none() { self.validate_original_argument_order_v1(function)?; }
        let mut covered = std::collections::BTreeSet::new();
        for (ordinal, argument) in self.prepared.arguments.iter().enumerate() {
            if let LexicalCallArgumentProjectionV1::Expression(packet) = argument {
                let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal: position, .. } = packet.frame.source().root()
                    else { return Err(freeze("argument-expression/source-purpose")) };
                if packet.frame.source().owner() != self.call_site().owner()
                    || call.site() != self.call_site().site() || *position as usize != ordinal
                    || self.row.loan().argument_sites().get(ordinal) != packet.frame.source().nodes().first().map(|node| node.site()) {
                    return Err(freeze("argument-expression/enclosing-call"));
                }
                let prefix = ExpressionCallTransientPrefixV1::for_prepared_call_argument(
                    original.ok_or_else(|| freeze("argument-expression/prefix-original-call"))?,
                    ordinal, &self.prepared.arguments[..ordinal], context, inherited)?;
                prefix.for_argument(packet.frame.evaluation())?;
                covered.extend(argument.validate_expression_physical_with_prefix_v1(
                    context, function, finishing, bindings, coordinates, Some(&prefix))?);
                continue;
            }
            covered.extend(argument.validate_expression_physical_with_prefix_v1(context, function, finishing, bindings, coordinates, inherited)?);
            if let LexicalCallArgumentProjectionV1::CallResult(inner) = argument {
                let source = original.and_then(|source| match source.arguments().get(ordinal) {
                    Some(LocalCallArgumentV1::CallResult(inner)) => Some(ExpressionCallArgumentSourceV1::Nested(inner)), _ => None,
                });
                let has_deferred = self.prepared.arguments[..ordinal].iter()
                    .map(|argument| argument.deferred_text_values_v1(context))
                    .collect::<Result<Vec<_>, _>>()?.iter().any(|values| !values.is_empty());
                if has_deferred {
                    let prefix = ExpressionCallTransientPrefixV1::for_prepared_call_argument(
                        original.ok_or_else(|| freeze("argument-expression/prefix-original-call"))?,
                        ordinal, &self.prepared.arguments[..ordinal], context, inherited)?;
                    covered.extend(inner.validate_expression_arguments_at_source_v1(context, function, finishing,
                        bindings, coordinates, Some(&prefix), source)?);
                } else {
                    covered.extend(inner.validate_expression_arguments_at_source_v1(context, function, finishing,
                        bindings, coordinates, inherited, source)?);
                }
            }
        }
        covered.extend(self.validate_argument_lifetime_v1(context, function, finishing, inherited, original)?);
        Ok(covered)
    }

    /// Opaque children keep their own ownership checks. This only connects
    /// their original evaluation boundaries in source order before capture.
    pub(in crate::mir) fn validate_original_argument_order_v1(
        &self, function: &crate::mir::MirFunction,
    ) -> Result<(), String> {
        let mut previous = None;
        for argument in &self.prepared.arguments {
            if let Some((entry, completion)) = argument.original_argument_boundaries_v1()? {
                if let Some(previous) = previous { super::super::expression_projection::normal_order(function, &[previous, entry])?; }
                previous = Some(completion);
            }
        }
        if let Some(previous) = previous { super::super::expression_projection::normal_order(function, &[previous, self.invoke.clone()])?; }
        Ok(())
    }

    /// Verify the complete Call boundary, including an argument-free nested
    /// Call which still inherits earlier argument acquisitions.
    fn validate_argument_lifetime_v1(
        &self, context: CallValidationContextV1<'_>, function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>, inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
        original: Option<ExpressionCallArgumentSourceV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        use super::super::super::root_cleanup_graph::ordered_paths as paths;
        let ends = self.prepared.deferred_argument_ends_v1(context)?;
        if ends.is_empty() && inherited.is_none() {
            if self.argument_cleanup.is_some() { return Err(freeze("call-argument/unexpected-cleanup")); }
            return Ok(Default::default());
        }
        if matches!(context, CallValidationContextV1::Finalized(_)) {
            finishing.ok_or_else(|| freeze("call-argument/finished-boundary-missing"))?
                .validate_finished_sequences_v1(function)?;
        }
        let source = original.ok_or_else(|| freeze("call-argument/original-source"))?;
        let mut suffix = inherited.map(|prefix| prefix.unwind_for_call(self.call_site(), source.prior_homes()))
            .transpose()?.unwrap_or_default();
        suffix.extend(context.ledger().prior_home_unwind_for(source.prior_homes())?);
        let operations: Vec<_> = ends.iter().chain(&suffix).cloned().collect();
        let mut seen = std::collections::BTreeSet::new();
        for operation in &operations {
            if let InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value }) = operation {
                if !seen.insert(*value) { return Err(freeze("call-argument/duplicate-acquisition")); }
            }
        }
        let mapped = |binding: &Binding| match finishing {
            Some(boundary) => boundary.binding(binding.0, &binding.1)?
                .ok_or_else(|| freeze("call-argument/required-binding")),
            None => Ok(binding.clone()),
        };
        let invoke = mapped(&self.invoke)?;
        if !super::super::super::physical_boundary::check_binding(function, None, invoke.0, &invoke.1)? {
            return Err(freeze("call-argument/invoke-missing"));
        }
        let MirInstruction::Invoke { fault_frame, normal_landing, fault_landing, .. } = invoke.1 else {
            return Err(freeze("call-argument/invoke-shape"));
        };
        let mut covered = paths::pending_operations(function, fault_landing, &operations, Some(fault_frame))?;
        covered.insert(invoke.0);
        if ends.is_empty() {
            if self.argument_cleanup.is_some() { return Err(freeze("call-argument/unexpected-cleanup")); }
            return Ok(covered);
        }
        let cleanup = self.argument_cleanup.as_ref().ok_or_else(|| freeze("call-argument/cleanup-missing"))?;
        if cleanup.normal.len() != ends.len()
            || !matches!(&self.invoke.1, MirInstruction::Invoke { normal_landing, .. } if *normal_landing == cleanup.ingress.0) {
            return Err(freeze("call-argument/cleanup-delimiters"));
        }
        for delimiter in [&cleanup.ingress, &cleanup.continuation] {
            match finishing {
                Some(boundary) => { boundary.structural_jump_binding(function, delimiter,
                    matches!(context, CallValidationContextV1::Finalized(_)))?; }
                None if function.blocks.get(&delimiter.0).and_then(|block| block.terminator.as_ref()) == Some(&delimiter.1) => (),
                None => return Err(freeze("call-argument/cleanup-delimiters")),
            }
        }
        let stop = match finishing {
            Some(boundary) => boundary.destination(cleanup.continuation.0)
                .ok_or_else(|| freeze("call-argument/continuation-removed"))?,
            None => cleanup.continuation.0,
        };
        let mut normal = normal_landing;
        for (index, operation) in ends.iter().enumerate() {
            normal = paths::skip_until(function, normal, Some(stop))?;
            let observed = mapped(&cleanup.normal[index])?;
            if observed.0 != normal || !super::super::super::physical_boundary::check_binding(function, None, observed.0, &observed.1)? {
                return Err(freeze("call-argument/normal-end-origin"));
            }
            let (next, fault) = paths::release(function, normal, operation, Some(fault_frame))?;
            covered.insert(normal);
            covered.extend(paths::pending_operations(function, fault, &operations[index + 1..], Some(fault_frame))?);
            normal = next;
        }
        if paths::skip_until(function, normal, Some(stop))? != stop {
            return Err(freeze("call-argument/normal-continuation"));
        }
        Ok(covered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn same_owner_and_ordinal_cannot_substitute_another_original_argument_frame() {
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            "static box Values { find(p: i64): i64 { return p } run(n: i64): i64 { local a = Values.find(n + 1) local b = Values.find(n + 2) return 0 } } static box Main { main(args) { local out = Values.run(1) return 0 } }",
            crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let mut frames = Vec::new();
        for declaration in package.batch().declarations() {
            if let Some(flow) = ledger.completion_for_owner(declaration.owner()).and_then(|completion| completion.cleanup().root_flow()) {
                frames.extend(flow.expression_plans().iter().filter_map(|plan| plan.argument_frame()).cloned());
            }
        }
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].source().owner(), frames[1].source().owner());
        let context = CallValidationContextV1::Lowering(ledger);
        let prefix = ExpressionCallTransientPrefixV1::for_prepared_argument(&frames[0], &[], context, None).unwrap();
        assert!(prefix.for_argument(frames[0].evaluation()).unwrap().is_empty());
        assert!(prefix.for_argument(frames[1].evaluation()).is_err(),
            "same owner and ordinal must not widen a synchronous prefix to another original Call");
        let source = LocalCallArgumentV1::Expression { ordinal: 0, site: frames[1].source().nodes()[0].site().clone() };
        let projection = LexicalCallArgumentProjectionV1::Expression(Box::new(EmittedArgumentExpressionProjectionV1 {
            frame: std::rc::Rc::clone(&frames[0]), values: Vec::new(), calls: Vec::new(), path_reads: Vec::new(),
            reference_reads: Default::default(), entry: (BasicBlockId(1), MirInstruction::Jump { target: BasicBlockId(2), edge_args: None }),
            terminal: (BasicBlockId(2), MirInstruction::Jump { target: BasicBlockId(3), edge_args: None }),
            continuation: (BasicBlockId(3), MirInstruction::Jump { target: BasicBlockId(4), edge_args: None }), original: Vec::new(),
            completion: ArgumentCompletionV1::ScalarCompleted,
        }));
        assert!(projection.value_for_source_v1(&source, frames[1].source().owner(), frames[1].source().nodes()[0].site(),
            None, Some(CallValidationContextV1::Lowering(ledger))).unwrap_err().contains("argument-expression/ordinal-owner"),
            "foreign occurrence must refuse before examining any physical payload");
    }
}

impl PreparedLexicalCallProjectionV1 {
    /// Derive retained ownership from the existing ordered original packets.
    pub(in crate::mir) fn deferred_argument_ends_v1(
        &self, context: CallValidationContextV1<'_>,
    ) -> Result<Vec<InvokeOperation>, String> {
        let mut values = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for argument in &self.arguments {
            for value in argument.deferred_text_values_v1(context)? {
                if !seen.insert(value) { return Err(freeze("call-argument/duplicate-acquisition")); }
                values.push(value);
            }
        }
        Ok(values.into_iter().rev().map(|value| InvokeOperation::String(
            crate::mir::instruction::StringInvokeOperation::End { value })).collect())
    }
}

impl LexicalCallArgumentProjectionV1 {
    fn original_argument_boundaries_v1(&self) -> Result<Option<(Binding, Binding)>, String> {
        Ok(match self {
            Self::Expression(packet) => Some((packet.entry.clone(), packet.continuation.clone())),
            Self::CallResult(packet) => {
                let mut entry = packet.invoke.clone();
                for argument in &packet.prepared.arguments {
                    if let Some((first, _)) = argument.original_argument_boundaries_v1()? { entry = first; break; }
                }
                let completion = match &packet.argument_cleanup {
                    Some(cleanup) => cleanup.continuation.clone(),
                    None => packet.projection.value()?.clone(),
                };
                Some((entry, completion))
            }
            _ => None,
        })
    }
}

impl EmittedLexicalCallProjectionV1 {
    /// Lend the computed descriptor from the SAME original actual/frame
    /// and verified Call packet. Physical SSA classification cannot issue it.
    pub(in crate::mir) fn borrowed_computed_argument_operand_v1(
        &self, ordinal: u32, context: CallValidationContextV1<'_>,
    ) -> Result<Option<crate::mir::instruction::StringInvokeOperand>, String> {
        let actuals = match context {
            CallValidationContextV1::Finalized(source) => source.borrowed_packet_actuals_v1(self)?,
            CallValidationContextV1::Lowering(_) => self.row.loan().borrowed_actuals_in_context(context)?,
        };
        let Some(actuals) = actuals else { return Ok(None) };
        let Some(actual) = actuals.iter().find(|actual| actual.ordinal == ordinal) else { return Ok(None) };
        let crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1::Expression(source) = &actual.source
            else { return Ok(None); };
        let argument = self.prepared.arguments.get(ordinal as usize)
            .ok_or_else(|| freeze("call-argument/text-ordinal"))?;
        let value = argument.borrowed_expression_value_v1(&actual.source, self.call_site(), ordinal, &actual.site, context)?;
        Ok(Some(if source.frame().is_i64() { crate::mir::instruction::StringInvokeOperand::NormalInteger(value) }
            else { crate::mir::instruction::StringInvokeOperand::OwnedString(value) }))
    }
}
