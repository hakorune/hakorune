use crate::mir::numeric_substrate::generic_g0::{
    GenericG0NumericLiteralRoleV1, VerifiedGenericNumericFactLeaseG0,
};

use super::super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopValueKeyV1,
};
use super::super::schema::{
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopConditionV1, LoopNodeV1, LoopOperationV1,
    LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1, LoopRecipeItemRowV1,
    LoopRecipeItemV1, LoopRecipeV1, LoopRecipeValueV1, LoopValueClassV1,
};

pub(super) fn generic_g0_recipe(
    numeric: &VerifiedGenericNumericFactLeaseG0,
) -> Result<LoopRecipeV1, GenericG0RecipeShapeRejectV1> {
    let bound_outer = literal(numeric, GenericG0NumericLiteralRoleV1::OuterConditionRhs)?;
    let bound_inner = literal(numeric, GenericG0NumericLiteralRoleV1::InnerConditionRhs)?;
    let delta_outer = literal(numeric, GenericG0NumericLiteralRoleV1::OuterUpdateRhs)?;
    let delta_inner = literal(numeric, GenericG0NumericLiteralRoleV1::InnerUpdateRhs)?;

    let l0 = LoopNodeKeyV1::new(0);
    let l1 = LoopNodeKeyV1::new(1);
    let b0 = LoopBindingKeyV1::new(0);
    let b1 = LoopBindingKeyV1::new(1);
    let values = (0..=14)
        .map(|raw| LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(raw),
            class: if matches!(raw, 4 | 8) {
                LoopValueClassV1::Bool
            } else {
                LoopValueClassV1::I64
            },
        })
        .collect();

    Ok(LoopRecipeV1 {
        root_loop: l0,
        loops: vec![
            LoopNodeV1 {
                key: l0,
                parent: None,
                condition: LoopConditionV1::Predicate {
                    block: LoopBlockKeyV1::new(0),
                    value: LoopValueKeyV1::new(4),
                },
                body: LoopBlockKeyV1::new(1),
            },
            LoopNodeV1 {
                key: l1,
                parent: Some(l0),
                condition: LoopConditionV1::Predicate {
                    block: LoopBlockKeyV1::new(2),
                    value: LoopValueKeyV1::new(8),
                },
                body: LoopBlockKeyV1::new(3),
            },
        ],
        blocks: vec![
            block(0, l0, 0..=2),
            LoopRecipeBlockV1 {
                key: LoopBlockKeyV1::new(1),
                owner_loop: l0,
                items: vec![
                    LoopItemKeyV1::new(3),
                    LoopItemKeyV1::new(4),
                    LoopItemKeyV1::new(12),
                    LoopItemKeyV1::new(13),
                    LoopItemKeyV1::new(14),
                    LoopItemKeyV1::new(15),
                ],
            },
            block(2, l1, 5..=7),
            block(3, l1, 8..=11),
        ],
        items: vec![
            read(0, b0, 2),
            constant(1, 3, bound_outer),
            compare(2, 2, 3, 4),
            read(3, b1, 5),
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(4),
                item: LoopRecipeItemV1::Loop { loop_key: l1 },
            },
            read(5, b1, 6),
            constant(6, 7, bound_inner),
            compare(7, 6, 7, 8),
            read(8, b1, 9),
            constant(9, 10, delta_inner),
            add(10, 9, 10, 11),
            write(11, b1, 11),
            read(12, b0, 12),
            constant(13, 13, delta_outer),
            add(14, 12, 13, 14),
            write(15, b0, 14),
        ],
        bindings: vec![
            LoopRecipeBindingV1 {
                key: b0,
                label: "binding_0".into(),
                class: LoopValueClassV1::I64,
            },
            LoopRecipeBindingV1 {
                key: b1,
                label: "binding_1".into(),
                class: LoopValueClassV1::I64,
            },
        ],
        values,
        inputs: vec![LoopValueKeyV1::new(0), LoopValueKeyV1::new(1)],
        carriers: vec![
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(0),
                owner_loop: l0,
                binding: b0,
                class: LoopValueClassV1::I64,
                entry_value: LoopValueKeyV1::new(0),
            },
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(1),
                owner_loop: l0,
                binding: b1,
                class: LoopValueClassV1::I64,
                entry_value: LoopValueKeyV1::new(1),
            },
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(2),
                owner_loop: l1,
                binding: b1,
                class: LoopValueClassV1::I64,
                entry_value: LoopValueKeyV1::new(5),
            },
        ],
        exits: Vec::new(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0RecipeShapeRejectV1 {
    MissingLiteral(GenericG0NumericLiteralRoleV1),
    LiteralOutOfI64(GenericG0NumericLiteralRoleV1),
}

fn literal(
    numeric: &VerifiedGenericNumericFactLeaseG0,
    role: GenericG0NumericLiteralRoleV1,
) -> Result<i64, GenericG0RecipeShapeRejectV1> {
    let Some(row) = numeric.literals().iter().find(|row| row.role == role) else {
        return Err(GenericG0RecipeShapeRejectV1::MissingLiteral(role));
    };
    i64::try_from(row.value).map_err(|_| GenericG0RecipeShapeRejectV1::LiteralOutOfI64(role))
}

fn block(
    key: u32,
    owner_loop: LoopNodeKeyV1,
    items: std::ops::RangeInclusive<u32>,
) -> LoopRecipeBlockV1 {
    LoopRecipeBlockV1 {
        key: LoopBlockKeyV1::new(key),
        owner_loop,
        items: items.map(LoopItemKeyV1::new).collect(),
    }
}

fn read(key: u32, binding: LoopBindingKeyV1, result: u32) -> LoopRecipeItemRowV1 {
    operation(
        key,
        LoopOperationV1::ReadBinding {
            binding,
            result: LoopValueKeyV1::new(result),
        },
    )
}

fn constant(key: u32, result: u32, value: i64) -> LoopRecipeItemRowV1 {
    operation(
        key,
        LoopOperationV1::ConstI64 {
            result: LoopValueKeyV1::new(result),
            value,
        },
    )
}

fn compare(key: u32, left: u32, right: u32, result: u32) -> LoopRecipeItemRowV1 {
    operation(
        key,
        LoopOperationV1::CompareI64 {
            op: LoopCompareI64OpV1::Less,
            left: LoopValueKeyV1::new(left),
            right: LoopValueKeyV1::new(right),
            result: LoopValueKeyV1::new(result),
        },
    )
}

fn add(key: u32, left: u32, right: u32, result: u32) -> LoopRecipeItemRowV1 {
    operation(
        key,
        LoopOperationV1::BinaryI64 {
            op: LoopBinaryI64OpV1::Add,
            left: LoopValueKeyV1::new(left),
            right: LoopValueKeyV1::new(right),
            result: LoopValueKeyV1::new(result),
        },
    )
}

fn write(key: u32, binding: LoopBindingKeyV1, value: u32) -> LoopRecipeItemRowV1 {
    operation(
        key,
        LoopOperationV1::WriteBinding {
            binding,
            value: LoopValueKeyV1::new(value),
        },
    )
}

fn operation(key: u32, operation: LoopOperationV1) -> LoopRecipeItemRowV1 {
    LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(key),
        item: LoopRecipeItemV1::Operation { operation },
    }
}
