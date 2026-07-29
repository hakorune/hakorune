//! Final collector rows retain the admission fact that produced each live draft.

use crate::mir::MirFunction;

use super::receipt::CollectedDraftReplacementDispositionV1;
use super::{DraftPublicationPolicyV1, FunctionDraftKeyV1};

#[derive(Debug)]
pub(super) struct CollectedFunctionDraftV1 {
    pub(super) draft: MirFunction,
    pub(super) admission: CollectedDraftFinalAdmissionV1,
}

#[derive(Debug)]
pub(super) struct CollectedDraftFinalAdmissionV1 {
    pub(super) key: FunctionDraftKeyV1,
    pub(super) symbol: Box<str>,
    pub(super) arity: usize,
    pub(super) policy: DraftPublicationPolicyV1,
    pub(super) replacement: CollectedDraftReplacementDispositionV1,
}

impl CollectedDraftFinalAdmissionV1 {
    pub(super) fn new(
        key: FunctionDraftKeyV1,
        symbol: Box<str>,
        arity: usize,
        policy: DraftPublicationPolicyV1,
        replacement: CollectedDraftReplacementDispositionV1,
    ) -> Self {
        Self {
            key,
            symbol,
            arity,
            policy,
            replacement,
        }
    }
}
