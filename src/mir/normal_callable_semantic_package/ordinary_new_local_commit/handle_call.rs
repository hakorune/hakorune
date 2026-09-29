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
        call.prior_homes()
            .iter()
            .rev()
            .map(|binding| match installed_home(&rows, *binding) {
                Ok(row) if row.end_available() => Ok(row.end_operation()),
                _ => Err(freeze("handle-call-prior-home-unavailable")),
            })
            .collect()
    }

    /// The canonical object the callee's result claim/commit minted for a
    /// `return new` site — the identity a caller-side received handle keeps.
    pub(crate) fn result_object(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Option<CanonicalObjectIdV1> {
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
        let construction_site = match self.terminal_relation_for_owner(callee) {
            Some(TerminalRelationV1::Value(row)) if row.owner() == callee => {
                match row.returned() {
                    TerminalReturnedSourceV1::Construction(owned)
                        if owned.owner() == callee =>
                    {
                        owned.clone()
                    }
                    _ => return Err(freeze("handle-result-terminal-mismatch")),
                }
            }
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
        if !matches!(row.progress, CallReceivedProgress::Emitting)
            || bindings.is_empty()
        {
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
