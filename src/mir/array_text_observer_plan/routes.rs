use std::collections::BTreeSet;

use super::super::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use super::super::{
    array_text_observer_region_contract::{
        derive_observer_store_region_contract, ArrayTextObserverExecutorContract,
    },
    definitions::Callee,
    BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction, MirInstruction, MirModule, ValueId,
};

use super::{
    ArrayTextObserverArgRepr, ArrayTextObserverConsumerShape, ArrayTextObserverKind,
    ArrayTextObserverProofRegion, ArrayTextObserverPublicationBoundary,
    ArrayTextObserverResultRepr, ArrayTextObserverRoute,
};

pub fn refresh_module_array_text_observer_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_text_observer_routes(function);
    }
}

pub fn refresh_function_array_text_observer_routes(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut routes = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            if let Some(route) = match_array_text_indexof_route(
                function,
                &def_map,
                block_id,
                instruction_index,
                inst,
            ) {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.observer_instruction_index));
    function.metadata.array_text_observer_routes = routes;
}

fn root(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> ValueId {
    resolve_value_origin(function, def_map, value)
}

#[allow(clippy::too_many_arguments)]
fn match_array_text_indexof_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    observer_instruction_index: usize,
    inst: &MirInstruction,
) -> Option<ArrayTextObserverRoute> {
    let (result_value, receiver_value, observer_arg0) = match_indexof_call(inst)?;
    let source_value = root(function, def_map, receiver_value);
    let (get_block, get_instruction_index, array_value, index_value) =
        match_array_get_source(function, def_map, source_value)?;
    let observer_arg0_root = root(function, def_map, observer_arg0);
    let observer_arg0_repr = const_utf8(function, def_map, observer_arg0_root)
        .map(|text| ArrayTextObserverArgRepr::ConstUtf8 {
            byte_len: text.len(),
            text,
        })
        .unwrap_or(ArrayTextObserverArgRepr::Value);

    let consumer_shape = if has_found_predicate_consumer(function, def_map, result_value) {
        ArrayTextObserverConsumerShape::FoundPredicate
    } else {
        ArrayTextObserverConsumerShape::DirectScalar
    };
    let (selected_route, selected_bridge_symbol) = if observer_arg0_repr.is_const_utf8() {
        (
            "hako.array_text.session_indexof_const_utf8",
            "hako.array_text.session_indexof_const_utf8",
        )
    } else {
        (
            "hako.array_text.session_indexof_handle_needle",
            "hako.array_text.session_indexof_handle_needle",
        )
    };
    let no_covered_source_values = BTreeSet::new();

    let mut route = ArrayTextObserverRoute {
        block,
        observer_instruction_index,
        get_block,
        get_instruction_index,
        array_value,
        index_value,
        source_value,
        observer_kind: ArrayTextObserverKind::IndexOf,
        observer_arg0: observer_arg0_root,
        observer_arg0_repr,
        observer_arg0_keep_live: has_non_observer_value_use(
            function,
            def_map,
            observer_arg0_root,
            observer_arg0,
            block,
            observer_instruction_index,
            &no_covered_source_values,
        ),
        result_value,
        consumer_shape,
        proof_region: ArrayTextObserverProofRegion::ArrayGetReceiverIndexOf,
        publication_boundary: ArrayTextObserverPublicationBoundary::None,
        result_repr: ArrayTextObserverResultRepr::ScalarI64,
        keep_get_live: has_non_observer_source_use(
            function,
            def_map,
            source_value,
            receiver_value,
            array_value,
            index_value,
            block,
            observer_instruction_index,
        ),
        selected_route,
        selected_bridge_symbol,
        fallback_route: "nyash.array.string_indexof_hisi",
        fallback_policy: "fail_fast",
        executor_contract: None,
    };
    route.executor_contract = derive_observer_store_region_contract(function, def_map, &route)
        .map(ArrayTextObserverExecutorContract::conditional_suffix_store_single_region);
    assert!(
        route.selected_route.starts_with("hako.array_text."),
        "selected observer route must stay in the hako.array_text namespace"
    );
    assert!(
        route.selected_bridge_symbol.starts_with("hako.array_text."),
        "observer bridge symbol must stay in the hako.array_text namespace"
    );
    assert_eq!(
        route.fallback_route, "nyash.array.string_indexof_hisi",
        "compat fallback route must stay legacy-only"
    );
    assert_eq!(
        route.fallback_policy, "fail_fast",
        "selected observer route must not silently fallback"
    );
    assert_eq!(
        route.publication_boundary.as_str(),
        "none",
        "selected observer route must not publish inside the selected region"
    );
    if let Some(contract) = route.executor_contract.as_ref() {
        assert_eq!(
            contract.publication_boundary(),
            "none",
            "observer executor contract must not publish inside the selected region"
        );
    }
    Some(route)
}

fn match_indexof_call(inst: &MirInstruction) -> Option<(ValueId, ValueId, ValueId)> {
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
        } if method == "indexOf" && args.len() == 1 => Some((*dst, *receiver, args[0])),
        _ => None,
    }
}

fn match_array_get_source(
    function: &MirFunction,
    def_map: &ValueDefMap,
    source_value: ValueId,
) -> Option<(BasicBlockId, usize, ValueId, ValueId)> {
    let (block, index) = def_map.get(&source_value).copied()?;
    let inst = function.blocks.get(&block)?.instructions.get(index)?;
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(array_value),
                    ..
                }),
            args,
            ..
        } if method == "get"
            && args.len() == 1
            && matches!(box_name.as_str(), "RuntimeDataBox" | "ArrayBox") =>
        {
            Some((
                block,
                index,
                root(function, def_map, *array_value),
                root(function, def_map, args[0]),
            ))
        }
        _ => None,
    }
}

fn has_found_predicate_consumer(
    function: &MirFunction,
    def_map: &ValueDefMap,
    result_value: ValueId,
) -> bool {
    let result_root = root(function, def_map, result_value);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            if compare_is_found_predicate(function, def_map, inst, result_root) {
                return true;
            }
        }
        if let Some(term) = &block.terminator {
            if compare_is_found_predicate(function, def_map, term, result_root) {
                return true;
            }
        }
    }
    false
}

fn compare_is_found_predicate(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    result_root: ValueId,
) -> bool {
    let MirInstruction::Compare { op, lhs, rhs, .. } = inst else {
        return false;
    };
    let lhs_root = root(function, def_map, *lhs);
    let rhs_root = root(function, def_map, *rhs);
    if lhs_root == result_root {
        return match const_i64(function, def_map, *rhs) {
            Some(0) => matches!(op, CompareOp::Ge),
            Some(-1) => matches!(op, CompareOp::Gt | CompareOp::Ne),
            _ => false,
        };
    }
    if rhs_root == result_root {
        return match const_i64(function, def_map, *lhs) {
            Some(0) => matches!(op, CompareOp::Le),
            Some(-1) => matches!(op, CompareOp::Lt | CompareOp::Ne),
            _ => false,
        };
    }
    false
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

fn has_non_observer_source_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    source_value: ValueId,
    observer_receiver_value: ValueId,
    array_value: ValueId,
    index_value: ValueId,
    observer_block: BasicBlockId,
    observer_instruction_index: usize,
) -> bool {
    let covered_slot_suffix_values = same_slot_const_suffix_store_source_values(
        function,
        def_map,
        source_value,
        array_value,
        index_value,
    );
    has_non_observer_value_use(
        function,
        def_map,
        source_value,
        observer_receiver_value,
        observer_block,
        observer_instruction_index,
        &covered_slot_suffix_values,
    )
}

fn has_non_observer_value_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    source_value: ValueId,
    observer_value: ValueId,
    observer_block: BasicBlockId,
    observer_instruction_index: usize,
    covered_source_values: &BTreeSet<ValueId>,
) -> bool {
    let source_root = root(function, def_map, source_value);
    let observer_chain = copy_chain_values(function, def_map, observer_value);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            if block_id == observer_block && instruction_index == observer_instruction_index {
                continue;
            }
            if is_observer_chain_copy(function, def_map, inst, source_root, &observer_chain) {
                continue;
            }
            if is_covered_slot_consumer_source_use(
                function,
                def_map,
                inst,
                source_root,
                covered_source_values,
            ) {
                continue;
            }
            if inst
                .used_values()
                .into_iter()
                .any(|value| root(function, def_map, value) == source_root)
            {
                return true;
            }
        }
        if let Some(term) = &block.terminator {
            if term
                .used_values()
                .into_iter()
                .any(|value| root(function, def_map, value) == source_root)
            {
                return true;
            }
        }
    }
    false
}

fn same_slot_const_suffix_store_source_values(
    function: &MirFunction,
    def_map: &ValueDefMap,
    source_value: ValueId,
    array_value: ValueId,
    index_value: ValueId,
) -> BTreeSet<ValueId> {
    let source_root = root(function, def_map, source_value);
    let array_root = root(function, def_map, array_value);
    let index_root = root(function, def_map, index_value);
    let mut covered = BTreeSet::new();

    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            let Some((concat_value, source_operand)) =
                const_suffix_concat_from_source(function, def_map, inst, source_root)
            else {
                continue;
            };
            if !has_same_slot_set_consumer(function, def_map, concat_value, array_root, index_root)
            {
                continue;
            }
            covered.extend(copy_chain_values(function, def_map, source_operand));
            covered.insert(source_root);
            covered.insert(concat_value);
        }
    }

    covered
}

fn const_suffix_concat_from_source(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_root: ValueId,
) -> Option<(ValueId, ValueId)> {
    let MirInstruction::BinOp {
        dst,
        op: BinaryOp::Add,
        lhs,
        rhs,
        ..
    } = inst
    else {
        return None;
    };
    let lhs_root = root(function, def_map, *lhs);
    let rhs_root = root(function, def_map, *rhs);
    if lhs_root == source_root && const_utf8(function, def_map, rhs_root).is_some() {
        return Some((*dst, *lhs));
    }
    if rhs_root == source_root && const_utf8(function, def_map, lhs_root).is_some() {
        return Some((*dst, *rhs));
    }
    None
}

fn has_same_slot_set_consumer(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    array_root: ValueId,
    index_root: ValueId,
) -> bool {
    let value_root = root(function, def_map, value);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            if is_same_slot_set_consumer(
                function, def_map, inst, value_root, array_root, index_root,
            ) {
                return true;
            }
        }
    }
    false
}

fn is_same_slot_set_consumer(
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
            root(function, def_map, *receiver) == array_root
                && root(function, def_map, args[0]) == index_root
                && root(function, def_map, args[1]) == value_root
        }
        _ => false,
    }
}

fn is_covered_slot_consumer_source_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_root: ValueId,
    covered_source_values: &BTreeSet<ValueId>,
) -> bool {
    match inst {
        MirInstruction::Copy { dst, src } => {
            covered_source_values.contains(dst) && root(function, def_map, *src) == source_root
        }
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
            ..
        } => {
            covered_source_values.contains(dst)
                && (root(function, def_map, *lhs) == source_root
                    || root(function, def_map, *rhs) == source_root)
        }
        _ => false,
    }
}

fn copy_chain_values(
    function: &MirFunction,
    def_map: &ValueDefMap,
    mut value: ValueId,
) -> BTreeSet<ValueId> {
    let mut chain = BTreeSet::new();
    while chain.insert(value) {
        let Some((block, index)) = def_map.get(&value).copied() else {
            break;
        };
        let Some(inst) = function
            .blocks
            .get(&block)
            .and_then(|block| block.instructions.get(index))
        else {
            break;
        };
        match inst {
            MirInstruction::Copy { src, .. } => value = *src,
            _ => break,
        }
    }
    chain
}

fn is_observer_chain_copy(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_root: ValueId,
    observer_chain: &BTreeSet<ValueId>,
) -> bool {
    match inst {
        MirInstruction::Copy { dst, src } => {
            observer_chain.contains(dst) && root(function, def_map, *src) == source_root
        }
        _ => false,
    }
}
