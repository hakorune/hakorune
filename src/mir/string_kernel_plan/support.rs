use super::{StringKernelPlanBorrowContract, StringKernelPlanPublicationContract};
use crate::mir::{
    string_corridor::StringCorridorBorrowContract,
    string_corridor_placement::{StringCorridorCandidateKind, StringCorridorPublicationContract},
};

pub(super) fn candidate_priority(kind: StringCorridorCandidateKind) -> u8 {
    match kind {
        StringCorridorCandidateKind::DirectKernelEntry => 0,
        StringCorridorCandidateKind::PublicationSink => 1,
        StringCorridorCandidateKind::MaterializationSink => 2,
        StringCorridorCandidateKind::BorrowCorridorFusion => 3,
    }
}

pub(super) fn publication_contract_from_plan(
    plan: crate::mir::string_corridor_placement::StringCorridorCandidatePlan,
) -> Option<StringKernelPlanPublicationContract> {
    match plan.publication_contract {
        Some(
            StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        ) => Some(
            StringKernelPlanPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        ),
        None => None,
    }
}

pub(super) fn borrow_contract_from_plan(
    plan: crate::mir::string_corridor_placement::StringCorridorCandidatePlan,
) -> Option<StringKernelPlanBorrowContract> {
    match plan.borrow_contract {
        Some(StringCorridorBorrowContract::BorrowTextFromObject) => {
            Some(StringKernelPlanBorrowContract::BorrowTextFromObject)
        }
        None => None,
    }
}
