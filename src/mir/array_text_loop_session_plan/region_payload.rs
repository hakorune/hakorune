use crate::mir::array_receiver_proof::value_root;
use crate::mir::array_string_len_window_plan::ArrayStringLenWindowRoute;
use crate::mir::value_origin::ValueDefMap;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction, MirInstruction, ValueId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextLoopSessionRegionPayload {
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
}

impl ArrayTextLoopSessionRegionPayload {
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
}

pub(super) fn derive_region_payload(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route: &ArrayStringLenWindowRoute,
    loop_header: BasicBlockId,
    body_block: BasicBlockId,
    loop_exit: BasicBlockId,
) -> Option<ArrayTextLoopSessionRegionPayload> {
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
    let (accumulator_phi_value, accumulator_initial_value, accumulator_next_value) =
        match_accumulator_phi(
            function,
            def_map,
            header,
            body,
            preheader,
            body_block,
            route.len_value(),
        )?;
    let accumulator_initial_const = const_i64(function, def_map, accumulator_initial_value)?;
    if accumulator_initial_const != 0 {
        return None;
    }
    if !block_uses_root(function, def_map, exit, accumulator_phi_value) {
        return None;
    }

    Some(ArrayTextLoopSessionRegionPayload::new(
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
        accumulator_phi_value,
        row_index_value,
        row_modulus_value,
        row_modulus_const,
    ))
}

pub(super) fn single_preheader_jump_to_header(
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

pub(super) fn const_i64(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<i64> {
    let value = value_root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::Const {
            value: ConstValue::Integer(actual),
            ..
        } => Some(*actual),
        _ => None,
    }
}

pub(super) fn match_loop_index_condition(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &BasicBlock,
) -> Option<(ValueId, ValueId)> {
    let condition = match header.terminator.as_ref()? {
        MirInstruction::Branch { condition, .. } => *condition,
        _ => return None,
    };
    let condition = value_root(function, def_map, condition);
    let compare = header.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Compare {
            dst,
            op: CompareOp::Lt,
            lhs,
            rhs,
        } if value_root(function, def_map, *dst) == condition => Some((*lhs, *rhs)),
        _ => None,
    })?;
    let loop_index_phi_value = value_root(function, def_map, compare.0);
    if !is_phi_dst(header, loop_index_phi_value) {
        return None;
    }
    Some((
        loop_index_phi_value,
        value_root(function, def_map, compare.1),
    ))
}

fn is_phi_dst(block: &BasicBlock, value: ValueId) -> bool {
    block.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::Phi { dst, .. } if *dst == value
        )
    })
}

pub(super) fn phi_input_from(
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

pub(super) fn is_add_const_one_from(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &BasicBlock,
    next_value: ValueId,
    source_value: ValueId,
) -> bool {
    let next_value = value_root(function, def_map, next_value);
    body.instructions.iter().any(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } if *dst == next_value => {
            (value_root(function, def_map, *lhs) == source_value
                && const_i64(function, def_map, *rhs) == Some(1))
                || (value_root(function, def_map, *rhs) == source_value
                    && const_i64(function, def_map, *lhs) == Some(1))
        }
        _ => false,
    })
}

pub(super) fn match_row_modulus(
    function: &MirFunction,
    def_map: &ValueDefMap,
    index_value: ValueId,
    loop_index_phi_value: ValueId,
) -> Option<(ValueId, ValueId, i64)> {
    let row_index_value = value_root(function, def_map, index_value);
    let (block, index) = def_map.get(&row_index_value).copied()?;
    match function.blocks.get(&block)?.instructions.get(index)? {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Mod,
            lhs,
            rhs,
        } if *dst == row_index_value
            && value_root(function, def_map, *lhs) == loop_index_phi_value =>
        {
            let row_modulus_value = value_root(function, def_map, *rhs);
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
    preheader: BasicBlockId,
    body_block: BasicBlockId,
    len_value: ValueId,
) -> Option<(ValueId, ValueId, ValueId)> {
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
        is_accumulator_next_value(function, def_map, body, next, *dst, len_value)
            .then_some((*dst, initial, next))
    })
}

fn is_accumulator_next_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &BasicBlock,
    next_value: ValueId,
    accumulator_phi_value: ValueId,
    len_value: ValueId,
) -> bool {
    let next_value = value_root(function, def_map, next_value);
    let len_value = value_root(function, def_map, len_value);
    body.instructions.iter().any(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } if *dst == next_value => {
            let lhs = value_root(function, def_map, *lhs);
            let rhs = value_root(function, def_map, *rhs);
            (lhs == accumulator_phi_value && rhs == len_value)
                || (rhs == accumulator_phi_value && lhs == len_value)
        }
        _ => false,
    })
}

pub(super) fn block_uses_root(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &BasicBlock,
    value: ValueId,
) -> bool {
    block.instructions.iter().any(|inst| {
        inst.used_values()
            .into_iter()
            .any(|used| value_root(function, def_map, used) == value)
    }) || block.terminator.as_ref().is_some_and(|inst| {
        inst.used_values()
            .into_iter()
            .any(|used| value_root(function, def_map, used) == value)
    })
}
