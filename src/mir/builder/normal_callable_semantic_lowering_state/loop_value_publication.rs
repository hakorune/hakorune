//! Loop-carrier value publication and the `values` projection transaction.
//!
//! Carrier header/final phis emit physical `ValueId`s that must reach the
//! binding-store `values` projection — the sole authority `read_variable`
//! resolves through.  These methods own that publication edge plus the
//! snapshot/restore pair speculative branch lanes use to keep the
//! projection transactional with name-map state.

use super::*;

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn publish_source_loop_final_value(
        &mut self,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Result<(), String> {
        if binding.owner() != self.owner {
            return Err(freeze("loop-final-binding-owner-mismatch"));
        }
        let previous = self
            .values
            .get(&binding)
            .copied()
            .ok_or_else(|| freeze("loop-final-binding-before-materialization"))?;
        self.dynamic_origins
            .invalidate_rebind(binding, previous)
            .map_err(|error| error.to_string())?;
        self.values.insert(binding, value);
        Ok(())
    }

    /// Publishes a loop-carrier physical value for the resolver-owned binding
    /// that carries `name`.  Loop producers collect carrier names from the
    /// co-sealed AST; this name index resolves the name to the single
    /// materialized binding so carrier publication and `read_variable` share
    /// `values` as the sole physical authority.  Unknown or ambiguous names
    /// freeze rather than guess a binding.
    pub(in crate::mir::builder) fn publish_source_loop_final_value_named(
        &mut self,
        name: &str,
        value: ValueId,
    ) -> Result<(), String> {
        let mut candidates = self
            .binding_names
            .iter()
            .filter(|(binding, recorded)| {
                recorded.as_ref() == name && self.values.contains_key(*binding)
            })
            .map(|(binding, _)| *binding);
        let Some(binding) = candidates.next() else {
            return Err(freeze("loop-final-name-unknown"));
        };
        if candidates.next().is_some() {
            return Err(freeze("loop-final-name-ambiguous"));
        }
        self.publish_source_loop_final_value(binding, value)
    }

    /// Captures the `values` projection for a speculative branch lane.
    ///
    /// Source-bound `if` lowering produces branch bodies before the
    /// condition and rolls name-map state back between them; the ledger's
    /// `values` projection must follow the same transaction or the
    /// condition would read branch-produced rebinds.  The snapshot covers
    /// `values` only: consumption receipts and `active_origins` are facts
    /// about already-emitted values and stay monotone.
    pub(in crate::mir::builder) fn source_values_snapshot(
        &self,
    ) -> crate::mir::builder::normal_callable_semantic_lowering_state::MaterializedValuesV1 {
        self.values.clone()
    }

    /// Restores the `values` projection captured by `source_values_snapshot`.
    pub(in crate::mir::builder) fn restore_source_values(
        &mut self,
        snapshot: crate::mir::builder::normal_callable_semantic_lowering_state::MaterializedValuesV1,
    ) {
        self.values = snapshot;
    }
}
