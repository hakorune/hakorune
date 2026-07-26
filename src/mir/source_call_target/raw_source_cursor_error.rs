use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourceStmtSiteV1};

/// Rejections while structurally navigating one catalog-owned Raw callable.
///
/// This is deliberately a source-only vocabulary: it owns no Builder, MIR,
/// route, or retry state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RawSourceCursorErrorV1 {
    CallerOutsideCatalog {
        caller: CanonicalSameModuleCallableKeyV1,
    },
    ForeignView {
        caller: CanonicalSameModuleCallableKeyV1,
    },
    SourceIndexOverflow {
        caller: CanonicalSameModuleCallableKeyV1,
        value: usize,
        role: &'static str,
    },
    BodyIndexOutOfBounds {
        caller: CanonicalSameModuleCallableKeyV1,
        index: u32,
        len: u32,
    },
    StatementExpressionRequired {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceStmtSiteV1,
    },
    MethodCallRequired {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    MethodCallArgumentIndexOutOfBounds {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        index: u32,
        len: u32,
    },
    ExpressionRoleParentMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceNodeSiteV1,
    },
    BodyRoleParentMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceNodeSiteV1,
    },
    ProjectionExpectedNode {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceNodeSiteV1,
    },
    ProjectionExpectedBody {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceNodeSiteV1,
    },
}
