//! Consuming Script-family handoff to the shared source-result recipe.
//!
//! This boundary owns no Builder, physical entry, publication, or Raw
//! invocation state. It only turns the one sealed Script source family into
//! the existing source-classified `RawScriptBodyRecipeV1` vocabulary.

use crate::mir::compiler::raw_root_source_facts::{
    project_raw_script_body_recipe_v1, RawScriptRecipeProjectionErrorV1,
};
use crate::mir::raw_root_body_recipe::RawScriptBodyRecipeV1;

use super::product::{
    NormalTopLevelSiteV1, PreparedNormalSourcePlanInputV1, SealedNormalScriptSourceV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalScriptRecipeStageV1 {
    SourceProjection,
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalScriptRecipeV1 {
    source: RetainedNormalScriptSourceV1,
    recipe: RawScriptBodyRecipeV1,
    _seal: VerifiedNormalScriptRecipeSealV1,
}

#[derive(Debug)]
struct VerifiedNormalScriptRecipeSealV1;

/// Opaque source retention paired with the exact Script recipe. It has no AST
/// accessor or reclassification terminal: source observation ended before the
/// recipe was issued.
#[derive(Debug)]
pub(crate) struct RetainedNormalScriptSourceV1 {
    input: PreparedNormalSourcePlanInputV1,
    statements: Box<[NormalTopLevelSiteV1]>,
    _seal: RetainedNormalScriptSourceSealV1,
}

#[derive(Debug)]
struct RetainedNormalScriptSourceSealV1;

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
    let source = RetainedNormalScriptSourceV1::from_sealed(source);
    Ok(VerifiedNormalScriptRecipeV1 {
        recipe,
        source,
        _seal: VerifiedNormalScriptRecipeSealV1,
    })
}

impl VerifiedNormalScriptRecipeV1 {
    pub(in crate::mir) fn recipe(&self) -> &RawScriptBodyRecipeV1 {
        &self.recipe
    }

    pub(in crate::mir) fn source_identity(&self) -> &str {
        self.source.input.identity().display_name()
    }

    pub(in crate::mir) fn into_recipe(self) -> RawScriptBodyRecipeV1 {
        self.recipe
    }

    pub(crate) fn into_physical_parts(
        self,
    ) -> (RetainedNormalScriptSourceV1, RawScriptBodyRecipeV1) {
        (self.source, self.recipe)
    }

    #[cfg(test)]
    pub(crate) fn retained_source_statement_count(&self) -> usize {
        self.source.statements.len()
    }
}

impl RetainedNormalScriptSourceV1 {
    fn from_sealed(source: SealedNormalScriptSourceV1) -> Self {
        let (input, statements) = source.into_parts();
        Self {
            input,
            statements,
            _seal: RetainedNormalScriptSourceSealV1,
        }
    }
}

impl RejectedNormalScriptRecipeV1 {
    pub(crate) fn into_parts(
        self,
    ) -> (SealedNormalScriptSourceV1, RawScriptRecipeProjectionErrorV1) {
        (self.source, self.error)
    }

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
