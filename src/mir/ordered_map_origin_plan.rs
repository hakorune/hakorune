/*!
 * Focused carrier-data OrderedMapBox result-origin publication.
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

pub fn refresh_module_carrier_api_ordered_map_get_result_origins(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        match function.signature.name.as_str() {
            "CarrierInfoApi.from_snapshot/3"
            | "CarrierInfoApi.with_explicit_carriers_from_snapshot/5" => {
                refresh_function_ordered_map_get_result_origins(function);
            }
            _ => {}
        }
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
    override_ordered_map_get_generic_route_origins(function);
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

fn override_ordered_map_get_generic_route_origins(function: &mut MirFunction) {
    let value_types = function.metadata.value_types.clone();
    for route in &mut function.metadata.generic_method_routes {
        if route.box_name() != "OrderedMapBox" || route.method() != "get" {
            continue;
        }
        let Some(result_value) = route.result_value() else {
            continue;
        };
        let Some(MirType::Box(box_name)) = value_types.get(&result_value) else {
            continue;
        };
        route.override_result_origin_box(box_name.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirModule, MirType, ValueId,
    };
    use std::collections::BTreeMap;

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn push_const(block: &mut crate::mir::BasicBlock, dst: u32, value: ConstValue) {
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(dst),
            value,
        });
    }

    fn push_call(
        block: &mut crate::mir::BasicBlock,
        dst: u32,
        callee: Callee,
        args: Vec<u32>,
        effects: EffectMask,
    ) {
        block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(dst)),
            func: ValueId::INVALID,
            callee: Some(callee),
            args: args.into_iter().map(ValueId::new).collect(),
            effects,
        });
    }

    fn string_consts(function: &MirFunction) -> BTreeMap<ValueId, String> {
        let mut out = BTreeMap::new();
        for block in function.blocks.values() {
            for instruction in &block.instructions {
                if let MirInstruction::Const {
                    dst,
                    value: ConstValue::String(text),
                } = instruction
                {
                    out.insert(*dst, text.clone());
                }
            }
        }
        out
    }

    fn method_calls<'a>(
        function: &'a MirFunction,
        box_name: &str,
        method: &str,
    ) -> Vec<&'a MirInstruction> {
        let mut calls = Vec::new();
        for block in function.blocks.values() {
            for instruction in &block.instructions {
                if let MirInstruction::Call {
                    callee:
                        Some(Callee::Method {
                            box_name: call_box_name,
                            method: call_method,
                            ..
                        }),
                    ..
                } = instruction
                {
                    if call_box_name == box_name && call_method == method {
                        calls.push(instruction);
                    }
                }
            }
        }
        calls
    }

    #[test]
    fn publishes_arraybox_origin_and_rewrites_nested_reads() {
        let mut module = MirModule::new("ordered_map_origin_plan_test".to_string());
        let mut function = make_function();
        let entry = function.get_block_mut(BasicBlockId::new(0)).expect("entry");

        entry.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "OrderedMapBox".to_string(),
            args: vec![],
        });
        push_const(entry, 2, ConstValue::String("i".to_string()));
        push_const(entry, 3, ConstValue::String("snapshot".to_string()));
        push_call(
            entry,
            10,
            Callee::Global("CarrierInfoApi.from_snapshot/3".to_string()),
            vec![1, 2, 3],
            EffectMask::IO,
        );
        push_const(entry, 4, ConstValue::String("carrier_names".to_string()));
        push_call(
            entry,
            11,
            Callee::Method {
                box_name: "OrderedMapBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            },
            vec![4],
            EffectMask::PURE,
        );
        push_const(entry, 5, ConstValue::Integer(0));
        push_call(
            entry,
            12,
            Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(11)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            },
            vec![5],
            EffectMask::PURE,
        );

        module.add_function(function);
        refresh_module_ordered_map_get_result_origins(&mut module);

        let function = module.get_function("main").expect("main");
        assert_eq!(
            function.metadata.value_types.get(&ValueId::new(11)),
            Some(&MirType::Box("ArrayBox".to_string()))
        );

        let consts = string_consts(function);
        let ordered_map_get_calls = method_calls(function, "OrderedMapBox", "get");
        assert_eq!(ordered_map_get_calls.len(), 1);
        let ordered_map_call = ordered_map_get_calls[0];
        let ordered_map_dst = match ordered_map_call {
            MirInstruction::Call { dst, args, .. } => {
                assert_eq!(consts.get(&args[0]), Some(&"carrier_names".to_string()));
                dst.expect("dst")
            }
            _ => unreachable!(),
        };
        assert_eq!(ordered_map_dst, ValueId::new(11));

        let nested_calls = method_calls(function, "ArrayBox", "get");
        assert_eq!(nested_calls.len(), 1);
        match nested_calls[0] {
            MirInstruction::Call {
                callee:
                    Some(Callee::Method {
                        box_name,
                        method,
                        receiver: Some(receiver),
                        ..
                    }),
                args,
                ..
            } => {
                assert_eq!(box_name, "ArrayBox");
                assert_eq!(method, "get");
                assert_eq!(*receiver, ValueId::new(11));
                assert_eq!(args.len(), 1);
            }
            _ => panic!("expected rewritten ArrayBox.get call"),
        }
    }

    #[test]
    fn publishes_arraybox_origin_for_explicit_carrier_snapshot_reads() {
        let mut module = MirModule::new("ordered_map_origin_plan_explicit_test".to_string());
        let mut function = make_function();
        let entry = function.get_block_mut(BasicBlockId::new(0)).expect("entry");

        entry.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "OrderedMapBox".to_string(),
            args: vec![],
        });
        push_const(entry, 2, ConstValue::String("i".to_string()));
        push_const(entry, 3, ConstValue::String("snapshot".to_string()));
        push_call(
            entry,
            10,
            Callee::Global("CarrierInfoApi.with_explicit_carriers_from_snapshot/5".to_string()),
            vec![1, 2, 99, 3, 4],
            EffectMask::IO,
        );
        push_const(entry, 4, ConstValue::String("requested_names".to_string()));
        push_call(
            entry,
            11,
            Callee::Method {
                box_name: "OrderedMapBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            },
            vec![4],
            EffectMask::PURE,
        );
        push_const(entry, 5, ConstValue::Integer(0));
        push_call(
            entry,
            12,
            Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(11)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            },
            vec![5],
            EffectMask::PURE,
        );
        push_const(entry, 6, ConstValue::String("carrier_names".to_string()));
        push_call(
            entry,
            13,
            Callee::Method {
                box_name: "OrderedMapBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            },
            vec![6],
            EffectMask::PURE,
        );
        push_const(entry, 7, ConstValue::Integer(1));
        push_call(
            entry,
            14,
            Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(13)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            },
            vec![7],
            EffectMask::PURE,
        );
        push_const(entry, 8, ConstValue::String("carrier_host_ids".to_string()));
        push_call(
            entry,
            15,
            Callee::Method {
                box_name: "OrderedMapBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            },
            vec![8],
            EffectMask::PURE,
        );
        push_const(entry, 9, ConstValue::Integer(2));
        push_call(
            entry,
            16,
            Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(15)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            },
            vec![9],
            EffectMask::PURE,
        );

        module.add_function(function);
        refresh_module_ordered_map_get_result_origins(&mut module);

        let function = module.get_function("main").expect("main");
        for value in [11_u32, 13, 15] {
            assert_eq!(
                function.metadata.value_types.get(&ValueId::new(value)),
                Some(&MirType::Box("ArrayBox".to_string()))
            );
        }

        let consts = string_consts(function);
        let ordered_map_get_calls = method_calls(function, "OrderedMapBox", "get");
        assert_eq!(ordered_map_get_calls.len(), 3);
        let mut seen = BTreeMap::new();
        for call in ordered_map_get_calls {
            match call {
                MirInstruction::Call {
                    dst: Some(dst),
                    args,
                    ..
                } => {
                    let key = consts.get(&args[0]).cloned().expect("string key");
                    seen.insert(key, *dst);
                }
                _ => unreachable!(),
            }
        }
        assert_eq!(seen.get("requested_names"), Some(&ValueId::new(11)));
        assert_eq!(seen.get("carrier_names"), Some(&ValueId::new(13)));
        assert_eq!(seen.get("carrier_host_ids"), Some(&ValueId::new(15)));

        let array_get_calls = method_calls(function, "ArrayBox", "get");
        assert_eq!(array_get_calls.len(), 3);
        for call in array_get_calls {
            match call {
                MirInstruction::Call {
                    callee:
                        Some(Callee::Method {
                            box_name,
                            method,
                            receiver: Some(receiver),
                            ..
                        }),
                    args,
                    ..
                } => {
                    assert_eq!(box_name, "ArrayBox");
                    assert_eq!(method, "get");
                    assert!(matches!(
                        receiver,
                        ValueId(11) | ValueId(13) | ValueId(15)
                    ));
                    assert_eq!(args.len(), 1);
                }
                _ => panic!("expected rewritten ArrayBox.get call"),
            }
        }
    }
}
