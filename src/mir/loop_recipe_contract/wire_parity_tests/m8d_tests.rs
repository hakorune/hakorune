use super::*;

// ---------------------------------------------------------------------------
// S7B4 — M8D LoopCond break/continue wire-coverage cohort.
//
// The `.hako` entry `emit_m8d_loopcond_wire.hako` emits the canonical M8D
// artifact (provenance `loop_cond_break_continue_v1`) for the bounded
// source profile fixed by `loop_cond_function_for_test`:
// `local flag = 1; loop(flag < 2) { if flag == 1 { break }
// else { continue } }` — loop at body item index 1. The comparison
// target is rebuilt through the producer's own issuer chain — the
// typed source map carries the resolver-bound `VerifiedLoopRootSourceV1`
// plus both typed compares into `loop_cond_break_continue_recipe` —
// and the real `produce_loop_cond_break_continue_recipe_v1` product
// (issued through its own policy-demand chain) anchors the semantic arm.
// ---------------------------------------------------------------------------

use super::super::loop_cond_break_continue_producer::{
    loop_cond_break_continue_recipe, produce_loop_cond_break_continue_recipe_v1,
};
use super::super::loop_cond_break_continue_producer_tests::{
    demand_for as m8d_demand_for, typed_map_for as m8d_typed_map_for, unit as m8d_unit,
};

const HAKO_M8D_WIRE_EMISSION: &str =
    include_str!("../fixtures/hako_loop_recipe_wire_m8d_v1.json");

/// The canonical M8D artifact, assembled by the same issuer calls
/// `produce_loop_cond_break_continue_recipe_v1` makes for the recipe
/// half — `loop_cond_break_continue_recipe(&loop_condition,
/// &branch_condition)` -> `LoopRecipeVerifierV1::verify` -> the
/// map-carried `VerifiedLoopRootSourceV1::into_root_claim` — with
/// `LoopCondBreakContinueV1` provenance.
fn m8d_rust_artifact() -> LoopRecipeArtifactV1 {
    let unit = m8d_unit();
    let input = unit.root_function_input().expect("m8d function input");
    let (source_root, _projection, _carrier, loop_condition, branch_condition, _frame) =
        m8d_typed_map_for(input).into_parts();
    let recipe = loop_cond_break_continue_recipe(&loop_condition, &branch_condition);
    let verified_recipe =
        LoopRecipeVerifierV1::verify(recipe.clone()).expect("m8d recipe verifies");
    let source_binding = source_root.into_root_claim(&verified_recipe);
    LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::LoopCondBreakContinueV1),
        source_binding,
        recipe,
    )
}

#[test]
fn m8d_hako_emission_verifies_and_matches_rust_producer_artifact() {
    let hako_verified = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8D_WIRE_EMISSION)
        .expect("m8d .hako emission decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8d_rust_artifact()).expect("m8d rust verifies");

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
fn m8d_reconstructed_artifact_matches_real_producer_product() {
    let reconstructed = LoopRecipeVerifierV1::verify_artifact(m8d_rust_artifact())
        .expect("m8d artifact verifies");
    let unit = m8d_unit();
    let input = unit.root_function_input().expect("m8d function input");
    let product = produce_loop_cond_break_continue_recipe_v1(m8d_demand_for(input), input.function())
        .expect("real producer seals");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_semantic(reconstructed.recipe())
            .expect("reconstructed normalize_semantic"),
        LoopRecipeNormalizerV1::normalize_semantic(product.recipe())
            .expect("producer normalize_semantic"),
    );
}

#[test]
fn m8d_hako_emission_is_single_line_loopcond_provenance_with_two_exits() {
    let trimmed = HAKO_M8D_WIRE_EMISSION.trim_end();
    assert!(!trimmed.contains('\n'), "m8d emission is one compact line");
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(trimmed).expect("m8d emission decodes as artifact");
    assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V1);
    assert_eq!(
        artifact.provenance.producer_id,
        LoopRecipeProducerIdV1::LoopCondBreakContinueV1
    );
    assert_eq!(artifact.recipe.exits.len(), 2);
    assert!(artifact.recipe.exits.iter().any(|exit| matches!(
        exit.kind,
        super::super::schema::LoopExitKindV1::Break { .. }
    )));
    assert!(artifact.recipe.exits.iter().any(|exit| matches!(
        exit.kind,
        super::super::schema::LoopExitKindV1::Continue { .. }
    )));
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(
            row.item,
            LoopRecipeItemV1::If {
                else_block: Some(_),
                ..
            }
        )));
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(row.item, LoopRecipeItemV1::Exit { .. })));
}

#[test]
fn m8d_hako_emission_normalization_is_deterministic() {
    let first = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8D_WIRE_EMISSION)
        .expect("first m8d decode verifies");
    let second = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8D_WIRE_EMISSION)
        .expect("second m8d decode verifies");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_artifact(&first).expect("first normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&second).expect("second normalize"),
    );
}

#[test]
fn m8d_wire_with_foreign_provenance_is_semantic_drift() {
    let mut value: serde_json::Value =
        serde_json::from_str(HAKO_M8D_WIRE_EMISSION).expect("m8d fixture is JSON");
    *value
        .get_mut("provenance")
        .and_then(|p| p.get_mut("producer_id"))
        .expect("producer_id") = serde_json::json!("generic_g0");
    let json = serde_json::to_string(&value).expect("encodes");
    let drifted = LoopRecipeNormalizerV1::decode_and_verify(&json)
        .expect("drifted artifact still decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8d_rust_artifact()).expect("m8d rust verifies");
    assert_ne!(
        LoopRecipeNormalizerV1::normalize_artifact(&drifted).expect("drifted normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&rust_verified).expect("rust normalize"),
    );
}

