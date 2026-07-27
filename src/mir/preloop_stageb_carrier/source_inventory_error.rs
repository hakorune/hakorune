use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::{
    CallableBodyProofIssueErrorV1, CallableResultCatalogErrorV1, StaticExactI64RequirementErrorV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::RawSourceCursorErrorV1;
use crate::mir::source_instance_result_contract::{
    NestedInstanceResultContractErrorV1, PreloopLocatedArgumentErrorV1,
    PreloopNestedResultAssociationErrorV1,
};

use super::activation::{
    PreloopStageBCarrierActivationErrorV1, PreloopStageBCarrierActivationStageV1,
};
use super::rejection::{
    PreloopOuterCarrierResultContractErrorV1, PreloopOuterCarrierResultContractStageV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreloopStageBSourceInventoryStageV1 {
    CallableResults,
    OuterRequirement,
    RawSourceProjection,
    CompleteObservation,
    InnerBodyProof,
    InnerContract,
    SourceAssociation,
    LocatedArgument,
    OuterContract,
    OwnedRow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreloopStageBSourceProofStageV1 {
    WholeSourceMethodObservation,
    CandidateInventory(PreloopStageBSourceInventoryStageV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PreloopStageBSourceInventoryCauseV1 {
    CallableResults(CallableResultCatalogErrorV1),
    OuterRequirement(StaticExactI64RequirementErrorV1),
    RawSourceProjection(RawSourceCursorErrorV1),
    OuterSiteProjectionMismatch,
    InnerCallMissingFromCompleteInventory,
    InnerBodyProof(CallableBodyProofIssueErrorV1),
    InnerContract(NestedInstanceResultContractErrorV1),
    SourceAssociation(PreloopNestedResultAssociationErrorV1),
    LocatedArgument(PreloopLocatedArgumentErrorV1),
    OuterContract {
        stage: PreloopOuterCarrierResultContractStageV1,
        cause: PreloopOuterCarrierResultContractErrorV1,
    },
    OwnedRow {
        stage: PreloopStageBCarrierActivationStageV1,
        cause: PreloopStageBCarrierActivationErrorV1,
    },
}

#[derive(Debug)]
pub(crate) struct PreloopStageBSourceInventoryErrorV1 {
    stage: PreloopStageBSourceInventoryStageV1,
    caller: Option<CanonicalSameModuleCallableKeyV1>,
    outer_site: Option<SourceExprSiteV1>,
    cause: PreloopStageBSourceInventoryCauseV1,
}

impl PreloopStageBSourceInventoryErrorV1 {
    pub(super) fn new(
        stage: PreloopStageBSourceInventoryStageV1,
        caller: Option<CanonicalSameModuleCallableKeyV1>,
        outer_site: Option<SourceExprSiteV1>,
        cause: PreloopStageBSourceInventoryCauseV1,
    ) -> Self {
        Self {
            stage,
            caller,
            outer_site,
            cause,
        }
    }

    pub(crate) const fn stage(&self) -> PreloopStageBSourceInventoryStageV1 {
        self.stage
    }

    pub(crate) const fn caller(&self) -> Option<&CanonicalSameModuleCallableKeyV1> {
        self.caller.as_ref()
    }

    pub(crate) const fn outer_site(&self) -> Option<&SourceExprSiteV1> {
        self.outer_site.as_ref()
    }

    pub(crate) const fn cause(&self) -> &PreloopStageBSourceInventoryCauseV1 {
        &self.cause
    }

    pub(crate) fn discard(self) {}
}
