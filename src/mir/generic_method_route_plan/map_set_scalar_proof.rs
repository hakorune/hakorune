use super::GenericMethodRouteProof;
use crate::mir::generic_method_route_facts::const_i64_value;
use crate::mir::value_origin::{resolve_value_origin, ValueDefMap};
use crate::mir::verification::utils::{compute_dominators, DominatorTree};
use crate::mir::{BasicBlockId, BinaryOp, Callee, MirFunction, MirInstruction, ValueId};

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
    if prove_covered_dynamic_i64_key_map_get(
        function,
        def_map,
        block_id,
        get_instruction_index,
        get_receiver,
        get_key,
    ) {
        return Some(ScalarI64MapGetStoreFact {
            route_proof: GenericMethodRouteProof::MapSetScalarI64CoveredDynamicI64KeyNoEscape,
            // The dynamic-key proof establishes scalar shape, not one constant value.
            stored_value: 0,
        });
    }
    None
}

fn prove_covered_dynamic_i64_key_map_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    get_block_id: BasicBlockId,
    get_instruction_index: usize,
    get_receiver: ValueId,
    get_key: ValueId,
) -> bool {
    let Some(modulus) = covered_dynamic_key_modulus(function, def_map, get_key) else {
        return false;
    };
    // 296x-864 intentionally opens only the selected i % 3 front. Widening this
    // requires a new proof row with range semantics for arbitrary modulus.
    if modulus != 3 {
        return false;
    }

    let receiver_root = resolve_value_origin(function, def_map, get_receiver);
    let dominators = compute_dominators(function);
    let mut candidate_sets = Vec::new();

    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        if !candidate_block_can_precede_get(&dominators, block_id, get_block_id) {
            continue;
        }
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            if block_id == get_block_id && instruction_index >= get_instruction_index {
                continue;
            }
            let Some(set_call) = map_set_call_shape(inst) else {
                continue;
            };
            if !same_value_origin(function, def_map, set_call.receiver, receiver_root) {
                continue;
            }
            let Some(key) = const_i64_value(function, def_map, set_call.key) else {
                continue;
            };
            if !(0..modulus).contains(&key) {
                continue;
            }
            if const_i64_value(function, def_map, set_call.value).is_none() {
                continue;
            }
            candidate_sets.push(MapSetCandidate {
                block: block_id,
                instruction_index,
                stored_value: key,
            });
        }
    }

    let mut covered = vec![false; modulus as usize];
    for candidate in &candidate_sets {
        covered[candidate.stored_value as usize] = true;
    }
    if !covered.into_iter().all(|is_covered| is_covered) {
        return false;
    }

    let Some(first_candidate) = candidate_sets
        .into_iter()
        .min_by_key(|candidate| (candidate.block, candidate.instruction_index))
    else {
        return false;
    };

    no_unknown_same_receiver_escape_after_candidate(
        function,
        def_map,
        &dominators,
        first_candidate,
        get_block_id,
        receiver_root,
        modulus,
    )
}

fn candidate_block_can_precede_get(
    dominators: &DominatorTree,
    candidate_block: BasicBlockId,
    get_block: BasicBlockId,
) -> bool {
    dominators.dominates(candidate_block, get_block) || candidate_block.0 <= get_block.0
}

fn covered_dynamic_key_modulus(
    function: &MirFunction,
    def_map: &ValueDefMap,
    key: ValueId,
) -> Option<i64> {
    let origin = resolve_value_origin(function, def_map, key);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    let MirInstruction::BinOp {
        op: BinaryOp::Mod,
        lhs,
        rhs,
        ..
    } = block.instructions.get(instruction_index)?
    else {
        return None;
    };
    let modulus = const_i64_value(function, def_map, *rhs)?;
    if modulus <= 0 || !is_standard_nonnegative_loop_index(function, def_map, *lhs) {
        return None;
    }
    Some(modulus)
}

fn is_standard_nonnegative_loop_index(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> bool {
    let origin = resolve_value_origin(function, def_map, value);
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    let Some(MirInstruction::Phi { inputs, .. }) = block.instructions.get(instruction_index) else {
        return false;
    };

    let has_zero_init = inputs
        .iter()
        .any(|(_, input)| const_i64_value(function, def_map, *input) == Some(0));
    let has_increment = inputs
        .iter()
        .any(|(_, input)| is_increment_by_one_of(function, def_map, *input, origin));
    has_zero_init && has_increment
}

fn is_increment_by_one_of(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    phi_origin: ValueId,
) -> bool {
    let origin = resolve_value_origin(function, def_map, value);
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    let Some(MirInstruction::BinOp {
        op: BinaryOp::Add,
        lhs,
        rhs,
        ..
    }) = block.instructions.get(instruction_index)
    else {
        return false;
    };

    (resolve_value_origin(function, def_map, *lhs) == phi_origin
        && const_i64_value(function, def_map, *rhs) == Some(1))
        || (resolve_value_origin(function, def_map, *rhs) == phi_origin
            && const_i64_value(function, def_map, *lhs) == Some(1))
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

fn no_unknown_same_receiver_escape_after_candidate(
    function: &MirFunction,
    def_map: &ValueDefMap,
    dominators: &DominatorTree,
    candidate: MapSetCandidate,
    get_block_id: BasicBlockId,
    receiver_root: ValueId,
    modulus: i64,
) -> bool {
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        if !candidate_scan_contains_block(dominators, candidate.block, block_id, get_block_id) {
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
            if !instruction_uses_origin(function, def_map, inst, receiver_root) {
                continue;
            }
            if covered_preseed_set_shape(function, def_map, inst, receiver_root, modulus) {
                continue;
            }
            if same_receiver_read_shape(function, def_map, inst, receiver_root) {
                continue;
            }
            if matches!(
                inst,
                MirInstruction::Copy { .. } | MirInstruction::KeepAlive { .. }
            ) {
                continue;
            }
            return false;
        }
    }
    true
}

fn candidate_scan_contains_block(
    dominators: &DominatorTree,
    candidate_block: BasicBlockId,
    block_id: BasicBlockId,
    get_block_id: BasicBlockId,
) -> bool {
    if dominators.dominates(candidate_block, block_id) {
        return true;
    }
    // Some builder paths leave BasicBlock successor sets sparse while block ids
    // still preserve the emitted forward order. Use this only as a conservative
    // syntactic fallback for the narrow covered-key proof.
    candidate_block.0 <= block_id.0 && block_id.0 <= get_block_id.0
}

fn covered_preseed_set_shape(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    receiver_root: ValueId,
    modulus: i64,
) -> bool {
    let Some(set_call) = map_set_call_shape(inst) else {
        return false;
    };
    if !same_value_origin(function, def_map, set_call.receiver, receiver_root) {
        return false;
    }
    let Some(key) = const_i64_value(function, def_map, set_call.key) else {
        return false;
    };
    (0..modulus).contains(&key) && const_i64_value(function, def_map, set_call.value).is_some()
}

fn same_receiver_read_shape(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    receiver_root: ValueId,
) -> bool {
    matches!(
        inst,
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            ..
        } if same_value_origin(function, def_map, *receiver, receiver_root)
            && matches!(method.as_str(), "get" | "has")
    )
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

    let (receiver, key, value) = match args.as_slice() {
        [key, value] => (*receiver, *key, *value),
        // Some source routes still carry the semantic receiver as the first
        // argument while the callee receiver points at an adapter value.
        [receiver_arg, key, value] => (*receiver_arg, *key, *value),
        _ => return None,
    };
    Some(MapSetCallShape {
        receiver,
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
