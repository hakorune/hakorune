//! Exact source/physical correspondence for the selected outer carrier Call.
//!
//! This box consumes the complete owned Stage-B body recipe together with the
//! located outer completion. It does not inspect MIR, infer a result, assign a
//! variable, publish a type, or activate a production caller.

use crate::mir::preloop_stageb_carrier::PreparedPreloopStageBFunctionBodyRecipeV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::ValueId;

use super::preloop_located_outer_completion::CompletedPreloopLocatedOuterRequestV1;
use super::preloop_nested_result_receipt::OwnedPreloopOuterPhysicalPartsV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopOuterCarrierCorrespondenceStageV1 {
    Caller,
    OuterSite,
    SelectedArgument,
    InnerSite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopOuterCarrierCorrespondenceErrorV1 {
    CallerMismatch,
    OuterSiteMismatch,
    SelectedArgumentMismatch,
    InnerSiteMismatch,
}

#[derive(Debug)]
pub(super) struct RejectedPreloopOuterCarrierCallV1<'site, 'view, 'catalog> {
    physical: CompletedPreloopLocatedOuterRequestV1<'site, 'view, 'catalog>,
    recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
    stage: PreloopOuterCarrierCorrespondenceStageV1,
    cause: PreloopOuterCarrierCorrespondenceErrorV1,
}

#[derive(Debug)]
pub(super) struct OwnedPreloopOuterCarrierPartsV1 {
    pub(super) physical: OwnedPreloopOuterPhysicalPartsV1,
    pub(super) recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
}

impl OwnedPreloopOuterCarrierPartsV1 {
    pub(super) fn discard(self) {
        self.physical.discard();
        let _ = self.recipe;
    }
}

#[derive(Debug)]
pub(super) struct OwnedRejectedPreloopOuterCarrierCallV1 {
    owner: OwnedPreloopOuterCarrierPartsV1,
    stage: PreloopOuterCarrierCorrespondenceStageV1,
    cause: PreloopOuterCarrierCorrespondenceErrorV1,
}

impl RejectedPreloopOuterCarrierCallV1<'_, '_, '_> {
    pub(super) const fn stage(&self) -> PreloopOuterCarrierCorrespondenceStageV1 {
        self.stage
    }

    pub(super) const fn cause(&self) -> PreloopOuterCarrierCorrespondenceErrorV1 {
        self.cause
    }

    pub(super) fn bounded_report(&self) -> String {
        format!("[preloop-outer-carrier/{:?}] {:?}", self.stage, self.cause)
    }

    pub(super) fn discard(self) {
        self.physical.discard();
        let _ = self.recipe;
    }

    pub(super) fn into_owned_rejection_v1(self) -> OwnedRejectedPreloopOuterCarrierCallV1 {
        OwnedRejectedPreloopOuterCarrierCallV1 {
            owner: OwnedPreloopOuterCarrierPartsV1 {
                physical: self.physical.into_owned_parts_v1(),
                recipe: self.recipe,
            },
            stage: self.stage,
            cause: self.cause,
        }
    }
}

impl OwnedRejectedPreloopOuterCarrierCallV1 {
    pub(super) const fn stage(&self) -> PreloopOuterCarrierCorrespondenceStageV1 {
        self.stage
    }

    pub(super) const fn cause(&self) -> PreloopOuterCarrierCorrespondenceErrorV1 {
        self.cause
    }

    pub(super) fn discard(self) {
        self.owner.discard();
    }
}

/// Complete owned Integer authority paired with both successful physical
/// Calls. The outer destination is projected only from the outer receipt.
#[derive(Debug)]
pub(super) struct CompletedPreloopOuterCarrierCallV1<'site, 'view, 'catalog> {
    physical: CompletedPreloopLocatedOuterRequestV1<'site, 'view, 'catalog>,
    recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
    _seal: CompletedPreloopOuterCarrierCallSealV1,
}

#[derive(Debug)]
struct CompletedPreloopOuterCarrierCallSealV1;

impl CompletedPreloopOuterCarrierCallV1<'_, '_, '_> {
    pub(super) const fn inner_destination(&self) -> ValueId {
        self.physical.inner_destination()
    }

    pub(super) const fn outer_destination(&self) -> ValueId {
        self.physical.outer_destination()
    }

    pub(super) const fn outer_site(&self) -> &SourceExprSiteV1 {
        self.recipe.outer_call_site()
    }

    pub(super) fn assignment_target(&self) -> &str {
        self.recipe.assignment_target().name()
    }

    pub(super) const fn result_is_integer(&self) -> bool {
        self.recipe.result().is_integer()
    }

    pub(super) fn discard(self) {
        self.physical.discard();
        let _ = self.recipe;
    }

    pub(super) fn into_owned_parts_v1(self) -> OwnedPreloopOuterCarrierPartsV1 {
        OwnedPreloopOuterCarrierPartsV1 {
            physical: self.physical.into_owned_parts_v1(),
            recipe: self.recipe,
        }
    }
}

pub(super) fn complete_preloop_outer_carrier_call_v1<'site, 'view, 'catalog>(
    physical: CompletedPreloopLocatedOuterRequestV1<'site, 'view, 'catalog>,
    recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
) -> Result<
    CompletedPreloopOuterCarrierCallV1<'site, 'view, 'catalog>,
    RejectedPreloopOuterCarrierCallV1<'site, 'view, 'catalog>,
> {
    if physical.caller() != recipe.caller() {
        return Err(reject(
            physical,
            recipe,
            PreloopOuterCarrierCorrespondenceStageV1::Caller,
            PreloopOuterCarrierCorrespondenceErrorV1::CallerMismatch,
        ));
    }
    if physical.outer_site() != recipe.outer_call_site() {
        return Err(reject(
            physical,
            recipe,
            PreloopOuterCarrierCorrespondenceStageV1::OuterSite,
            PreloopOuterCarrierCorrespondenceErrorV1::OuterSiteMismatch,
        ));
    }
    if physical.selected_index() != recipe.selected_argument_index() {
        return Err(reject(
            physical,
            recipe,
            PreloopOuterCarrierCorrespondenceStageV1::SelectedArgument,
            PreloopOuterCarrierCorrespondenceErrorV1::SelectedArgumentMismatch,
        ));
    }
    if physical.inner_site() != recipe.inner_call_site() {
        return Err(reject(
            physical,
            recipe,
            PreloopOuterCarrierCorrespondenceStageV1::InnerSite,
            PreloopOuterCarrierCorrespondenceErrorV1::InnerSiteMismatch,
        ));
    }
    Ok(CompletedPreloopOuterCarrierCallV1 {
        physical,
        recipe,
        _seal: CompletedPreloopOuterCarrierCallSealV1,
    })
}

fn reject<'site, 'view, 'catalog>(
    physical: CompletedPreloopLocatedOuterRequestV1<'site, 'view, 'catalog>,
    recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
    stage: PreloopOuterCarrierCorrespondenceStageV1,
    cause: PreloopOuterCarrierCorrespondenceErrorV1,
) -> RejectedPreloopOuterCarrierCallV1<'site, 'view, 'catalog> {
    RejectedPreloopOuterCarrierCallV1 {
        physical,
        recipe,
        stage,
        cause,
    }
}
