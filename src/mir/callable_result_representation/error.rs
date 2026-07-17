use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableResultCatalogErrorV1 {
    RequiredArgumentOrdinalOutOfRange {
        key: CanonicalSameModuleCallableKeyV1,
        ordinal: u32,
        arity: u32,
    },
    CallArityOverflow {
        caller: CanonicalSameModuleCallableKeyV1,
        arity: usize,
    },
    ResultRowCardinalityMismatch {
        static_declarations: usize,
        rows: usize,
    },
    SourceTargetCatalogBrandMismatch,
    SourceTargetCallerOutsideResultCatalog {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    SourceTargetOutsideResultCatalog {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    },
    ResultWorklistDidNotConverge {
        static_declarations: usize,
    },
    StableResultDrift {
        key: CanonicalSameModuleCallableKeyV1,
    },
    DuplicateCallResultSite {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
}
