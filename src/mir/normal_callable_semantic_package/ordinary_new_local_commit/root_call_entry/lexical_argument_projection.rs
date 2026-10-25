//! Tie an observed argument value to the original lexical argument meaning.
use super::super::expression_projection::{
    ExpressionCallArgumentSourceV1, ExpressionCallTransientPrefixV1,
};
use super::*;
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;

impl LexicalCallArgumentProjectionV1 {
    pub(super) fn source_loop_bindings_in_context_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let bindings = match self {
            Self::Expression(packet) => packet.source_loop_bindings_in_context_v1(context)?,
            Self::CallResult(call) => call.source_loop_argument_bindings_in_context_v1(context)?,
            _ => Vec::new(),
        };
        Ok(bindings)
    }

    pub(crate) fn dynamic_carrier_append_entry_binding_v1(
        &self,
    ) -> Result<&Binding, String> {
        let Self::Expression(packet) = self else {
            return Err(freeze("array-append/dynamic-expression-projection"));
        };
        let root = packet
            .frame
            .source()
            .nodes()
            .first()
            .ok_or_else(|| freeze("array-append/dynamic-expression-root"))?
            .site();
        let mut calls = packet
            .dynamic_calls
            .iter()
            .filter(|call| call.source().site().site() == root);
        let entry = calls
            .next()
            .ok_or_else(|| freeze("array-append/dynamic-expression-call"))?;
        if calls.next().is_some() {
            return Err(freeze("array-append/dynamic-expression-call-duplicate"));
        }
        Ok(entry.invoke_binding())
    }

    pub(super) fn borrowed_expression_value_v1(
        &self,
        actual: &crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        let crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1::Expression(
            original,
        ) = actual
        else {
            return Err(freeze("argument-expression/borrowed-source-required"));
        };
        let Self::Expression(packet) = self else {
            return Err(freeze("argument-expression/borrowed-packet-required"));
        };
        let source = packet.frame.source();
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call: outer, ordinal: position, .. } = source.root()
            else { return Err(freeze("argument-expression/source-purpose")) };
        if !std::rc::Rc::ptr_eq(original.frame(), &packet.frame)
            || packet.completion
                != if packet.frame.is_i64() {
                    ArgumentCompletionV1::ScalarCompleted
                } else if packet.frame.is_dynamic_carrier_append_v1() {
                    ArgumentCompletionV1::DynamicCarrierDeferred
                } else {
                    ArgumentCompletionV1::OwnedTextDeferred
                }
            || source.owner() != call.owner()
            || outer.site() != call.site()
            || *position != ordinal
            || source.nodes().first().map(|node| node.site()) != Some(site)
        {
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
                if packet.completion != ArgumentCompletionV1::ScalarCompleted
                    || packet.ordinal()? != *ordinal
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
enum ArgumentCompletionV1 {
    ScalarCompleted,
    OwnedTextDeferred,
    DynamicCarrierDeferred,
}

/// Observations remain within the enclosing Call packet, never a second pool.
#[derive(Debug)]
pub(in crate::mir) struct EmittedArgumentExpressionProjectionV1 {
    frame: std::rc::Rc<
        crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
    >,
    values: Vec<Option<EvaluatedExpressionValueV1>>,
    calls: Vec<std::rc::Rc<EmittedLexicalCallProjectionV1>>,
    dynamic_calls: Vec<std::rc::Rc<crate::mir::normal_callable_semantic_package::EmittedDynamicInvocationCallProjectionV1>>,
    path_reads: Vec<std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>>,
    reference_reads: std::collections::BTreeMap<
        usize,
        super::super::expression_projection::ExpressionReferenceReadV1,
    >,
    entry: Binding,
    terminal: Binding,
    continuation: Binding,
    original: Vec<Binding>,
    purpose_acquisition: Option<[Binding; 2]>,
    completion: ArgumentCompletionV1,
    retained_argument_frames: Option<
        Box<[
            std::rc::Rc<
                crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
            >
        ]>,
    >,
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
        super::super::expression_projection::ExpressionPhysicalProjectionRefV1::new_with_dynamic(
            self.frame.evaluation(),
            &self.values,
            &self.calls,
            &self.dynamic_calls,
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
        let original = if let Some(frames) = &self.retained_argument_frames {
            let mut matches = frames.iter().filter(|frame| std::rc::Rc::ptr_eq(frame, &self.frame));
            let original = matches
                .next()
                .ok_or_else(|| freeze("argument-expression/retained-frame-membership"))?;
            if matches.next().is_some()
                || original.source().owner() != source.owner()
                || !std::rc::Rc::ptr_eq(original.source_loan().plan(), original.source())
                || !original.evaluation().reference_coverage_is_exact()
                || original.source().nodes().first().is_none_or(|root| root.site() != site)
            {
                return Err(freeze("argument-expression/retained-frame-drift"));
            }
            std::rc::Rc::clone(original)
        } else {
            context.ledger().qualified_argument_expression_frame_v1(
                &crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                    source.owner(),
                    call.site().clone(),
                ),
                *ordinal,
                site,
            )?
        };
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
        if self.purpose_acquisition.is_some() != self.frame.materializes_root() {
            return Err(freeze("argument-expression/deferred-source-root"));
        }
        let materialized = match &self.purpose_acquisition {
            Some([invoke, normal]) => {
                let input = self
                    .values
                    .first()
                    .and_then(Option::as_ref)
                    .ok_or_else(|| freeze("argument-expression/result"))?
                    .as_operand()?;
                let value = normal
                    .1
                    .dst_value()
                    .ok_or_else(|| freeze("argument-expression/result"))?;
                if !input.is_text()
                    || !matches!(&invoke.1, MirInstruction::Invoke {
                    operation: InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::Materialize { value }),
                    normal_landing, .. } if value == input && *normal_landing == normal.0)
                    || normal.1
                        != (MirInstruction::InvokeNormalResult {
                            invoke_block: invoke.0,
                            dst: value,
                        })
                    || !self.original.contains(invoke)
                    || !self.original.contains(normal)
                {
                    return Err(freeze("argument-expression/result"));
                }
                Some(value)
            }
            None => None,
        };
        let value = match (
            self.completion,
            self.values
                .first()
                .and_then(Option::as_ref)
                .map(|value| value.as_operand())
                .transpose()?,
        ) {
            (
                ArgumentCompletionV1::ScalarCompleted,
                Some(crate::mir::instruction::StringInvokeOperand::NormalInteger(value)),
            ) if self.frame.is_i64() => *value,
            (
                ArgumentCompletionV1::DynamicCarrierDeferred,
                Some(crate::mir::instruction::StringInvokeOperand::DynamicCarrier(value)),
            ) if self.frame.is_dynamic_carrier_append_v1() => *value,
            (ArgumentCompletionV1::OwnedTextDeferred, _) if materialized.is_some() => {
                materialized.unwrap()
            }
            (
                ArgumentCompletionV1::OwnedTextDeferred,
                Some(crate::mir::instruction::StringInvokeOperand::OwnedString(value)),
            ) if !self.frame.is_i64()
                && self.frame.evaluation().owned_result_nodes().contains(&0) =>
            {
                *value
            }
            _ => return Err(freeze("argument-expression/result")),
        };
        self.expression().validate_source(context)?;
        Ok(value)
    }
    fn scope_bindings_in_context_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<(Vec<Binding>, Vec<Binding>), String> {
        self.validate_source(context)?;
        let mut ordered = self.expression().ordered_bindings_in_context(context)?;
        if let Some(pair) = &self.purpose_acquisition {
            ordered.extend(pair.iter().cloned());
        }
        ordered.push(self.terminal.clone());

        let mut copies = self.copy_dependencies_in_context_v1(context)?;
        for call in &self.calls {
            copies.extend(
                call.copy_dependencies_in_context(context)?
                    .into_iter()
                    .map(|(_, binding)| binding),
            );
        }
        Ok((ordered, copies))
    }
    fn source_loop_bindings_in_context_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let (ordered, copies) = self.scope_bindings_in_context_v1(context)?;
        let mut bindings = ordered;
        bindings.extend(copies);
        for read in self.reference_reads.values() {
            bindings.extend(read.dependencies()?);
        }
        for call in &self.calls {
            bindings.extend(call.source_loop_argument_bindings_in_context_v1(context)?);
        }
        for call in &self.dynamic_calls {
            let (ordered, copies) = call.scope_bindings_in_context_v1(context)?;
            bindings.extend(ordered);
            bindings.extend(copies);
        }
        let mut unique = Vec::new();
        for binding in bindings {
            if !unique.contains(&binding) {
                unique.push(binding);
            }
        }
        Ok(unique)
    }
    fn copy_dependencies_in_context_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let mut copies = Vec::new();
        for read in &self.path_reads {
            copies.extend_from_slice(read.original_copy_dependencies_v1()?);
        }
        for read in self.reference_reads.values() {
            copies.extend_from_slice(read.original_copy_dependencies_v1()?);
        }
        copies.extend(self.expression().scalar_copy_dependencies_v1(context)?);
        for node in self.frame.source().nodes() {
            copies.extend(context.ledger().checked_field_copy_dependencies_v1(
                &OwnedExprSiteV1::new(self.frame.source().owner(), node.site().clone()),
                context,
            )?);
        }
        Ok(copies)
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
            ArgumentCompletionV1::ScalarCompleted => {
                self.expression().validate_scalar_continuation_v1(
                    context,
                    function,
                    finishing,
                    &self.entry,
                    &self.terminal,
                    &self.continuation,
                    inherited,
                )
            }
            ArgumentCompletionV1::OwnedTextDeferred => self
                .expression()
                .validate_deferred_argument_continuation_v1(
                    context,
                    function,
                    finishing,
                    &self.entry,
                    &self.terminal,
                    &self.continuation,
                    inherited,
                    self.purpose_acquisition.as_ref(),
                ),
            ArgumentCompletionV1::DynamicCarrierDeferred => {
                let carrier = self.validate_source(context)?;
                self.expression()
                    .validate_dynamic_carrier_argument_continuation_v1(
                        context,
                        function,
                        finishing,
                        &self.entry,
                        &self.terminal,
                        &self.continuation,
                        inherited,
                        carrier,
                    )
            }
        }
    }
}
impl LexicalCallArgumentProjectionV1 {
    pub(crate) fn dynamic_call_argument_scope_bindings_v1(
        &self,
        source: &LocalCallArgumentV1,
        call: &OwnedExprSiteV1,
        ordinal: usize,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<(Vec<Binding>, Vec<Binding>), String> {
        let (
            Self::Expression(packet),
            LocalCallArgumentV1::Expression {
                ordinal: observed,
                site: observed_site,
            },
        ) = (self, source)
        else {
            return Err(freeze("dynamic-expression/argument-purpose"));
        };
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
            call: original,
            ordinal: position,
            ..
        } = packet.frame.source().root()
        else {
            return Err(freeze("dynamic-expression/argument-purpose"));
        };
        if *observed as usize != ordinal
            || observed_site != site
            || original.owner() != call.owner()
            || original.site() != call.site()
            || *position as usize != ordinal
            || packet.frame.source().owner() != call.owner()
            || packet
                .frame
                .source()
                .nodes()
                .first()
                .is_none_or(|node| node.site() != site)
            || packet.completion != ArgumentCompletionV1::ScalarCompleted
            || !packet.frame.is_i64()
        {
            return Err(freeze("dynamic-expression/argument-source"));
        }
        packet.scope_bindings_in_context_v1(context)
    }

    pub(in crate::mir) fn from_expression_v1(
        frame: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
        >,
        values: Vec<Option<EvaluatedExpressionValueV1>>,
        calls: Vec<std::rc::Rc<EmittedLexicalCallProjectionV1>>,
        dynamic_calls: Vec<std::rc::Rc<crate::mir::normal_callable_semantic_package::EmittedDynamicInvocationCallProjectionV1>>,
        path_reads: Vec<std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>>,
        reference_reads: std::collections::BTreeMap<
            usize,
            super::super::expression_projection::ExpressionReferenceReadV1,
        >,
        entry: Binding,
        terminal: Binding,
        continuation: Binding,
        purpose_acquisition: Option<[Binding; 2]>,
        original: Vec<Binding>,
        retained_argument_frames: &[std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
        >],
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
        function: &crate::mir::MirFunction,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<Self, String> {
        let retained_argument_frames = if retained_argument_frames
            .iter()
            .any(|candidate| std::rc::Rc::ptr_eq(candidate, &frame))
        {
            Some(retained_argument_frames.to_vec().into_boxed_slice())
        } else {
            None
        };
        let completion = if frame.is_i64() {
            ArgumentCompletionV1::ScalarCompleted
        } else if frame.is_dynamic_carrier_append_v1() {
            ArgumentCompletionV1::DynamicCarrierDeferred
        } else if frame.materializes_root() || frame.evaluation().owned_result_nodes().contains(&0)
        {
            ArgumentCompletionV1::OwnedTextDeferred
        } else {
            return Err(freeze("argument-expression/deferred-source-root"));
        };
        let packet = EmittedArgumentExpressionProjectionV1 {
            completion,
            purpose_acquisition,
            frame,
            values,
            calls,
            dynamic_calls,
            path_reads,
            reference_reads,
            entry,
            terminal,
            continuation,
            original,
            retained_argument_frames,
        };
        let context = CallValidationContextV1::Lowering(ledger);
        let covered = packet.validate(context, function, None, inherited)?;
        let mut ordered = packet.expression().ordered_bindings_in_context(context)?;
        if let Some(pair) = &packet.purpose_acquisition {
            ordered.extend(pair.iter().cloned());
        }
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
    pub(crate) fn dynamic_carrier_append_value_v1(
        &self,
        append: &crate::mir::normal_callable_semantic_package::OwnedArrayAppendSourceLoanV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<(ValueId, Binding), String> {
        let Self::Expression(packet) = self else {
            return Err(freeze("array-append/dynamic-expression-projection"));
        };
        let source = packet.frame.source();
        let Some(root) = source.nodes().first() else {
            return Err(freeze("array-append/dynamic-expression-root"));
        };
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
            call,
            ordinal,
            ..
        } = source.root()
        else {
            return Err(freeze("array-append/dynamic-expression-purpose"));
        };
        let holder = append
            .dynamic_carrier_source()
            .ok_or_else(|| freeze("array-append/dynamic-holder-source"))?;
        if packet.completion != ArgumentCompletionV1::DynamicCarrierDeferred
            || !packet.frame.is_dynamic_carrier_append_v1()
            || source.owner() != append.site().owner()
            || call.owner() != append.site().owner()
            || call.site() != append.site().site()
            || *ordinal != 0
            || root.site() != append.candidate().site()
            || packet.frame.source_loan().dynamic_array_carrier_append_v1() != Some(&holder)
        {
            return Err(freeze("array-append/dynamic-expression-source"));
        }
        let root_call = packet
            .dynamic_calls
            .iter()
            .filter(|call| call.source().site().site() == root.site());
        let mut root_calls = root_call;
        let call_packet = root_calls
            .next()
            .ok_or_else(|| freeze("array-append/dynamic-expression-call"))?;
        if root_calls.next().is_some() {
            return Err(freeze("array-append/dynamic-expression-call-duplicate"));
        }
        let value = packet.validate_source(context)?;
        if call_packet.value() != value {
            return Err(freeze("array-append/dynamic-expression-result"));
        }
        Ok((value, call_packet.invoke_binding().clone()))
    }
    /// Source-bound live-out observations, never a second acquisition inventory.
    pub(in crate::mir) fn deferred_text_values_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<ValueId>, String> {
        let Self::Expression(packet) = self else {
            return Ok(Vec::new());
        };
        let root = packet.validate_source(context)?;
        if packet.completion == ArgumentCompletionV1::ScalarCompleted {
            return Ok(Vec::new());
        }
        let mut values = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for &node in packet.frame.evaluation().ordered_nodes() {
            if !packet
                .frame
                .evaluation()
                .owned_result_nodes()
                .contains(&node)
            {
                continue;
            }
            let Some(crate::mir::instruction::StringInvokeOperand::OwnedString(value)) = packet
                .values
                .get(node)
                .and_then(Option::as_ref)
                .map(|value| value.as_operand())
                .transpose()?
            else {
                return Err(freeze("argument-expression/deferred-value"));
            };
            if !seen.insert(*value) {
                return Err(freeze("argument-expression/deferred-duplicate"));
            }
            values.push(*value);
        }
        if packet.purpose_acquisition.is_some() {
            if !seen.insert(root) {
                return Err(freeze("argument-expression/deferred-duplicate"));
            }
            values.push(root);
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
        &self,
        call: &OwnedExprSiteV1,
        ordinal: usize,
        homes: &[LiveHomeV1],
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        let Self::Expression(packet) = self else {
            return Ok(());
        };
        let source = packet.frame.source();
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call: original, ordinal: position, .. } = source.root()
            else { return Err(freeze("argument-expression/source-purpose")) };
        if source.owner() != call.owner()
            || original.site() != call.site()
            || *position as usize != ordinal
            || packet.frame.prior_homes() != homes
        {
            return Err(freeze("argument-expression/prefix-position"));
        }
        packet.validate_source(context).map(drop)
    }
    pub(in crate::mir) fn validate_expression_recorded_v1(
        &self,
        bindings: &[Binding],
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        if let Self::Expression(packet) = self {
            packet.expression().validate_recorded(bindings, finishing)?;
        }
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
        self.validate_expression_physical_with_prefix_v1(
            context,
            function,
            finishing,
            bindings,
            coordinates,
            None,
        )
    }
    fn validate_expression_physical_with_prefix_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>,
        bindings: &[Binding],
        coordinates: Option<&FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        if let Self::Expression(packet) = self {
            let covered = packet.validate(context, function, finishing, inherited)?;
            packet
                .expression()
                .validate_recorded(bindings, coordinates)?;
            return Ok(covered);
        }
        Ok(Default::default())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn expression_copy_dependencies_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let mut copies = Vec::new();
        if let Self::Expression(packet) = self {
            packet.validate_source(context)?;
            copies.extend(packet.copy_dependencies_in_context_v1(context)?);
        }
        Ok(copies)
    }

    /// Child packets remain borrowed from their original evaluation, never
    /// reconstructed from the flattened MIR or promoted to another pool.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn expression_calls_v1(
        &self,
    ) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        match self {
            Self::Expression(packet) => Some(packet.as_ref()),
            _ => None,
        }
        .into_iter()
        .flat_map(|packet| packet.calls.iter().map(std::rc::Rc::as_ref))
    }

    pub(super) fn visit_expression_calls_v1(
        &self,
        owner: FunctionOwnerIdV1,
        context: CallValidationContextV1<'_>,
        visit: &mut impl FnMut(
            &EmittedLexicalCallProjectionV1,
            &[LocalCallArgumentV1],
        ) -> Result<(), String>,
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
    pub(in crate::mir::normal_callable_semantic_package) fn deferred_argument_ends_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<InvokeOperation>, String> {
        self.prepared.deferred_argument_ends_v1(context)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn argument_completion_binding_v1(
        &self,
    ) -> Result<&Binding, String> {
        match &self.argument_cleanup {
            Some(cleanup) => Ok(&cleanup.continuation),
            None => self.outer_bindings().map(|(_, normal)| normal),
        }
    }

    /// The enclosing Call is the common consumer of its source-bound argument
    /// evaluations. Completed scalar children stay outside its live-temp stack.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_expression_arguments_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>,
        bindings: &[Binding],
        coordinates: Option<&FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_expression_arguments_with_prefix_v1(
            context,
            function,
            finishing,
            bindings,
            coordinates,
            None,
        )
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_expression_arguments_with_prefix_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>,
        bindings: &[Binding],
        coordinates: Option<&FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_expression_arguments_with_cancellation_v1(context, function, finishing,
            bindings, coordinates, inherited, &[])
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_expression_arguments_with_cancellation_v1(
        &self, context: CallValidationContextV1<'_>, function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>, bindings: &[Binding],
        coordinates: Option<&FinishedBindings>, inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
        normal_fault_cancellation: &[InvokeOperation],
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        let original = match self.row.loan() {
            crate::mir::normal_callable_semantic_package::CallPacketSourceLoanV1::Static { observation, .. } => {
                self.row.loan().validate_static(context.ledger())?;
                Some(ExpressionCallArgumentSourceV1::from_local(observation))
            }
            crate::mir::normal_callable_semantic_package::CallPacketSourceLoanV1::Instance(_)
            | crate::mir::normal_callable_semantic_package::CallPacketSourceLoanV1::SelectedStaticLoop { .. } => {
                context.ledger().local_call_for_owner(self.call_site().owner(), self.call_site().site())
                    .map(ExpressionCallArgumentSourceV1::from_local)
            }
        };
        self.validate_expression_arguments_at_source_v1(
            context,
            function,
            finishing,
            bindings,
            coordinates,
            inherited,
            original,
            normal_fault_cancellation,
        )
    }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_expression_arguments_at_source_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        finishing: Option<&FinishedBindings>,
        bindings: &[Binding],
        coordinates: Option<&FinishedBindings>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
        original: Option<ExpressionCallArgumentSourceV1<'_>>,
        normal_fault_cancellation: &[InvokeOperation],
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        if let Some(prefix) = inherited {
            prefix.require_call(self)?;
        }
        if original.is_some_and(|source| {
            source.site() != self.call_site()
                || source.arguments().len() != self.prepared.arguments.len()
        }) {
            return Err(freeze("argument-expression/prefix-original-call"));
        }
        if finishing.is_none() {
            self.validate_original_argument_order_v1(function)?;
        }
        let mut covered = std::collections::BTreeSet::new();
        for (ordinal, argument) in self.prepared.arguments.iter().enumerate() {
            if let LexicalCallArgumentProjectionV1::Expression(packet) = argument {
                let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal: position, .. } = packet.frame.source().root()
                    else { return Err(freeze("argument-expression/source-purpose")) };
                if packet.frame.source().owner() != self.call_site().owner()
                    || call.site() != self.call_site().site()
                    || *position as usize != ordinal
                    || self.row.loan().argument_sites().get(ordinal)
                        != packet
                            .frame
                            .source()
                            .nodes()
                            .first()
                            .map(|node| node.site())
                {
                    return Err(freeze("argument-expression/enclosing-call"));
                }
                let prefix = ExpressionCallTransientPrefixV1::for_prepared_call_argument(
                    original.ok_or_else(|| freeze("argument-expression/prefix-original-call"))?,
                    ordinal,
                    &self.prepared.arguments[..ordinal],
                    context,
                    inherited,
                )?;
                prefix.for_argument(packet.frame.evaluation())?;
                covered.extend(argument.validate_expression_physical_with_prefix_v1(
                    context,
                    function,
                    finishing,
                    bindings,
                    coordinates,
                    Some(&prefix),
                )?);
                continue;
            }
            covered.extend(argument.validate_expression_physical_with_prefix_v1(
                context,
                function,
                finishing,
                bindings,
                coordinates,
                inherited,
            )?);
            if let LexicalCallArgumentProjectionV1::CallResult(inner) = argument {
                let source = original.and_then(|source| match source.arguments().get(ordinal) {
                    Some(LocalCallArgumentV1::CallResult(inner)) => {
                        Some(ExpressionCallArgumentSourceV1::from_nested(inner))
                    }
                    _ => None,
                });
                let has_deferred = self.prepared.arguments[..ordinal]
                    .iter()
                    .map(|argument| argument.deferred_text_values_v1(context))
                    .collect::<Result<Vec<_>, _>>()?
                    .iter()
                    .any(|values| !values.is_empty());
                if has_deferred {
                    let prefix = ExpressionCallTransientPrefixV1::for_prepared_call_argument(
                        original
                            .ok_or_else(|| freeze("argument-expression/prefix-original-call"))?,
                        ordinal,
                        &self.prepared.arguments[..ordinal],
                        context,
                        inherited,
                    )?;
                    covered.extend(inner.validate_expression_arguments_at_source_v1(
                        context,
                        function,
                        finishing,
                        bindings,
                        coordinates,
                        Some(&prefix),
                        source,
                        &[],
                    )?);
                } else {
                    covered.extend(inner.validate_expression_arguments_at_source_v1(
                        context,
                        function,
                        finishing,
                        bindings,
                        coordinates,
                        inherited,
                        source,
                        &[],
                    )?);
                }
            }
        }
        covered.extend(self.validate_argument_lifetime_v1(
            context,
            function,
            finishing,
            inherited,
            original,
            normal_fault_cancellation,
        )?);
        Ok(covered)
    }

    /// Opaque children keep their own ownership checks. This only connects
    /// their original evaluation boundaries in source order before capture.
    pub(in crate::mir) fn validate_original_argument_order_v1(
        &self,
        function: &crate::mir::MirFunction,
    ) -> Result<(), String> {
        let mut previous = None;
        for argument in &self.prepared.arguments {
            if let Some((entry, completion)) = argument.original_argument_boundaries_v1()? {
                if let Some(previous) = previous {
                    super::super::expression_projection::normal_order(
                        function,
                        &[previous, entry],
                    )?;
                }
                previous = Some(completion);
            }
        }
        if let Some(previous) = previous {
            super::super::expression_projection::normal_order(
                function,
                &[previous, self.invoke.clone()],
            )?;
        }
        Ok(())
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
            if let Some(flow) = ledger
                .completion_for_owner(declaration.owner())
                .and_then(|completion| completion.cleanup().root_flow())
            {
                frames.extend(
                    flow.expression_plans()
                        .iter()
                        .filter_map(|plan| plan.argument_frame())
                        .cloned(),
                );
            }
        }
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].source().owner(), frames[1].source().owner());
        let context = CallValidationContextV1::Lowering(ledger);
        let prefix =
            ExpressionCallTransientPrefixV1::for_prepared_argument(&frames[0], &[], context, None)
                .unwrap();
        assert!(prefix
            .for_argument(frames[0].evaluation())
            .unwrap()
            .is_empty());
        assert!(
            prefix.for_argument(frames[1].evaluation()).is_err(),
            "same owner and ordinal must not widen a synchronous prefix to another original Call"
        );
        let source = LocalCallArgumentV1::Expression {
            ordinal: 0,
            site: frames[1].source().nodes()[0].site().clone(),
        };
        let projection = LexicalCallArgumentProjectionV1::Expression(Box::new(
            EmittedArgumentExpressionProjectionV1 {
                frame: std::rc::Rc::clone(&frames[0]),
                values: Vec::new(),
                calls: Vec::new(),
                dynamic_calls: Vec::new(),
                retained_argument_frames: None,
                path_reads: Vec::new(),
                reference_reads: Default::default(),
                entry: (
                    BasicBlockId(1),
                    MirInstruction::Jump {
                        target: BasicBlockId(2),
                        edge_args: None,
                    },
                ),
                terminal: (
                    BasicBlockId(2),
                    MirInstruction::Jump {
                        target: BasicBlockId(3),
                        edge_args: None,
                    },
                ),
                continuation: (
                    BasicBlockId(3),
                    MirInstruction::Jump {
                        target: BasicBlockId(4),
                        edge_args: None,
                    },
                ),
                original: Vec::new(),
                completion: ArgumentCompletionV1::ScalarCompleted,
                purpose_acquisition: None,
            },
        ));
        assert!(
            projection
                .value_for_source_v1(
                    &source,
                    frames[1].source().owner(),
                    frames[1].source().nodes()[0].site(),
                    None,
                    Some(CallValidationContextV1::Lowering(ledger))
                )
                .unwrap_err()
                .contains("argument-expression/ordinal-owner"),
            "foreign occurrence must refuse before examining any physical payload"
        );
    }
}

impl PreparedLexicalCallProjectionV1 {
    /// Derive retained ownership from the existing ordered original packets.
    pub(in crate::mir) fn deferred_argument_ends_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<InvokeOperation>, String> {
        let mut values = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for argument in &self.arguments {
            for value in argument.deferred_text_values_v1(context)? {
                if !seen.insert(value) {
                    return Err(freeze("call-argument/duplicate-acquisition"));
                }
                values.push(value);
            }
        }
        Ok(values
            .into_iter()
            .rev()
            .map(|value| {
                InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End {
                    value,
                })
            })
            .collect())
    }
}

impl LexicalCallArgumentProjectionV1 {
    fn original_argument_boundaries_v1(&self) -> Result<Option<(Binding, Binding)>, String> {
        Ok(match self {
            Self::Expression(packet) => Some((packet.entry.clone(), packet.continuation.clone())),
            Self::CallResult(packet) => {
                let mut entry = packet.invoke.clone();
                for argument in &packet.prepared.arguments {
                    if let Some((first, _)) = argument.original_argument_boundaries_v1()? {
                        entry = first;
                        break;
                    }
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
        &self,
        ordinal: u32,
        context: CallValidationContextV1<'_>,
    ) -> Result<Option<crate::mir::instruction::StringInvokeOperand>, String> {
        let actuals = match context {
            CallValidationContextV1::Finalized(source) => {
                source.borrowed_packet_actuals_v1(self)?
            }
            CallValidationContextV1::Lowering(_) => {
                self.row.loan().borrowed_actuals_in_context(context)?
            }
        };
        let Some(actuals) = actuals else {
            return Ok(None);
        };
        let Some(actual) = actuals.iter().find(|actual| actual.ordinal == ordinal) else {
            return Ok(None);
        };
        let crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1::Expression(
            source,
        ) = &actual.source
        else {
            return Ok(None);
        };
        let argument = self
            .prepared
            .arguments
            .get(ordinal as usize)
            .ok_or_else(|| freeze("call-argument/text-ordinal"))?;
        let value = argument.borrowed_expression_value_v1(
            &actual.source,
            self.call_site(),
            ordinal,
            &actual.site,
            context,
        )?;
        Ok(Some(if source.frame().is_i64() {
            crate::mir::instruction::StringInvokeOperand::NormalInteger(value)
        } else {
            crate::mir::instruction::StringInvokeOperand::OwnedString(value)
        }))
    }
}
