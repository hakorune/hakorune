use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{ShadowResolveErrorV0, SourceExprSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableResultActivationErrorV1 {
    BorrowedResultCatalogBrandMismatch,
    ActivationRowsCatalogBrandMismatch,
    MethodCallInventory {
        caller: CanonicalSameModuleCallableKeyV1,
        error: ShadowResolveErrorV0,
    },
    SourceTargetRowOutsideInventory {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    SelectedTargetMustBeStatic {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    },
    RequiredCallArgumentOutOfRange {
        target: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        ordinal: u32,
        arity: u32,
    },
    StaticSourceTargetEvidenceMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    },
    CalleeRequiredArgumentOrdinalMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
        target_required_i64_arguments: Box<[u32]>,
        evidence_required_i64_arguments: Box<[u32]>,
    },
}
