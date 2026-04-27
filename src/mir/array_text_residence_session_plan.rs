/*!
 * MIR-owned array/text residence session plans.
 *
 * H25 keeps this metadata-only: it proves where a future backend may hold an
 * array text residence session, but it does not change lowering or runtime
 * behavior. Runtime remains executor-only; legality lives here.
 */

use std::collections::HashSet;

use super::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use super::{
    array_text_loopcarry_plan::ArrayTextLoopCarryLenStoreRoute, BasicBlockId, BinaryOp, CompareOp,
    ConstValue, MirFunction, MirInstruction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextResidenceSessionScope {
    LoopBackedgeSingleBody,
}

impl ArrayTextResidenceSessionScope {
    fn as_str(self) -> &'static str {
        match self {
            Self::LoopBackedgeSingleBody => "loop_backedge_single_body",
        }
    }
}

impl std::fmt::Display for ArrayTextResidenceSessionScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextResidenceSessionProof {
    LoopcarryLenStoreOnly,
}

impl ArrayTextResidenceSessionProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::LoopcarryLenStoreOnly => "loopcarry_len_store_only",
        }
    }
}

impl std::fmt::Display for ArrayTextResidenceSessionProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextResidenceSessionBeginPlacement {
    BeforePreheaderJump,
}

impl ArrayTextResidenceSessionBeginPlacement {
    fn as_str(self) -> &'static str {
        match self {
            Self::BeforePreheaderJump => "before_preheader_jump",
        }
    }
}

impl std::fmt::Display for ArrayTextResidenceSessionBeginPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextResidenceSessionUpdatePlacement {
    RouteInstruction,
}

impl ArrayTextResidenceSessionUpdatePlacement {
    fn as_str(self) -> &'static str {
        match self {
            Self::RouteInstruction => "route_instruction",
        }
    }
}

impl std::fmt::Display for ArrayTextResidenceSessionUpdatePlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextResidenceSessionEndPlacement {
    ExitBlockEntry,
}

impl ArrayTextResidenceSessionEndPlacement {
    fn as_str(self) -> &'static str {
        match self {
            Self::ExitBlockEntry => "exit_block_entry",
        }
    }
}

impl std::fmt::Display for ArrayTextResidenceSessionEndPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextResidenceExecutorExecutionMode {
    SingleRegionExecutor,
}

impl std::fmt::Display for ArrayTextResidenceExecutorExecutionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextResidenceExecutorExecutionMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::SingleRegionExecutor => "single_region_executor",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextResidenceExecutorCarrier {
    ArrayLaneTextCell,
}

impl std::fmt::Display for ArrayTextResidenceExecutorCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextResidenceExecutorCarrier {
    fn as_str(self) -> &'static str {
        match self {
            Self::ArrayLaneTextCell => "array_lane_text_cell",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextResidenceExecutorEffect {
    StoreCell,
    LengthOnlyResultCarry,
}

impl std::fmt::Display for ArrayTextResidenceExecutorEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextResidenceExecutorEffect {
    fn as_str(self) -> &'static str {
        match self {
            Self::StoreCell => "store.cell",
            Self::LengthOnlyResultCarry => "length_only_result_carry",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextResidenceExecutorConsumerCapability {
    SinkStore,
    LengthOnly,
}

impl std::fmt::Display for ArrayTextResidenceExecutorConsumerCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextResidenceExecutorConsumerCapability {
    fn as_str(self) -> &'static str {
        match self {
            Self::SinkStore => "sink_store",
            Self::LengthOnly => "length_only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextResidenceExecutorMaterializationPolicy {
    TextResidentOrStringlikeSlot,
}

impl std::fmt::Display for ArrayTextResidenceExecutorMaterializationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextResidenceExecutorMaterializationPolicy {
    fn as_str(self) -> &'static str {
        match self {
            Self::TextResidentOrStringlikeSlot => "text_resident_or_stringlike_slot",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextResidenceLoopRegionMapping {
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

impl ArrayTextResidenceLoopRegionMapping {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextResidenceExecutorContract {
    execution_mode: ArrayTextResidenceExecutorExecutionMode,
    proof_region: ArrayTextResidenceSessionScope,
    publication_boundary: &'static str,
    carrier: ArrayTextResidenceExecutorCarrier,
    effects: Vec<ArrayTextResidenceExecutorEffect>,
    consumer_capabilities: Vec<ArrayTextResidenceExecutorConsumerCapability>,
    materialization_policy: ArrayTextResidenceExecutorMaterializationPolicy,
    region_mapping: Option<ArrayTextResidenceLoopRegionMapping>,
}

impl ArrayTextResidenceExecutorContract {
    pub fn execution_mode(&self) -> &'static str {
        self.execution_mode.as_str()
    }

    pub fn proof_region(&self) -> &'static str {
        self.proof_region.as_str()
    }

    pub fn publication_boundary(&self) -> &'static str {
        self.publication_boundary
    }

    pub fn carrier(&self) -> &'static str {
        self.carrier.as_str()
    }

    pub fn effects(&self) -> Vec<&'static str> {
        self.effects.iter().map(|effect| effect.as_str()).collect()
    }

    pub fn consumer_capabilities(&self) -> Vec<&'static str> {
        self.consumer_capabilities
            .iter()
            .map(|capability| capability.as_str())
            .collect()
    }

    pub fn materialization_policy(&self) -> &'static str {
        self.materialization_policy.as_str()
    }

    pub fn region_mapping(&self) -> Option<&ArrayTextResidenceLoopRegionMapping> {
        self.region_mapping.as_ref()
    }

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
    begin_block: BasicBlockId,
    begin_to_header_block: BasicBlockId,
    begin_placement: ArrayTextResidenceSessionBeginPlacement,
    header_block: BasicBlockId,
    body_block: BasicBlockId,
    exit_block: BasicBlockId,
    update_block: BasicBlockId,
    update_instruction_index: usize,
    update_placement: ArrayTextResidenceSessionUpdatePlacement,
    end_block: BasicBlockId,
    end_placement: ArrayTextResidenceSessionEndPlacement,
    route_instruction_index: usize,
    array_value: ValueId,
    index_value: ValueId,
    source_value: ValueId,
    result_len_value: ValueId,
    middle_value: ValueId,
    middle_length: i64,
    skip_instruction_indices: Vec<usize>,
    scope: ArrayTextResidenceSessionScope,
    proof: ArrayTextResidenceSessionProof,
    executor_contract: Option<ArrayTextResidenceExecutorContract>,
}

impl ArrayTextResidenceSessionRoute {
    pub fn begin_block(&self) -> BasicBlockId {
        self.begin_block
    }

    pub fn begin_to_header_block(&self) -> BasicBlockId {
        self.begin_to_header_block
    }

    pub fn begin_placement(&self) -> &'static str {
        self.begin_placement.as_str()
    }

    pub fn header_block(&self) -> BasicBlockId {
        self.header_block
    }

    pub fn body_block(&self) -> BasicBlockId {
        self.body_block
    }

    pub fn exit_block(&self) -> BasicBlockId {
        self.exit_block
    }

    pub fn update_block(&self) -> BasicBlockId {
        self.update_block
    }

    pub fn update_instruction_index(&self) -> usize {
        self.update_instruction_index
    }

    pub fn update_placement(&self) -> &'static str {
        self.update_placement.as_str()
    }

    pub fn end_block(&self) -> BasicBlockId {
        self.end_block
    }

    pub fn end_placement(&self) -> &'static str {
        self.end_placement.as_str()
    }

    pub fn route_instruction_index(&self) -> usize {
        self.route_instruction_index
    }

    pub fn array_value(&self) -> ValueId {
        self.array_value
    }

    pub fn index_value(&self) -> ValueId {
        self.index_value
    }

    pub fn source_value(&self) -> ValueId {
        self.source_value
    }

    pub fn result_len_value(&self) -> ValueId {
        self.result_len_value
    }

    pub fn middle_value(&self) -> ValueId {
        self.middle_value
    }

    pub fn middle_length(&self) -> i64 {
        self.middle_length
    }

    pub fn skip_instruction_indices(&self) -> &[usize] {
        &self.skip_instruction_indices
    }

    pub fn scope(&self) -> &'static str {
        self.scope.as_str()
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }

    pub fn executor_contract(&self) -> Option<&ArrayTextResidenceExecutorContract> {
        self.executor_contract.as_ref()
    }
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
        .filter(|route| route.block() == body_block)
        .count()
}

fn body_has_only_covered_route_and_pure_loop_bookkeeping(
    body: &super::BasicBlock,
    route: &ArrayTextLoopCarryLenStoreRoute,
) -> bool {
    let covered: HashSet<usize> = route.covered_instruction_indices().collect();

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
