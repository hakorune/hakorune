//! Static packet actuals borrow the same complete checked incoming owner.
//! Publication proves the result; it does not replace entry or actual proof.
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    /// Only a cohort with an original CurrentOwner zero-input source selects
    /// the new protocol. Selection is independent of readiness and Completion.
    fn is_static_zero_packet_cohort_v1(&self, original: &StaticIncomingSourceV1) -> bool {
        original.is_zeroarg_i64_v1()
            && self.borrowed_formal_source.as_ref().is_some_and(|source| {
                source.as_ref().is_ok_and(|source| {
                    source
                        .source_incoming
                        .static_observations()
                        .values()
                        .any(|row| {
                            row.as_ref().is_ok_and(|seed| {
                                seed.current_owner_source().is_some()
                                    && seed.is_zeroarg_i64_v1()
                                    && seed.callee_owner() == original.callee_owner()
                                    && seed.target() == original.target()
                                    && seed.target_batch_slot() == original.target_batch_slot()
                            })
                        })
                })
            })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn select_static_zero_local_routes_v1(
        &mut self,
    ) {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return;
        };
        let sites: Vec<_> = source
            .source_incoming
            .static_observations()
            .values()
            .filter_map(|row| row.as_ref().ok())
            .filter(|original| self.is_static_zero_packet_cohort_v1(original))
            .map(|original| original.call_site().clone())
            .collect();
        if let Some(Ok(selected)) = self.borrowed_static_source_sites.as_mut() {
            selected.extend(sites);
        }
    }

    /// Final borrowed incoming or the bounded original zero-input cohort
    /// selects this protocol. Membership alone grants no executable packet.
    pub(crate) fn selected_static_local_source_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<Rc<StaticIncomingSourceV1>>, String> {
        let Some(selection) = self.borrowed_static_source_sites.as_ref() else {
            return Ok(None);
        };
        if !selection.as_ref().map_err(Clone::clone)?.contains(site) {
            return Ok(None);
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-static/selected-source-missing"))?;
        let source = source.as_ref().map_err(Clone::clone)?;
        if let Some(original) = source.target_static.values().flat_map(|cohort| cohort.incoming.iter())
            .find_map(|row| match &row.source {
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                    if &row.call == site => Some(original),
                _ => None,
            })
        {
            self.checked_completed_static_one_actuals_v1(original)?
                .ok_or_else(|| freeze("borrowed-static/local-actuals-missing"))?;
            self.verify_original_static_packet_source_v1(original)?;
            self.borrowed_static_packet_actuals_v1(original)?
                .ok_or_else(|| freeze("borrowed-static/local-actuals-missing"))?;
            self.lexical_i64_call_source(site)
                .ok_or_else(|| freeze("borrowed-static/local-completion-missing"))?;
            return Ok(Some(Rc::clone(original)));
        }
        if let Some(original) = source.source_incoming.static_observations().get(site) {
            let original = original.as_ref().map_err(Clone::clone)?;
            if self.is_static_zero_packet_cohort_v1(original) {
                self.verify_original_static_packet_source_v1(original)?;
                self.borrowed_static_packet_actuals_v1(original)?
                    .ok_or_else(|| freeze("borrowed-static/local-actuals-missing"))?;
                self.lexical_i64_call_source(site)
                    .ok_or_else(|| freeze("borrowed-static/local-completion-missing"))?;
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
        if !original.is_qualified() {
            self.checked_completed_static_one_actuals_v1(original)?
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
    /// Validate ready cohorts before marking. Unready zero-input cohorts stay
    /// source-only; their selected demand still refuses at emission.
    pub(in crate::mir::normal_callable_semantic_package) fn co_seal_static_local_routes_v1(
        &self,
    ) -> Result<(), String> {
        let Some(source) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let Ok(source) = source.as_ref() else {
            return Ok(());
        };
        let mut routed = Vec::new();
        for row in &source.incoming {
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &row.source else { continue };
            if self.is_static_zero_packet_cohort_v1(original)
                || (!original.is_qualified()
                    && self.checked_completed_static_one_actuals_v1(original)?.is_none())
            {
                continue;
            }
            self.selected_static_local_source_v1(&row.call)?
                .ok_or_else(|| freeze("borrowed-static/local-route-source-missing"))?;
            routed.push(row.call.clone());
        }
        for cohort in source.target_static.values() {
            for row in &cohort.incoming {
                self.selected_static_local_source_v1(&row.call)?
                    .ok_or_else(|| freeze("borrowed-static/local-route-source-missing"))?;
                routed.push(row.call.clone());
            }
        }
        // Source-only CurrentOwner packets are not co-sealed. Their canonical
        // demand remains selected: emission will preserve the original refusal
        // before taking publication instead of retrying the generic call.
        for row in source.source_incoming.exact_rows() {
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &row.source
            else {
                continue;
            };
            if self.is_static_zero_packet_cohort_v1(original)
                && self
                    .selected_static_local_source_v1(&row.call)
                    .is_ok_and(|row| row.is_some())
            {
                routed.push(row.call.clone());
            }
        }
        // The selected arity-one CurrentOwner loop owns a separate checked
        // packet source. Only its successfully co-sealed original call joins
        // the same source-ordered lifecycle group; general SourceStatic stays
        // closed for opaque actuals.
        for packet in self.loop_static_packet_sources.borrow().values() {
            if let Ok(packet) = packet {
                routed.push(packet.local_call_site().clone());
            }
        }
        for site in routed {
            self.record_lifecycle_local_call_site(site.owner(), site);
        }
        Ok(())
    }

    /// Select the existing ordinary root route only from original final
    /// Static incoming membership and the already co-sealed lifecycle sites.
    pub(crate) fn has_routed_static_local_for_owner_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        let Some(source) = self.borrowed_formal_source.as_ref() else {
            return Ok(false);
        };
        let Ok(source) = source.as_ref() else {
            return Ok(false);
        };
        let routed = self.lifecycle_local_call_sites.borrow();
        let mut found = false;
        for row in source
            .incoming
            .iter()
            .filter(|row| row.call.owner() == owner)
        {
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &row.source else { continue };
            if self.is_static_zero_packet_cohort_v1(original)
                || (!original.is_qualified()
                    && self.checked_completed_static_one_actuals_v1(original)?.is_none())
            {
                continue;
            }
            self.selected_static_local_source_v1(&row.call)?
                .ok_or_else(|| freeze("borrowed-static/local-route-source-missing"))?;
            if !routed
                .get(&owner)
                .is_some_and(|sites| sites.contains(&row.call))
            {
                return Err(freeze("borrowed-static/local-route-not-sealed"));
            }
            found = true;
        }
        for cohort in source.target_static.values() {
            for row in cohort.incoming.iter().filter(|row| row.call.owner() == owner) {
                self.selected_static_local_source_v1(&row.call)?
                    .ok_or_else(|| freeze("borrowed-static/local-route-source-missing"))?;
                if !routed.get(&owner).is_some_and(|sites| sites.contains(&row.call)) {
                    return Err(freeze("borrowed-static/local-route-not-sealed"));
                }
                found = true;
            }
        }
        for row in source
            .source_incoming
            .exact_rows()
            .filter(|row| row.call.owner() == owner)
        {
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &row.source
            else {
                continue;
            };
            if self.is_static_zero_packet_cohort_v1(original)
                && routed
                    .get(&owner)
                    .is_some_and(|sites| sites.contains(&row.call))
            {
                self.selected_static_local_source_v1(&row.call)?
                    .ok_or_else(|| freeze("borrowed-static/local-route-source-missing"))?;
                found = true;
            }
        }
        Ok(found)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn verify_original_static_packet_source_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<(), String> {
        if !original.is_zeroarg_i64_v1() && !original.is_qualified() {
            self.checked_completed_static_one_actuals_v1(original)?
                .ok_or_else(|| freeze("ordinary-new/borrowed-entry/source-only-static-actuals"))?;
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-static/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let retained = source
            .source_incoming
            .static_observations()
            .get(original.call_site())
            .ok_or_else(|| freeze("borrowed-static/inventory-site"))?
            .as_ref()
            .map_err(Clone::clone)?;
        if !Rc::ptr_eq(retained, original) {
            return Err(freeze("borrowed-static/original-source-drift"));
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_static_packet_actuals_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        if original.is_current_owner_i64_source_v1() && original.argument_sites().len() == 1 {
            let row = self.checked_completed_static_one_actuals_v1(original)?
                .ok_or_else(|| freeze("ordinary-new/borrowed-entry/source-only-static-actuals"))?;
            return Ok(Some(row.opaque_actuals.as_ref()));
        }
        if self.is_static_zero_packet_cohort_v1(original) {
            let Some(arguments) = self.checked_completed_static_zero_arguments_v1(original)? else {
                return Ok(None);
            };
            let observation = self
                .local_call_for_owner(original.call_site().owner(), original.call_site().site())
                .ok_or_else(|| freeze("borrowed-static/local-source-missing"))?;
            if observation.arguments() != arguments {
                return Err(freeze("borrowed-static/ordered-arguments-drift"));
            }
            let rows = self
                .borrowed_formal_actuals
                .get(original.call_site())
                .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            return Ok(Some(rows.opaque_actuals.as_ref()));
        }
        original.require_qualified()?;
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-static/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let mut incoming = source
            .incoming
            .iter()
            .filter(|call| &call.call == original.call_site());
        let Some(call) = incoming.next() else {
            return Err(freeze("borrowed-static/incoming-missing"));
        };
        if incoming.next().is_some()
            || call.callee != original.callee_owner()
            || !matches!(&call.source,
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(retained)
                    if Rc::ptr_eq(retained, original))
            || original.argument_sites().len() != original.target().arity() as usize
            || call.arguments.iter().any(|(ordinal, site, formal)| {
                original.argument_sites().get(*ordinal as usize) != Some(site)
                    || original
                        .parameters()
                        .get(*ordinal as usize)
                        .is_none_or(|parameter| parameter.binding != *formal)
            })
        {
            return Err(freeze("borrowed-static/packet-identity"));
        }
        if !self.completion_index.get(&call.callee).is_some_and(|row| {
            row.as_ref()
                .is_ok_and(|completion| completion.owner() == call.callee)
        }) {
            return Err(freeze("borrowed-static/callee-completion-missing"));
        }
        self.checked_borrowed_entry_incoming(source, call.callee)?;
        let rows = self
            .borrowed_formal_actuals
            .get(&call.call)
            .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        rows.require_executable_v1()?;
        let arguments = rows.ordered_arguments_for_v1(call)?;
        let observation = self
            .local_call_for_owner(call.call.owner(), call.call.site())
            .ok_or_else(|| freeze("borrowed-static/local-source-missing"))?;
        if observation.arguments() != arguments {
            return Err(freeze("borrowed-static/ordered-arguments-drift"));
        }
        Ok(Some(rows.opaque_actuals.as_ref()))
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_packet_entry_tests.rs"]
mod tests;
