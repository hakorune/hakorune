//! Receiver-call result-class observations for `local x = me.m(..)`.
//!
//! Source authority: the resolver's sealed initializer rows, method-call
//! inventory and binding kinds — the same passive facts the claim issuer
//! walks. Canonical issuer: this module's single pass, driven from
//! `issue_ordinary_source_cohort_v1` *after* the result-class fixpoint —
//! the claim map cannot exist during the per-declaration completion
//! scan, so `me.m` sites are observed by this deferred pass over the
//! composed `callable_result_classes` product (the claim draft's own
//! two-pass precedent).
//!
//! An observation row is claim-faithful evidence only: `Object(C)` is
//! Handle-eligible evidence, `NullableObject(C)` is nullable evidence
//! that can never mint Handle. The pass changes no emission path —
//! every existing consumer gate (prior-homes, emitted-commit sealedness,
//! `begin_handle_call_emission`) keeps failing closed, and sites whose
//! callee is unclaimed keep the existing `BoundValue` + `PrefixNotCovered`
//! floor. `me.f.m(..)` receivers (`Other`) and return-position `me.m`
//! sites (already carried by the claim product) stay outside this lane.

use std::collections::BTreeMap;

use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, OwnedExprSiteV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    ResolvedMethodCallReceiverSourceV1,
};
use hakorune_mir_defs::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};

/// One observed `local x = me.m(..)` call result bound to a composed
/// callee claim — the owning map key is the exact call site. Additive
/// evidence — minted only when the callee's canonical key resolves
/// uniquely and carries a claim; never a Handle authorization by itself.
#[derive(Debug, Clone)]
pub(crate) struct ReceiverCallClassObservationV1 {
    callee: CanonicalSameModuleCallableKeyV1,
    class: super::OrdinaryNewResultClassV1,
}

impl ReceiverCallClassObservationV1 {
    /// The resolved callee — the same canonical key the claim map uses.
    pub(crate) fn callee(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.callee
    }

    /// The callee's composed result-class claim, carried verbatim.
    pub(crate) fn class(&self) -> &super::OrdinaryNewResultClassV1 {
        &self.class
    }
}

/// Deferred claim-aware pass over `local x = me.m(..)` initializers.
/// Runs after the result-class fixpoint so callee claims are final;
/// every unresolvable or unclaimed edge simply produces no row.
pub(super) fn install_v1(
    ledger: &mut super::OrdinaryNewClaimLedgerV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    claims: &super::result_class_claim::OrdinaryNewResultClassClaimsV1,
) {
    ledger.receiver_call_observations = walk(batch, selected, claims);
}

fn walk(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    claims: &super::result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> BTreeMap<OwnedExprSiteV1, ReceiverCallClassObservationV1> {
    let mut observations = BTreeMap::new();
    for declaration in batch.declarations() {
        let Some(own_box) = selected
            .keys()
            .filter_map(|selected_key| {
                let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                    return None;
                };
                (selected.batch_slot(selected_key) == Some(declaration.batch_slot())
                    && key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod)
                    .then(|| key.owner().to_owned())
            })
            .next()
        else {
            continue;
        };
        let _ = batch.with_lowering_input(declaration.batch_slot(), |input| {
            let function = input.function();
            let owner = input.owner();
            for initializer in function.expression_source().initializers() {
                let Some(site) = initializer.initializer_site() else {
                    continue;
                };
                let Some(call) = function.method_call(site) else {
                    continue;
                };
                let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(
                    receiver,
                )) = call.receiver()
                else {
                    continue;
                };
                if !function
                    .binding(receiver)
                    .is_some_and(|record| record.kind() == BindingKindV1::Receiver)
                {
                    continue;
                }
                // A rebound destination cannot carry stable class evidence.
                let destination = initializer.binding();
                if function.assignment_targets().any(|(_, target)| {
                    matches!(
                        target,
                        ResolvedAssignmentTargetV1::BindingRebind(rebound)
                            if *rebound == destination
                    )
                }) {
                    continue;
                }
                let Some((callee, _slot)) = super::lexical_instance_call::unique_instance_target(
                    selected,
                    own_box.as_ref(),
                    call.selector(),
                    call.arity(),
                ) else {
                    continue;
                };
                let Some(class) = claims.get(&callee) else {
                    continue;
                };
                observations.insert(
                    OwnedExprSiteV1::new(owner, site.clone()),
                    ReceiverCallClassObservationV1 {
                        callee: callee.clone(),
                        class: class.clone(),
                    },
                );
            }
        });
    }
    observations
}
