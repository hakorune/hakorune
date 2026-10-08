//! Requests through the same source-flow argument callback, with explicit roles.
use super::{BorrowedCallActualCandidateV1, LocalCallArgumentV1, QualifiedStaticCallClaimV1};
#[derive(Debug, Clone, Copy)]
pub(crate) enum BorrowedCallActualRequestV1<'a> {
    Observe(&'a [BorrowedCallActualCandidateV1]),
    ScalarArguments,
    I64ResultArguments,
    ReceivedHandleArguments(crate::mir::resolved_semantics::BindingRefV1),
    ObjectArguments(
        &'a crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1,
        Option<crate::mir::resolved_semantics::BindingRefV1>,
    ),
    QualifiedStaticSourceArguments(&'a QualifiedStaticCallClaimV1),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedCallArgumentsV1 {
    Scalar(Box<[LocalCallArgumentV1]>),
    StaticSource(Box<[LocalCallArgumentV1]>),
    /// Source observation only; no result, entry or executable permission.
    HandleSource(Box<[LocalCallArgumentV1]>),
    Object {
        qualification:
            crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1,
        arguments: ObjectCallSourceSupportV1,
    },
}

/// Retained source availability only; never a Normal or physical permission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ObjectCallSourceSupportV1 {
    Observed(Box<[LocalCallArgumentV1]>),
    SourceOnly(Box<[LocalCallArgumentV1]>),
    Unavailable,
}
