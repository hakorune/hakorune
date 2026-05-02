use std::collections::BTreeSet;

use crate::mir::{MirFunction, MirType, ValueId};

pub(super) fn generic_pure_string_return_allows_param_passthrough(
    function: &MirFunction,
    value: ValueId,
    return_param_values: &BTreeSet<ValueId>,
) -> bool {
    generic_pure_string_return_type_accepts_param_passthrough(&function.signature.return_type)
        && return_param_values.contains(&value)
}

fn generic_pure_string_return_type_accepts_param_passthrough(ty: &MirType) -> bool {
    matches!(ty, MirType::String) || matches!(ty, MirType::Box(name) if name == "StringBox")
}

pub(super) fn generic_pure_string_abi_type_is_handle_compatible(ty: &MirType) -> bool {
    match ty {
        MirType::Integer | MirType::String | MirType::Unknown => true,
        MirType::Box(name) => name == "StringBox",
        _ => false,
    }
}
