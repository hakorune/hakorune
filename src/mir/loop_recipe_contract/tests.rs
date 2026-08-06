use super::error::LoopRecipeRejectReasonV1 as Reject;
use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
use super::join_sig::{LoopJoinEdgeRoleV1, LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1};
use super::normalize::{LoopRecipeDecodeErrorV1, LoopRecipeNormalizerV1};
use super::producer_id::LoopRecipeProducerIdV1;
use super::schema::{
    LoopExitKindV1, LoopRecipeArtifactV1, LoopRecipeBlockV1, LoopRecipeExitV1, LoopRecipeItemRowV1,
    LoopRecipeItemV1, LoopRecipeSourceOwnerV1, LoopSourcePathStepV1, LoopValueClassV1,
};
use super::verify::LoopRecipeVerifierV1;

const ACCUM_NESTED_GOLDEN: &str = include_str!("fixtures/accum_nested_v1.json");
fn golden() -> LoopRecipeArtifactV1 {
    serde_json::from_str(ACCUM_NESTED_GOLDEN).expect("valid golden JSON")
}

fn reject(artifact: LoopRecipeArtifactV1) -> Reject {
    LoopRecipeVerifierV1::verify_artifact(artifact).expect_err("fixture must reject")
}

fn verified_recipe() -> super::verify::VerifiedLoopRecipeV1 {
    LoopRecipeVerifierV1::verify(golden().recipe).expect("golden recipe verifies")
}

#[test]
fn join_sig_accum_nested_is_deterministic_and_closed() {
    let left = LoopJoinSigElaboratorV1::elaborate(&verified_recipe()).expect("left join signature");
    let right =
        LoopJoinSigElaboratorV1::elaborate(&verified_recipe()).expect("right join signature");
    assert_eq!(left.as_sig(), right.as_sig());
    assert_eq!(left.as_sig().loops.len(), 2);
    assert_eq!(left.as_sig().loops[0].carriers.len(), 2);
    assert_eq!(left.as_sig().loops[1].carriers.len(), 0);
    assert!(left.as_sig().loops[0].edges.iter().any(|edge| {
        edge.role == LoopJoinEdgeRoleV1::PredicateTrue
            && edge
                .payload
                .iter()
                .any(|payload| payload.value == LoopValueKeyV1::new(0))
    }));
    assert!(left.as_sig().loops[1]
        .edges
        .iter()
        .any(|edge| edge.role == LoopJoinEdgeRoleV1::Break));
    let inner_break = left.as_sig().loops[1]
        .edges
        .iter()
        .find(|edge| edge.role == LoopJoinEdgeRoleV1::Break)
        .expect("inner break edge");
    assert!(inner_break.payload.iter().any(|payload| {
        payload.binding == LoopBindingKeyV1::new(1) && payload.value == LoopValueKeyV1::new(6)
    }));
}

#[test]
fn join_sig_rejects_late_value_use_before_any_physical_effect() {
    let mut artifact = golden();
    if let LoopRecipeItemV1::Operation {
        operation: super::schema::LoopOperationV1::BinaryI64 { left, .. },
    } = &mut artifact.recipe.items[7].item
    {
        *left = LoopValueKeyV1::new(5);
    }
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::ValueNotAvailable {
            value: LoopValueKeyV1::new(5)
        })
    );
}

#[test]
fn join_sig_rejects_missing_carrier_closure() {
    let mut artifact = golden();
    artifact.recipe.carriers[0].owner_loop = LoopNodeKeyV1::new(1);
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::MissingCarrierClosure {
            loop_key: LoopNodeKeyV1::new(0),
            binding: LoopBindingKeyV1::new(0),
        })
    );
}

#[test]
fn join_sig_rejects_item_after_terminal_exit() {
    let mut artifact = golden();
    artifact
        .recipe
        .values
        .push(super::schema::LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(7),
            class: LoopValueClassV1::I64,
        });
    artifact.recipe.items.push(LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(10),
        item: LoopRecipeItemV1::Operation {
            operation: super::schema::LoopOperationV1::ConstI64 {
                result: LoopValueKeyV1::new(7),
                value: 0,
            },
        },
    });
    artifact.recipe.blocks[1].items.push(LoopItemKeyV1::new(10));
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::UnreachableItem {
            item: LoopItemKeyV1::new(10)
        })
    );
}

#[test]
fn join_sig_propagates_nested_return_as_terminal() {
    let mut artifact = golden();
    artifact.recipe.exits[1].kind = LoopExitKindV1::Return { value: None };
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::UnreachableItem {
            item: LoopItemKeyV1::new(6)
        })
    );
}

#[test]
fn accum_nested_golden_roundtrips_and_normalizes() {
    let expected = golden();
    let verified =
        LoopRecipeNormalizerV1::decode_and_verify(ACCUM_NESTED_GOLDEN).expect("golden verifies");
    let normalized =
        LoopRecipeNormalizerV1::normalize_artifact(&verified).expect("normalization succeeds");
    let decoded: LoopRecipeArtifactV1 =
        serde_json::from_str(&normalized).expect("normalized artifact decodes");

    assert_eq!(decoded, expected);
    assert_eq!(verified.recipe().as_recipe().loops.len(), 2);
    assert_eq!(verified.recipe().as_recipe().carriers.len(), 2);
    assert_eq!(verified.recipe().as_recipe().exits.len(), 2);
}

#[test]
fn semantic_normalization_is_independent_of_route_provenance() {
    let original = golden();
    let mut alternate = original.clone();
    alternate.provenance =
        super::schema::LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::GenericG0);

    let left = LoopRecipeVerifierV1::verify_artifact(original).expect("left verifies");
    let right = LoopRecipeVerifierV1::verify_artifact(alternate).expect("right verifies");
    let left_json =
        LoopRecipeNormalizerV1::normalize_semantic(left.recipe()).expect("left semantic normalize");
    let right_json = LoopRecipeNormalizerV1::normalize_semantic(right.recipe())
        .expect("right semantic normalize");

    assert_eq!(left_json, right_json);
}

#[test]
fn semantic_normalization_is_independent_of_source_binding() {
    let original = golden();
    let mut alternate = original.clone();
    alternate.source_binding.owner = LoopRecipeSourceOwnerV1::function_body(0, 7);
    alternate.source_binding.loops[0].path.steps[0] = LoopSourcePathStepV1::BodyItem { index: 9 };
    alternate.source_binding.loops[1].path.steps[0] = LoopSourcePathStepV1::BodyItem { index: 9 };

    let left = LoopRecipeVerifierV1::verify_artifact(original).expect("left verifies");
    let right = LoopRecipeVerifierV1::verify_artifact(alternate).expect("right verifies");
    let left_json =
        LoopRecipeNormalizerV1::normalize_semantic(left.recipe()).expect("left semantic normalize");
    let right_json = LoopRecipeNormalizerV1::normalize_semantic(right.recipe())
        .expect("right semantic normalize");

    assert_eq!(left_json, right_json);
}

#[test]
fn source_bound_normalization_excludes_route_and_includes_source() {
    let original = golden();
    let mut alternate_route = original.clone();
    alternate_route.provenance =
        super::schema::LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::GenericG0);
    let mut alternate_source = original.clone();
    alternate_source.source_binding.owner = LoopRecipeSourceOwnerV1::function_body(0, 7);

    let original = LoopRecipeVerifierV1::verify_artifact(original).expect("original verifies");
    let alternate_route =
        LoopRecipeVerifierV1::verify_artifact(alternate_route).expect("route verifies");
    let alternate_source =
        LoopRecipeVerifierV1::verify_artifact(alternate_source).expect("source verifies");
    let original_json = LoopRecipeNormalizerV1::normalize_source_bound(&original)
        .expect("original source-bound normalize");
    let route_json = LoopRecipeNormalizerV1::normalize_source_bound(&alternate_route)
        .expect("route source-bound normalize");
    let source_json = LoopRecipeNormalizerV1::normalize_source_bound(&alternate_source)
        .expect("source source-bound normalize");

    assert_eq!(original_json, route_json);
    assert_ne!(original_json, source_json);
}

#[test]
fn legacy_producer_route_wire_is_not_a_v1_alias() {
    let mut value: serde_json::Value =
        serde_json::from_str(ACCUM_NESTED_GOLDEN).expect("golden value");
    let provenance = value["provenance"]
        .as_object_mut()
        .expect("provenance object");
    let producer_id = provenance.remove("producer_id").expect("producer id field");
    provenance.insert("producer_route".to_owned(), producer_id);
    let json = serde_json::to_string(&value).expect("legacy fixture encodes");

    assert!(matches!(
        LoopRecipeNormalizerV1::decode_and_verify(&json),
        Err(LoopRecipeDecodeErrorV1::Json(_))
    ));
}

#[test]
fn unsupported_version_is_typed() {
    let mut artifact = golden();
    artifact.schema_version = 2;
    assert_eq!(reject(artifact), Reject::UnsupportedVersion { found: 2 });
}

#[test]
fn source_binding_requires_exact_coverage() {
    let mut artifact = golden();
    artifact.source_binding.loops.pop();
    assert_eq!(
        reject(artifact),
        Reject::SourceBindingCoverageMismatch {
            expected: 2,
            found: 1,
        }
    );
}

#[test]
fn source_binding_order_is_canonical() {
    let mut artifact = golden();
    artifact.source_binding.loops.swap(0, 1);
    assert_eq!(
        reject(artifact),
        Reject::NonCanonicalSourceBindingOrder {
            expected: LoopNodeKeyV1::new(0),
            found: LoopNodeKeyV1::new(1),
        }
    );
}

#[test]
fn source_binding_paths_are_unique() {
    let mut artifact = golden();
    artifact.source_binding.loops[1].path = artifact.source_binding.loops[0].path.clone();
    assert_eq!(
        reject(artifact),
        Reject::DuplicateLoopSourcePath {
            first: LoopNodeKeyV1::new(0),
            second: LoopNodeKeyV1::new(1),
        }
    );
}

#[test]
fn root_source_path_starts_with_body_item() {
    let mut artifact = golden();
    artifact.source_binding.loops[0].path.steps =
        vec![LoopSourcePathStepV1::LoopBodyItem { index: 0 }];
    assert_eq!(
        reject(artifact),
        Reject::RootSourcePathMustStartWithBodyItem {
            loop_key: LoopNodeKeyV1::new(0),
        }
    );
}

#[test]
fn source_path_rejects_body_item_after_root() {
    let mut artifact = golden();
    artifact.source_binding.loops[0]
        .path
        .steps
        .push(LoopSourcePathStepV1::BodyItem { index: 3 });
    assert_eq!(
        reject(artifact),
        Reject::SourcePathBodyItemAfterRoot {
            loop_key: LoopNodeKeyV1::new(0),
            step_index: 1,
        }
    );
}

#[test]
fn root_source_path_may_include_outer_loop_ancestry() {
    let mut artifact = golden();
    artifact.source_binding.loops[0]
        .path
        .steps
        .push(LoopSourcePathStepV1::LoopBodyItem { index: 4 });
    artifact.source_binding.loops[1]
        .path
        .steps
        .insert(1, LoopSourcePathStepV1::LoopBodyItem { index: 4 });
    LoopRecipeVerifierV1::verify_artifact(artifact).expect("outer source Loop ancestry is valid");
}

#[test]
fn nested_source_path_is_a_strict_parent_prefix_descendant() {
    let mut artifact = golden();
    artifact.source_binding.loops[1].path.steps = vec![
        LoopSourcePathStepV1::BodyItem { index: 9 },
        LoopSourcePathStepV1::LoopBodyItem { index: 0 },
    ];
    assert_eq!(
        reject(artifact),
        Reject::NestedSourcePathNotDescendant {
            loop_key: LoopNodeKeyV1::new(1),
            parent_loop: LoopNodeKeyV1::new(0),
        }
    );
}

#[test]
fn nested_source_path_enters_parent_loop_body_first() {
    let mut artifact = golden();
    artifact.source_binding.loops[1].path.steps[1] =
        LoopSourcePathStepV1::ScopeBodyItem { index: 0 };
    assert_eq!(
        reject(artifact),
        Reject::NestedSourcePathMustEnterLoopBody {
            loop_key: LoopNodeKeyV1::new(1),
            parent_loop: LoopNodeKeyV1::new(0),
        }
    );
}

#[test]
fn nested_source_path_rejects_additional_body_item() {
    let mut artifact = golden();
    artifact.source_binding.loops[1]
        .path
        .steps
        .push(LoopSourcePathStepV1::BodyItem { index: 3 });
    assert_eq!(
        reject(artifact),
        Reject::SourcePathBodyItemAfterRoot {
            loop_key: LoopNodeKeyV1::new(1),
            step_index: 2,
        }
    );
}

#[test]
fn nested_source_path_rejects_intermediate_loop_skip() {
    let mut artifact = golden();
    artifact.source_binding.loops[1]
        .path
        .steps
        .push(LoopSourcePathStepV1::LoopBodyItem { index: 1 });
    assert_eq!(
        reject(artifact),
        Reject::NestedSourcePathSkipsIntermediateLoop {
            loop_key: LoopNodeKeyV1::new(1),
            parent_loop: LoopNodeKeyV1::new(0),
            step_index: 2,
        }
    );
}

#[test]
fn nested_source_path_allows_scopes_after_direct_loop_entry() {
    let mut artifact = golden();
    artifact.source_binding.loops[1]
        .path
        .steps
        .push(LoopSourcePathStepV1::ScopeBodyItem { index: 3 });
    LoopRecipeVerifierV1::verify_artifact(artifact).expect("trailing source scopes are valid");
}

#[test]
fn noncanonical_local_key_is_typed() {
    let mut artifact = golden();
    artifact.recipe.blocks[1].key = LoopBlockKeyV1::new(9);
    assert_eq!(
        reject(artifact),
        Reject::NonCanonicalKeyOrder { domain: "blocks" }
    );
}

#[test]
fn duplicate_arena_row_key_is_typed() {
    let mut artifact = golden();
    artifact.recipe.items[1].key = LoopItemKeyV1::new(0);
    assert_eq!(
        reject(artifact),
        Reject::NonCanonicalKeyOrder { domain: "items" }
    );
}

#[test]
fn dangling_predicate_value_is_typed() {
    let mut artifact = golden();
    artifact.recipe.loops[0].condition = super::schema::LoopConditionV1::Predicate {
        block: LoopBlockKeyV1::new(0),
        value: LoopValueKeyV1::new(99),
    };
    assert_eq!(
        reject(artifact),
        Reject::DanglingValue {
            key: LoopValueKeyV1::new(99)
        }
    );
}

#[test]
fn nested_loop_must_name_its_actual_parent() {
    let mut artifact = golden();
    artifact.recipe.loops[1].parent = Some(LoopNodeKeyV1::new(1));
    assert_eq!(
        reject(artifact),
        Reject::InvalidLoopParent {
            loop_key: LoopNodeKeyV1::new(1)
        }
    );
}

#[test]
fn exit_target_must_be_self_or_ancestor() {
    let mut artifact = golden();
    artifact.recipe.exits[0].kind = LoopExitKindV1::Continue {
        target_loop: LoopNodeKeyV1::new(1),
    };
    assert_eq!(
        reject(artifact),
        Reject::ExitTargetNotAncestor {
            key: super::ids::LoopExitKeyV1::new(0)
        }
    );
}

#[test]
fn carrier_class_must_match_binding_and_entry_value() {
    let mut artifact = golden();
    artifact.recipe.carriers[0].class = LoopValueClassV1::Bool;
    assert_eq!(
        reject(artifact),
        Reject::ValueClassMismatch {
            key: LoopValueKeyV1::new(0)
        }
    );
}

#[test]
fn duplicate_item_membership_is_typed() {
    let mut artifact = golden();
    artifact.recipe.blocks[2].items.push(LoopItemKeyV1::new(8));
    assert_eq!(
        reject(artifact),
        Reject::DuplicateItemUse {
            key: LoopItemKeyV1::new(8)
        }
    );
}

#[test]
fn recursive_loop_preorder_is_canonical() {
    let mut artifact = golden();
    let mut second = artifact.recipe.loops[1].clone();
    second.key = LoopNodeKeyV1::new(2);
    second.body = LoopBlockKeyV1::new(2);
    artifact.recipe.loops[1].body = LoopBlockKeyV1::new(3);
    artifact.recipe.loops.push(second);

    artifact.recipe.items[2].item = LoopRecipeItemV1::Loop {
        loop_key: LoopNodeKeyV1::new(2),
    };
    artifact.recipe.blocks[2].owner_loop = LoopNodeKeyV1::new(2);
    artifact.recipe.exits[1].owner_loop = LoopNodeKeyV1::new(2);
    artifact.recipe.exits[1].kind = LoopExitKindV1::Break {
        target_loop: LoopNodeKeyV1::new(2),
    };

    artifact.recipe.blocks[1].items.push(LoopItemKeyV1::new(10));
    artifact.recipe.blocks.push(LoopRecipeBlockV1 {
        key: LoopBlockKeyV1::new(3),
        owner_loop: LoopNodeKeyV1::new(1),
        items: vec![LoopItemKeyV1::new(11)],
    });
    artifact.recipe.items.push(LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(10),
        item: LoopRecipeItemV1::Loop {
            loop_key: LoopNodeKeyV1::new(1),
        },
    });
    artifact.recipe.items.push(LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(11),
        item: LoopRecipeItemV1::Exit {
            exit: LoopExitKeyV1::new(2),
        },
    });
    artifact.recipe.exits.push(LoopRecipeExitV1 {
        key: LoopExitKeyV1::new(2),
        owner_loop: LoopNodeKeyV1::new(1),
        kind: LoopExitKindV1::Break {
            target_loop: LoopNodeKeyV1::new(1),
        },
    });
    assert_eq!(
        reject(artifact),
        Reject::NonCanonicalKeyOrder {
            domain: "recursive_loops"
        }
    );
}

#[test]
fn every_declared_value_must_have_a_definition() {
    let mut artifact = golden();
    artifact
        .recipe
        .values
        .push(super::schema::LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(7),
            class: LoopValueClassV1::I64,
        });
    assert_eq!(
        reject(artifact),
        Reject::UndefinedValue {
            key: LoopValueKeyV1::new(7)
        }
    );
}

#[test]
fn carrier_entry_must_exist_before_its_loop_entry() {
    let mut artifact = golden();
    artifact.recipe.carriers[0].entry_value = LoopValueKeyV1::new(1);
    assert_eq!(
        reject(artifact),
        Reject::CarrierEntryNotAvailable {
            key: LoopCarrierKeyV1::new(0)
        }
    );
}

#[test]
fn decode_error_preserves_structural_reject() {
    let mut artifact = golden();
    artifact.recipe.bindings[0].label.clear();
    let json = serde_json::to_string(&artifact).expect("fixture encodes");
    let error = LoopRecipeNormalizerV1::decode_and_verify(&json).expect_err("must reject");
    match error {
        LoopRecipeDecodeErrorV1::Rejected(Reject::EmptyBindingLabel { key }) => {
            assert_eq!(key, LoopBindingKeyV1::new(0));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn malformed_json_stays_distinct_from_structural_rejection() {
    let error = LoopRecipeNormalizerV1::decode_and_verify("{").expect_err("must reject JSON");
    assert!(matches!(error, LoopRecipeDecodeErrorV1::Json(_)));
}

#[test]
fn unknown_wire_field_is_rejected_instead_of_silently_ignored() {
    let mut value: serde_json::Value =
        serde_json::from_str(ACCUM_NESTED_GOLDEN).expect("golden value");
    value
        .as_object_mut()
        .expect("artifact object")
        .insert("legacy_family".to_owned(), serde_json::json!("loop_v0"));
    let json = serde_json::to_string(&value).expect("fixture encodes");
    assert!(matches!(
        LoopRecipeNormalizerV1::decode_and_verify(&json),
        Err(LoopRecipeDecodeErrorV1::Json(_))
    ));
}

#[test]
fn source_owner_wire_rejects_program_body() {
    let mut value: serde_json::Value =
        serde_json::from_str(ACCUM_NESTED_GOLDEN).expect("golden value");
    value["source_binding"]["owner"] = serde_json::json!({
        "kind": "program_body",
        "compilation_unit_ordinal": 0,
        "program_ordinal": 0
    });
    let json = serde_json::to_string(&value).expect("fixture encodes");
    assert!(matches!(
        LoopRecipeNormalizerV1::decode_and_verify(&json),
        Err(LoopRecipeDecodeErrorV1::Json(_))
    ));
}

#[test]
fn semantic_loop_wire_rejects_embedded_source_authority() {
    let mut value: serde_json::Value =
        serde_json::from_str(ACCUM_NESTED_GOLDEN).expect("golden value");
    value["recipe"]["loops"][0]
        .as_object_mut()
        .expect("loop object")
        .insert(
            "source".to_owned(),
            serde_json::json!({"steps": [{"kind": "body_item", "index": 2}]}),
        );
    let json = serde_json::to_string(&value).expect("fixture encodes");
    assert!(matches!(
        LoopRecipeNormalizerV1::decode_and_verify(&json),
        Err(LoopRecipeDecodeErrorV1::Json(_))
    ));
}

#[test]
fn semantic_verifier_has_no_route_or_physical_authority_import() {
    let source = include_str!("verify.rs");
    for forbidden in [
        "LoopRouteId",
        "producer_route",
        "MirBuilder",
        "CorePlan",
        "BasicBlockId",
        "ValueId",
        "RouteAttemptOutcome",
        "mutation_family",
    ] {
        assert!(
            !source.contains(forbidden),
            "semantic verifier must not contain {forbidden}"
        );
    }
}
