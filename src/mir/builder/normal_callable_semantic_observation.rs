//! Exact source-site observations for one callable semantic lowering state.
//!
//! This module is a child of the state owner so it can inspect the state's
//! private ledgers. It only consumes already issued source/evidence relations;
//! it does not issue a new semantic product or allocate a physical value.

use crate::mir::resolved_semantics::{BindingRefV1, SourceNodeSiteV1};
use crate::mir::ValueId;

use super::CallableSemanticLoweringState;

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn record_completed_ordinary_new_local(
        &self,
        site: &SourceNodeSiteV1,
        completed: &crate::mir::builder::stmts::CompletedLocalStatementV1,
        claims: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<(), String> {
        let bindings = self
            .locals
            .get(site)
            .ok_or_else(|| super::freeze("missing-local-site"))?;
        if bindings.len() != completed.bindings().len() || !self.materialized_locals.contains(site)
        {
            return Err(super::freeze("ordinary-new-before-local-completion"));
        }
        let rows = bindings
            .iter()
            .zip(completed.bindings())
            .map(|(&binding, row)| (binding, row.ordinal(), row.initializer(), row.local()))
            .collect::<Vec<_>>();
        claims.complete_local_installation(self.owner, site, &rows)
    }
    /// The variable-read site behind a selected `new` actual: the actual itself
    /// when the source resolved it as a variable read, else the operand of its
    /// explicit `move`. The choice comes from the source's variable table; no
    /// second attempt is made after a failed observation.
    pub(in crate::mir::builder) fn actual_variable_site<'a>(
        &self,
        actual: &'a SourceNodeSiteV1,
        move_operand: &'a SourceNodeSiteV1,
    ) -> &'a SourceNodeSiteV1 {
        if self.variables.contains_key(actual) {
            actual
        } else {
            move_operand
        }
    }

    pub(in crate::mir::builder) fn observe_variable_site(
        &mut self,
        site: &SourceNodeSiteV1,
        binding: BindingRefV1,
        semantic_value: ValueId,
    ) -> Result<(), String> {
        if self.variables.get(site).copied() != Some(binding)
            || !self.consumed_variables.insert(site.clone())
        {
            return Err(super::freeze("variable-observation-mismatch"));
        }
        if self.values.get(&binding).copied() != Some(semantic_value) {
            return Err(super::freeze("variable-observation-current-mismatch"));
        }
        if self
            .dynamic_origins
            .source()
            .origin_for_binding(binding)
            .is_some()
            && self.dynamic_origins.current_origin(binding, semantic_value) != Some(binding)
        {
            return Err(super::freeze("variable-observation-current-mismatch"));
        }
        Ok(())
    }

}
