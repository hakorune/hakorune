use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{BindingRefV1, SourceNodeSiteV1};
use crate::mir::ValueId;

use super::CallableSemanticLoweringState;

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn validate_variable_accum_source_sites(
        &self,
        reads: &[(SourceNodeSiteV1, BindingRefV1)],
        writes: &[(SourceNodeSiteV1, BindingRefV1)],
    ) -> Result<(), String> {
        let mut sites = BTreeSet::new();
        for (site, binding) in reads {
            if !sites.insert(site.clone())
                || binding.owner() != self.owner
                || self
                    .variables
                    .get(site)
                    .copied()
                    .or_else(|| self.assignments.get(site).copied())
                    != Some(*binding)
                || self.consumed_variables.contains(site)
            {
                return Err(
                    "[freeze:contract][callable-loop/variable-accum/read-site-invalid]".to_owned(),
                );
            }
            if !self.values.contains_key(binding) {
                return Err(
                    "[freeze:contract][callable-loop/variable-accum/entry-value-missing]"
                        .to_owned(),
                );
            }
        }
        for (site, binding) in writes {
            if !sites.insert(site.clone())
                || binding.owner() != self.owner
                || self.assignments.get(site).copied() != Some(*binding)
                || self.consumed_assignments.contains(site)
                || self
                    .variables
                    .get(site)
                    .is_some_and(|read| *read != *binding || self.consumed_variables.contains(site))
            {
                return Err(
                    "[freeze:contract][callable-loop/variable-accum/write-site-invalid]".to_owned(),
                );
            }
            if !self.values.contains_key(binding) {
                return Err(
                    "[freeze:contract][callable-loop/variable-accum/entry-value-missing]"
                        .to_owned(),
                );
            }
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn consume_variable_accum_source_sites(
        &mut self,
        reads: &[(SourceNodeSiteV1, BindingRefV1)],
        writes: &[(SourceNodeSiteV1, BindingRefV1)],
        writebacks: &[(BindingRefV1, ValueId)],
    ) -> Result<(), String> {
        self.validate_variable_accum_source_sites(reads, writes)?;

        let mut writeback_values = std::collections::BTreeMap::new();
        for (binding, value) in writebacks {
            if binding.owner() != self.owner || writeback_values.insert(*binding, *value).is_some()
            {
                return Err(
                    "[freeze:contract][callable-loop/variable-accum/writeback-invalid]".to_owned(),
                );
            }
        }
        let expected_writes = writes
            .iter()
            .map(|(_, binding)| *binding)
            .collect::<BTreeSet<_>>();
        if writeback_values.keys().copied().collect::<BTreeSet<_>>() != expected_writes {
            return Err(
                "[freeze:contract][callable-loop/variable-accum/writeback-set-mismatch]".to_owned(),
            );
        }

        for (site, binding) in reads {
            let before = self.values.get(binding).copied().ok_or_else(|| {
                "[freeze:contract][callable-loop/variable-accum/read-value-missing]".to_owned()
            })?;
            let observed = self.read_variable(site)?;
            if observed != before {
                return Err(
                    "[freeze:contract][callable-loop/variable-accum/read-value-drift]".to_owned(),
                );
            }
        }
        for (site, binding) in writes {
            let value = writeback_values.get(binding).copied().ok_or_else(|| {
                "[freeze:contract][callable-loop/variable-accum/writeback-missing]".to_owned()
            })?;
            self.rebind(site, value)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "loop_recipe_accounting_tests.rs"]
mod tests;
