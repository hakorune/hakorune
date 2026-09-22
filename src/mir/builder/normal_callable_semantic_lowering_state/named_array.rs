//! Exact source-owned Array emission accounting, without method-name selection.
use super::*;
use crate::mir::resolved_semantics::SourceExprSiteV1;

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn finish(self) -> Result<(), String> {
        let rows = self.finish_with_named_arrays()?;
        if !rows.is_empty() {
            return Err(freeze("named-array-collector-missing"));
        }
        Ok(())
    }
    pub(in crate::mir::builder) fn finish_with_named_arrays(
        self,
    ) -> Result<
        Vec<crate::mir::normal_callable_semantic_package::EmittedNamedArrayRequirementV1>,
        String,
    > {
        self.construction.finish()?;
        self.dynamic_origins
            .finish()
            .map_err(|error| error.to_string())?;
        if let Some(loop_break_source) = self.loop_break_source.as_ref() {
            loop_break_source.finish_empty()?;
        }
        let loop_break_transport_kind = self
            .loop_break_source
            .as_ref()
            .map(LoopBreakSourcePackageLoanV1::is_candidate);
        let missing_variables = self
            .variables
            .keys()
            .filter(|site| !self.consumed_variables.contains(*site))
            .collect::<Vec<_>>();
        if !self.entry_installed
            || self.materialized_locals.len() != self.locals.len()
            || self.consumed_variables.len() != self.variables.len()
            || self.consumed_assignments.len() != self.assignments.len()
            || self.consumed_direct_lambdas.len() != self.direct_lambda_captures.len()
            || self.consumed_brand_constructors.len() != self.brand_constructors.constructor_count()
            || !self.source_core_method_calls.is_empty()
            || !self.source_static_result_publications.is_empty()
        {
            return Err(format!(
                "{} owner={:?} entry={} locals={}/{} variables={}/{} missing_variables={:?} assignments={}/{} lambdas={}/{} loop_break_transport_kind={:?}",
                freeze("incomplete-consumption"),
                self.owner,
                self.entry_installed,
                self.materialized_locals.len(),
                self.locals.len(),
                self.consumed_variables.len(),
                self.variables.len(),
                missing_variables,
                self.consumed_assignments.len(),
                self.assignments.len(),
                self.consumed_direct_lambdas.len(),
                self.direct_lambda_captures.len(),
                loop_break_transport_kind,
            ));
        }
        self.named_array_writes
            .into_iter()
            .map(|port| port.take_completed())
            .collect()
    }
    pub(in crate::mir::builder) fn requires_named_array_allocation(
        &self,
        site: &SourceExprSiteV1,
    ) -> bool {
        self.source_core_method_calls.values().any(|row| {
            row.contract()
                .named_array_requirement()
                .is_some_and(|requirement| requirement.construction() == site)
        })
    }

    pub(in crate::mir::builder) fn record_named_array_allocation(
        &mut self,
        site: &SourceExprSiteV1,
        destination: ValueId,
    ) -> Result<(), String> {
        for row in self.source_core_method_calls.values_mut() {
            row.record_named_allocation(self.owner, site, destination)?;
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn take_source_array_push(
        &mut self,
        site: &SourceExprSiteV1,
        method: &str,
        arity: u32,
    ) -> Result<
        Option<(
            ValueId,
            crate::mir::normal_callable_semantic_package::NamedArrayWriteEmissionPortV1,
        )>,
        String,
    > {
        let Some(row) = self.source_core_method_calls.get(site) else {
            if self.consumed_source_core_method_calls.contains(site) {
                return Err(freeze("duplicate-core-method-call-consumption"));
            }
            return Ok(None);
        };
        if row.contract().named_array_requirement().is_none() {
            return Ok(None);
        }
        let contract = row.contract();
        let target = contract.target();
        if contract.owner() != self.owner
            || target.row().arity() != arity
            || arity != 1
            || target.row().row().canonical != method
            || target.row().row().op != crate::mir::core_method_op::CoreMethodOp::ArrayPush
        {
            return Err(freeze("named-array-call-shape"));
        }
        let receiver_site = contract.receiver_site().node().clone();
        let receiver = self.read_variable(&receiver_site)?;
        let row = self
            .source_core_method_calls
            .remove(site)
            .expect("checked source row");
        if !self.consumed_source_core_method_calls.insert(site.clone()) {
            return Err(freeze("duplicate-core-method-call-consumption"));
        }
        let emission = row.into_named_array_emission()?.into_write_port();
        self.named_array_writes.push(emission.clone());
        Ok(Some((receiver, emission)))
    }
}
