//! One-shot verified Recipe/JoinSig pairing for a physical consumer.
//!
//! The pair is produced by a verified recipe producer, not by a physicalizer.
//! Keeping the pair owned and non-Clone prevents a consumer from accepting a
//! recipe and a JoinSig that were verified from different semantic products.

use super::direct_accum_producer::VerifiedDirectAccumRecipeProductV1;
use super::join_sig::VerifiedLoopJoinSigV1;
use super::verify::VerifiedLoopRecipeV1;

#[derive(Debug)]
pub(crate) struct VerifiedLoopPhysicalInputV1 {
    recipe: VerifiedLoopRecipeV1,
    join_sig: VerifiedLoopJoinSigV1,
}

impl VerifiedLoopPhysicalInputV1 {
    pub(crate) fn from_direct_accum(
        product: VerifiedDirectAccumRecipeProductV1,
    ) -> Self {
        let (recipe, join_sig) = product.into_parts();
        Self { recipe, join_sig }
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
