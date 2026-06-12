use super::value_class::{generic_i64_value_class, GenericI64ValueClass};
use crate::mir::same_module_body_shape::supported_backend_global;
use crate::mir::{BasicBlockId, MirFunction, ValueId};
use std::collections::BTreeMap;

pub(super) fn generic_i64_route_value_class(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
) -> Option<GenericI64ValueClass> {
    function
        .metadata
        .generic_method_routes
        .iter()
        .find(|route| {
            route.block() == block
                && route.instruction_index() == instruction_index
                && route.proof_tag() == "mir_json_numeric_value_field"
                && route.route_id() == "generic_method.get"
                && route.route_kind_tag() == "runtime_data_load_any"
                && route.key_const_text() == Some("value")
        })
        .map(|_| GenericI64ValueClass::StringOrVoid)
}

pub(super) fn generic_i64_global_call_result_class(
    values: &BTreeMap<ValueId, GenericI64ValueClass>,
    dst: &Option<ValueId>,
) -> GenericI64ValueClass {
    if dst
        .map(|dst| generic_i64_value_class(values, dst) == GenericI64ValueClass::Bool)
        .unwrap_or(false)
    {
        GenericI64ValueClass::Bool
    } else {
        GenericI64ValueClass::I64
    }
}

pub(super) fn generic_i64_accepts_backend_global_call(
    function: &MirFunction,
    name: &str,
    dst: &Option<ValueId>,
    args: &[ValueId],
) -> bool {
    if !supported_backend_global(name) || args.len() != 1 {
        return false;
    }
    dst.map(|value| !generic_i64_value_is_used(function, value))
        .unwrap_or(true)
}

pub(super) fn generic_i64_value_is_used(function: &MirFunction, value: ValueId) -> bool {
    function.blocks.values().any(|block| {
        block
            .instructions
            .iter()
            .chain(block.terminator.iter())
            .any(|instruction| instruction.used_values().contains(&value))
    })
}
