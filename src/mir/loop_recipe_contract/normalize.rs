//! Deterministic JSON normalization for the portable Loop recipe wire.

use super::error::LoopRecipeRejectReasonV1;
use super::schema::LoopRecipeArtifactV1;
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
    pub(crate) fn decode_and_verify(
        json: &str,
    ) -> Result<VerifiedLoopRecipeArtifactV1, LoopRecipeDecodeErrorV1> {
        let artifact: LoopRecipeArtifactV1 = serde_json::from_str(json)?;
        Ok(LoopRecipeVerifierV1::verify_artifact(artifact)?)
    }

    pub(crate) fn normalize_artifact(
        verified: &VerifiedLoopRecipeArtifactV1,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(&LoopRecipeArtifactV1 {
            schema_version: super::schema::LOOP_RECIPE_SCHEMA_VERSION_V1,
            provenance: verified.provenance().clone(),
            recipe: verified.recipe().as_recipe().clone(),
        })
    }

    /// Semantic parity excludes producer-route provenance; typed Loop source
    /// paths remain part of the semantic recipe.
    pub(crate) fn normalize_semantic(
        verified: &VerifiedLoopRecipeV1,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(verified.as_recipe())
    }
}
