use super::GenericMethodRouteProof;
use crate::mir::generic_method_route_facts::const_i64_value;
use crate::mir::value_origin::{resolve_value_origin, ValueDefMap};
use crate::mir::verification::utils::{compute_dominators, DominatorTree};
use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction, ValueId};

#[derive(Clone, Copy)]
struct MapSetCallShape {
    receiver: ValueId,
    key: ValueId,
    value: ValueId,
}

#[derive(Clone, Copy)]
struct MapSetCandidate {
    block: BasicBlockId,
    instruction_index: usize,
    stored_value: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ScalarI64MapGetStoreFact {
    pub route_proof: GenericMethodRouteProof,
    pub stored_value: i64,
}

pub(crate) fn prove_scalar_i64_map_get_store_fact(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    get_instruction_index: usize,
    get_receiver: ValueId,
    get_key: ValueId,
) -> Option<ScalarI64MapGetStoreFact> {
    if let Some(stored_value) = prove_same_block_scalar_i64_map_get(
        function,
        def_map,
        block_id,
        get_instruction_index,
        get_receiver,
        get_key,
    ) {
        return Some(ScalarI64MapGetStoreFact {
            route_proof: GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
            stored_value,
        });
    }
    if let Some(stored_value) =
        prove_dominating_scalar_i64_map_get(function, def_map, block_id, get_receiver, get_key)
    {
        return Some(ScalarI64MapGetStoreFact {
            route_proof: GenericMethodRouteProof::MapSetScalarI64DominatesNoEscape,
            stored_value,
        });
    }
    None
}

fn prove_same_block_scalar_i64_map_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    get_instruction_index: usize,
    get_receiver: ValueId,
    get_key: ValueId,
) -> Option<i64> {
    let Some(get_key_const) = const_i64_value(function, def_map, get_key) else {
        return None;
    };
    let receiver_root = resolve_value_origin(function, def_map, get_receiver);
    let Some(block) = function.blocks.get(&block_id) else {
        return None;
    };

    for inst in block.instructions.iter().take(get_instruction_index).rev() {
        if let Some(set_call) = map_set_call_shape(inst) {
            if same_value_origin(function, def_map, set_call.receiver, receiver_root) {
                let Some(set_key_const) = const_i64_value(function, def_map, set_call.key) else {
                    return None;
                };
                if set_key_const != get_key_const {
                    return None;
                }
                return const_i64_value(function, def_map, set_call.value);
            }
        }

        if instruction_may_escape_or_mutate_receiver(function, def_map, inst, receiver_root) {
            return None;
        }
    }

    None
}

fn prove_dominating_scalar_i64_map_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    get_block_id: BasicBlockId,
    get_receiver: ValueId,
    get_key: ValueId,
) -> Option<i64> {
    let Some(get_key_const) = const_i64_value(function, def_map, get_key) else {
        return None;
    };
    let receiver_root = resolve_value_origin(function, def_map, get_receiver);
    let dominators = compute_dominators(function);
    let mut candidates = Vec::new();

    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        if block_id == get_block_id || !dominators.dominates(block_id, get_block_id) {
            continue;
        }
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            let Some(set_call) = map_set_call_shape(inst) else {
                continue;
            };
            if !same_value_origin(function, def_map, set_call.receiver, receiver_root) {
                continue;
            }
            if const_i64_value(function, def_map, set_call.key) != Some(get_key_const) {
                continue;
            }
            let Some(stored_value) = const_i64_value(function, def_map, set_call.value) else {
                continue;
            };
            candidates.push(MapSetCandidate {
                block: block_id,
                instruction_index,
                stored_value,
            });
        }
    }

    candidates.into_iter().rev().find_map(|candidate| {
        dominating_candidate_has_no_same_receiver_escape(
            function,
            def_map,
            &dominators,
            candidate,
            receiver_root,
        )
        .then_some(candidate.stored_value)
    })
}

fn dominating_candidate_has_no_same_receiver_escape(
    function: &MirFunction,
    def_map: &ValueDefMap,
    dominators: &DominatorTree,
    candidate: MapSetCandidate,
    receiver_root: ValueId,
) -> bool {
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        if !dominators.dominates(candidate.block, block_id) {
            continue;
        }
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        let start = if block_id == candidate.block {
            candidate.instruction_index + 1
        } else {
            0
        };
        for inst in block.instructions.iter().skip(start) {
            if instruction_may_escape_or_mutate_receiver(function, def_map, inst, receiver_root) {
                return false;
            }
        }
    }
    true
}

fn map_set_call_shape(inst: &MirInstruction) -> Option<MapSetCallShape> {
    let MirInstruction::Call {
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if method != "set" || !matches!(box_name.as_str(), "MapBox" | "RuntimeDataBox") {
        return None;
    }

    let (key, value) = match args.as_slice() {
        [key, value] => (*key, *value),
        // Some source routes still carry the receiver as the first argument.
        [_receiver_arg, key, value] => (*key, *value),
        _ => return None,
    };
    Some(MapSetCallShape {
        receiver: *receiver,
        key,
        value,
    })
}

pub(crate) fn instruction_may_escape_or_mutate_receiver(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    receiver_root: ValueId,
) -> bool {
    if !instruction_uses_origin(function, def_map, inst, receiver_root) {
        return false;
    }

    match inst {
        MirInstruction::Copy { .. } | MirInstruction::KeepAlive { .. } => false,
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            ..
        } if same_value_origin(function, def_map, *receiver, receiver_root)
            && matches!(method.as_str(), "get" | "has") =>
        {
            false
        }
        _ => true,
    }
}

fn instruction_uses_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    origin: ValueId,
) -> bool {
    inst.used_values()
        .into_iter()
        .any(|value| same_value_origin(function, def_map, value, origin))
}

fn same_value_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    origin: ValueId,
) -> bool {
    resolve_value_origin(function, def_map, value) == origin
}
