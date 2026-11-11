//! Existing computed-argument physical proof, within the lexical argument family.
use super::*;

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
        body: crate::mir::builder::EmittedExpressionBodyV1,
        terminal: Option<Binding>,
        continuation: Option<Binding>,
        purpose_acquisition: Option<[Binding; 2]>,
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
        } else if frame.materializes_root()
            || frame.evaluation().owned_text_root_node_v1().is_some()
        {
            ArgumentCompletionV1::OwnedTextDeferred
        } else {
            return Err(freeze("argument-expression/deferred-source-root"));
        };
        let packet = EmittedArgumentExpressionProjectionV1 {
            completion,
            purpose_acquisition,
            frame,
            body,
            terminal,
            continuation,
            retained_argument_frames,
        };
        let context = CallValidationContextV1::Lowering(ledger);
        let covered = packet.validate(context, function, None, inherited)?;
        let mut ordered = packet.expression().ordered_bindings_in_context(context)?;
        if let Some(pair) = &packet.purpose_acquisition {
            ordered.extend(pair.iter().cloned());
        }
        ordered.extend(packet.terminal.iter().cloned());
        packet.expression().validate_original_scope(
            CallValidationContextV1::Lowering(ledger),
            function,
            &covered,
            &packet.body.entry,
            &packet.terminal,
            &packet.continuation,
            &ordered,
            None,
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
            .body
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
    pub(in crate::mir) fn deferred_temporaries_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<ExpressionOwnedTemporaryV1>, String> {
        let Self::Expression(packet) = self else {
            return Ok(Vec::new());
        };
        let root = packet.validate_source(context)?;
        match packet.completion {
            ArgumentCompletionV1::ScalarCompleted => return Ok(Vec::new()),
            // This continuation has already ended every child temporary;
            // only the affine root is deferred to the append consumer.
            ArgumentCompletionV1::DynamicCarrierDeferred => {
                return Ok(vec![ExpressionOwnedTemporaryV1::DynamicCarrier(root)]);
            }
            ArgumentCompletionV1::OwnedTextDeferred => {}
        }
        packet.body.ordered_owned_temporaries_v1(
            packet.frame.evaluation(),
            root,
            packet.purpose_acquisition.is_some(),
        )
    }
    /// Operations owed before Call entry. A taken New's obligation ends
    /// at entry, so it does not join the post-entry temporary stack.
    pub(in crate::mir) fn pre_entry_end_operations_v1(
        &self,
        call: &OwnedExprSiteV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<InvokeOperation>, String> {
        if let Self::TakenObjectArgument {
            site,
            value,
            origin: TakenObjectArgumentOriginV1::Completed(_),
            ..
        } = self
        {
            let acquisition = OwnedExprSiteV1::new(call.owner(), site.clone());
            return context
                .ledger()
                .new_argument_object_cleanup_v1(&acquisition, *value)
                .map(|(_, operations)| operations);
        }
        Ok(self
            .deferred_temporaries_v1(context)?
            .into_iter()
            .rev()
            .map(ExpressionOwnedTemporaryV1::end_operation)
            .collect())
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
        if let Self::TakenObjectArgument {
            ordinal: position,
            site,
            ..
        } = self
        {
            if *position as usize != ordinal || !site.is_argument_of(call.site(), *position) {
                return Err(freeze("argument-expression/prefix-position"));
            }
            return Ok(());
        }
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
            Self::Expression(packet) => &packet.body.bindings,
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
    pub(super) fn validate_expression_physical_with_prefix_v1(
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
        .flat_map(|packet| packet.body.calls.iter().map(std::rc::Rc::as_ref))
    }

    pub(in super::super) fn visit_expression_calls_v1(
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
            for (source, call) in packet
                .frame
                .evaluation()
                .calls()
                .iter()
                .zip(&packet.body.calls)
            {
                call.visit_original_nodes_in_context_v1(owner, source.arguments(), context, visit)?;
            }
        }
        Ok(())
    }
}
