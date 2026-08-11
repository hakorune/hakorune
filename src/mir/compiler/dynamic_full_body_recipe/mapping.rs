//! Deterministic complete V2 Recipe for the accepted Dynamic Loop body.

use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV2, LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopCompareI64OpV2,
    LoopConditionV2, LoopExitKeyV1, LoopExitKindV2, LoopItemKeyV1, LoopNodeKeyV1, LoopNodeV2,
    LoopOperationV2, LoopRecipeBindingV2, LoopRecipeBlockV2, LoopRecipeCarrierV2, LoopRecipeExitV2,
    LoopRecipeItemRowV2, LoopRecipeItemV2, LoopRecipeV2, LoopRecipeValueV2, LoopValueClassV2,
    LoopValueKeyV1,
};

pub(super) fn complete_dynamic_loop_recipe_v2() -> LoopRecipeV2 {
    let binding = LoopBindingKeyV1::new(0);
    LoopRecipeV2 {
        root_loop: LoopNodeKeyV1::new(0),
        loops: vec![LoopNodeV2 {
            key: LoopNodeKeyV1::new(0),
            parent: None,
            condition: LoopConditionV2::Predicate {
                block: LoopBlockKeyV1::new(0),
                value: LoopValueKeyV1::new(5),
            },
            body: LoopBlockKeyV1::new(1),
        }],
        blocks: vec![
            block(0, &[0, 1]),
            block(1, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14, 15, 16]),
            block(2, &[11, 12]),
        ],
        items: vec![
            operation(
                0,
                LoopOperationV2::ReadBinding {
                    binding,
                    result: value_key(4),
                },
            ),
            operation(
                1,
                LoopOperationV2::CompareI64 {
                    op: LoopCompareI64OpV2::Less,
                    left: value_key(4),
                    right: value_key(2),
                    result: value_key(5),
                },
            ),
            operation(
                2,
                LoopOperationV2::ReadBinding {
                    binding,
                    result: value_key(6),
                },
            ),
            operation(
                3,
                LoopOperationV2::ReadBinding {
                    binding,
                    result: value_key(7),
                },
            ),
            operation(
                4,
                LoopOperationV2::ConstI64 {
                    result: value_key(8),
                    value: 1,
                },
            ),
            operation(
                5,
                LoopOperationV2::BinaryI64 {
                    op: LoopBinaryI64OpV2::Add,
                    left: value_key(7),
                    right: value_key(8),
                    result: value_key(9),
                },
            ),
            operation(
                6,
                LoopOperationV2::CallSlot {
                    receiver: Some(value_key(0)),
                    args: vec![value_key(6), value_key(9)],
                    result: Some(value_key(10)),
                },
            ),
            operation(
                7,
                LoopOperationV2::CallSlot {
                    receiver: Some(value_key(3)),
                    args: vec![value_key(10)],
                    result: Some(value_key(11)),
                },
            ),
            operation(
                8,
                LoopOperationV2::ConstI64 {
                    result: value_key(12),
                    value: 0,
                },
            ),
            operation(
                9,
                LoopOperationV2::DynamicLess {
                    left: value_key(11),
                    right: value_key(12),
                    result: value_key(13),
                },
            ),
            LoopRecipeItemRowV2 {
                key: item_key(10),
                item: LoopRecipeItemV2::If {
                    condition: value_key(13),
                    then_block: LoopBlockKeyV1::new(2),
                    else_block: None,
                },
            },
            operation(
                11,
                LoopOperationV2::ReadBinding {
                    binding,
                    result: value_key(14),
                },
            ),
            LoopRecipeItemRowV2 {
                key: item_key(12),
                item: LoopRecipeItemV2::Exit {
                    exit: LoopExitKeyV1::new(0),
                },
            },
            operation(
                13,
                LoopOperationV2::ReadBinding {
                    binding,
                    result: value_key(15),
                },
            ),
            operation(
                14,
                LoopOperationV2::ConstI64 {
                    result: value_key(16),
                    value: 1,
                },
            ),
            operation(
                15,
                LoopOperationV2::BinaryI64 {
                    op: LoopBinaryI64OpV2::Add,
                    left: value_key(15),
                    right: value_key(16),
                    result: value_key(17),
                },
            ),
            operation(
                16,
                LoopOperationV2::WriteBinding {
                    binding,
                    value: value_key(17),
                },
            ),
        ],
        bindings: vec![LoopRecipeBindingV2 {
            key: binding,
            label: "induction".to_owned(),
            class: LoopValueClassV2::I64,
        }],
        values: (0..18).map(|raw| value(raw, value_class(raw))).collect(),
        inputs: (0..4).map(value_key).collect(),
        carriers: vec![LoopRecipeCarrierV2 {
            key: LoopCarrierKeyV1::new(0),
            owner_loop: LoopNodeKeyV1::new(0),
            binding,
            class: LoopValueClassV2::I64,
            entry_value: value_key(1),
        }],
        exits: vec![LoopRecipeExitV2 {
            key: LoopExitKeyV1::new(0),
            owner_loop: LoopNodeKeyV1::new(0),
            kind: LoopExitKindV2::Return {
                value: Some(value_key(14)),
            },
        }],
    }
}

fn block(raw: u32, items: &[u32]) -> LoopRecipeBlockV2 {
    LoopRecipeBlockV2 {
        key: LoopBlockKeyV1::new(raw),
        owner_loop: LoopNodeKeyV1::new(0),
        items: items.iter().copied().map(item_key).collect(),
    }
}

fn operation(raw: u32, operation: LoopOperationV2) -> LoopRecipeItemRowV2 {
    LoopRecipeItemRowV2 {
        key: item_key(raw),
        item: LoopRecipeItemV2::Operation { operation },
    }
}

fn value(raw: u32, class: LoopValueClassV2) -> LoopRecipeValueV2 {
    LoopRecipeValueV2 {
        key: value_key(raw),
        class,
    }
}

fn value_class(raw: u32) -> LoopValueClassV2 {
    match raw {
        5 | 13 => LoopValueClassV2::Bool,
        1 | 2 | 4 | 6 | 7 | 8 | 9 | 12 | 14 | 15 | 16 | 17 => LoopValueClassV2::I64,
        _ => LoopValueClassV2::Dynamic,
    }
}

const fn item_key(raw: u32) -> LoopItemKeyV1 {
    LoopItemKeyV1::new(raw)
}

const fn value_key(raw: u32) -> LoopValueKeyV1 {
    LoopValueKeyV1::new(raw)
}
