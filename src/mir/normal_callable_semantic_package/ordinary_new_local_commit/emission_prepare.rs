//! New-emission preparation and completion coordination on the shared ledger.
use super::super::{
    ConstructionEligibilityV1, OrdinaryNewAdmissionClaimV1, OrdinaryNewConstructorDispositionV1,
    OrdinaryNewResultClaimV1, OrdinaryNewTrivialArgumentV1,
};
use super::*;
use crate::mir::function::ObjectDestructionDispositionV1;

pub(crate) struct PendingChildPhysicalValidationV1<'ledger> {
    ledger: &'ledger OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    state: ChildPhysicalValidation,
}

impl PendingChildPhysicalValidationV1<'_> {
    pub(crate) fn commit_after_collected(self) {
        let mut states = self.ledger.child_physical_validation.borrow_mut();
        assert!(states.insert(self.owner, self.state).is_none());
    }
}

impl OrdinaryNewClaimLedgerV1 {
    #[cfg(test)]
    pub(in crate::mir) fn child_physical_validation_count_for_test(&self) -> usize {
        self.child_physical_validation.borrow().len()
    }

    /// Shared prepared-state computation: ABI check, reclaim origin and
    /// prior-Home unwind operands. `prior_homes` carries the caller's
    /// resolved `(prior_homes, required_unwind)`; `None` marks a retained
    /// unavailable prefix.
    fn compute_emission_prepare(
        rows: &std::collections::BTreeMap<OwnedExprSiteV1, LocalCommitV1>,
        site: &OwnedExprSiteV1,
        arity: usize,
        constructor: &OrdinaryNewConstructorDispositionV1,
        construction: &ConstructionEligibilityV1,
        execution: Result<super::super::NewExecutionProfileRefV1<'_>, super::super::ConstructionUnavailableV1>,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        destruction: ObjectDestructionDispositionV1,
        children: Option<&[super::super::OwnedFieldChildV1]>,
        prior_homes: Option<(&[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1], &OwnedExprSiteV1)>,
    ) -> Result<NewEmissionProgress, String> {
        if let OrdinaryNewConstructorDispositionV1::Birth(recipe) = constructor {
            let physical = arity
                .checked_add(1)
                .ok_or_else(|| freeze("arity-overflow"))?;
            recipe
                .abi()
                .validate(arity, physical)
                .map_err(|error| format!("[freeze:contract][ordinary-new/abi/{error:?}]"))?;
        }
        let reclaim = match (constructor, construction) {
            (OrdinaryNewConstructorDispositionV1::NoBirthZero, _) => None,
            (OrdinaryNewConstructorDispositionV1::Birth(_), Err(_)) => None,
            (OrdinaryNewConstructorDispositionV1::Birth(recipe), Ok(plan)) => {
                let (constructor_source, constructor_owner) = plan
                    .constructor()
                    .ok_or_else(|| freeze("reclaim-origin-constructor-missing"))?;
                if !plan.reclaims_unpublished_outer_storage()
                    || plan.object() != object
                    || !constructor_source.same_as(recipe.source_id())
                {
                    return Err(freeze("reclaim-origin-source-drift"));
                }
                Some(ReclaimUnpublishedOriginV1 {
                    site: site.clone(),
                    constructor_source: constructor_source.clone(),
                    constructor_owner: *constructor_owner,
                    object: plan.object(),
                })
            }
        };
        let mut operands = Vec::new();
        let mut available = execution.is_ok();
        // Owned publication still requires sealed residences for Normal
        // Home teardown. Birth owns partial-field Fault cleanup; its
        // external caller retains only source-backed storage reclaim.
        available &= destruction.supports_complete_execution(children.is_some());
        match prior_homes {
            None => available = false,
            Some((prior_homes, required_unwind)) => {
                if required_unwind != site {
                    return Err(freeze("prepare-outward-site"));
                }
                for binding in prior_homes {
                    let prior = installed_source_home(rows, binding).map_err(|error| match error {
                        HomeLookupError::Missing => freeze("prior-home-not-installed"),
                        HomeLookupError::Duplicate => freeze("duplicate-prior-home"),
                    })?;
                    available &= prior.end_available();
                    operands.extend(prior.end_operations());
                }
            }
        }
        Ok(if available {
            NewEmissionProgress::Prepared { operands, reclaim }
        } else {
            NewEmissionProgress::RetainedUnavailable {
                progress: UnavailableLocalProgress::PendingExpression,
            }
        })
    }

    /// Select from retained source products before argument descent. No new
    /// source fact is issued here; prior Homes retain the issuer's order.
    pub(crate) fn prepare_new_emission(
        &self,
        claim: &OrdinaryNewAdmissionClaimV1,
    ) -> Result<bool, String> {
        if claim.core.allocation_execution.is_some() {
            let completion = self.completion_source_for_owner_v1(claim.site().owner())
                .ok_or_else(|| freeze("allocation-only/completion-missing"))?;
            if !self.root_home_flow_ready_v1(completion)? {
                return Err(freeze("allocation-only/source-home-closure-unavailable"));
            }
        }
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get(claim.site())
            .and_then(LocalCommitV1::ordinary)
            .ok_or_else(|| freeze("prepare-without-take"))?;
        if !matches!(row.emission, NewEmissionProgress::Unprepared)
            || row.object != claim.object()
            || row.binding != claim.destination
            || !row.box_source.same_source_as(claim.box_source())
            || row.allocation_execution != claim.core.allocation_execution
            || row.allocation_execution.as_ref().is_some_and(|lease|
                !lease.matches_source(&row.construction_source) || !lease.matches_source(claim.construction_source()))
        {
            return Err(freeze("prepare-state-or-source-mismatch"));
        }
        let prior_homes = claim
            .home_prefix()
            .ok()
            .map(|prefix| (prefix.prior_homes(), prefix.required_unwind()));
        let next = Self::compute_emission_prepare(
            &rows,
            claim.site(),
            claim.arity(),
            &claim.core.constructor,
            claim.construction(),
            claim.execution_profile(),
            claim.object(),
            claim.core.destruction,
            claim.core.children.as_deref(),
            prior_homes,
        )?;
        let available = matches!(next, NewEmissionProgress::Prepared { .. });
        if !available {
            self.release_staged_argument_reads(claim.site())?;
        }
        rows.get_mut(claim.site())
            .and_then(LocalCommitV1::ordinary_mut)
            .expect("checked ordinary row")
            .emission = next;
        Ok(available)
    }

    /// Return-position claims prepare through the same selected products;
    /// the only absent facts are the destination binding and its statement.
    pub(crate) fn prepare_result_new_emission(
        &self,
        claim: &OrdinaryNewResultClaimV1,
    ) -> Result<bool, String> {
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get(claim.site())
            .and_then(LocalCommitV1::result)
            .ok_or_else(|| freeze("prepare-without-take"))?;
        if !matches!(row.emission, NewEmissionProgress::Unprepared)
            || row.object != claim.object()
            || !row.box_source.same_source_as(claim.box_source())
        {
            return Err(freeze("prepare-state-or-source-mismatch"));
        }
        let prior_homes = claim
            .home_prefix()
            .ok()
            .map(|prefix| (prefix.prior_homes(), prefix.required_unwind()));
        let mut next = Self::compute_emission_prepare(
            &rows,
            claim.site(),
            claim.arity(),
            &claim.core.constructor,
            claim.construction(),
            claim.core.execution_profile(),
            claim.object(),
            claim.core.destruction,
            claim.core.children.as_deref(),
            prior_homes,
        )?;
        // A non-trivial argument co-seal is a retained claim, not a take-time
        // freeze: the site keeps its raw-lane emission and the row records
        // the truthful unavailable terminal.
        if claim.argument_rows().is_err()
            && matches!(next, NewEmissionProgress::Prepared { .. })
        {
            next = NewEmissionProgress::RetainedUnavailable {
                progress: UnavailableLocalProgress::PendingExpression,
            };
        }
        let available = matches!(next, NewEmissionProgress::Prepared { .. });
        if !available {
            self.release_staged_argument_reads(claim.site())?;
        }
        rows.get_mut(claim.site())
            .and_then(LocalCommitV1::result_mut)
            .expect("checked result row")
            .emission = next;
        Ok(available)
    }

    /// Shared begin for ordinary and result rows — the physical emission
    /// shape is identical; only the destination handling differs.
    pub(crate) fn begin_new_emission(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<(Vec<InvokeOperation>, Option<ReclaimUnpublishedOriginV1>), String> {
        let mut rows = self.local_commits.borrow_mut();
        let emission = rows
            .get_mut(site)
            .and_then(LocalCommitV1::new_emission_mut)
            .ok_or_else(|| freeze("emit-without-take"))?;
        if !matches!(emission, NewEmissionProgress::Prepared { .. }) {
            return Err(freeze("emit-without-prepare-or-duplicate"));
        }
        let NewEmissionProgress::Prepared { operands, reclaim } =
            std::mem::replace(emission, NewEmissionProgress::Emitting)
        else {
            unreachable!()
        };
        Ok((operands, reclaim))
    }

    /// These are validation snapshots of actual instructions, not metadata
    /// operands used for liveness. The physical instructions own every use.
    /// Shared by ordinary and result rows.
    pub(crate) fn record_new_emission(
        &self,
        site: &OwnedExprSiteV1,
        result: ValueId,
        arguments: Vec<ValueId>,
        reclaim: Option<(ReclaimUnpublishedOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let reads = (0..arguments.len()).map(|_| None).collect();
        self.record_new_emission_with_field_array_reads(site, result, arguments.into_iter().map(crate::mir::instruction::BirthArgumentV1::Value).collect(), reads, reclaim, bindings)
    }

    pub(crate) fn record_new_emission_with_field_array_reads(
        &self, site: &OwnedExprSiteV1, result: ValueId, arguments: Vec<crate::mir::instruction::BirthArgumentV1>,
        reads: Vec<Option<crate::mir::normal_callable_semantic_package::EmittedFieldArrayReadArgumentV1>>,
        reclaim: Option<(ReclaimUnpublishedOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get_mut(site)
            .ok_or_else(|| freeze("record-without-take"))?;
        let source_arguments: Vec<OrdinaryNewTrivialArgumentV1> = row
            .new_argument_rows()
            .ok_or_else(|| freeze("record-without-take"))?
            .as_ref()
            .map_err(|_| freeze("argument-source-unavailable"))?
            .to_vec();
        if source_arguments.len() != arguments.len() || reads.len() != arguments.len() {
            return Err(freeze("argument-count-drift"));
        }
        let emission = row
            .new_emission_mut()
            .expect("new_argument_rows implies a new row");
        if !matches!(emission, NewEmissionProgress::Emitting) || bindings.is_empty() {
            return Err(freeze("record-without-emission-or-duplicate"));
        }
        let arguments = source_arguments
            .iter()
            .cloned()
            .zip(arguments).zip(reads)
            .map(|((source, payload), field_array_read)| {
                match (source.field_array_read(), &field_array_read, &payload) {
                    (None, None, _) => {},
                    (Some(original), Some(read), crate::mir::instruction::BirthArgumentV1::Value(value)) if read.matches(original, *value) => {},
                    _ => return Err(freeze("argument-read-record-drift")),
                }
                Ok(EmittedNewArgumentV1 { source, payload, field_array_read })
            })
            .collect::<Result<_, String>>()?;
        *emission = NewEmissionProgress::Emitted {
            result,
            arguments,
            reclaim: reclaim.map(
                |(origin, block, instruction)| ReclaimUnpublishedEmissionV1 {
                    origin,
                    block,
                    instruction,
                },
            ),
            bindings,
            progress: EmittedLocalProgress::PendingExpression,
        };
        Ok(())
    }

    /// Record a provider `new` Birth edge emitted inside a Birth unit. The
    /// store emission itself is the commit boundary — the row is complete
    /// the moment it is recorded, so no progress state machine is needed.
    pub(crate) fn record_provider_birth(
        &self,
        site: OwnedExprSiteV1,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        handoff: BirthAbiHandoffV1,
        receiver: ValueId,
        arguments: Vec<(OrdinaryNewTrivialArgumentV1, ValueId)>,
    ) -> Result<(), String> {
        if site.owner() == handoff.owner() {
            return Err(freeze("provider-birth-owner-drift"));
        }
        let mut emitted = Vec::with_capacity(arguments.len());
        for (ordinal, (source, value)) in arguments.into_iter().enumerate() {
            if source.ordinal() as usize != ordinal
                || source.new_site() != &site
                || source.owner() != site.owner()
            {
                return Err(freeze("provider-birth-argument-drift"));
            }
            if source.field_array_read().is_some() { return Err(freeze("provider-argument-read-unconnected")); }
            emitted.push(EmittedNewArgumentV1 { source, payload: crate::mir::instruction::BirthArgumentV1::Value(value), field_array_read: None });
        }
        self.provider_births.borrow_mut().push(ProviderBirthRecordV1 {
            site,
            object,
            handoff,
            receiver,
            arguments: emitted.into_boxed_slice(),
        });
        Ok(())
    }

    pub(crate) fn complete_new_emissions(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_new_emissions(owner, function)?;
        for row in self
            .local_commits
            .borrow_mut()
            .values_mut()
            .filter(|row| row.owner() == owner)
        {
            match row {
                LocalCommitV1::Ordinary(row) => row.emission.mark_checked(),
                LocalCommitV1::Result(row) => row.emission.mark_result_checked(),
                LocalCommitV1::Map(row) => row.mark_checked(),
                LocalCommitV1::OwnedArray(row) => row.mark_checked(),
                LocalCommitV1::CallReceived(row) => row.mark_checked(),
                LocalCommitV1::TextResidence(_) => {},
            }
        }
        Ok(())
    }

    pub(crate) fn validate_finalized_child_emissions(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_child_emissions_with_pending(owner, function, None)?
            .commit_after_collected();
        Ok(())
    }

    /// Lend only the two source-checked Static Loop calls to the existing
    /// finished-child boundary. No lexical Home row or publication is issued.
    pub(in crate::mir) fn validate_selected_static_loop_child_emissions(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        pending: &super::PendingSelectedStaticLoopViewV1<'_, '_>,
    ) -> Result<PendingChildPhysicalValidationV1<'_>, String> {
        let entry = pending.entry();
        let body = pending.body();
        if entry.source_site().owner() != owner
            || body.source_site().owner() != owner
            || entry.source_site() == body.source_site()
        {
            return Err(freeze("selected-static-loop-child-source-drift"));
        }
        self.validate_child_emissions_with_pending(owner, function, Some(pending))
    }

    fn validate_child_emissions_with_pending(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        pending: Option<&super::PendingSelectedStaticLoopViewV1<'_, '_>>,
    ) -> Result<PendingChildPhysicalValidationV1<'_>, String> {
        if self.child_physical_validation.borrow().contains_key(&owner) {
            return Err(freeze("duplicate-child-physical-validation"));
        }
        self.seal_physical_observations_v1(owner)?;
        if let Some(view) = pending {
            self.validate_new_emissions_with_pending(owner, function, view)?;
        } else {
            self.validate_new_emissions(owner, function)?;
        }
        self.validate_field_reads(owner, function)?;
        self.validate_terminal_scalar_literal_return(owner, function)?;
        self.validate_terminal_i64_field_return_projected(owner, function, None)?;
        self.validate_terminal_i64_add_return_projected(owner, function, None)?;
        let (bindings, selected_body_block) = if let Some(view) = pending {
            self.validate_root_home_exit_with_pending(owner, function, None, view)?;
            self.validate_root_cleanup_shape_with_pending(owner, function, view)?;
            (
                self.lifecycle_bindings_with_pending(owner, view)?,
                Some(view.body().original_invoke_binding().0),
            )
        } else {
            self.validate_root_home_exit(owner, function, None)?;
            self.validate_root_cleanup_shape(owner, function)?;
            (self.lifecycle_bindings(owner)?, None)
        };
        let copies = self.source_local_copies(owner)?;
        let aliases = self.borrowed_ordinary_alias_bindings_v1(owner)?;
        let mut source_phi_roles: std::collections::BTreeMap<_, _> = self
            .recorded_loop_cond_after_blocks(owner, function, &bindings)?
            .into_iter()
            .map(|(block, phis)| (block, (phis, None)))
            .collect();
        if let Some(view) = pending {
            if let Some((normal, predecessor, phis)) = view.body().header_normal_phi_role() {
                if source_phi_roles
                    .insert(normal, (phis.into(), Some(predecessor)))
                    .is_some()
                {
                    return Err(freeze("duplicate-source-phi-role"));
                }
            }
        }
        let boundary = physical_boundary::PhysicalBoundary::capture_with_source_loop_roles_v2(
            function,
            &bindings,
            &copies,
            &aliases,
            &source_phi_roles,
            selected_body_block,
            &self.recorded_source_loop_roles_v2(owner),
        )?;
        Ok(PendingChildPhysicalValidationV1 {
            ledger: self,
            owner,
            state: ChildPhysicalValidation::Checked {
                symbol: function.signature.name.clone(),
                boundary,
            },
        })
    }

    /// Called after all New overrides, never merely after Birth returns.
    pub(crate) fn complete_new_expression(
        &self,
        site: &OwnedExprSiteV1,
        class: &str,
        value: ValueId,
    ) -> Result<(), String> {
        if !self
            .ordinary_box_names
            .iter()
            .any(|name| name.as_ref() == class)
        {
            return Ok(());
        }
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get_mut(site)
            .ok_or_else(|| freeze("expression-without-target-take"))?;
        let Some(box_source) = row.new_box_source() else {
            return Err(freeze("expression-without-target-take"));
        };
        if box_source.name() != class {
            return Err(freeze("expression-parent-mismatch"));
        }
        row.new_emission_mut()
            .expect("new_box_source implies a new row")
            .complete_expression(value)
    }
}
