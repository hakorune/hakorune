//! Typed V2 operation-domain verification.
//!
//! Kept separate from structural loop checks so adding a logical operation
//! does not push the wire verifier past its responsibility boundary.

use std::collections::BTreeMap;

use super::super::ids::{LoopBindingKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use super::super::schema_v2::{LoopOperationV2, LoopValueClassV2};
use super::LoopRecipeV2RejectReason;

pub(super) fn check_operation(
    item_key: &LoopItemKeyV1,
    operation: &LoopOperationV2,
    bindings: &BTreeMap<LoopBindingKeyV1, LoopValueClassV2>,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
    definitions: &mut BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<(), LoopRecipeV2RejectReason> {
    match operation {
        LoopOperationV2::ReadBinding { binding, result } => {
            let Some(class) = bindings.get(binding) else {
                return Err(LoopRecipeV2RejectReason::UnknownBinding { key: *binding });
            };
            define_value(*item_key, *result, *class, values, definitions)
        }
        LoopOperationV2::ConstI64 { result, .. } => define_value(
            *item_key,
            *result,
            LoopValueClassV2::I64,
            values,
            definitions,
        ),
        LoopOperationV2::ConstText { result, .. } => define_value(
            *item_key,
            *result,
            LoopValueClassV2::Text,
            values,
            definitions,
        ),
        LoopOperationV2::BinaryI64 {
            left,
            right,
            result,
            ..
        } => {
            expect_defined_class(*item_key, *left, LoopValueClassV2::I64, values, definitions)?;
            expect_defined_class(
                *item_key,
                *right,
                LoopValueClassV2::I64,
                values,
                definitions,
            )?;
            define_value(
                *item_key,
                *result,
                LoopValueClassV2::I64,
                values,
                definitions,
            )
        }
        LoopOperationV2::CompareI64 {
            left,
            right,
            result,
            ..
        } => {
            expect_defined_class(*item_key, *left, LoopValueClassV2::I64, values, definitions)?;
            expect_defined_class(
                *item_key,
                *right,
                LoopValueClassV2::I64,
                values,
                definitions,
            )?;
            define_value(
                *item_key,
                *result,
                LoopValueClassV2::Bool,
                values,
                definitions,
            )
        }
        LoopOperationV2::DynamicAdd {
            left,
            right,
            result,
        } => {
            expect_defined_class(
                *item_key,
                *left,
                LoopValueClassV2::Dynamic,
                values,
                definitions,
            )?;
            expect_defined_class(
                *item_key,
                *right,
                LoopValueClassV2::I64,
                values,
                definitions,
            )?;
            define_value(
                *item_key,
                *result,
                LoopValueClassV2::Dynamic,
                values,
                definitions,
            )
        }
        LoopOperationV2::DynamicLess {
            left,
            right,
            result,
        } => {
            expect_defined_class(
                *item_key,
                *left,
                LoopValueClassV2::Dynamic,
                values,
                definitions,
            )?;
            let right_class = super::expect_defined_value(*item_key, *right, values, definitions)?;
            if !matches!(
                right_class,
                LoopValueClassV2::Dynamic | LoopValueClassV2::I64
            ) {
                return Err(LoopRecipeV2RejectReason::InvalidOperationDomain { item: *item_key });
            }
            define_value(
                *item_key,
                *result,
                LoopValueClassV2::Bool,
                values,
                definitions,
            )
        }
        LoopOperationV2::WriteBinding { binding, value } => {
            let Some(class) = bindings.get(binding) else {
                return Err(LoopRecipeV2RejectReason::UnknownBinding { key: *binding });
            };
            if super::expect_defined_value(*item_key, *value, values, definitions)? != *class {
                return Err(LoopRecipeV2RejectReason::ValueClassMismatch { key: *value });
            }
            Ok(())
        }
        LoopOperationV2::CallSlot {
            receiver,
            args,
            result,
        } => {
            if let Some(key) = receiver {
                super::expect_defined_value(*item_key, *key, values, definitions)?;
            }
            for key in args {
                super::expect_defined_value(*item_key, *key, values, definitions)?;
            }
            if let Some(result) = result {
                let class = *values
                    .get(result)
                    .ok_or(LoopRecipeV2RejectReason::UnknownValue { key: *result })?;
                define_value(*item_key, *result, class, values, definitions)
            } else {
                Ok(())
            }
        }
        LoopOperationV2::TextEq {
            left,
            right,
            result,
        } => {
            if super::expect_defined_value(*item_key, *left, values, definitions)?
                != LoopValueClassV2::Text
                || super::expect_defined_value(*item_key, *right, values, definitions)?
                    != LoopValueClassV2::Text
            {
                return Err(LoopRecipeV2RejectReason::TextEqOperandClassMismatch {
                    item: *item_key,
                });
            }
            if values.get(result) != Some(&LoopValueClassV2::Bool) {
                return Err(LoopRecipeV2RejectReason::TextEqResultClassMismatch {
                    item: *item_key,
                });
            }
            define_value(
                *item_key,
                *result,
                LoopValueClassV2::Bool,
                values,
                definitions,
            )
        }
    }
}

fn expect_defined_class(
    item: LoopItemKeyV1,
    key: LoopValueKeyV1,
    expected: LoopValueClassV2,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
    definitions: &BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<(), LoopRecipeV2RejectReason> {
    if super::expect_defined_value(item, key, values, definitions)? == expected {
        Ok(())
    } else {
        Err(LoopRecipeV2RejectReason::InvalidOperationDomain { item })
    }
}

fn define_value(
    item: LoopItemKeyV1,
    key: LoopValueKeyV1,
    class: LoopValueClassV2,
    values: &BTreeMap<LoopValueKeyV1, LoopValueClassV2>,
    definitions: &mut BTreeMap<LoopValueKeyV1, Option<LoopItemKeyV1>>,
) -> Result<(), LoopRecipeV2RejectReason> {
    if values.get(&key) != Some(&class) {
        return Err(LoopRecipeV2RejectReason::ValueClassMismatch { key });
    }
    if definitions.insert(key, Some(item)).is_some() {
        return Err(LoopRecipeV2RejectReason::DuplicateValueDefinition { key });
    }
    Ok(())
}
