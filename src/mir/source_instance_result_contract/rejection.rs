use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::CallableResultUnavailableReasonV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CurrentOwnerInstanceResultTargetErrorV1 {
    CanonicalMeReceiverRequired {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    CallerNotInstanceBoxMethod {
        caller: CanonicalSameModuleCallableKeyV1,
    },
    TargetOutsideCatalog {
        owner: Box<str>,
        method: Box<str>,
        arity: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedInstanceResultContractStageV1 {
    CoSeal,
    BodyProof,
    ResultClosure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NestedInstanceResultContractErrorV1 {
    TargetProofMismatch,
    BodyUnavailable(CallableResultUnavailableReasonV1),
    SealedDependencyPending,
    NonEmptyRequiredArguments {
        count: usize,
    },
}
