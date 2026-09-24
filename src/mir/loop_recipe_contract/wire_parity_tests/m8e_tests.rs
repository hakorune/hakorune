use super::*;

// ---------------------------------------------------------------------------
// S7B5 — M8E Generic residual wire-coverage cohort.
//
// The `.hako` entry `emit_m8e_generic_wire.hako` emits the canonical M8E
// artifact (provenance `generic_residual_v1`) for the bounded source
// profile fixed by `generic_residual_function_for_test`:
// `generic_residual_projection(i, limit) { loop(i < limit)
// { local tmp = 0; i = i + 1 } }` — loop at body item index 0. The
// comparison target is rebuilt through the producer's own issuer chain —
// the typed source map carries the resolver-bound
// `VerifiedLoopRootSourceV1` plus carrier/condition/body_rows/carrier_step
// into `generic_residual_recipe` — and the real
// `produce_generic_residual_recipe_v1` product (issued through its own
// policy-demand chain) anchors the semantic arm.
// ---------------------------------------------------------------------------

use super::super::generic_residual_producer::{
    generic_residual_recipe, produce_generic_residual_recipe_v1,
};
use super::super::generic_residual_producer_tests::{demand as m8e_demand, typed_map as m8e_typed_map};

const HAKO_M8E_WIRE_EMISSION: &str =
    include_str!("../fixtures/hako_loop_recipe_wire_m8e_v1.json");

/// The canonical M8E artifact, assembled by the same issuer calls
/// `produce_generic_residual_recipe_v1` makes for the recipe half —
/// `generic_residual_recipe(carrier, &condition, &body_rows,
/// &carrier_step)` -> `LoopRecipeVerifierV1::verify` -> the
/// map-carried `VerifiedLoopRootSourceV1::into_root_claim` — with
/// `GenericResidualV1` provenance.
fn m8e_rust_artifact() -> LoopRecipeArtifactV1 {
    let (source_root, _projection, carrier, condition, body_rows, carrier_step, _frame) =
        m8e_typed_map().into_parts();
    let recipe = generic_residual_recipe(carrier, &condition, &body_rows, &carrier_step);
    let verified_recipe =
        LoopRecipeVerifierV1::verify(recipe.clone()).expect("m8e recipe verifies");
    let source_binding = source_root.into_root_claim(&verified_recipe);
    LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::GenericResidualV1),
        source_binding,
        recipe,
    )
}

#[test]
fn m8e_hako_emission_verifies_and_matches_rust_producer_artifact() {
    let hako_verified = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8E_WIRE_EMISSION)
        .expect("m8e .hako emission decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8e_rust_artifact()).expect("m8e rust verifies");

    assert_eq!(
        LoopRecipeNormalizerV1::normalize_artifact(&hako_verified)
            .expect("hako normalize_artifact"),
        LoopRecipeNormalizerV1::normalize_artifact(&rust_verified)
            .expect("rust normalize_artifact"),
    );
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_semantic(hako_verified.recipe())
            .expect("hako normalize_semantic"),
        LoopRecipeNormalizerV1::normalize_semantic(rust_verified.recipe())
            .expect("rust normalize_semantic"),
    );
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_source_bound(&hako_verified)
            .expect("hako normalize_source_bound"),
        LoopRecipeNormalizerV1::normalize_source_bound(&rust_verified)
            .expect("rust normalize_source_bound"),
    );
}

#[test]
fn m8e_reconstructed_artifact_matches_real_producer_product() {
    let reconstructed = LoopRecipeVerifierV1::verify_artifact(m8e_rust_artifact())
        .expect("m8e artifact verifies");
    let product =
        produce_generic_residual_recipe_v1(m8e_demand()).expect("real producer seals");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_semantic(reconstructed.recipe())
            .expect("reconstructed normalize_semantic"),
        LoopRecipeNormalizerV1::normalize_semantic(product.recipe())
            .expect("producer normalize_semantic"),
    );
}

#[test]
fn m8e_hako_emission_is_single_line_generic_provenance_with_two_carriers() {
    let trimmed = HAKO_M8E_WIRE_EMISSION.trim_end();
    assert!(!trimmed.contains('\n'), "m8e emission is one compact line");
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(trimmed).expect("m8e emission decodes as artifact");
    assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V1);
    assert_eq!(
        artifact.provenance.producer_id,
        LoopRecipeProducerIdV1::GenericResidualV1
    );
    assert_eq!(artifact.recipe.exits.len(), 0);
    assert_eq!(artifact.recipe.bindings.len(), 2);
    assert_eq!(artifact.recipe.carriers.len(), 2);
    assert_eq!(artifact.recipe.inputs.len(), 2);
}

#[test]
fn m8e_hako_emission_normalization_is_deterministic() {
    let first = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8E_WIRE_EMISSION)
        .expect("first m8e decode verifies");
    let second = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8E_WIRE_EMISSION)
        .expect("second m8e decode verifies");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_artifact(&first).expect("first normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&second).expect("second normalize"),
    );
}

#[test]
fn m8e_wire_with_foreign_provenance_is_semantic_drift() {
    let mut value: serde_json::Value =
        serde_json::from_str(HAKO_M8E_WIRE_EMISSION).expect("m8e fixture is JSON");
    *value
        .get_mut("provenance")
        .and_then(|p| p.get_mut("producer_id"))
        .expect("producer_id") = serde_json::json!("loop_cond_break_continue_v1");
    let json = serde_json::to_string(&value).expect("encodes");
    let drifted = LoopRecipeNormalizerV1::decode_and_verify(&json)
        .expect("drifted artifact still decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8e_rust_artifact()).expect("m8e rust verifies");
    assert_ne!(
        LoopRecipeNormalizerV1::normalize_artifact(&drifted).expect("drifted normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&rust_verified).expect("rust normalize"),
    );
}
