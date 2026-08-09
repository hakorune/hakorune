use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopValueKeyV1,
};
use super::producer_id::LoopRecipeProducerIdV1;
use super::schema::{
    LoopNodeSourceBindingV1, LoopRecipeProvenanceV1, LoopRecipeSourceBindingV1,
    LoopRecipeSourceOwnerV1, LoopSourcePathStepV1, LoopSourcePathV1,
};
use super::schema_v2::{
    LoopConditionV2, LoopOperationV2, LoopRecipeArtifactV2, LoopRecipeBindingV2, LoopRecipeBlockV2,
    LoopRecipeCarrierV2, LoopRecipeItemRowV2, LoopRecipeItemV2, LoopRecipeV2, LoopRecipeValueV2,
    LoopValueClassV2,
};
use super::typed_schema_v2::{LoopRecipeV2RejectReason, LoopRecipeVerifierV2};

fn value(key: u32, class: LoopValueClassV2) -> LoopRecipeValueV2 {
    LoopRecipeValueV2 {
        key: LoopValueKeyV1::new(key),
        class,
    }
}

fn operation(key: u32, operation: LoopOperationV2) -> LoopRecipeItemRowV2 {
    LoopRecipeItemRowV2 {
        key: LoopItemKeyV1::new(key),
        item: LoopRecipeItemV2::Operation { operation },
    }
}

fn dynamic_operation_recipe() -> LoopRecipeArtifactV2 {
    let binding = LoopBindingKeyV1::new(0);
    let recipe = LoopRecipeV2 {
        root_loop: LoopNodeKeyV1::new(0),
        loops: vec![super::schema_v2::LoopNodeV2 {
            key: LoopNodeKeyV1::new(0),
            parent: None,
            condition: LoopConditionV2::Predicate {
                block: LoopBlockKeyV1::new(0),
                value: LoopValueKeyV1::new(3),
            },
            body: LoopBlockKeyV1::new(1),
        }],
        blocks: vec![
            LoopRecipeBlockV2 {
                key: LoopBlockKeyV1::new(0),
                owner_loop: LoopNodeKeyV1::new(0),
                items: (0..2).map(LoopItemKeyV1::new).collect(),
            },
            LoopRecipeBlockV2 {
                key: LoopBlockKeyV1::new(1),
                owner_loop: LoopNodeKeyV1::new(0),
                items: (2..10).map(LoopItemKeyV1::new).collect(),
            },
        ],
        items: vec![
            operation(
                0,
                LoopOperationV2::ReadBinding {
                    binding,
                    result: LoopValueKeyV1::new(2),
                },
            ),
            operation(
                1,
                LoopOperationV2::DynamicLess {
                    left: LoopValueKeyV1::new(2),
                    right: LoopValueKeyV1::new(1),
                    result: LoopValueKeyV1::new(3),
                },
            ),
            operation(
                2,
                LoopOperationV2::ConstI64 {
                    result: LoopValueKeyV1::new(4),
                    value: 1,
                },
            ),
            operation(
                3,
                LoopOperationV2::DynamicAdd {
                    left: LoopValueKeyV1::new(2),
                    right: LoopValueKeyV1::new(4),
                    result: LoopValueKeyV1::new(5),
                },
            ),
            operation(
                4,
                LoopOperationV2::CallSlot {
                    receiver: Some(LoopValueKeyV1::new(2)),
                    args: vec![],
                    result: Some(LoopValueKeyV1::new(6)),
                },
            ),
            operation(
                5,
                LoopOperationV2::ConstI64 {
                    result: LoopValueKeyV1::new(7),
                    value: 0,
                },
            ),
            operation(
                6,
                LoopOperationV2::DynamicLess {
                    left: LoopValueKeyV1::new(6),
                    right: LoopValueKeyV1::new(7),
                    result: LoopValueKeyV1::new(8),
                },
            ),
            operation(
                7,
                LoopOperationV2::ConstI64 {
                    result: LoopValueKeyV1::new(9),
                    value: 1,
                },
            ),
            operation(
                8,
                LoopOperationV2::DynamicAdd {
                    left: LoopValueKeyV1::new(2),
                    right: LoopValueKeyV1::new(9),
                    result: LoopValueKeyV1::new(10),
                },
            ),
            operation(
                9,
                LoopOperationV2::WriteBinding {
                    binding,
                    value: LoopValueKeyV1::new(10),
                },
            ),
        ],
        bindings: vec![LoopRecipeBindingV2 {
            key: binding,
            label: "induction".to_string(),
            class: LoopValueClassV2::Dynamic,
        }],
        values: vec![
            value(0, LoopValueClassV2::Dynamic),
            value(1, LoopValueClassV2::Dynamic),
            value(2, LoopValueClassV2::Dynamic),
            value(3, LoopValueClassV2::Bool),
            value(4, LoopValueClassV2::I64),
            value(5, LoopValueClassV2::Dynamic),
            value(6, LoopValueClassV2::Dynamic),
            value(7, LoopValueClassV2::I64),
            value(8, LoopValueClassV2::Bool),
            value(9, LoopValueClassV2::I64),
            value(10, LoopValueClassV2::Dynamic),
        ],
        inputs: vec![LoopValueKeyV1::new(0), LoopValueKeyV1::new(1)],
        carriers: vec![LoopRecipeCarrierV2 {
            key: LoopCarrierKeyV1::new(0),
            owner_loop: LoopNodeKeyV1::new(0),
            binding,
            class: LoopValueClassV2::Dynamic,
            entry_value: LoopValueKeyV1::new(0),
        }],
        exits: vec![],
    };
    LoopRecipeArtifactV2::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::CallableSingleLoopV1),
        LoopRecipeSourceBindingV1::new(
            LoopRecipeSourceOwnerV1::function_body(0, 0),
            vec![LoopNodeSourceBindingV1::new(
                LoopNodeKeyV1::new(0),
                LoopSourcePathV1::new(vec![LoopSourcePathStepV1::BodyItem { index: 0 }]),
            )],
        ),
        recipe,
    )
}

#[test]
fn v2_dynamic_operations_cover_the_four_unchanged_source_roles() {
    let artifact = dynamic_operation_recipe();
    let json = serde_json::to_string(&artifact).expect("Dynamic operations encode");
    assert_eq!(json.matches("dynamic_add").count(), 2);
    assert_eq!(json.matches("dynamic_less").count(), 2);

    let literals = artifact
        .recipe
        .items
        .iter()
        .filter_map(|row| match row.item {
            LoopRecipeItemV2::Operation {
                operation: LoopOperationV2::ConstI64 { value, .. },
            } => Some((row.key, value)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        literals,
        vec![
            (LoopItemKeyV1::new(2), 1),
            (LoopItemKeyV1::new(5), 0),
            (LoopItemKeyV1::new(7), 1),
        ]
    );

    let decoded = serde_json::from_str(&json).expect("Dynamic operations decode");
    LoopRecipeVerifierV2::verify_artifact(decoded).expect("Dynamic operations verify");
}

#[test]
fn v2_dynamic_add_rejects_reversed_operands() {
    let mut artifact = dynamic_operation_recipe();
    if let LoopRecipeItemV2::Operation {
        operation: LoopOperationV2::DynamicAdd { left, right, .. },
    } = &mut artifact.recipe.items[3].item
    {
        std::mem::swap(left, right);
    }
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(LoopRecipeV2RejectReason::InvalidOperationDomain {
            item: LoopItemKeyV1::new(3),
        })
    );
}

#[test]
fn v2_dynamic_add_rejects_dynamic_rhs() {
    let mut artifact = dynamic_operation_recipe();
    if let LoopRecipeItemV2::Operation {
        operation: LoopOperationV2::DynamicAdd { right, .. },
    } = &mut artifact.recipe.items[3].item
    {
        *right = LoopValueKeyV1::new(1);
    }
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(LoopRecipeV2RejectReason::InvalidOperationDomain {
            item: LoopItemKeyV1::new(3),
        })
    );
}

#[test]
fn v2_dynamic_less_rejects_non_dynamic_left() {
    let mut artifact = dynamic_operation_recipe();
    if let LoopRecipeItemV2::Operation {
        operation: LoopOperationV2::DynamicLess { left, .. },
    } = &mut artifact.recipe.items[6].item
    {
        *left = LoopValueKeyV1::new(7);
    }
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(LoopRecipeV2RejectReason::InvalidOperationDomain {
            item: LoopItemKeyV1::new(6),
        })
    );
}

#[test]
fn v2_dynamic_less_rejects_bool_rhs() {
    let mut artifact = dynamic_operation_recipe();
    if let LoopRecipeItemV2::Operation {
        operation: LoopOperationV2::DynamicLess { right, .. },
    } = &mut artifact.recipe.items[6].item
    {
        *right = LoopValueKeyV1::new(3);
    }
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(LoopRecipeV2RejectReason::InvalidOperationDomain {
            item: LoopItemKeyV1::new(6),
        })
    );
}

#[test]
fn v2_dynamic_add_rejects_wrong_result_class() {
    let mut artifact = dynamic_operation_recipe();
    artifact.recipe.values[10].class = LoopValueClassV2::Bool;
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(LoopRecipeV2RejectReason::ValueClassMismatch {
            key: LoopValueKeyV1::new(10),
        })
    );
}

#[test]
fn v2_dynamic_less_rejects_forward_call_result() {
    let mut artifact = dynamic_operation_recipe();
    if let LoopRecipeItemV2::Operation {
        operation: LoopOperationV2::DynamicLess { left, .. },
    } = &mut artifact.recipe.items[1].item
    {
        *left = LoopValueKeyV1::new(6);
    }
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(LoopRecipeV2RejectReason::ValueUsedBeforeDefinition {
            item: LoopItemKeyV1::new(1),
            key: LoopValueKeyV1::new(6),
        })
    );
}

#[test]
fn v1_does_not_decode_v2_dynamic_operation_wire() {
    let json = serde_json::to_string(&dynamic_operation_recipe()).expect("V2 encodes");
    assert!(serde_json::from_str::<super::schema::LoopRecipeArtifactV1>(&json).is_err());
}
