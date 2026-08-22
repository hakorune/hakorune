//! Exact source-site observations for one callable semantic lowering state.
//!
//! This module is a child of the state owner so it can inspect the state's
//! private ledgers. It only consumes already issued source/evidence relations;
//! it does not issue a new semantic product or allocate a physical value.

use crate::mir::resolved_semantics::{BindingRefV1, SourceNodeSiteV1};
use crate::mir::ValueId;

use super::CallableSemanticLoweringState;

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCallableSourceBackedRebindV1 {
    site: SourceNodeSiteV1,
    binding: BindingRefV1,
    result: ValueId,
    consumes_target_read: bool,
}

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn observe_preloop_alias(
        &mut self,
        site: &SourceNodeSiteV1,
        binding: BindingRefV1,
        formal: BindingRefV1,
        value: ValueId,
        ordinal: u32,
    ) -> Result<(), String> {
        let bindings = self
            .locals
            .get(site)
            .cloned()
            .ok_or_else(|| super::freeze("missing-alias-local-site"))?;
        if bindings.as_ref() != [binding]
            || ordinal != 0
            || !self.materialized_locals.insert(site.clone())
        {
            return Err(super::freeze("preloop-alias-shape-mismatch"));
        }
        if self
            .dynamic_origins
            .source()
            .origin_for_binding(binding)
            .is_some()
        {
            self.insert_value(binding, value)?;
            return self
                .dynamic_origins
                .record_alias_local(site, binding, formal, value, ordinal)
                .map_err(|error| error.to_string());
        }
        if self.values.get(&formal).copied() != Some(value) {
            return Err(super::freeze("static-alias-formal-value-mismatch"));
        }
        self.insert_value(binding, value)
    }

    pub(in crate::mir::builder) fn observe_existing_local(
        &mut self,
        site: &SourceNodeSiteV1,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Result<(), String> {
        let bindings = self
            .locals
            .get(site)
            .cloned()
            .ok_or_else(|| super::freeze("missing-existing-local-site"))?;
        if bindings.as_ref() != [binding] || !self.materialized_locals.insert(site.clone()) {
            return Err(super::freeze("existing-local-shape-mismatch"));
        }
        if self
            .dynamic_origins
            .source()
            .origin_for_binding(binding)
            .is_some()
        {
            return Err(super::freeze(
                "dynamic-local-requires-source-alias-observation",
            ));
        }
        self.insert_value(binding, value)
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

    pub(in crate::mir::builder) fn observe_tail_site(
        &mut self,
        site: &SourceNodeSiteV1,
        binding: BindingRefV1,
        semantic_value: ValueId,
    ) -> Result<(), String> {
        if self.variables.get(site).copied() != Some(binding)
            || !self.consumed_variables.insert(site.clone())
        {
            return Err(super::freeze("tail-observation-mismatch"));
        }
        if self.values.get(&binding).copied() != Some(semantic_value) {
            return Err(super::freeze("tail-observation-current-mismatch"));
        }
        if self
            .dynamic_origins
            .source()
            .origin_for_binding(binding)
            .is_some()
            && self.dynamic_origins.current_origin(binding, semantic_value) != Some(binding)
        {
            return Err(super::freeze("tail-observation-current-mismatch"));
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn require_empty_side_observations(&self) -> Result<(), String> {
        if !self.direct_lambda_captures.is_empty()
            || self.brand_constructors.constructor_count() != 0
        {
            return Err(super::freeze("unsupported-side-observation-cohort"));
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn has_dynamic_origin(&self, binding: BindingRefV1) -> bool {
        self.dynamic_origins
            .source()
            .origin_for_binding(binding)
            .is_some()
    }

    pub(in crate::mir::builder) fn prepare_source_backed_static_rebind(
        &self,
        site: &SourceNodeSiteV1,
        expected_binding: BindingRefV1,
        expected_previous: ValueId,
        result: ValueId,
    ) -> Result<PreparedCallableSourceBackedRebindV1, String> {
        let binding = self
            .assignments
            .get(site)
            .copied()
            .ok_or_else(|| super::freeze("missing-assignment-site"))?;
        if binding != expected_binding || self.consumed_assignments.contains(site) {
            return Err(super::freeze("static-rebind-assignment-mismatch"));
        }
        let consumes_target_read = match self.variables.get(site).copied() {
            Some(read_binding)
                if read_binding == binding && !self.consumed_variables.contains(site) =>
            {
                true
            }
            Some(_) => return Err(super::freeze("assignment-target-read-mismatch")),
            None => false,
        };
        if self.values.get(&binding).copied() != Some(expected_previous) {
            return Err(super::freeze("static-rebind-current-mismatch"));
        }
        if result == expected_previous {
            return Err(super::freeze("static-rebind-result-reuses-current"));
        }
        Ok(PreparedCallableSourceBackedRebindV1 {
            site: site.clone(),
            binding,
            result,
            consumes_target_read,
        })
    }

    pub(in crate::mir::builder) fn commit_source_backed_static_rebind(
        &mut self,
        prepared: PreparedCallableSourceBackedRebindV1,
    ) {
        debug_assert_ne!(self.values.get(&prepared.binding), Some(&prepared.result));
        let inserted_assignment = self.consumed_assignments.insert(prepared.site.clone());
        debug_assert!(inserted_assignment);
        if prepared.consumes_target_read {
            let inserted_read = self.consumed_variables.insert(prepared.site.clone());
            debug_assert!(inserted_read);
        }
        self.values.insert(prepared.binding, prepared.result);
    }
}
