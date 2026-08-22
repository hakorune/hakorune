//! Assignment lowering and local contract publication.

use super::vars;
use super::{MirBuilder, ValueId};
use crate::mir::MirInstruction;

impl MirBuilder {
    /// Build assignment from an already-evaluated value.
    ///
    /// This is the shared shell used by ordinary lowering and fastmem lowering.
    pub(in crate::mir::builder) fn build_assignment_from_value(
        &mut self,
        var_name: String,
        value_id: ValueId,
    ) -> Result<ValueId, String> {
        vars::assignment_resolver::AssignmentResolverBox::ensure_declared(self, &var_name)?;

        if !var_name.starts_with("__pin$") {
            let local_slot_id = self
                .function_state
                .binding_ctx
                .lookup(&var_name)
                .map(crate::mir::LocalSlotId::from);
            let local_contract = local_slot_id.and_then(|slot| {
                self.function_state
                    .current_function
                    .as_ref()
                    .and_then(|function| {
                        crate::mir::type_contracts::local_slot::local_slot_contract(function, slot)
                            .cloned()
                    })
            });
            let typed_array_spec = local_slot_id.and_then(|slot| {
                self.function_state
                    .current_function
                    .as_ref()
                    .and_then(|function| {
                        crate::mir::type_contracts::typed_array::local_slot_spec(function, slot)
                    })
            });
            if let (Some(local_slot_id), Some(spec)) = (local_slot_id, typed_array_spec) {
                let contract_id = format!(
                    "typed-array:local:{}:reassign:{}",
                    local_slot_id.binding_id().raw(),
                    value_id.as_u32()
                );
                let function = self
                    .function_state
                    .current_function
                    .as_mut()
                    .ok_or_else(|| {
                        "[type/typed_array_contract_carrier_missing] function=<none>".to_string()
                    })?;
                function.metadata.typed_array_contract_sources.push(
                    crate::mir::function::TypedArrayContractSource {
                        contract_id: contract_id.clone(),
                        boundary: crate::mir::function::TypedArrayContractBoundary::LocalReassign,
                        source_identity:
                            crate::mir::function::TypedArrayContractSourceIdentity::LocalSlot(
                                local_slot_id,
                            ),
                        boundary_value: crate::mir::function::TypedArrayBoundaryValue::Value(
                            value_id,
                        ),
                        element_spec: spec,
                    },
                );
                self.emit_instruction(MirInstruction::ArrayStateContractClaim {
                    contract_id,
                    array: value_id,
                })?;
            }
            let published_value =
                if let (Some(local_slot_id), Some(_contract)) = (local_slot_id, local_contract) {
                    let dst = self.next_value_id();
                    self.emit_instruction(MirInstruction::LocalContractWrite {
                        dst,
                        src: value_id,
                        local_slot_id,
                        write_kind: crate::mir::function::LocalContractWriteKind::Reassign,
                    })?;
                    crate::mir::builder::metadata::propagate::propagate(self, value_id, dst);
                    dst
                } else {
                    value_id
                };

            // Release the previous strong reference before updating variable_map.
            if !self.is_current_block_terminated() {
                if let Some(prev) = self
                    .function_state
                    .variable_ctx
                    .variable_map
                    .get(&var_name)
                    .copied()
                {
                    self.emit_instruction(MirInstruction::ReleaseStrong { values: vec![prev] })?;
                }
            }
            self.function_state
                .variable_ctx
                .variable_map
                .insert(var_name.clone(), published_value);
            return Ok(published_value);
        }

        Ok(value_id)
    }

    /// Check if the current basic block is terminated.
    pub(in crate::mir::builder) fn is_current_block_terminated(&self) -> bool {
        if let (Some(block_id), Some(ref function)) = (
            self.function_state.current_block,
            &self.function_state.current_function,
        ) {
            if let Some(block) = function.get_block(block_id) {
                return block.is_terminated();
            }
        }
        false
    }
}
