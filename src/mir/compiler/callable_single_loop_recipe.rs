//! Canonical Recipe shape for the bounded callable single-loop profile.
//!
//! This is a profile adapter, not a second Recipe authority. The shared
//! `LoopRecipeVerifierV1` remains the only semantic verifier; this module only
//! constructs the fixed seven-operation shape selected by the callable source
//! relation issuer.

use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopCompareI64OpV1, LoopConditionV1,
    LoopItemKeyV1, LoopNodeKeyV1, LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1,
    LoopRecipeItemRowV1, LoopRecipeItemV1, LoopRecipeV1, LoopRecipeValueV1, LoopValueClassV1,
    LoopValueKeyV1,
};

pub(crate) fn canonical_callable_single_loop_recipe_v1() -> LoopRecipeV1 {
    let loop_key = LoopNodeKeyV1::new(0);
    let condition_block = LoopBlockKeyV1::new(0);
    let body_block = LoopBlockKeyV1::new(1);
    let binding = crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0);
    let values = (0..=6)
        .map(|raw| LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(raw),
            class: if raw == 3 {
                LoopValueClassV1::Bool
            } else {
                LoopValueClassV1::I64
            },
        })
        .collect();
    let item = |key, item| LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(key),
        item,
    };
    LoopRecipeV1 {
        root_loop: loop_key,
        loops: vec![crate::mir::loop_recipe_contract::LoopNodeV1 {
            key: loop_key,
            parent: None,
            condition: LoopConditionV1::Predicate {
                block: condition_block,
                value: LoopValueKeyV1::new(3),
            },
            body: body_block,
        }],
        blocks: vec![
            LoopRecipeBlockV1 {
                key: condition_block,
                owner_loop: loop_key,
                items: vec![
                    LoopItemKeyV1::new(0),
                    LoopItemKeyV1::new(1),
                    LoopItemKeyV1::new(2),
                ],
            },
            LoopRecipeBlockV1 {
                key: body_block,
                owner_loop: loop_key,
                items: vec![
                    LoopItemKeyV1::new(3),
                    LoopItemKeyV1::new(4),
                    LoopItemKeyV1::new(5),
                    LoopItemKeyV1::new(6),
                ],
            },
        ],
        items: vec![
            item(
                0,
                LoopRecipeItemV1::Operation {
                    operation: crate::mir::loop_recipe_contract::LoopOperationV1::ConstI64 {
                        result: LoopValueKeyV1::new(2),
                        value: 1,
                    },
                },
            ),
            item(
                1,
                LoopRecipeItemV1::Operation {
                    operation: crate::mir::loop_recipe_contract::LoopOperationV1::ReadBinding {
                        binding,
                        result: LoopValueKeyV1::new(1),
                    },
                },
            ),
            item(
                2,
                LoopRecipeItemV1::Operation {
                    operation: crate::mir::loop_recipe_contract::LoopOperationV1::CompareI64 {
                        op: LoopCompareI64OpV1::Less,
                        left: LoopValueKeyV1::new(1),
                        right: LoopValueKeyV1::new(2),
                        result: LoopValueKeyV1::new(3),
                    },
                },
            ),
            item(
                3,
                LoopRecipeItemV1::Operation {
                    operation: crate::mir::loop_recipe_contract::LoopOperationV1::ReadBinding {
                        binding,
                        result: LoopValueKeyV1::new(4),
                    },
                },
            ),
            item(
                4,
                LoopRecipeItemV1::Operation {
                    operation: crate::mir::loop_recipe_contract::LoopOperationV1::ConstI64 {
                        result: LoopValueKeyV1::new(5),
                        value: 1,
                    },
                },
            ),
            item(
                5,
                LoopRecipeItemV1::Operation {
                    operation: crate::mir::loop_recipe_contract::LoopOperationV1::BinaryI64 {
                        op: LoopBinaryI64OpV1::Add,
                        left: LoopValueKeyV1::new(4),
                        right: LoopValueKeyV1::new(5),
                        result: LoopValueKeyV1::new(6),
                    },
                },
            ),
            item(
                6,
                LoopRecipeItemV1::Operation {
                    operation: crate::mir::loop_recipe_contract::LoopOperationV1::WriteBinding {
                        binding,
                        value: LoopValueKeyV1::new(6),
                    },
                },
            ),
        ],
        bindings: vec![LoopRecipeBindingV1 {
            key: binding,
            label: "induction".into(),
            class: LoopValueClassV1::I64,
        }],
        values,
        inputs: vec![LoopValueKeyV1::new(0)],
        carriers: vec![LoopRecipeCarrierV1 {
            key: LoopCarrierKeyV1::new(0),
            owner_loop: loop_key,
            binding,
            class: LoopValueClassV1::I64,
            entry_value: LoopValueKeyV1::new(0),
        }],
        exits: Vec::new(),
    }
}
