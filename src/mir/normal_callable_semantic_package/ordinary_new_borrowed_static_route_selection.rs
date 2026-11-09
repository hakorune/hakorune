//! Route selection for Static local calls: which original Static sites take the local route,
//! and the co-seal that registers their lifecycle sites. Membership grants no executable packet.
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::LocalRouteWriterV1;
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1;
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    /// The CurrentOwner zero I64 cohort uses the Zero input protocol. Eligibility does not grant readiness or result publication.
    pub(super) fn is_static_zero_packet_cohort_v1(&self, original: &StaticIncomingSourceV1) -> bool {
        original.is_zero_input_source_v1()
            && self.borrowed_formal_source.as_ref().is_some_and(|source| {
                source.as_ref().is_ok_and(|source| source.source_incoming.source_rows_with_legacy_v1().any(|(row, legacy)|
                    matches!(&row.source, super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(seed)
                        if seed.is_zero_input_source_v1()
                            && legacy && seed.current_owner_source().is_some()
                            && seed.callee_owner() == original.callee_owner()
                            && seed.target() == original.target()
                            && seed.target_batch_slot() == original.target_batch_slot())))
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
            .source_rows_with_legacy_v1()
            .filter_map(|(row, legacy)| match &row.source {
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                    if legacy => Some(original),
                _ => None,
            })
            .filter(|original| self.is_static_zero_packet_cohort_v1(original))
            .map(|original| original.call_site().clone())
            .collect();
        if let Some(Ok(selected)) = self.borrowed_static_source_sites.as_mut() {
            selected.extend(sites);
        }
    }

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
        if let Some(Ok(selected)) = self.borrowed_static_source_sites.as_mut() {
            selected.extend(sites);
        }
        Ok(())
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
            self.checked_completed_static_one_actuals_v1(original)?
                .ok_or_else(|| freeze("borrowed-static/local-actuals-missing"))?;
            self.verify_original_static_packet_source_v1(original)?;
            self.borrowed_static_packet_actuals_v1(original)?
                .ok_or_else(|| freeze("borrowed-static/local-actuals-missing"))?;
            self.local_call_for_owner(site.owner(), site.site())
                .ok_or_else(|| freeze("borrowed-static/local-completion-missing"))?;
            return Ok(Some(Rc::clone(original)));
        }
        if let Some(original) = super::super::borrowed_formal_actuals::original_static_source_v1(source, site)? {
            let qualified = self.checked_completed_static_expression_input_v1(original)?;
            if qualified.is_some() || self.is_static_expression_input_owner_v1(source, original.callee_owner()) {
                qualified.ok_or_else(|| format!("{}: call={site:?}; callee={:?}; target={:?}",
                    freeze("static-input/cohort-unfinished"), original.callee_owner(), original.target()))?;
                self.verify_original_static_packet_source_v1(original)?;
                self.borrowed_static_packet_actuals_v1(original)?
                    .ok_or_else(|| freeze("borrowed-static/local-actuals-missing"))?;
                return Ok(Some(Rc::clone(original)));
            }
            if self
                .checked_static_tagged_packet_actuals_v1(original)?
                .is_some()
            {
                self.verify_original_static_packet_source_v1(original)?;
                return Ok(Some(Rc::clone(original)));
            }
            if self
                .checked_static_scalar_packet_actuals_v1(original)?
                .is_some()
            {
                self.verify_original_static_packet_source_v1(original)?;
                return Ok(Some(Rc::clone(original)));
            }
            if self
                .checked_static_mixed_packet_actuals_v1(original)?
                .is_some()
            {
                self.verify_original_static_packet_source_v1(original)?;
                return Ok(Some(Rc::clone(original)));
            }
            if self.is_static_zero_packet_cohort_v1(original) {
                self.verify_original_static_packet_source_v1(original)?;
                self.borrowed_static_packet_actuals_v1(original)?
                    .ok_or_else(|| freeze("borrowed-static/local-actuals-missing"))?;
                if original.is_zeroarg_i64_v1() {
                    self.lexical_i64_call_source(site)
                        .ok_or_else(|| freeze("borrowed-static/local-completion-missing"))?;
                } else {
                    self.local_call_for_owner(site.owner(), site.site())
                        .ok_or_else(|| freeze("borrowed-static/local-completion-missing"))?;
                }
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
        // A checked source argument is not a finished input cohort. Retain
        // its selected reservation, but leave physical route registration to
        // executable phases. Any eventual demand still validates the cohort.
        let passive_arguments = |original: &Rc<StaticIncomingSourceV1>| -> Result<bool, String> {
            if original.qualified_scalar_result_v1().is_none() || original.is_zero_input_source_v1() {
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
        let mut routed = std::collections::BTreeSet::new(); // a call can sit in several rows: route it once
        for row in &source.incoming {
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &row.source
            else {
                continue;
            };
            if passive_arguments(original)? { continue; }
            if self.is_static_zero_packet_cohort_v1(original)
                || (original.qualified_scalar_result_v1().is_none()
                    && self
                        .checked_completed_static_one_actuals_v1(original)?
                        .is_none())
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
                    if passive_arguments(original)? { continue; }
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
            if !legacy && !applies { continue; }
            let qualified = applies && self.checked_completed_static_expression_input_v1(original)?.is_some();
            let completed = qualified || self
                .checked_static_scalar_packet_actuals_v1(original)?
                .is_some()
                || self
                    .checked_static_mixed_packet_actuals_v1(original)?
                    .is_some()
                || self
                    .checked_static_tagged_packet_actuals_v1(original)?
                    .is_some();
            let selected = if completed {
                self.selected_static_local_source_v1(&row.call)?.is_some()
            } else if self.is_static_zero_packet_cohort_v1(original) {
                // Zero-input source-only rows may be unready; their original
                // demand still refuses at emission, as before this slice.
                self.selected_static_local_source_v1(&row.call)
                    .is_ok_and(|row| row.is_some())
            } else {
                false
            };
            if selected {
                routed.insert(row.call.clone());
            }
        }
        // The selected arity-one CurrentOwner loop owns a separate checked
        // packet source. Only its successfully co-sealed original call joins
        // the same source-ordered lifecycle group; general SourceStatic stays
        // closed for opaque actuals.
        for packet in self.loop_static_packet_sources.borrow().values() {
            if let Ok(packet) = packet {
                routed.insert(packet.local_call_site().clone());
            }
        }
        for site in routed {
            self.record_lifecycle_local_call_site(site, LocalRouteWriterV1::StaticPacket)?;
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
                &row.source
            else {
                continue;
            };
            if self.is_static_zero_packet_cohort_v1(original)
                || (original.qualified_scalar_result_v1().is_none()
                    && self
                        .checked_completed_static_one_actuals_v1(original)?
                        .is_none())
            {
                continue;
            }
            self.selected_static_local_source_v1(&row.call)?
                .ok_or_else(|| freeze("borrowed-static/local-route-source-missing"))?;
            if !routed.is_local(&row.call) {
                return Err(freeze("borrowed-static/local-route-not-sealed"));
            }
            found = true;
        }
        for cohort in source.target_static.values() {
            for row in cohort
                .incoming
                .iter()
                .filter(|row| row.call.owner() == owner)
            {
                self.selected_static_local_source_v1(&row.call)?
                    .ok_or_else(|| freeze("borrowed-static/local-route-source-missing"))?;
                if !routed.is_local(&row.call) {
                    return Err(freeze("borrowed-static/local-route-not-sealed"));
                }
                found = true;
            }
        }
        // An exact-i64 Static call standing as an expression value (argument,
        // operand, condition, printed or returned value) is realized by its
        // checked argument frame and needs no local route row; its owner still
        // selects the ordinary root route.
        for row in source.source_incoming.source_rows_v1().filter(|row| row.call.owner() == owner) {
            if let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) = &row.source {
                if original.qualified_scalar_result_v1().is_some()
                    && crate::mir::resolved_value_profile::expression_position::CallExecutorV1::at(row.call.site()).is_argument_frame()
                    && original.parameters().iter().all(|formal| formal.kind
                        == CallableParameterContractKindV1::ExactTrivial(
                            crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64))
                {
                    found = true;
                }
            }
        }
        for (row, legacy) in source
            .source_incoming
            .exact_rows().map(|row| (row, true))
            .chain(source.source_incoming.source_rows_with_legacy_v1().filter(|(_, legacy)| !legacy))
            .filter(|(row, _)| row.call.owner() == owner)
        {
            let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
                &row.source
            else {
                continue;
            };
            let applies = self.is_static_expression_input_owner_v1(source, original.callee_owner());
            if !legacy && !applies { continue; }
            let qualified = applies && self.checked_completed_static_expression_input_v1(original)?.is_some();
            if (qualified || self.is_static_zero_packet_cohort_v1(original)
                || self
                    .checked_static_scalar_packet_actuals_v1(original)?
                    .is_some()
                || self
                    .checked_static_mixed_packet_actuals_v1(original)?
                    .is_some()
                || self
                    .checked_static_tagged_packet_actuals_v1(original)?
                    .is_some())
                && routed.is_local(&row.call)
            {
                self.selected_static_local_source_v1(&row.call)?
                    .ok_or_else(|| freeze("borrowed-static/local-route-source-missing"))?;
                found = true;
            }
        }
        Ok(found)
    }

}
