//! Static packet actuals borrow the same complete checked incoming owner.
//! Publication proves the result; it does not replace entry or actual proof.
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use crate::mir::normal_callable_semantic_package::PreparedStaticTerminalRootRouteV1;
use std::rc::Rc;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;

impl OrdinaryNewClaimLedgerV1 {
    /// Lend the canonical incoming source without exposing its inventory.
    pub(in crate::mir::normal_callable_semantic_package)
    fn borrow_original_static_source_v1(&self, site: &OwnedExprSiteV1)
        -> Result<Option<&Rc<StaticIncomingSourceV1>>, String> {
        let Some(source) = self.borrowed_formal_source.as_ref() else { return Ok(None) };
        let source = source.as_ref().map_err(Clone::clone)?;
        source.source_incoming.static_observations().get(site)
            .map(|row| row.as_ref().map_err(Clone::clone)).transpose()
    }

    /// Bridge the completed original scalar cohort to its ordered packet actuals.
    fn checked_static_scalar_packet_actuals_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        self.with_checked_static_scalar_packet_actuals_v1(original, |arguments, _| {
            let Some(observation) = self.local_call_for_owner(
                original.call_site().owner(), original.call_site().site(),
            ) else { return Ok(false); };
            if observation.arguments() != arguments {
                return Err(freeze("static-scalar/packet-ordered-arguments-drift"));
            }
            Ok(true)
        })
    }

    fn with_checked_static_scalar_packet_actuals_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
        corroborate: impl FnOnce(&[LocalCallArgumentV1], &[PreparedBorrowedFormalActualV1])
            -> Result<bool, String>,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        let Some(Ok(source)) = self.borrowed_formal_source.as_ref() else {
            return Ok(None);
        };
        let Some(cohort) =
            self.checked_completed_static_scalar_cohort_v1(source, original.callee_owner())?
        else {
            return Ok(None);
        };
        let call = cohort
            .iter()
            .find(|row| {
                row.call == *original.call_site()
                    && matches!(&row.source,
                    super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(retained)
                        if Rc::ptr_eq(retained, original))
            })
            .ok_or_else(|| freeze("static-scalar/packet-incoming-missing"))?;
        let rows = self
            .borrowed_formal_actuals
            .get(original.call_site())
            .ok_or_else(|| freeze("static-scalar/packet-actuals-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        rows.require_executable_v1()?;
        let arguments = rows.ordered_arguments_for_v1(call)?;
        if !corroborate(arguments, rows.opaque_actuals.as_ref())? {
            return Ok(None);
        }
        Ok(Some(rows.opaque_actuals.as_ref()))
    }

    /// A provider borrows the same scalar nucleus using its own original
    /// constructor source. It never passes through the Local observation owner.
    pub(crate) fn prepare_provider_static_packet_v1(
        &self,
        provider: &Rc<crate::mir::normal_callable_semantic_package::ProviderStaticArgumentSourceV1>,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::PreparedProviderStaticArgumentV1>, String> {
        let Some(prepared) = self.borrowed_formal_source.as_ref() else { return Ok(None); };
        let source = prepared.as_ref().map_err(Clone::clone)?;
        let (caller, site, target, literals) = provider.original_call_v1()
            .ok_or_else(|| freeze("provider-static/call-source-missing"))?;
        source.corroborate_original_provider_static_source_v1(provider)?;
        if source.scalar_qualified_source_call_v1(&site)?.is_none() { return Ok(None); }
        let original = source.source_incoming.static_observations().get(&site)
            .ok_or_else(|| freeze("provider-static/incoming-missing"))?
            .as_ref().map_err(Clone::clone)?;
        self.verify_original_static_packet_source_v1(original)?;
        if original.caller() != caller || original.target() != target
            || original.call_site() != &site || !original.is_qualified_i64_source_v1()
            || original.argument_sites().len() != literals.len() {
            return Err(freeze("provider-static/packet-source-drift"));
        }
        let mut ordered = None;
        self.with_checked_static_scalar_packet_actuals_v1(original, |arguments, actuals| {
            if arguments.len() != literals.len() {
                return Err(freeze("provider-static/packet-arity"));
            }
            for (ordinal, (argument, literal)) in arguments.iter().zip(literals).enumerate() {
                use crate::mir::normal_callable_semantic_package::QualifiedStaticCallArgumentKindV1 as Literal;
                let valid = match argument {
                    LocalCallArgumentV1::Integer(value) => matches!(literal, Literal::Integer(expected) if value == expected),
                    LocalCallArgumentV1::Bool(value) => matches!(literal, Literal::Bool(expected) if value == expected),
                    LocalCallArgumentV1::BorrowedActual { ordinal: index, site } => {
                        *index as usize == ordinal && original.argument_sites().get(ordinal) == Some(site)
                            && actuals.iter().filter(|actual| actual.ordinal == *index).count() == 1
                            && actuals.iter().any(|actual| actual.ordinal == *index && &actual.site == site
                                && original.parameters().get(ordinal).is_some_and(|formal| formal.binding == actual.formal)
                                && match (&actual.source, literal) {
                                    (BorrowedFormalActualSourceV1::Integer(value), Literal::Integer(expected)) => value == expected,
                                    (BorrowedFormalActualSourceV1::Bool(value), Literal::Bool(expected)) => value == expected,
                                    _ => false,
                                })
                    }
                    _ => false,
                };
                if !valid { return Err(freeze("provider-static/packet-actual-drift")); }
            }
            ordered = Some(arguments.to_vec().into_boxed_slice());
            Ok(true)
        })?.ok_or_else(|| freeze("provider-static/whole-input-unready"))?;
        Ok(Some(crate::mir::normal_callable_semantic_package::PreparedProviderStaticArgumentV1 {
            source: Rc::clone(provider), incoming: Rc::clone(original),
            arguments: ordered.ok_or_else(|| freeze("provider-static/ordered-arguments-missing"))?,
        }))
    }

    pub(crate) fn borrowed_provider_packet_actuals_v1(
        &self,
        packet: &crate::mir::normal_callable_semantic_package::PreparedProviderStaticArgumentV1,
    ) -> Result<&[PreparedBorrowedFormalActualV1], String> {
        let checked = self.prepare_provider_static_packet_v1(packet.original_source())?
            .ok_or_else(|| freeze("provider-static/final-source-unavailable"))?;
        if !Rc::ptr_eq(checked.incoming(), packet.incoming())
            || checked.arguments() != packet.arguments() {
            return Err(freeze("provider-static/final-source-drift"));
        }
        // The preceding original-source lender checked these same immutable rows.
        let rows = self.borrowed_formal_actuals.get(packet.incoming().call_site())
            .ok_or_else(|| freeze("provider-static/final-actuals-missing"))?
            .as_ref().map_err(Clone::clone)?;
        Ok(rows.opaque_actuals.as_ref())
    }

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

    /// Select only a fully finished original CurrentOwner Scalar cohort.
    /// The incoming inventory stays the sole source; this is route membership.
    pub(in crate::mir::normal_callable_semantic_package) fn select_static_scalar_local_routes_v1(
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
            self.lexical_i64_call_source(site)
                .ok_or_else(|| freeze("borrowed-static/local-completion-missing"))?;
            return Ok(Some(Rc::clone(original)));
        }
        if let Some(original) = source.source_incoming.static_observations().get(site) {
            let original = original.as_ref().map_err(Clone::clone)?;
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
        if !original.is_qualified_i64_source_v1() {
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
                &row.source
            else {
                continue;
            };
            if self.is_static_zero_packet_cohort_v1(original)
                || (!original.is_qualified_i64_source_v1()
                    && self
                        .checked_completed_static_one_actuals_v1(original)?
                        .is_none())
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
            let completed = self
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
                &row.source
            else {
                continue;
            };
            if self.is_static_zero_packet_cohort_v1(original)
                || (!original.is_qualified_i64_source_v1()
                    && self
                        .checked_completed_static_one_actuals_v1(original)?
                        .is_none())
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
            for row in cohort
                .incoming
                .iter()
                .filter(|row| row.call.owner() == owner)
            {
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
            if (self.is_static_zero_packet_cohort_v1(original)
                || self
                    .checked_static_scalar_packet_actuals_v1(original)?
                    .is_some()
                || self
                    .checked_static_mixed_packet_actuals_v1(original)?
                    .is_some()
                || self
                    .checked_static_tagged_packet_actuals_v1(original)?
                    .is_some())
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

    /// The root packet owns this exact call's Invoke. Move its previously
    /// co-sealed local route out of the local binding-group expectation only
    /// when root emission actually selects the packet.
    pub(in crate::mir) fn claim_static_terminal_root_route_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<(), String> {
        self.prepare_static_terminal_root_route_v1(original)?
            .commit_prechecked();
        Ok(())
    }

    pub(crate) fn prepare_static_terminal_root_route_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<PreparedStaticTerminalRootRouteV1<'_>, String> {
        let selected = self
            .selected_static_local_source_v1(original.call_site())?
            .ok_or_else(|| freeze("static-terminal/route-missing"))?;
        if !Rc::ptr_eq(&selected, original) {
            return Err(freeze("static-terminal/route-drift"));
        }
        let routed = self.lifecycle_local_call_sites.borrow();
        let sites = routed
            .get(&original.call_site().owner())
            .ok_or_else(|| freeze("static-terminal/local-route-missing"))?;
        if !sites.iter().any(|site| site == original.call_site()) {
            return Err(freeze("static-terminal/local-route-missing"));
        }
        Ok(PreparedStaticTerminalRootRouteV1 {
            ledger: self,
            site: original.call_site().clone(),
        })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn verify_original_static_packet_source_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<(), String> {
        if !original.is_zeroarg_i64_v1()
            && !original.is_qualified_i64_source_v1()
            && self
                .checked_static_scalar_packet_actuals_v1(original)?
                .is_none()
            && self
                .checked_static_mixed_packet_actuals_v1(original)?
                .is_none()
        {
            self.checked_completed_static_one_actuals_v1(original)?
                .ok_or_else(|| freeze("ordinary-new/borrowed-entry/source-only-static-actuals"))?;
        }
        self.verify_original_static_source_v1(original)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn verify_original_static_execution_signature_v1(
        &self, source: &Rc<StaticIncomingSourceV1>,
        signatures: &Rc<crate::mir::normal_callable_semantic_package::physical_signature::VerifiedCallablePhysicalSignatureCohortV1>,
    ) -> Result<(), String> {
        self.verify_original_static_source_v1(source)?;
        let (selected, original) = self.original_instance_input_issuance.as_ref().ok_or_else(|| freeze("static-execution/issuance"))?;
        if !Rc::ptr_eq(original, signatures) || !source.catalog_brand().is_same(selected.catalog_brand())
            || selected.batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(source.target().clone())) != Some(source.target_batch_slot()) {
            return Err(freeze("static-execution/original-signature"));
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn verify_original_static_source_v1(
        &self, original: &Rc<StaticIncomingSourceV1>,
    ) -> Result<(), String> {
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
        if let Some(rows) = self.checked_completed_static_expression_input_v1(original)? {
            let observation = self.local_call_for_owner(original.call_site().owner(), original.call_site().site())
                .ok_or_else(|| freeze("static-input/local-source-missing"))?;
            if observation.arguments() != rows.ordered_arguments.as_ref() {
                return Err(freeze("static-input/ordered-arguments-drift"));
            }
            return Ok(Some(rows.opaque_actuals.as_ref()));
        }
        if let Some(rows) = self.checked_static_tagged_packet_actuals_v1(original)? {
            return Ok(Some(rows));
        }
        if original.is_qualified_i64_source_v1() {
            if let Some(rows) = self.checked_static_scalar_packet_actuals_v1(original)? {
                return Ok(Some(rows));
            }
        }
        if original.is_current_owner_i64_source_v1() && original.argument_sites().len() == 1 {
            if let Some(rows) = self.checked_static_scalar_packet_actuals_v1(original)? {
                return Ok(Some(rows));
            }
            if let Some(rows) = self.checked_static_mixed_packet_actuals_v1(original)? {
                return Ok(Some(rows));
            }
            let row = self
                .checked_completed_static_one_actuals_v1(original)?
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
        self.with_qualified_static_packet_actuals_v1(original, |arguments| {
            let observation = self
                .local_call_for_owner(original.call_site().owner(), original.call_site().site())
                .ok_or_else(|| freeze("borrowed-static/local-source-missing"))?;
            if observation.arguments() != arguments {
                return Err(freeze("borrowed-static/ordered-arguments-drift"));
            }
            Ok(())
        }).map(Some)
    }

    /// Join the one immutable qualified source, its whole incoming cohort and
    /// executable ordered actuals before a continuation corroborates its own
    /// source representation. Local observations remain a caller obligation;
    /// this lender neither creates one nor grants result/physical permission.
    fn with_qualified_static_packet_actuals_v1(
        &self,
        original: &Rc<StaticIncomingSourceV1>,
        corroborate_arguments: impl FnOnce(&[LocalCallArgumentV1]) -> Result<(), String>,
    ) -> Result<&[PreparedBorrowedFormalActualV1], String> {
        original.require_qualified_i64_source_v1()?;
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
        self.checked_borrowed_entry_incoming_core_v1(source, call.callee, true)?;
        let rows = self
            .borrowed_formal_actuals
            .get(&call.call)
            .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        rows.require_executable_v1()?;
        let arguments = rows.ordered_arguments_for_v1(call)?;
        corroborate_arguments(&arguments)?;
        Ok(rows.opaque_actuals.as_ref())
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_packet_entry_tests.rs"]
mod tests;
