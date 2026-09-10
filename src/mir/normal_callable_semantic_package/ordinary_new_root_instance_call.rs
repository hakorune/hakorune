//! Root lexical instance-call crosswalk for the bounded ordinary-New cohort.
//!
//! The resolver owns the MethodCall and initializer facts. This module only
//! joins those facts with an already selected instance declaration and its
//! result/signature products. It never resolves a name from MIR or C input.

use super::OrdinaryNewClaimLedgerV1;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::normal_callable_semantic_package::physical_signature::{
    PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1,
};
use crate::mir::normal_callable_semantic_package::result_contract::
    VerifiedCallableResultContractCohortV1;
use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedAssignmentTargetV1,
    ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
};
use crate::mir::normal_callable_semantic_package::selected_mapping::
    VerifiedSelectedCallableBatchMapV1;
use hakorune_mir_defs::{
    CanonicalObjectIdV1, CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
};

#[derive(Debug)]
pub(crate) struct RootInstanceCallDispositionRowV1 {
    call_site: OwnedExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver_binding: BindingRefV1,
    receiver_initializer: OwnedExprSiteV1,
    receiver_object: CanonicalObjectIdV1,
    target: CanonicalSameModuleCallableKeyV1,
    target_batch_slot: u32,
    argument_sites: Box<[SourceExprSiteV1]>,
}

impl RootInstanceCallDispositionRowV1 {
    pub(crate) fn call_site(&self) -> &OwnedExprSiteV1 {
        &self.call_site
    }

    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) const fn receiver_binding(&self) -> BindingRefV1 {
        self.receiver_binding
    }

    pub(crate) fn receiver_initializer(&self) -> &OwnedExprSiteV1 {
        &self.receiver_initializer
    }

    pub(crate) const fn receiver_object(&self) -> CanonicalObjectIdV1 {
        self.receiver_object
    }

    pub(crate) fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }

    pub(crate) const fn target_batch_slot(&self) -> u32 {
        self.target_batch_slot
    }

    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }
}

#[derive(Debug)]
pub(super) enum RootInstanceCallDispositionSlotV1 {
    Ready(RootInstanceCallDispositionRowV1),
    Taken,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn issue_root_instance_call_dispositions(
        &mut self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        selected: &VerifiedSelectedCallableBatchMapV1,
        results: &VerifiedCallableResultContractCohortV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<(), String> {
        let owner = input.owner();
        let Some(TerminalRelationV1::Call(terminal)) = self.terminal_relation.as_ref() else {
            return Ok(());
        };
        if terminal.owner() != owner {
            return Err(freeze("root-instance-call-owner"));
        }
        let Some((site, call)) = input
            .function()
            .method_calls()
            .find(|(site, _)| *site == terminal.call_site())
        else {
            return Ok(());
        };
        self.root_instance_call_expected.borrow_mut().insert(owner);
        let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
            call.receiver()
        else {
            return Err(freeze("root-instance-call-receiver"));
        };
        if binding.owner() != owner {
            return Err(freeze("root-instance-call-binding-owner"));
        }
        if input.function().assignment_targets().any(|(_, target)| {
            matches!(target, ResolvedAssignmentTargetV1::BindingRebind(rebound) if *rebound == binding)
        }) {
            return Err(freeze("root-instance-call-reassignment"));
        }
        let mut initializers = input
            .function()
            .expression_source()
            .initializers()
            .filter(|initializer| initializer.binding() == binding);
        let initializer = initializers
            .next()
            .ok_or_else(|| freeze("root-instance-call-initializer-missing"))?;
        if initializers.next().is_some() {
            return Err(freeze("root-instance-call-initializer-duplicate"));
        }
        let initializer_site = initializer
            .initializer_site()
            .ok_or_else(|| freeze("root-instance-call-initializer-absent"))?;
        let owned_initializer = OwnedExprSiteV1::new(owner, initializer_site.clone());
        let claims = self.claims.borrow();
        let claim = claims
            .get(&owned_initializer)
            .ok_or_else(|| freeze("root-instance-call-new-missing"))?;
        if claim.destination != binding {
            return Err(freeze("root-instance-call-new-binding"));
        }
        let target_matches = selected
            .keys()
            .filter_map(|selected_key| {
                let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                    return None;
                };
                if key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod
                    && key.owner() == claim.class()
                    && key.name() == call.selector()
                    && key.arity() == call.arity()
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
            return Err(freeze("root-instance-call-target-selection"));
        };
        let Some(selected_identity) = selected.identity_for_batch_slot(*target_batch_slot) else {
            return Err(freeze("root-instance-call-target-identity"));
        };
        let Some(result) = results.row(*target_batch_slot) else {
            // A selected target without a package-owned result row remains
            // source-unavailable. The root terminal must not fabricate an
            // i64 ABI from the method body or MIR observation.
            return Ok(());
        };
        if result.result().is_none() || !result.borrow().completion().returns_value() {
            return Ok(());
        }
        if result.owner() == owner
            || !result.identity().same_as(selected_identity)
            || result.result() != Some(ExactTrivialScalarAbiV1::I64)
        {
            return Err(freeze("root-instance-call-result-contract"));
        }
        let signature = signatures
            .row(*target_batch_slot)
            .ok_or_else(|| freeze("root-instance-call-signature-missing"))?;
        if signature.mode()
            != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
            || signature.owner() != result.owner()
            || signature.source_logical_arity() != call.arity()
            || signature.receiver_lane_count() != 1
            || signature.lanes().first().is_none_or(|lane| {
                lane.index() != 0
                    || lane.role()
                        != PhysicalCallableLaneRoleV1::InstanceReceiver
            })
        {
            return Err(freeze("root-instance-call-signature-contract"));
        }
        if !call.arguments().is_empty() || !terminal.arguments().is_empty() {
            return Err(freeze("root-instance-call-arguments-unsupported"));
        }
        let receiver_object = claim.object();
        drop(claims);
        let mut rows = self.root_instance_calls.borrow_mut();
        let call_site = OwnedExprSiteV1::new(owner, site.clone());
        if rows
            .insert(
                call_site.clone(),
                RootInstanceCallDispositionSlotV1::Ready(RootInstanceCallDispositionRowV1 {
                    call_site,
                    receiver_site: call.receiver_site().clone(),
                    receiver_binding: binding,
                    receiver_initializer: owned_initializer,
                    receiver_object,
                    target: target.clone(),
                    target_batch_slot: *target_batch_slot,
                    argument_sites: call
                        .arguments()
                        .iter()
                        .map(|argument| argument.site().clone())
                        .collect(),
                }),
            )
            .is_some()
        {
            return Err(freeze("root-instance-call-duplicate"));
        }
        Ok(())
    }

    pub(crate) fn take_root_instance_call(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
    ) -> Result<Option<RootInstanceCallDispositionRowV1>, String> {
        let key = OwnedExprSiteV1::new(owner, site.clone());
        let mut rows = self.root_instance_calls.borrow_mut();
        let Some(slot) = rows.get_mut(&key) else {
            return Ok(None);
        };
        match std::mem::replace(slot, RootInstanceCallDispositionSlotV1::Taken) {
            RootInstanceCallDispositionSlotV1::Ready(row) => Ok(Some(row)),
            RootInstanceCallDispositionSlotV1::Taken => {
                Err(freeze("root-instance-call-already-taken"))
            }
        }
    }

    pub(crate) fn take_root_instance_call_for_return(
        &self,
        owner: FunctionOwnerIdV1,
        return_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<RootInstanceCallDispositionRowV1>, String> {
        let Some((completion, terminal)) = self.call_source_completion_for_owner(owner) else {
            return Ok(None);
        };
        if completion.explicit_site().is_none_or(|site| site.node() != return_site)
            || terminal.return_site().node() != return_site
        {
            return Err(freeze("root-instance-call-return-site"));
        }
        self.take_root_instance_call(owner, terminal.call_site())
    }

    pub(crate) fn root_instance_call_is_empty(&self) -> bool {
        self.root_instance_calls
            .borrow()
            .values()
            .all(|slot| matches!(slot, RootInstanceCallDispositionSlotV1::Taken))
    }

    pub(crate) fn root_instance_call_expected(&self, owner: FunctionOwnerIdV1) -> bool {
        self.root_instance_call_expected.borrow().contains(&owner)
    }
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/{reason}]")
}
