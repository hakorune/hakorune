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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SelectedDynamicW6RootPreflightErrorV1 {
    Rejected,
}

/// The root-only result after the R4 policy/census callback has succeeded.
///
/// This is a typestate transition, not an old-edge witness: no census value,
/// manifest field, or callback-owned state is retained.  The final commit and
/// selected-caller transition remain deliberately unavailable here.
#[derive(Debug)]
pub(crate) struct PreparedSelectedDynamicW6RootReadyV1<'a> {
    activation: PreparedSelectedDynamicW6ActivationV1<'a>,
    _seal: PreparedSelectedDynamicW6RootReadySealV1,
}

#[derive(Debug)]
struct PreparedSelectedDynamicW6RootReadySealV1;

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

    /// Consume the candidate exactly once after the root has directly
    /// checked the R4 policy and current old-edge/compatibility census.
    ///
    /// The callback is the root census owner.  Its result is intentionally
    /// not stored, so this boundary cannot become a second retirement witness
    /// or a child-transport authority.  A failed check consumes/discards the
    /// unpublished candidate and never exposes a commit capability.
    pub(crate) fn consume_after_root_r4_preflight(
        self,
        preflight: impl FnOnce(
            &crate::mir::MirModule,
        ) -> Result<(), SelectedDynamicW6RootPreflightErrorV1>,
    ) -> Result<PreparedSelectedDynamicW6RootReadyV1<'a>, SelectedDynamicW6RootPreflightErrorV1>
    {
        let Self {
            candidate,
            _receipt,
            _seal,
        } = self;
        candidate
            .with_candidate_module(preflight)
            .map(|()| PreparedSelectedDynamicW6RootReadyV1 {
                activation: Self {
                    candidate,
                    _receipt,
                    _seal,
                },
                _seal: PreparedSelectedDynamicW6RootReadySealV1,
            })
    }

    /// Explicit pre-cutover discard terminal.  Commit is intentionally absent
    /// until the root R4 preflight and selected callback transition are wired.
    pub(crate) fn discard(self) {}
}

impl<'a> PreparedSelectedDynamicW6RootReadyV1<'a> {
    /// Keep the candidate opaque while the final commit terminal is still
    /// closed.  This borrow cannot escape and does not clone the module.
    pub(crate) fn with_candidate_module<R>(
        &self,
        observe: impl for<'module> FnOnce(&'module crate::mir::MirModule) -> R,
    ) -> R {
        self.activation.with_candidate_module(observe)
    }

    /// Explicitly discard a preflighted but unpublished candidate.
    pub(crate) fn discard(self) {}
}
