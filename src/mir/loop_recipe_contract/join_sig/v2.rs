//! Typed V2 issuer over the one neutral JoinSig engine.

use super::super::typed_schema_v2::VerifiedLoopRecipeV2;
use super::flow::elaborate_view;
use super::model::{LoopJoinSigRejectReasonV1, VerifiedLoopJoinSigV2};
use super::recipe_view_v2::LoopRecipeV2JoinView;

pub(crate) struct LoopJoinSigElaboratorV2;

impl LoopJoinSigElaboratorV2 {
    pub(crate) fn elaborate(
        verified: &VerifiedLoopRecipeV2,
    ) -> Result<VerifiedLoopJoinSigV2, LoopJoinSigRejectReasonV1> {
        elaborate_view(&LoopRecipeV2JoinView::verified(verified))
    }
}
