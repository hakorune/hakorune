//! Original lexical body fallthrough consumes the shared cleanup order.
use super::*;
use crate::mir::builder::EmittedSourceCleanupV2;
use crate::mir::resolved_semantics::SourceNodeSiteV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit::physical_boundary::FinishedBindings;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn prepare_lexical_body_cleanup(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<Option<SourceCleanupPlanV2<SourceNodeSiteV1>>, String> {
        let Some(end) = self
            .completion_for_owner(owner)
            .and_then(|completion| completion.cleanup().root_flow())
            .and_then(|flow| flow.lexical_body_ends().get(site))
        else {
            return Ok(None);
        };
        let end = end.as_ref().map_err(|issue| {
            format!("[freeze:contract][lexical-end/unavailable] {site:?}: {issue:?}")
        })?;
        if end.scope().owner() != owner {
            return Err(freeze("lexical-end/foreign-scope"));
        }
        let normal: Vec<_> = end.normal_homes().cloned().collect();
        let (full, normal) = if normal.is_empty() {
            (Vec::new(), Vec::new())
        } else {
            super::super::super::source_home_end_selection_v2(
                self,
                &self.local_commits.borrow(),
                end.homes(),
                &normal,
            )?
            .ok_or_else(|| freeze("lexical-end/home-not-installed"))?
        };
        SourceCleanupPlanV2::new(site.clone(), full, normal).map(Some)
    }

    pub(in crate::mir) fn record_lexical_body_cleanup(
        &self,
        owner: FunctionOwnerIdV1,
        emitted: Rc<EmittedSourceCleanupV2<SourceNodeSiteV1>>,
    ) -> Result<(), String> {
        let mut rows = self.lexical_cleanup.borrow_mut();
        let entries = rows.entry(owner).or_default();
        if entries.iter().any(|row| row.plan.exit == emitted.plan.exit) {
            return Err(freeze("lexical-end/duplicate"));
        }
        entries.push(emitted);
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_lexical_body_cleanup(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        let rows = self.lexical_cleanup.borrow();
        let rows = rows.get(&owner).map_or(&[][..], Vec::as_slice);
        if let Some(flow) = self
            .completion_for_owner(owner)
            .and_then(|completion| completion.cleanup().root_flow())
        {
            for (site, end) in flow.lexical_body_ends() {
                if end
                    .as_ref()
                    .is_ok_and(|end| end.normal_homes().next().is_some())
                    && !rows.iter().any(|row| &row.plan.exit == site)
                {
                    return Err(freeze("lexical-end/cleanup-missing"));
                }
            }
        }
        for emitted in rows {
            let expected = self
                .prepare_lexical_body_cleanup(owner, &emitted.plan.exit)?
                .ok_or_else(|| freeze("lexical-end/source-missing"))?;
            if emitted.plan.full != expected.full || emitted.plan.normal != expected.normal {
                return Err(freeze("lexical-end/source-plan-drift"));
            }
            emitted.plan.validate_cleanup_emission(
                emitted,
                CallValidationContextV1::Lowering(self),
                function,
                finishing,
            )?;
        }
        Ok(())
    }
}
