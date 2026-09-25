//! S7A wire-parity harness (caller-zero, test-only).
//!
//! The `.hako` subtree `lang/src/mir/builder/loop_recipe/` emits one fixed
//! minimal `LoopRecipeArtifactV1` to stdout; its checked-in emission is the
//! fixture below. This harness decodes + verifies + normalizes those bytes
//! with `LoopRecipeNormalizerV1` and asserts equality with the same artifact
//! assembled by Rust. It adds no public API and no production caller.

use super::error::LoopRecipeRejectReasonV1 as Reject;
use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopValueKeyV1,
};
use super::normalize::{LoopRecipeDecodeErrorV1, LoopRecipeNormalizerV1};
use super::producer_id::LoopRecipeProducerIdV1;
use super::schema::{
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopConditionV1, LoopNodeSourceBindingV1, LoopNodeV1,
    LoopOperationV1, LoopRecipeArtifactV1, LoopRecipeBindingV1, LoopRecipeBlockV1,
    LoopRecipeCarrierV1, LoopRecipeItemRowV1, LoopRecipeItemV1, LoopRecipeProvenanceV1,
    LoopRecipeSourceBindingV1, LoopRecipeSourceOwnerV1, LoopRecipeV1, LoopRecipeValueV1,
    LoopSourcePathStepV1, LoopSourcePathV1, LoopValueClassV1, LOOP_RECIPE_SCHEMA_VERSION_V1,
};
use super::variable_accum_break_producer::{
    break_recipe, produce_variable_accum_break_recipe_v1,
};
use super::variable_accum_recurrence_producer::{
    produce_variable_accum_recurrence_recipe_v1, recurrence_recipe,
};
use super::verify::LoopRecipeVerifierV1;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::variable_accum_break_projection::issue_variable_accum_break_source_attempt_v1;
use crate::mir::compiler::variable_accum_recurrence_projection::issue_variable_accum_recurrence_facts_from_membership_v1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_root_v1, VariableAccumBreakObservationCoverageV1,
    VariableAccumBreakSourceAttemptOutcomeV1, VerifiedVariableAccumBreakFactsV1,
    VerifiedVariableAccumRecurrenceFactsV1,
};
use crate::mir::resolved_semantics::CallableSemanticSourceLedgerView;

const HAKO_WIRE_EMISSION: &str = include_str!("fixtures/hako_loop_recipe_wire_v1.json");
const HAKO_M8A_WIRE_EMISSION: &str =
    include_str!("fixtures/hako_loop_recipe_wire_m8a_v1.json");
const HAKO_M8B_WIRE_EMISSION: &str =
    include_str!("fixtures/hako_loop_recipe_wire_m8b_v1.json");
const ACCUM_DIRECT_GOLDEN: &str = include_str!("fixtures/accum_direct_v1.json");

/// The same minimal artifact the `.hako` entry assembles from its named
/// string-fragment locals: one predicate loop `i < 3` over two i64 carriers
/// (`i`, `sum`) stepping `sum += 1; i += 1`.
fn rust_assembled_artifact() -> LoopRecipeArtifactV1 {
    let operation = |operation: LoopOperationV1| LoopRecipeItemV1::Operation { operation };
    LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::DirectAccumV1),
        LoopRecipeSourceBindingV1::new(
            LoopRecipeSourceOwnerV1::function_body(0, 0),
            vec![LoopNodeSourceBindingV1::new(
                LoopNodeKeyV1::new(0),
                LoopSourcePathV1::new(vec![LoopSourcePathStepV1::BodyItem { index: 0 }]),
            )],
        ),
        LoopRecipeV1 {
            root_loop: LoopNodeKeyV1::new(0),
            loops: vec![LoopNodeV1 {
                key: LoopNodeKeyV1::new(0),
                parent: None,
                condition: LoopConditionV1::Predicate {
                    block: LoopBlockKeyV1::new(0),
                    value: LoopValueKeyV1::new(4),
                },
                body: LoopBlockKeyV1::new(1),
            }],
            blocks: vec![
                LoopRecipeBlockV1 {
                    key: LoopBlockKeyV1::new(0),
                    owner_loop: LoopNodeKeyV1::new(0),
                    items: [0, 1, 2].into_iter().map(LoopItemKeyV1::new).collect(),
                },
                LoopRecipeBlockV1 {
                    key: LoopBlockKeyV1::new(1),
                    owner_loop: LoopNodeKeyV1::new(0),
                    items: [3, 4, 5, 6, 7, 8, 9, 10]
                        .into_iter()
                        .map(LoopItemKeyV1::new)
                        .collect(),
                },
            ],
            items: vec![
                LoopRecipeItemRowV1 {
                    key: LoopItemKeyV1::new(0),
                    item: operation(LoopOperationV1::ConstI64 {
                        result: LoopValueKeyV1::new(3),
                        value: 3,
                    }),
                },
                LoopRecipeItemRowV1 {
                    key: LoopItemKeyV1::new(1),
                    item: operation(LoopOperationV1::ReadBinding {
                        binding: LoopBindingKeyV1::new(0),
                        result: LoopValueKeyV1::new(2),
                    }),
                },
                LoopRecipeItemRowV1 {
                    key: LoopItemKeyV1::new(2),
                    item: operation(LoopOperationV1::CompareI64 {
                        op: LoopCompareI64OpV1::Less,
                        left: LoopValueKeyV1::new(2),
                        right: LoopValueKeyV1::new(3),
                        result: LoopValueKeyV1::new(4),
                    }),
                },
                LoopRecipeItemRowV1 {
                    key: LoopItemKeyV1::new(3),
                    item: operation(LoopOperationV1::ReadBinding {
                        binding: LoopBindingKeyV1::new(1),
                        result: LoopValueKeyV1::new(5),
                    }),
                },
                LoopRecipeItemRowV1 {
                    key: LoopItemKeyV1::new(4),
                    item: operation(LoopOperationV1::ConstI64 {
                        result: LoopValueKeyV1::new(6),
                        value: 1,
                    }),
                },
                LoopRecipeItemRowV1 {
                    key: LoopItemKeyV1::new(5),
                    item: operation(LoopOperationV1::BinaryI64 {
                        op: LoopBinaryI64OpV1::Add,
                        left: LoopValueKeyV1::new(5),
                        right: LoopValueKeyV1::new(6),
                        result: LoopValueKeyV1::new(7),
                    }),
                },
                LoopRecipeItemRowV1 {
                    key: LoopItemKeyV1::new(6),
                    item: operation(LoopOperationV1::WriteBinding {
                        binding: LoopBindingKeyV1::new(1),
                        value: LoopValueKeyV1::new(7),
                    }),
                },
                LoopRecipeItemRowV1 {
                    key: LoopItemKeyV1::new(7),
                    item: operation(LoopOperationV1::ReadBinding {
                        binding: LoopBindingKeyV1::new(0),
                        result: LoopValueKeyV1::new(8),
                    }),
                },
                LoopRecipeItemRowV1 {
                    key: LoopItemKeyV1::new(8),
                    item: operation(LoopOperationV1::ConstI64 {
                        result: LoopValueKeyV1::new(9),
                        value: 1,
                    }),
                },
                LoopRecipeItemRowV1 {
                    key: LoopItemKeyV1::new(9),
                    item: operation(LoopOperationV1::BinaryI64 {
                        op: LoopBinaryI64OpV1::Add,
                        left: LoopValueKeyV1::new(8),
                        right: LoopValueKeyV1::new(9),
                        result: LoopValueKeyV1::new(10),
                    }),
                },
                LoopRecipeItemRowV1 {
                    key: LoopItemKeyV1::new(10),
                    item: operation(LoopOperationV1::WriteBinding {
                        binding: LoopBindingKeyV1::new(0),
                        value: LoopValueKeyV1::new(10),
                    }),
                },
            ],
            bindings: vec![
                LoopRecipeBindingV1 {
                    key: LoopBindingKeyV1::new(0),
                    label: "i".to_owned(),
                    class: LoopValueClassV1::I64,
                },
                LoopRecipeBindingV1 {
                    key: LoopBindingKeyV1::new(1),
                    label: "sum".to_owned(),
                    class: LoopValueClassV1::I64,
                },
            ],
            values: [
                LoopValueClassV1::I64,
                LoopValueClassV1::I64,
                LoopValueClassV1::I64,
                LoopValueClassV1::I64,
                LoopValueClassV1::Bool,
                LoopValueClassV1::I64,
                LoopValueClassV1::I64,
                LoopValueClassV1::I64,
                LoopValueClassV1::I64,
                LoopValueClassV1::I64,
                LoopValueClassV1::I64,
            ]
            .into_iter()
            .enumerate()
            .map(|(key, class)| LoopRecipeValueV1 {
                key: LoopValueKeyV1::new(key as u32),
                class,
            })
            .collect(),
            inputs: [0, 1].into_iter().map(LoopValueKeyV1::new).collect(),
            carriers: vec![
                LoopRecipeCarrierV1 {
                    key: LoopCarrierKeyV1::new(0),
                    owner_loop: LoopNodeKeyV1::new(0),
                    binding: LoopBindingKeyV1::new(0),
                    class: LoopValueClassV1::I64,
                    entry_value: LoopValueKeyV1::new(0),
                },
                LoopRecipeCarrierV1 {
                    key: LoopCarrierKeyV1::new(1),
                    owner_loop: LoopNodeKeyV1::new(0),
                    binding: LoopBindingKeyV1::new(1),
                    class: LoopValueClassV1::I64,
                    entry_value: LoopValueKeyV1::new(1),
                },
            ],
            exits: vec![],
        },
    )
}

fn hako_emission_value() -> serde_json::Value {
    serde_json::from_str(HAKO_WIRE_EMISSION).expect("fixture is JSON")
}

#[test]
fn hako_emission_verifies_and_matches_rust_assembled_artifact() {
    let hako_verified = LoopRecipeNormalizerV1::decode_and_verify(HAKO_WIRE_EMISSION)
        .expect(".hako emission decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(rust_assembled_artifact()).expect("rust verifies");

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
fn hako_emission_is_single_line_v1_wire() {
    let trimmed = HAKO_WIRE_EMISSION.trim_end();
    assert!(!trimmed.contains('\n'), "emission is one compact line");
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(trimmed).expect("emission decodes as artifact");
    assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V1);
    assert_eq!(
        artifact.provenance.producer_id,
        LoopRecipeProducerIdV1::DirectAccumV1
    );
}

#[test]
fn hako_emission_matches_direct_accum_golden() {
    let hako: LoopRecipeArtifactV1 =
        serde_json::from_str(HAKO_WIRE_EMISSION).expect("hako artifact decodes");
    let golden: LoopRecipeArtifactV1 =
        serde_json::from_str(ACCUM_DIRECT_GOLDEN).expect("golden decodes");
    assert_eq!(hako, golden);
}

#[test]
fn hako_emission_normalization_is_deterministic() {
    let first = LoopRecipeNormalizerV1::decode_and_verify(HAKO_WIRE_EMISSION)
        .expect("first decode verifies");
    let second = LoopRecipeNormalizerV1::decode_and_verify(HAKO_WIRE_EMISSION)
        .expect("second decode verifies");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_artifact(&first).expect("first normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&second).expect("second normalize"),
    );
}

#[test]
fn hako_wire_with_wrong_schema_version_is_typed_reject() {
    let mut value = hako_emission_value();
    *value.get_mut("schema_version").expect("schema_version") = serde_json::json!(2);
    let json = serde_json::to_string(&value).expect("encodes");
    assert!(matches!(
        LoopRecipeNormalizerV1::decode_and_verify(&json),
        Err(LoopRecipeDecodeErrorV1::Rejected(
            Reject::UnsupportedVersion { found: 2 }
        ))
    ));
}

#[test]
fn hako_wire_with_unknown_field_is_typed_reject() {
    let mut value = hako_emission_value();
    value
        .as_object_mut()
        .expect("artifact object")
        .insert("route_hint".to_owned(), serde_json::json!("loop_break"));
    let json = serde_json::to_string(&value).expect("encodes");
    assert!(matches!(
        LoopRecipeNormalizerV1::decode_and_verify(&json),
        Err(LoopRecipeDecodeErrorV1::Json(_))
    ));
}

#[test]
fn hako_wire_missing_recipe_field_is_typed_reject() {
    let mut value = hako_emission_value();
    value
        .as_object_mut()
        .expect("artifact object")
        .remove("recipe");
    let json = serde_json::to_string(&value).expect("encodes");
    assert!(matches!(
        LoopRecipeNormalizerV1::decode_and_verify(&json),
        Err(LoopRecipeDecodeErrorV1::Json(_))
    ));
}

#[test]
fn hako_wire_with_unknown_producer_id_is_typed_reject() {
    let mut value = hako_emission_value();
    *value
        .get_mut("provenance")
        .and_then(|p| p.get_mut("producer_id"))
        .expect("producer_id") = serde_json::json!("wire_fixture_v0");
    let json = serde_json::to_string(&value).expect("encodes");
    assert!(matches!(
        LoopRecipeNormalizerV1::decode_and_verify(&json),
        Err(LoopRecipeDecodeErrorV1::Json(_))
    ));
}

#[test]
fn hako_wire_with_noncanonical_binding_order_is_structural_reject() {
    let mut value = hako_emission_value();
    value["recipe"]["bindings"]
        .as_array_mut()
        .expect("bindings array")
        .swap(0, 1);
    let json = serde_json::to_string(&value).expect("encodes");
    assert!(matches!(
        LoopRecipeNormalizerV1::decode_and_verify(&json),
        Err(LoopRecipeDecodeErrorV1::Rejected(
            Reject::NonCanonicalKeyOrder {
                domain: "bindings"
            }
        ))
    ));
}

#[test]
fn hako_wire_truncated_json_is_typed_reject() {
    let truncated = &HAKO_WIRE_EMISSION[..HAKO_WIRE_EMISSION.len() / 2];
    assert!(matches!(
        LoopRecipeNormalizerV1::decode_and_verify(truncated),
        Err(LoopRecipeDecodeErrorV1::Json(_))
    ));
}


#[path = "wire_parity_tests/m8a_tests.rs"]
mod m8a_tests;
#[path = "wire_parity_tests/m8b_tests.rs"]
mod m8b_tests;
#[path = "wire_parity_tests/m8c_tests.rs"]
mod m8c_tests;
#[path = "wire_parity_tests/m8d_tests.rs"]
mod m8d_tests;
