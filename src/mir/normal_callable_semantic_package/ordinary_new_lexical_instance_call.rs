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
use crate::mir::normal_callable_semantic_package::physical_signature::{
    PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1,
};
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedAssignmentTargetV1,
    ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
};
use hakorune_mir_defs::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};

#[derive(Debug)]
pub(crate) struct LexicalInstanceCallDispositionRowV1 {
    call_site: OwnedExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver_binding: BindingRefV1,
    target: CanonicalSameModuleCallableKeyV1,
    target_batch_slot: u32,
}

impl LexicalInstanceCallDispositionRowV1 {
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
}

#[derive(Debug)]
pub(crate) enum LexicalInstanceCallDispositionSlotV1 {
    Ready(LexicalInstanceCallDispositionRowV1),
    Taken,
}

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
    /// Whether the receiver binding is rebound anywhere in the callee.
    rebound: bool,
}

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
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<(), String> {
        let mut needs = Vec::new();
        for declaration in batch.declarations() {
            let slot = declaration.batch_slot();
            batch
                .with_lowering_input(slot, |input| {
                    let owner = input.owner();
                    for (site, call) in input.function().method_calls() {
                        let ResolvedMethodCallReceiverSourceV1::Lexical(
                            ResolvedLexicalRefV1::Local(binding),
                        ) = call.receiver()
                        else {
                            continue;
                        };
                        if binding.owner() != owner {
                            continue;
                        }
                        let Some(record) = input.function().binding(binding) else {
                            continue;
                        };
                        let parameter_index = match record.kind() {
                            BindingKindV1::Parameter { index } => Some(index),
                            BindingKindV1::Local { .. } => None,
                            _ => continue,
                        };
                        let rebound = input.function().assignment_targets().any(|(_, target)| {
                            matches!(
                                target,
                                ResolvedAssignmentTargetV1::BindingRebind(rebound)
                                    if *rebound == binding
                            )
                        });
                        needs.push(LexicalInstanceCallNeedV1 {
                            owner,
                            callee_slot: slot,
                            call_site: site.clone(),
                            receiver_site: call.receiver_site().clone(),
                            receiver_binding: binding,
                            parameter_index,
                            selector: call.selector().into(),
                            arity: call.arity(),
                            rebound,
                        });
                    }
                })
                .map_err(|_| freeze("lexical-instance-call/batch-loan"))?;
        }

        for need in needs {
            let class = match need.parameter_index {
                Some(index) => {
                    match self.prove_parameter_class(batch, selected, &need, index)? {
                        Some(class) => class,
                        None => continue,
                    }
                }
                None => match self.claim_local_class(batch, &need)? {
                    Some(class) => class,
                    None => continue,
                },
            };
            // Missing or contradictory evidence leaves the call unarmed:
            // outside an armed loop it keeps the existing dynamic path, and
            // inside one the route coverage names the uncovered site. Only
            // structural corruption (batch loan, duplicate issuance) is a
            // hard freeze.
            if need.rebound {
                continue;
            }
            let target_matches = selected
                .keys()
                .filter_map(|selected_key| {
                    let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                        return None;
                    };
                    if key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod
                        && key.owner() == class.as_ref()
                        && key.name() == need.selector.as_ref()
                        && key.arity() == need.arity
                    {
                        selected
                            .batch_slot(selected_key)
                            .map(|slot| (key.clone(), slot))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let [(target, target_batch_slot)] = target_matches.as_slice() else {
                continue;
            };
            let Some(signature) = signatures.row(*target_batch_slot) else {
                continue;
            };
            if signature.mode()
                != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
                || signature.source_logical_arity() != need.arity
                || signature.receiver_lane_count() != 1
                || signature.lanes().first().is_none_or(|lane| {
                    lane.index() != 0
                        || lane.role() != PhysicalCallableLaneRoleV1::InstanceReceiver
                })
            {
                continue;
            }
            let call_site = OwnedExprSiteV1::new(need.owner, need.call_site.clone());
            let mut rows = self.lexical_instance_calls.borrow_mut();
            if rows
                .insert(
                    call_site.clone(),
                    LexicalInstanceCallDispositionSlotV1::Ready(
                        LexicalInstanceCallDispositionRowV1 {
                            call_site,
                            receiver_site: need.receiver_site,
                            receiver_binding: need.receiver_binding,
                            target: target.clone(),
                            target_batch_slot: *target_batch_slot,
                        },
                    ),
                )
                .is_some()
            {
                return Err(freeze("lexical-instance-call/duplicate"));
            }
        }
        Ok(())
    }

    /// Prove one callee `Parameter{index}` receiver's class from caller
    /// edges. The callee must be the only selected declaration carrying its
    /// name+arity, otherwise candidate edges cannot be attributed. Every
    /// lexical/current-owner call site spelled with that selector+arity then
    /// contributes its argument binding's sole-initializer claim class; the
    /// parameter is proven only when every observed claim class collapses to
    /// exactly one. Edges whose argument is rebound, lacks a sole
    /// initializer, or is not a claim-proven local read carry no evidence —
    /// they neither prove nor veto the parameter.
    fn prove_parameter_class(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        need: &LexicalInstanceCallNeedV1,
        index: u32,
    ) -> Result<Option<Box<str>>, String> {
        let Some(callee_key) = selected.keys().find_map(|selected_key| {
            let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                return None;
            };
            (selected.batch_slot(selected_key) == Some(need.callee_slot)).then(|| key.clone())
        }) else {
            return Ok(None);
        };
        let callee_declarations = selected
            .keys()
            .filter(|selected_key| {
                let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                    return false;
                };
                key.name() == callee_key.name() && key.arity() == callee_key.arity()
            })
            .count();
        if callee_declarations != 1 {
            return Ok(None);
        }
        // Universal quantification: every package call site spelled with the
        // callee's selector+arity is a potential edge into it. Arming the
        // parameter receiver requires every such edge to pass an argument
        // that is a sole-initializer, never-rebound, claim-proven local read
        // of one ordinary box — and all observed classes must collapse to a
        // single name. Any edge we cannot prove vetoes the parameter; a
        // vetoed or unproven parameter simply stays unarmed.
        let mut classes = std::collections::BTreeSet::<Box<str>>::new();
        for declaration in batch.declarations() {
            let slot = declaration.batch_slot();
            let edge_proven = batch
                .with_lowering_input(slot, |input| {
                    let owner = input.owner();
                    for (_site, call) in input.function().method_calls() {
                        if call.selector() != callee_key.name()
                            || call.arity() != callee_key.arity()
                        {
                            continue;
                        }
                        let Some(class) = ({
                            if !matches!(
                                call.receiver(),
                                ResolvedMethodCallReceiverSourceV1::Lexical(
                                    ResolvedLexicalRefV1::Local(_)
                                ) | ResolvedMethodCallReceiverSourceV1::CurrentOwner
                            ) {
                                None
                            } else {
                                let Some(argument) = call
                                    .arguments()
                                    .iter()
                                    .find(|argument| argument.ordinal() == index)
                                else {
                                    return false;
                                };
                                let Some(ResolvedLexicalRefV1::Local(argument_binding)) =
                                    input.function().variable_ref(argument.site())
                                else {
                                    return false;
                                };
                                if argument_binding.owner() != owner
                                    || input.function().assignment_targets().any(|(_, target)| {
                                        matches!(
                                            target,
                                            ResolvedAssignmentTargetV1::BindingRebind(rebound)
                                                if *rebound == argument_binding
                                        )
                                    })
                                {
                                    return false;
                                }
                                let mut initializers = input
                                    .function()
                                    .expression_source()
                                    .initializers()
                                    .filter(|initializer| {
                                        initializer.binding() == argument_binding
                                    });
                                let Some(initializer) = initializers.next() else {
                                    return false;
                                };
                                if initializers.next().is_some() {
                                    return false;
                                }
                                let Some(initializer_site) = initializer.initializer_site()
                                else {
                                    return false;
                                };
                                self.claims
                                    .borrow()
                                    .get(&OwnedExprSiteV1::new(
                                        owner,
                                        initializer_site.clone(),
                                    ))
                                    .map(|claim| claim.class().into())
                            }
                        }) else {
                            return false;
                        };
                        let class: Box<str> = class;
                        if !self
                            .ordinary_box_names
                            .iter()
                            .any(|name| name.as_ref() == class.as_ref())
                        {
                            return false;
                        }
                        classes.insert(class);
                    }
                    true
                })
                .map_err(|_| freeze("lexical-instance-call/batch-loan"))?;
            if !edge_proven {
                return Ok(None);
            }
        }
        match classes.len() {
            1 => Ok(classes.into_iter().next()),
            _ => Ok(None),
        }
    }

    /// Prove one claim-local receiver's class from its sole initializer's
    /// ordinary-new claim inside the callee itself.
    fn claim_local_class(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        need: &LexicalInstanceCallNeedV1,
    ) -> Result<Option<Box<str>>, String> {
        batch
            .with_lowering_input(need.callee_slot, |input| {
                if input.owner() != need.owner {
                    return None;
                }
                let mut initializers = input
                    .function()
                    .expression_source()
                    .initializers()
                    .filter(|initializer| initializer.binding() == need.receiver_binding);
                let initializer = initializers.next()?;
                if initializers.next().is_some() {
                    return None;
                }
                let initializer_site = initializer.initializer_site()?;
                let claims = self.claims.borrow();
                let claim =
                    claims.get(&OwnedExprSiteV1::new(need.owner, initializer_site.clone()))?;
                if !self
                    .ordinary_box_names
                    .iter()
                    .any(|name| name.as_ref() == claim.class())
                {
                    return None;
                }
                Some(claim.class().into())
            })
            .map_err(|_| freeze("lexical-instance-call/batch-loan"))
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

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/{reason}]")
}
