//! Sole collector terminal for one completed selected Stage-B method.
//!
//! The F6 session retains its complete draft and semantic payload until every
//! fallible collector preflight has succeeded.  Commit then consists only of
//! consuming the prevalidated draft and moving it into the invocation-local
//! collector.

use crate::mir::builder::module_draft_collector::{
    CollectedDraftAdmissionReceiptV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    ModuleDraftAdmissionErrorV1,
};
use crate::mir::builder::module_lowering_invocation::ModuleLoweringPortV1;
use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::preloop_stageb_carrier::PreparedPreloopStageBFunctionIngressV1;
use crate::mir::MirBuilder;

use super::{
    capture_preloop_stageb_instance_function_from_ingress_v1,
    CompletedPreloopStageBInstanceFunctionPayloadV1, CompletedPreloopStageBInstanceFunctionV1,
    RejectedPreloopStageBInstanceFunctionSessionV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum PreloopStageBInstanceFunctionCollectionErrorV1 {
    SymbolMismatch { expected: String, actual: String },
    ArityMismatch { expected: usize, actual: usize },
    Admission(ModuleDraftAdmissionErrorV1),
}

#[derive(Debug)]
enum RetainedPreloopStageBInstanceFunctionCollectionOwnerV1 {
    Session(RejectedPreloopStageBInstanceFunctionSessionV1),
    Completed(CompletedPreloopStageBInstanceFunctionV1),
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedPreloopStageBInstanceFunctionCollectionV1 {
    owner: RetainedPreloopStageBInstanceFunctionCollectionOwnerV1,
    error: Option<PreloopStageBInstanceFunctionCollectionErrorV1>,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CollectedPreloopStageBInstanceFunctionV1 {
    caller: CanonicalSameModuleCallableKeyV1,
    payload: CompletedPreloopStageBInstanceFunctionPayloadV1,
    receipt: CollectedDraftAdmissionReceiptV1,
    _seal: CollectedPreloopStageBInstanceFunctionSealV1,
}

#[derive(Debug)]
struct CollectedPreloopStageBInstanceFunctionSealV1;

impl CollectedPreloopStageBInstanceFunctionV1 {
    pub(in crate::mir::builder) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.caller
    }

    pub(in crate::mir::builder) const fn receipt(&self) -> &CollectedDraftAdmissionReceiptV1 {
        &self.receipt
    }

    pub(in crate::mir::builder) fn discard(self) {
        self.payload.discard();
        let _ = (self.caller, self.receipt);
    }
}

impl RejectedPreloopStageBInstanceFunctionCollectionV1 {
    pub(in crate::mir::builder) fn bounded_report(&self) -> Box<str> {
        match (&self.owner, &self.error) {
            (RetainedPreloopStageBInstanceFunctionCollectionOwnerV1::Session(rejected), None) => {
                rejected.bounded_report()
            }
            (_, Some(error)) => {
                format!("[mir/preloop-stageb/instance-function/collector] {error:?}")
                    .into_boxed_str()
            }
            _ => unreachable!("collector rejection owner/error correspondence"),
        }
    }

    pub(in crate::mir::builder) fn discard(self) {
        match self.owner {
            RetainedPreloopStageBInstanceFunctionCollectionOwnerV1::Session(rejected) => {
                rejected.discard()
            }
            RetainedPreloopStageBInstanceFunctionCollectionOwnerV1::Completed(completed) => {
                let (_draft, payload) = completed.into_parts();
                payload.discard();
            }
        }
    }
}

pub(in crate::mir::builder) fn collect_preloop_stageb_instance_function_v1(
    builder: &mut MirBuilder,
    module_port: &mut ModuleLoweringPortV1<'_>,
    ingress: PreparedPreloopStageBFunctionIngressV1,
) -> Result<
    CollectedPreloopStageBInstanceFunctionV1,
    RejectedPreloopStageBInstanceFunctionCollectionV1,
> {
    let caller = ingress.recipe().caller().clone();
    let pending = capture_preloop_stageb_instance_function_from_ingress_v1(
        builder,
        RawInvocationChildPortV1::new(module_port),
        ingress,
    )
    .map_err(
        |rejected| RejectedPreloopStageBInstanceFunctionCollectionV1 {
            owner: RetainedPreloopStageBInstanceFunctionCollectionOwnerV1::Session(rejected),
            error: None,
        },
    )?;
    let completed = pending.complete();
    let expected_symbol = caller.mir_symbol_projection();
    let expected_arity = caller.arity() as usize + 1;
    if completed.draft().signature.name != expected_symbol {
        let actual = completed.draft().signature.name.clone();
        return Err(reject_completed(
            completed,
            PreloopStageBInstanceFunctionCollectionErrorV1::SymbolMismatch {
                expected: expected_symbol,
                actual,
            },
        ));
    }
    if completed.draft().signature.params.len() != expected_arity {
        let actual = completed.draft().signature.params.len();
        return Err(reject_completed(
            completed,
            PreloopStageBInstanceFunctionCollectionErrorV1::ArityMismatch {
                expected: expected_arity,
                actual,
            },
        ));
    }
    let admission = match module_port.prepare_draft_admission(
        FunctionDraftKeyV1::LegacySymbol(expected_symbol.clone()),
        expected_symbol,
        expected_arity,
        DraftPublicationPolicyV1::LegacyReplaceWholePair,
    ) {
        Ok(admission) => admission,
        Err(error) => {
            return Err(reject_completed(
                completed,
                PreloopStageBInstanceFunctionCollectionErrorV1::Admission(error),
            ))
        }
    };
    let (draft, payload) = completed.into_parts();
    let receipt = admission
        .seal_after_exact_signature_preflight(draft)
        .collect();
    Ok(CollectedPreloopStageBInstanceFunctionV1 {
        caller,
        payload,
        receipt,
        _seal: CollectedPreloopStageBInstanceFunctionSealV1,
    })
}

fn reject_completed(
    completed: CompletedPreloopStageBInstanceFunctionV1,
    error: PreloopStageBInstanceFunctionCollectionErrorV1,
) -> RejectedPreloopStageBInstanceFunctionCollectionV1 {
    RejectedPreloopStageBInstanceFunctionCollectionV1 {
        owner: RetainedPreloopStageBInstanceFunctionCollectionOwnerV1::Completed(completed),
        error: Some(error),
    }
}
