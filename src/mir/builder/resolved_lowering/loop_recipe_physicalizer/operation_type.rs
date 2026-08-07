//! Type publication for prepared Loop operation leaves.
//!
//! Unsealed loop-header PHIs may temporarily carry `Unknown`.  This module
//! admits only the exact type supplied by the verified Recipe value class;
//! concrete conflicts and missing types remain fail-fast errors.

use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::LoopValueClassV1;
use crate::mir::{MirType, ValueId};

pub(super) fn expected_mir_type(class: LoopValueClassV1) -> MirType {
    match class {
        LoopValueClassV1::I64 => MirType::Integer,
        LoopValueClassV1::Bool => MirType::Bool,
        LoopValueClassV1::Unit => MirType::Void,
    }
}

pub(super) fn ensure_provisional_value_class(
    builder: &mut MirBuilder,
    value: ValueId,
    class: LoopValueClassV1,
) -> Result<MirType, String> {
    let expected = expected_mir_type(class);
    match builder.function_state.type_ctx.get_type(value) {
        Some(existing) if existing == &expected => Ok(expected),
        Some(MirType::Unknown) => {
            builder
                .function_state
                .type_ctx
                .value_types
                .insert(value, expected);
            Ok(expected_mir_type(class))
        }
        Some(existing) => Err(format!(
            "[freeze:contract][loop_operation/provisional_value_type] value={value:?} expected={expected:?} found={existing:?}"
        )),
        None => Err(format!(
            "[freeze:contract][loop_operation/provisional_value_type_missing] value={value:?}"
        )),
    }
}
