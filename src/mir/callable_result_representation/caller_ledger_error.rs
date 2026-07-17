use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1};

use super::CallableResultLegacyLocationErrorV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableResultCallerLedgerErrorV1 {
    UnknownCaller(CanonicalSameModuleCallableKeyV1),
    LegacyLocation(CallableResultLegacyLocationErrorV1),
    ForeignPlan,
    ForeignCaller {
        expected: CanonicalSameModuleCallableKeyV1,
        actual: CanonicalSameModuleCallableKeyV1,
    },
    ClaimRequiresMethodCall {
        site: SourceExprSiteV1,
    },
    Duplicate {
        site: SourceExprSiteV1,
    },
    WrongOrder {
        expected: SourceExprSiteV1,
        actual: SourceExprSiteV1,
    },
    Unexpected {
        site: SourceExprSiteV1,
    },
    RowsUnderPrefix {
        prefix: Option<SourceNodeSiteV1>,
        first: SourceExprSiteV1,
    },
    Missing {
        site: SourceExprSiteV1,
        remaining: usize,
    },
}
