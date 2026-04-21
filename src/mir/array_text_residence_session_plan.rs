/*!
 * MIR-owned array/text residence session plans.
 *
 * H25 keeps this metadata-only: it proves where a future backend may hold an
 * array text residence session, but it does not change lowering or runtime
 * behavior. Runtime remains executor-only; legality lives here.
 */

use std::collections::HashSet;

use super::{
    array_text_loopcarry_plan::ArrayTextLoopCarryLenStoreRoute, build_value_def_map,
    resolve_value_origin, BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction,
    MirInstruction, MirModule, ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceSessionScope {
    LoopBackedgeSingleBody,
}

impl std::fmt::Display for ArrayTextResidenceSessionScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoopBackedgeSingleBody => f.write_str("loop_backedge_single_body"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceSessionProof {
    LoopcarryLenStoreOnly,
}

impl std::fmt::Display for ArrayTextResidenceSessionProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoopcarryLenStoreOnly => f.write_str("loopcarry_len_store_only"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceSessionBeginPlacement {
    BeforePreheaderJump,
}

impl std::fmt::Display for ArrayTextResidenceSessionBeginPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BeforePreheaderJump => f.write_str("before_preheader_jump"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceSessionUpdatePlacement {
    RouteInstruction,
}

impl std::fmt::Display for ArrayTextResidenceSessionUpdatePlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RouteInstruction => f.write_str("route_instruction"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceSessionEndPlacement {
    ExitBlockEntry,
}

impl std::fmt::Display for ArrayTextResidenceSessionEndPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExitBlockEntry => f.write_str("exit_block_entry"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceExecutorExecutionMode {
    SingleRegionExecutor,
}

impl std::fmt::Display for ArrayTextResidenceExecutorExecutionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SingleRegionExecutor => f.write_str("single_region_executor"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceExecutorCarrier {
    ArrayLaneTextCell,
}

impl std::fmt::Display for ArrayTextResidenceExecutorCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArrayLaneTextCell => f.write_str("array_lane_text_cell"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceExecutorEffect {
    StoreCell,
    LengthOnlyResultCarry,
}

impl std::fmt::Display for ArrayTextResidenceExecutorEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StoreCell => f.write_str("store.cell"),
            Self::LengthOnlyResultCarry => f.write_str("length_only_result_carry"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceExecutorConsumerCapability {
    SinkStore,
    LengthOnly,
}

impl std::fmt::Display for ArrayTextResidenceExecutorConsumerCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SinkStore => f.write_str("sink_store"),
            Self::LengthOnly => f.write_str("length_only"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextResidenceExecutorMaterializationPolicy {
    TextResidentOrStringlikeSlot,
}

impl std::fmt::Display for ArrayTextResidenceExecutorMaterializationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextResidentOrStringlikeSlot => f.write_str("text_resident_or_stringlike_slot"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextResidenceLoopRegionMapping {
    pub loop_index_phi_value: ValueId,
    pub loop_index_initial_value: ValueId,
    pub loop_index_next_value: ValueId,
    pub loop_bound_value: ValueId,
    pub loop_bound_const: i64,
    pub accumulator_phi_value: ValueId,
    pub accumulator_initial_value: ValueId,
    pub accumulator_next_value: ValueId,
    pub exit_accumulator_value: ValueId,
    pub row_index_value: ValueId,
    pub row_modulus_value: ValueId,
    pub row_modulus_const: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextResidenceExecutorContract {
    pub execution_mode: ArrayTextResidenceExecutorExecutionMode,
    pub proof_region: ArrayTextResidenceSessionScope,
    pub publication_boundary: &'static str,
    pub carrier: ArrayTextResidenceExecutorCarrier,
    pub effects: Vec<ArrayTextResidenceExecutorEffect>,
    pub consumer_capabilities: Vec<ArrayTextResidenceExecutorConsumerCapability>,
    pub materialization_policy: ArrayTextResidenceExecutorMaterializationPolicy,
    pub region_mapping: Option<ArrayTextResidenceLoopRegionMapping>,
}

impl ArrayTextResidenceExecutorContract {
    fn loopcarry_len_store_single_region(
        region_mapping: ArrayTextResidenceLoopRegionMapping,
    ) -> Self {
        Self {
            execution_mode: ArrayTextResidenceExecutorExecutionMode::SingleRegionExecutor,
            proof_region: ArrayTextResidenceSessionScope::LoopBackedgeSingleBody,
            publication_boundary: "none",
            carrier: ArrayTextResidenceExecutorCarrier::ArrayLaneTextCell,
            effects: vec![
                ArrayTextResidenceExecutorEffect::StoreCell,
                ArrayTextResidenceExecutorEffect::LengthOnlyResultCarry,
            ],
            consumer_capabilities: vec![
                ArrayTextResidenceExecutorConsumerCapability::SinkStore,
                ArrayTextResidenceExecutorConsumerCapability::LengthOnly,
            ],
            materialization_policy:
                ArrayTextResidenceExecutorMaterializationPolicy::TextResidentOrStringlikeSlot,
            region_mapping: Some(region_mapping),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextResidenceSessionRoute {
    pub begin_block: BasicBlockId,
    pub begin_to_header_block: BasicBlockId,
    pub begin_placement: ArrayTextResidenceSessionBeginPlacement,
    pub header_block: BasicBlockId,
    pub body_block: BasicBlockId,
    pub exit_block: BasicBlockId,
    pub update_block: BasicBlockId,
    pub update_instruction_index: usize,
    pub update_placement: ArrayTextResidenceSessionUpdatePlacement,
    pub end_block: BasicBlockId,
    pub end_placement: ArrayTextResidenceSessionEndPlacement,
    pub route_instruction_index: usize,
    pub array_value: ValueId,
    pub index_value: ValueId,
    pub source_value: ValueId,
    pub result_len_value: ValueId,
    pub middle_value: ValueId,
    pub middle_length: i64,
    pub skip_instruction_indices: Vec<usize>,
    pub scope: ArrayTextResidenceSessionScope,
    pub proof: ArrayTextResidenceSessionProof,
    pub executor_contract: Option<ArrayTextResidenceExecutorContract>,
}

pub fn refresh_module_array_text_residence_session_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_text_residence_session_routes(function);
    }
}

pub fn refresh_function_array_text_residence_session_routes(function: &mut MirFunction) {
    let routes = function
        .metadata
        .array_text_loopcarry_len_store_routes
        .iter()
        .filter_map(|route| derive_loopcarry_session(function, route))
        .collect();
    function.metadata.array_text_residence_sessions = routes;
}

fn derive_loopcarry_session(
    function: &MirFunction,
    route: &ArrayTextLoopCarryLenStoreRoute,
) -> Option<ArrayTextResidenceSessionRoute> {
    if count_routes_in_body(function, route.block) != 1 {
        return None;
    }

    let body = function.blocks.get(&route.block)?;
    let header_block = match body.terminator.as_ref()? {
        MirInstruction::Jump { target, .. } => *target,
        _ => return None,
    };

    let header = function.blocks.get(&header_block)?;
    if !header.predecessors.contains(&route.block) {
        return None;
    }
    let exit_block = match header.terminator.as_ref()? {
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *then_bb == route.block => *else_bb,
        MirInstruction::Branch {
            then_bb, else_bb, ..
        } if *else_bb == route.block => *then_bb,
        _ => return None,
    };

    let begin_block = single_preheader_jump_to_header(function, header_block, route.block)?;
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
        route.block,
        exit_block,
    )?;

    Some(ArrayTextResidenceSessionRoute {
        begin_block,
        begin_to_header_block: header_block,
        begin_placement: ArrayTextResidenceSessionBeginPlacement::BeforePreheaderJump,
        header_block,
        body_block: route.block,
        exit_block,
        update_block: route.block,
        update_instruction_index: route.instruction_index,
        update_placement: ArrayTextResidenceSessionUpdatePlacement::RouteInstruction,
        end_block: exit_block,
        end_placement: ArrayTextResidenceSessionEndPlacement::ExitBlockEntry,
        route_instruction_index: route.instruction_index,
        array_value: route.array_value,
        index_value: route.index_value,
        source_value: route.source_value,
        result_len_value: route.result_len_value,
        middle_value: route.middle_value,
        middle_length: route.middle_length,
        skip_instruction_indices: route.skip_instruction_indices.clone(),
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
        match_row_modulus(function, def_map, route.index_value, loop_index_phi_value)?;
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
    if !block_uses_root(function, def_map, exit, accumulator_phi_value) {
        return None;
    }

    Some(ArrayTextResidenceLoopRegionMapping {
        loop_index_phi_value,
        loop_index_initial_value,
        loop_index_next_value,
        loop_bound_value,
        loop_bound_const,
        accumulator_phi_value,
        accumulator_initial_value,
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
    header: &super::BasicBlock,
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

fn is_phi_dst(block: &super::BasicBlock, value: ValueId) -> bool {
    block.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::Phi { dst, .. } if *dst == value
        )
    })
}

fn phi_input_from(
    block: &super::BasicBlock,
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
    body: &super::BasicBlock,
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
    header: &super::BasicBlock,
    body: &super::BasicBlock,
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
        is_accumulator_next_value(function, def_map, body, next, *dst, route.result_len_value)
            .then_some((*dst, initial, next))
    })
}

fn is_accumulator_next_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &super::BasicBlock,
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
    block: &super::BasicBlock,
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
        .filter(|route| route.block == body_block)
        .count()
}

fn body_has_only_covered_route_and_pure_loop_bookkeeping(
    body: &super::BasicBlock,
    route: &ArrayTextLoopCarryLenStoreRoute,
) -> bool {
    let covered: HashSet<usize> = std::iter::once(route.instruction_index)
        .chain(route.skip_instruction_indices.iter().copied())
        .collect();

    body.instructions
        .iter()
        .enumerate()
        .all(|(index, inst)| covered.contains(&index) || is_session_safe_bookkeeping(inst))
}

fn block_has_only_session_safe_lifetime_bookkeeping(block: &super::BasicBlock) -> bool {
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
