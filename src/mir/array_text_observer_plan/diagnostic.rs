use super::{ArrayTextObserverProducerShapeDiagnostic, ArrayTextObserverRoute};
use crate::mir::value_origin::{resolve_value_origin, ValueDefMap};
use crate::mir::{definitions::Callee, BinaryOp, ConstValue, MirFunction, MirInstruction, ValueId};

pub(super) fn diagnose_append_update_producer_shape(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route: &ArrayTextObserverRoute,
) -> ArrayTextObserverProducerShapeDiagnostic {
    let source_root = root(function, def_map, route.source_value());
    let array_root = root(function, def_map, route.array_value());
    let index_root = root(function, def_map, route.index_value());

    let concat = find_const_suffix_concat(function, def_map, source_root);
    let (same_slot_set_seen, concat_length_use_count, non_length_concat_use_count, length_values) =
        if let Some(concat) = concat {
            let same_slot_set_seen =
                has_same_slot_set(function, def_map, concat.value, array_root, index_root);
            let uses = classify_concat_uses(function, def_map, concat.value);
            (
                same_slot_set_seen,
                uses.length_values.len(),
                uses.non_length_use_count,
                uses.length_values,
            )
        } else {
            (false, 0, 0, Vec::new())
        };

    let row_modulus_const = row_modulus_const(function, def_map, index_root);
    let length_result_feeds_accumulator_add = length_values
        .iter()
        .any(|value| feeds_accumulator_add(function, def_map, *value));

    let const_suffix_concat_seen = concat.is_some();
    let concat_length_use_seen = concat_length_use_count > 0;
    let row_index_mod_const_seen = row_modulus_const.is_some();
    let failure_reason = if !route.has_found_predicate_consumer() {
        "missing_found_predicate"
    } else if !route.observer_arg0_is_const_utf8() {
        "missing_const_utf8_needle"
    } else if !const_suffix_concat_seen {
        "missing_const_suffix_concat"
    } else if !same_slot_set_seen {
        "missing_same_slot_set"
    } else if !concat_length_use_seen {
        "missing_concat_length_use"
    } else if non_length_concat_use_count > 0 {
        "concat_has_non_length_extra_use"
    } else if !row_index_mod_const_seen {
        "missing_row_index_mod_const"
    } else if !length_result_feeds_accumulator_add {
        "missing_length_accumulator_add"
    } else if route.keep_get_live() {
        "store_only_contract_rejects_length_carry"
    } else {
        "store_contract_candidate"
    };

    ArrayTextObserverProducerShapeDiagnostic::new(
        const_suffix_concat_seen,
        same_slot_set_seen,
        concat_length_use_seen,
        concat_length_use_count,
        non_length_concat_use_count,
        row_index_mod_const_seen,
        row_modulus_const,
        length_result_feeds_accumulator_add,
        failure_reason,
    )
}

#[derive(Debug, Clone, Copy)]
struct ConstSuffixConcat {
    value: ValueId,
}

#[derive(Default)]
struct ConcatUseSummary {
    length_values: Vec<ValueId>,
    non_length_use_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RowModSignature {
    source_root: ValueId,
    modulus_const: i64,
}

fn root(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> ValueId {
    resolve_value_origin(function, def_map, value)
}

fn find_const_suffix_concat(
    function: &MirFunction,
    def_map: &ValueDefMap,
    source_root: ValueId,
) -> Option<ConstSuffixConcat> {
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            let MirInstruction::BinOp {
                dst,
                op: BinaryOp::Add,
                lhs,
                rhs,
                ..
            } = inst
            else {
                continue;
            };
            let lhs_root = root(function, def_map, *lhs);
            let rhs_root = root(function, def_map, *rhs);
            if lhs_root == source_root && const_utf8(function, def_map, rhs_root).is_some() {
                return Some(ConstSuffixConcat { value: *dst });
            }
            if rhs_root == source_root && const_utf8(function, def_map, lhs_root).is_some() {
                return Some(ConstSuffixConcat { value: *dst });
            }
        }
    }
    None
}

fn const_utf8(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<String> {
    let value = root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::Const {
            value: ConstValue::String(actual),
            ..
        } => Some(actual.clone()),
        _ => None,
    }
}

fn has_same_slot_set(
    function: &MirFunction,
    def_map: &ValueDefMap,
    concat_value: ValueId,
    array_root: ValueId,
    index_root: ValueId,
) -> bool {
    let concat_root = root(function, def_map, concat_value);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            if is_same_slot_set(function, def_map, inst, concat_root, array_root, index_root) {
                return true;
            }
        }
    }
    false
}

fn is_same_slot_set(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    value_root: ValueId,
    array_root: ValueId,
    index_root: ValueId,
) -> bool {
    match inst {
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
        } if method == "set"
            && args.len() == 2
            && matches!(box_name.as_str(), "RuntimeDataBox" | "ArrayBox") =>
        {
            let actual_index = root(function, def_map, args[0]);
            let same_index = actual_index == index_root
                || row_mod_signature(function, def_map, actual_index)
                    .zip(row_mod_signature(function, def_map, index_root))
                    .is_some_and(|(actual, expected)| actual == expected);
            root(function, def_map, *receiver) == array_root
                && same_index
                && root(function, def_map, args[1]) == value_root
        }
        _ => false,
    }
}

fn classify_concat_uses(
    function: &MirFunction,
    def_map: &ValueDefMap,
    concat_value: ValueId,
) -> ConcatUseSummary {
    let concat_root = root(function, def_map, concat_value);
    let mut summary = ConcatUseSummary::default();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            if is_copy_from(function, def_map, inst, concat_root) {
                continue;
            }
            if is_length_call_on(function, def_map, inst, concat_root)
                .map(|length_value| {
                    summary.length_values.push(length_value);
                })
                .is_some()
            {
                continue;
            }
            if is_set_value_use(function, def_map, inst, concat_root) {
                continue;
            }
            if inst
                .used_values()
                .into_iter()
                .any(|value| root(function, def_map, value) == concat_root)
            {
                summary.non_length_use_count += 1;
            }
        }
        if let Some(term) = &block.terminator {
            if term
                .used_values()
                .into_iter()
                .any(|value| root(function, def_map, value) == concat_root)
            {
                summary.non_length_use_count += 1;
            }
        }
    }

    summary
}

fn is_length_call_on(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    receiver_root: ValueId,
) -> Option<ValueId> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } if method == "length"
            && args.is_empty()
            && root(function, def_map, *receiver) == receiver_root =>
        {
            Some(*dst)
        }
        _ => None,
    }
}

fn is_set_value_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    value_root: ValueId,
) -> bool {
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(_),
                    ..
                }),
            args,
            ..
        } if method == "set" && args.len() == 2 => root(function, def_map, args[1]) == value_root,
        _ => false,
    }
}

fn row_modulus_const(
    function: &MirFunction,
    def_map: &ValueDefMap,
    index_root: ValueId,
) -> Option<i64> {
    row_mod_signature(function, def_map, index_root).map(|signature| signature.modulus_const)
}

fn row_mod_signature(
    function: &MirFunction,
    def_map: &ValueDefMap,
    index_root: ValueId,
) -> Option<RowModSignature> {
    let (block, index) = def_map.get(&index_root).copied()?;
    let MirInstruction::BinOp {
        op: BinaryOp::Mod,
        lhs,
        rhs,
        ..
    } = function.blocks.get(&block)?.instructions.get(index)?
    else {
        return None;
    };
    Some(RowModSignature {
        source_root: root(function, def_map, *lhs),
        modulus_const: const_i64(function, def_map, *rhs)?,
    })
}

fn const_i64(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<i64> {
    let value = root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::Const {
            value: ConstValue::Integer(actual),
            ..
        } => Some(*actual),
        _ => None,
    }
}

fn is_copy_from(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_root: ValueId,
) -> bool {
    matches!(
        inst,
        MirInstruction::Copy { src, .. } if root(function, def_map, *src) == source_root
    )
}

fn feeds_accumulator_add(
    function: &MirFunction,
    def_map: &ValueDefMap,
    length_value: ValueId,
) -> bool {
    let length_root = root(function, def_map, length_value);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            let MirInstruction::BinOp {
                op: BinaryOp::Add,
                lhs,
                rhs,
                ..
            } = inst
            else {
                continue;
            };
            if root(function, def_map, *lhs) == length_root
                || root(function, def_map, *rhs) == length_root
            {
                return true;
            }
        }
    }
    false
}
