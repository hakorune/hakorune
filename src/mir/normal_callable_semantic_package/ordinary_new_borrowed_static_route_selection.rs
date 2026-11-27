//! Route selection for Static local calls: which original Static sites take the local route,
//! and the co-seal that registers their lifecycle sites. Membership grants no executable packet.
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::StaticLocalSourceSelectionV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::LocalRouteWriterV1;
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    /// Select completed original scalar-result cohorts with their actual source observations.
    /// The incoming inventory stays the sole source; this is route membership.
    pub(in crate::mir::normal_callable_semantic_package) fn select_completed_static_local_routes_v1(
        &mut self,
    ) -> Result<(), String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let mut sites = Vec::new();
        for owner in source.source_only_definitions.keys() {
            if let Some(cohort) = self.checked_completed_static_scalar_cohort_v1(source, *owner)? {
                sites.extend(cohort.iter().filter_map(|row| {
                    self.local_call_for_owner(row.call.owner(), row.call.site())
                        .map(|_| row.call.clone())
                }));
            }
        }
        for (row, legacy) in source.source_incoming.source_rows_with_legacy_v1() {
            if legacy { continue; }
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) = &row.source
                else { continue; };
            if !crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1::owning_ordinary_source_input_cohort_v1(original.parameters()) {
                continue;
            }
            let Some(observation) = self.local_call_for_owner(row.call.owner(), row.call.site())
                else { continue; };
            let same_source = matches!(observation.result_source(),
                Some(crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1::Ordinary(loan))
                    if loan.static_source().is_some_and(|source| Rc::ptr_eq(source, original)));
            if same_source && self.checked_completed_static_expression_input_v1(original)?.is_some() {
                sites.push(row.call.clone());
            }
        }
        if let Some(Ok(StaticLocalSourceSelectionV1::Pending(selected))) = self.borrowed_static_source_sites.as_mut() {
            selected.extend(sites);
        }
        Ok(())
    }

    /// Check each selected source once, after input closure and before any
    /// root or builder consumer. A pending source's refusal stays scoped to
    /// its site until a consumer demands it.
    pub(in crate::mir::normal_callable_semantic_package) fn seal_static_local_sources_v1(&mut self) {
        let Some(Ok(StaticLocalSourceSelectionV1::Pending(sites))) = self.borrowed_static_source_sites.as_ref() else {
            return;
        };
        let sealed = sites.iter().map(|site| (site.clone(), self.check_static_local_source_v1(site))).collect();
        self.borrowed_static_source_sites = Some(Ok(StaticLocalSourceSelectionV1::Sealed { sites: sealed, root_routes: BTreeMap::new() }));
    }

    pub(crate) fn selected_static_local_source_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<Rc<StaticIncomingSourceV1>>, String> {
        match self.borrowed_static_source_sites.as_ref() {
            None => Ok(None),
            Some(Err(error)) => Err(error.clone()),
            Some(Ok(StaticLocalSourceSelectionV1::Sealed { sites, .. })) => {
                sites.get(site).cloned().unwrap_or(Ok(None))
            }
            Some(Ok(StaticLocalSourceSelectionV1::Pending(sites))) if !sites.contains(site) => Ok(None),
            Some(Ok(StaticLocalSourceSelectionV1::Pending(_))) => Err(freeze("borrowed-static/local-route-not-sealed")),
        }
    }

    fn check_static_local_source_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<Rc<StaticIncomingSourceV1>>, String> {
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-static/selected-source-missing"))?;
        let source = source.as_ref().map_err(Clone::clone)?;
        if let Some(original) =
            source
                .target_static
                .values()
                .flat_map(|cohort| cohort.incoming.iter())
                .find_map(|row| match &row.source {
                    super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(
                        original,
                    ) if &row.call == site => Some(original),
                    _ => None,
                })
        {
            self.checked_completed_static_forwarded_actuals_v1(original)?
                .ok_or_else(|| freeze("borrowed-static/local-actuals-missing"))?;
            self.verify_original_static_packet_source_v1(original)?;
            self.borrowed_static_packet_actuals_v1(original)?
                .ok_or_else(|| freeze("borrowed-static/local-actuals-missing"))?;
            self.local_call_for_owner(site.owner(), site.site())
                .ok_or_else(|| freeze("borrowed-static/local-completion-missing"))?;
            return Ok(Some(Rc::clone(original)));
        }
        if let Some(original) =
            super::super::borrowed_formal_actuals::original_static_source_v1(source, site)?
        {
            let completed = self.checked_completed_static_expression_input_v1(original)?;
            if completed.is_some()
                || self.is_static_expression_input_owner_v1(source, original.callee_owner())
            {
                completed.ok_or_else(|| {
                    format!(
                        "{}: call={site:?}; callee={:?}; target={:?}",
                        freeze("static-input/cohort-unfinished"),
                        original.callee_owner(),
                        original.target()
                    )
                })?;
                self.verify_original_static_packet_source_v1(original)?;
                self.preflight_static_input_packet_v1(original)?;
                return Ok(Some(Rc::clone(original)));
            }
            if self
                .checked_static_scalar_packet_actuals_v1(original)?
                .is_some()
            {
                self.verify_original_static_packet_source_v1(original)?;
                return Ok(Some(Rc::clone(original)));
            }
        }
        let mut rows = source.incoming.iter().filter(|row| &row.call == site);
        let Some(row) = rows.next() else {
            return Ok(None);
        };
        let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
            &row.source
        else {
            return Ok(None);
        };
        if original.qualified_scalar_result_v1().is_none() {
            self.checked_completed_static_forwarded_actuals_v1(original)?
                .ok_or_else(|| freeze("ordinary-new/borrowed-entry/source-only-static-actuals"))?;
        }
        if rows.next().is_some() {
            return Err(freeze("borrowed-static/local-incoming-duplicate"));
        }
        self.verify_original_static_packet_source_v1(original)?;
        self.borrowed_static_packet_actuals_v1(original)?
            .ok_or_else(|| freeze("borrowed-static/local-actuals-missing"))?;
        self.lexical_i64_call_source(site)
            .ok_or_else(|| freeze("borrowed-static/local-completion-missing"))?;
        Ok(Some(Rc::clone(original)))
    }

    /// Co-seal the original selected Static local edges before emission.
    /// Validate ready cohorts before marking.
    pub(in crate::mir::normal_callable_semantic_package) fn co_seal_static_local_routes_v1(
        &mut self,
    ) -> Result<(), String> {
        let Some(source) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let Ok(source) = source.as_ref() else {
            return Ok(());
        };
        // A checked source argument is not a finished input cohort. Retain
        // its selected reservation, but leave physical route registration to
        // executable phases. Any eventual demand still validates the cohort.
        let passive_arguments = |original: &Rc<StaticIncomingSourceV1>| -> Result<bool, String> {
            if original.qualified_scalar_result_v1().is_none() || original.is_zeroarg_i64_v1() {
                return Ok(false);
            }
            if !self.borrowed_formal_actuals.get(original.call_site())
                .is_some_and(|row| row.as_ref().is_ok_and(|row| row.has_qualified_static_argument_source_v1())) {
                return Ok(false);
            }
            super::super::borrowed_formal_actuals::project_pending_static_source_arguments_for_route_v1(
                source, &self.borrowed_formal_actuals, original.call_site(), original.required_i64_arguments(), false,
            )?.ok_or_else(|| freeze("borrowed-static/source-unavailable"))?;
            Ok(true)
        };
        let mut root_routes = BTreeMap::new();
        let mut deferred_root_checks = Vec::new();
        let mut routed = std::collections::BTreeSet::new(); // a call can sit in several rows: route it once
        for row in &source.incoming {
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &row.source
            else {
                continue;
            };
            if passive_arguments(original)? {
                deferred_root_checks.push(&row.call);
                continue;
            }
            if original.qualified_scalar_result_v1().is_none()
                && self
                    .checked_completed_static_forwarded_actuals_v1(original)?
                    .is_none()
            {
                continue;
            }
            self.selected_static_local_source_v1(&row.call)?
                .ok_or_else(|| freeze("borrowed-static/local-route-source-missing"))?;
            routed.insert(row.call.clone());
        }
        for cohort in source.target_static.values() {
            for row in &cohort.incoming {
                if let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) = &row.source {
                    if passive_arguments(original)? {
                        deferred_root_checks.push(&row.call);
                        continue;
                    }
                }
                self.selected_static_local_source_v1(&row.call)?
                    .ok_or_else(|| freeze("borrowed-static/local-route-source-missing"))?;
                routed.insert(row.call.clone());
            }
        }
        // Source-only CurrentOwner packets are not co-sealed. Their canonical
        // demand remains selected: emission will preserve the original refusal
        // before taking publication instead of retrying the generic call.
        for (row, legacy) in source.source_incoming.exact_rows().map(|row| (row, true))
            .chain(source.source_incoming.source_rows_with_legacy_v1().filter(|(_, legacy)| !legacy)) {
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &row.source
            else {
                continue;
            };
            let applies = self.is_static_expression_input_owner_v1(source, original.callee_owner());
            if !legacy && !applies {
                continue;
            }
            let completed_input = applies && self.checked_completed_static_expression_input_v1(original)?.is_some();
            let completed = completed_input
                || self
                    .checked_static_scalar_packet_actuals_v1(original)?
                    .is_some();
            if completed && self.selected_static_local_source_v1(&row.call)?.is_some() {
                routed.insert(row.call.clone());
            }
        }
        for site in &routed {
            root_routes.insert(site.owner(), Ok(()));
        }
        // A qualified scalar expression is owned by its argument frame, so
        // selecting the ordinary root does not require a local packet.
        for row in source.source_incoming.source_rows_v1() {
            if let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) = &row.source {
                if original.qualified_scalar_result_v1().is_some()
                    && crate::mir::resolved_value_profile::expression_position::CallExecutorV1::at(row.call.site()).is_argument_frame()
                    && original.parameters().iter().all(|formal| formal.kind
                        == CallableParameterContractKindV1::ExactTrivial(
                            crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64))
                {
                    root_routes.insert(row.call.owner(), Ok(()));
                }
            }
        }
        for site in deferred_root_checks {
            let check = self.selected_static_local_source_v1(site)
                .and_then(|source| source.ok_or_else(|| freeze("borrowed-static/local-route-source-missing")))
                .and_then(|_| {
                    (routed.contains(site) || self.lifecycle_local_call_sites.borrow().is_local(site))
                        .then_some(())
                        .ok_or_else(|| freeze("borrowed-static/local-route-not-sealed"))
                });
            if let Err(error) = check {
                let row = root_routes.entry(site.owner()).or_insert(Ok(()));
                if row.is_ok() { *row = Err(error); }
            }
        }
        let mut routes = self.lifecycle_local_call_sites.borrow().clone();
        for site in routed {
            routes.route_local(site, LocalRouteWriterV1::StaticPacket)?;
        }
        *self.lifecycle_local_call_sites.get_mut() = routes;
        if let Some(Ok(StaticLocalSourceSelectionV1::Sealed { root_routes: target, .. })) = &mut self.borrowed_static_source_sites {
            *target = root_routes;
        }
        Ok(())
    }

    pub(crate) fn has_routed_static_local_for_owner_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        let Some(Ok(StaticLocalSourceSelectionV1::Sealed { root_routes, .. })) = &self.borrowed_static_source_sites else {
            return Ok(false);
        };
        root_routes.get(&owner).cloned().transpose().map(|route| route.is_some())
    }
}
