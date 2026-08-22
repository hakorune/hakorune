//! Private row-level view for the S6C product-first consumer seam.
//!
//! This module projects only typed Recipe rows selected through the fixed S6C
//! role seal. It never lends `LoopRecipeV2`, issues keys, or creates JoinIR,
//! MIR, physical IDs, selectors, fallback, or production effects.

use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
use super::s6c_scan_with_init::{
    DefinedRoleV2, ExitRoleV2, S6CScanWithInitRecipeRolesRefV2, WriteRoleV2,
};
use super::schema_v2::{
    LoopBinaryI64OpV2, LoopCompareI64OpV2, LoopConditionV2, LoopExitKindV2, LoopOperationV2,
    LoopRecipeBlockV2, LoopRecipeCarrierV2, LoopRecipeItemV2, LoopRecipeValueV2, LoopValueClassV2,
};
use super::typed_schema_v2::VerifiedLoopRecipeV2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CRecipeOperationRowRefV2<'a> {
    ReadBinding {
        binding: LoopBindingKeyV1,
        result: LoopValueKeyV1,
    },
    ConstI64 {
        result: LoopValueKeyV1,
        value: i64,
    },
    BinaryI64 {
        op: LoopBinaryI64OpV2,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
    CompareI64 {
        op: LoopCompareI64OpV2,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
    WriteBinding {
        binding: LoopBindingKeyV1,
        value: LoopValueKeyV1,
    },
    CallSlot {
        receiver: Option<LoopValueKeyV1>,
        args: &'a [LoopValueKeyV1],
        result: Option<LoopValueKeyV1>,
    },
    TextEq {
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CRecipeIfRowRefV2 {
    pub(crate) condition: LoopValueKeyV1,
    pub(crate) then_block: LoopBlockKeyV1,
    pub(crate) else_block: Option<LoopBlockKeyV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CRecipeExitRowRefV2 {
    pub(crate) key: LoopExitKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) kind: LoopExitKindV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CRecipeValueRowRefV2 {
    pub(crate) key: LoopValueKeyV1,
    pub(crate) class: LoopValueClassV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CRecipeLoopRowRefV2 {
    pub(crate) key: LoopNodeKeyV1,
    pub(crate) parent: Option<LoopNodeKeyV1>,
    pub(crate) condition: LoopConditionV2,
    pub(crate) body: LoopBlockKeyV1,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CRecipeBlockRowRefV2<'a> {
    pub(crate) key: LoopBlockKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) items: &'a [LoopItemKeyV1],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S6CRecipeCarrierRowRefV2 {
    pub(crate) key: LoopCarrierKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) binding: LoopBindingKeyV1,
    pub(crate) class: LoopValueClassV2,
    pub(crate) entry_value: LoopValueKeyV1,
}

/// Borrowed Recipe rows selected only through fixed S6C role accessors.
#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CScanWithInitRecipeRowsRefV2<'a> {
    recipe: &'a VerifiedLoopRecipeV2,
    roles: S6CScanWithInitRecipeRolesRefV2<'a>,
}

impl<'a> S6CScanWithInitRecipeRowsRefV2<'a> {
    pub(crate) fn new(
        recipe: &'a VerifiedLoopRecipeV2,
        roles: S6CScanWithInitRecipeRolesRefV2<'a>,
    ) -> Self {
        Self { recipe, roles }
    }

    pub(crate) fn loop_count(self) -> usize {
        self.recipe.as_recipe().loops.len()
    }

    pub(crate) fn block_count(self) -> usize {
        self.recipe.as_recipe().blocks.len()
    }

    pub(crate) fn binding_count(self) -> usize {
        self.recipe.as_recipe().bindings.len()
    }

    pub(crate) fn input_count(self) -> usize {
        self.recipe.as_recipe().inputs.len()
    }

    pub(crate) fn value_count(self) -> usize {
        self.recipe.as_recipe().values.len()
    }

    pub(crate) fn item_count(self) -> usize {
        self.recipe.as_recipe().items.len()
    }

    pub(crate) fn carrier_count(self) -> usize {
        self.recipe.as_recipe().carriers.len()
    }

    pub(crate) fn exit_count(self) -> usize {
        self.recipe.as_recipe().exits.len()
    }

    pub(crate) fn root_loop(self) -> Option<S6CRecipeLoopRowRefV2> {
        self.loop_row(self.roles.root_loop())
    }

    pub(crate) fn condition_block(self) -> Option<S6CRecipeBlockRowRefV2<'a>> {
        self.block(self.roles.condition_block())
    }

    pub(crate) fn body_block(self) -> Option<S6CRecipeBlockRowRefV2<'a>> {
        self.block(self.roles.body_block())
    }

    pub(crate) fn text_equal_then_block(self) -> Option<S6CRecipeBlockRowRefV2<'a>> {
        self.block(self.roles.text_eq_then_block())
    }

    pub(crate) fn index_carrier(self) -> Option<S6CRecipeCarrierRowRefV2> {
        let key = self.roles.index_carrier();
        self.recipe
            .as_recipe()
            .carriers
            .iter()
            .find(|row| row.key == key)
            .map(carrier_row)
    }

    pub(crate) fn subject_input(self) -> Option<S6CRecipeValueRowRefV2> {
        self.value(self.roles.subject_input())
    }

    pub(crate) fn needle_input(self) -> Option<S6CRecipeValueRowRefV2> {
        self.value(self.roles.needle_input())
    }

    pub(crate) fn index_input(self) -> Option<S6CRecipeValueRowRefV2> {
        self.value(self.roles.index_input())
    }

    pub(crate) fn value_class(self, key: LoopValueKeyV1) -> Option<LoopValueClassV2> {
        self.value(key).map(|row| row.class)
    }

    fn value(self, key: LoopValueKeyV1) -> Option<S6CRecipeValueRowRefV2> {
        self.recipe
            .as_recipe()
            .values
            .iter()
            .find(|row| row.key == key)
            .map(value_row)
    }

    pub(crate) fn operation(self, role: DefinedRoleV2) -> Option<S6CRecipeOperationRowRefV2<'a>> {
        let item = self.item(role.item())?;
        let LoopRecipeItemV2::Operation { operation } = &item.item else {
            return None;
        };
        let row = operation_row(operation);
        (operation_result(operation) == Some(role.result())).then_some(row)
    }

    pub(crate) fn operation_result_class(self, role: DefinedRoleV2) -> Option<LoopValueClassV2> {
        self.value(role.result()).map(|value| value.class)
    }

    pub(crate) fn write(self, role: WriteRoleV2) -> Option<S6CRecipeOperationRowRefV2<'a>> {
        let item = self.item(role.item())?;
        let LoopRecipeItemV2::Operation { operation } = &item.item else {
            return None;
        };
        let LoopOperationV2::WriteBinding { binding, value } = operation else {
            return None;
        };
        (*binding == role.binding() && *value == role.value()).then_some(
            S6CRecipeOperationRowRefV2::WriteBinding {
                binding: *binding,
                value: *value,
            },
        )
    }

    pub(crate) fn text_equal_if(self) -> Option<S6CRecipeIfRowRefV2> {
        self.if_row(self.roles.text_equal_if())
    }

    fn if_row(self, item_key: LoopItemKeyV1) -> Option<S6CRecipeIfRowRefV2> {
        let item = self.item(item_key)?;
        let LoopRecipeItemV2::If {
            condition,
            then_block,
            else_block,
        } = &item.item
        else {
            return None;
        };
        Some(S6CRecipeIfRowRefV2 {
            condition: *condition,
            then_block: *then_block,
            else_block: *else_block,
        })
    }

    pub(crate) fn loop_exit(self, role: ExitRoleV2) -> Option<S6CRecipeExitRowRefV2> {
        let item = self.item(role.item())?;
        let LoopRecipeItemV2::Exit { exit } = &item.item else {
            return None;
        };
        if *exit != role.exit() {
            return None;
        }
        self.recipe
            .as_recipe()
            .exits
            .iter()
            .find(|row| row.key == *exit)
            .map(exit_row)
    }

    fn item(self, key: LoopItemKeyV1) -> Option<&'a super::schema_v2::LoopRecipeItemRowV2> {
        self.recipe
            .as_recipe()
            .items
            .iter()
            .find(|row| row.key == key)
    }

    fn loop_row(self, key: LoopNodeKeyV1) -> Option<S6CRecipeLoopRowRefV2> {
        self.recipe
            .as_recipe()
            .loops
            .iter()
            .find(|row| row.key == key)
            .map(loop_row)
    }

    fn block(self, key: LoopBlockKeyV1) -> Option<S6CRecipeBlockRowRefV2<'a>> {
        self.recipe
            .as_recipe()
            .blocks
            .iter()
            .find(|row| row.key == key)
            .map(block_row)
    }
}

fn operation_result(operation: &LoopOperationV2) -> Option<LoopValueKeyV1> {
    match operation {
        LoopOperationV2::ReadBinding { result, .. }
        | LoopOperationV2::ConstI64 { result, .. }
        | LoopOperationV2::BinaryI64 { result, .. }
        | LoopOperationV2::CompareI64 { result, .. }
        | LoopOperationV2::DynamicAdd { result, .. }
        | LoopOperationV2::DynamicLess { result, .. }
        | LoopOperationV2::TextEq { result, .. } => Some(*result),
        LoopOperationV2::WriteBinding { .. } => None,
        LoopOperationV2::CallSlot { result, .. } => *result,
    }
}

fn operation_row<'a>(operation: &'a LoopOperationV2) -> S6CRecipeOperationRowRefV2<'a> {
    match operation {
        LoopOperationV2::ReadBinding { binding, result } => {
            S6CRecipeOperationRowRefV2::ReadBinding {
                binding: *binding,
                result: *result,
            }
        }
        LoopOperationV2::ConstI64 { result, value } => S6CRecipeOperationRowRefV2::ConstI64 {
            result: *result,
            value: *value,
        },
        LoopOperationV2::BinaryI64 {
            op,
            left,
            right,
            result,
        } => S6CRecipeOperationRowRefV2::BinaryI64 {
            op: *op,
            left: *left,
            right: *right,
            result: *result,
        },
        LoopOperationV2::CompareI64 {
            op,
            left,
            right,
            result,
        } => S6CRecipeOperationRowRefV2::CompareI64 {
            op: *op,
            left: *left,
            right: *right,
            result: *result,
        },
        LoopOperationV2::DynamicAdd {
            left,
            right,
            result,
        } => S6CRecipeOperationRowRefV2::BinaryI64 {
            op: LoopBinaryI64OpV2::Add,
            left: *left,
            right: *right,
            result: *result,
        },
        LoopOperationV2::DynamicLess {
            left,
            right,
            result,
        } => S6CRecipeOperationRowRefV2::CompareI64 {
            op: LoopCompareI64OpV2::Less,
            left: *left,
            right: *right,
            result: *result,
        },
        LoopOperationV2::WriteBinding { binding, value } => {
            S6CRecipeOperationRowRefV2::WriteBinding {
                binding: *binding,
                value: *value,
            }
        }
        LoopOperationV2::CallSlot {
            receiver,
            args,
            result,
        } => S6CRecipeOperationRowRefV2::CallSlot {
            receiver: *receiver,
            args,
            result: *result,
        },
        LoopOperationV2::TextEq {
            left,
            right,
            result,
        } => S6CRecipeOperationRowRefV2::TextEq {
            left: *left,
            right: *right,
            result: *result,
        },
    }
}

fn value_row(row: &LoopRecipeValueV2) -> S6CRecipeValueRowRefV2 {
    S6CRecipeValueRowRefV2 {
        key: row.key,
        class: row.class,
    }
}

fn loop_row(row: &super::schema_v2::LoopNodeV2) -> S6CRecipeLoopRowRefV2 {
    S6CRecipeLoopRowRefV2 {
        key: row.key,
        parent: row.parent,
        condition: row.condition,
        body: row.body,
    }
}

fn block_row<'a>(row: &'a LoopRecipeBlockV2) -> S6CRecipeBlockRowRefV2<'a> {
    S6CRecipeBlockRowRefV2 {
        key: row.key,
        owner_loop: row.owner_loop,
        items: &row.items,
    }
}

fn carrier_row(row: &LoopRecipeCarrierV2) -> S6CRecipeCarrierRowRefV2 {
    S6CRecipeCarrierRowRefV2 {
        key: row.key,
        owner_loop: row.owner_loop,
        binding: row.binding,
        class: row.class,
        entry_value: row.entry_value,
    }
}

fn exit_row(row: &super::schema_v2::LoopRecipeExitV2) -> S6CRecipeExitRowRefV2 {
    S6CRecipeExitRowRefV2 {
        key: row.key,
        owner_loop: row.owner_loop,
        kind: row.kind,
    }
}
