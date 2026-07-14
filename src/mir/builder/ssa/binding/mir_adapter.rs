//! Real-MIR adapter for the disconnected Binding SSA algorithm.
//!
//! This box owns only mechanical PHI lifecycle access. It deliberately keeps
//! every provisional and patched PHI fact at `MirType::Unknown`; a future
//! concrete fact refinement requires its own accepted owner and proof.

use super::BindingSsaIrV1;
use crate::mir::builder::emission::phi_lifecycle::{PhiToken, PhiTxn};
use crate::mir::builder::MirBuilder;
use crate::mir::verification::utils::{compute_def_blocks, compute_dominators};
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::collections::BTreeSet;

const ADAPTER_TAG: &str = "canonical_binding_ssa/mir_adapter";

/// Borrowed mechanical bridge from Binding SSA to one real MIR PHI transaction.
pub(in crate::mir::builder) struct MirBindingSsaAdapterV1<'a> {
    builder: &'a mut MirBuilder,
    phis: &'a mut PhiTxn,
}

impl<'a> MirBindingSsaAdapterV1<'a> {
    pub(in crate::mir::builder) fn new(builder: &'a mut MirBuilder, phis: &'a mut PhiTxn) -> Self {
        Self { builder, phis }
    }
}

impl BindingSsaIrV1 for MirBindingSsaAdapterV1<'_> {
    type PhiToken = PhiToken;

    fn define_provisional_phi(
        &mut self,
        block: BasicBlockId,
    ) -> Result<(ValueId, Self::PhiToken), String> {
        let dst = self.builder.next_value_id();
        let token =
            self.phis
                .define_provisional_phi(self.builder, block, dst, "binding-ssa-define")?;
        self.builder
            .type_ctx
            .value_types
            .insert(dst, MirType::Unknown);
        Ok((dst, token))
    }

    fn patch_phi_inputs(
        &mut self,
        token: Self::PhiToken,
        inputs: &[(BasicBlockId, ValueId)],
    ) -> Result<(), String> {
        self.phis
            .patch_phi_inputs(self.builder, token, inputs.to_vec(), "binding-ssa-patch")?;
        self.builder
            .type_ctx
            .value_types
            .insert(token.dst(), MirType::Unknown);
        Ok(())
    }

    fn verify_phi_input(&self, predecessor: BasicBlockId, value: ValueId) -> Result<(), String> {
        let function = self
            .builder
            .scope_ctx
            .current_function
            .as_ref()
            .ok_or_else(|| format!("{ADAPTER_TAG}: no current function"))?;
        if function.get_block(predecessor).is_none() {
            return Err(format!(
                "{ADAPTER_TAG}: missing predecessor block {predecessor}"
            ));
        }
        if !is_reachable_from_entry(function, predecessor) {
            return Err(format!(
                "{ADAPTER_TAG}: predecessor block {predecessor} is unreachable"
            ));
        }
        let definitions = compute_def_blocks(function);
        let definition = definitions
            .get(&value)
            .copied()
            .ok_or_else(|| format!("{ADAPTER_TAG}: value {value} has no MIR definition"))?;
        if !compute_dominators(function).dominates(definition, predecessor) {
            return Err(format!(
                "{ADAPTER_TAG}: value {value} from {definition} does not dominate {predecessor}"
            ));
        }
        Ok(())
    }

    fn rollback_phi(&mut self, token: Self::PhiToken) -> Result<(), String> {
        self.phis
            .rollback_pending_phi(self.builder, token, "binding-ssa-rollback")?;
        self.builder.type_ctx.value_types.remove(&token.dst());
        Ok(())
    }
}

fn is_reachable_from_entry(function: &crate::mir::MirFunction, target: BasicBlockId) -> bool {
    let mut seen = BTreeSet::new();
    let mut pending = vec![function.entry_block];
    while let Some(block) = pending.pop() {
        if !seen.insert(block) {
            continue;
        }
        if block == target {
            return true;
        }
        let Some(block) = function.get_block(block) else {
            continue;
        };
        pending.extend(block.successors_from_terminator());
    }
    false
}
