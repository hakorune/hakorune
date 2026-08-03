use super::*;

fn path(steps: Vec<IfSourcePathStepV1>) -> IfSourcePathV1 {
    IfSourcePathV1 { steps }
}

fn source_binding() -> IfRecipeSourceBindingV1 {
    IfRecipeSourceBindingV1 {
        owner: IfRecipeSourceOwnerV1::FunctionBody {
            compilation_unit_ordinal: 0,
            function_ordinal: 0,
        },
        claims: vec![
            IfSourceClaimV1 {
                role: IfSourceClaimRoleV1::IfNode,
                path: path(vec![IfSourcePathStepV1::BodyItem { index: 1 }]),
            },
            IfSourceClaimV1 {
                role: IfSourceClaimRoleV1::Condition,
                path: path(vec![
                    IfSourcePathStepV1::BodyItem { index: 1 },
                    IfSourcePathStepV1::IfCondition,
                ]),
            },
            IfSourceClaimV1 {
                role: IfSourceClaimRoleV1::ThenAssignment,
                path: path(vec![
                    IfSourcePathStepV1::BodyItem { index: 1 },
                    IfSourcePathStepV1::IfThenItem { index: 0 },
                ]),
            },
            IfSourceClaimV1 {
                role: IfSourceClaimRoleV1::ElseAssignment,
                path: path(vec![
                    IfSourcePathStepV1::BodyItem { index: 1 },
                    IfSourcePathStepV1::IfElseItem { index: 0 },
                ]),
            },
        ],
    }
}

fn item(key: u32, operation: IfOperationV1) -> IfRecipeItemRowV1 {
    IfRecipeItemRowV1 {
        key: IfItemKeyV1::new(key),
        operation,
    }
}

fn golden() -> IfRecipeArtifactV1 {
    let b = IfBindingKeyV1::new(0);
    let values = (0..7)
        .map(|raw| IfRecipeValueV1 {
            key: IfValueKeyV1::new(raw),
            class: if raw == 3 {
                IfValueClassV1::Bool
            } else {
                IfValueClassV1::I64
            },
        })
        .collect();
    IfRecipeArtifactV1::new(
        IfRecipeProvenanceV1 {
            profile: IfRecipeProfileV1::ResolvedTrivialExplicitElse,
        },
        source_binding(),
        IfRecipeV1 {
            condition_block: IfRecipeBlockV1 {
                key: IfBlockKeyV1::new(0),
                role: IfBlockRoleV1::Condition,
                items: vec![
                    item(
                        0,
                        IfOperationV1::ReadBinding {
                            binding: b,
                            result: IfValueKeyV1::new(1),
                        },
                    ),
                    item(
                        1,
                        IfOperationV1::ConstI64 {
                            result: IfValueKeyV1::new(2),
                            value: 1,
                        },
                    ),
                    item(
                        2,
                        IfOperationV1::CompareI64 {
                            op: IfCompareOpV1::Less,
                            left: IfValueKeyV1::new(1),
                            right: IfValueKeyV1::new(2),
                            result: IfValueKeyV1::new(3),
                        },
                    ),
                ],
            },
            then_block: IfRecipeBlockV1 {
                key: IfBlockKeyV1::new(1),
                role: IfBlockRoleV1::Then,
                items: vec![
                    item(
                        3,
                        IfOperationV1::ConstI64 {
                            result: IfValueKeyV1::new(4),
                            value: 1,
                        },
                    ),
                    item(
                        4,
                        IfOperationV1::WriteBinding {
                            binding: b,
                            value: IfValueKeyV1::new(4),
                        },
                    ),
                ],
            },
            else_block: Some(IfRecipeBlockV1 {
                key: IfBlockKeyV1::new(2),
                role: IfBlockRoleV1::Else,
                items: vec![
                    item(
                        5,
                        IfOperationV1::ConstI64 {
                            result: IfValueKeyV1::new(5),
                            value: 2,
                        },
                    ),
                    item(
                        6,
                        IfOperationV1::WriteBinding {
                            binding: b,
                            value: IfValueKeyV1::new(5),
                        },
                    ),
                ],
            }),
            continuation_block: IfRecipeBlockV1 {
                key: IfBlockKeyV1::new(3),
                role: IfBlockRoleV1::Continuation,
                items: vec![item(
                    7,
                    IfOperationV1::ReadBinding {
                        binding: b,
                        result: IfValueKeyV1::new(6),
                    },
                )],
            },
            else_disposition: IfElseDispositionV1::Explicit,
            condition: IfValueKeyV1::new(3),
            inputs: vec![IfValueKeyV1::new(0)],
            bindings: vec![IfRecipeBindingV1 {
                key: b,
                role: IfBindingRoleV1::MergeTarget,
                class: IfValueClassV1::I64,
            }],
            values,
            joins: vec![IfJoinRowV1 {
                binding: b,
                class: IfValueClassV1::I64,
                entry_value: IfValueKeyV1::new(0),
                then_value: IfValueKeyV1::new(4),
                else_value: IfValueKeyV1::new(5),
            }],
            continuation: IfContinuationV1 { required_read: b },
        },
    )
}

#[test]
fn golden_explicit_else_verifies_and_normalizes_deterministically() {
    let artifact = golden();
    let verified = IfRecipeVerifierV1::verify_artifact(artifact.clone()).expect("golden verifies");
    let normalized = IfRecipeNormalizerV1::normalize_artifact(&verified).expect("normalize");
    let decoded: IfRecipeArtifactV1 = serde_json::from_str(&normalized).expect("decode");
    assert_eq!(decoded, artifact);
    let semantic = IfRecipeNormalizerV1::normalize_semantic(verified.recipe()).expect("semantic");
    let semantic_again =
        IfRecipeNormalizerV1::normalize_semantic(verified.recipe()).expect("repeat");
    assert_eq!(semantic, semantic_again);
}

#[test]
fn semantic_normalization_excludes_source_and_receipt() {
    let original = golden();
    let mut alternate = original.clone();
    alternate.source_binding.owner = IfRecipeSourceOwnerV1::FunctionBody {
        compilation_unit_ordinal: 2,
        function_ordinal: 4,
    };
    let left = IfRecipeVerifierV1::verify_artifact(original).expect("left");
    let right = IfRecipeVerifierV1::verify_artifact(alternate).expect("right");
    assert_eq!(
        IfRecipeNormalizerV1::normalize_semantic(left.recipe()).expect("left semantic"),
        IfRecipeNormalizerV1::normalize_semantic(right.recipe()).expect("right semantic")
    );
    assert_ne!(
        IfRecipeNormalizerV1::normalize_source_bound(&left).expect("left source"),
        IfRecipeNormalizerV1::normalize_source_bound(&right).expect("right source")
    );
}

#[test]
fn typed_rejects_cover_version_keys_else_and_join() {
    let mut wrong_version = golden();
    wrong_version.schema_version = 2;
    assert_eq!(
        IfRecipeVerifierV1::verify_artifact(wrong_version).unwrap_err(),
        IfRecipeRejectReasonV1::UnsupportedVersion { found: 2 }
    );

    let mut wrong_items = golden();
    wrong_items.recipe.then_block.items[0].key = IfItemKeyV1::new(99);
    assert!(matches!(
        IfRecipeVerifierV1::verify_artifact(wrong_items),
        Err(IfRecipeRejectReasonV1::NonCanonicalKeyOrder { domain: "items" })
    ));

    let mut implicit = golden();
    implicit.recipe.else_disposition = IfElseDispositionV1::ImplicitFallthrough;
    assert_eq!(
        IfRecipeVerifierV1::verify_artifact(implicit).unwrap_err(),
        IfRecipeRejectReasonV1::ExplicitElseRequired
    );

    let mut wrong_join = golden();
    wrong_join.recipe.joins[0].then_value = IfValueKeyV1::new(0);
    assert_eq!(
        IfRecipeVerifierV1::verify_artifact(wrong_join).unwrap_err(),
        IfRecipeRejectReasonV1::JoinValueMismatch
    );
}

#[test]
fn source_claim_requires_fixed_role_order_and_path_shape() {
    let mut wrong_order = golden();
    wrong_order.source_binding.claims.swap(0, 1);
    assert_eq!(
        IfRecipeVerifierV1::verify_artifact(wrong_order).unwrap_err(),
        IfRecipeRejectReasonV1::SourceClaimOrderMismatch
    );

    let mut wrong_path = golden();
    wrong_path.source_binding.claims[0].path.steps = vec![IfSourcePathStepV1::IfCondition];
    assert_eq!(
        IfRecipeVerifierV1::verify_artifact(wrong_path).unwrap_err(),
        IfRecipeRejectReasonV1::InvalidSourcePath
    );
}

#[test]
fn unknown_json_fields_are_rejected_before_verification() {
    let json = serde_json::to_string(&golden()).expect("encode");
    let mut value: serde_json::Value = serde_json::from_str(&json).expect("value");
    value["recipe"]["unknown"] = serde_json::Value::Bool(true);
    let error = IfRecipeNormalizerV1::decode_and_verify(&value.to_string()).unwrap_err();
    assert!(matches!(error, IfRecipeDecodeErrorV1::Json(_)));
}

#[test]
fn logical_joinsig_elaborates_fixed_shell_deterministically() {
    let artifact = golden();
    let verified = IfRecipeVerifierV1::verify_artifact(artifact).expect("golden verifies");
    let left = IfJoinSigElaboratorV1::elaborate(verified.recipe()).expect("left JoinSig");
    let right = IfJoinSigElaboratorV1::elaborate(verified.recipe()).expect("right JoinSig");
    assert_eq!(left, right);

    let sig = left.as_sig();
    assert_eq!(sig.ports.len(), 5);
    assert_eq!(sig.edges.len(), 5);
    assert_eq!(sig.edges[0].role, IfJoinEdgeRoleV1::Enter);
    assert_eq!(sig.edges[0].from, IfJoinPortV1::Entry);
    assert_eq!(sig.edges[0].to, IfJoinPortV1::Condition);
    assert_eq!(sig.edges[1].role, IfJoinEdgeRoleV1::True);
    assert_eq!(sig.edges[1].to, IfJoinPortV1::Then);
    assert_eq!(sig.edges[2].role, IfJoinEdgeRoleV1::False);
    assert_eq!(sig.edges[2].to, IfJoinPortV1::Else);
    assert_eq!(sig.edges[3].role, IfJoinEdgeRoleV1::ThenTransfer);
    assert_eq!(sig.edges[3].from, IfJoinPortV1::Then);
    assert_eq!(sig.edges[4].role, IfJoinEdgeRoleV1::ElseTransfer);
    assert_eq!(sig.edges[4].from, IfJoinPortV1::Else);
    assert_eq!(sig.edges[3].to, IfJoinPortV1::Continuation);
    assert_eq!(sig.edges[4].to, IfJoinPortV1::Continuation);
    assert_eq!(sig.join.binding, IfBindingKeyV1::new(0));
    assert_eq!(sig.join.entry_value, IfValueKeyV1::new(0));
    assert_eq!(sig.join.then_value, IfValueKeyV1::new(4));
    assert_eq!(sig.join.else_value, IfValueKeyV1::new(5));
    assert_eq!(sig.join.class, IfValueClassV1::I64);
}
