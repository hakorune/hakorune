use super::ids::{LoopBindingKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::join_sig::{LoopJoinEdgeRoleV1, LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1};
use super::schema::{
    LoopBinaryI64OpV1, LoopConditionV1, LoopExitKindV1, LoopOperationV1, LoopRecipeArtifactV1,
    LoopRecipeBlockV1, LoopRecipeExitV1, LoopRecipeItemRowV1, LoopRecipeItemV1, LoopRecipeValueV1,
    LoopValueClassV1,
};
use super::verify::LoopRecipeVerifierV1;

const NESTED_PREDICATE_GOLDEN: &str = include_str!("fixtures/nested_predicate_v1.json");

fn nested_predicate_recipe() -> super::verify::VerifiedLoopRecipeV1 {
    let artifact = nested_artifact();
    LoopRecipeVerifierV1::verify(artifact.recipe).expect("nested predicate recipe verifies")
}

fn nested_artifact() -> LoopRecipeArtifactV1 {
    serde_json::from_str(NESTED_PREDICATE_GOLDEN).expect("nested predicate golden")
}

#[test]
fn nested_predicate_artifact_roundtrips() {
    let artifact = nested_artifact();
    LoopRecipeVerifierV1::verify_artifact(artifact).expect("nested predicate artifact verifies");
}

#[test]
fn join_sig_nested_predicate_is_deterministic_and_scoped() {
    let left = LoopJoinSigElaboratorV1::elaborate(&nested_predicate_recipe())
        .expect("left nested predicate JoinSig");
    let right = LoopJoinSigElaboratorV1::elaborate(&nested_predicate_recipe())
        .expect("right nested predicate JoinSig");
    assert_eq!(left.as_sig(), right.as_sig());
    assert_eq!(left.as_sig().loops.len(), 2);

    let root = &left.as_sig().loops[0];
    let child = &left.as_sig().loops[1];
    assert_eq!(root.carriers.len(), 2);
    assert_eq!(child.carriers.len(), 1);
    assert_eq!(
        root.edges.iter().map(|edge| edge.role).collect::<Vec<_>>(),
        vec![
            LoopJoinEdgeRoleV1::Enter,
            LoopJoinEdgeRoleV1::PredicateTrue,
            LoopJoinEdgeRoleV1::PredicateFalse,
            LoopJoinEdgeRoleV1::Backedge,
        ]
    );
    assert_eq!(
        child.edges.iter().map(|edge| edge.role).collect::<Vec<_>>(),
        vec![
            LoopJoinEdgeRoleV1::Enter,
            LoopJoinEdgeRoleV1::PredicateTrue,
            LoopJoinEdgeRoleV1::PredicateFalse,
            LoopJoinEdgeRoleV1::Backedge,
        ]
    );
    let child_backedge = child
        .edges
        .iter()
        .find(|edge| edge.role == LoopJoinEdgeRoleV1::Backedge)
        .expect("child backedge");
    assert_eq!(
        child_backedge
            .payload
            .iter()
            .map(|payload| (payload.binding, payload.value))
            .collect::<Vec<_>>(),
        vec![
            (LoopBindingKeyV1::new(0), LoopValueKeyV1::new(0)),
            (LoopBindingKeyV1::new(1), LoopValueKeyV1::new(11)),
            (LoopBindingKeyV1::new(2), LoopValueKeyV1::new(14)),
        ]
    );
    let root_backedge = root
        .edges
        .iter()
        .find(|edge| edge.role == LoopJoinEdgeRoleV1::Backedge)
        .expect("root backedge");
    assert_eq!(
        root_backedge
            .payload
            .iter()
            .map(|payload| (payload.binding, payload.value))
            .collect::<Vec<_>>(),
        vec![
            (LoopBindingKeyV1::new(0), LoopValueKeyV1::new(17)),
            (LoopBindingKeyV1::new(1), LoopValueKeyV1::new(11)),
        ]
    );
    assert!(root
        .edges
        .iter()
        .flat_map(|edge| edge.payload.iter())
        .all(|payload| payload.binding != LoopBindingKeyV1::new(2)));
}

#[test]
fn nested_predicate_rejects_parent_tail_use_of_child_binding() {
    let artifact = nested_artifact();
    let mut recipe = artifact.recipe;
    recipe.items[16].item = LoopRecipeItemV1::Operation {
        operation: LoopOperationV1::ReadBinding {
            binding: LoopBindingKeyV1::new(2),
            result: LoopValueKeyV1::new(15),
        },
    };
    let verified = LoopRecipeVerifierV1::verify(recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::BindingNotAvailable {
            binding: LoopBindingKeyV1::new(2),
        })
    );
}

#[test]
fn nested_predicate_keeps_typed_reject_for_impure_condition() {
    let artifact = nested_artifact();
    let mut recipe = artifact.recipe;
    recipe.items[5].item = LoopRecipeItemV1::Operation {
        operation: LoopOperationV1::BinaryI64 {
            op: LoopBinaryI64OpV1::Add,
            left: LoopValueKeyV1::new(5),
            right: LoopValueKeyV1::new(5),
            result: LoopValueKeyV1::new(6),
        },
    };
    if let LoopRecipeItemV1::Operation {
        operation: LoopOperationV1::CompareI64 { left, .. },
    } = &mut recipe.items[7].item
    {
        *left = LoopValueKeyV1::new(6);
    }
    let verified = LoopRecipeVerifierV1::verify(recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::UnsupportedNestedPredicate {
            loop_key: LoopNodeKeyV1::new(1),
        })
    );
}

#[test]
fn nested_predicate_rejects_missing_child_carrier() {
    let mut artifact = nested_artifact();
    artifact
        .recipe
        .carriers
        .retain(|carrier| carrier.binding != LoopBindingKeyV1::new(2));
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::BindingNotAvailable {
            binding: LoopBindingKeyV1::new(2),
        })
    );
}

#[test]
fn nested_predicate_rejects_missing_ancestor_carrier() {
    let mut artifact = nested_artifact();
    artifact
        .recipe
        .carriers
        .retain(|carrier| carrier.binding != LoopBindingKeyV1::new(1));
    artifact.recipe.carriers[1].key = super::ids::LoopCarrierKeyV1::new(1);
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::BindingNotAvailable {
            binding: LoopBindingKeyV1::new(1),
        })
    );
}

#[test]
fn nested_predicate_rejects_unavailable_predicate_value() {
    let mut artifact = nested_artifact();
    artifact.recipe.values.push(LoopRecipeValueV1 {
        key: LoopValueKeyV1::new(18),
        class: LoopValueClassV1::Bool,
    });
    artifact.recipe.items.push(LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(20),
        item: LoopRecipeItemV1::Operation {
            operation: LoopOperationV1::CompareI64 {
                op: super::schema::LoopCompareI64OpV1::Less,
                left: LoopValueKeyV1::new(15),
                right: LoopValueKeyV1::new(16),
                result: LoopValueKeyV1::new(18),
            },
        },
    });
    artifact.recipe.blocks[1].items.push(LoopItemKeyV1::new(20));
    if let LoopConditionV1::Predicate { value, .. } = &mut artifact.recipe.loops[1].condition {
        *value = LoopValueKeyV1::new(18);
    }
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::ValueNotAvailable {
            value: LoopValueKeyV1::new(18),
        })
    );
}

#[test]
fn nested_predicate_rejects_explicit_child_exit() {
    let mut artifact = nested_artifact();
    artifact.recipe.items[15].item = LoopRecipeItemV1::Exit {
        exit: super::ids::LoopExitKeyV1::new(0),
    };
    artifact.recipe.exits.push(LoopRecipeExitV1 {
        key: super::ids::LoopExitKeyV1::new(0),
        owner_loop: LoopNodeKeyV1::new(1),
        kind: LoopExitKindV1::Break {
            target_loop: LoopNodeKeyV1::new(1),
        },
    });
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::UnsupportedNestedPredicate {
            loop_key: LoopNodeKeyV1::new(1),
        })
    );
}

#[test]
fn nested_predicate_rejects_branch_shape() {
    let mut artifact = nested_artifact();
    artifact.recipe.blocks.push(LoopRecipeBlockV1 {
        key: super::ids::LoopBlockKeyV1::new(4),
        owner_loop: LoopNodeKeyV1::new(1),
        items: vec![LoopItemKeyV1::new(9)],
    });
    artifact.recipe.blocks[3]
        .items
        .retain(|item| *item != LoopItemKeyV1::new(9));
    artifact.recipe.items[8].item = LoopRecipeItemV1::If {
        condition: LoopValueKeyV1::new(4),
        then_block: super::ids::LoopBlockKeyV1::new(4),
        else_block: None,
    };
    artifact.recipe.items[9].item = LoopRecipeItemV1::Operation {
        operation: LoopOperationV1::ConstI64 {
            result: LoopValueKeyV1::new(9),
            value: 0,
        },
    };
    artifact.recipe.items[10].item = LoopRecipeItemV1::Operation {
        operation: LoopOperationV1::ConstI64 {
            result: LoopValueKeyV1::new(10),
            value: 1,
        },
    };
    artifact.recipe.items[11].item = LoopRecipeItemV1::Operation {
        operation: LoopOperationV1::ConstI64 {
            result: LoopValueKeyV1::new(11),
            value: 1,
        },
    };
    artifact.recipe.items[12].item = LoopRecipeItemV1::Operation {
        operation: LoopOperationV1::ConstI64 {
            result: LoopValueKeyV1::new(12),
            value: 0,
        },
    };
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::UnsupportedNestedPredicate {
            loop_key: LoopNodeKeyV1::new(1),
        })
    );
}

#[test]
fn nested_predicate_rejects_deeper_predicate() {
    let mut artifact = nested_artifact();
    artifact.recipe.loops.push(super::schema::LoopNodeV1 {
        key: LoopNodeKeyV1::new(2),
        parent: Some(LoopNodeKeyV1::new(1)),
        condition: LoopConditionV1::Always,
        body: super::ids::LoopBlockKeyV1::new(4),
    });
    artifact.recipe.blocks.push(LoopRecipeBlockV1 {
        key: super::ids::LoopBlockKeyV1::new(4),
        owner_loop: LoopNodeKeyV1::new(2),
        items: Vec::new(),
    });
    artifact.recipe.items[8].item = LoopRecipeItemV1::Loop {
        loop_key: LoopNodeKeyV1::new(2),
    };
    artifact.recipe.items.push(LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(20),
        item: LoopRecipeItemV1::Operation {
            operation: LoopOperationV1::ReadBinding {
                binding: LoopBindingKeyV1::new(1),
                result: LoopValueKeyV1::new(9),
            },
        },
    });
    artifact.recipe.blocks[1].items.push(LoopItemKeyV1::new(20));
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe).expect("shape still verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::UnsupportedNestedPredicate {
            loop_key: LoopNodeKeyV1::new(1),
        })
    );
}
