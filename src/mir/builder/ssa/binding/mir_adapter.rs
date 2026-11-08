//! Real-MIR adapter for the function-owned Binding SSA algorithm.
//!
//! This box owns only mechanical PHI lifecycle access. Provisional PHIs start
//! as `Unknown`; successful patch completion delegates exact type publication
//! to the canonical PHI lifecycle owner.

use super::BindingSsaIrV1;
use crate::mir::builder::emission::phi_lifecycle::{PhiToken, PhiTxn};
use crate::mir::builder::MirBuilder;
use crate::mir::verification::utils::{compute_def_blocks, compute_dominators};
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::cell::OnceCell;
use std::collections::{BTreeSet, HashMap};

const ADAPTER_TAG: &str = "canonical_binding_ssa/mir_adapter";

/// Borrowed mechanical bridge from Binding SSA to one real MIR PHI transaction.
pub(in crate::mir::builder) struct MirBindingSsaAdapterV1<'a> {
    builder: &'a mut MirBuilder,
    phis: &'a mut PhiTxn,
    deferred_reachable_block: Option<BasicBlockId>,
    analysis: OnceCell<PhiInputAnalysisV1>,
}

struct PhiInputAnalysisV1 {
    definitions: HashMap<ValueId, BasicBlockId>,
    dominators: crate::mir::verification::utils::DominatorTree,
    reachable: BTreeSet<BasicBlockId>,
    deferred: BTreeSet<BasicBlockId>,
}

impl<'a> MirBindingSsaAdapterV1<'a> {
    pub(in crate::mir::builder) fn new(builder: &'a mut MirBuilder, phis: &'a mut PhiTxn) -> Self {
        Self {
            builder,
            phis,
            deferred_reachable_block: None,
            analysis: OnceCell::new(),
        }
    }

    /// Recheck all published inputs with this adapter's final reachability and
    /// dominance policy; the function owner does not reconstruct an input list.
    pub(in crate::mir::builder) fn verify_function_phi_inputs(&self) -> Result<(), String> {
        let function = self
            .builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| format!("{ADAPTER_TAG}: no current function"))?;
        for block in function.blocks.values() {
            for instruction in &block.instructions {
                if let crate::mir::MirInstruction::Phi { inputs, .. } = instruction {
                    for (predecessor, value) in inputs {
                        self.verify_phi_input(*predecessor, *value)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn new_with_deferred_reachable(
        builder: &'a mut MirBuilder,
        phis: &'a mut PhiTxn,
        deferred_reachable_block: BasicBlockId,
    ) -> Self {
        Self {
            builder,
            phis,
            deferred_reachable_block: Some(deferred_reachable_block),
            analysis: OnceCell::new(),
        }
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
            .function_state
            .type_ctx
            .value_types
            .insert(dst, MirType::Unknown);
        if let Some(analysis) = self.analysis.get_mut() {
            analysis.definitions.insert(dst, block);
        }
        Ok((dst, token))
    }

    fn patch_phi_inputs(
        &mut self,
        token: Self::PhiToken,
        inputs: &[(BasicBlockId, ValueId)],
    ) -> Result<(), String> {
        self.phis
            .patch_phi_inputs(self.builder, token, inputs.to_vec(), "binding-ssa-patch")?;
        Ok(())
    }

    fn verify_phi_input(&self, predecessor: BasicBlockId, value: ValueId) -> Result<(), String> {
        let function = self
            .builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| format!("{ADAPTER_TAG}: no current function"))?;
        if function.get_block(predecessor).is_none() {
            return Err(format!(
                "{ADAPTER_TAG}: missing predecessor block {predecessor}"
            ));
        }
        // This adapter exclusively borrows the Builder while the SSA kernel
        // can only define, patch or roll back PHIs; none changes CFG edges.
        let analysis = self.analysis.get_or_init(|| PhiInputAnalysisV1 {
            definitions: compute_def_blocks(function),
            dominators: compute_dominators(function),
            reachable: reachable_from(function, function.entry_block),
            deferred: self
                .deferred_reachable_block
                .map(|root| reachable_from(function, root))
                .unwrap_or_default(),
        });
        let reachable_from_deferred = analysis.deferred.contains(&predecessor);
        let reachable_from_entry = analysis.reachable.contains(&predecessor);
        if !reachable_from_entry && !reachable_from_deferred {
            return Err(format!(
                "{ADAPTER_TAG}: predecessor block {predecessor} is unreachable"
            ));
        }
        let definition = analysis
            .definitions
            .get(&value)
            .copied()
            .ok_or_else(|| format!("{ADAPTER_TAG}: value {value} has no MIR definition"))?;
        if !reachable_from_entry && self.deferred_reachable_block.is_some() {
            // The physical E1 -> cursor graph is being co-written in this
            // unpublished session.  The exact deferred-entry adapter still
            // requires a real MIR definition, while canonical CFG sealing
            // below closes the final reachability/dominance witness.
            return Ok(());
        }
        if !analysis.dominators.dominates(definition, predecessor) {
            return Err(format!(
                "{ADAPTER_TAG}: value {value} from {definition} does not dominate {predecessor}"
            ));
        }
        Ok(())
    }

    fn rollback_phi(&mut self, token: Self::PhiToken) -> Result<(), String> {
        self.phis
            .rollback_pending_phi(self.builder, token, "binding-ssa-rollback")?;
        self.builder
            .function_state
            .type_ctx
            .value_types
            .remove(&token.dst());
        if let Some(analysis) = self.analysis.get_mut() {
            analysis.definitions.remove(&token.dst());
        }
        Ok(())
    }
}

fn reachable_from(
    function: &crate::mir::MirFunction,
    root: BasicBlockId,
) -> BTreeSet<BasicBlockId> {
    let mut seen = BTreeSet::new();
    let mut pending = vec![root];
    while let Some(block) = pending.pop() {
        if !seen.insert(block) {
            continue;
        }
        let Some(block) = function.get_block(block) else {
            continue;
        };
        pending.extend(block.successors_from_terminator());
    }
    seen
}
