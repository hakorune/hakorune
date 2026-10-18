//! One physical read observation; source proof stays at its original frame node.
use super::*;
use crate::mir::builder::ExactLexicalReadV1;
use crate::mir::resolved_semantics::home_new_prefix::{CurrentScalarReadV1, CurrentScalarProducerV1 as Producer};
use crate::mir::{ValueId, ConstValue};
use crate::mir::instruction::StringInvokeOperand as Operand;
use crate::mir::resolved_semantics::home_new_prefix::{CheckedExpressionEvaluationV1, CheckedExpressionReferenceV1 as Reference};

#[derive(Debug)]
enum ReferenceProducerV1 { Definition(Binding), Entry(ValueId) }
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
    /// Prepare before the existing exact-read batch commits. Source proof stays
    /// in the shared evaluation, not in another reference/operand registry.
    pub(in crate::mir) fn prepare(
        frame: &CheckedExpressionEvaluationV1, node: usize, read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<PreparedExpressionReferenceReadV1, String> {
        let source = original_reference(frame, node)?;
        let producer = match source {
            Reference::Scalar(source) => return Self::prepare_scalar(
                source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?, read, context, function),
            Reference::StaticText(original) => {
                let value = read.loan_local_initializer_v1(original.definition_binding())?.0;
                let mut rows = function.blocks.iter().flat_map(|(block, body)| body.all_instructions()
                    .map(move |row| (*block, row))).filter(|(_, row)| row.dst_value() == Some(value));
                let (block, row) = rows.next().ok_or_else(|| freeze("expression/static-initializer-missing"))?;
                if rows.next().is_some() { return Err(freeze("expression/static-initializer-producer")); }
                ReferenceProducerV1::Definition((block, row.clone()))
            }
            Reference::Residence(original) => {
                let binding = context.with_original_initializer_v1(original.acquisition(), |packet| {
                    context.ledger().validate_text_initializer_home_v1(packet, context)?;
                    Ok(packet.root_acquisition_binding_v1()?.clone())
                })?;
                ReferenceProducerV1::Definition(binding)
            }
            _ => return Err(freeze("expression/reference-execution-pending")),
        };
        reference_operand(frame, node, read, &producer, context)?;
        validate_dependencies(read, &producer, function)?;
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
            validate_entry_layout(source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?,
                &self.producer, context, function)?;
        }
        Ok(())
    }
    pub(in crate::mir) fn definition(&self) -> Option<&Binding> {
        match &self.producer { ReferenceProducerV1::Definition(binding) => Some(binding), _ => None }
    }
    pub(super) fn dependencies(&self) -> Result<impl Iterator<Item = &Binding>, String> {
        Ok(self.definition().into_iter().chain(self.read.original_copy_dependencies_v1()?))
    }
    fn prepare_scalar(
        source: &CurrentScalarReadV1, read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<PreparedExpressionReferenceReadV1, String> {
        let producer = match source.producer() {
            Producer::SignedEntry(formal) => {
                let ordinal = signed_entry_ordinal(source, *formal, context)?;
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
        validate_entry_layout(source, &producer, context, function)?;
        validate_source(source, read, &producer, context)?;
        validate_dependencies(read, &producer, function)?;
        Ok(PreparedExpressionReferenceReadV1 { producer })
    }

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
            source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?, read, producer, context)?)),
        Reference::Residence(original) => {
            if frame.prior_homes().iter().filter(|binding| **binding == original.root()).count() != 1
                || original.root() != original.acquisition().binding() {
                return Err(freeze("expression/residence-current-home"));
            }
            context.with_original_initializer_v1(original.acquisition(), |packet| {
                context.ledger().validate_text_initializer_home_v1(packet, context)?;
                let value = read.loan_local_initializer_v1(original.root())?.0;
                if value != packet.value() || packet.local_value() != value
                    || !matches!(producer, ReferenceProducerV1::Definition(binding) if binding == packet.root_acquisition_binding_v1()?) {
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
fn signed_entry_ordinal(source: &CurrentScalarReadV1, formal: crate::mir::resolved_semantics::BindingRefV1,
    context: CallValidationContextV1<'_>,
) -> Result<usize, String> {
    use crate::mir::callable_parameter_contract::{CallableParameterDeclarationModeV1 as Mode, CallableParameterContractKindV1 as Kind};
    let catalog = context.ledger().original_parameter_contracts.as_deref().ok_or_else(|| freeze("expression/scalar-entry-catalog"))?;
    let mut declarations = catalog.iter().filter(|row| row.owner == source.binding().owner());
    let declaration = declarations.next().ok_or_else(|| freeze("expression/scalar-entry-owner"))?;
    if declarations.next().is_some() || declaration.mode != Mode::StaticBoxMethod { return Err(freeze("expression/scalar-entry-layout-pending")); }
    let mut rows = declaration.parameters.iter().filter(|row| row.binding == formal);
    let row = rows.next().ok_or_else(|| freeze("expression/scalar-entry-formal"))?;
    if rows.next().is_some() || declaration.parameters.get(row.ordinal as usize).is_none_or(|original| original.binding != formal)
        || !matches!(row.kind, Kind::ExactTrivial(abi) if abi.is_i64()) { return Err(freeze("expression/scalar-entry-contract")); }
    Ok(row.ordinal as usize)
}
fn initializer_value(source: &CurrentScalarReadV1, read: &ExactLexicalReadV1, context: CallValidationContextV1<'_>) -> Result<ValueId, String> {
    let (binding, expected) = match source.producer() {
        Producer::Integer { binding, .. } => (*binding, None),
        Producer::Call(call) => {
            if context.ledger().lexical_i64_call_source(call.site()) != Some(call) { return Err(freeze("expression/scalar-call-source")); }
            let binding = call.local_binding().ok_or_else(|| freeze("expression/scalar-call-local"))?.1;
            let value = context.original_scalar_call_result_v1(call)?;
            (binding, Some(value))
        }
        Producer::Initializer(frame) => (frame.binding(), Some(context.with_original_initializer_v1(frame, |packet| Ok(packet.value()))?)),
        Producer::SignedEntry(_) => return Err(freeze("expression/scalar-entry-not-initializer")),
    };
    let value = read.loan_local_initializer_v1(binding)?.0;
    if expected.is_some_and(|expected| expected != value) { return Err(freeze("expression/scalar-original-result")); }
    Ok(value)
}
fn validate_source(source: &CurrentScalarReadV1, read: &ExactLexicalReadV1, producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
) -> Result<ValueId, String> {
    let value = read.value_for(source.binding().owner(), source.read().node(), source.binding()).map_err(|error| error.to_string())?;
    match (source.producer(), producer) {
        (Producer::SignedEntry(formal), ReferenceProducerV1::Entry(entry)) => {
            signed_entry_ordinal(source, *formal, context)?;
            read.loan_forwarded_copies(*formal, *entry)?;
        }
        (Producer::SignedEntry(_), _) | (_, ReferenceProducerV1::Entry(_)) => return Err(freeze("expression/scalar-producer-kind")),
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
fn validate_dependencies(read: &ExactLexicalReadV1, producer: &ReferenceProducerV1, function: &MirFunction) -> Result<(), String> {
    let definition = match producer {
        ReferenceProducerV1::Definition(binding) => Some(binding),
        ReferenceProducerV1::Entry(entry) => {
            validate_dependencies_for_entry(*entry, function)?;
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

fn validate_entry_layout(source: &CurrentScalarReadV1, producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>, function: &MirFunction,
) -> Result<(), String> {
    let (Producer::SignedEntry(formal), ReferenceProducerV1::Entry(entry)) = (source.producer(), producer) else { return Ok(()) };
    let ordinal = signed_entry_ordinal(source, *formal, context)?;
    let declaration = context.ledger().original_parameter_contracts.as_deref().unwrap().iter()
        .find(|row| row.owner == source.binding().owner()).unwrap();
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
