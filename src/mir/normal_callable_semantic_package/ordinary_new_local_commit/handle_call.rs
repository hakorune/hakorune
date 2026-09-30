//! Handle-result local-call commit coordination on the shared ledger.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    /// The sealed handle-result local call at this expression site: the
    /// caller receives an owned transferred object at the Return edge.
    pub(crate) fn handle_call_source(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Option<&LocalCallObservationV1> {
        self.completion_for_owner(site.owner())
            .and_then(|c| c.cleanup().root_flow())
            .and_then(|flow| {
                flow.local_calls().iter().find(|call| {
                    call.site() == site
                        && call.owner() == site.owner()
                        && call.result() == LocalCallResultClassV1::Handle
                })
            })
    }

    /// Prior-home unwind operands for a Handle local call's fault landing:
    /// every Home recorded live before the call must still be installed
    /// and releasable, released newest-first like every other selected
    /// unwind chain. A lexical receiver call can never strand a live
    /// prior Home.
    pub(crate) fn handle_call_prior_home_unwind(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Vec<InvokeOperation>, String> {
        let call = self
            .handle_call_source(site)
            .ok_or_else(|| freeze("handle-call-source-missing"))?;
        let rows = self.local_commits.borrow();
        let mut operations = Vec::new();
        for binding in call.prior_homes().iter().rev() {
            match installed_home(&rows, *binding) {
                Ok(row) if row.end_available() => {
                    operations.extend(row.end_operations());
                }
                _ => return Err(freeze("handle-call-prior-home-unavailable")),
            }
        }
        Ok(operations)
    }

    /// The sealed nullable-result receiver call at this expression site:
    /// the callee's claim is `NullableObject` — the caller owns the result
    /// only when it is not the `Void` sentinel.
    pub(crate) fn nullable_call_source(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Option<&LocalCallObservationV1> {
        self.completion_for_owner(site.owner())
            .and_then(|c| c.cleanup().root_flow())
            .and_then(|flow| {
                flow.local_calls().iter().find(|call| {
                    call.site() == site
                        && call.owner() == site.owner()
                        && call.result() == LocalCallResultClassV1::Nullable
                })
            })
    }

    /// The sealed `local x = me.m(..)` class observation at one exact call
    /// site — callee key, agreed class, and typed argument evidence.
    pub(crate) fn receiver_call_observation(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Option<&crate::mir::normal_callable_semantic_package::ReceiverCallClassObservationV1>
    {
        self.receiver_call_observations.get(site)
    }

    /// Prior-home unwind operands for a Nullable receiver call's fault
    /// landing — the same newest-first live-Home chain the Handle lane
    /// emits; a `me` receiver borrows and never joins `prior_homes`.
    pub(crate) fn nullable_call_prior_home_unwind(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Vec<InvokeOperation>, String> {
        let call = self
            .nullable_call_source(site)
            .ok_or_else(|| freeze("nullable-call-source-missing"))?;
        let rows = self.local_commits.borrow();
        let mut operations = Vec::new();
        for binding in call.prior_homes().iter().rev() {
            match installed_home(&rows, *binding) {
                Ok(row) if row.end_available() => {
                    operations.extend(row.end_operations());
                }
                _ => return Err(freeze("nullable-call-prior-home-unavailable")),
            }
        }
        Ok(operations)
    }

    /// Begin emission for a nullable-result receiver call: the sealed
    /// local-call relation is sole membership, the callee's `NullableObject`
    /// claim names the class, and the result claims for that class mint
    /// the canonical object the live arm of the result carries. The `Void`
    /// sentinel arm mints no object — the exit release is checked.
    pub(crate) fn begin_nullable_call_emission(
        &self,
        site: &OwnedExprSiteV1,
        callee: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    ) -> Result<(), String> {
        let call = self
            .nullable_call_source(site)
            .ok_or_else(|| freeze("nullable-call-source-missing"))?;
        if !matches!(call.declaration(), SourceBindingSiteV1::Local { .. }) {
            return Err(freeze("nullable-call-declaration-drift"));
        }
        let class = match self.callable_result_classes.get(callee) {
            Some(
                crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::NullableObject(
                    class,
                ),
            ) => class.clone(),
            _ => return Err(freeze("nullable-result-class-mismatch")),
        };
        // Object identity is per-class canonical: every result claim naming
        // this class — whichever `new` site minted it — must agree on the
        // same object. An already-lowered `return new` site moves its row
        // from `result_claims` to the `Result` commit, so the lookup covers
        // both stores. No claims or a drifted set freezes rather than
        // borrowing an arbitrary site's identity.
        let object = {
            let claims = self.result_claims.borrow();
            let commits = self.local_commits.borrow();
            let mut objects = claims
                .values()
                .filter(|claim| claim.class() == class.as_ref())
                .map(|claim| claim.object())
                .chain(commits.values().filter_map(|row| match row {
                    LocalCommitV1::Result(result)
                        if result.box_source.name() == class.as_ref() =>
                    {
                        Some(result.object())
                    }
                    _ => None,
                }));
            let object = objects
                .next()
                .ok_or_else(|| freeze("nullable-result-object-missing"))?;
            if objects.any(|other| other != object) {
                return Err(freeze("nullable-result-object-drift"));
            }
            object
        };
        let mut rows = self.local_commits.borrow_mut();
        if rows.contains_key(site) {
            return Err(freeze("nullable-duplicate-emission"));
        }
        rows.insert(
            site.clone(),
            LocalCommitV1::CallReceived(CallReceivedCommitV1 {
                owner: site.owner(),
                binding: call.destination(),
                declaration: call.declaration().clone(),
                object,
                release: CallReceivedReleaseV1::Nullable,
                // A `Void` sentinel carries no readable field residence: an
                // owned-ArrayBox callee object cannot be discharged through
                // the checked release, so its children stay unproven here
                // even when the residence claim exists.
                end_children: self
                    .owned_field_children
                    .get(&object)
                    .map(|_| None),
                progress: CallReceivedProgress::Emitting,
            }),
        );
        Ok(())
    }

    /// The canonical object the callee's result claim/commit minted for a
    /// `return new` site — the identity a caller-side received handle keeps.
    pub(crate) fn result_object(&self, site: &OwnedExprSiteV1) -> Option<CanonicalObjectIdV1> {
        if let Some(LocalCommitV1::Result(row)) = self.local_commits.borrow().get(site) {
            return Some(row.object());
        }
        self.result_claims
            .borrow()
            .get(site)
            .map(|claim| claim.object())
    }

    /// Begin emission for a handle-result local `Call`: the sealed
    /// local-call relation is sole membership, the callee's retained
    /// `Value(Construction)` terminal is the sole result-kind authority,
    /// and its minted object is the identity the received handle keeps.
    pub(crate) fn begin_handle_call_emission(
        &self,
        site: &OwnedExprSiteV1,
        callee: FunctionOwnerIdV1,
    ) -> Result<(), String> {
        let call = self
            .handle_call_source(site)
            .ok_or_else(|| freeze("handle-call-source-missing"))?;
        if !matches!(call.declaration(), SourceBindingSiteV1::Local { .. }) {
            return Err(freeze("handle-call-declaration-drift"));
        }
        // The callee must prove exactly one `return new` exit: several
        // construction sites mint different canonical objects, and the
        // caller cannot observe which exit ran — a mixed callee stays
        // unadmitted rather than borrowing an arbitrary site's object.
        let construction_site = match self.terminal_relations_for_owner(callee).as_slice() {
            [TerminalRelationV1::Value(row)] if row.owner() == callee => match row.returned() {
                TerminalReturnedSourceV1::Construction(owned) if owned.owner() == callee => {
                    owned.clone()
                }
                _ => return Err(freeze("handle-result-terminal-mismatch")),
            },
            _ => return Err(freeze("handle-result-terminal-missing")),
        };
        let object = self
            .result_object(&construction_site)
            .ok_or_else(|| freeze("handle-result-object-missing"))?;
        let mut rows = self.local_commits.borrow_mut();
        if rows.contains_key(site) {
            return Err(freeze("handle-duplicate-emission"));
        }
        rows.insert(
            site.clone(),
            LocalCommitV1::CallReceived(CallReceivedCommitV1 {
                owner: site.owner(),
                binding: call.destination(),
                declaration: call.declaration().clone(),
                object,
                release: CallReceivedReleaseV1::Handle,
                end_children: self.owned_field_children.get(&object).cloned(),
                progress: CallReceivedProgress::Emitting,
            }),
        );
        Ok(())
    }

    pub(crate) fn record_handle_call_emission(
        &self,
        site: &OwnedExprSiteV1,
        result: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut rows = self.local_commits.borrow_mut();
        let Some(LocalCommitV1::CallReceived(row)) = rows.get_mut(site) else {
            return Err(freeze("handle-record-without-begin"));
        };
        if !matches!(row.progress, CallReceivedProgress::Emitting) || bindings.is_empty() {
            return Err(freeze("handle-emission-state-drift"));
        }
        row.progress = CallReceivedProgress::Emitted {
            result,
            bindings,
            phase: CallReceivedPhase::ExpressionCompleted,
        };
        Ok(())
    }
}
