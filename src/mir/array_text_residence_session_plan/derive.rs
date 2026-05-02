use std::collections::HashSet;

use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{
    array_text_loopcarry_plan::ArrayTextLoopCarryLenStoreRoute, BasicBlock, BasicBlockId, BinaryOp,
    CompareOp, ConstValue, MirFunction, MirInstruction, ValueId,
};

use super::{
    ArrayTextResidenceExecutorContract, ArrayTextResidenceLoopRegionMapping,
    ArrayTextResidenceSessionBeginPlacement, ArrayTextResidenceSessionEndPlacement,
    ArrayTextResidenceSessionProof, ArrayTextResidenceSessionRoute, ArrayTextResidenceSessionScope,
    ArrayTextResidenceSessionUpdatePlacement,
};

pub(super) fn derive_loopcarry_session(
    function: &MirFunction,
    route: &ArrayTextLoopCarryLenStoreRoute,
) -> Option<ArrayTextResidenceSessionRoute> {
    if count_routes_in_body(function, route.block()) != 1 {
        return None;
    }

    let body = function.blocks.get(&route.block())?;
    let header_block = match body.terminator.as_ref()? {
        MirInstruction::Jump { target, .. } => *target,
        _ => return None,
    };

    let header = function.blocks.get(&header_block)?;
    if !header.predecessors.contains(&route.block()) {
        return None;
    }
    let exit_block = match header.terminator.as_ref()? {
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *then_bb == route.block() => *else_bb,
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *else_bb == route.block() => *then_bb,
        _ => return None,
    };

    let begin_block = single_preheader_jump_to_header(function, header_block, route.block())?;
    let exit = function.blocks.get(&exit_block)?;
    if exit.predecessors.len() != 1 || !exit.predecessors.contains(&header_block) {
        return None;
    }

    if !block_has_only_session_safe_lifetime_bookkeeping(header) {
        return None;
    }
    if !body_has_only_covered_route_and_pure_loop_bookkeeping(body, route) {
        return None;
    }
    let def_map = build_value_def_map(function);
    let region_mapping = derive_loop_region_mapping(
        function,
        &def_map,
        route,
        begin_block,
        header_block,
        route.block(),
        exit_block,
    )?;

    Some(ArrayTextResidenceSessionRoute {
        begin_block,
        begin_to_header_block: header_block,
        begin_placement: ArrayTextResidenceSessionBeginPlacement::BeforePreheaderJump,
        header_block,
        body_block: route.block(),
        exit_block,
        update_block: route.block(),
        update_instruction_index: route.instruction_index(),
        update_placement: ArrayTextResidenceSessionUpdatePlacement::RouteInstruction,
        end_block: exit_block,
        end_placement: ArrayTextResidenceSessionEndPlacement::ExitBlockEntry,
        route_instruction_index: route.instruction_index(),
        array_value: route.array_value(),
        index_value: route.index_value(),
        source_value: route.source_value(),
        result_len_value: route.result_len_value(),
        middle_value: route.middle_value(),
        middle_length: route.middle_length(),
        skip_instruction_indices: route.skip_instruction_indices().to_vec(),
        scope: ArrayTextResidenceSessionScope::LoopBackedgeSingleBody,
        proof: ArrayTextResidenceSessionProof::LoopcarryLenStoreOnly,
        executor_contract: Some(
            ArrayTextResidenceExecutorContract::loopcarry_len_store_single_region(region_mapping),
        ),
    })
}

fn derive_loop_region_mapping(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route: &ArrayTextLoopCarryLenStoreRoute,
    begin_block: BasicBlockId,
    header_block: BasicBlockId,
    body_block: BasicBlockId,
    exit_block: BasicBlockId,
) -> Option<ArrayTextResidenceLoopRegionMapping> {
    let header = function.blocks.get(&header_block)?;
    let body = function.blocks.get(&body_block)?;
    let exit = function.blocks.get(&exit_block)?;
    let (loop_index_phi_value, loop_bound_value) =
        match_loop_index_condition(function, def_map, header)?;
    let loop_index_initial_value = phi_input_from(header, loop_index_phi_value, begin_block)?;
    let loop_index_initial_const = const_i64(function, def_map, loop_index_initial_value)?;
    if loop_index_initial_const != 0 {
        return None;
    }
    let loop_index_next_value = phi_input_from(header, loop_index_phi_value, body_block)?;
    if !is_add_const_one_from(
        function,
        def_map,
        body,
        loop_index_next_value,
        loop_index_phi_value,
    ) {
        return None;
    }

    let loop_bound_const = const_i64(function, def_map, loop_bound_value)?;
    let (row_index_value, row_modulus_value, row_modulus_const) =
        match_row_modulus(function, def_map, route.index_value(), loop_index_phi_value)?;
    let (accumulator_phi_value, accumulator_initial_value, accumulator_next_value) =
        match_accumulator_phi(
            function,
            def_map,
            header,
            body,
            begin_block,
            body_block,
            route,
        )?;
    let accumulator_initial_const = const_i64(function, def_map, accumulator_initial_value)?;
    if accumulator_initial_const != 0 {
        return None;
    }
    if !block_uses_root(function, def_map, exit, accumulator_phi_value) {
        return None;
    }

    Some(ArrayTextResidenceLoopRegionMapping {
        array_root_value: root(function, def_map, route.array_value()),
        loop_index_phi_value,
        loop_index_initial_value,
        loop_index_initial_const,
        loop_index_next_value,
        loop_bound_value,
        loop_bound_const,
        accumulator_phi_value,
        accumulator_initial_value,
        accumulator_initial_const,
        accumulator_next_value,
        exit_accumulator_value: accumulator_phi_value,
        row_index_value,
        row_modulus_value,
        row_modulus_const,
    })
}

fn root(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> ValueId {
    resolve_value_origin(function, def_map, value)
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

fn match_loop_index_condition(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &BasicBlock,
) -> Option<(ValueId, ValueId)> {
    let condition = match header.terminator.as_ref()? {
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
    body: &BasicBlock,
    next_value: ValueId,
    source_value: ValueId,
) -> bool {
    let next_value = root(function, def_map, next_value);
    body.instructions.iter().any(|inst| match inst {
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

fn match_row_modulus(
    function: &MirFunction,
    def_map: &ValueDefMap,
    index_value: ValueId,
    loop_index_phi_value: ValueId,
) -> Option<(ValueId, ValueId, i64)> {
    let row_index_value = root(function, def_map, index_value);
    let (block, index) = def_map.get(&row_index_value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Mod,
            lhs,
            rhs,
        } if *dst == row_index_value && root(function, def_map, *lhs) == loop_index_phi_value => {
            let row_modulus_value = root(function, def_map, *rhs);
            Some((
                row_index_value,
                row_modulus_value,
                const_i64(function, def_map, row_modulus_value)?,
            ))
        }
        _ => None,
    }
}

fn match_accumulator_phi(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &BasicBlock,
    body: &BasicBlock,
    begin_block: BasicBlockId,
    body_block: BasicBlockId,
    route: &ArrayTextLoopCarryLenStoreRoute,
) -> Option<(ValueId, ValueId, ValueId)> {
    header.instructions.iter().find_map(|inst| {
        let MirInstruction::Phi { dst, inputs, .. } = inst else {
            return None;
        };
        let initial = inputs
            .iter()
            .find_map(|(block, value)| (*block == begin_block).then_some(*value))?;
        let next = inputs
            .iter()
            .find_map(|(block, value)| (*block == body_block).then_some(*value))?;
        is_accumulator_next_value(
            function,
            def_map,
            body,
            next,
            *dst,
            route.result_len_value(),
        )
        .then_some((*dst, initial, next))
    })
}

fn is_accumulator_next_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &BasicBlock,
    next_value: ValueId,
    accumulator_phi_value: ValueId,
    result_len_value: ValueId,
) -> bool {
    let next_value = root(function, def_map, next_value);
    let result_len_value = root(function, def_map, result_len_value);
    body.instructions.iter().any(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } if *dst == next_value => {
            let lhs = root(function, def_map, *lhs);
            let rhs = root(function, def_map, *rhs);
            (lhs == accumulator_phi_value && rhs == result_len_value)
                || (rhs == accumulator_phi_value && lhs == result_len_value)
        }
        _ => false,
    })
}

fn block_uses_root(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &BasicBlock,
    value: ValueId,
) -> bool {
    block.instructions.iter().any(|inst| {
        inst.used_values()
            .into_iter()
            .any(|used| root(function, def_map, used) == value)
    }) || block.terminator.as_ref().is_some_and(|inst| {
        inst.used_values()
            .into_iter()
            .any(|used| root(function, def_map, used) == value)
    })
}

fn single_preheader_jump_to_header(
    function: &MirFunction,
    header_block: BasicBlockId,
    latch_block: BasicBlockId,
) -> Option<BasicBlockId> {
    let header = function.blocks.get(&header_block)?;
    let mut non_latch_predecessors = header
        .predecessors
        .iter()
        .copied()
        .filter(|predecessor| *predecessor != latch_block);
    let preheader = non_latch_predecessors.next()?;
    if non_latch_predecessors.next().is_some() {
        return None;
    }

    let preheader_block = function.blocks.get(&preheader)?;
    match preheader_block.terminator.as_ref()? {
        MirInstruction::Jump { target, .. } if *target == header_block => Some(preheader),
        _ => None,
    }
}

fn count_routes_in_body(function: &MirFunction, body_block: BasicBlockId) -> usize {
    function
        .metadata
        .array_text_loopcarry_len_store_routes
        .iter()
        .filter(|route| route.block() == body_block)
        .count()
}

fn body_has_only_covered_route_and_pure_loop_bookkeeping(
    body: &BasicBlock,
    route: &ArrayTextLoopCarryLenStoreRoute,
) -> bool {
    let covered: HashSet<usize> = route.covered_instruction_indices().collect();

    body.instructions
        .iter()
        .enumerate()
        .all(|(index, inst)| covered.contains(&index) || is_session_safe_bookkeeping(inst))
}

fn block_has_only_session_safe_lifetime_bookkeeping(block: &BasicBlock) -> bool {
    block
        .instructions
        .iter()
        .all(is_session_safe_lifetime_bookkeeping)
}

fn is_session_safe_bookkeeping(inst: &MirInstruction) -> bool {
    matches!(
        inst,
        MirInstruction::Const { .. }
            | MirInstruction::Copy { .. }
            | MirInstruction::BinOp { .. }
            | MirInstruction::Compare { .. }
    )
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
