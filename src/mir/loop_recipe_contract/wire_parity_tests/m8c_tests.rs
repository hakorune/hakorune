use super::*;

// ---------------------------------------------------------------------------
// S7B3 — M8C scans wire-coverage cohort (first LoopRecipeArtifactV2 row).
//
// The `.hako` entry `emit_m8c_scans_wire.hako` emits the canonical M8C
// artifact (schema_version 2, provenance `scan_with_init_v2`) for the
// bounded source profile `apps/tests/scan_with_init_typed_ok_min.hako`:
// `find_ok(s, ch): i64 { local i = 0; loop(i < s.length()) {
// if s.substring(i, i + 1) == ch { return i }; i = i + 1 }; return -1 }`
// — loop at body item index 1. The comparison target is assembled
// through the producer's own issuer calls: `build_recipe` ->
// `LoopRecipeVerifierV2::verify` -> resolver `bind_resolved_loop_root_v1`
// + `into_root_claim_v2` -> `LoopRecipeVerifierV2::bind_verified_artifact`
// with `ScanWithInitV2` provenance; the real
// `produce_s6c_scan_with_init_recipe_v2` product anchors the semantic arm.
// ---------------------------------------------------------------------------

use super::super::s6c_scan_with_init::{build_recipe, produce_s6c_scan_with_init_recipe_v2};
use super::super::s6c_scan_with_init_tests::issue_facts_and_loop_source;
use super::super::schema_v2::LoopRecipeArtifactV2;
use super::super::typed_schema_v2::LoopRecipeVerifierV2;
use super::super::{LoopRecipeNormalizerV2, LOOP_RECIPE_SCHEMA_VERSION_V2};

const HAKO_M8C_WIRE_EMISSION: &str =
    include_str!("../fixtures/hako_loop_recipe_wire_m8c_v2.json");
const M8C_FIXTURE_SOURCE: &str =
    include_str!("../../../../apps/tests/scan_with_init_typed_ok_min.hako");
const M8C_COMPILATION_UNIT_ORDINAL: u32 = 0;

/// The canonical M8C artifact, assembled by the same issuer calls
/// `produce_s6c_scan_with_init_recipe_v2` makes for the recipe half —
/// `build_recipe` -> `LoopRecipeVerifierV2::verify` — plus the resolver
/// source-claim issuer `bind_resolved_loop_root_v1` + `into_root_claim_v2`
/// and `bind_verified_artifact` with `ScanWithInitV2` provenance.
fn m8c_rust_artifact() -> super::super::typed_schema_v2::VerifiedLoopRecipeArtifactV2 {
    let (_facts, loop_source) =
        issue_facts_and_loop_source(M8C_FIXTURE_SOURCE, M8C_COMPILATION_UNIT_ORDINAL);
    let verified_recipe =
        LoopRecipeVerifierV2::verify(build_recipe()).expect("m8c recipe verifies");
    let source_binding = bind_resolved_loop_root_v1(loop_source)
        .expect("m8c root binding")
        .into_root_claim_v2(&verified_recipe);
    LoopRecipeVerifierV2::bind_verified_artifact(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::ScanWithInitV2),
        source_binding,
        verified_recipe,
    )
    .expect("m8c artifact binds")
}

#[test]
fn m8c_hako_emission_verifies_and_matches_rust_producer_artifact() {
    let hako_verified = LoopRecipeNormalizerV2::decode_and_verify(HAKO_M8C_WIRE_EMISSION)
        .expect("m8c .hako emission decodes and verifies");
    let rust_verified = m8c_rust_artifact();

    assert_eq!(
        LoopRecipeNormalizerV2::normalize_artifact(&hako_verified)
            .expect("hako normalize_artifact"),
        LoopRecipeNormalizerV2::normalize_artifact(&rust_verified)
            .expect("rust normalize_artifact"),
    );
    assert_eq!(
        LoopRecipeNormalizerV2::normalize_semantic(hako_verified.recipe())
            .expect("hako normalize_semantic"),
        LoopRecipeNormalizerV2::normalize_semantic(rust_verified.recipe())
            .expect("rust normalize_semantic"),
    );
    assert_eq!(
        LoopRecipeNormalizerV2::normalize_source_bound(&hako_verified)
            .expect("hako normalize_source_bound"),
        LoopRecipeNormalizerV2::normalize_source_bound(&rust_verified)
            .expect("rust normalize_source_bound"),
    );
}

#[test]
fn m8c_reconstructed_artifact_matches_real_producer_product() {
    let reconstructed = m8c_rust_artifact();
    let (facts, _loop_source) =
        issue_facts_and_loop_source(M8C_FIXTURE_SOURCE, M8C_COMPILATION_UNIT_ORDINAL);
    let product =
        produce_s6c_scan_with_init_recipe_v2(facts).expect("real producer seals");
    let product_recipe_json = product.with_product(|view| {
        serde_json::to_string(view.recipe().as_recipe()).expect("producer recipe encodes")
    });
    assert_eq!(
        LoopRecipeNormalizerV2::normalize_semantic(reconstructed.recipe())
            .expect("reconstructed normalize_semantic"),
        product_recipe_json,
    );
}

#[test]
fn m8c_hako_emission_is_single_line_v2_scan_provenance_with_return_exit() {
    let trimmed = HAKO_M8C_WIRE_EMISSION.trim_end();
    assert!(!trimmed.contains('\n'), "m8c emission is one compact line");
    let artifact: LoopRecipeArtifactV2 =
        serde_json::from_str(trimmed).expect("m8c emission decodes as artifact");
    assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V2);
    assert_eq!(
        artifact.provenance.producer_id,
        LoopRecipeProducerIdV1::ScanWithInitV2
    );
    assert_eq!(artifact.recipe.exits.len(), 1);
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(row.item, super::super::schema_v2::LoopRecipeItemV2::If { .. })));
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(row.item, super::super::schema_v2::LoopRecipeItemV2::Exit { .. })));
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(
            row.item,
            super::super::schema_v2::LoopRecipeItemV2::Operation {
                operation: super::super::schema_v2::LoopOperationV2::CallSlot { .. }
            }
        )));
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(
            row.item,
            super::super::schema_v2::LoopRecipeItemV2::Operation {
                operation: super::super::schema_v2::LoopOperationV2::TextEq { .. }
            }
        )));
}

#[test]
fn m8c_hako_emission_normalization_is_deterministic() {
    let first = LoopRecipeNormalizerV2::decode_and_verify(HAKO_M8C_WIRE_EMISSION)
        .expect("first m8c decode verifies");
    let second = LoopRecipeNormalizerV2::decode_and_verify(HAKO_M8C_WIRE_EMISSION)
        .expect("second m8c decode verifies");
    assert_eq!(
        LoopRecipeNormalizerV2::normalize_artifact(&first).expect("first normalize"),
        LoopRecipeNormalizerV2::normalize_artifact(&second).expect("second normalize"),
    );
}

#[test]
fn m8c_wire_with_foreign_provenance_is_semantic_drift() {
    let mut value: serde_json::Value =
        serde_json::from_str(HAKO_M8C_WIRE_EMISSION).expect("m8c fixture is JSON");
    *value
        .get_mut("provenance")
        .and_then(|p| p.get_mut("producer_id"))
        .expect("producer_id") = serde_json::json!("callable_single_loop_v1");
    let json = serde_json::to_string(&value).expect("encodes");
    let drifted = LoopRecipeNormalizerV2::decode_and_verify(&json)
        .expect("drifted artifact still decodes and verifies");
    let rust_verified = m8c_rust_artifact();
    assert_ne!(
        LoopRecipeNormalizerV2::normalize_artifact(&drifted).expect("drifted normalize"),
        LoopRecipeNormalizerV2::normalize_artifact(&rust_verified)
            .expect("rust normalize"),
    );
}

#[test]
fn m8c_wire_with_v1_schema_version_is_typed_reject() {
    let mut value: serde_json::Value =
        serde_json::from_str(HAKO_M8C_WIRE_EMISSION).expect("m8c fixture is JSON");
    *value
        .get_mut("schema_version")
        .expect("schema_version") = serde_json::json!(1);
    let json = serde_json::to_string(&value).expect("encodes");
    let rejected = LoopRecipeNormalizerV2::decode_and_verify(&json);
    assert!(matches!(
        rejected,
        Err(super::super::LoopRecipeDecodeErrorV2::Rejected(
            super::super::typed_schema_v2::LoopRecipeV2RejectReason::UnsupportedVersion { .. }
        ))
    ));
}

