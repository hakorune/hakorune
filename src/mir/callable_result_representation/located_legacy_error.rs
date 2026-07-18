use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::SourceNodeSiteV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableResultLegacyLocationErrorV1 {
    UnknownCaller(CanonicalSameModuleCallableKeyV1),
    ForeignCarrier {
        expected: CanonicalSameModuleCallableKeyV1,
        actual: CanonicalSameModuleCallableKeyV1,
    },
    BodyIndexOverflow {
        index: usize,
    },
    BodyIndexOutOfBounds {
        body: Option<SourceNodeSiteV1>,
        index: u32,
        len: usize,
    },
    BodySuffixIndexOverflow {
        index: usize,
    },
    BodySuffixLengthOverflow {
        len: usize,
    },
    BodySuffixStartOutOfBounds {
        body: Option<SourceNodeSiteV1>,
        start: u32,
        len: usize,
    },
    StatementIsNotExpression(SourceNodeSiteV1),
    ExpressionRoleParentMismatch(SourceNodeSiteV1),
    ExpressionRoleHasNoSyntaxNode(SourceNodeSiteV1),
    BodyRoleParentMismatch(SourceNodeSiteV1),
    RootBodyRequestedAsChild(SourceNodeSiteV1),
    UnlocatedCannotClaimActivation,
    UnlocatedCannotProveInactive,
}
