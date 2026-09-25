//! S7G all-route wire-parity census (caller-zero, test-only).
//!
//! The S7B cohorts pinned per-family parity in `wire_parity_tests.rs`. This
//! module closes the ladder: two new parity arms cover the remaining
//! attested-backed routes (`NestedLoopMinimal`, `LoopTrueBreakContinue`), the
//! S7A `direct_accum_v1` emission gains a live-producer semantic anchor, and
//! one census pins exactly one coverage class per canonical route — wire-
//! backed parity or typed decline — cross-checked against the sealed
//! `ATTESTED_RECIPE_BACKED_V1` table and the migration `RECEIPTS` inventory.
//! It adds no public API, no production caller, and no new semantic receipt.

use super::direct_accum_producer::direct_accum_recipe;
use super::direct_accum_producer_tests::{demand_for as direct_accum_demand_for, direct_accum_product_for_test};
use super::loop_true_break_continue_producer::{
    loop_true_break_continue_recipe, produce_loop_true_break_continue_recipe_v1,
};
use super::loop_true_break_continue_producer_tests::{
    demand_for as loop_true_demand_for, unit as loop_true_unit,
};
use super::normalize::LoopRecipeNormalizerV1;
use super::producer_id::LoopRecipeProducerIdV1;
use super::producer_id_migration_tests::RECEIPTS;
use super::route_id::LoopRouteId;
use super::schema::{
    LoopRecipeArtifactV1, LoopRecipeProvenanceV1, LOOP_RECIPE_SCHEMA_VERSION_V1,
};
use super::verify::LoopRecipeVerifierV1;
use super::{LoopRecipeNormalizerV2, LOOP_RECIPE_SCHEMA_VERSION_V2};
use crate::mir::compiler::nested_predicate_producer::{
    nested_recipe, produce_nested_predicate_recipe_v1,
};
use crate::mir::compiler::{nested_function_for_p3_test, nested_projection_for_test, VerifiedResolvedSourceUnitV1};
use crate::mir::loop_route_policy::{
    LoopRouteRecipeBackingV1, ATTESTED_RECIPE_BACKED_V1, CANONICAL_LOOP_ROUTE_ORDER_V1,
};

const HAKO_WIRE_V1: &str = include_str!("fixtures/hako_loop_recipe_wire_v1.json");
const HAKO_WIRE_M8A: &str = include_str!("fixtures/hako_loop_recipe_wire_m8a_v1.json");
const HAKO_WIRE_M8B: &str = include_str!("fixtures/hako_loop_recipe_wire_m8b_v1.json");
const HAKO_WIRE_M8C: &str = include_str!("fixtures/hako_loop_recipe_wire_m8c_v2.json");
const HAKO_WIRE_M8D: &str = include_str!("fixtures/hako_loop_recipe_wire_m8d_v1.json");
const HAKO_WIRE_NESTED: &str =
    include_str!("fixtures/hako_loop_recipe_wire_nested_v1.json");
const HAKO_WIRE_LOOP_TRUE: &str =
    include_str!("fixtures/hako_loop_recipe_wire_loop_true_v1.json");
const NESTED_GOLDEN: &str = include_str!("fixtures/nested_predicate_v1.json");
const ACCUM_DIRECT_GOLDEN: &str = include_str!("fixtures/accum_direct_v1.json");

// ---------------------------------------------------------------------------
// NestedLoopMinimal arm (cursor 11, `nested_predicate_v1`).
//
// The checked-in `nested_predicate_v1.json` golden is an older-era witness
// (source-name labels); the live producer emits role labels
// (`root_0`/`root_1`/`child_0`). The cohort artifact is the live producer's
// own output for the bounded `nested_function` profile; the golden remains a
// decode-and-verify witness.
// ---------------------------------------------------------------------------

/// The artifact the producer's own issuer calls make for the bounded
/// `nested_function` profile (`nested_loop_minimal` with a root `loop(i < 3)`
/// containing `local j; j = 0;` and the child `loop(j < 3)` accumulation):
/// projection `into_parts` -> forest binding + shape -> `nested_recipe` ->
/// verify -> `into_source_binding` -> `NestedPredicateV1` provenance.
fn nested_rust_artifact() -> LoopRecipeArtifactV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function_for_p3_test())
        .expect("nested function resolves");
    let input = unit.root_function_input().expect("nested function input");
    let (forest_binding, shape, _frame_key) =
        nested_projection_for_test(input).into_parts();
    let recipe = nested_recipe(&shape);
    let verified_for_source =
        LoopRecipeVerifierV1::verify(recipe.clone()).expect("recipe verifies");
    let source_binding = forest_binding
        .into_source_binding(&verified_for_source)
        .expect("forest binding claims loops");
    LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::NestedPredicateV1),
        source_binding,
        recipe,
    )
}

#[test]
fn nested_hako_emission_verifies_and_matches_rust_producer_artifact() {
    let hako_verified = LoopRecipeNormalizerV1::decode_and_verify(HAKO_WIRE_NESTED)
        .expect(".hako nested emission decodes and verifies");
    let rust_verified = LoopRecipeVerifierV1::verify_artifact(nested_rust_artifact())
        .expect("rust artifact verifies");
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
fn nested_reconstructed_artifact_matches_real_producer_product() {
    let reconstructed = LoopRecipeVerifierV1::verify_artifact(nested_rust_artifact())
        .expect("reconstructed verifies");
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function_for_p3_test())
        .expect("nested function resolves");
    let input = unit.root_function_input().expect("nested function input");
    let product = produce_nested_predicate_recipe_v1(
        nested_projection_for_test(input),
        input.function(),
    )
    .expect("real nested producer seals");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_semantic(reconstructed.recipe())
            .expect("reconstructed normalize_semantic"),
        LoopRecipeNormalizerV1::normalize_semantic(product.recipe())
            .expect("producer normalize_semantic"),
    );
}

#[test]
fn nested_hako_emission_is_single_line_nested_provenance_with_two_loops() {
    let trimmed = HAKO_WIRE_NESTED.trim();
    assert!(!trimmed.contains('\n'), "nested emission is one compact line");
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(trimmed).expect("nested emission is serde json");
    assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V1);
    assert_eq!(
        artifact.provenance.producer_id,
        LoopRecipeProducerIdV1::NestedPredicateV1
    );
    assert_eq!(artifact.recipe.loops.len(), 2);
    assert_eq!(
        artifact.recipe.loops[1].parent,
        Some(artifact.recipe.loops[0].key)
    );
    assert_eq!(artifact.recipe.inputs.len(), 2);
}

#[test]
fn nested_golden_stays_a_decode_and_verify_witness() {
    // The older-era golden (source-name labels) still decodes and verifies as
    // a valid artifact; the cohort parity anchor is the live producer.
    LoopRecipeNormalizerV1::decode_and_verify(NESTED_GOLDEN)
        .expect("nested golden decodes and verifies");
}

// ---------------------------------------------------------------------------
// LoopTrueBreakContinue arm (cursor 12, `loop_true_break_continue_v1`).
// ---------------------------------------------------------------------------

/// The artifact the producer's own issuer calls make for the bounded
/// `positive_function` profile (`loop(true)` with an `if` break/else
/// continue branch at `body_item(1)`): demand -> projection `into_parts`
/// -> resolver-bound root source + shape -> recipe issuer -> verify ->
/// `into_root_claim` -> `LoopTrueBreakContinueV1` provenance.
fn loop_true_rust_artifact() -> LoopRecipeArtifactV1 {
    let unit = loop_true_unit();
    let input = unit.root_function_input().expect("loop-true function input");
    let (_receipt, projection) = loop_true_demand_for(input).into_parts();
    let (source_root, shape, _frame_key) = projection.into_parts();
    let recipe = loop_true_break_continue_recipe(&shape);
    let verified_for_source =
        LoopRecipeVerifierV1::verify(recipe.clone()).expect("recipe verifies");
    let source_binding = source_root.into_root_claim(&verified_for_source);
    LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::LoopTrueBreakContinueV1),
        source_binding,
        recipe,
    )
}

#[test]
fn loop_true_hako_emission_verifies_and_matches_rust_producer_artifact() {
    let hako_verified = LoopRecipeNormalizerV1::decode_and_verify(HAKO_WIRE_LOOP_TRUE)
        .expect(".hako loop-true emission decodes and verifies");
    let rust_artifact = loop_true_rust_artifact();
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(rust_artifact).expect("rust artifact verifies");
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
fn loop_true_reconstructed_artifact_matches_real_producer_product() {
    let reconstructed = LoopRecipeVerifierV1::verify_artifact(loop_true_rust_artifact())
        .expect("reconstructed verifies");
    let unit = loop_true_unit();
    let input = unit.root_function_input().expect("loop-true function input");
    let product = produce_loop_true_break_continue_recipe_v1(
        loop_true_demand_for(input),
        input.function(),
    )
    .expect("producer seals");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_semantic(reconstructed.recipe())
            .expect("reconstructed normalize_semantic"),
        LoopRecipeNormalizerV1::normalize_semantic(product.recipe())
            .expect("producer normalize_semantic"),
    );
}

#[test]
fn loop_true_hako_emission_is_single_line_provenance_with_two_exits() {
    let trimmed = HAKO_WIRE_LOOP_TRUE.trim();
    assert!(!trimmed.contains('\n'), "loop-true emission is one compact line");
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(trimmed).expect("loop-true emission is serde json");
    assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V1);
    assert_eq!(
        artifact.provenance.producer_id,
        LoopRecipeProducerIdV1::LoopTrueBreakContinueV1
    );
    assert_eq!(artifact.recipe.exits.len(), 2);
    assert_eq!(artifact.recipe.blocks.len(), 3);
}

// ---------------------------------------------------------------------------
// AccumConstLoop arm (cursor 10, `direct_accum_v1`).
//
// The S7A substrate emission (`hako_loop_recipe_wire_v1.json`) stays pinned to
// the older-era `accum_direct_v1.json` golden — its landed claim is wire
// transport, not producer parity. The AccumConstLoop cohort artifact is the
// live producer's own output for the bounded `accum` profile: role labels
// `induction`/`accumulator`, loop at `body_item(1)`.
// ---------------------------------------------------------------------------

const HAKO_WIRE_ACCUM_DIRECT: &str =
    include_str!("fixtures/hako_loop_recipe_wire_accum_direct_v1.json");

/// The artifact the producer's own issuer calls make for the bounded `accum`
/// profile (`accum() { local i, sum; loop(i < 3) { sum += 1; i += 1 } }`,
/// loop at `body_item(1)`): demand `into_parts` -> facts payload ->
/// `direct_accum_recipe` -> verify -> resolver-bound root claim ->
/// `DirectAccumV1` provenance.
fn direct_accum_rust_artifact() -> LoopRecipeArtifactV1 {
    let unit = super::direct_accum_producer_tests::unit();
    let input = unit.root_function_input().expect("direct accum function input");
    let (facts, source) = direct_accum_demand_for(input).into_parts();
    let shape = facts.into_direct_accum_v1().expect("direct accum payload");
    let source_root =
        crate::mir::loop_structural_facts::bind_resolved_loop_root_v1(source)
            .expect("resolved loop root binds");
    let recipe = direct_accum_recipe(&shape);
    let verified_for_source =
        LoopRecipeVerifierV1::verify(recipe.clone()).expect("recipe verifies");
    let source_binding = source_root.into_root_claim(&verified_for_source);
    LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::DirectAccumV1),
        source_binding,
        recipe,
    )
}

#[test]
fn direct_accum_hako_emission_verifies_and_matches_rust_producer_artifact() {
    let hako_verified = LoopRecipeNormalizerV1::decode_and_verify(HAKO_WIRE_ACCUM_DIRECT)
        .expect(".hako direct-accum emission decodes and verifies");
    let rust_verified = LoopRecipeVerifierV1::verify_artifact(direct_accum_rust_artifact())
        .expect("rust artifact verifies");
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
fn direct_accum_reconstructed_artifact_matches_live_producer_product() {
    let reconstructed = LoopRecipeVerifierV1::verify_artifact(direct_accum_rust_artifact())
        .expect("reconstructed verifies");
    let product = direct_accum_product_for_test();
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_semantic(reconstructed.recipe())
            .expect("reconstructed normalize_semantic"),
        LoopRecipeNormalizerV1::normalize_semantic(product.recipe())
            .expect("producer normalize_semantic"),
    );
}

#[test]
fn direct_accum_hako_emission_is_single_line_provenance_with_role_labels() {
    let trimmed = HAKO_WIRE_ACCUM_DIRECT.trim();
    assert!(!trimmed.contains('\n'), "direct-accum emission is one compact line");
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(trimmed).expect("direct-accum emission is serde json");
    assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V1);
    assert_eq!(
        artifact.provenance.producer_id,
        LoopRecipeProducerIdV1::DirectAccumV1
    );
    assert_eq!(artifact.recipe.bindings.len(), 2);
    assert_eq!(artifact.recipe.carriers.len(), 2);
}

#[test]
fn s7a_substrate_emission_stays_pinned_to_accum_direct_golden() {
    // The S7A substrate file is the wire-transport witness: it keeps matching
    // the older-era golden artifact, which still decodes and verifies.
    LoopRecipeNormalizerV1::decode_and_verify(ACCUM_DIRECT_GOLDEN)
        .expect("golden decodes and verifies");
    let hako: LoopRecipeArtifactV1 =
        serde_json::from_str(HAKO_WIRE_V1).expect("s7a emission decodes");
    let golden: LoopRecipeArtifactV1 =
        serde_json::from_str(ACCUM_DIRECT_GOLDEN).expect("golden decodes");
    assert_eq!(hako, golden);
}

// ---------------------------------------------------------------------------
// All19 coverage census.
// ---------------------------------------------------------------------------

/// One coverage class per canonical route. `V1Parity`/`V2Parity` carry the
/// checked-in `.hako` emission bytes plus the producer id the emission must
/// claim; `TypedDeclined` marks a route with no recipe backing.
#[derive(Clone, Copy)]
enum RouteWireCoverageV1 {
    V1Parity(&'static str, LoopRecipeProducerIdV1),
    V2Parity(&'static str, LoopRecipeProducerIdV1),
    TypedDeclined,
}

const ROUTE_WIRE_COVERAGE_V1: &[(LoopRouteId, RouteWireCoverageV1)] = &[
    (
        LoopRouteId::LoopBreakRecipe,
        RouteWireCoverageV1::V1Parity(HAKO_WIRE_M8B, LoopRecipeProducerIdV1::VariableAccumBreakV1),
    ),
    (LoopRouteId::IfPhiJoin, RouteWireCoverageV1::TypedDeclined),
    (LoopRouteId::LoopContinueOnly, RouteWireCoverageV1::TypedDeclined),
    (
        LoopRouteId::LoopTrueEarlyExit,
        RouteWireCoverageV1::TypedDeclined,
    ),
    (
        LoopRouteId::LoopSimpleWhile,
        RouteWireCoverageV1::V1Parity(
            HAKO_WIRE_M8A,
            LoopRecipeProducerIdV1::VariableAccumRecurrenceV1,
        ),
    ),
    (LoopRouteId::LoopCharMap, RouteWireCoverageV1::TypedDeclined),
    (LoopRouteId::LoopArrayJoin, RouteWireCoverageV1::TypedDeclined),
    (
        LoopRouteId::ScanWithInit,
        RouteWireCoverageV1::V2Parity(HAKO_WIRE_M8C, LoopRecipeProducerIdV1::ScanWithInitV2),
    ),
    (LoopRouteId::SplitScan, RouteWireCoverageV1::TypedDeclined),
    (
        LoopRouteId::BoolPredicateScan,
        RouteWireCoverageV1::TypedDeclined,
    ),
    (
        LoopRouteId::AccumConstLoop,
        RouteWireCoverageV1::V1Parity(
            HAKO_WIRE_ACCUM_DIRECT,
            LoopRecipeProducerIdV1::DirectAccumV1,
        ),
    ),
    (
        LoopRouteId::NestedLoopMinimal,
        RouteWireCoverageV1::V1Parity(HAKO_WIRE_NESTED, LoopRecipeProducerIdV1::NestedPredicateV1),
    ),
    (
        LoopRouteId::LoopTrueBreakContinue,
        RouteWireCoverageV1::V1Parity(
            HAKO_WIRE_LOOP_TRUE,
            LoopRecipeProducerIdV1::LoopTrueBreakContinueV1,
        ),
    ),
    (
        LoopRouteId::LoopCondBreakContinue,
        RouteWireCoverageV1::V1Parity(
            HAKO_WIRE_M8D,
            LoopRecipeProducerIdV1::LoopCondBreakContinueV1,
        ),
    ),
    (
        LoopRouteId::LoopCondContinueOnly,
        RouteWireCoverageV1::TypedDeclined,
    ),
    (
        LoopRouteId::LoopCondContinueWithReturn,
        RouteWireCoverageV1::TypedDeclined,
    ),
    (
        LoopRouteId::LoopCondReturnInBody,
        RouteWireCoverageV1::TypedDeclined,
    ),
    (LoopRouteId::GenericLoopV0, RouteWireCoverageV1::TypedDeclined),
    (LoopRouteId::GenericLoopV1, RouteWireCoverageV1::TypedDeclined),
];

#[test]
fn census_covers_every_canonical_route_exactly_once() {
    assert_eq!(ROUTE_WIRE_COVERAGE_V1.len(), CANONICAL_LOOP_ROUTE_ORDER_V1.len());
    for (index, route) in CANONICAL_LOOP_ROUTE_ORDER_V1.iter().enumerate() {
        assert_eq!(
            ROUTE_WIRE_COVERAGE_V1[index].0, *route,
            "census row {index} follows canonical order",
        );
    }
}

#[test]
fn census_wire_backed_rows_match_sealed_attestation() {
    for (route, coverage) in ROUTE_WIRE_COVERAGE_V1 {
        let attested = ATTESTED_RECIPE_BACKED_V1
            .iter()
            .find(|(attested_route, _)| attested_route == route)
            .map(|(_, backing)| *backing);
        match coverage {
            RouteWireCoverageV1::V1Parity(_, producer_id) => {
                assert_eq!(
                    attested,
                    Some(LoopRouteRecipeBackingV1::PortableProducer(*producer_id)),
                    "{route:?} wire coverage must match a PortableProducer attestation",
                );
            }
            RouteWireCoverageV1::V2Parity(_, producer_id) => {
                assert_eq!(
                    attested,
                    Some(LoopRouteRecipeBackingV1::ScanWithInitV2),
                    "{route:?} wire coverage must match the ScanWithInitV2 attestation",
                );
                assert_eq!(*producer_id, LoopRecipeProducerIdV1::ScanWithInitV2);
            }
            RouteWireCoverageV1::TypedDeclined => {
                assert!(
                    attested.is_none(),
                    "{route:?} typed-declined routes carry no recipe backing",
                );
            }
        }
    }
}

#[test]
fn census_matches_migration_receipt_inventory() {
    assert_eq!(RECEIPTS.len(), CANONICAL_LOOP_ROUTE_ORDER_V1.len());
    for (route, coverage) in ROUTE_WIRE_COVERAGE_V1 {
        let receipt = RECEIPTS
            .iter()
            .find(|receipt| receipt.legacy_route == *route)
            .unwrap_or_else(|| panic!("{route:?} has a migration receipt"));
        match coverage {
            RouteWireCoverageV1::V1Parity(_, producer_id) => {
                assert_eq!(receipt.producer_id, Some(*producer_id));
                assert_eq!(receipt.disposition, "portable_producer");
            }
            RouteWireCoverageV1::V2Parity(_, producer_id) => {
                assert_eq!(receipt.producer_id, Some(*producer_id));
                assert_eq!(receipt.disposition, "portable_v2_producer");
            }
            RouteWireCoverageV1::TypedDeclined => {
                assert_eq!(receipt.producer_id, None);
                assert_eq!(receipt.disposition, "legacy_only");
            }
        }
    }
}

#[test]
fn census_wire_backed_fixtures_decode_and_verify() {
    for (route, coverage) in ROUTE_WIRE_COVERAGE_V1 {
        match coverage {
            RouteWireCoverageV1::V1Parity(bytes, producer_id) => {
                LoopRecipeNormalizerV1::decode_and_verify(bytes)
                    .unwrap_or_else(|_| panic!("{route:?} fixture decodes and verifies"));
                let artifact: LoopRecipeArtifactV1 =
                    serde_json::from_str(bytes.trim()).expect("fixture is serde json");
                assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V1);
                assert_eq!(artifact.provenance.producer_id, *producer_id);
            }
            RouteWireCoverageV1::V2Parity(bytes, producer_id) => {
                LoopRecipeNormalizerV2::decode_and_verify(bytes)
                    .unwrap_or_else(|_| panic!("{route:?} fixture decodes and verifies"));
                let artifact: serde_json::Value =
                    serde_json::from_str(bytes.trim()).expect("fixture is serde json");
                assert_eq!(
                    artifact["schema_version"].as_u64(),
                    Some(LOOP_RECIPE_SCHEMA_VERSION_V2 as u64)
                );
                assert_eq!(
                    artifact["provenance"]["producer_id"].as_str(),
                    Some(match producer_id {
                        LoopRecipeProducerIdV1::ScanWithInitV2 => "scan_with_init_v2",
                        _ => unreachable!("v2 census row is scan_with_init"),
                    })
                );
            }
            RouteWireCoverageV1::TypedDeclined => {}
        }
    }
}
