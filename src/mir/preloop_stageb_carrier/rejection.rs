use crate::mir::callable_result_representation::VerifiedStaticExactI64RequirementV1;
use crate::mir::source_instance_result_contract::PreparedPreloopLocatedArgumentV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreloopOuterCarrierResultContractStageV1 {
    CatalogAllocation,
    Caller,
    OuterSite,
    SelectedArgument,
    RequiredArguments,
    InnerContract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PreloopOuterCarrierResultContractErrorV1 {
    ForeignCatalog,
    CallerMismatch,
    OuterSiteMismatch,
    SelectedArgumentMismatch { expected: u32, actual: u32 },
    RequiredArgumentsMismatch { selected: u32, actual: Box<[u32]> },
    InnerContractCatalogMismatch,
}

#[derive(Debug)]
pub(crate) struct RejectedPreloopOuterCarrierResultContractV1<'result, 'site, 'view, 'catalog> {
    requirement: VerifiedStaticExactI64RequirementV1<'result, 'catalog>,
    prepared: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    stage: PreloopOuterCarrierResultContractStageV1,
    cause: PreloopOuterCarrierResultContractErrorV1,
}

impl<'result, 'site, 'view, 'catalog>
    RejectedPreloopOuterCarrierResultContractV1<'result, 'site, 'view, 'catalog>
{
    pub(super) fn new(
        requirement: VerifiedStaticExactI64RequirementV1<'result, 'catalog>,
        prepared: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        stage: PreloopOuterCarrierResultContractStageV1,
        cause: PreloopOuterCarrierResultContractErrorV1,
    ) -> Self {
        Self {
            requirement,
            prepared,
            stage,
            cause,
        }
    }

    pub(crate) const fn stage(&self) -> PreloopOuterCarrierResultContractStageV1 {
        self.stage
    }

    pub(crate) const fn cause(&self) -> &PreloopOuterCarrierResultContractErrorV1 {
        &self.cause
    }

    pub(crate) fn discard(self) {
        let Self {
            requirement,
            prepared,
            ..
        } = self;
        let _ = (requirement, prepared);
    }
}
