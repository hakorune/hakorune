//! Ordered constructor Call progress, sharing the existing affine Call packet
//! and field-discharge owner. No caller Home or provider ValueId is invented.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    EmittedLexicalCallProjectionV1, LexicalInstanceCallDispositionRowV1,
    OrdinaryNewClaimLedgerV1, OwnedFieldChildV1,
};
use crate::mir::{MirBuilder, MirInstruction};

#[derive(Debug)]
pub(super) struct ConstructionCallEmissionV1 {
    packet: EmittedLexicalCallProjectionV1,
    bindings: Vec<(BasicBlockId, MirInstruction)>,
    base: ValueId,
    discharge: BasicBlockId,
    pub(super) discharge_fields: Box<[OwnedFieldChildV1]>,
}

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn has_next_constructor_call_v1(
        &self, site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Result<bool, String> {
        let ConstructionState::Selected { steps, next_step, completed, .. } = &self.construction else { return Ok(false); };
        if !steps.iter().any(|step| matches!(step, ConstructionSourceStepV1::StoredChildCall(call) if call.site() == site)) {
            return Ok(false);
        }
        if *completed || site.owner() != self.owner
            || !matches!(steps.get(*next_step), Some(ConstructionSourceStepV1::StoredChildCall(call)) if call.site() == site) {
            return Err(fault("call-step-order"));
        }
        Ok(true)
    }

    pub(in crate::mir::builder) fn emit_constructor_unit_call_v1(
        &mut self, builder: &mut MirBuilder, ledger: &OrdinaryNewClaimLedgerV1,
        row: LexicalInstanceCallDispositionRowV1,
    ) -> Result<(), String> {
        if !self.has_next_constructor_call_v1(row.call_site())? { return Err(fault("call-unselected")); }
        let loan = ledger.constructor_unit_call_source_v1(&row)?.ok_or_else(|| fault("call-source-missing"))?;
        let (index, source, frame, tail) = match &self.construction {
            ConstructionState::Selected { steps, next_step, frame: Some((frame, tail)), stores, .. } => {
                let Some(ConstructionSourceStepV1::StoredChildCall(source)) = steps.get(*next_step) else { return Err(fault("call-step-kind")); };
                if loan.call() != source || self.receiver != Some(source.receiver_binding())
                    || steps[..*next_step].iter().any(|step| match step {
                        ConstructionSourceStepV1::Store(site) => !stores.get(site.node())
                            .is_some_and(|store| matches!(store.progress, StoreProgress::Emitted { .. })),
                        ConstructionSourceStepV1::StoredChildCall(_) => false,
                    })
                    || !stores.values().any(|store| store.field == source.field()
                        && matches!(store.progress, StoreProgress::Emitted { .. })
                        && matches!(&store.rhs, ConstructionStoreRhsV1::ProviderConstruction { object: Some(child), .. } if child.object() == source.child())) {
                    return Err(fault("call-receiver-not-committed"));
                }
                (*next_step, source.clone(), *frame, *tail)
            }
            _ => return Err(fault("call-frame-or-state")),
        };
        let (binding, _, field, child) = row.source_target().stored_receiver().ok_or_else(|| fault("call-receiver-kind"))?;
        if binding != source.receiver_binding() || field != source.field() || child != source.child() {
            return Err(fault("call-receiver-drift"));
        }
        let base = self.value_for_exact_binding(self.owner, binding).map_err(|error| error.to_string())?;
        let discharge = super::fault_cleanup::emit_discharge(builder, source.fault_discharge(), base, frame, tail)?;
        let (packet, bindings) = crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_constructor_unit(
            builder, self, ledger, row, frame, discharge)?;
        let ConstructionState::Selected { calls, next_step, completed, .. } = &mut self.construction else { return Err(fault("call-state-moved")); };
        if *completed || *next_step != index { return Err(fault("call-step-drift")); }
        calls.push(ConstructionCallEmissionV1 { packet, bindings, base, discharge,
            discharge_fields: source.fault_discharge().into() });
        *next_step += 1;
        Ok(())
    }
}

impl ConstructionCallEmissionV1 {
    pub(super) fn owns_exact_field_read(&self, block: BasicBlockId, instruction: &MirInstruction) -> bool {
        self.packet.stored_receiver_read_v1().is_some_and(|read| read.0 == block && &read.1 == instruction)
    }

    pub(super) fn validate(
        &self, function: &MirFunction, step: &ConstructionSourceStepV1,
        frame: Option<(ValueId, BasicBlockId)>,
    ) -> Result<(BasicBlockId, BasicBlockId), String> {
        let ConstructionSourceStepV1::StoredChildCall(source) = step else { return Err(fault("call-step-kind")); };
        if self.packet.call_site() != source.site() || self.discharge_fields.as_ref() != source.fault_discharge() {
            return Err(fault("call-source-drift"));
        }
        self.packet.validate_recorded(&self.bindings)?;
        for (block, instruction) in &self.bindings {
            if !function.blocks.get(block).is_some_and(|block| block.all_instructions().any(|actual| actual == instruction)) {
                return Err(fault("call-binding-drift"));
            }
        }
        let (origin, MirInstruction::Invoke { normal_landing, fault_landing, fault_frame,
            operation: crate::mir::instruction::InvokeOperation::Call { result: crate::mir::instruction::InvokeCallResultKind::Unit, .. } }) = self.packet.invoke_binding()
            else { return Err(fault("call-invoke-drift")); };
        if !frame.is_some_and(|(id, tail)| id == *fault_frame
            && super::validation::field_discharge_chain(function, self.discharge, &self.discharge_fields, self.base, frame) == Some(tail))
            || *fault_landing != self.discharge {
            return Err(fault("call-discharge-drift"));
        }
        Ok((*origin, *normal_landing))
    }
}

impl RetainedConstructionValidation {
    /// Lend the original constructor Call pool after its complete ordered
    /// store/Call/Normal/Fault validation. No packet or source is reissued.
    pub(in crate::mir) fn visit_constructor_unit_packets_v1(
        &self, key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        function: &MirFunction,
        mut visit: impl FnMut(&EmittedLexicalCallProjectionV1,
            (BasicBlockId, usize)) -> Result<(), String>,
    ) -> Result<(), String> {
        if key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
            || function.signature.name != key.mir_symbol_projection() {
            return Err(fault("unit-call-birth-owner"));
        }
        let ConstructionState::Selected { calls, .. } = &self.construction else { return Ok(()) };
        if calls.is_empty() { return Ok(()); }
        self.validate_artifact_after_compiler_finishing(function)?;
        for call in calls {
            let packet = &call.packet;
            if packet.call_site().owner() != self.owner {
                return Err(fault("unit-call-birth-source"));
            }
            let (block, instruction) = packet.invoke_binding();
            let body = function.blocks.get(block).ok_or_else(|| fault("unit-call-block"))?;
            if body.terminator.as_ref() != Some(instruction)
                || body.all_instructions().filter(|row| *row == instruction).count() != 1 {
                return Err(fault("unit-call-final-coordinate"));
            }
            visit(packet, (*block, body.instructions.len()))?;
        }
        Ok(())
    }
}
