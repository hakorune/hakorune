//! Deterministic JSON normalization for the portable Loop recipe wire.

use serde::Serialize;

use super::schema::LoopRecipeSourceBindingV1;

use super::schema_v2::{LoopRecipeArtifactV2, LoopRecipeV2, LOOP_RECIPE_SCHEMA_VERSION_V2};
use super::typed_schema_v2::{
    LoopRecipeV2RejectReason, LoopRecipeVerifierV2, VerifiedLoopRecipeArtifactV2,
    VerifiedLoopRecipeV2,
};

#[derive(Debug)]
pub(crate) enum LoopRecipeDecodeErrorV2 {
    Json(serde_json::Error),
    Rejected(LoopRecipeV2RejectReason),
}

impl From<serde_json::Error> for LoopRecipeDecodeErrorV2 {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<LoopRecipeV2RejectReason> for LoopRecipeDecodeErrorV2 {
    fn from(value: LoopRecipeV2RejectReason) -> Self {
        Self::Rejected(value)
    }
}

pub(crate) struct LoopRecipeNormalizerV2;

impl LoopRecipeNormalizerV2 {
    pub(crate) fn decode_and_verify(
        json: &str,
    ) -> Result<VerifiedLoopRecipeArtifactV2, LoopRecipeDecodeErrorV2> {
        let artifact: LoopRecipeArtifactV2 = serde_json::from_str(json)?;
        Ok(LoopRecipeVerifierV2::verify_artifact(artifact)?)
    }

    pub(crate) fn normalize_artifact(
        verified: &VerifiedLoopRecipeArtifactV2,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(&LoopRecipeArtifactV2 {
            schema_version: LOOP_RECIPE_SCHEMA_VERSION_V2,
            provenance: verified.provenance().clone(),
            source_binding: verified.source_binding().clone(),
            recipe: verified.recipe().as_recipe().clone(),
        })
    }

    /// Semantic parity excludes producer-route provenance and source binding.
    pub(crate) fn normalize_semantic(
        verified: &VerifiedLoopRecipeV2,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(verified.as_recipe())
    }

    /// Source-bound parity includes claimed wire source coordinates but
    /// excludes the route receipt that produced the recipe.
    pub(crate) fn normalize_source_bound(
        verified: &VerifiedLoopRecipeArtifactV2,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(&LoopRecipeSourceBoundViewV2 {
            schema_version: LOOP_RECIPE_SCHEMA_VERSION_V2,
            source_binding: verified.source_binding(),
            recipe: verified.recipe().as_recipe(),
        })
    }
}

#[derive(Serialize)]
struct LoopRecipeSourceBoundViewV2<'a> {
    schema_version: u16,
    source_binding: &'a LoopRecipeSourceBindingV1,
    recipe: &'a LoopRecipeV2,
}
