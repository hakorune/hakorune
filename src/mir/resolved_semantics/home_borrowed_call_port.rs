//! Requests through the same source-flow argument callback, with explicit roles.
use super::{BorrowedCallActualCandidateV1, LocalCallArgumentV1, QualifiedStaticCallClaimV1};
#[derive(Debug, Clone, Copy)]
pub(crate) enum BorrowedCallActualRequestV1<'a> {
    Observe(&'a [BorrowedCallActualCandidateV1]),
    ScalarArguments,
    QualifiedStaticSourceArguments(&'a QualifiedStaticCallClaimV1),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedCallArgumentsV1 {
    Scalar(Box<[LocalCallArgumentV1]>),
    StaticSource(Box<[LocalCallArgumentV1]>),
}
