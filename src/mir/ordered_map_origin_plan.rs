/*!
 * Focused OrderedMapBox result-origin publication.
 *
 * This is not general dependent map typing. It only publishes object result
 * origins for constant-key OrderedMapBox reads when a prior focused producer
 * proves the stored value type in the same function.
 */

use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{
    BasicBlockId, Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};
use hakorune_mir_defs::{CalleeBoxKind, TypeCertainty};
use std::collections::BTreeMap;

type MapKey = (ValueId, String);

pub fn refresh_module_ordered_map_get_result_origins(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_ordered_map_get_result_origins(function);
    }
}

fn refresh_function_ordered_map_get_result_origins(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut schema: BTreeMap<MapKey, MirType> = BTreeMap::new();
    let mut published: Vec<(ValueId, MirType)> = Vec::new();

    for block in function.blocks.values() {
        for instruction in &block.instructions {
            let MirInstruction::Call {
                dst,
                callee,
                args,
                ..
            } = instruction
            else {
                continue;
            };

            if let Some(Callee::Global(symbol)) = callee {
                seed_carrier_info_output_schema(function, &def_map, &mut schema, symbol, args);
                continue;
            }

            let Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }) = callee
            else {
                continue;
            };
            if box_name != "OrderedMapBox" {
                continue;
            }

            match method.as_str() {
                "set" => seed_ordered_map_set_schema(
                    function,
                    &def_map,
                    &mut schema,
                    *receiver,
                    args,
                ),
                "get" => {
                    let Some(dst) = dst else {
                        continue;
                    };
                    let Some(key) = logical_method_arg(args, *receiver, 0)
                        .and_then(|arg| const_string(function, &def_map, arg))
                    else {
                        continue;
                    };
                    let recv_origin = resolve_value_origin(function, &def_map, *receiver);
                    if let Some(ty) = schema.get(&(recv_origin, key)).cloned() {
                        published.push((*dst, ty));
                    }
                }
                _ => {}
            }
        }
    }

    for (value, ty) in published {
        function.metadata.value_types.insert(value, ty);
    }
    override_ordered_map_get_user_box_route_origins(function);
    rewrite_runtime_data_receivers_with_published_origins(function);
}

fn seed_carrier_info_output_schema(
    function: &MirFunction,
    def_map: &ValueDefMap,
    schema: &mut BTreeMap<MapKey, MirType>,
    symbol: &str,
    args: &[ValueId],
) {
    let Some(output_arg) = args.first().copied() else {
        return;
    };
    let output_origin = resolve_value_origin(function, def_map, output_arg);
    match symbol {
        "CarrierInfoApi.from_snapshot/3" => {
            seed_array_key(schema, output_origin, "carrier_names");
            seed_array_key(schema, output_origin, "carrier_host_ids");
        }
        "CarrierInfoApi.with_explicit_carriers_from_snapshot/5" => {
            seed_array_key(schema, output_origin, "requested_names");
            seed_array_key(schema, output_origin, "carrier_names");
            seed_array_key(schema, output_origin, "carrier_host_ids");
        }
        _ => {}
    }
}

fn seed_ordered_map_set_schema(
    function: &MirFunction,
    def_map: &ValueDefMap,
    schema: &mut BTreeMap<MapKey, MirType>,
    receiver: ValueId,
    args: &[ValueId],
) {
    let Some(key_arg) = logical_method_arg(args, receiver, 0) else {
        return;
    };
    let Some(value_arg) = logical_method_arg(args, receiver, 1) else {
        return;
    };
    let Some(key) = const_string(function, def_map, key_arg) else {
        return;
    };
    let Some(ty @ MirType::Box(_)) = value_box_type(function, def_map, value_arg) else {
        return;
    };
    let recv_origin = resolve_value_origin(function, def_map, receiver);
    schema.insert((recv_origin, key), ty);
}

fn seed_array_key(schema: &mut BTreeMap<MapKey, MirType>, receiver: ValueId, key: &str) {
    schema.insert(
        (receiver, key.to_string()),
        MirType::Box("ArrayBox".to_string()),
    );
}

fn logical_method_arg(args: &[ValueId], receiver: ValueId, index: usize) -> Option<ValueId> {
    let start = if args.first().copied() == Some(receiver) { 1 } else { 0 };
    args.get(start + index).copied()
}

fn const_string(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::Const {
            value: ConstValue::String(text),
            ..
        } => Some(text.clone()),
        _ => None,
    }
}

fn value_box_type(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<MirType> {
    let origin = resolve_value_origin(function, def_map, value);
    if let Some(MirType::Box(name)) = function.metadata.value_types.get(&origin) {
        return Some(MirType::Box(name.clone()));
    }
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::NewBox { box_type, .. } => Some(MirType::Box(box_type.clone())),
        _ => None,
    }
}

fn rewrite_runtime_data_receivers_with_published_origins(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut rewrites: Vec<(BasicBlockId, usize, String)> = Vec::new();
    for (block_id, block) in &function.blocks {
        for (instruction_index, instruction) in block.instructions.iter().enumerate() {
            let MirInstruction::Call {
                callee:
                    Some(Callee::Method {
                        box_name,
                        receiver: Some(receiver),
                        ..
                    }),
                ..
            } = instruction
            else {
                continue;
            };
            if box_name != "RuntimeDataBox" {
                continue;
            }
            let origin = resolve_value_origin(function, &def_map, *receiver);
            let Some(MirType::Box(origin_box)) = function.metadata.value_types.get(&origin) else {
                continue;
            };
            if origin_box == "ArrayBox" {
                rewrites.push((*block_id, instruction_index, origin_box.clone()));
            }
        }
    }

    for (block_id, instruction_index, origin_box) in rewrites {
        let Some(block) = function.blocks.get_mut(&block_id) else {
            continue;
        };
        let Some(MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    box_name,
                    certainty,
                    box_kind,
                    ..
                }),
            ..
        }) = block.instructions.get_mut(instruction_index)
        else {
            continue;
        };
        *box_name = origin_box;
        *certainty = TypeCertainty::Known;
        *box_kind = CalleeBoxKind::RuntimeData;
    }
}

fn override_ordered_map_get_user_box_route_origins(function: &mut MirFunction) {
    let value_types = function.metadata.value_types.clone();
    for route in &mut function.metadata.user_box_method_routes {
        if route.box_name() != "OrderedMapBox" || route.method() != "get" {
            continue;
        }
        let Some(result_value) = route.result_value() else {
            continue;
        };
        let Some(MirType::Box(box_name)) = value_types.get(&result_value) else {
            continue;
        };
        if box_name == "ArrayBox" {
            route.override_target_result_box_name(box_name.clone());
        }
    }
}
