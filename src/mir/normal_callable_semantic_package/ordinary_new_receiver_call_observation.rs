//! Receiver-call result-class observations for `local x = me.m(..)`.
//!
//! Source authority: the resolver's sealed initializer rows, method-call
//! inventory and binding kinds — the same passive facts the claim issuer
//! walks. Canonical issuer: this module's per-declaration observation,
//! driven inside the `issue_ordinary_source_cohort_v1` verified walk —
//! the claim map is sealed before that walk begins, so `me.m` sites mint
//! in the same sweep with no deferred pass.
//!
//! An observation row is claim-faithful evidence only: `Object(C)` is
//! Handle-eligible evidence, `NullableObject(C)` is nullable evidence
//! that can never mint Handle. No emission path changes — every existing
//! consumer gate (prior-homes, emitted-commit sealedness,
//! `begin_handle_call_emission`) keeps failing closed, and sites whose
//! callee is unclaimed keep the existing `BoundValue` +
//! `PrefixNotCovered` floor. `me.f.m(..)` receivers (`Other`) and
//! return-position `me.m` sites (already carried by the claim product)
//! stay outside this lane.

use std::collections::BTreeMap;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, OwnedExprSiteV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    ResolvedMethodCallReceiverSourceV1,
};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

/// One observed `local x = me.m(..)` call result bound to a composed
/// callee claim — the owning map key is the exact call site and the row
/// carries the exact destination local. Additive evidence — minted only
/// when the entry loan proves the receiver is this declaration's own
/// `me`, the callee's canonical key resolves uniquely, and it carries a
/// claim; never a Handle authorization by itself.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct ReceiverCallClassObservationV1 {
    callee: CanonicalSameModuleCallableKeyV1,
    class: super::OrdinaryNewResultClassV1,
    destination: BindingRefV1,
}

impl ReceiverCallClassObservationV1 {
    /// The resolved callee — the same canonical key the claim map uses.
    #[cfg(test)]
    pub(crate) fn callee(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.callee
    }

    /// The callee's composed result-class claim, carried verbatim.
    #[cfg(test)]
    pub(crate) fn class(&self) -> &super::OrdinaryNewResultClassV1 {
        &self.class
    }

    /// The sole local bound by the `local x = ..` initializer — exact
    /// destination evidence for the site-keyed row.
    #[cfg(test)]
    pub(crate) fn destination(&self) -> BindingRefV1 {
        self.destination
    }
}

/// In-walk observation over one declaration's `local x = me.m(..)`
/// initializers. `receiver_proof` is the entry-loan proof — the sole
/// Home ABI issuer bound `me` to this declaration's own box — and the
/// receiver binding must equal the loan's exact `me` binding; nothing
/// else counts as the receiver. A rebound destination cannot carry
/// stable class evidence; every unresolvable or unclaimed edge simply
/// produces no row.
pub(super) fn observe_receiver_call_sites(
    input: ResolvedFunctionLoweringInputV1<'_>,
    receiver_proof: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    claims: &super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    observations: &mut BTreeMap<OwnedExprSiteV1, ReceiverCallClassObservationV1>,
) {
    let Some((receiver, box_row)) = receiver_proof else {
        return;
    };
    let function = input.function();
    let owner = input.owner();
    for initializer in function.expression_source().initializers() {
        let Some(site) = initializer.initializer_site() else {
            continue;
        };
        let Some(call) = function.method_call(site) else {
            continue;
        };
        let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
            call.receiver()
        else {
            continue;
        };
        if binding != receiver {
            continue;
        }
        let destination = initializer.binding();
        if function.assignment_targets().any(|(_, target)| {
            matches!(
                target,
                ResolvedAssignmentTargetV1::BindingRebind(rebound) if *rebound == destination
            )
        }) {
            continue;
        }
        let Some((callee, _slot)) = super::lexical_instance_call::unique_instance_target(
            selected,
            box_row.name(),
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
                destination,
            },
        );
    }
}
