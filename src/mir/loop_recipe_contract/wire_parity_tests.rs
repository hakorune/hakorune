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

// ---------------------------------------------------------------------------
// S7B1 — M8A recurrence wire-coverage cohort.
//
// The `.hako` entry `emit_m8a_recurrence_wire.hako` emits the canonical M8A
// artifact (provenance `variable_accum_recurrence_v1`) for the bounded source
// profile `main() { local i = 0; local acc = 0; loop(i < 4) { acc = acc + i;
// i = i + 1 }; print(acc); return 0 }`. The comparison target below is built
// through the same issuer calls the real producer makes — resolver facts ->
// `bind_resolved_loop_root_v1` + `recurrence_recipe(bound, delta)` +
// `VariableAccumRecurrenceV1` provenance — and the real producer product is
// also checked for semantic equality.
// ---------------------------------------------------------------------------

fn m8a_variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn m8a_integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn m8a_assignment(target: &str, left: &str, right: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(m8a_variable(target)),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(m8a_variable(left)),
            right: Box::new(right),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

fn m8a_function() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "main".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: Some("i64".into()),
        body: vec![
            ASTNode::Local {
                variables: vec!["i".into()],
                initial_values: vec![Some(Box::new(m8a_integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Local {
                variables: vec!["acc".into()],
                initial_values: vec![Some(Box::new(m8a_integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Less,
                    left: Box::new(m8a_variable("i")),
                    right: Box::new(m8a_integer(4)),
                    span: Span::unknown(),
                }),
                body: vec![
                    m8a_assignment("acc", "acc", m8a_variable("i")),
                    m8a_assignment("i", "i", m8a_integer(1)),
                ],
                span: Span::unknown(),
            },
            ASTNode::Print {
                expression: Box::new(m8a_variable("acc")),
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(m8a_integer(0))),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: crate::ast::DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn m8a_facts() -> VerifiedVariableAccumRecurrenceFactsV1 {
    let input: ResolvedFunctionLoweringInputV1<'static> = {
        let unit = Box::leak(Box::new(
            VerifiedResolvedSourceUnitV1::resolve_function(m8a_function())
                .expect("m8a fixture resolves"),
        ));
        unit.root_function_input().expect("root input")
    };
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    issue_variable_accum_recurrence_facts_from_membership_v1(input, &ledger, membership)
        .expect("candidate facts")
}

/// The canonical M8A artifact, assembled by the same issuer calls
/// `produce_variable_accum_recurrence_recipe_v1` makes: resolver-bound source
/// claim + `recurrence_recipe(bound, delta)` + `VariableAccumRecurrenceV1`
/// provenance.
fn m8a_rust_artifact() -> LoopRecipeArtifactV1 {
    let (source, _owner, _scope, _bindings, _inputs, condition, _update, step, _coverage) =
        m8a_facts().into_parts();
    let recipe = recurrence_recipe(condition.bound(), step.delta());
    let verified_recipe = LoopRecipeVerifierV1::verify(recipe.clone())
        .expect("m8a recipe verifies");
    let source_binding = bind_resolved_loop_root_v1(source)
        .expect("m8a root binding")
        .into_root_claim(&verified_recipe);
    LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::VariableAccumRecurrenceV1),
        source_binding,
        recipe,
    )
}

#[test]
fn m8a_hako_emission_verifies_and_matches_rust_producer_artifact() {
    let hako_verified = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8A_WIRE_EMISSION)
        .expect("m8a .hako emission decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8a_rust_artifact()).expect("m8a rust verifies");

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
fn m8a_reconstructed_artifact_matches_real_producer_product() {
    let reconstructed = LoopRecipeVerifierV1::verify_artifact(m8a_rust_artifact())
        .expect("m8a artifact verifies");
    let product = produce_variable_accum_recurrence_recipe_v1(m8a_facts())
        .expect("real producer seals");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_semantic(reconstructed.recipe())
            .expect("reconstructed normalize_semantic"),
        LoopRecipeNormalizerV1::normalize_semantic(product.operations().core().recipe())
            .expect("producer normalize_semantic"),
    );
}

#[test]
fn m8a_hako_emission_is_single_line_recurrence_provenance() {
    let trimmed = HAKO_M8A_WIRE_EMISSION.trim_end();
    assert!(!trimmed.contains('\n'), "m8a emission is one compact line");
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(trimmed).expect("m8a emission decodes as artifact");
    assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V1);
    assert_eq!(
        artifact.provenance.producer_id,
        LoopRecipeProducerIdV1::VariableAccumRecurrenceV1
    );
}

#[test]
fn m8a_hako_emission_normalization_is_deterministic() {
    let first = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8A_WIRE_EMISSION)
        .expect("first m8a decode verifies");
    let second = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8A_WIRE_EMISSION)
        .expect("second m8a decode verifies");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_artifact(&first).expect("first normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&second).expect("second normalize"),
    );
}

#[test]
fn m8a_wire_with_foreign_provenance_is_semantic_drift() {
    let mut value: serde_json::Value =
        serde_json::from_str(HAKO_M8A_WIRE_EMISSION).expect("m8a fixture is JSON");
    *value
        .get_mut("provenance")
        .and_then(|p| p.get_mut("producer_id"))
        .expect("producer_id") = serde_json::json!("direct_accum_v1");
    let json = serde_json::to_string(&value).expect("encodes");
    let drifted = LoopRecipeNormalizerV1::decode_and_verify(&json)
        .expect("drifted artifact still decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8a_rust_artifact()).expect("m8a rust verifies");
    assert_ne!(
        LoopRecipeNormalizerV1::normalize_artifact(&drifted).expect("drifted normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&rust_verified).expect("rust normalize"),
    );
}
// ---------------------------------------------------------------------------
// S7B2 — M8B exits/joins wire-coverage cohort.
//
// The `.hako` entry `emit_m8b_break_wire.hako` emits the canonical M8B
// artifact (provenance `variable_accum_break_v1`) for the bounded source
// profile `main() { local sum = 0; local i = 0; loop(i < 10) {
// if(i == 5) { sum = sum + 10; break }; sum = sum + 1; i = i + 1 };
// return sum }`. The comparison target below is built through the same
// issuer calls the real producer makes — attempt facts ->
// `bind_resolved_loop_root_v1` + `break_recipe(loop_bound, branch_bound)`
// + `VariableAccumBreakV1` provenance — and the real producer product is
// also checked for semantic equality.
// ---------------------------------------------------------------------------

fn m8b_variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn m8b_integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn m8b_binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn m8b_assignment(target: &str, left: &str, right: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(m8b_variable(target)),
        value: Box::new(m8b_binary(BinaryOperator::Add, m8b_variable(left), right)),
        span: Span::unknown(),
    }
}

fn m8b_function() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "main".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: Some("i64".into()),
        body: vec![
            ASTNode::Local {
                variables: vec!["sum".into()],
                initial_values: vec![Some(Box::new(m8b_integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Local {
                variables: vec!["i".into()],
                initial_values: vec![Some(Box::new(m8b_integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(m8b_binary(
                    BinaryOperator::Less,
                    m8b_variable("i"),
                    m8b_integer(10),
                )),
                body: vec![
                    ASTNode::If {
                        condition: Box::new(m8b_binary(
                            BinaryOperator::Equal,
                            m8b_variable("i"),
                            m8b_integer(5),
                        )),
                        then_body: vec![
                            m8b_assignment("sum", "sum", m8b_integer(10)),
                            ASTNode::Break {
                                span: Span::unknown(),
                            },
                        ],
                        else_body: None,
                        span: Span::unknown(),
                    },
                    m8b_assignment("sum", "sum", m8b_integer(1)),
                    m8b_assignment("i", "i", m8b_integer(1)),
                ],
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(m8b_variable("sum"))),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: crate::ast::DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn m8b_facts() -> VerifiedVariableAccumBreakFactsV1 {
    let input: ResolvedFunctionLoweringInputV1<'static> = {
        let unit = Box::leak(Box::new(
            VerifiedResolvedSourceUnitV1::resolve_function(m8b_function())
                .expect("m8b fixture resolves"),
        ));
        unit.root_function_input().expect("root input")
    };
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let attempt = issue_variable_accum_break_source_attempt_v1(
        input,
        &ledger,
        membership,
        VariableAccumBreakObservationCoverageV1::Complete,
    );
    let VariableAccumBreakSourceAttemptOutcomeV1::Candidate(facts) = attempt.into_parts().0
    else {
        panic!("expected m8b candidate facts");
    };
    facts
}

/// The canonical M8B artifact, assembled by the same issuer calls
/// `produce_variable_accum_break_recipe_v1` makes: resolver-bound source
/// claim + `break_recipe(loop_bound, branch_bound)` +
/// `VariableAccumBreakV1` provenance.
fn m8b_rust_artifact() -> LoopRecipeArtifactV1 {
    let (
        source,
        _owner,
        _scope,
        _bindings,
        _inputs,
        loop_condition,
        branch_condition,
        _terminal_update,
        _normal_update,
        _induction_step,
        _branch_site,
        _break_site,
        _coverage,
    ) = m8b_facts().into_parts();
    let recipe = break_recipe(loop_condition.bound(), branch_condition.bound());
    let verified_recipe =
        LoopRecipeVerifierV1::verify(recipe.clone()).expect("m8b recipe verifies");
    let source_binding = bind_resolved_loop_root_v1(source)
        .expect("m8b root binding")
        .into_root_claim(&verified_recipe);
    LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::VariableAccumBreakV1),
        source_binding,
        recipe,
    )
}

#[test]
fn m8b_hako_emission_verifies_and_matches_rust_producer_artifact() {
    let hako_verified = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8B_WIRE_EMISSION)
        .expect("m8b .hako emission decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8b_rust_artifact()).expect("m8b rust verifies");

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
fn m8b_reconstructed_artifact_matches_real_producer_product() {
    let reconstructed = LoopRecipeVerifierV1::verify_artifact(m8b_rust_artifact())
        .expect("m8b artifact verifies");
    let product =
        produce_variable_accum_break_recipe_v1(m8b_facts()).expect("real producer seals");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_semantic(reconstructed.recipe())
            .expect("reconstructed normalize_semantic"),
        LoopRecipeNormalizerV1::normalize_semantic(product.recipe())
            .expect("producer normalize_semantic"),
    );
}

#[test]
fn m8b_hako_emission_is_single_line_break_provenance_with_exit() {
    let trimmed = HAKO_M8B_WIRE_EMISSION.trim_end();
    assert!(!trimmed.contains('\n'), "m8b emission is one compact line");
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(trimmed).expect("m8b emission decodes as artifact");
    assert_eq!(artifact.schema_version, LOOP_RECIPE_SCHEMA_VERSION_V1);
    assert_eq!(
        artifact.provenance.producer_id,
        LoopRecipeProducerIdV1::VariableAccumBreakV1
    );
    assert_eq!(artifact.recipe.exits.len(), 1);
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(row.item, LoopRecipeItemV1::If { .. })));
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(row.item, LoopRecipeItemV1::Exit { .. })));
}

#[test]
fn m8b_hako_emission_normalization_is_deterministic() {
    let first = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8B_WIRE_EMISSION)
        .expect("first m8b decode verifies");
    let second = LoopRecipeNormalizerV1::decode_and_verify(HAKO_M8B_WIRE_EMISSION)
        .expect("second m8b decode verifies");
    assert_eq!(
        LoopRecipeNormalizerV1::normalize_artifact(&first).expect("first normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&second).expect("second normalize"),
    );
}

#[test]
fn m8b_wire_with_foreign_provenance_is_semantic_drift() {
    let mut value: serde_json::Value =
        serde_json::from_str(HAKO_M8B_WIRE_EMISSION).expect("m8b fixture is JSON");
    *value
        .get_mut("provenance")
        .and_then(|p| p.get_mut("producer_id"))
        .expect("producer_id") = serde_json::json!("variable_accum_recurrence_v1");
    let json = serde_json::to_string(&value).expect("encodes");
    let drifted = LoopRecipeNormalizerV1::decode_and_verify(&json)
        .expect("drifted artifact still decodes and verifies");
    let rust_verified =
        LoopRecipeVerifierV1::verify_artifact(m8b_rust_artifact()).expect("m8b rust verifies");
    assert_ne!(
        LoopRecipeNormalizerV1::normalize_artifact(&drifted).expect("drifted normalize"),
        LoopRecipeNormalizerV1::normalize_artifact(&rust_verified).expect("rust normalize"),
    );
}

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

use super::s6c_scan_with_init::{build_recipe, produce_s6c_scan_with_init_recipe_v2};
use super::s6c_scan_with_init_tests::issue_facts_and_loop_source;
use super::schema_v2::LoopRecipeArtifactV2;
use super::typed_schema_v2::LoopRecipeVerifierV2;
use super::{LoopRecipeNormalizerV2, LOOP_RECIPE_SCHEMA_VERSION_V2};

const HAKO_M8C_WIRE_EMISSION: &str =
    include_str!("fixtures/hako_loop_recipe_wire_m8c_v2.json");
const M8C_FIXTURE_SOURCE: &str =
    include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");
const M8C_COMPILATION_UNIT_ORDINAL: u32 = 0;

/// The canonical M8C artifact, assembled by the same issuer calls
/// `produce_s6c_scan_with_init_recipe_v2` makes for the recipe half —
/// `build_recipe` -> `LoopRecipeVerifierV2::verify` — plus the resolver
/// source-claim issuer `bind_resolved_loop_root_v1` + `into_root_claim_v2`
/// and `bind_verified_artifact` with `ScanWithInitV2` provenance.
fn m8c_rust_artifact() -> super::typed_schema_v2::VerifiedLoopRecipeArtifactV2 {
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
        .any(|row| matches!(row.item, super::schema_v2::LoopRecipeItemV2::If { .. })));
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(row.item, super::schema_v2::LoopRecipeItemV2::Exit { .. })));
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(
            row.item,
            super::schema_v2::LoopRecipeItemV2::Operation {
                operation: super::schema_v2::LoopOperationV2::CallSlot { .. }
            }
        )));
    assert!(artifact
        .recipe
        .items
        .iter()
        .any(|row| matches!(
            row.item,
            super::schema_v2::LoopRecipeItemV2::Operation {
                operation: super::schema_v2::LoopOperationV2::TextEq { .. }
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
        Err(super::LoopRecipeDecodeErrorV2::Rejected(
            super::typed_schema_v2::LoopRecipeV2RejectReason::UnsupportedVersion { .. }
        ))
    ));
}

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

use super::loop_cond_break_continue_producer::{
    loop_cond_break_continue_recipe, produce_loop_cond_break_continue_recipe_v1,
};
use super::loop_cond_break_continue_producer_tests::{
    demand_for as m8d_demand_for, typed_map_for as m8d_typed_map_for, unit as m8d_unit,
};

const HAKO_M8D_WIRE_EMISSION: &str =
    include_str!("fixtures/hako_loop_recipe_wire_m8d_v1.json");

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
        super::schema::LoopExitKindV1::Break { .. }
    )));
    assert!(artifact.recipe.exits.iter().any(|exit| matches!(
        exit.kind,
        super::schema::LoopExitKindV1::Continue { .. }
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
        .expect("producer_id") = serde_json::json!("variable_accum_break_v1");
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

use super::generic_residual_producer::{
    generic_residual_recipe, produce_generic_residual_recipe_v1,
};
use super::generic_residual_producer_tests::{demand as m8e_demand, typed_map as m8e_typed_map};

const HAKO_M8E_WIRE_EMISSION: &str =
    include_str!("fixtures/hako_loop_recipe_wire_m8e_v1.json");

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
