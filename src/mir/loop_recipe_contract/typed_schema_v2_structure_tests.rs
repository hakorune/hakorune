use super::ids::{LoopBlockKeyV1, LoopExitKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::producer_id::LoopRecipeProducerIdV1;
use super::schema::{
    LoopNodeSourceBindingV1, LoopRecipeProvenanceV1, LoopRecipeSourceBindingV1,
    LoopRecipeSourceOwnerV1, LoopSourcePathStepV1, LoopSourcePathV1,
};
use super::schema_v2::{
    LoopConditionV2, LoopExitKindV2, LoopNodeV2, LoopOperationV2, LoopRecipeArtifactV2,
    LoopRecipeBlockV2, LoopRecipeExitV2, LoopRecipeItemRowV2, LoopRecipeItemV2, LoopRecipeV2,
    LoopRecipeValueV2, LoopValueClassV2,
};
use super::typed_schema_v2::{LoopRecipeV2RejectReason as Reject, LoopRecipeVerifierV2};

fn control_artifact() -> LoopRecipeArtifactV2 {
    let predicate = LoopValueKeyV1::new(0);
    let resumed = LoopValueKeyV1::new(1);
    LoopRecipeArtifactV2::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::CallableSingleLoopV1),
        source_binding(),
        LoopRecipeV2 {
            root_loop: LoopNodeKeyV1::new(0),
            loops: vec![LoopNodeV2 {
                key: LoopNodeKeyV1::new(0),
                parent: None,
                condition: LoopConditionV2::Predicate {
                    block: LoopBlockKeyV1::new(0),
                    value: predicate,
                },
                body: LoopBlockKeyV1::new(1),
            }],
            blocks: vec![block(0, vec![]), block(1, vec![0, 2]), block(2, vec![1])],
            items: vec![
                row(
                    0,
                    LoopRecipeItemV2::If {
                        condition: predicate,
                        then_block: LoopBlockKeyV1::new(2),
                        else_block: None,
                    },
                ),
                row(
                    1,
                    LoopRecipeItemV2::Exit {
                        exit: LoopExitKeyV1::new(0),
                    },
                ),
                row(
                    2,
                    LoopRecipeItemV2::Operation {
                        operation: LoopOperationV2::ConstI64 {
                            result: resumed,
                            value: 1,
                        },
                    },
                ),
            ],
            bindings: vec![],
            values: vec![
                LoopRecipeValueV2 {
                    key: predicate,
                    class: LoopValueClassV2::Bool,
                },
                LoopRecipeValueV2 {
                    key: resumed,
                    class: LoopValueClassV2::I64,
                },
            ],
            inputs: vec![predicate],
            carriers: vec![],
            exits: vec![LoopRecipeExitV2 {
                key: LoopExitKeyV1::new(0),
                owner_loop: LoopNodeKeyV1::new(0),
                kind: LoopExitKindV2::Return { value: None },
            }],
        },
    )
}

fn source_binding() -> LoopRecipeSourceBindingV1 {
    LoopRecipeSourceBindingV1::new(
        LoopRecipeSourceOwnerV1::function_body(0, 0),
        vec![LoopNodeSourceBindingV1::new(
            LoopNodeKeyV1::new(0),
            LoopSourcePathV1::new(vec![LoopSourcePathStepV1::BodyItem { index: 0 }]),
        )],
    )
}

fn block(key: u32, items: Vec<u32>) -> LoopRecipeBlockV2 {
    LoopRecipeBlockV2 {
        key: LoopBlockKeyV1::new(key),
        owner_loop: LoopNodeKeyV1::new(0),
        items: items.into_iter().map(LoopItemKeyV1::new).collect(),
    }
}

fn row(key: u32, item: LoopRecipeItemV2) -> LoopRecipeItemRowV2 {
    LoopRecipeItemRowV2 {
        key: LoopItemKeyV1::new(key),
        item,
    }
}

#[test]
fn v2_accepts_one_sided_return_with_parent_fallthrough_structure() {
    LoopRecipeVerifierV2::verify_artifact(control_artifact())
        .expect("structure is valid without claiming JoinSig authorization");
}

#[test]
fn v2_rejects_if_child_owned_by_another_loop() {
    let mut artifact = control_artifact();
    artifact.recipe.loops.push(LoopNodeV2 {
        key: LoopNodeKeyV1::new(1),
        parent: Some(LoopNodeKeyV1::new(0)),
        condition: LoopConditionV2::Always,
        body: LoopBlockKeyV1::new(2),
    });
    artifact.recipe.blocks[2].owner_loop = LoopNodeKeyV1::new(1);
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(Reject::BlockOwnerMismatch {
            key: LoopBlockKeyV1::new(2),
        })
    );
}

#[test]
fn v2_rejects_unused_block() {
    let mut artifact = control_artifact();
    artifact.recipe.blocks.push(block(3, vec![]));
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(Reject::UnusedBlock {
            key: LoopBlockKeyV1::new(3),
        })
    );
}

#[test]
fn v2_rejects_non_recursive_item_preorder() {
    let mut artifact = control_artifact();
    artifact.recipe.blocks[1].items = vec![LoopItemKeyV1::new(0), LoopItemKeyV1::new(1)];
    artifact.recipe.blocks[2].items = vec![LoopItemKeyV1::new(2)];
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(Reject::NonCanonicalKeyOrder {
            domain: "recursive_items",
            expected: 1,
            found: 2,
        })
    );
}

#[test]
fn v2_rejects_same_exit_used_by_both_if_arms() {
    let mut artifact = control_artifact();
    artifact.recipe.blocks.push(block(3, vec![2]));
    artifact.recipe.blocks[1].items = vec![LoopItemKeyV1::new(0), LoopItemKeyV1::new(3)];
    artifact.recipe.blocks[2].items = vec![LoopItemKeyV1::new(1)];
    artifact.recipe.items[0].item = LoopRecipeItemV2::If {
        condition: LoopValueKeyV1::new(0),
        then_block: LoopBlockKeyV1::new(2),
        else_block: Some(LoopBlockKeyV1::new(3)),
    };
    artifact.recipe.items[2].item = LoopRecipeItemV2::Exit {
        exit: LoopExitKeyV1::new(0),
    };
    artifact.recipe.items.push(row(
        3,
        LoopRecipeItemV2::Operation {
            operation: LoopOperationV2::ConstI64 {
                result: LoopValueKeyV1::new(1),
                value: 1,
            },
        },
    ));
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(Reject::DuplicateExitUse {
            key: LoopExitKeyV1::new(0),
        })
    );
}

#[test]
fn v2_rejects_duplicate_source_loop_path() {
    let mut artifact = control_artifact();
    artifact.recipe.loops.push(LoopNodeV2 {
        key: LoopNodeKeyV1::new(1),
        parent: Some(LoopNodeKeyV1::new(0)),
        condition: LoopConditionV2::Always,
        body: LoopBlockKeyV1::new(3),
    });
    artifact.recipe.blocks.push(LoopRecipeBlockV2 {
        key: LoopBlockKeyV1::new(3),
        owner_loop: LoopNodeKeyV1::new(1),
        items: vec![],
    });
    artifact.recipe.items.push(row(
        3,
        LoopRecipeItemV2::Loop {
            loop_key: LoopNodeKeyV1::new(1),
        },
    ));
    artifact.recipe.blocks[1].items.push(LoopItemKeyV1::new(3));
    artifact
        .source_binding
        .loops
        .push(artifact.source_binding.loops[0].clone());
    artifact.source_binding.loops[1].loop_key = LoopNodeKeyV1::new(1);
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(Reject::SourceBinding(
            super::error::LoopRecipeRejectReasonV1::DuplicateLoopSourcePath {
                first: LoopNodeKeyV1::new(0),
                second: LoopNodeKeyV1::new(1),
            },
        ))
    );
}
