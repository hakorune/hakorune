//! Borrow the original Birth formal, acquire an independent pair, transfer on
//! store Normal only. Source admission stays on the same construction plan.
use super::*;
use crate::mir::instruction::{FieldSetModeV1 as Mode, InvokeOperation, StringInvokeOperand as Operand, StringInvokeOperation as Op};
use crate::mir::{BasicBlock, MirBuilder, MirInstruction, MirType};
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(super) struct PreparedTextInputV1 {
    sum: Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>,
    ordinal: usize,
}
#[derive(Debug)]
pub(in crate::mir::builder) struct TextStoreAcquisitionV1 {
    input: PreparedTextInputV1,
    pair: [Binding; 2],
}
impl CallableSemanticLoweringState {
    pub(super) fn prepare_construction_text_input_v1(
        &self, taken: &TakenConstructionStore, builder: &MirBuilder,
    ) -> Result<Option<PreparedTextInputV1>, String> {
        let ConstructionState::Selected { plan, stores, parameters, .. } = &self.construction else { return Ok(None) };
        let store = stores.get(&taken.site).ok_or_else(|| fault("emission-site"))?;
        if store.mode != Mode::DynamicTextInitialize { return Ok(None); }
        let (ordinal, source) = plan.stores().iter().enumerate()
            .find(|(_, source)| source.assignment().statement_site().node() == &taken.site)
            .ok_or_else(|| fault("source-or-cleanup-contract"))?;
        let sum = plan.store_execution_v1(ordinal).filter(|execution| execution.mode() == store.mode)
            .and_then(|execution| execution.text_input_v1()).filter(|sum| sum.text_only())
            .ok_or_else(|| fault("source-or-cleanup-contract"))?;
        let ConstructionStoreRhsV1::Parameter { binding, provided: None, .. } = source.rhs()
            else { return Err(fault("source-or-cleanup-contract")) };
        if parameters.as_ref() != self.parameters.as_ref() || source.rhs() != &taken.rhs
            || source.field() != taken.field || source.rhs() != &store.rhs
            || plan.constructor().map(|(_, owner)| *owner) != Some(self.owner)
            || binding.owner() != self.owner {
            return Err(fault("source-or-cleanup-contract"));
        }
        let mut positions = parameters.iter().enumerate().filter(|(_, original)| *original == binding);
        let ordinal = positions.next().map(|(index, _)| index).ok_or_else(|| fault("emission-state"))?;
        if positions.next().is_some() { return Err(fault("emission-state")); }
        let function = builder.function_state.current_function.as_ref().ok_or_else(|| fault("no-function"))?;
        require_parameter_layout(function, parameters.len())?;
        if function.params.get(ordinal + 1).copied() != Some(self.value_for_exact_binding(self.owner, *binding)
            .map_err(|error| error.to_string())?) {
            return Err(fault("emission-state"));
        }
        Ok(Some(PreparedTextInputV1 { sum: Rc::clone(sum), ordinal }))
    }
}
fn require_parameter_layout(function: &MirFunction, count: usize) -> Result<(), String> {
    crate::mir::instance_constructor_abi::InstanceConstructorAbiV1::issue(count)
        .and_then(|abi| abi.validate(count, function.params.len())).map_err(|_| fault("emission-state"))?;
    if function.signature.params.len() != function.params.len()
        || function.metadata.declared_param_decls.len() != function.params.len()
        || !function.metadata.declared_param_decls.first().is_some_and(|row| row.implicit_receiver)
        || function.metadata.declared_param_decls.iter().skip(1).any(|row| row.implicit_receiver)
        || function.metadata.physical_param_carriers.as_ref().is_none_or(|rows| rows.len() != function.params.len()) {
        return Err(fault("emission-state"));
    }
    Ok(())
}
impl PreparedTextInputV1 {
    pub(super) fn materialize_v1(self, builder: &mut MirBuilder, input: ValueId,
        frame: ValueId, discharge: BasicBlockId,
    ) -> Result<TextStoreAcquisitionV1, String> {
        let function = builder.function_state.current_function.as_mut().ok_or_else(|| fault("no-function"))?;
        let lane = self.ordinal + 1; // The existing instance-constructor ABI is receiver then source parameters.
        if function.params.get(lane) != Some(&input) || !self.sum.text_only()
            || !matches!(function.signature.params.get(lane), Some(MirType::Unknown | MirType::Integer | MirType::String)) {
            return Err(fault("source-or-cleanup-contract"));
        }
        function.metadata.physical_param_carriers.as_mut().ok_or_else(|| fault("emission-state"))?[lane] = Carrier::BorrowedTaggedValue;
        function.signature.params[lane] = MirType::Integer;
        builder.function_state.type_ctx.value_types.insert(input, MirType::Integer);
        use crate::mir::builder::ordinary_new_admission::selected::{emit_scalar_operation_to_landing_v1,
            root_graph::{RawRootGraphWriterV1, RootGraphWriterPortV1}};
        let mut writer = RawRootGraphWriterV1;
        let normal = writer.reserve_block(builder)?;
        let (_, _, pair) = emit_scalar_operation_to_landing_v1(builder, &mut writer,
            InvokeOperation::String(Op::Materialize { value: Operand::BorrowedText(input) }),
            frame, normal, discharge, &mut Vec::new())?;
        Ok(TextStoreAcquisitionV1 { input: self, pair })
    }
}
impl TextStoreAcquisitionV1 {
    pub(super) fn value(&self) -> ValueId { self.pair[1].1.dst_value().expect("shared writer returns a NormalResult") }
    pub(super) fn entry(&self) -> BasicBlockId { self.pair[0].0 }
    pub(super) fn is_projection(&self, block: BasicBlockId, row: &MirInstruction) -> bool {
        self.pair[1].0 == block && &self.pair[1].1 == row
    }
    pub(super) fn candidate_fault_v1(&self, builder: &mut MirBuilder,
        frame: ValueId, discharge: BasicBlockId,
    ) -> Result<BasicBlockId, String> {
        let head = builder.next_block_id();
        let normal = super::fault_cleanup::jump_landing(builder, discharge)?;
        let fault_edge = super::fault_cleanup::jump_landing(builder, discharge)?;
        let mut block = BasicBlock::new(head);
        block.set_terminator(MirInstruction::Invoke { operation: InvokeOperation::String(Op::End { value: self.value() }),
            fault_frame: frame, normal_landing: normal, fault_landing: fault_edge });
        builder.function_state.current_function.as_mut().ok_or_else(|| fault("no-function"))?.add_block(block);
        Ok(head)
    }
    fn validate_v1(&self, store: &SelectedConstructionStore, function: &MirFunction,
        parameters: &[crate::mir::resolved_semantics::BindingRefV1], frame: ValueId,
    ) -> Result<(), String> {
        let ConstructionStoreRhsV1::Parameter { binding, provided: None, .. } = &store.rhs
            else { return Err(fault("source-or-cleanup-contract")) };
        if parameters.get(self.input.ordinal) != Some(binding) { return Err(fault("source-or-cleanup-contract")); }
        let StoreProgress::Emitted { block, normal, value, discharge, provider: None, provider_child: None, .. } = store.progress
            else { return Err(fault("emission-drift")) };
        let input = *function.params.get(self.input.ordinal + 1).ok_or_else(|| fault("emission-state"))?;
        if input == value || self.value() != value || self.pair[1].0 != block
            || self.pair[1].1 != (MirInstruction::InvokeNormalResult { invoke_block: self.entry(), dst: value })
            || !function.blocks.get(&block).is_some_and(|body| body.instructions.iter().filter(|row| *row == &self.pair[1].1).count() == 1)
            || function.params.iter().filter(|value| **value == input).count() != 1
            || function.blocks.values().flat_map(|body| body.all_instructions()).any(|row| row.dst_value() == Some(input))
            || self.pair[0].1 != (MirInstruction::Invoke {
                operation: InvokeOperation::String(Op::Materialize { value: Operand::BorrowedText(input) }),
                fault_frame: frame, normal_landing: block, fault_landing: discharge })
            || function.blocks.get(&self.entry()).and_then(|body| body.terminator.as_ref()) != Some(&self.pair[0].1) {
            return Err(fault("emission-drift"));
        }
        let Some(MirInstruction::Invoke { operation: InvokeOperation::FieldSet { mode: Mode::DynamicTextInitialize,
            field, base, value: actual }, fault_frame, normal_landing, fault_landing }) =
            function.blocks.get(&block).and_then(|body| body.terminator.as_ref()) else { return Err(fault("emission-drift")) };
        if *field != store.field || *actual != value || *normal_landing != normal || *fault_frame != frame
            || !matches!(&store.progress, StoreProgress::Emitted { base: expected, .. } if expected == base)
            || !matches!(function.blocks.get(fault_landing).and_then(|body| body.terminator.as_ref()),
                Some(MirInstruction::Invoke { operation: InvokeOperation::String(Op::End { value: released }),
                    fault_frame: same, normal_landing: n, fault_landing: f })
                if *released == value && *same == frame && n != f
                    && super::validation::jump_target(function, *n) == Some(discharge)
                    && super::validation::jump_target(function, *f) == Some(discharge)) {
            return Err(fault("emission-drift"));
        }
        Ok(())
    }
}
impl ConstructionState {
    pub(super) fn validate_text_materializations_v1(&self, function: &MirFunction) -> Result<(), String> {
        let Self::Selected { plan, parameters, stores, frame, .. } = self else { return Ok(()) };
        // Non-Text physical fixtures retain their existing protocol. Only a
        // source or emitted Text obligation demands this original-source join.
        if !plan.stores().iter().enumerate().any(|(index, _)| plan.store_execution_v1(index)
                .is_some_and(|execution| execution.mode() == Mode::DynamicTextInitialize))
            && !stores.values().any(|store| store.mode == Mode::DynamicTextInitialize
                || matches!(&store.progress, StoreProgress::Emitted { text: Some(_), .. })) {
            return Ok(());
        }
        let mut formals = std::collections::BTreeSet::new();
        for (ordinal, source) in plan.stores().iter().enumerate() {
            let store = stores.get(source.assignment().statement_site().node()).ok_or_else(|| fault("emission-site"))?;
            let text = match &store.progress { StoreProgress::Emitted { text, .. } => text.as_ref(), _ => None };
            let execution = plan.store_execution_v1(ordinal).ok_or_else(|| fault("source-or-cleanup-contract"))?;
            if execution.mode() != store.mode { return Err(fault("source-or-cleanup-contract")); }
            match (store.mode, execution.text_input_v1(), text) {
                (Mode::DynamicTextInitialize, Some(sum), Some(text)) if sum.text_only() && Rc::ptr_eq(sum, &text.input.sum) => {
                    let (_, owner) = plan.constructor().ok_or_else(|| fault("source-or-cleanup-contract"))?;
                    if parameters.iter().any(|binding| binding.owner() != *owner)
                        || source.rhs() != &store.rhs || source.field() != store.field {
                        return Err(fault("source-or-cleanup-contract"));
                    }
                    require_parameter_layout(function, parameters.len())?;
                    let (frame, _) = frame.ok_or_else(|| fault("emission-state"))?;
                    text.validate_v1(store, function, parameters, frame)?;
                    formals.insert(text.input.ordinal + 1);
                }
                (Mode::DynamicTextInitialize, _, _) | (_, _, Some(_)) => return Err(fault("source-or-cleanup-contract")),
                _ => (),
            }
        }
        if !formals.is_empty() && function.metadata.physical_param_carriers.as_ref().is_none_or(|rows|
            rows.iter().enumerate().any(|(index, actual)| *actual != if formals.contains(&index) {
                Carrier::BorrowedTaggedValue } else { Carrier::ExistingCallableI64 })) {
            return Err(fault("emission-state"));
        }
        Ok(())
    }
}
