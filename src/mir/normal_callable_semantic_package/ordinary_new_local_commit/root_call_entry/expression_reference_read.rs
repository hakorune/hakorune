//! One physical read observation; source proof stays at its original frame node.
use super::*;
use crate::mir::builder::{ExactLexicalReadV1, BindingValueObservationV2};
use crate::mir::resolved_semantics::home_new_prefix::{CurrentScalarReadV1, CurrentScalarProducerV1 as Producer};
use crate::mir::{ValueId, ConstValue};
use crate::mir::instruction::StringInvokeOperand as Operand;
use crate::mir::resolved_semantics::home_new_prefix::{CheckedExpressionEvaluationV1, CheckedExpressionReferenceV1 as Reference};

#[derive(Debug, Clone)]
pub(in crate::mir) enum ReferenceProducerV1 {
    Definition(Binding), Entry(ValueId),
    Phi(std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>),
}
impl PartialEq for ReferenceProducerV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Definition(a), Self::Definition(b)) => a == b,
            (Self::Entry(a), Self::Entry(b)) => a == b,
            (Self::Phi(a), Self::Phi(b)) => std::rc::Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
#[derive(Debug)]
pub(in crate::mir) struct ExpressionReferenceReadV1 {
    read: std::rc::Rc<ExactLexicalReadV1>,
    producer: ReferenceProducerV1,
}
pub(in crate::mir) struct PreparedExpressionReferenceReadV1 { producer: ReferenceProducerV1 }
impl PreparedExpressionReferenceReadV1 {
    pub(in crate::mir) fn finish(self, read: ExactLexicalReadV1) -> ExpressionReferenceReadV1 {
        self.finish_shared(std::rc::Rc::new(read))
    }
    pub(in crate::mir) fn finish_shared(self, read: std::rc::Rc<ExactLexicalReadV1>) -> ExpressionReferenceReadV1 {
        ExpressionReferenceReadV1 { read, producer: self.producer }
    }
}
impl ExpressionReferenceReadV1 {
    /// External initializer/Copy producers are dependencies, not newly emitted
    /// expression instructions. Only the sealed finisher approves omissions.
    pub(in crate::mir) fn validate_dependencies_v1(&self, frame: &CheckedExpressionEvaluationV1, node: usize,
        context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_layout(frame, node, context, function)?;
        if let Reference::Scalar(read) = original_reference(frame, node)? {
            if let Some(source) = read.original_read() {
                if matches!(source.producer(), Producer::BranchNormal(_)) {
                    self.read.loan_scalar_join_v1(source.producer())?.branch()
                        .ok_or_else(|| freeze("expression/scalar-producer-kind"))?.validate(context, function, finishing)?;
                }
            }
        }
        if let ReferenceProducerV1::Phi(point) = &self.producer {
            if let Some(projection) = finishing {
                if matches!(context, CallValidationContextV1::Finalized(_)) {
                    point.require_completed_v1()?;
                    point.verify_projected(function, &mut |row| projection.relation_binding(function, row))?;
                }
            } else { point.verify(function)?; }
        }
        if let (Some(projection), CallValidationContextV1::Finalized(_)) = (finishing, context) {
            return self.validate_finished_dependencies_v1(function, projection);
        }
        for original in self.dependencies()? {
            match (finishing, context) {
                (Some(projection), CallValidationContextV1::Finalized(_)) => { projection.relation_binding(function, &original)?; }
                (Some(projection), CallValidationContextV1::Lowering(_)) => {
                    let dst = original.1.dst_value().ok_or_else(|| freeze("expression/reference-dependency-dst"))?;
                    if projection.captured_definition(dst)? != &original { return Err(freeze("expression/reference-captured-dependency")); }
                }
                (None, _) => {
                    if !super::super::super::physical_boundary::check_binding(function, None, original.0, &original.1)? {
                        return Err(freeze("expression/reference-dependency-missing"));
                    }
                }
            }
        }
        Ok(())
    }
    pub(in crate::mir) fn validate_finished_dependencies_v1(&self, function: &MirFunction,
        projection: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit::physical_boundary::FinishedBindings,
    ) -> Result<(), String> {
        if let ReferenceProducerV1::Phi(point) = &self.producer {
            point.require_completed_v1()?;
            point.verify_projected(function, &mut |row| projection.relation_binding(function, row))?;
        }
        for original in self.dependencies()? { projection.relation_binding(function, &original)?; }
        Ok(())
    }
    /// Prepare before the existing exact-read batch commits. Source proof stays
    /// in the shared evaluation, not in another reference/operand registry.
    pub(in crate::mir) fn prepare(
        frame: &CheckedExpressionEvaluationV1, node: usize, read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<PreparedExpressionReferenceReadV1, String> {
        let source = original_reference(frame, node)?;
        let producer = match source {
            Reference::Scalar(_) | Reference::FiniteEntry { .. } => return Self::prepare_source_reference_v1(
                frame.source(), node, source, read, context, function),
            Reference::StaticText(original) => {
                let value = read.loan_local_initializer_v1(original.definition_binding())?.0;
                let mut rows = function.blocks.iter().flat_map(|(block, body)| body.all_instructions()
                    .map(move |row| (*block, row))).filter(|(_, row)| row.dst_value() == Some(value));
                let (block, row) = rows.next().ok_or_else(|| freeze("expression/static-initializer-missing"))?;
                if rows.next().is_some() { return Err(freeze("expression/static-initializer-producer")); }
                ReferenceProducerV1::Definition((block, row.clone()))
            }
            Reference::Residence(original) => {
                let binding = context.with_text_residence_v1(original.acquisition(), read,
                    |_, binding| Ok(binding.clone()))?;
                ReferenceProducerV1::Definition(binding)
            }
            Reference::ArgumentEntry(requirement) => ReferenceProducerV1::Entry(
                prepare_entry(requirement, None, read, context, function)?),
        };
        reference_operand(frame, node, read, &producer, context)?;
        if !matches!(source, Reference::ArgumentEntry(_) | Reference::FiniteEntry { .. }) {
            validate_dependencies(read.binding_observation_v2(), &producer, function)?;
        }
        Ok(PreparedExpressionReferenceReadV1 { producer })
    }
    pub(in crate::mir) fn operand(&self, frame: &CheckedExpressionEvaluationV1, node: usize,
        context: CallValidationContextV1<'_>,
    ) -> Result<Operand, String> {
        reference_operand(frame, node, &self.read, &self.producer, context)
    }
    pub(super) fn validate_layout(&self, frame: &CheckedExpressionEvaluationV1, node: usize,
        context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<(), String> {
        if let Reference::Scalar(source) = original_reference(frame, node)? {
            if matches!(self.producer, ReferenceProducerV1::Phi(_)) {
                validate_source(frame.source(), source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?,
                    &self.read, &self.producer, context)?;
            }
            let original = source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?;
            validate_entry_layout(original.binding(), original.producer(), &self.producer, context, function)?;
        }
        let entry = match original_reference(frame, node)? {
            Reference::ArgumentEntry(required) => Some(prepare_entry(required, None, &self.read, context, function)?),
            Reference::FiniteEntry { read, .. } => Some(prepare_entry(read.requirement(), Some(read), &self.read, context, function)?),
            _ => None,
        };
        if entry.is_some_and(|entry| !matches!(self.producer, ReferenceProducerV1::Entry(original) if original == entry)) {
            return Err(freeze("expression/reference-text-entry-producer"));
        }
        Ok(())
    }
    pub(in crate::mir) fn definition(&self) -> Option<&Binding> {
        match &self.producer { ReferenceProducerV1::Definition(binding) => Some(binding), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn original_copy_dependencies_v1(&self) -> Result<&[Binding], String> {
        self.read.original_copy_dependencies_v1()
    }
    pub(in crate::mir) fn dependencies(&self) -> Result<std::vec::IntoIter<Binding>, String> {
        let mut rows: Vec<_> = self.definition().cloned().into_iter().collect();
        if let ReferenceProducerV1::Phi(point) = &self.producer {
            point.append_observed_bindings_v1(&mut rows);
        }
        rows.extend_from_slice(self.read.original_copy_dependencies_v1()?);
        Ok(rows.into_iter())
    }
    /// A parent Loop owns its indexed Phi once. Reads keep the full dependency,
    /// but the child cannot claim that Phi as its own emitted instruction.
    pub(in crate::mir) fn without_borrowed_phis_v2<'a>(bindings: Vec<Binding>,
        reads: impl Iterator<Item = &'a Self>,
    ) -> Result<Vec<Binding>, String> {
        let reads: Vec<_> = reads.collect();
        let mut own = Vec::new();
        for binding in bindings {
            let mut borrowed = false;
            if matches!(binding.1, MirInstruction::Phi { .. }) {
                for read in &reads {
                    if let ReferenceProducerV1::Phi(point) = &read.producer {
                        if point.resolve_observed_binding_v1(&binding, false)?.is_some() { borrowed = true; }
                    }
                }
            }
            if !borrowed { own.push(binding); }
        }
        Ok(own)
    }
    /// Reuse the RHS's already checked producer. An alias Copy is itself the
    /// result definition; self-assignment of an entry has no new instruction.
    pub(in crate::mir) fn result_producer_v1(&self, value: ValueId) -> Result<ReferenceProducerV1, String> {
        if self.read.source_value().2 != value { return Err(freeze("rebind-expression/rhs-read-value")); }
        if let Some(copy) = self.read.original_copy_dependencies_v1()?.last() {
            if copy.1.dst_value() == Some(value) { return Ok(ReferenceProducerV1::Definition(copy.clone())); }
        }
        match &self.producer {
            ReferenceProducerV1::Definition(binding) if binding.1.dst_value() == Some(value) => Ok(self.producer.clone()),
            ReferenceProducerV1::Entry(entry) if *entry == value => Ok(self.producer.clone()),
            ReferenceProducerV1::Phi(point) if point.values().0 == value => Ok(self.producer.clone()),
            _ => Err(freeze("rebind-expression/rhs-producer")),
        }
    }
    pub(in crate::mir) fn prepare_scalar_read_v2(
        plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>, source: &CurrentScalarReadV1, read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<PreparedExpressionReferenceReadV1, String> {
        let producer = prepare_scalar_binding_producer(source.binding(), source.producer(),
            read.binding_observation_v2(), context, function)?;
        validate_source(plan, source, read, &producer, context)?;
        Ok(PreparedExpressionReferenceReadV1 { producer })
    }
    /// A terminal source read uses the same producer/Copy/Phi verifier without
    /// manufacturing an expression frame or consuming another lexical read.
    pub(in crate::mir) fn scalar_read_value_v2(&self,
        plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        source: &CurrentScalarReadV1, context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<ValueId, String> { self.validate_scalar_read_v2(plan, source, context, function, None) }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_scalar_read_v2(&self,
        plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        source: &CurrentScalarReadV1, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<ValueId, String> {
        let value = validate_source(plan, source, &self.read, &self.producer, context)?;
        validate_scalar_binding_observation_v1(source.binding(), source.producer(), self.read.binding_observation_v2(),
            &self.producer, context, function, finishing)?;
        Ok(value)
    }
    /// The same value reference serves eager RHS and Bool control purposes.
    /// Bool remains a scalar value, never a StringInvokeOperand or I64 cast.
    pub(in crate::mir) fn prepare_source_reference_v1(
        plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        node: usize, source: &Reference, read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<PreparedExpressionReferenceReadV1, String> {
        if !source.matches_node(plan, node) { return Err(freeze("expression/reference-source-node")); }
        match source {
            Reference::Scalar(source) => Self::prepare_scalar_read_v2(plan,
                source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?, read, context, function),
            Reference::FiniteEntry { read: original, .. } => Ok(PreparedExpressionReferenceReadV1 {
                producer: ReferenceProducerV1::Entry(prepare_entry(original.requirement(), Some(original), read, context, function)?),
            }),
            _ => Err(freeze("expression/reference-source-node")),
        }
    }
    pub(in crate::mir) fn source_reference_value_v1(&self,
        plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        node: usize, source: &Reference, context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<ValueId, String> { self.validate_source_reference_v1(plan, node, source, context, function, None) }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_source_reference_v1(&self,
        plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        node: usize, source: &Reference, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<ValueId, String> {
        if !source.matches_node(plan, node) { return Err(freeze("expression/reference-source-node")); }
        match source {
            Reference::Scalar(source) => self.validate_scalar_read_v2(plan,
                source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?, context, function, finishing),
            Reference::FiniteEntry { read, .. } => {
                let entry = prepare_entry(read.requirement(), Some(read), &self.read, context, function)?;
                if !matches!(self.producer, ReferenceProducerV1::Entry(original) if original == entry) {
                    return Err(freeze("expression/reference-text-entry-producer"));
                }
                if let Some(finishing) = finishing { self.validate_finished_dependencies_v1(function, finishing)?; }
                entry_value(read.requirement(), &self.read, &self.producer, context)
            },
            _ => Err(freeze("expression/reference-source-node")),
        }
    }
    /// The SAME scalar producer/provenance check for a loop's original Entry.
    /// No expression frame or read site is synthesized or consumed.
    pub(in crate::mir) fn validate_scalar_binding_v2(binding: crate::mir::resolved_semantics::BindingRefV1,
        source: &Producer, read: &BindingValueObservationV2, context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<Vec<Binding>, String> {
        let producer = prepare_scalar_binding_producer(binding, source, read, context, function)?;
        let mut dependencies = Vec::new();
        match producer {
            ReferenceProducerV1::Definition(row) => dependencies.push(row),
            ReferenceProducerV1::Phi(point) => point.append_observed_bindings_v1(&mut dependencies),
            ReferenceProducerV1::Entry(_) => {},
        }
        dependencies.extend_from_slice(read.original_copy_dependencies_v1()?);
        Ok(dependencies)
    }

}
fn prepare_scalar_binding_producer(binding: crate::mir::resolved_semantics::BindingRefV1,
    source: &Producer, read: &BindingValueObservationV2, context: CallValidationContextV1<'_>,
    function: &MirFunction,
) -> Result<ReferenceProducerV1, String> {
        let producer = match source {
            Producer::BranchNormal(_) | Producer::LoopHeader(_) | Producer::LoopAfter { .. } =>
                ReferenceProducerV1::Phi(std::rc::Rc::clone(read.loan_scalar_join_v1(source)?.relation())),
            Producer::BindingRebind(frame) => context.with_original_rebind_v1(frame, |packet| {
                let value = read.loan_checked_rebind_v1(frame)?.0;
                if packet.value() != value { return Err(freeze("expression/rebind-assigned-value")); }
                Ok(packet.root_producer().clone())
            })?,
            Producer::SignedEntry(formal) => {
                let (ordinal, _, _, _) = signed_entry_layout(binding, *formal, context)?;
                let value = *function.params.get(ordinal).ok_or_else(|| freeze("expression/scalar-entry-param"))?;
                ReferenceProducerV1::Entry(value)
            }
            _ => {
                let value = initializer_value(source, read, context)?;
                let mut rows = function.blocks.iter().flat_map(|(block, body)| body.all_instructions()
                    .map(move |row| (*block, row))).filter(|(_, row)| row.dst_value() == Some(value));
                let (block, row) = rows.next().ok_or_else(|| freeze("expression/scalar-producer-missing"))?;
                if rows.next().is_some() || function.params.contains(&value) { return Err(freeze("expression/scalar-producer-unique")); }
                ReferenceProducerV1::Definition((block, row.clone()))
            }
        };
        validate_scalar_binding_observation_v1(binding, source, read, &producer, context, function, None)?;
        Ok(producer)
}
fn original_reference(frame: &CheckedExpressionEvaluationV1, node: usize) -> Result<&Reference, String> {
    let reference = frame.references().get(&node).ok_or_else(|| freeze("expression/reference-source-missing"))?;
    if !reference.matches_node(frame.source(), node) { return Err(freeze("expression/reference-source-node")); }
    Ok(reference)
}
fn reference_operand(frame: &CheckedExpressionEvaluationV1, node: usize, read: &ExactLexicalReadV1,
    producer: &ReferenceProducerV1, context: CallValidationContextV1<'_>,
) -> Result<Operand, String> {
    let reference = original_reference(frame, node)?;
    let site = &frame.source().nodes()[node];
    read.value_for(frame.source().owner(), site.site().node(), reference.binding()).map_err(|error| error.to_string())?;
    match reference {
        Reference::StaticText(original) => {
            let value = read.loan_local_initializer_v1(original.definition_binding())?.0;
            if !matches!(producer, ReferenceProducerV1::Definition((_, MirInstruction::Const {
                dst, value: ConstValue::String(text) })) if *dst == value && text == original.utf8()) {
                return Err(freeze("expression/reference-static-value"));
            }
            Ok(Operand::StaticText { utf8: original.utf8().to_owned() })
        }
        Reference::Scalar(source) => Ok(Operand::NormalInteger(validate_source(
            frame.source(), source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?, read, producer, context)?)),
        Reference::Residence(original) => {
            if frame.prior_homes().iter().filter(|home| matches!(home, crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Text(acquisition) if acquisition.same_source(original.acquisition()))).count() != 1
                || original.root() != original.acquisition().binding() {
                return Err(freeze("expression/residence-current-home"));
            }
            context.with_text_residence_v1(original.acquisition(), read, |value, original_binding| {
                if !matches!(producer, ReferenceProducerV1::Definition(binding) if binding == original_binding) {
                    return Err(freeze("expression/residence-original-pair"));
                }
                // Borrow the original pair: the alias observation proves its
                // correspondence but creates no new acquisition or cleanup.
                Ok(Operand::OwnedString(value))
            })
        }
        Reference::ArgumentEntry(required) => text_entry_operand(required, read, producer, context),
        Reference::FiniteEntry { read: original, .. } => {
            let value = entry_value(original.requirement(), read, producer, context)?;
            if original.signed_only() { Ok(Operand::NormalInteger(value)) }
            else if original.leaf_class(None) == Some(crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
                crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String)) { Ok(Operand::BorrowedText(value)) }
            else { Err(freeze("expression/reference-execution-pending/text-entry-domain")) }
        },
    }
}
fn prepare_entry(
    required: &crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1,
    initializer: Option<&crate::mir::normal_callable_semantic_package::FiniteEntryReadV1>,
    read: &ExactLexicalReadV1, context: CallValidationContextV1<'_>, function: &MirFunction,
) -> Result<ValueId, String> {
    let (entry, _, domain) = context.ledger().validate_prepared_finite_entry_read_v1(required, function, read)?;
    if initializer.map_or(!domain.text_only(), |original|
        !original.corroborates_finite_domain_v1(&domain)
            || !(domain.text_only() || domain.signed_only() || domain.bool_only())) {
        return Err(freeze("expression/reference-execution-pending/text-entry-domain"));
    }
    let (index, count, receivers, symbol) = context.ledger().original_entry_layout_v1(
        required.binding(), required.formal().1, |kind| matches!(kind,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1::OpaqueHandle))?;
    if function.signature.name != symbol || function.params.len() != count
        || function.params.get(index) != Some(&entry) || function.metadata.declared_param_decls.len() != count
        || function.metadata.declared_param_decls.iter().enumerate().any(|(index, row)|
            row.implicit_receiver != (receivers == 1 && index == 0)) {
        return Err(freeze("expression/reference-text-entry-layout"));
    }
    Ok(entry)
}
fn text_entry_operand(required: &crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1,
    read: &ExactLexicalReadV1, producer: &ReferenceProducerV1, context: CallValidationContextV1<'_>,
) -> Result<Operand, String> { entry_value(required, read, producer, context).map(Operand::BorrowedText) }
fn entry_value(required: &crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1,
    read: &ExactLexicalReadV1, producer: &ReferenceProducerV1, context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    let entries = context.ledger().borrowed_ordinary_entry_values_v1(required.binding().owner())?;
    let mut matching = entries.iter().filter(|(ordinal, formal, _)| (*ordinal, *formal) == required.formal());
    let entry = matching.next().ok_or_else(|| freeze("expression/reference-text-entry-value"))?.2;
    if matching.next().is_some() || !matches!(producer, ReferenceProducerV1::Entry(value) if *value == entry) {
        return Err(freeze("expression/reference-text-entry-producer"));
    }
    read.loan_forwarded_copies(required.formal().1, entry)?;
    read.value_for(required.binding().owner(), required.read().node(), required.binding())
        .map_err(|error| error.to_string())
}
fn signed_entry_layout(binding: crate::mir::resolved_semantics::BindingRefV1, formal: crate::mir::resolved_semantics::BindingRefV1,
    context: CallValidationContextV1<'_>,
) -> Result<(usize, usize, usize, String), String> {
    context.ledger().original_entry_layout_v1(binding, formal, |kind| matches!(kind,
        crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()))
}

fn initializer_value(source: &Producer, read: &BindingValueObservationV2, context: CallValidationContextV1<'_>) -> Result<ValueId, String> {
    let (binding, expected) = match source {
        Producer::Integer { binding, .. } => (*binding, None),
        Producer::Call(call) => {
            if context.ledger().lexical_i64_call_source(call.site()) != Some(call) { return Err(freeze("expression/scalar-call-source")); }
            let binding = call.local_binding().ok_or_else(|| freeze("expression/scalar-call-local"))?.1;
            let value = context.original_scalar_call_result_v1(call)?;
            (binding, Some(value))
        }
        Producer::Initializer(frame) => (frame.binding(), Some(context.with_original_initializer_v1(frame, |packet| Ok(packet.value()))?)),
        Producer::SignedEntry(_) => return Err(freeze("expression/scalar-entry-not-initializer")),
        Producer::BranchNormal(_) => return Err(freeze("expression/scalar-producer-kind")),
        Producer::BindingRebind(_) => return Err(freeze("expression/scalar-rebind-not-initializer")),
        Producer::LoopHeader(_) => return Err(freeze("expression/loop-header-physical-pending")),
        Producer::LoopAfter { .. } => return Err(freeze("expression/loop-after-physical-pending")),
    };
    let value = read.loan_local_initializer_v1(binding)?.0;
    if expected.is_some_and(|expected| expected != value) { return Err(freeze("expression/scalar-original-result")); }
    Ok(value)
}
fn validate_source(plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>, source: &CurrentScalarReadV1, read: &ExactLexicalReadV1, producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    read.value_for(source.binding().owner(), source.read().node(), source.binding()).map_err(|error| error.to_string())?;
    if plan.owner() != source.binding().owner() || !plan.nodes().iter().any(|node| node.site() == source.read()
        && matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(
            crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == source.binding())) {
        return Err(freeze("expression/reference-source-node"));
    }
    if matches!(source.producer(), Producer::LoopHeader(_)) {
        let product = context.source_loop_point_product_v2(source.producer())?;
        if !product.owns_expression_v2(plan) { return Err(freeze("expression/loop-header-source-scope")); }
    }
    validate_scalar_origin(source.binding(), source.producer(), read.binding_observation_v2(), producer, context)
}
fn validate_scalar_origin(binding: crate::mir::resolved_semantics::BindingRefV1,
    source: &Producer, read: &BindingValueObservationV2, producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    let value = read.value_for(binding.owner(), binding).map_err(|error| error.to_string())?;
    if let Producer::BranchNormal(basis) = source {
        context.ledger().require_branch_scalar_source_v1(basis)?;
        let point = read.loan_scalar_join_v1(source)?;
        let ReferenceProducerV1::Phi(relation) = producer else { return Err(freeze("expression/scalar-producer-kind")); };
        if !std::rc::Rc::ptr_eq(point.relation(), relation) || point.branch().is_none() {
            return Err(freeze("expression/scalar-producer-kind"));
        }
        relation.require_completed_v1()?;
        return Ok(value);
    }
    if matches!(source, Producer::LoopHeader(_) | Producer::LoopAfter { .. }) {
        let ReferenceProducerV1::Phi(point) = producer else { return Err(freeze("expression/loop-point-producer-kind")); };
        if !std::rc::Rc::ptr_eq(read.loan_scalar_join_v1(source)?.relation(), point) {
            return Err(freeze("expression/loop-point-original-purpose"));
        }
        context.source_loop_point_product_v2(source)?;
        if matches!(source, Producer::LoopAfter { .. }) || matches!(context, CallValidationContextV1::Finalized(_)) {
            point.require_completed_v1()?;
        }
        return Ok(value);
    }
    if let Producer::BindingRebind(frame) = source {
        return context.with_original_rebind_v1(frame, |packet| {
            let assigned = read.loan_checked_rebind_v1(frame)?.0;
            if assigned != packet.value() || producer != packet.root_producer() {
                return Err(freeze("expression/rebind-original-producer"));
            }
            Ok(value)
        });
    }
    match (source, producer) {
        (Producer::SignedEntry(formal), ReferenceProducerV1::Entry(entry)) => {
            signed_entry_layout(binding, *formal, context)?;
            read.loan_forwarded_copies(*formal, *entry)?;
        }
        (Producer::SignedEntry(_), _) | (_, ReferenceProducerV1::Entry(_)) => return Err(freeze("expression/scalar-producer-kind")),
        (_, ReferenceProducerV1::Phi(_)) => return Err(freeze("expression/loop-point-source-kind")),
        (original, ReferenceProducerV1::Definition(binding)) => {
            if binding.1.dst_value() != Some(initializer_value(source, read, context)?) { return Err(freeze("expression/scalar-producer-value")); }
            if let Producer::Integer { value: expected, .. } = original {
                if !matches!(binding.1, MirInstruction::Const { value: ConstValue::Integer(actual), .. } if actual == *expected) {
                    return Err(freeze("expression/scalar-literal-producer"));
                }
            }
        }
    }
    Ok(value)
}
fn validate_dependencies(read: &BindingValueObservationV2, producer: &ReferenceProducerV1, function: &MirFunction) -> Result<(), String> {
    let definition = match producer {
        ReferenceProducerV1::Definition(binding) => Some(binding),
        ReferenceProducerV1::Entry(entry) => {
            validate_dependencies_for_entry(*entry, function)?;
            None
        }
        ReferenceProducerV1::Phi(point) => {
            point.identity_phi_binding_v1()?;
            point.verify(function)?;
            None
        }
    };
    for binding in definition.into_iter().chain(read.original_copy_dependencies_v1()?) {
        let dst = binding.1.dst_value().ok_or_else(|| freeze("expression/reference-dependency-dst"))?;
        let mut rows = function.blocks.iter().flat_map(|(block, body)| body.all_instructions().map(move |row| (*block, row)))
            .filter(|(_, row)| row.dst_value() == Some(dst));
        if function.params.contains(&dst) || rows.next() != Some((binding.0, &binding.1)) || rows.next().is_some() {
            return Err(freeze("expression/reference-dependency-identity"));
        }
    }
    Ok(())
}

fn validate_entry_layout(binding: crate::mir::resolved_semantics::BindingRefV1, source: &Producer, producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>, function: &MirFunction,
) -> Result<(), String> {
    let (Producer::SignedEntry(formal), ReferenceProducerV1::Entry(entry)) = (source, producer) else { return Ok(()) };
    let (ordinal, count, receivers, symbol) = signed_entry_layout(binding, *formal, context)?;
    let declarations = &function.metadata.declared_param_decls;
    if function.signature.name != symbol || count != function.params.len()
        || function.params.get(ordinal) != Some(entry) || declarations.len() != count
        || declarations.iter().enumerate().any(|(index, row)| row.implicit_receiver != (receivers == 1 && index == 0))
        || receivers == 1 && function.metadata.physical_param_carriers.as_deref().and_then(|rows| rows.first())
            != Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::ExistingCallableI64) {
        return Err(freeze("expression/scalar-entry-ordinal-value"));
    }
    // Reuse the complete physical entry and exact Copy checks, independently
    // of optional projection permission for ordinary instruction definitions.
    validate_dependencies_for_entry(*entry, function)
}
fn validate_dependencies_for_entry(entry: ValueId, function: &MirFunction) -> Result<(), String> {
    use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
    let carriers = function.metadata.physical_param_carriers.as_deref().ok_or_else(|| freeze("expression/scalar-entry-carriers"))?;
    if function.params.len() != function.signature.params.len() || carriers.len() != function.params.len()
        || function.params.iter().filter(|value| **value == entry).count() != 1
        || function.params.iter().copied().collect::<std::collections::BTreeSet<_>>().len() != function.params.len()
        || function.blocks.values().flat_map(|block| block.all_instructions()).any(|row| row.dst_value().is_some_and(|dst| function.params.contains(&dst))) {
        return Err(freeze("expression/scalar-entry-physical"));
    }
    let index = function.params.iter().position(|value| *value == entry).unwrap();
    if function.metadata.declared_param_decls.get(index).is_none_or(|row| row.implicit_receiver)
        || carriers[index] != Carrier::ExistingCallableI64 || function.signature.params[index] != crate::mir::MirType::Integer {
        return Err(freeze("expression/scalar-entry-carrier"));
    }
    Ok(())
}


/// The original IfForm observation and its sole indexed Phi. This belongs to
/// the same value purpose; it is not another CFG writer or source registry.
#[derive(Debug)]
pub(in crate::mir) struct BranchScalarPhiObservationV1 {
    source: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>,
    pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
    branch: Binding,
    exits: [BasicBlockId; 2],
    jumps: [Binding; 2],
    inputs: [BindingValueObservationV2; 2],
    producers: [ReferenceProducerV1; 2],
    relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
}
type Finished = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit::physical_boundary::FinishedBindings;
impl BranchScalarPhiObservationV1 {
    pub(in crate::mir) fn pending(&self) -> &std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1> { &self.pending }
    pub(in crate::mir) fn source(&self) -> &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1> { &self.source }
    pub(in crate::mir) fn relation(&self) -> &std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1> { &self.relation }
    pub(in crate::mir) fn validate(&self, context: CallValidationContextV1<'_>, function: &MirFunction, finishing: Option<&Finished>) -> Result<(), String> {
        context.ledger().require_branch_scalar_source_v1(&self.source)?;
        let map_block = |block| finishing.and_then(|projection| projection.destination(block)).unwrap_or(block);
        let actual = |row: &Binding| -> Result<Option<Binding>, String> {
            match finishing {
                Some(projection) if row.1.dst_value().is_some() => projection.relation_binding(function, row),
                Some(projection) => projection.captured_effect_binding(function, row).map(Some),
                None => {
                    if function.blocks.get(&row.0).is_none_or(|body| body.all_instructions().filter(|old| **old == row.1).count() != 1) {
                        return Err(freeze("expression/reference-dependency-missing"));
                    }
                    Ok(Some(row.clone()))
                }
            }
        };
        self.relation.require_completed_v1()?;
        if finishing.is_some() {
            self.relation.verify_projected(function, &mut |row| actual(row))?;
            self.pending.relation().verify_projected(function, &mut |row| actual(row))?;
        } else { self.relation.verify(function)?; self.pending.relation().verify(function)?; }
        let branch = actual(&self.branch)?.ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
        let merge = map_block(self.pending.merge());
        let arms = [map_block(self.pending.arms().0), map_block(self.pending.arms().1)];
        let exits = self.exits.map(map_block);
        if branch.0 != map_block(self.pending.block()) || !matches!(branch.1,
            MirInstruction::Branch { condition, then_bb, else_bb, .. }
                if condition == self.pending.values().1 && [then_bb, else_bb] == arms) {
            return Err(freeze("expression/scalar-producer-kind"));
        }
        for (index, jump) in self.jumps.iter().enumerate() {
            let jump = actual(jump)?.ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
            if jump.0 != exits[index] || !matches!(jump.1, MirInstruction::Jump { target, .. } if target == merge)
                || !branch_reaches_v1(function, arms[index], exits[index], merge)
                || branch_reaches_v1(function, arms[index], exits[1-index], merge) {
                return Err(freeze("expression/scalar-producer-kind"));
            }
        }
        let expected: std::collections::BTreeSet<_> = exits.into_iter().collect();
        let actual_preds: std::collections::BTreeSet<_> = crate::mir::verification::utils::compute_predecessors(function)
            .remove(&merge).unwrap_or_default().into_iter().collect();
        if expected.len() != 2 || actual_preds != expected || function.blocks.get(&merge)
            .is_none_or(|body| body.predecessors != expected) {
            return Err(freeze("expression/scalar-producer-kind"));
        }
        let phi = actual(self.relation.identity_phi_binding_v1()?)?.ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
        let values = [self.inputs[0].binding_value().1, self.inputs[1].binding_value().1];
        let mut expected_inputs = [(exits[0], values[0]), (exits[1], values[1])];
        expected_inputs.sort_by_key(|(block, _)| block.0);
        if phi.0 != merge || !matches!(phi.1, MirInstruction::Phi { dst, ref inputs, .. }
            if dst == self.relation.values().0 && inputs.as_slice() == expected_inputs) {
            return Err(freeze("expression/scalar-producer-kind"));
        }
        for (index, (_, _, source)) in self.source.incoming().iter().enumerate() {
            validate_scalar_binding_observation_v1(self.source.binding(), source, &self.inputs[index],
                &self.producers[index], context, function, finishing)?;
        }
        Ok(())
    }
}
fn branch_reaches_v1(function: &MirFunction, entry: BasicBlockId, exit: BasicBlockId, merge: BasicBlockId) -> bool {
    let mut pending = vec![entry];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(block) = pending.pop() {
        if block == merge || !seen.insert(block) { continue; }
        if block == exit { return true; }
        if let Some(body) = function.blocks.get(&block) { pending.extend(body.out_edges().into_iter().map(|edge| edge.target)); }
    }
    false
}
fn validate_scalar_binding_observation_v1(binding: crate::mir::resolved_semantics::BindingRefV1,
    source: &Producer, read: &BindingValueObservationV2, producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>, function: &MirFunction, finishing: Option<&Finished>,
) -> Result<(), String> {
    validate_entry_layout(binding, source, producer, context, function)?;
    validate_scalar_origin(binding, source, read, producer, context)?;
    if matches!(source, Producer::BranchNormal(_)) {
        read.loan_scalar_join_v1(source)?.branch().ok_or_else(|| freeze("expression/scalar-producer-kind"))?
            .validate(context, function, finishing)?;
    }
    if let Some(projection) = finishing {
        if let ReferenceProducerV1::Phi(point) = producer {
            point.require_completed_v1()?;
            point.verify_projected(function, &mut |row| projection.relation_binding(function, row))?;
        }
        if let ReferenceProducerV1::Definition(row) = producer { projection.relation_binding(function, row)?; }
        for row in read.original_copy_dependencies_v1()? { projection.relation_binding(function, row)?; }
    } else { validate_dependencies(read, producer, function)?; }
    Ok(())
}
impl ExpressionReferenceReadV1 {
    pub(in crate::mir) fn prepare_branch_scalar_phi_v1(
        source: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>,
        pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
        branch: Binding, exits: [Option<BasicBlockId>; 2], inputs: [BindingValueObservationV2; 2],
        relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
        context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<std::rc::Rc<BranchScalarPhiObservationV1>, String> {
        context.ledger().require_branch_scalar_source_v1(&source)?;
        let [Some(then_exit), Some(else_exit)] = exits else { return Err(freeze("expression/scalar-producer-kind")); };
        let get_jump = |block| function.blocks.get(&block).and_then(|body| body.terminator.clone())
            .map(|row| (block, row)).ok_or_else(|| freeze("expression/reference-dependency-missing"));
        let producers = [prepare_scalar_binding_producer(source.binding(), &source.incoming()[0].2, &inputs[0], context, function)?,
            prepare_scalar_binding_producer(source.binding(), &source.incoming()[1].2, &inputs[1], context, function)?];
        let observed = std::rc::Rc::new(BranchScalarPhiObservationV1 { source, pending, branch, exits: [then_exit, else_exit],
            jumps: [get_jump(then_exit)?, get_jump(else_exit)?], inputs, producers, relation });
        observed.validate(context, function, None)?;
        Ok(observed)
    }
}
