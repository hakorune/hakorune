use crate::mir::generic_method_route_plan::GenericMethodRoute;
use crate::mir::{BasicBlockId, MirFunction, MirType};

use super::generic_string_facts::GenericPureValueClass;
use super::model::GlobalCallTargetFacts;

pub(super) fn generic_pure_string_route_value_class(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
) -> Option<GenericPureValueClass> {
    let route = function
        .metadata
        .generic_method_routes
        .iter()
        .find(|route| {
            route.block() == block
                && route.instruction_index() == instruction_index
                && matches!(
                    route.route_id(),
                    "generic_method.get"
                        | "generic_method.keys"
                        | "generic_method.len"
                        | "generic_method.substring"
                        | "generic_method.indexOf"
                        | "generic_method.lastIndexOf"
                        | "generic_method.contains"
                )
        })?;
    match route.route_kind_tag() {
        "string_len" | "array_slot_len" => Some(GenericPureValueClass::I64),
        "string_substring" => Some(GenericPureValueClass::String),
        "string_indexof" | "string_lastindexof" => Some(GenericPureValueClass::I64),
        "string_contains" => Some(GenericPureValueClass::Bool),
        _ => generic_pure_string_get_route_value_class(route),
    }
}

fn generic_pure_string_get_route_value_class(
    route: &GenericMethodRoute,
) -> Option<GenericPureValueClass> {
    match route.proof_tag() {
        "mir_json_const_value_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "type" => Some(GenericPureValueClass::StringOrVoid),
                "value" => Some(GenericPureValueClass::I64),
                _ => None,
            }
        }
        "mir_json_phi_incoming_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::Array)
        }
        "mir_json_phi_incoming_pair_scalar" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::I64)
        }
        "mir_json_callee_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "receiver" => Some(GenericPureValueClass::ScalarOrVoid),
                "type" | "name" | "box_name" | "method" | "box_type" => {
                    Some(GenericPureValueClass::StringOrVoid)
                }
                _ => None,
            }
        }
        "mir_json_vid_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::I64)
        }
        "mir_json_effects_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::StringOrVoid)
        }
        "mir_json_block_inst_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::ScalarOrVoid)
        }
        "mir_json_function_block_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::ScalarOrVoid)
        }
        "mir_json_params_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::I64)
        }
        "mir_json_flags_rec_access" => match route.route_kind_tag() {
            "array_slot_load_any" => Some(GenericPureValueClass::String),
            "runtime_data_load_any" => Some(GenericPureValueClass::StringOrVoid),
            _ => None,
        },
        "mir_json_flags_keys" if route.route_kind_tag() == "map_keys_array" => {
            Some(GenericPureValueClass::Array)
        }
        "keys_surface_policy" if route.route_kind_tag() == "map_keys_array" => {
            Some(GenericPureValueClass::Array)
        }
        "mir_json_block_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "instructions" => Some(GenericPureValueClass::Array),
                "id" => Some(GenericPureValueClass::ScalarOrVoid),
                _ => None,
            }
        }
        "mir_json_function_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "name" => Some(GenericPureValueClass::StringOrVoid),
                "params" | "blocks" => Some(GenericPureValueClass::Array),
                "flags" => Some(GenericPureValueClass::Map),
                _ => None,
            }
        }
        "mir_json_module_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "functions" => Some(GenericPureValueClass::Array),
                "functions_0" => Some(GenericPureValueClass::Map),
                _ => None,
            }
        }
        "mir_json_module_function_array_item"
            if route.route_kind_tag() == "array_slot_load_any" =>
        {
            Some(GenericPureValueClass::Map)
        }
        "mir_json_inst_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "op" | "operation" | "op_kind" | "cmp" | "value" => {
                    Some(GenericPureValueClass::StringOrVoid)
                }
                "args" | "effects" => Some(GenericPureValueClass::Array),
                "dst" | "lhs" | "rhs" | "cond" | "then" | "else" | "target" | "incoming"
                | "values" | "mir_call" | "callee" | "func" | "name" => {
                    Some(GenericPureValueClass::ScalarOrVoid)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

pub(super) fn generic_pure_string_generic_i64_target_value_class(
    target: &GlobalCallTargetFacts,
) -> GenericPureValueClass {
    match target.return_type() {
        Some(MirType::Bool) => GenericPureValueClass::Bool,
        Some(MirType::Unknown | MirType::Void) => GenericPureValueClass::ScalarOrVoid,
        _ => GenericPureValueClass::I64,
    }
}

pub(super) fn generic_pure_select_value_class(
    then_class: GenericPureValueClass,
    else_class: GenericPureValueClass,
) -> Option<GenericPureValueClass> {
    if then_class == else_class {
        return Some(then_class);
    }
    match (then_class, else_class) {
        (GenericPureValueClass::String, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::String)
        | (GenericPureValueClass::StringOrVoid, GenericPureValueClass::String)
        | (GenericPureValueClass::String, GenericPureValueClass::StringOrVoid)
        | (GenericPureValueClass::StringOrVoid, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::StringOrVoid) => {
            Some(GenericPureValueClass::StringOrVoid)
        }
        (GenericPureValueClass::Array, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::Array)
        | (GenericPureValueClass::ArrayOrVoid, GenericPureValueClass::Array)
        | (GenericPureValueClass::Array, GenericPureValueClass::ArrayOrVoid)
        | (GenericPureValueClass::ArrayOrVoid, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::ArrayOrVoid) => {
            Some(GenericPureValueClass::ArrayOrVoid)
        }
        (GenericPureValueClass::Map, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::Map)
        | (GenericPureValueClass::MapOrVoid, GenericPureValueClass::Map)
        | (GenericPureValueClass::Map, GenericPureValueClass::MapOrVoid)
        | (GenericPureValueClass::MapOrVoid, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::MapOrVoid) => {
            Some(GenericPureValueClass::MapOrVoid)
        }
        (GenericPureValueClass::I64, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::I64)
        | (GenericPureValueClass::Bool, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::Bool)
        | (GenericPureValueClass::ScalarOrVoid, GenericPureValueClass::I64)
        | (GenericPureValueClass::I64, GenericPureValueClass::ScalarOrVoid)
        | (GenericPureValueClass::ScalarOrVoid, GenericPureValueClass::Bool)
        | (GenericPureValueClass::Bool, GenericPureValueClass::ScalarOrVoid)
        | (GenericPureValueClass::ScalarOrVoid, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::ScalarOrVoid) => {
            Some(GenericPureValueClass::ScalarOrVoid)
        }
        _ => None,
    }
}
