use crate::mir::array_receiver_proof::value_root;
use crate::mir::array_text_observer_plan::ArrayTextObserverRoute;
use crate::mir::value_origin::ValueDefMap;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, CompareOp, MirFunction, MirInstruction, ValueId,
};

use super::region_payload::{
    block_uses_root, const_i64, is_add_const_one_from, match_loop_index_condition,
    match_row_modulus, phi_input_from, single_preheader_jump_to_header,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextIndexOfConstRegionPlan {
    loop_header: BasicBlockId,
    loop_body: BasicBlockId,
    loop_exit: BasicBlockId,
    array_value: ValueId,
    index_value: ValueId,
    get_instruction_index: usize,
    observer_instruction_index: usize,
    observer_arg0_value: ValueId,
    needle_const_text: String,
    needle_byte_len: usize,
    consumer_shape: &'static str,
    selected_helper_symbol: &'static str,
    region_payload: ArrayTextIndexOfConstRegionPayload,
}

impl ArrayTextIndexOfConstRegionPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        loop_header: BasicBlockId,
        loop_body: BasicBlockId,
        loop_exit: BasicBlockId,
        array_value: ValueId,
        index_value: ValueId,
        get_instruction_index: usize,
        observer_instruction_index: usize,
        observer_arg0_value: ValueId,
        needle_const_text: String,
        needle_byte_len: usize,
        consumer_shape: &'static str,
        selected_helper_symbol: &'static str,
        region_payload: ArrayTextIndexOfConstRegionPayload,
    ) -> Self {
        Self {
            loop_header,
            loop_body,
            loop_exit,
            array_value,
            index_value,
            get_instruction_index,
            observer_instruction_index,
            observer_arg0_value,
            needle_const_text,
            needle_byte_len,
            consumer_shape,
            selected_helper_symbol,
            region_payload,
        }
    }

    pub fn loop_header(&self) -> BasicBlockId {
        self.loop_header
    }

    pub fn loop_body(&self) -> BasicBlockId {
        self.loop_body
    }

    pub fn loop_exit(&self) -> BasicBlockId {
        self.loop_exit
    }

    pub fn array_value(&self) -> ValueId {
        self.array_value
    }

    pub fn index_value(&self) -> ValueId {
        self.index_value
    }

    pub fn get_instruction_index(&self) -> usize {
        self.get_instruction_index
    }

    pub fn observer_instruction_index(&self) -> usize {
        self.observer_instruction_index
    }

    pub fn observer_arg0_value(&self) -> ValueId {
        self.observer_arg0_value
    }

    pub fn needle_const_text(&self) -> &str {
        self.needle_const_text.as_str()
    }

    pub fn needle_byte_len(&self) -> usize {
        self.needle_byte_len
    }

    pub fn consumer_shape(&self) -> &'static str {
        self.consumer_shape
    }

    pub fn selected_helper_symbol(&self) -> &'static str {
        self.selected_helper_symbol
    }

    pub fn region_payload(&self) -> &ArrayTextIndexOfConstRegionPayload {
        &self.region_payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextIndexOfConstRegionPayload {
    array_root_value: ValueId,
    loop_index_phi_value: ValueId,
    loop_index_initial_value: ValueId,
    loop_index_initial_const: i64,
    loop_index_next_value: ValueId,
    loop_bound_value: ValueId,
    loop_bound_const: i64,
    accumulator_phi_value: ValueId,
    accumulator_initial_value: ValueId,
    accumulator_initial_const: i64,
    accumulator_next_value: ValueId,
    exit_accumulator_value: ValueId,
    row_index_value: ValueId,
    row_modulus_value: ValueId,
    row_modulus_const: i64,
    get_result_value: ValueId,
    indexof_result_value: ValueId,
    predicate_value: ValueId,
}

impl ArrayTextIndexOfConstRegionPayload {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        array_root_value: ValueId,
        loop_index_phi_value: ValueId,
        loop_index_initial_value: ValueId,
        loop_index_initial_const: i64,
        loop_index_next_value: ValueId,
        loop_bound_value: ValueId,
        loop_bound_const: i64,
        accumulator_phi_value: ValueId,
        accumulator_initial_value: ValueId,
        accumulator_initial_const: i64,
        accumulator_next_value: ValueId,
        exit_accumulator_value: ValueId,
        row_index_value: ValueId,
        row_modulus_value: ValueId,
        row_modulus_const: i64,
        get_result_value: ValueId,
        indexof_result_value: ValueId,
        predicate_value: ValueId,
    ) -> Self {
        Self {
            array_root_value,
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
            exit_accumulator_value,
            row_index_value,
            row_modulus_value,
            row_modulus_const,
            get_result_value,
            indexof_result_value,
            predicate_value,
        }
    }

    pub fn array_root_value(&self) -> ValueId {
        self.array_root_value
    }

    pub fn loop_index_phi_value(&self) -> ValueId {
        self.loop_index_phi_value
    }

    pub fn loop_index_initial_value(&self) -> ValueId {
        self.loop_index_initial_value
    }

    pub fn loop_index_initial_const(&self) -> i64 {
        self.loop_index_initial_const
    }

    pub fn loop_index_next_value(&self) -> ValueId {
        self.loop_index_next_value
    }

    pub fn loop_bound_value(&self) -> ValueId {
        self.loop_bound_value
    }

    pub fn loop_bound_const(&self) -> i64 {
        self.loop_bound_const
    }

    pub fn accumulator_phi_value(&self) -> ValueId {
        self.accumulator_phi_value
    }

    pub fn accumulator_initial_value(&self) -> ValueId {
        self.accumulator_initial_value
    }

    pub fn accumulator_initial_const(&self) -> i64 {
        self.accumulator_initial_const
    }

    pub fn accumulator_next_value(&self) -> ValueId {
        self.accumulator_next_value
    }

    pub fn exit_accumulator_value(&self) -> ValueId {
        self.exit_accumulator_value
    }

    pub fn row_index_value(&self) -> ValueId {
        self.row_index_value
    }

    pub fn row_modulus_value(&self) -> ValueId {
        self.row_modulus_value
    }

    pub fn row_modulus_const(&self) -> i64 {
        self.row_modulus_const
    }

    pub fn get_result_value(&self) -> ValueId {
        self.get_result_value
    }

    pub fn indexof_result_value(&self) -> ValueId {
        self.indexof_result_value
    }

    pub fn predicate_value(&self) -> ValueId {
        self.predicate_value
    }
}

pub(super) fn derive_indexof_const_region_payload(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route: &ArrayTextObserverRoute,
    loop_header: BasicBlockId,
    body_block: BasicBlockId,
    loop_exit: BasicBlockId,
) -> Option<ArrayTextIndexOfConstRegionPayload> {
    let header = function.blocks.get(&loop_header)?;
    let body = function.blocks.get(&body_block)?;
    let exit = function.blocks.get(&loop_exit)?;
    let preheader = single_preheader_jump_to_header(function, loop_header, body_block)?;

    let (loop_index_phi_value, loop_bound_value) =
        match_loop_index_condition(function, def_map, header)?;
    let loop_index_initial_value = phi_input_from(header, loop_index_phi_value, preheader)?;
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
    let (accumulator_phi_value, accumulator_initial_value, accumulator_next_value, predicate_value) =
        match_found_predicate_accumulator_phi(
            function,
            def_map,
            header,
            body,
            preheader,
            body_block,
            route.result_value(),
        )?;
    let accumulator_initial_const = const_i64(function, def_map, accumulator_initial_value)?;
    if accumulator_initial_const != 0 {
        return None;
    }
    let exit_accumulator_value = exit_phi_dst_for_input(exit, loop_header, accumulator_phi_value)
        .unwrap_or(accumulator_phi_value);
    if !block_uses_root(function, def_map, exit, exit_accumulator_value) {
        return None;
    }

    Some(ArrayTextIndexOfConstRegionPayload::new(
        value_root(function, def_map, route.array_value()),
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
        exit_accumulator_value,
        row_index_value,
        row_modulus_value,
        row_modulus_const,
        route.source_value(),
        route.result_value(),
        predicate_value,
    ))
}

#[allow(clippy::too_many_arguments)]
fn match_found_predicate_accumulator_phi(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &BasicBlock,
    body: &BasicBlock,
    preheader: BasicBlockId,
    body_block: BasicBlockId,
    indexof_result_value: ValueId,
) -> Option<(ValueId, ValueId, ValueId, ValueId)> {
    header.instructions.iter().find_map(|inst| {
        let MirInstruction::Phi { dst, inputs, .. } = inst else {
            return None;
        };
        let initial = inputs
            .iter()
            .find_map(|(block, value)| (*block == preheader).then_some(*value))?;
        let next = inputs
            .iter()
            .find_map(|(block, value)| (*block == body_block).then_some(*value))?;
        let predicate = match_found_predicate_select(
            function,
            def_map,
            body,
            next,
            *dst,
            indexof_result_value,
        )?;
        Some((*dst, initial, next, predicate))
    })
}

fn match_found_predicate_select(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &BasicBlock,
    next_value: ValueId,
    accumulator_phi_value: ValueId,
    indexof_result_value: ValueId,
) -> Option<ValueId> {
    let next_value = value_root(function, def_map, next_value);
    body.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Select {
            dst,
            cond,
            then_val,
            else_val,
        } if *dst == next_value => {
            let predicate = value_root(function, def_map, *cond);
            if !is_indexof_ge_zero_predicate(
                function,
                def_map,
                body,
                predicate,
                indexof_result_value,
            ) {
                return None;
            }
            if value_root(function, def_map, *else_val) != accumulator_phi_value {
                return None;
            }
            is_increment_by_one_from(function, def_map, body, *then_val, accumulator_phi_value)
                .then_some(predicate)
        }
        _ => None,
    })
}

fn is_indexof_ge_zero_predicate(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &BasicBlock,
    predicate_value: ValueId,
    indexof_result_value: ValueId,
) -> bool {
    let predicate_value = value_root(function, def_map, predicate_value);
    let indexof_result_value = value_root(function, def_map, indexof_result_value);
    body.instructions.iter().any(|inst| match inst {
        MirInstruction::Compare {
            dst,
            op: CompareOp::Ge,
            lhs,
            rhs,
        } if *dst == predicate_value => {
            value_root(function, def_map, *lhs) == indexof_result_value
                && const_i64(function, def_map, *rhs) == Some(0)
        }
        _ => false,
    })
}

fn is_increment_by_one_from(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &BasicBlock,
    increment_value: ValueId,
    accumulator_phi_value: ValueId,
) -> bool {
    let increment_value = value_root(function, def_map, increment_value);
    body.instructions.iter().any(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } if *dst == increment_value => {
            (value_root(function, def_map, *lhs) == accumulator_phi_value
                && const_i64(function, def_map, *rhs) == Some(1))
                || (value_root(function, def_map, *rhs) == accumulator_phi_value
                    && const_i64(function, def_map, *lhs) == Some(1))
        }
        _ => false,
    })
}

fn exit_phi_dst_for_input(
    exit: &BasicBlock,
    predecessor: BasicBlockId,
    predecessor_value: ValueId,
) -> Option<ValueId> {
    exit.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Phi { dst, inputs, .. } => inputs
            .iter()
            .any(|(block, value)| *block == predecessor && *value == predecessor_value)
            .then_some(*dst),
        _ => None,
    })
}
