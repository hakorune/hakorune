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
    read: ExactLexicalReadV1,
    producer: ReferenceProducerV1,
}
pub(in crate::mir) struct PreparedExpressionReferenceReadV1 { producer: ReferenceProducerV1 }
impl PreparedExpressionReferenceReadV1 {
    pub(in crate::mir) fn finish(self, read: ExactLexicalReadV1) -> ExpressionReferenceReadV1 {
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
            Reference::Scalar(source) => return Self::prepare_scalar(
                frame, source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?, read, context, function),
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
            _ => return Err(freeze("expression/reference-execution-pending")),
        };
        reference_operand(frame, node, read, &producer, context)?;
        validate_dependencies(read.binding_observation_v2(), &producer, function)?;
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
                validate_source(frame, source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?,
                    &self.read, &self.producer, context)?;
            }
            let original = source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?;
            validate_entry_layout(original.binding(), original.producer(), &self.producer, context, function)?;
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
    fn prepare_scalar(
        frame: &CheckedExpressionEvaluationV1, source: &CurrentScalarReadV1, read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<PreparedExpressionReferenceReadV1, String> {
        let producer = prepare_scalar_binding_producer(source.binding(), source.producer(),
            read.binding_observation_v2(), context, function)?;
        validate_source(frame, source, read, &producer, context)?;
        Ok(PreparedExpressionReferenceReadV1 { producer })
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
            Producer::LoopHeader(_) | Producer::LoopAfter { .. } =>
                ReferenceProducerV1::Phi(std::rc::Rc::clone(read.loan_loop_point_v1(source)?.relation())),
            Producer::BindingRebind(frame) => context.with_original_rebind_v1(frame, |packet| {
                let value = read.loan_checked_rebind_v1(frame)?.0;
                if packet.value() != value { return Err(freeze("expression/rebind-assigned-value")); }
                Ok(packet.root_producer().clone())
            })?,
            Producer::SignedEntry(formal) => {
                let ordinal = signed_entry_ordinal(binding, *formal, context)?;
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
        validate_entry_layout(binding, source, &producer, context, function)?;
        validate_scalar_origin(binding, source, read, &producer, context)?;
        validate_dependencies(read, &producer, function)?;
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
            frame, source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?, read, producer, context)?)),
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
        _ => Err(freeze("expression/reference-execution-pending")),
    }
}
fn signed_entry_ordinal(binding: crate::mir::resolved_semantics::BindingRefV1, formal: crate::mir::resolved_semantics::BindingRefV1,
    context: CallValidationContextV1<'_>,
) -> Result<usize, String> {
    use crate::mir::callable_parameter_contract::{CallableParameterDeclarationModeV1 as Mode, CallableParameterContractKindV1 as Kind};
    let catalog = context.ledger().original_parameter_contracts.as_deref().ok_or_else(|| freeze("expression/scalar-entry-catalog"))?;
    let mut declarations = catalog.iter().filter(|row| row.owner == binding.owner());
    let declaration = declarations.next().ok_or_else(|| freeze("expression/scalar-entry-owner"))?;
    if declarations.next().is_some() || declaration.mode != Mode::StaticBoxMethod { return Err(freeze("expression/scalar-entry-layout-pending")); }
    let mut rows = declaration.parameters.iter().filter(|row| row.binding == formal);
    let row = rows.next().ok_or_else(|| freeze("expression/scalar-entry-formal"))?;
    if rows.next().is_some() || declaration.parameters.get(row.ordinal as usize).is_none_or(|original| original.binding != formal)
        || !matches!(row.kind, Kind::ExactTrivial(abi) if abi.is_i64()) { return Err(freeze("expression/scalar-entry-contract")); }
    Ok(row.ordinal as usize)
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
        Producer::BindingRebind(_) => return Err(freeze("expression/scalar-rebind-not-initializer")),
        Producer::LoopHeader(_) => return Err(freeze("expression/loop-header-physical-pending")),
        Producer::LoopAfter { .. } => return Err(freeze("expression/loop-after-physical-pending")),
    };
    let value = read.loan_local_initializer_v1(binding)?.0;
    if expected.is_some_and(|expected| expected != value) { return Err(freeze("expression/scalar-original-result")); }
    Ok(value)
}
fn validate_source(frame: &CheckedExpressionEvaluationV1, source: &CurrentScalarReadV1, read: &ExactLexicalReadV1, producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    read.value_for(source.binding().owner(), source.read().node(), source.binding()).map_err(|error| error.to_string())?;
    if matches!(source.producer(), Producer::LoopHeader(_)) {
        let product = context.source_loop_point_product_v2(source.producer())?;
        if !product.owns_expression_v2(frame.source()) { return Err(freeze("expression/loop-header-source-scope")); }
    }
    validate_scalar_origin(source.binding(), source.producer(), read.binding_observation_v2(), producer, context)
}
fn validate_scalar_origin(binding: crate::mir::resolved_semantics::BindingRefV1,
    source: &Producer, read: &BindingValueObservationV2, producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    let value = read.value_for(binding.owner(), binding).map_err(|error| error.to_string())?;
    if matches!(source, Producer::LoopHeader(_) | Producer::LoopAfter { .. }) {
        let ReferenceProducerV1::Phi(point) = producer else { return Err(freeze("expression/loop-point-producer-kind")); };
        if !std::rc::Rc::ptr_eq(read.loan_loop_point_v1(source)?.relation(), point) {
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
            signed_entry_ordinal(binding, *formal, context)?;
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
    let ordinal = signed_entry_ordinal(binding, *formal, context)?;
    let declaration = context.ledger().original_parameter_contracts.as_deref().unwrap().iter()
        .find(|row| row.owner == binding.owner()).unwrap();
    if declaration.parameters.len() != function.params.len() || function.params.get(ordinal) != Some(entry) {
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
        || function.metadata.declared_param_decls.iter().any(|row| row.implicit_receiver)
        || function.blocks.values().flat_map(|block| block.all_instructions()).any(|row| row.dst_value().is_some_and(|dst| function.params.contains(&dst))) {
        return Err(freeze("expression/scalar-entry-physical"));
    }
    let index = function.params.iter().position(|value| *value == entry).unwrap();
    if carriers[index] != Carrier::ExistingCallableI64 || function.signature.params[index] != crate::mir::MirType::Integer {
        return Err(freeze("expression/scalar-entry-carrier"));
    }
    Ok(())
}
