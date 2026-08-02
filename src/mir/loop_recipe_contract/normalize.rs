//! Deterministic JSON normalization for the portable Loop recipe wire.

use super::error::LoopRecipeRejectReasonV1;
use serde::Serialize;

use super::schema::{
    LoopRecipeArtifactV1, LoopRecipeSourceBindingV1, LoopRecipeV1, LOOP_RECIPE_SCHEMA_VERSION_V1,
};
use super::verify::{LoopRecipeVerifierV1, VerifiedLoopRecipeArtifactV1, VerifiedLoopRecipeV1};

#[derive(Debug)]
pub(crate) enum LoopRecipeDecodeErrorV1 {
    Json(serde_json::Error),
    Rejected(LoopRecipeRejectReasonV1),
}

impl From<serde_json::Error> for LoopRecipeDecodeErrorV1 {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<LoopRecipeRejectReasonV1> for LoopRecipeDecodeErrorV1 {
    fn from(value: LoopRecipeRejectReasonV1) -> Self {
        Self::Rejected(value)
    }
}

pub(crate) struct LoopRecipeNormalizerV1;

impl LoopRecipeNormalizerV1 {
    pub(super) fn decode_and_verify(
        json: &str,
    ) -> Result<VerifiedLoopRecipeArtifactV1, LoopRecipeDecodeErrorV1> {
        let artifact: LoopRecipeArtifactV1 = serde_json::from_str(json)?;
        Ok(LoopRecipeVerifierV1::verify_artifact(artifact)?)
    }

    pub(super) fn normalize_artifact(
        verified: &VerifiedLoopRecipeArtifactV1,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(&LoopRecipeArtifactV1 {
            schema_version: LOOP_RECIPE_SCHEMA_VERSION_V1,
            provenance: verified.provenance().clone(),
            source_binding: verified.source_binding().as_source_binding().clone(),
            recipe: verified.recipe().as_recipe().clone(),
        })
    }

    /// Semantic parity excludes producer-route provenance and source binding.
    pub(crate) fn normalize_semantic(
        verified: &VerifiedLoopRecipeV1,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(verified.as_recipe())
    }

    /// Source-bound parity includes claimed wire source coordinates but
    /// excludes the route receipt that produced the recipe.
    pub(super) fn normalize_source_bound(
        verified: &VerifiedLoopRecipeArtifactV1,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(&LoopRecipeSourceBoundViewV1 {
            schema_version: LOOP_RECIPE_SCHEMA_VERSION_V1,
            source_binding: verified.source_binding().as_source_binding(),
            recipe: verified.recipe().as_recipe(),
        })
    }
}

#[derive(Serialize)]
struct LoopRecipeSourceBoundViewV1<'a> {
    schema_version: u16,
    source_binding: &'a LoopRecipeSourceBindingV1,
    recipe: &'a LoopRecipeV1,
}
