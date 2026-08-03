//! Deterministic JSON normalization for the portable If recipe wire.

use serde::Serialize;

use super::error::IfRecipeRejectReasonV1;
use super::schema::{
    IfRecipeArtifactV1, IfRecipeSourceBindingV1, IfRecipeV1, IF_RECIPE_SCHEMA_VERSION_V1,
};
use super::verify::{IfRecipeVerifierV1, VerifiedIfRecipeArtifactV1, VerifiedIfRecipeV1};

#[derive(Debug)]
pub(crate) enum IfRecipeDecodeErrorV1 {
    Json(serde_json::Error),
    Rejected(IfRecipeRejectReasonV1),
}

impl From<serde_json::Error> for IfRecipeDecodeErrorV1 {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<IfRecipeRejectReasonV1> for IfRecipeDecodeErrorV1 {
    fn from(value: IfRecipeRejectReasonV1) -> Self {
        Self::Rejected(value)
    }
}

pub(crate) struct IfRecipeNormalizerV1;

impl IfRecipeNormalizerV1 {
    pub(crate) fn decode_and_verify(
        json: &str,
    ) -> Result<VerifiedIfRecipeArtifactV1, IfRecipeDecodeErrorV1> {
        let artifact: IfRecipeArtifactV1 = serde_json::from_str(json)?;
        Ok(IfRecipeVerifierV1::verify_artifact(artifact)?)
    }

    pub(crate) fn normalize_artifact(
        verified: &VerifiedIfRecipeArtifactV1,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(&IfRecipeArtifactV1 {
            schema_version: IF_RECIPE_SCHEMA_VERSION_V1,
            provenance: *verified.provenance(),
            source_binding: verified.source_binding().as_source_binding().clone(),
            recipe: verified.recipe().as_recipe().clone(),
        })
    }

    pub(crate) fn normalize_semantic(
        verified: &VerifiedIfRecipeV1,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(verified.as_recipe())
    }

    pub(crate) fn normalize_source_bound(
        verified: &VerifiedIfRecipeArtifactV1,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(&IfRecipeSourceBoundViewV1 {
            schema_version: IF_RECIPE_SCHEMA_VERSION_V1,
            source_binding: verified.source_binding().as_source_binding(),
            recipe: verified.recipe().as_recipe(),
        })
    }
}

#[derive(Serialize)]
struct IfRecipeSourceBoundViewV1<'a> {
    schema_version: u16,
    source_binding: &'a IfRecipeSourceBindingV1,
    recipe: &'a IfRecipeV1,
}
