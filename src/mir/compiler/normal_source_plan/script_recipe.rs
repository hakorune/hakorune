//! Consuming Script-family handoff to the shared source-result recipe.
//!
//! This boundary owns no Builder, physical entry, publication, or Raw
//! invocation state. It only turns the one sealed Script source family into
//! the existing source-classified `RawScriptBodyRecipeV1` vocabulary.

use crate::mir::compiler::raw_root_source_facts::{
    project_raw_script_body_recipe_v1, RawScriptRecipeProjectionErrorV1,
};
use crate::mir::raw_root_body_recipe::RawRootBodyRecipeV1;

use super::product::SealedNormalScriptSourceV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalScriptRecipeStageV1 {
    SourceProjection,
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalScriptRecipeV1 {
    recipe: RawRootBodyRecipeV1,
    source_identity: Box<str>,
    _seal: VerifiedNormalScriptRecipeSealV1,
}

#[derive(Debug)]
struct VerifiedNormalScriptRecipeSealV1;

#[derive(Debug)]
pub(crate) struct RejectedNormalScriptRecipeV1 {
    source: SealedNormalScriptSourceV1,
    stage: NormalScriptRecipeStageV1,
    error: RawScriptRecipeProjectionErrorV1,
}

pub(super) fn prepare(
    source: SealedNormalScriptSourceV1,
) -> Result<VerifiedNormalScriptRecipeV1, RejectedNormalScriptRecipeV1> {
    let recipe = match project_raw_script_body_recipe_v1(source_input(&source)) {
        Ok(recipe) => recipe,
        Err(error) => {
            return Err(RejectedNormalScriptRecipeV1 {
                source,
                stage: NormalScriptRecipeStageV1::SourceProjection,
                error,
            })
        }
    };
    let input = source.into_input();
    let (source, identity) = input.into_parts();
    drop(source);
    Ok(VerifiedNormalScriptRecipeV1 {
        recipe,
        source_identity: identity.display_name().into(),
        _seal: VerifiedNormalScriptRecipeSealV1,
    })
}

impl VerifiedNormalScriptRecipeV1 {
    pub(in crate::mir) fn recipe(&self) -> &RawRootBodyRecipeV1 {
        &self.recipe
    }

    pub(in crate::mir) fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub(in crate::mir) fn into_recipe(self) -> RawRootBodyRecipeV1 {
        self.recipe
    }
}

impl RejectedNormalScriptRecipeV1 {
    pub(crate) const fn stage(&self) -> NormalScriptRecipeStageV1 {
        self.stage
    }

    pub(crate) fn error(&self) -> &RawScriptRecipeProjectionErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

fn source_input(source: &SealedNormalScriptSourceV1) -> &crate::ast::ASTNode {
    // The Script recipe is projected before the owner is consumed, allowing a
    // typed rejection to retain the exact source family without clone/retry.
    source.source_ast()
}
