//! One-shot verified Recipe/JoinSig pairing for a physical consumer.
//!
//! The pair is produced by a verified recipe producer, not by a physicalizer.
//! Keeping the pair owned and non-Clone prevents a consumer from accepting a
//! recipe and a JoinSig that were verified from different semantic products.

use super::direct_accum_producer::VerifiedDirectAccumRecipeProductV1;
use super::join_sig::{VerifiedLoopAfterBindingV1, VerifiedLoopJoinSigV1};
use super::source_bound_core::{
    VerifiedLoopCoreProductV1, VerifiedLoopRecipeBindingRelationV1,
};
use super::verify::VerifiedLoopRecipeV1;

#[derive(Debug)]
pub(crate) struct VerifiedLoopPhysicalInputV1 {
    recipe: VerifiedLoopRecipeV1,
    join_sig: VerifiedLoopJoinSigV1,
}

impl VerifiedLoopPhysicalInputV1 {
    /// Extracts the verified Recipe/JoinSig pair plus the Recipe-layer binding
    /// relations, which remain the sole owner of the `LoopBindingKeyV1` ->
    /// `BindingRefV1` mapping for the resolved identity adapter.
    pub(crate) fn from_direct_accum_with_relations(
        product: VerifiedDirectAccumRecipeProductV1,
    ) -> (Self, Box<[VerifiedLoopRecipeBindingRelationV1]>) {
        let (operations, _inputs) = product.into_parts();
        let (core, _evidence) = operations.into_parts();
        let (_owner, recipe, join_sig, _claim, bindings, _effects) = core.into_parts();
        (Self { recipe, join_sig }, bindings)
    }

    pub(crate) fn recipe(&self) -> &VerifiedLoopRecipeV1 {
        &self.recipe
    }

    pub(crate) fn join_sig(&self) -> &VerifiedLoopJoinSigV1 {
        &self.join_sig
    }

    pub(crate) fn into_parts(self) -> (VerifiedLoopRecipeV1, VerifiedLoopJoinSigV1) {
        (self.recipe, self.join_sig)
    }
}

/// Neutral move-only handoff for the common topology physicalizer.
///
/// The Core remains the sole source/effect relation owner; `after` is the
/// already-issued logical continuation capability. This wrapper adds no
/// Recipe, CFG, SSA, PHI, or profile authority.
#[derive(Debug)]
pub(crate) struct VerifiedLoopPhysicalBoundaryV1 {
    core: VerifiedLoopCoreProductV1,
    after: VerifiedLoopAfterBindingV1,
}

impl VerifiedLoopPhysicalBoundaryV1 {
    pub(crate) fn from_parts(
        core: VerifiedLoopCoreProductV1,
        after: VerifiedLoopAfterBindingV1,
    ) -> Self {
        Self { core, after }
    }

    pub(crate) fn core(&self) -> &VerifiedLoopCoreProductV1 {
        &self.core
    }

    pub(crate) fn recipe(&self) -> &VerifiedLoopRecipeV1 {
        self.core.recipe()
    }

    pub(crate) fn join_sig(&self) -> &VerifiedLoopJoinSigV1 {
        self.core.join_sig()
    }

    pub(crate) fn after(&self) -> &VerifiedLoopAfterBindingV1 {
        &self.after
    }

    pub(crate) fn into_parts(self) -> (VerifiedLoopCoreProductV1, VerifiedLoopAfterBindingV1) {
        (self.core, self.after)
    }
}
