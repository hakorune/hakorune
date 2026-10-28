//! Source selection for the same Local initializer physical consumer.
use super::*;
use crate::mir::instruction::StringInvokeOperand as Operand;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn initializer_expression_frame_v1(
        &self, owner: FunctionOwnerIdV1, statement: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1>>, String>{
        let Some(flow) = self
            .completion_for_owner(owner)
            .and_then(|completion| completion.cleanup().root_flow())
        else {
            return Ok(None);
        };
        let mut frames = flow
            .expression_plans()
            .iter()
            .filter_map(|plan| plan.initializer_frame())
            .filter(|frame| frame.source().root().statement().node() == statement);
        let frame = frames.next().cloned();
        if frames.next().is_some() {
            return Err(freeze("text-initializer/duplicate-source-frame"));
        }
        if frame
            .as_ref()
            .is_some_and(|frame| frame.source().owner() != owner)
        {
            return Err(freeze("text-initializer/source-owner"));
        }
        Ok(frame)
    }

}

/// The completed Local receipt and the sole original evaluation share one pool.
#[derive(Debug)]
pub(crate) struct EmittedInitializerExpressionProjectionV1 {
    frame: Rc<CheckedInitializerExpressionFrameV1>,
    value: ValueId,
    local: ValueId,
    copy: Option<Binding>,
    install: Option<Binding>,
    values: Box<[Option<EvaluatedExpressionValueV1>]>,
    calls: Box<[Rc<EmittedLexicalCallProjectionV1>]>,
    dynamic_calls: Box<[Rc<EmittedDynamicInvocationCallProjectionV1>]>,
    path_reads: Box<[std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>]>,
    reference_reads:
        std::collections::BTreeMap<usize, super::expression_projection::ExpressionReferenceReadV1>,
    entry: Binding,
    terminal: Binding,
    continuation: Binding,
    loop_residency: Option<(Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        Vec<crate::mir::builder::BindingValueObservationV2>)>,
    /// The `Materialize` pair when the Local owns a Text snapshot of an owned carrier.
    purpose_acquisition: Option<[Binding; 2]>,
    original: Vec<Binding>,
}
impl EmittedInitializerExpressionProjectionV1 {
    pub(super) fn corroborate_loop_frame_v2(
        &self,
        entry: BasicBlockId,
        end: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        if self.entry.0 != entry
            || self.value() != value
            || !matches!(self.continuation.1, MirInstruction::Jump { target, edge_args: None } if target == end)
        {
            return Err(freeze("source-loop/binding-frame-correspondence"));
        }
        Ok(())
    }
    pub(crate) fn frame(&self) -> &Rc<CheckedInitializerExpressionFrameV1> {
        &self.frame
    }
    pub(crate) fn value(&self) -> ValueId {
        self.value
    }
    pub(in crate::mir) fn reference_read_v1(
        &self,
        node: usize,
    ) -> Option<&super::expression_projection::ExpressionReferenceReadV1> {
        self.reference_reads.get(&node)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn local_value(&self) -> ValueId {
        self.local
    }
    pub(in crate::mir::normal_callable_semantic_package) fn install_binding(
        &self,
    ) -> Option<&Binding> {
        self.install.as_ref()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn has_text_home(&self) -> bool {
        self.frame.is_text_residence() && self.value == self.local && self.copy.is_none()
            && self.install.as_ref().is_some_and(|(_, row)| matches!(row,
                MirInstruction::StringResidenceInstall { value, replaces: None, returning: false } if *value == self.value))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn has_dynamic_carrier(&self) -> bool {
        self.frame.is_dynamic_invocation_result()
            && self.value == self.local
            && self.copy.is_none()
            && self.install.is_none()
            && self.values.first() == Some(&Some(Operand::DynamicCarrier(self.value).into()))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn has_dynamic_alias(&self) -> bool {
        self.frame.is_dynamic_alias() && self.install.is_none()
            && self.values.first() == Some(&Some(Operand::Dynamic(self.value).into()))
            && self.copy.as_ref().is_some_and(|(_, row)| matches!(row,
                MirInstruction::Copy { dst, src } if *dst == self.local && *src == self.value))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn dynamic_read_binding_v1(&self) -> Result<&Binding, String> {
        if self.has_dynamic_alias() {
            return self.copy.as_ref().ok_or_else(|| freeze("initializer-expression/local-copy-receipt"));
        }
        self.dynamic_result_binding_v1()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn dynamic_result_binding_v1(
        &self,
    ) -> Result<&Binding, String> {
        if !self.has_dynamic_carrier() {
            return Err(freeze("initializer-expression/dynamic-carrier-local-handoff"));
        }
        let Some(root) = self.frame.source().nodes().first() else {
            return Err(freeze("initializer-expression/dynamic-source-root"));
        };
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            self.frame.source().owner(),
            root.site().clone(),
        );
        let mut ordinary = self.calls.iter().filter(|call| call.call_site() == &site
            && call.original_source().result() == Some(crate::mir::instruction::InvokeCallResultKind::DynamicCarrier));
        if let Some(call) = ordinary.next() {
            if ordinary.next().is_some() || self.dynamic_calls.iter().any(|call| call.source().site() == &site) {
                return Err(freeze("initializer-expression/dynamic-result-pair"));
            }
            let (_, normal) = call.outer_bindings()?;
            if !matches!(normal.1, MirInstruction::InvokeNormalResult { dst, .. } if dst == self.value) {
                return Err(freeze("initializer-expression/dynamic-result-pair"));
            }
            return Ok(normal);
        }
        let mut calls = self
            .dynamic_calls
            .iter()
            .filter(|call| call.source().site() == &site && call.value() == self.value);
        let call = calls
            .next()
            .ok_or_else(|| freeze("initializer-expression/dynamic-result-pair"))?;
        if calls.next().is_some() {
            return Err(freeze("initializer-expression/dynamic-result-pair"));
        }
        Ok(call.projection_binding())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn root_acquisition_binding_v1(
        &self,
    ) -> Result<&Binding, String> {
        if !self.has_text_home() {
            return Err(freeze("initializer-expression/no-text-home"));
        }
        self.expression().require_unique(|row| {
            matches!(row,
            MirInstruction::InvokeNormalResult { dst, .. } if *dst == self.value)
        })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn receipt_copy_binding_v1(
        &self,
    ) -> Option<&Binding> {
        self.copy.as_ref()
    }
    pub(crate) fn calls(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        self.calls.iter().map(Rc::as_ref)
    }
    /// Scalar dependencies of this same complete initializer. Call/field/
    /// ownership effects keep their own source consumers and are never lent.
    pub(in crate::mir::normal_callable_semantic_package) fn scalar_source_bindings_v1(
        &self, context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        if !self.frame.is_i64() { return Err(freeze("expression/scalar-entry-contract")); }
        let expression = self.expression();
        let mut source = expression.ordered_bindings_in_context(context)?;
        source.extend(expression.scalar_copy_dependencies_v1(context)?);
        for read in self.reference_reads.values() { source.extend(read.dependencies()?); }
        source.extend(self.receipt_copy_binding_v1().cloned());
        let mut bindings = Vec::new();
        for binding in source.into_iter().filter(|(_, instruction)| matches!(instruction,
            MirInstruction::Copy { .. } | MirInstruction::Phi { .. }
                | MirInstruction::BinOp { .. } | MirInstruction::Compare { .. })) {
            if !bindings.contains(&binding) { bindings.push(binding); }
        }
        Ok(bindings)
    }

    pub(super) fn source_loop_dependencies_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let mut dependencies = Vec::new();
        for read in &self.path_reads {
            dependencies.extend_from_slice(read.original_copy_dependencies_v1()?);
        }
        for read in self.reference_reads.values() {
            dependencies.extend(read.dependencies()?);
        }
        dependencies.extend(self.expression().ordered_bindings_in_context(context)?);
        dependencies.extend(self.expression().scalar_copy_dependencies_v1(context)?);
        for call in self.calls() {
            dependencies.extend(call.source_loop_argument_bindings_in_context_v1(context)?);
            dependencies.extend(
                call.copy_dependencies_in_context(context)?
                    .into_iter()
                    .map(|(_, binding)| binding),
            );
        }
        for call in &self.dynamic_calls {
            dependencies.extend(call.scope_bindings_in_context_v1(context)?.1);
        }
        for node in self.frame.source().nodes() {
            dependencies.extend(context.ledger().checked_field_copy_dependencies_v1(
                &OwnedExprSiteV1::new(self.frame.source().owner(), node.site().clone()),
                context,
            )?);
        }
        let mut unique = Vec::new();
        for dependency in dependencies {
            if !unique.contains(&dependency) {
                unique.push(dependency);
            }
        }
        Ok(unique)
    }
    pub(in crate::mir) fn new(
        frame: Rc<CheckedInitializerExpressionFrameV1>,
        value: ValueId,
        local: ValueId,
        copy: Option<Binding>,
        values: Vec<Option<EvaluatedExpressionValueV1>>,
        calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
        dynamic_calls: Vec<Rc<EmittedDynamicInvocationCallProjectionV1>>,
        path_reads: Vec<std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>>,
        reference_reads: std::collections::BTreeMap<
            usize,
            super::expression_projection::ExpressionReferenceReadV1,
        >,
        entry: Binding,
        terminal: Binding,
        continuation: Binding,
        loop_residency: Option<(Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
            Vec<crate::mir::builder::BindingValueObservationV2>)>,
        purpose_acquisition: Option<[Binding; 2]>,
        original: Vec<Binding>,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        let install = original
            .iter()
            .find(|(_, row)| matches!(row, MirInstruction::StringResidenceInstall { .. }))
            .cloned();
        let packet = Self {
            frame,
            value,
            local,
            copy,
            install,
            values: values.into_boxed_slice(),
            calls: calls.into_boxed_slice(),
            dynamic_calls: dynamic_calls.into_boxed_slice(),
            path_reads: path_reads.into_boxed_slice(),
            reference_reads,
            entry,
            terminal,
            continuation,
            loop_residency,
            purpose_acquisition,
            original,
        };
        packet.validate_source(CallValidationContextV1::Lowering(ledger))?;
        let covered =
            packet.validate_paths(CallValidationContextV1::Lowering(ledger), function, None)?;
        let mut ordered = packet
            .expression()
            .ordered_bindings_in_context(CallValidationContextV1::Lowering(ledger))?;
        if let Some(pair) = &packet.purpose_acquisition {
            ordered.extend(pair.iter().cloned());
        }
        if let Some(install) = &packet.install {
            ordered.push(install.clone());
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
        Ok(packet)
    }
    /// Lend original tagged inputs of source-checked String Add operations.
    /// The caller first validates this SAME packet and its finished group.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn visit_tagged_text_operands_v1(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: &super::super::physical_boundary::FinishedBindings,
        mut visit: impl FnMut(&Binding, usize, BindingRefV1, ValueId) -> Result<(), String>,
    ) -> Result<(), String> {
        use crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1 as Reference;
        use crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1 as Normal;
        let frame = self.frame.evaluation();
        let expression = self.expression();
        for loan in frame.operations() {
            let index = loan.node();
            if frame.transfers_node(index) { continue; }
            let Some(binary) = frame.source().nodes().get(index).and_then(|node| node.binary()) else { continue; };
            if binary.operator() != crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add
                || loan.normal_result_v1() != Some(Normal::OwnedStringValue) { continue; }
            if !Rc::ptr_eq(loan.source(), frame.source()) { return Err(freeze("print-expression/add-source")); }
            let Some(EvaluatedExpressionValueV1::Operand(Operand::OwnedString(result))) = self.values.get(index).and_then(Option::as_ref)
                else { return Err(freeze("print-expression/add-values")); };
            let normal = expression.require_unique(|row| matches!(row,
                MirInstruction::InvokeNormalResult { dst, .. } if dst == result))?;
            let MirInstruction::InvokeNormalResult { invoke_block, .. } = normal.1 else { unreachable!() };
            let invoke = expression.require_unique(|row| matches!(row,
                MirInstruction::Invoke { operation: InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::Add { .. }), normal_landing, .. }
                    if *normal_landing == normal.0))?;
            if invoke.0 != invoke_block { return Err(freeze("print-expression/add-producer")); }
            for (side, site) in [binary.lhs(), binary.rhs()].into_iter().enumerate() {
                let node = frame.source().nodes().iter().position(|node| node.site() == site)
                    .ok_or_else(|| freeze("print-expression/dynamic-add-operand"))?;
                let Some(EvaluatedExpressionValueV1::Operand(Operand::Dynamic(value))) = self.values.get(node).and_then(Option::as_ref)
                    else { continue; };
                let formal = match frame.references().get(&node) {
                    Some(Reference::FiniteEntry { read, exclusion }) if read.tagged_string_integer_only_v1(exclusion.as_deref()) => {
                        if let Some(exclusion) = exclusion {
                            context.validate_finite_entry_nonnull_use_v1(read, exclusion, function, finishing, invoke)?;
                        }
                        read.requirement().formal().1
                    }
                    Some(Reference::DynamicEntry(read)) => read.requirement().formal().1,
                    _ => continue,
                };
                let read = self.reference_reads.get(&node).ok_or_else(|| freeze("expression/reference-read-missing"))?;
                if read.operand(frame, node, context)? != Operand::Dynamic(*value) {
                    return Err(freeze("expression/reference-source-value"));
                }
                visit(invoke, side, formal, *value)?;
            }
        }
        Ok(())
    }
    fn expression(&self) -> super::expression_projection::ExpressionPhysicalProjectionRefV1<'_> {
        super::expression_projection::ExpressionPhysicalProjectionRefV1::new_with_dynamic(
            self.frame.evaluation(),
            &self.values,
            &self.calls,
            &self.dynamic_calls,
            &self.path_reads,
            &self.reference_reads,
            &self.original,
        )
    }
    fn validate_source(&self, context: CallValidationContextV1<'_>) -> Result<(), String> {
        let selected = context
            .ledger()
            .initializer_expression_frame_v1(
                self.frame.source().owner(),
                self.frame.source().root().statement().node(),
            )?
            .ok_or_else(|| freeze("initializer-expression/source-missing"))?;
        let expected = if self.frame.is_i64() {
            Operand::NormalInteger(self.value)
        } else if self.has_dynamic_carrier() {
            Operand::DynamicCarrier(self.value)
        } else if self.has_dynamic_alias() {
            Operand::Dynamic(self.value)
        } else if self.has_text_home() {
            // A snapshot Local keeps the original carrier as the evaluation root.
            match &self.purpose_acquisition {
                Some([invoke, _]) => match &invoke.1 {
                    MirInstruction::Invoke {
                        operation: InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::Materialize { value }),
                        ..
                    } => match value {
                        crate::mir::instruction::StringInvokeOperand::DynamicCarrier(carrier) => Operand::DynamicCarrier(*carrier),
                        _ => return Err(freeze("initializer-expression/source-or-result")),
                    },
                    _ => return Err(freeze("initializer-expression/source-or-result")),
                },
                None => Operand::OwnedString(self.value),
            }
        } else {
            return Err(freeze("initializer-expression/source-or-result"));
        };
        if self.purpose_acquisition.is_some() != self.frame.materializes_root() {
            return Err(freeze("initializer-expression/snapshot-root"));
        }
        if let Some([invoke, normal]) = &self.purpose_acquisition {
            if !matches!(&invoke.1, MirInstruction::Invoke {
                    operation: InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::Materialize { .. }),
                    normal_landing, .. } if *normal_landing == normal.0)
                || normal.1 != (MirInstruction::InvokeNormalResult { invoke_block: invoke.0, dst: self.value })
                || !self.original.contains(invoke)
                || !self.original.contains(normal)
            {
                return Err(freeze("initializer-expression/snapshot-pair"));
            }
        }
        if !Rc::ptr_eq(&selected, &self.frame)
            || self.values.first() != Some(&Some(expected.into()))
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
            return Err(freeze("initializer-expression/source-or-result"));
        }
        let installs = self
            .original
            .iter()
            .filter(|(_, row)| matches!(row, MirInstruction::StringResidenceInstall { .. }))
            .count();
        if installs != usize::from(self.has_text_home()) {
            return Err(freeze("initializer-expression/install-census"));
        }
        if self.frame.is_dynamic_invocation_result() && !self.has_dynamic_carrier() {
            return Err(freeze(
                "initializer-expression/dynamic-carrier-local-handoff",
            ));
        }
        if self.frame.is_i64()
            && self.copy.as_ref().is_none_or(|(_, row)| {
                !matches!(row, MirInstruction::Copy { dst, src }
            if *dst == self.local && *src == self.value)
            })
        {
            return Err(freeze("initializer-expression/local-copy-receipt"));
        }
        if let Some(sum) = self.frame.signed_root_sum() {
            let Some(crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::Scalar(read))
                = self.frame.evaluation().references().get(&0) else { return Err(freeze("initializer-expression/source-or-result")); };
            let read = read
                .original_read()
                .ok_or_else(|| freeze("initializer-expression/source-or-result"))?;
            let source = context
                .ledger()
                .borrowed_formal_source
                .as_ref()
                .ok_or_else(|| freeze("initializer-expression/source-or-result"))?
                .as_ref()
                .map_err(Clone::clone)?;
            let current = source
                .lend_current_scalar_sum_v1(read)
                .ok_or_else(|| freeze("initializer-expression/source-or-result"))?;
            if !self.frame.is_i64()
                || self.frame.owns_root()
                || !sum.signed_only()
                || !Rc::ptr_eq(sum, &current)
                || read.read() != self.frame.source().nodes()[0].site()
            {
                return Err(freeze("initializer-expression/source-or-result"));
            }
        }
        self.expression().validate_source(context)
    }
    pub(super) fn validate_recorded(
        &self,
        bindings: &[Binding],
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.expression().validate_recorded(bindings, finishing)?;
        for expected in [&self.entry, &self.terminal, &self.continuation] {
            if !self.original.contains(expected) {
                return Err(freeze("initializer-expression/structural-membership"));
            }
        }
        if finishing.is_none()
            && self
                .copy
                .as_ref()
                .is_some_and(|copy| bindings.iter().filter(|row| *row == copy).count() != 1)
        {
            return Err(freeze("initializer-expression/local-copy-membership"));
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_paths(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_source(context)?;
        context.ledger().validate_initializer_local_copy_v1(
            context,
            function,
            finishing,
            self.value,
            self.local,
            self.copy.as_ref(),
        )?;
        let unwind = self.loop_residency.as_ref().map(|(product, reads)|
            context.ledger().loop_expression_text_unwind_v2(self.frame.evaluation(), product, reads,
                context, function, finishing)).transpose()?;
        let expression = self.expression();
        let expression = match &unwind {
            Some(unwind) => expression.with_prior_home_unwind(unwind),
            None => expression,
        };
        if self.has_dynamic_carrier() {
            if self.dynamic_result_binding_v1()?.1.dst_value() != Some(self.value) {
                return Err(freeze("initializer-expression/dynamic-result-drift"));
            }
            expression.validate_dynamic_local_handoff_continuation_v1(
                context,
                function,
                finishing,
                &self.entry,
                &self.terminal,
                &self.continuation,
                self.value,
            )
        } else {
            expression.validate_local_continuation_with_purpose_v1(
                context,
                function,
                finishing,
                &self.entry,
                &self.terminal,
                &self.continuation,
                self.install.as_ref(),
                None,
                self.purpose_acquisition.as_ref(),
            )
        }
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn prepare_initializer_expression_bindings(
        &self,
        packet: Rc<EmittedInitializerExpressionProjectionV1>,
        bindings: Vec<Binding>,
    ) -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        self.prepare_local_call_binding_group(
            packet.frame.source().owner(),
            RootLifecyclePrefixBindingGroupV1::new_initializer(packet, bindings)?,
        )
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn validate_initializer_expression_source_v1(
        &self,
        frame: &Rc<CheckedInitializerExpressionFrameV1>,
    ) -> Result<(), String> {
        let site = OwnedExprSiteV1::new(
            frame.source().owner(),
            frame.source().nodes()[0].site().clone(),
        );
        let source = self.next_prefix_source_v2(frame.source().owner(), &site)?;
        if !matches!(source.as_ref(), Some(LifecyclePrefixSourceV1::InitializerExpression { frame: original, .. }) if Rc::ptr_eq(original, frame))
        {
            return Err(freeze("initializer-expression/source-prefix-order"));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "initializer_expression_projection_tests.rs"]
mod tests;

impl OrdinaryNewClaimLedgerV1 {
    /// Same completed Local Copy law for scalar/Text and nonowning get purposes.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_initializer_local_copy_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        value: ValueId,
        local: ValueId,
        copy: Option<&Binding>,
    ) -> Result<Option<Binding>, String> {
        if let Some(copy) = copy {
            if !matches!(copy.1, MirInstruction::Copy { dst, src } if dst == local && src == value)
            {
                return Err(freeze("initializer-expression/local-copy-receipt"));
            }
            let actual = match (finishing, context) {
                (Some(boundary), CallValidationContextV1::Finalized(_)) => {
                    boundary.relation_binding(function, copy)?
                }
                (Some(boundary), CallValidationContextV1::Lowering(_)) => {
                    if boundary.captured_definition(local)? != copy {
                        return Err(freeze("initializer-expression/local-copy-capture"));
                    }
                    None
                }
                (None, _) => Some(copy.clone()),
            };
            if let Some((block, instruction)) = &actual {
                let mut definitions = function
                    .blocks
                    .iter()
                    .flat_map(|(id, body)| body.all_instructions().map(move |row| (*id, row)))
                    .filter(|(_, row)| row.dst_value() == Some(local));
                if function.params.contains(&local)
                    || definitions.next() != Some((*block, instruction))
                    || definitions.next().is_some()
                {
                    return Err(freeze("initializer-expression/local-definition-identity"));
                }
                if !super::super::physical_boundary::check_binding(
                    function,
                    None,
                    *block,
                    instruction,
                )? {
                    return Err(freeze("initializer-expression/local-copy-missing"));
                }
            }
            return Ok(actual);
        }
        Ok(None)
    }
}
