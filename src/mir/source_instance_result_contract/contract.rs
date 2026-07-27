use crate::mir::callable_result_representation::{
    VerifiedUnannotatedCallableBodyResultOutcomeV1, VerifiedUnannotatedCallableBodyResultProofV1,
};

use super::owned_rebind::OwnedNestedInstanceResultRebindWitnessSealV1;
use super::{
    NestedInstanceResultContractErrorV1, NestedInstanceResultContractStageV1,
    VerifiedCurrentOwnerInstanceResultTargetV1,
};

#[derive(Debug)]
pub(crate) struct SealedNestedInstanceResultContractV1<'site, 'catalog> {
    target: VerifiedCurrentOwnerInstanceResultTargetV1<'site, 'catalog>,
}

impl<'site, 'catalog> SealedNestedInstanceResultContractV1<'site, 'catalog> {
    pub(crate) const fn target(
        &self,
    ) -> &VerifiedCurrentOwnerInstanceResultTargetV1<'site, 'catalog> {
        &self.target
    }

    pub(crate) const fn result_is_integer(&self) -> bool {
        true
    }

    pub(super) fn into_rebind_target(
        self,
    ) -> VerifiedCurrentOwnerInstanceResultTargetV1<'site, 'catalog> {
        self.target
    }

    pub(super) fn from_owned_rebind(
        target: VerifiedCurrentOwnerInstanceResultTargetV1<'site, 'catalog>,
        _seal: OwnedNestedInstanceResultRebindWitnessSealV1,
    ) -> Self {
        Self { target }
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNestedInstanceResultContractV1<'site, 'catalog> {
    target: VerifiedCurrentOwnerInstanceResultTargetV1<'site, 'catalog>,
    stage: NestedInstanceResultContractStageV1,
    cause: NestedInstanceResultContractErrorV1,
}

impl<'site, 'catalog> RejectedNestedInstanceResultContractV1<'site, 'catalog> {
    pub(crate) const fn stage(&self) -> NestedInstanceResultContractStageV1 {
        self.stage
    }

    pub(crate) const fn cause(&self) -> &NestedInstanceResultContractErrorV1 {
        &self.cause
    }

    pub(crate) fn discard(self) {}

    #[cfg(test)]
    pub(crate) const fn target(
        &self,
    ) -> &VerifiedCurrentOwnerInstanceResultTargetV1<'site, 'catalog> {
        &self.target
    }
}

pub(crate) fn seal_nested_instance_result_contract<'site, 'catalog>(
    target: VerifiedCurrentOwnerInstanceResultTargetV1<'site, 'catalog>,
    proof: VerifiedUnannotatedCallableBodyResultProofV1<'catalog>,
) -> Result<
    SealedNestedInstanceResultContractV1<'site, 'catalog>,
    RejectedNestedInstanceResultContractV1<'site, 'catalog>,
> {
    if !proof.matches_declaration(target.target()) {
        proof.discard();
        return Err(reject(
            target,
            NestedInstanceResultContractStageV1::CoSeal,
            NestedInstanceResultContractErrorV1::TargetProofMismatch,
        ));
    }

    let rejection = match proof.outcome() {
        VerifiedUnannotatedCallableBodyResultOutcomeV1::ExactI64 {
            required_i64_arguments,
        } if required_i64_arguments.is_empty() => None,
        VerifiedUnannotatedCallableBodyResultOutcomeV1::ExactI64 {
            required_i64_arguments,
        } => Some((
            NestedInstanceResultContractStageV1::ResultClosure,
            NestedInstanceResultContractErrorV1::NonEmptyRequiredArguments {
                count: required_i64_arguments.len(),
            },
        )),
        VerifiedUnannotatedCallableBodyResultOutcomeV1::Unavailable(reason) => Some((
            NestedInstanceResultContractStageV1::BodyProof,
            NestedInstanceResultContractErrorV1::BodyUnavailable(reason.clone()),
        )),
        VerifiedUnannotatedCallableBodyResultOutcomeV1::PendingDependency => Some((
            NestedInstanceResultContractStageV1::BodyProof,
            NestedInstanceResultContractErrorV1::SealedDependencyPending,
        )),
    };
    proof.discard();
    if let Some((stage, cause)) = rejection {
        return Err(reject(target, stage, cause));
    }
    Ok(SealedNestedInstanceResultContractV1 { target })
}

fn reject<'site, 'catalog>(
    target: VerifiedCurrentOwnerInstanceResultTargetV1<'site, 'catalog>,
    stage: NestedInstanceResultContractStageV1,
    cause: NestedInstanceResultContractErrorV1,
) -> RejectedNestedInstanceResultContractV1<'site, 'catalog> {
    RejectedNestedInstanceResultContractV1 {
        target,
        stage,
        cause,
    }
}
