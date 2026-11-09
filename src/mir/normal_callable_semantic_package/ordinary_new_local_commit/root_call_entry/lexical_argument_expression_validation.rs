//! Existing computed-argument physical proof, within the lexical argument family.
use super::*;

impl EmittedArgumentExpressionProjectionV1 {
    pub(super) fn ordinal(&self) -> Result<u32, String> {
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { ordinal, .. } = self.frame.source().root()
            else { return Err(freeze("argument-expression/source-purpose")) };
        Ok(*ordinal)
    }
    pub(super) fn expression(
        &self,
    ) -> super::super::super::expression_projection::ExpressionPhysicalProjectionRefV1<'_> {
        super::super::super::expression_projection::ExpressionPhysicalProjectionRefV1::new_with_dynamic(
            self.frame.evaluation(),
            &self.body.values,
            &self.body.calls,
            &self.body.dynamic_calls,
            &self.body.path_reads,
            &self.body.reference_reads,
            &self.body.bindings,
        ).with_inline_range_v1(self.body.inline_range)
    }
    pub(super) fn validate_source(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        let source = self.frame.source();
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal, .. } = source.root()
            else { return Err(freeze("argument-expression/source-purpose")) };
        let site = source
            .nodes()
            .first()
            .ok_or_else(|| freeze("argument-expression/root-missing"))?
            .site();
        let original = if let Some(frames) = &self.retained_argument_frames {
            let mut matches = frames
                .iter()
                .filter(|frame| std::rc::Rc::ptr_eq(frame, &self.frame));
            let original = matches
                .next()
                .ok_or_else(|| freeze("argument-expression/retained-frame-membership"))?;
            if matches.next().is_some()
                || original.source().owner() != source.owner()
                || !std::rc::Rc::ptr_eq(original.source_loan().plan(), original.source())
                || !original.evaluation().reference_coverage_is_exact()
                || original
                    .source()
                    .nodes()
                    .first()
                    .is_none_or(|root| root.site() != site)
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
            || context
                .ledger()
                .expression_requires_fault_frame_v1(self.frame.evaluation())
                && !matches!(
                    self.terminal.as_ref().map(|row| &row.1),
                    Some(MirInstruction::Jump {
                        edge_args: None,
                        ..
                    })
                )
            || context
                .ledger()
                .expression_requires_fault_frame_v1(self.frame.evaluation())
                && !matches!(
                    self.continuation.as_ref().map(|row| &row.1),
                    Some(MirInstruction::Jump {
                        edge_args: None,
                        ..
                    })
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
                    .body
                    .values
                    .first()
                    .and_then(Option::as_ref)
                    .ok_or_else(|| freeze("argument-expression/result"))?
                    .as_operand()?;
                let value = normal
                    .1
                    .dst_value()
                    .ok_or_else(|| freeze("argument-expression/result"))?;
                let snapshot = matches!(
                    input,
                    crate::mir::instruction::StringInvokeOperand::DynamicCarrier(_)
                ) && self.frame.evaluation().text_snapshot_root_v1().is_some_and(
                    |node| {
                        self.body
                            .values
                            .get(node)
                            .and_then(Option::as_ref)
                            .and_then(|value| value.as_operand().ok())
                            == Some(input)
                    },
                );
                if !(input.is_text() || snapshot)
                    || !matches!(&invoke.1, MirInstruction::Invoke {
                    operation: InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::Materialize { value }),
                    normal_landing, .. } if value == input && *normal_landing == normal.0)
                    || normal.1
                        != (MirInstruction::InvokeNormalResult {
                            invoke_block: invoke.0,
                            dst: value,
                        })
                    || !self.body.bindings.contains(invoke)
                    || !self.body.bindings.contains(normal)
                {
                    return Err(freeze("argument-expression/result"));
                }
                Some(value)
            }
            None => None,
        };
        let value = match (
            self.completion,
            self.body
                .values
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
                && self.frame.evaluation().owned_text_root_node_v1().is_some() =>
            {
                *value
            }
            _ => return Err(freeze("argument-expression/result")),
        };
        self.expression().validate_source(context)?;
        Ok(value)
    }
    pub(super) fn scope_bindings_in_context_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<(Vec<Binding>, Vec<Binding>), String> {
        self.validate_source(context)?;
        let mut ordered = self.expression().ordered_bindings_in_context(context)?;
        if let Some(pair) = &self.purpose_acquisition {
            ordered.extend(pair.iter().cloned());
        }
        ordered.extend(self.terminal.iter().cloned());

        let mut copies = self.copy_dependencies_in_context_v1(context)?;
        for call in &self.body.calls {
            copies.extend(
                call.copy_dependencies_in_context(context)?
                    .into_iter()
                    .map(|(_, binding)| binding),
            );
        }
        Ok((ordered, copies))
    }
    pub(super) fn source_loop_bindings_in_context_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let (ordered, copies) = self.scope_bindings_in_context_v1(context)?;
        let mut bindings = ordered;
        bindings.extend(copies);
        for read in self.body.reference_reads.values() {
            bindings.extend(read.dependencies()?);
        }
        for call in &self.body.calls {
            bindings.extend(call.source_loop_argument_bindings_in_context_v1(context)?);
        }
        for call in &self.body.dynamic_calls {
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
    pub(super) fn copy_dependencies_in_context_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let mut copies = Vec::new();
        for read in &self.body.path_reads {
            copies.extend_from_slice(read.original_copy_dependencies_v1()?);
        }
        for read in self.body.reference_reads.values() {
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
    pub(super) fn validate(
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
                    &self.body.entry,
                    &self.terminal,
                    &self.continuation,
                    inherited,
                )
            }
            ArgumentCompletionV1::OwnedTextDeferred => {
                self.expression().validate_deferred_owned_continuation_v1(
                    context,
                    function,
                    finishing,
                    &self.body.entry,
                    &self.terminal,
                    &self.continuation,
                    inherited,
                    self.purpose_acquisition.as_ref(),
                )
            }
            ArgumentCompletionV1::DynamicCarrierDeferred => {
                let carrier = self.validate_source(context)?;
                self.expression()
                    .validate_dynamic_carrier_argument_continuation_v1(
                        context,
                        function,
                        finishing,
                        &self.body.entry,
                        &self.terminal,
                        &self.continuation,
                        inherited,
                        carrier,
                    )
            }
        }
    }
}
