//! W6 root-only candidate/fence co-seal.
//!
//! This product is intentionally pre-cutover.  It reuses the existing
//! external-commit owner and remembers only that the root artifact validator
//! succeeded.  No receipt fields, old-edge witness, or mutable Builder state
//! are copied into this aggregate.

use super::external_commit::PreparedModuleExternalCommitV1;

#[derive(Debug)]
pub(crate) struct StaticArtifactReceiptConsumedFenceV1 {
    _seal: StaticArtifactReceiptConsumedFenceSealV1,
}

#[derive(Debug)]
struct StaticArtifactReceiptConsumedFenceSealV1;

impl StaticArtifactReceiptConsumedFenceV1 {
    /// The root receipt validator is the sole issuer of this physical fence.
    /// A guard keeps this constructor's production caller count at one.
    pub(crate) fn issue_from_root_validator() -> Self {
        Self {
            _seal: StaticArtifactReceiptConsumedFenceSealV1,
        }
    }

    #[cfg(test)]
    pub(crate) fn issue_for_test() -> Self {
        Self {
            _seal: StaticArtifactReceiptConsumedFenceSealV1,
        }
    }
}

#[derive(Debug)]
pub(crate) struct PreparedSelectedDynamicW6ActivationV1<'a> {
    candidate: PreparedModuleExternalCommitV1<'a>,
    _receipt: StaticArtifactReceiptConsumedFenceV1,
    _seal: PreparedSelectedDynamicW6ActivationSealV1,
}

#[derive(Debug)]
struct PreparedSelectedDynamicW6ActivationSealV1;

impl<'a> PreparedSelectedDynamicW6ActivationV1<'a> {
    /// Co-seal one prepared candidate with one root-validated artifact fence.
    /// This constructor does not publish, switch callbacks, or retire edges.
    pub(crate) fn prepare(
        candidate: PreparedModuleExternalCommitV1<'a>,
        receipt: StaticArtifactReceiptConsumedFenceV1,
    ) -> Self {
        Self {
            candidate,
            _receipt: receipt,
            _seal: PreparedSelectedDynamicW6ActivationSealV1,
        }
    }

    /// Borrow the candidate module without cloning or exposing Builder state.
    pub(crate) fn with_candidate_module<R>(
        &self,
        observe: impl for<'module> FnOnce(&'module crate::mir::MirModule) -> R,
    ) -> R {
        self.candidate.with_candidate_module(observe)
    }

    /// Explicit pre-cutover discard terminal.  Commit is intentionally absent
    /// until the root R4 preflight and selected callback transition are wired.
    pub(crate) fn discard(self) {}
}
