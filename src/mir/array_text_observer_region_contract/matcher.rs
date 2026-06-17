use super::model::ArrayTextObserverStoreRegionMapping;
use crate::mir::value_origin::{resolve_value_origin, ValueDefMap};
use crate::mir::{
    array_text_observer_plan::ArrayTextObserverRoute, definitions::Callee, BasicBlock,
    BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction, MirInstruction, ValueId,
};

pub(crate) fn derive_observer_store_region_contract(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route: &ArrayTextObserverRoute,
) -> Option<ArrayTextObserverStoreRegionMapping> {
    if !route.has_found_predicate_consumer() {
        return None;
    }
    if !route.observer_arg0_is_const_utf8() {
        return None;
    }
    if route.keep_get_live() || route.observer_arg0_keep_live() {
        return None;
    }

    let observer_block = function.blocks.get(&route.block())?;
    let (predicate_value, branch_then_block, branch_else_block) =
        match_found_predicate_branch(function, def_map, observer_block, route.result_value())?;
    let (then_block, latch_block, store_instruction_index, suffix_value, suffix_text) =
        if let Some((store_instruction_index, suffix_value, suffix_text)) =
            match_then_same_slot_suffix_store(function, def_map, branch_then_block, route)
        {
            (
                branch_then_block,
                branch_else_block,
                store_instruction_index,
                suffix_value,
                suffix_text,
            )
        } else if let Some((store_instruction_index, suffix_value, suffix_text)) =
            match_then_same_slot_suffix_store(function, def_map, branch_else_block, route)
        {
            (
                branch_else_block,
                branch_then_block,
                store_instruction_index,
                suffix_value,
                suffix_text,
            )
        } else {
            return None;
        };
    let latch_pred_block = then_block_reaches_latch(function, then_block, latch_block)?;
    let latch = function.blocks.get(&latch_block)?;
    let header_block = match block_terminal(latch)? {
        MirInstruction::Jump { target, .. } => *target,
        _ => return None,
    };
    let header = function.blocks.get(&header_block)?;
    if !block_has_predecessor(function, header_block, latch_block) {
        return None;
    }
    let exit_block = match block_terminal(header)? {
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *then_bb == route.block() => *else_bb,
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *else_bb == route.block() => *then_bb,
        _ => return None,
    };
    let exit = function.blocks.get(&exit_block)?;
    if exit
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Phi { .. }))
    {
        return None;
    }
    if !block_has_predecessor(function, route.block(), header_block)
        || !block_has_predecessor(function, latch_block, route.block())
        || !block_has_predecessor(function, latch_block, latch_pred_block)
    {
        return None;
    }
    let preheader = single_preheader_jump_to_header(function, header_block, latch_block)?;
    let (loop_index_phi_value, loop_bound_value) =
        match_loop_index_condition(function, def_map, header)?;
    if loop_index_phi_value != root(function, def_map, route.index_value()) {
        return None;
    }
    let loop_index_initial_value = phi_input_from(header, loop_index_phi_value, preheader)?;
    let loop_index_initial_const = const_i64(function, def_map, loop_index_initial_value)?;
    if loop_index_initial_const != 0 {
        return None;
    }
    let loop_index_next_value = phi_input_from(header, loop_index_phi_value, latch_block)?;
    if !is_add_const_one_from(
        function,
        def_map,
        latch,
        loop_index_next_value,
        loop_index_phi_value,
    ) {
        return None;
    }
    let loop_bound_const = const_i64(function, def_map, loop_bound_value)?;
    let suffix_byte_len = suffix_text.len();

    Some(ArrayTextObserverStoreRegionMapping::new(
        root(function, def_map, route.array_value()),
        loop_index_phi_value,
        loop_index_initial_value,
        loop_index_initial_const,
        loop_index_next_value,
        loop_bound_value,
        loop_bound_const,
        preheader,
        header_block,
        header_block,
        route.block(),
        route.observer_instruction_index(),
        predicate_value,
        then_block,
        store_instruction_index,
        suffix_value,
        suffix_text,
        suffix_byte_len,
        latch_block,
        exit_block,
    ))
}

pub(crate) fn derive_observer_store_len_sum_region_contract(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route: &ArrayTextObserverRoute,
) -> Option<ArrayTextObserverStoreRegionMapping> {
    if !route.has_found_predicate_consumer() {
        return None;
    }
    if !route.observer_arg0_is_const_utf8() {
        return None;
    }
    if route.observer_arg0_keep_live() {
        return None;
    }

    let observer_block = function.blocks.get(&route.block())?;
    let (predicate_value, branch_then_block, branch_else_block) =
        match_found_predicate_branch(function, def_map, observer_block, route.result_value())?;
    let (loop_index_phi_value, loop_bound_value) =
        match_observer_loop_shape(function, def_map, route)?;
    let (row_index_value, row_modulus_value, row_modulus_const) =
        match_row_modulus(function, def_map, route.index_value(), loop_index_phi_value)?;
    let (then_block, latch_block, store_payload) = if let Some(payload) =
        match_then_same_row_suffix_store_len_sum(
            function,
            def_map,
            branch_then_block,
            route,
            loop_index_phi_value,
            row_modulus_const,
        ) {
        (branch_then_block, branch_else_block, payload)
    } else if let Some(payload) = match_then_same_row_suffix_store_len_sum(
        function,
        def_map,
        branch_else_block,
        route,
        loop_index_phi_value,
        row_modulus_const,
    ) {
        (branch_else_block, branch_then_block, payload)
    } else {
        return None;
    };

    let latch_pred_block = then_block_reaches_latch(function, then_block, latch_block)?;
    let latch = function.blocks.get(&latch_block)?;
    let header_block = match block_terminal(latch)? {
        MirInstruction::Jump { target, .. } => *target,
        _ => return None,
    };
    let header = function.blocks.get(&header_block)?;
    if !block_has_predecessor(function, header_block, latch_block) {
        return None;
    }
    let exit_block = match block_terminal(header)? {
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *then_bb == route.block() => *else_bb,
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *else_bb == route.block() => *then_bb,
        _ => return None,
    };
    let exit = function.blocks.get(&exit_block)?;
    if !exit_has_only_header_phi(exit, header_block) {
        return None;
    }
    if !block_has_predecessor(function, route.block(), header_block)
        || !block_has_predecessor(function, latch_block, route.block())
        || !block_has_predecessor(function, latch_block, latch_pred_block)
    {
        return None;
    }
    let preheader = single_preheader_jump_to_header(function, header_block, latch_block)?;
    let loop_index_initial_value = phi_input_from(header, loop_index_phi_value, preheader)?;
    let loop_index_initial_const = const_i64(function, def_map, loop_index_initial_value)?;
    if loop_index_initial_const != 0 {
        return None;
    }
    let loop_index_next_value = phi_input_from(header, loop_index_phi_value, latch_block)?;
    if !is_add_const_one_from(
        function,
        def_map,
        latch,
        loop_index_next_value,
        loop_index_phi_value,
    ) {
        return None;
    }
    let loop_bound_const = const_i64(function, def_map, loop_bound_value)?;
    let suffix_byte_len = store_payload.suffix_text.len();

    let mapping = ArrayTextObserverStoreRegionMapping::new(
        root(function, def_map, route.array_value()),
        loop_index_phi_value,
        loop_index_initial_value,
        loop_index_initial_const,
        loop_index_next_value,
        loop_bound_value,
        loop_bound_const,
        preheader,
        header_block,
        header_block,
        route.block(),
        route.observer_instruction_index(),
        predicate_value,
        then_block,
        store_payload.store_instruction_index,
        store_payload.suffix_value,
        store_payload.suffix_text,
        suffix_byte_len,
        latch_block,
        exit_block,
    )
    .with_len_sum_payload(
        row_index_value,
        row_modulus_value,
        row_modulus_const,
        store_payload.length_result_value,
        store_payload.accumulator_phi_value,
        store_payload.accumulator_next_value,
    );
    Some(mapping)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LenSumStorePayload {
    store_instruction_index: usize,
    suffix_value: ValueId,
    suffix_text: String,
    length_result_value: ValueId,
    accumulator_phi_value: ValueId,
    accumulator_next_value: ValueId,
}

fn root(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> ValueId {
    resolve_value_origin(function, def_map, value)
}

fn block_terminal(block: &BasicBlock) -> Option<&MirInstruction> {
    block.terminator.as_ref().or_else(|| {
        block
            .instructions
            .last()
            .filter(|inst| is_terminal_instruction(inst))
    })
}

fn is_terminal_instruction(inst: &MirInstruction) -> bool {
    matches!(
        inst,
        MirInstruction::Jump { .. } | MirInstruction::Branch { .. } | MirInstruction::Return { .. }
    )
}

fn block_successors(block: &BasicBlock) -> Vec<BasicBlockId> {
    match block_terminal(block) {
        Some(MirInstruction::Jump { target, .. }) => vec![*target],
        Some(MirInstruction::Branch {
            then_bb, else_bb, ..
        }) => vec![*then_bb, *else_bb],
        _ => Vec::new(),
    }
}

fn block_predecessors(function: &MirFunction, block: BasicBlockId) -> Vec<BasicBlockId> {
    let mut predecessors: Vec<_> = function
        .blocks
        .iter()
        .filter_map(|(candidate, candidate_block)| {
            block_successors(candidate_block)
                .contains(&block)
                .then_some(*candidate)
        })
        .collect();
    predecessors.sort_by_key(|block| block.as_u32());
    predecessors
}

fn block_has_predecessor(
    function: &MirFunction,
    block: BasicBlockId,
    predecessor: BasicBlockId,
) -> bool {
    function
        .blocks
        .get(&predecessor)
        .is_some_and(|candidate| block_successors(candidate).contains(&block))
}

fn exit_has_only_header_phi(exit: &BasicBlock, header_block: BasicBlockId) -> bool {
    exit.instructions.iter().all(|inst| match inst {
        MirInstruction::Phi { inputs, .. } => {
            inputs.iter().all(|(block, _)| *block == header_block)
        }
        _ => true,
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

fn then_block_reaches_latch(
    function: &MirFunction,
    then_block: BasicBlockId,
    latch_block: BasicBlockId,
) -> Option<BasicBlockId> {
    let then = function.blocks.get(&then_block)?;
    let first_jump_target = match block_terminal(then)? {
        MirInstruction::Jump { target, .. } => *target,
        _ => return None,
    };
    if first_jump_target == latch_block {
        return Some(then_block);
    }

    let bridge = function.blocks.get(&first_jump_target)?;
    if block_predecessors(function, first_jump_target) != vec![then_block] {
        return None;
    }
    if !block_has_only_session_safe_lifetime_bookkeeping(bridge) {
        return None;
    }
    match block_terminal(bridge)? {
        MirInstruction::Jump { target, .. } if *target == latch_block => Some(first_jump_target),
        _ => None,
    }
}

fn block_has_only_session_safe_lifetime_bookkeeping(block: &BasicBlock) -> bool {
    block
        .instructions
        .iter()
        .filter(|inst| !is_terminal_instruction(inst))
        .all(is_session_safe_lifetime_bookkeeping)
}

fn is_session_safe_lifetime_bookkeeping(inst: &MirInstruction) -> bool {
    matches!(
        inst,
        MirInstruction::Const { .. }
            | MirInstruction::Copy { .. }
            | MirInstruction::BinOp { .. }
            | MirInstruction::Compare { .. }
            | MirInstruction::Phi { .. }
            | MirInstruction::Select { .. }
            | MirInstruction::KeepAlive { .. }
    )
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

fn match_found_predicate_branch(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &BasicBlock,
    result_value: ValueId,
) -> Option<(ValueId, BasicBlockId, BasicBlockId)> {
    let result_root = root(function, def_map, result_value);
    let predicate_value = block.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Compare { dst, .. }
            if compare_is_found_predicate(function, def_map, inst, result_root) =>
        {
            Some(*dst)
        }
        _ => None,
    })?;
    let MirInstruction::Branch {
        condition,
        then_bb,
        else_bb,
        ..
    } = block_terminal(block)?
    else {
        return None;
    };
    if root(function, def_map, *condition) != root(function, def_map, predicate_value) {
        return None;
    }
    Some((predicate_value, *then_bb, *else_bb))
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

fn match_then_same_slot_suffix_store(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    route: &ArrayTextObserverRoute,
) -> Option<(usize, ValueId, String)> {
    let block = function.blocks.get(&block_id)?;
    let source_root = root(function, def_map, route.source_value());
    let array_root = root(function, def_map, route.array_value());
    let index_root = root(function, def_map, route.index_value());
    for inst in &block.instructions {
        let Some((concat_value, suffix_value, suffix_text)) =
            const_suffix_concat_details(function, def_map, inst, source_root)
        else {
            continue;
        };
        for (store_instruction_index, store) in block.instructions.iter().enumerate() {
            if is_same_slot_set_consumer(
                function,
                def_map,
                store,
                concat_value,
                array_root,
                index_root,
            ) {
                return Some((store_instruction_index, suffix_value, suffix_text));
            }
        }
    }
    None
}

fn match_observer_loop_shape(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route: &ArrayTextObserverRoute,
) -> Option<(ValueId, ValueId)> {
    let header_block = block_predecessors(function, route.block())
        .into_iter()
        .find(|candidate| {
            function
                .blocks
                .get(candidate)
                .and_then(block_terminal)
                .is_some_and(|term| {
                    matches!(
                        term,
                        MirInstruction::Branch { then_bb, else_bb, .. }
                            if *then_bb == route.block() || *else_bb == route.block()
                    )
                })
        })?;
    let header = function.blocks.get(&header_block)?;
    match_loop_index_condition(function, def_map, header)
}

fn match_row_modulus(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    loop_index_phi_value: ValueId,
) -> Option<(ValueId, ValueId, i64)> {
    let value = root(function, def_map, value);
    let (block_id, instruction_index) = def_map.get(&value).copied()?;
    let block = function.blocks.get(&block_id)?;
    let MirInstruction::BinOp {
        dst,
        op: BinaryOp::Mod,
        lhs,
        rhs,
    } = block.instructions.get(instruction_index)?
    else {
        return None;
    };
    if root(function, def_map, *dst) != value {
        return None;
    }
    let modulus_const = if root(function, def_map, *lhs) == loop_index_phi_value {
        const_i64(function, def_map, *rhs)?
    } else if root(function, def_map, *rhs) == loop_index_phi_value {
        const_i64(function, def_map, *lhs)?
    } else {
        return None;
    };
    if modulus_const <= 0 {
        return None;
    }
    Some((value, value, modulus_const))
}

fn match_then_same_row_suffix_store_len_sum(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    route: &ArrayTextObserverRoute,
    loop_index_phi_value: ValueId,
    row_modulus_const: i64,
) -> Option<LenSumStorePayload> {
    let block = function.blocks.get(&block_id)?;
    let source_root = root(function, def_map, route.source_value());
    let array_root = root(function, def_map, route.array_value());
    for inst in &block.instructions {
        let Some((concat_value, suffix_value, suffix_text)) =
            const_suffix_concat_details(function, def_map, inst, source_root)
        else {
            continue;
        };
        let Some((store_instruction_index, _store_index_value)) = block
            .instructions
            .iter()
            .enumerate()
            .find_map(|(instruction_index, store)| {
                is_same_row_set_consumer(
                    function,
                    def_map,
                    store,
                    concat_value,
                    array_root,
                    loop_index_phi_value,
                    row_modulus_const,
                )
                .map(|index_value| (instruction_index, index_value))
            })
        else {
            continue;
        };
        let length_result_value = match_length_result_of(function, def_map, block, concat_value)?;
        let (accumulator_phi_value, accumulator_next_value) =
            match_accumulator_add(function, def_map, block, length_result_value)?;
        return Some(LenSumStorePayload {
            store_instruction_index,
            suffix_value,
            suffix_text,
            length_result_value,
            accumulator_phi_value,
            accumulator_next_value,
        });
    }
    None
}

fn const_suffix_concat_details(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    source_root: ValueId,
) -> Option<(ValueId, ValueId, String)> {
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
    if lhs_root == source_root {
        let suffix_text = const_utf8(function, def_map, rhs_root)?;
        return Some((*dst, rhs_root, suffix_text));
    }
    if rhs_root == source_root {
        let suffix_text = const_utf8(function, def_map, lhs_root)?;
        return Some((*dst, lhs_root, suffix_text));
    }
    None
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

fn is_same_row_set_consumer(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    value_root: ValueId,
    array_root: ValueId,
    loop_index_phi_value: ValueId,
    row_modulus_const: i64,
) -> Option<ValueId> {
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
            && matches!(box_name.as_str(), "RuntimeDataBox" | "ArrayBox")
            && root(function, def_map, *receiver) == array_root
            && root(function, def_map, args[1]) == value_root =>
        {
            let (index_value, _, actual_modulus_const) =
                match_row_modulus(function, def_map, args[0], loop_index_phi_value)?;
            (actual_modulus_const == row_modulus_const).then_some(index_value)
        }
        _ => None,
    }
}

fn match_length_result_of(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &BasicBlock,
    concat_value: ValueId,
) -> Option<ValueId> {
    let concat_root = root(function, def_map, concat_value);
    block.instructions.iter().find_map(|inst| match inst {
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
            && root(function, def_map, *receiver) == concat_root =>
        {
            Some(*dst)
        }
        _ => None,
    })
}

fn match_accumulator_add(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &BasicBlock,
    length_result_value: ValueId,
) -> Option<(ValueId, ValueId)> {
    let length_result_root = root(function, def_map, length_result_value);
    block.instructions.iter().find_map(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } => {
            let lhs_root = root(function, def_map, *lhs);
            let rhs_root = root(function, def_map, *rhs);
            if lhs_root == length_result_root && is_header_phi_like(function, def_map, rhs_root) {
                return Some((rhs_root, *dst));
            }
            if rhs_root == length_result_root && is_header_phi_like(function, def_map, lhs_root) {
                return Some((lhs_root, *dst));
            }
            None
        }
        _ => None,
    })
}

fn is_header_phi_like(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> bool {
    let value = root(function, def_map, value);
    let Some((block_id, instruction_index)) = def_map.get(&value).copied() else {
        return false;
    };
    function
        .blocks
        .get(&block_id)
        .and_then(|block| block.instructions.get(instruction_index))
        .is_some_and(|inst| matches!(inst, MirInstruction::Phi { dst, .. } if *dst == value))
}

fn single_preheader_jump_to_header(
    function: &MirFunction,
    header_block: BasicBlockId,
    latch_block: BasicBlockId,
) -> Option<BasicBlockId> {
    let mut non_latch_predecessors = block_predecessors(function, header_block)
        .into_iter()
        .filter(|predecessor| *predecessor != latch_block);
    let preheader = non_latch_predecessors.next()?;
    if non_latch_predecessors.next().is_some() {
        return None;
    }
    let preheader_block = function.blocks.get(&preheader)?;
    match block_terminal(preheader_block)? {
        MirInstruction::Jump { target, .. } if *target == header_block => Some(preheader),
        _ => None,
    }
}

fn match_loop_index_condition(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &BasicBlock,
) -> Option<(ValueId, ValueId)> {
    let condition = match block_terminal(header)? {
        MirInstruction::Branch { condition, .. } => *condition,
        _ => return None,
    };
    let condition = root(function, def_map, condition);
    let compare = header.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Compare {
            dst,
            op: CompareOp::Lt,
            lhs,
            rhs,
        } if root(function, def_map, *dst) == condition => Some((*lhs, *rhs)),
        _ => None,
    })?;
    let loop_index_phi_value = root(function, def_map, compare.0);
    if !is_phi_dst(header, loop_index_phi_value) {
        return None;
    }
    Some((loop_index_phi_value, root(function, def_map, compare.1)))
}

fn is_phi_dst(block: &BasicBlock, value: ValueId) -> bool {
    block.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::Phi { dst, .. } if *dst == value
        )
    })
}

fn phi_input_from(
    block: &BasicBlock,
    phi_value: ValueId,
    predecessor: BasicBlockId,
) -> Option<ValueId> {
    block.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Phi { dst, inputs, .. } if *dst == phi_value => inputs
            .iter()
            .find_map(|(block, value)| (*block == predecessor).then_some(*value)),
        _ => None,
    })
}

fn is_add_const_one_from(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &BasicBlock,
    next_value: ValueId,
    source_value: ValueId,
) -> bool {
    let next_value = root(function, def_map, next_value);
    block.instructions.iter().any(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } if *dst == next_value => {
            (root(function, def_map, *lhs) == source_value
                && const_i64(function, def_map, *rhs) == Some(1))
                || (root(function, def_map, *rhs) == source_value
                    && const_i64(function, def_map, *lhs) == Some(1))
        }
        _ => false,
    })
}
