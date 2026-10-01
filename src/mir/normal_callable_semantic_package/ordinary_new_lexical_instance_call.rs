//! Lexical-receiver instance-call disposition for the bounded ordinary-New
//! cohort.
//!
//! The resolver owns the MethodCall, initializer, variable-ref and binding
//! facts. This issuer joins those facts with the already selected instance
//! declarations and their physical signatures. It admits two receiver
//! provenance shapes only:
//!
//! * claim-local receivers: `local x = new C()` then `x.m(...)` — the sole
//!   initializer carries the ordinary-new claim and therefore the class;
//! * parameter receivers: `m(self_box)` where every selector+arity candidate
//!   edge in the package passes an argument that carries exactly one
//!   ordinary-new claim class (the `map_argument_edge` caller->callee
//!   argument-provenance precedent).
//!
//! It never resolves a name from MIR or C input, never consumes Dynamic
//! products, and never guesses a receiver class from declared-type spellings.

use super::OrdinaryNewClaimLedgerV1;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::normal_callable_semantic_package::physical_signature::{
    PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1,
};
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, BodyExpressionShapeV1, BodyMeReceiverV1, FunctionOwnerIdV1,
    OwnedExprSiteV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
};
use hakorune_mir_defs::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};

/// Immutable source-target relation shared by preflight and final issuance.
/// Result, completion, ABI adoption and affine consumption are not issued here.
#[derive(Debug, Clone)]
pub(crate) struct LexicalInstanceCallSourceTargetV1 {
    call_site: OwnedExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver_binding: BindingRefV1,
    target: CanonicalSameModuleCallableKeyV1,
    target_batch_slot: u32,
    /// Owner of the callee declaration the target resolves to — the
    /// terminal-relation lookup key for the co-sealed result contract.
    callee_owner: FunctionOwnerIdV1,
    /// Exact non-receiver argument sites in source order.
    argument_sites: Box<[SourceExprSiteV1]>,
}

#[derive(Debug)]
pub(crate) struct LexicalInstanceCallDispositionRowV1 {
    source: LexicalInstanceCallSourceTargetV1,
    // Outcome authorization remains on the final affine disposition only.
    result: Option<InvokeCallResultKind>,
}

impl LexicalInstanceCallSourceTargetV1 {
    pub(crate) fn call_site(&self) -> &OwnedExprSiteV1 {
        &self.call_site
    }

    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) const fn receiver_binding(&self) -> BindingRefV1 {
        self.receiver_binding
    }

    pub(crate) fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }

    pub(crate) const fn target_batch_slot(&self) -> u32 {
        self.target_batch_slot
    }

    pub(crate) const fn callee_owner(&self) -> FunctionOwnerIdV1 {
        self.callee_owner
    }

    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }
}

impl LexicalInstanceCallDispositionRowV1 {
    pub(crate) const fn source_target(&self) -> &LexicalInstanceCallSourceTargetV1 {
        &self.source
    }
    pub(crate) fn call_site(&self) -> &OwnedExprSiteV1 {
        self.source_target().call_site()
    }
    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 {
        self.source_target().receiver_site()
    }
    pub(crate) const fn receiver_binding(&self) -> BindingRefV1 {
        self.source_target().receiver_binding()
    }
    pub(crate) fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.source_target().target()
    }
    pub(crate) const fn target_batch_slot(&self) -> u32 {
        self.source_target().target_batch_slot()
    }
    pub(crate) const fn callee_owner(&self) -> FunctionOwnerIdV1 {
        self.source_target().callee_owner()
    }
    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        self.source_target().argument_sites()
    }
    pub(crate) const fn result(&self) -> Option<InvokeCallResultKind> {
        self.result
    }
}

pub(crate) type LexicalInstanceCallDispositionSlotV1 =
    crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1<
        LexicalInstanceCallDispositionRowV1,
    >;

/// One callee-side call site whose lexical receiver still needs a class.
struct LexicalInstanceCallNeedV1 {
    owner: FunctionOwnerIdV1,
    /// Batch slot of the function that contains the call site.
    callee_slot: u32,
    call_site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver_binding: BindingRefV1,
    /// `Some(index)` for parameter receivers; `None` for claim-local
    /// receivers whose sole initializer proves the class.
    parameter_index: Option<u32>,
    selector: Box<str>,
    arity: u32,
    argument_sites: Box<[SourceExprSiteV1]>,
    /// Whether the receiver binding is rebound anywhere in the callee.
    rebound: bool,
}

#[path = "ordinary_new_lexical_instance_call_provenance.rs"]
mod provenance;

#[path = "ordinary_new_lexical_instance_call_source.rs"]
mod source;
pub(super) use source::{
    prepare_lexical_source_targets_v1, PreparedLexicalInstanceCallSourceTargetsV1,
};

impl OrdinaryNewClaimLedgerV1 {
    /// Join lexical receiver provenance to selected `InstanceBoxMethod`
    /// targets for every method call in the batch. Parameter receivers are
    /// proven by caller edges whose argument carries exactly one claim class;
    /// claim-local receivers are proven by their sole initializer's claim.
    /// Missing, ambiguous, or contradictory evidence simply leaves the call
    /// unarmed — outside an armed loop the existing dynamic path still owns
    /// it, and inside one the route coverage names the uncovered site. Only
    /// structural corruption (batch loan failure, duplicate issuance) is a
    /// hard freeze here.
    pub(in crate::mir::normal_callable_semantic_package) fn issue_lexical_instance_call_dispositions(
        &mut self,
        _batch: &VerifiedResolvedCallableSemanticBatchV1,
        _selected: &VerifiedSelectedCallableBatchMapV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let prepared = self
            .lexical_source_targets
            .take()
            .ok_or_else(|| freeze("lexical-instance-call/missing-source-preparation"))??;
        for source in prepared {
            let Some(source) = source? else {
                continue;
            };
            let target_batch_slot = source.target_batch_slot();
            let target = source.target().clone();
            let call_site = source.call_site().clone();
            let callee_owner = source.callee_owner();
            let Some(signature) = signatures.row(target_batch_slot) else {
                continue;
            };
            if signature.mode()
                != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
                || signature.source_logical_arity() != target.arity()
                || signature.receiver_lane_count() != 1
                || signature.lanes().first().is_none_or(|lane| {
                    lane.index() != 0
                        || lane.role() != PhysicalCallableLaneRoleV1::InstanceReceiver
                })
            {
                continue;
            }
            // The caller-side scan and this co-seal are independent proofs
            // over the same sealed facts: a caller-minted Handle local-call
            // observation must be matched by a callee-side `Value(
            // Construction)` terminal, an unannotated declared result, and a
            // `callable_result_classes` claim; a caller-minted i64
            // observation must be matched by literal-return relations that
            // uniformly classify `I64`. Any half-sealed edge freezes.
            let callee_result = super::super::direct_call_loan::lifecycle::uniform_call_result_kind(
                self.terminal_relations_for_owner(callee_owner).into_iter(),
            );
            let handle_observation = self.handle_call_source(&call_site).is_some();
            let scalar_observation = self.lexical_i64_call_source(&call_site).is_some();
            let result = match (handle_observation, scalar_observation, callee_result) {
                (true, false, Some(InvokeCallResultKind::Handle))
                    if results
                        .row(target_batch_slot)
                        .and_then(|row| row.result())
                        .is_none()
                        && self.callable_result_class(&target).is_some() =>
                {
                    Some(InvokeCallResultKind::Handle)
                }
                (true, ..) => return Err(freeze("lexical-instance-call/handle-result-mismatch")),
                // The observation owes the lifecycle lane: route the site
                // so the emitter's binding-group expectation covers it.
                (false, true, Some(InvokeCallResultKind::I64)) => {
                    self.record_lifecycle_local_call_site(call_site.owner(), call_site.clone());
                    Some(InvokeCallResultKind::I64)
                }
                (false, true, _) => {
                    return Err(freeze("lexical-instance-call/i64-result-mismatch"))
                }
                (false, false, other) => other,
            };
            let mut rows = self.lexical_instance_calls.borrow_mut();
            if rows
                .insert(
                    call_site.clone(),
                    LexicalInstanceCallDispositionSlotV1::Ready(
                        LexicalInstanceCallDispositionRowV1 { source, result },
                    ),
                )
                .is_some()
            {
                return Err(freeze("lexical-instance-call/duplicate"));
            }
        }
        Ok(())
    }

    /// Whether a lexical instance-call row is still armed for one exact call
    /// site. The route probe uses this to classify covered loop items without
    /// consuming the disposition.
    pub(crate) fn lexical_instance_call_covered(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
    ) -> bool {
        self.lexical_instance_calls
            .borrow()
            .get(&OwnedExprSiteV1::new(owner, site.clone()))
            .is_some_and(|slot| matches!(slot, LexicalInstanceCallDispositionSlotV1::Ready(_)))
    }

    /// Consume the armed disposition for one exact call site. Emission is
    /// one-shot per source site, mirroring the locator contract.
    pub(crate) fn take_lexical_instance_call(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
    ) -> Result<Option<LexicalInstanceCallDispositionRowV1>, String> {
        let key = OwnedExprSiteV1::new(owner, site.clone());
        let mut rows = self.lexical_instance_calls.borrow_mut();
        let Some(slot) = rows.get_mut(&key) else {
            return Ok(None);
        };
        match std::mem::replace(slot, LexicalInstanceCallDispositionSlotV1::Taken) {
            LexicalInstanceCallDispositionSlotV1::Ready(row) => Ok(Some(row)),
            LexicalInstanceCallDispositionSlotV1::Taken => {
                Err(freeze("lexical-instance-call/already-taken"))
            }
        }
    }
}

/// The unique selected `InstanceBoxMethod` target for
/// (`owner_box`, `selector`, `arity`), when exactly one exists.
pub(in crate::mir::normal_callable_semantic_package) fn unique_instance_target(
    selected: &VerifiedSelectedCallableBatchMapV1,
    owner: &str,
    selector: &str,
    arity: u32,
) -> Option<(CanonicalSameModuleCallableKeyV1, u32)> {
    let matches = selected
        .keys()
        .filter_map(|selected_key| {
            let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                return None;
            };
            (key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod
                && key.owner() == owner
                && key.name() == selector
                && key.arity() == arity)
                .then(|| {
                    selected
                        .batch_slot(selected_key)
                        .map(|slot| (key.clone(), slot))
                })
                .flatten()
        })
        .collect::<Vec<_>>();
    let [pair] = matches.as_slice() else {
        return None;
    };
    Some(pair.clone())
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/{reason}]")
}
