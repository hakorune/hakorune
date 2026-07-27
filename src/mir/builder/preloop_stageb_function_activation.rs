//! Exact-caller activation for one selected pre-loop Stage-B function.
//!
//! The ledger remains stack-owned.  It observes canonical keys only through
//! the already-installed declaration catalog, consumes the selected source
//! row once, and retains the invocation-local collector together with the
//! collected F6 payload.  It never stores source identity in `MirBuilder`.

use super::calls::{
    collect_preloop_stageb_instance_function_v1, CollectedPreloopStageBInstanceFunctionV1,
    RejectedPreloopStageBInstanceFunctionCollectionV1,
};
use super::module_lifecycle::{
    lower_ordinary_instance_method_v1, InstanceMethodCapturePortV1, InstanceMethodCaptureRequestV1,
};
use super::module_lowering_invocation::{ModuleLoweringInvocationV1, ModuleLoweringPortV1};
use super::module_lowering_invocation_state::ModuleLoweringInvocationStateV1;
use super::{CanonicalSameModuleCallableKeyV1, MirBuilder, ValueId};
use crate::ast::ASTNode;
use crate::mir::preloop_stageb_carrier::{
    PreparedPreloopStageBActivationLedgerPartsV1, PreparedPreloopStageBFunctionIngressV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum PreloopStageBFunctionActivationErrorV1 {
    SelectedCallerNotObserved,
    SelectedCallerConsumedTwice,
    SelectedRequestIdentityDrift,
    SelectedFunctionRejected,
    RootLowering(Box<str>),
}

#[derive(Debug)]
enum PreloopStageBFunctionActivationStateV1 {
    Armed(PreparedPreloopStageBActivationLedgerPartsV1),
    InFlight,
    Completed(CollectedPreloopStageBInstanceFunctionV1),
    Rejected {
        owner: RetainedPreloopStageBFunctionActivationOwnerV1,
        cause: PreloopStageBFunctionActivationErrorV1,
    },
    Transitioning,
}

#[derive(Debug)]
enum RetainedPreloopStageBFunctionActivationOwnerV1 {
    Armed(PreparedPreloopStageBActivationLedgerPartsV1),
    Completed(CollectedPreloopStageBInstanceFunctionV1),
    Collection(RejectedPreloopStageBInstanceFunctionCollectionV1),
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedPreloopStageBFunctionActivationV1 {
    selected: CanonicalSameModuleCallableKeyV1,
    state: PreloopStageBFunctionActivationStateV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CompletedPreloopStageBFunctionActivationV1 {
    result_value: ValueId,
    collected: CollectedPreloopStageBInstanceFunctionV1,
    invocation: ModuleLoweringInvocationStateV1,
    _seal: CompletedPreloopStageBFunctionActivationSealV1,
}

#[derive(Debug)]
struct CompletedPreloopStageBFunctionActivationSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RejectedPreloopStageBFunctionActivationV1 {
    selected: CanonicalSameModuleCallableKeyV1,
    owner: RetainedPreloopStageBFunctionActivationOwnerV1,
    cause: PreloopStageBFunctionActivationErrorV1,
    invocation: Option<ModuleLoweringInvocationStateV1>,
}

enum PreloopStageBFunctionSelectionV1 {
    Ordinary,
    Selected(PreparedPreloopStageBFunctionIngressV1),
}

struct SelectedInstanceMethodCapturePortV1<'ledger, 'port, 'collector> {
    ledger: &'ledger mut PreparedPreloopStageBFunctionActivationV1,
    module_port: &'port mut ModuleLoweringPortV1<'collector>,
}

impl PreparedPreloopStageBFunctionActivationV1 {
    pub(in crate::mir) fn armed(parts: PreparedPreloopStageBActivationLedgerPartsV1) -> Self {
        Self {
            selected: parts.row().caller().clone(),
            state: PreloopStageBFunctionActivationStateV1::Armed(parts),
        }
    }

    pub(in crate::mir) fn context(
        &self,
    ) -> &crate::mir::builder::preloop_stageb_context_install::InstalledPreloopStageBContextV1 {
        match &self.state {
            PreloopStageBFunctionActivationStateV1::Armed(parts) => parts.context(),
            _ => unreachable!("context is inspected only before exact-caller consumption"),
        }
    }

    pub(in crate::mir) fn row(
        &self,
    ) -> &crate::mir::preloop_stageb_carrier::OwnedPreloopStageBCarrierRowV1 {
        match &self.state {
            PreloopStageBFunctionActivationStateV1::Armed(parts) => parts.row(),
            _ => unreachable!("row is inspected only before exact-caller consumption"),
        }
    }

    fn observe_or_claim(
        &mut self,
        observed: &CanonicalSameModuleCallableKeyV1,
        request: &InstanceMethodCaptureRequestV1,
    ) -> Result<PreloopStageBFunctionSelectionV1, PreloopStageBFunctionActivationErrorV1> {
        if observed != &self.selected {
            return Ok(PreloopStageBFunctionSelectionV1::Ordinary);
        }
        if request.function_name != observed.mir_symbol_projection()
            || request.params.len() != observed.arity() as usize
        {
            return Err(PreloopStageBFunctionActivationErrorV1::SelectedRequestIdentityDrift);
        }
        if !matches!(self.state, PreloopStageBFunctionActivationStateV1::Armed(_)) {
            return Err(PreloopStageBFunctionActivationErrorV1::SelectedCallerConsumedTwice);
        }
        let state = std::mem::replace(
            &mut self.state,
            PreloopStageBFunctionActivationStateV1::InFlight,
        );
        let PreloopStageBFunctionActivationStateV1::Armed(parts) = state else {
            unreachable!("armed state checked before consuming exact caller")
        };
        Ok(PreloopStageBFunctionSelectionV1::Selected(
            parts.prepare_function_ingress(),
        ))
    }

    fn complete(&mut self, completed: CollectedPreloopStageBInstanceFunctionV1) {
        debug_assert!(matches!(
            self.state,
            PreloopStageBFunctionActivationStateV1::InFlight
        ));
        debug_assert_eq!(completed.caller(), &self.selected);
        self.state = PreloopStageBFunctionActivationStateV1::Completed(completed);
    }

    fn reject_collection(&mut self, rejected: RejectedPreloopStageBInstanceFunctionCollectionV1) {
        debug_assert!(matches!(
            self.state,
            PreloopStageBFunctionActivationStateV1::InFlight
        ));
        self.state = PreloopStageBFunctionActivationStateV1::Rejected {
            owner: RetainedPreloopStageBFunctionActivationOwnerV1::Collection(rejected),
            cause: PreloopStageBFunctionActivationErrorV1::SelectedFunctionRejected,
        };
    }

    fn reject(&mut self, cause: PreloopStageBFunctionActivationErrorV1) {
        if matches!(
            self.state,
            PreloopStageBFunctionActivationStateV1::Rejected { .. }
        ) {
            return;
        }
        let state = std::mem::replace(
            &mut self.state,
            PreloopStageBFunctionActivationStateV1::Transitioning,
        );
        let owner = match state {
            PreloopStageBFunctionActivationStateV1::Armed(parts) => {
                RetainedPreloopStageBFunctionActivationOwnerV1::Armed(parts)
            }
            PreloopStageBFunctionActivationStateV1::InFlight => {
                unreachable!("synchronous selected capture cannot expose InFlight")
            }
            PreloopStageBFunctionActivationStateV1::Completed(completed) => {
                RetainedPreloopStageBFunctionActivationOwnerV1::Completed(completed)
            }
            PreloopStageBFunctionActivationStateV1::Rejected { .. }
            | PreloopStageBFunctionActivationStateV1::Transitioning => {
                unreachable!("rejection transition must own one live state")
            }
        };
        self.state = PreloopStageBFunctionActivationStateV1::Rejected { owner, cause };
    }

    fn finish(
        mut self,
        result_value: ValueId,
        invocation: ModuleLoweringInvocationStateV1,
    ) -> Result<CompletedPreloopStageBFunctionActivationV1, RejectedPreloopStageBFunctionActivationV1>
    {
        let state = std::mem::replace(
            &mut self.state,
            PreloopStageBFunctionActivationStateV1::Transitioning,
        );
        match state {
            PreloopStageBFunctionActivationStateV1::Completed(collected) => {
                Ok(CompletedPreloopStageBFunctionActivationV1 {
                    result_value,
                    collected,
                    invocation,
                    _seal: CompletedPreloopStageBFunctionActivationSealV1,
                })
            }
            PreloopStageBFunctionActivationStateV1::Armed(parts) => {
                Err(RejectedPreloopStageBFunctionActivationV1 {
                    selected: self.selected,
                    owner: RetainedPreloopStageBFunctionActivationOwnerV1::Armed(parts),
                    cause: PreloopStageBFunctionActivationErrorV1::SelectedCallerNotObserved,
                    invocation: Some(invocation),
                })
            }
            PreloopStageBFunctionActivationStateV1::InFlight => {
                unreachable!("synchronous selected capture cannot finish InFlight")
            }
            PreloopStageBFunctionActivationStateV1::Rejected { owner, cause } => {
                Err(RejectedPreloopStageBFunctionActivationV1 {
                    selected: self.selected,
                    owner,
                    cause,
                    invocation: Some(invocation),
                })
            }
            PreloopStageBFunctionActivationStateV1::Transitioning => {
                unreachable!("transitioning ledger cannot finish")
            }
        }
    }

    fn into_rejection(self) -> RejectedPreloopStageBFunctionActivationV1 {
        let PreloopStageBFunctionActivationStateV1::Rejected { owner, cause } = self.state else {
            unreachable!("only rejected ledger may project a rejection")
        };
        RejectedPreloopStageBFunctionActivationV1 {
            selected: self.selected,
            owner,
            cause,
            invocation: None,
        }
    }
}

impl InstanceMethodCapturePortV1 for SelectedInstanceMethodCapturePortV1<'_, '_, '_> {
    fn lower_instance_method(
        &mut self,
        builder: &mut MirBuilder,
        request: InstanceMethodCaptureRequestV1,
    ) -> Result<(), String> {
        let observed = request.canonical_key.clone();
        match self.ledger.observe_or_claim(&observed, &request) {
            Ok(PreloopStageBFunctionSelectionV1::Ordinary) => {
                lower_ordinary_instance_method_v1(builder, request)
            }
            Ok(PreloopStageBFunctionSelectionV1::Selected(ingress)) => {
                match collect_preloop_stageb_instance_function_v1(
                    builder,
                    self.module_port,
                    ingress,
                ) {
                    Ok(completed) => {
                        self.ledger.complete(completed);
                        Ok(())
                    }
                    Err(rejected) => {
                        let report = rejected.bounded_report();
                        self.ledger.reject_collection(rejected);
                        Err(report.into())
                    }
                }
            }
            Err(cause) => {
                let detail = format!("[mir/preloop-stageb/function-activation] {cause:?}");
                self.ledger.reject(cause);
                Err(detail)
            }
        }
    }
}

impl CompletedPreloopStageBFunctionActivationV1 {
    pub(in crate::mir) const fn result_value(&self) -> ValueId {
        self.result_value
    }

    pub(in crate::mir) const fn selected(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.collected.caller()
    }

    pub(in crate::mir::builder) const fn collected(
        &self,
    ) -> &CollectedPreloopStageBInstanceFunctionV1 {
        &self.collected
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        ValueId,
        CollectedPreloopStageBInstanceFunctionV1,
        ModuleLoweringInvocationStateV1,
    ) {
        (self.result_value, self.collected, self.invocation)
    }

    pub(in crate::mir) fn discard(self) {
        let (_result, collected, _invocation) = self.into_parts();
        collected.discard();
    }
}

impl RejectedPreloopStageBFunctionActivationV1 {
    pub(in crate::mir) const fn cause(&self) -> &PreloopStageBFunctionActivationErrorV1 {
        &self.cause
    }

    pub(in crate::mir) fn bounded_report(&self) -> Box<str> {
        match &self.owner {
            RetainedPreloopStageBFunctionActivationOwnerV1::Collection(rejected) => {
                rejected.bounded_report()
            }
            _ => format!(
                "[mir/preloop-stageb/function-activation/{:?}] caller={:?}",
                self.cause, self.selected
            )
            .into_boxed_str(),
        }
    }

    pub(in crate::mir) fn discard(self) {
        match self.owner {
            RetainedPreloopStageBFunctionActivationOwnerV1::Armed(parts) => {
                let _ = parts;
            }
            RetainedPreloopStageBFunctionActivationOwnerV1::Completed(completed) => {
                completed.discard()
            }
            RetainedPreloopStageBFunctionActivationOwnerV1::Collection(rejected) => {
                rejected.discard()
            }
        }
        let _ = self.invocation;
    }

    #[cfg(test)]
    pub(in crate::mir) fn retained_completed_caller(
        &self,
    ) -> Option<&CanonicalSameModuleCallableKeyV1> {
        match &self.owner {
            RetainedPreloopStageBFunctionActivationOwnerV1::Completed(completed) => {
                Some(completed.caller())
            }
            _ => None,
        }
    }

    #[cfg(test)]
    pub(in crate::mir) const fn retains_invocation_state(&self) -> bool {
        self.invocation.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ASTNode;
    use crate::mir::builder::preloop_stageb_context_install::PreparedPreloopStageBAliasInstallV1;
    use crate::mir::callable_result_representation::actual_parser_add_fixture;

    fn prepared() -> PreparedPreloopStageBFunctionActivationV1 {
        let install =
            crate::mir::preloop_stageb_carrier::test_support::actual_parser_activation_plan()
                .into_module_install_parts_v1()
                .attach_aliases(PreparedPreloopStageBAliasInstallV1::None);
        let mut builder = MirBuilder::new();
        let installed = install.commit(&mut builder).expect("exact context install");
        PreparedPreloopStageBFunctionActivationV1::armed(installed.into_ledger_parts())
    }

    fn selected_request(
        selected: CanonicalSameModuleCallableKeyV1,
    ) -> InstanceMethodCaptureRequestV1 {
        let ASTNode::FunctionDeclaration {
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            ..
        } = actual_parser_add_fixture::method_declaration_for_lowering()
        else {
            panic!("actual selected declaration")
        };
        InstanceMethodCaptureRequestV1 {
            function_name: selected.mir_symbol_projection(),
            owner: selected.owner().to_owned(),
            method: selected.name().to_owned(),
            canonical_key: selected,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        }
    }

    #[test]
    fn exact_key_is_claimed_once_and_duplicate_claim_is_typed() {
        let mut ledger = prepared();
        let selected = ledger.selected.clone();
        let request = selected_request(selected.clone());
        assert!(matches!(
            ledger.observe_or_claim(&selected, &request),
            Ok(PreloopStageBFunctionSelectionV1::Selected(_))
        ));
        assert!(matches!(
            ledger.observe_or_claim(&selected, &request),
            Err(PreloopStageBFunctionActivationErrorV1::SelectedCallerConsumedTwice)
        ));
    }

    #[test]
    fn selected_identity_drift_rejects_before_consuming_armed_owner() {
        let mut ledger = prepared();
        let selected = ledger.selected.clone();
        let mut request = selected_request(selected.clone());
        request.function_name.push_str(".drift");
        assert!(matches!(
            ledger.observe_or_claim(&selected, &request),
            Err(PreloopStageBFunctionActivationErrorV1::SelectedRequestIdentityDrift)
        ));
        assert!(matches!(
            ledger.state,
            PreloopStageBFunctionActivationStateV1::Armed(_)
        ));
    }
}

impl MirBuilder {
    pub(in crate::mir) fn lower_root_with_preloop_stageb_function_activation_v1(
        &mut self,
        source: &ASTNode,
        prepared: PreparedPreloopStageBFunctionActivationV1,
    ) -> Result<CompletedPreloopStageBFunctionActivationV1, RejectedPreloopStageBFunctionActivationV1>
    {
        let mut invocation = ModuleLoweringInvocationV1::open(self);
        let mut ledger = prepared;
        let root_result = invocation.with_module_port(|builder, module_port| {
            let mut capture = SelectedInstanceMethodCapturePortV1 {
                ledger: &mut ledger,
                module_port,
            };
            builder.lower_root_after_callable_catalog_install_with_instance_port_v1(
                source.clone(),
                source,
                &mut capture,
            )
        });
        let result_value = match root_result {
            Ok(result_value) => result_value,
            Err(detail) => {
                ledger.reject(PreloopStageBFunctionActivationErrorV1::RootLowering(
                    detail.into_boxed_str(),
                ));
                return Err(ledger
                    .into_rejection()
                    .attach_invocation(invocation.into_state()));
            }
        };
        ledger.finish(result_value, invocation.into_state())
    }
}

impl RejectedPreloopStageBFunctionActivationV1 {
    fn attach_invocation(mut self, invocation: ModuleLoweringInvocationStateV1) -> Self {
        debug_assert!(self.invocation.is_none());
        self.invocation = Some(invocation);
        self
    }
}
