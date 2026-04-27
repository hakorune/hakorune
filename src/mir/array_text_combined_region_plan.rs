/*!
 * MIR-owned array/text combined region routes.
 *
 * This module owns the H39.3 proof that combines the outer len-half edit with
 * an already-proven nested observer-store region. It is metadata-only: backend
 * shims may consume the route later, but must not rediscover the shape from raw
 * MIR JSON.
 */

use super::{
    array_text_edit_plan::ArrayTextEditRoute,
    array_text_observer_plan::ArrayTextObserverRoute,
    array_text_observer_region_contract::{
        ArrayTextObserverExecutorContract, ArrayTextObserverExecutorExecutionMode,
        ArrayTextObserverStoreRegionMapping,
    },
    build_value_def_map, resolve_value_origin, BasicBlock, BasicBlockId, BinaryOp, CompareOp,
    ConstValue, MirFunction, MirInstruction, MirModule, ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextCombinedRegionProof {
    OuterLenHalfEditWithPeriodicObserverStore,
}

impl ArrayTextCombinedRegionProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::OuterLenHalfEditWithPeriodicObserverStore => {
                "outer_lenhalf_edit_with_periodic_observer_store"
            }
        }
    }
}

impl std::fmt::Display for ArrayTextCombinedRegionProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextCombinedRegionExecutionMode {
    SingleRegionExecutor,
}

impl ArrayTextCombinedRegionExecutionMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::SingleRegionExecutor => "single_region_executor",
        }
    }
}

impl std::fmt::Display for ArrayTextCombinedRegionExecutionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextCombinedRegionProofRegion {
    OuterLoopWithPeriodicObserverStore,
}

impl ArrayTextCombinedRegionProofRegion {
    fn as_str(self) -> &'static str {
        match self {
            Self::OuterLoopWithPeriodicObserverStore => "outer_loop_with_periodic_observer_store",
        }
    }
}

impl std::fmt::Display for ArrayTextCombinedRegionProofRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextCombinedRegionEffect {
    LenHalfInsertMidStoreCell,
    ObserveIndexOf,
    ConstSuffixStoreCell,
    ScalarAccumulatorAddOne,
}

impl std::fmt::Display for ArrayTextCombinedRegionEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LenHalfInsertMidStoreCell => f.write_str("store.cell(lenhalf_insert_mid_const)"),
            Self::ObserveIndexOf => f.write_str("observe.indexof"),
            Self::ConstSuffixStoreCell => f.write_str("store.cell(const_suffix_append)"),
            Self::ScalarAccumulatorAddOne => f.write_str("scalar_accumulator(+1)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextCombinedRegionConsumerCapability {
    SinkStore,
    CompareOnly,
    LengthOnlyResultCarry,
}

impl std::fmt::Display for ArrayTextCombinedRegionConsumerCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SinkStore => f.write_str("sink_store"),
            Self::CompareOnly => f.write_str("compare_only"),
            Self::LengthOnlyResultCarry => f.write_str("length_only_result_carry"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextCombinedRegionByteBoundaryProof {
    AsciiPreservedTextCell,
}

impl ArrayTextCombinedRegionByteBoundaryProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::AsciiPreservedTextCell => "ascii_preserved_text_cell",
        }
    }
}

impl std::fmt::Display for ArrayTextCombinedRegionByteBoundaryProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextCombinedRegionRoute {
    begin_block: BasicBlockId,
    header_block: BasicBlockId,
    edit_block: BasicBlockId,
    observer_begin_block: BasicBlockId,
    observer_header_block: BasicBlockId,
    observer_block: BasicBlockId,
    observer_store_block: BasicBlockId,
    observer_latch_block: BasicBlockId,
    observer_exit_block: BasicBlockId,
    latch_block: BasicBlockId,
    exit_block: BasicBlockId,
    array_value: ValueId,
    outer_index_phi_value: ValueId,
    outer_index_initial_value: ValueId,
    outer_index_initial_const: i64,
    outer_index_next_value: ValueId,
    loop_bound_value: ValueId,
    loop_bound_const: i64,
    row_index_value: ValueId,
    row_modulus_value: ValueId,
    row_modulus_const: i64,
    observer_period_value: ValueId,
    observer_period_const: i64,
    accumulator_phi_value: ValueId,
    accumulator_initial_value: ValueId,
    accumulator_initial_const: i64,
    accumulator_next_value: ValueId,
    edit_middle_value: ValueId,
    edit_middle_text: String,
    edit_middle_byte_len: usize,
    observer_bound_value: ValueId,
    observer_bound_const: i64,
    observer_needle_value: ValueId,
    observer_needle_text: String,
    observer_needle_byte_len: usize,
    observer_suffix_value: ValueId,
    observer_suffix_text: String,
    observer_suffix_byte_len: usize,
    execution_mode: ArrayTextCombinedRegionExecutionMode,
    proof_region: ArrayTextCombinedRegionProofRegion,
    proof: ArrayTextCombinedRegionProof,
    byte_boundary_proof: Option<ArrayTextCombinedRegionByteBoundaryProof>,
}

impl ArrayTextCombinedRegionRoute {
    pub fn begin_block(&self) -> BasicBlockId {
        self.begin_block
    }

    pub fn header_block(&self) -> BasicBlockId {
        self.header_block
    }

    pub fn edit_block(&self) -> BasicBlockId {
        self.edit_block
    }

    pub fn observer_begin_block(&self) -> BasicBlockId {
        self.observer_begin_block
    }

    pub fn observer_header_block(&self) -> BasicBlockId {
        self.observer_header_block
    }

    pub fn observer_block(&self) -> BasicBlockId {
        self.observer_block
    }

    pub fn observer_store_block(&self) -> BasicBlockId {
        self.observer_store_block
    }

    pub fn observer_latch_block(&self) -> BasicBlockId {
        self.observer_latch_block
    }

    pub fn observer_exit_block(&self) -> BasicBlockId {
        self.observer_exit_block
    }

    pub fn latch_block(&self) -> BasicBlockId {
        self.latch_block
    }

    pub fn exit_block(&self) -> BasicBlockId {
        self.exit_block
    }

    pub fn array_value(&self) -> ValueId {
        self.array_value
    }

    pub fn outer_index_phi_value(&self) -> ValueId {
        self.outer_index_phi_value
    }

    pub fn outer_index_initial_value(&self) -> ValueId {
        self.outer_index_initial_value
    }

    pub fn outer_index_initial_const(&self) -> i64 {
        self.outer_index_initial_const
    }

    pub fn outer_index_next_value(&self) -> ValueId {
        self.outer_index_next_value
    }

    pub fn loop_bound_value(&self) -> ValueId {
        self.loop_bound_value
    }

    pub fn loop_bound_const(&self) -> i64 {
        self.loop_bound_const
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

    pub fn observer_period_value(&self) -> ValueId {
        self.observer_period_value
    }

    pub fn observer_period_const(&self) -> i64 {
        self.observer_period_const
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

    pub fn edit_middle_value(&self) -> ValueId {
        self.edit_middle_value
    }

    pub fn edit_middle_text(&self) -> &str {
        &self.edit_middle_text
    }

    pub fn edit_middle_byte_len(&self) -> usize {
        self.edit_middle_byte_len
    }

    pub fn observer_bound_value(&self) -> ValueId {
        self.observer_bound_value
    }

    pub fn observer_bound_const(&self) -> i64 {
        self.observer_bound_const
    }

    pub fn observer_needle_value(&self) -> ValueId {
        self.observer_needle_value
    }

    pub fn observer_needle_text(&self) -> &str {
        &self.observer_needle_text
    }

    pub fn observer_needle_byte_len(&self) -> usize {
        self.observer_needle_byte_len
    }

    pub fn observer_suffix_value(&self) -> ValueId {
        self.observer_suffix_value
    }

    pub fn observer_suffix_text(&self) -> &str {
        &self.observer_suffix_text
    }

    pub fn observer_suffix_byte_len(&self) -> usize {
        self.observer_suffix_byte_len
    }

    pub fn execution_mode(&self) -> &'static str {
        self.execution_mode.as_str()
    }

    pub fn proof_region(&self) -> &'static str {
        self.proof_region.as_str()
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }

    pub fn byte_boundary_proof(&self) -> Option<&'static str> {
        self.byte_boundary_proof.map(|proof| proof.as_str())
    }
}

pub fn refresh_module_array_text_combined_region_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_text_combined_region_routes(function);
    }
}

pub fn refresh_function_array_text_combined_region_routes(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let routes = function
        .metadata
        .array_text_edit_routes
        .iter()
        .filter_map(|edit_route| derive_combined_region(function, &def_map, edit_route))
        .collect();
    function.metadata.array_text_combined_regions = routes;
}

fn derive_combined_region(
    function: &MirFunction,
    def_map: &ValueDefMap,
    edit_route: &ArrayTextEditRoute,
) -> Option<ArrayTextCombinedRegionRoute> {
    if !edit_route.is_lenhalf_insert_mid_same_slot() {
        return None;
    }

    let edit_block_id = edit_route.block();
    let edit_block = function.blocks.get(&edit_block_id)?;
    let (outer_index_phi_value, row_modulus_value, row_modulus_const) =
        match_mod_const(function, def_map, edit_route.index_value())?;
    let row_index_value = root(function, def_map, edit_route.index_value());
    let (observer_period_value, observer_period_const) =
        match_periodic_zero_condition(function, def_map, edit_block, outer_index_phi_value)?;

    let (header_block, exit_block) = find_loop_header_for_body(function, edit_block_id)?;
    let header = function.blocks.get(&header_block)?;
    let (loop_bound_value, loop_bound_const) = match_header_bound(
        function,
        def_map,
        header,
        outer_index_phi_value,
        edit_block_id,
    )?;

    let (observer_route, observer_mapping, observer_contract, latch_block) =
        match_nested_observer_region(function, edit_block, edit_route)?;
    if observer_contract.execution_mode
        != ArrayTextObserverExecutorExecutionMode::SingleRegionExecutor
    {
        return None;
    }
    if root(function, def_map, observer_route.array_value())
        != root(function, def_map, edit_route.array_value())
    {
        return None;
    }

    let begin_block = single_preheader_jump_to_header(function, header_block, latch_block)?;
    let latch = function.blocks.get(&latch_block)?;
    if !matches!(
        latch.terminator.as_ref()?,
        MirInstruction::Jump { target, .. } if *target == header_block
    ) {
        return None;
    }

    let observer_exit = function.blocks.get(&observer_mapping.exit_block)?;
    if !matches!(
        observer_exit.terminator.as_ref()?,
        MirInstruction::Jump { target, .. } if *target == latch_block
    ) {
        return None;
    }

    let (outer_index_initial_value, outer_index_next_value) =
        match_loop_phi_inputs(header, outer_index_phi_value, begin_block, latch_block)?;
    let outer_index_initial_const = const_i64(function, def_map, outer_index_initial_value)?;
    if outer_index_initial_const != 0 {
        return None;
    }
    if !is_add_const_one_from(
        function,
        def_map,
        latch,
        outer_index_next_value,
        outer_index_phi_value,
    ) {
        return None;
    }

    let (accumulator_phi_value, accumulator_initial_value, accumulator_next_value) =
        match_outer_accumulator(
            function,
            def_map,
            header,
            latch,
            begin_block,
            latch_block,
            outer_index_phi_value,
        )?;
    let accumulator_initial_const = const_i64(function, def_map, accumulator_initial_value)?;
    if accumulator_initial_const != 0 {
        return None;
    }

    let byte_boundary_proof = prove_ascii_preserved_text_cell_boundary(
        function,
        def_map,
        edit_route,
        observer_route,
        observer_mapping,
        begin_block,
        row_modulus_const,
    );

    Some(ArrayTextCombinedRegionRoute {
        begin_block,
        header_block,
        edit_block: edit_block_id,
        observer_begin_block: observer_mapping.begin_block,
        observer_header_block: observer_mapping.header_block,
        observer_block: observer_mapping.observer_block,
        observer_store_block: observer_mapping.then_store_block,
        observer_latch_block: observer_mapping.latch_block,
        observer_exit_block: observer_mapping.exit_block,
        latch_block,
        exit_block,
        array_value: root(function, def_map, edit_route.array_value()),
        outer_index_phi_value,
        outer_index_initial_value: root(function, def_map, outer_index_initial_value),
        outer_index_initial_const,
        outer_index_next_value: root(function, def_map, outer_index_next_value),
        loop_bound_value,
        loop_bound_const,
        row_index_value,
        row_modulus_value,
        row_modulus_const,
        observer_period_value,
        observer_period_const,
        accumulator_phi_value,
        accumulator_initial_value: root(function, def_map, accumulator_initial_value),
        accumulator_initial_const,
        accumulator_next_value: root(function, def_map, accumulator_next_value),
        edit_middle_value: root(function, def_map, edit_route.middle_value()),
        edit_middle_text: edit_route.middle_text().to_string(),
        edit_middle_byte_len: edit_route.middle_byte_len(),
        observer_bound_value: observer_mapping.loop_bound_value,
        observer_bound_const: observer_mapping.loop_bound_const,
        observer_needle_value: root(function, def_map, observer_route.observer_arg0()),
        observer_needle_text: observer_route.observer_arg0_text()?.to_string(),
        observer_needle_byte_len: observer_route.observer_arg0_byte_len()?,
        observer_suffix_value: observer_mapping.suffix_value,
        observer_suffix_text: observer_mapping.suffix_text.clone(),
        observer_suffix_byte_len: observer_mapping.suffix_byte_len,
        execution_mode: ArrayTextCombinedRegionExecutionMode::SingleRegionExecutor,
        proof_region: ArrayTextCombinedRegionProofRegion::OuterLoopWithPeriodicObserverStore,
        proof: ArrayTextCombinedRegionProof::OuterLenHalfEditWithPeriodicObserverStore,
        byte_boundary_proof,
    })
}

fn root(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> ValueId {
    resolve_value_origin(function, def_map, value)
}

fn defining_instruction<'a>(
    function: &'a MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<&'a MirInstruction> {
    let value = root(function, def_map, value);
    let (block, index) = def_map.get(&value).copied()?;
    function.blocks.get(&block)?.instructions.get(index)
}

fn const_i64(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<i64> {
    match defining_instruction(function, def_map, value)? {
        MirInstruction::Const {
            value: ConstValue::Integer(value),
            ..
        } => Some(*value),
        _ => None,
    }
}

fn const_string(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<String> {
    match defining_instruction(function, def_map, value)? {
        MirInstruction::Const {
            value: ConstValue::String(text),
            ..
        } => Some(text.clone()),
        _ => None,
    }
}

fn match_mod_const(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<(ValueId, ValueId, i64)> {
    let value = root(function, def_map, value);
    match defining_instruction(function, def_map, value)? {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Mod,
            lhs,
            rhs,
        } if root(function, def_map, *dst) == value => {
            let rhs = root(function, def_map, *rhs);
            Some((
                root(function, def_map, *lhs),
                rhs,
                const_i64(function, def_map, rhs)?,
            ))
        }
        _ => None,
    }
}

fn match_periodic_zero_condition(
    function: &MirFunction,
    def_map: &ValueDefMap,
    edit_block: &BasicBlock,
    outer_index_phi_value: ValueId,
) -> Option<(ValueId, i64)> {
    let cond = match edit_block.terminator.as_ref()? {
        MirInstruction::Branch { condition, .. } => root(function, def_map, *condition),
        _ => return None,
    };
    match defining_instruction(function, def_map, cond)? {
        MirInstruction::Compare {
            op: CompareOp::Eq,
            lhs,
            rhs,
            ..
        } => {
            let lhs = root(function, def_map, *lhs);
            let rhs = root(function, def_map, *rhs);
            let (mod_lhs, period_value, period_const) = match_mod_const(function, def_map, lhs)?;
            if mod_lhs == outer_index_phi_value && const_i64(function, def_map, rhs) == Some(0) {
                Some((period_value, period_const))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn find_loop_header_for_body(
    function: &MirFunction,
    body_block: BasicBlockId,
) -> Option<(BasicBlockId, BasicBlockId)> {
    for (block_id, block) in &function.blocks {
        let MirInstruction::Branch {
            then_bb, else_bb, ..
        } = block.terminator.as_ref()?
        else {
            continue;
        };
        if *then_bb == body_block {
            return Some((*block_id, *else_bb));
        }
        if *else_bb == body_block {
            return Some((*block_id, *then_bb));
        }
    }
    None
}

fn match_header_bound(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &BasicBlock,
    outer_index_phi_value: ValueId,
    body_block: BasicBlockId,
) -> Option<(ValueId, i64)> {
    let MirInstruction::Branch {
        condition,
        then_bb,
        else_bb,
        ..
    } = header.terminator.as_ref()?
    else {
        return None;
    };
    if *then_bb != body_block && *else_bb != body_block {
        return None;
    }
    let condition = root(function, def_map, *condition);
    match defining_instruction(function, def_map, condition)? {
        MirInstruction::Compare {
            op: CompareOp::Lt,
            lhs,
            rhs,
            ..
        } if root(function, def_map, *lhs) == outer_index_phi_value => {
            let rhs = root(function, def_map, *rhs);
            Some((rhs, const_i64(function, def_map, rhs)?))
        }
        _ => None,
    }
}

fn match_nested_observer_region<'a>(
    function: &'a MirFunction,
    edit_block: &BasicBlock,
    edit_route: &ArrayTextEditRoute,
) -> Option<(
    &'a ArrayTextObserverRoute,
    &'a ArrayTextObserverStoreRegionMapping,
    &'a ArrayTextObserverExecutorContract,
    BasicBlockId,
)> {
    let MirInstruction::Branch {
        then_bb, else_bb, ..
    } = edit_block.terminator.as_ref()?
    else {
        return None;
    };
    for observer_route in &function.metadata.array_text_observer_routes {
        if observer_route.array_value() != edit_route.array_value() {
            continue;
        }
        let contract = observer_route.executor_contract()?;
        let mapping = contract.region_mapping.as_ref()?;
        if mapping.begin_block == *then_bb {
            return Some((observer_route, mapping, contract, *else_bb));
        }
        if mapping.begin_block == *else_bb {
            return Some((observer_route, mapping, contract, *then_bb));
        }
    }
    None
}

fn single_preheader_jump_to_header(
    function: &MirFunction,
    header_block: BasicBlockId,
    latch_block: BasicBlockId,
) -> Option<BasicBlockId> {
    let header = function.blocks.get(&header_block)?;
    let mut candidates = header
        .predecessors
        .iter()
        .copied()
        .filter(|pred| *pred != latch_block);
    let begin_block = candidates.next()?;
    if candidates.next().is_some() {
        return None;
    }
    let begin = function.blocks.get(&begin_block)?;
    match begin.terminator.as_ref()? {
        MirInstruction::Jump { target, .. } if *target == header_block => Some(begin_block),
        _ => None,
    }
}

fn match_loop_phi_inputs(
    header: &BasicBlock,
    phi_value: ValueId,
    begin_block: BasicBlockId,
    latch_block: BasicBlockId,
) -> Option<(ValueId, ValueId)> {
    header.instructions.iter().find_map(|inst| {
        let MirInstruction::Phi { dst, inputs, .. } = inst else {
            return None;
        };
        if *dst != phi_value {
            return None;
        }
        let initial = inputs
            .iter()
            .find_map(|(block, value)| (*block == begin_block).then_some(*value))?;
        let next = inputs
            .iter()
            .find_map(|(block, value)| (*block == latch_block).then_some(*value))?;
        Some((initial, next))
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
    let source_value = root(function, def_map, source_value);
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

fn prove_ascii_preserved_text_cell_boundary(
    function: &MirFunction,
    def_map: &ValueDefMap,
    edit_route: &ArrayTextEditRoute,
    observer_route: &ArrayTextObserverRoute,
    observer_mapping: &ArrayTextObserverStoreRegionMapping,
    begin_block: BasicBlockId,
    row_modulus_const: i64,
) -> Option<ArrayTextCombinedRegionByteBoundaryProof> {
    if !edit_route.middle_text().is_ascii()
        || !observer_route.observer_arg0_text()?.is_ascii()
        || !observer_mapping.suffix_text.is_ascii()
    {
        return None;
    }
    ascii_seed_loop_initializes_text_array(
        function,
        def_map,
        begin_block,
        edit_route.array_value(),
        row_modulus_const,
    )
    .then_some(ArrayTextCombinedRegionByteBoundaryProof::AsciiPreservedTextCell)
}

fn ascii_seed_loop_initializes_text_array(
    function: &MirFunction,
    def_map: &ValueDefMap,
    begin_block: BasicBlockId,
    array_value: ValueId,
    expected_bound: i64,
) -> bool {
    let Some(begin) = function.blocks.get(&begin_block) else {
        return false;
    };
    if begin.predecessors.len() != 1 {
        return false;
    }
    if block_has_same_array_method_call(function, def_map, begin, array_value) {
        return false;
    }

    let seed_header_block = *begin.predecessors.iter().next().expect("checked len");
    let Some(seed_header) = function.blocks.get(&seed_header_block) else {
        return false;
    };
    let Some(seed_body_block) = loop_body_that_exits_to(seed_header, begin_block) else {
        return false;
    };
    let Some(seed_body) = function.blocks.get(&seed_body_block) else {
        return false;
    };
    if !matches!(
        seed_body.terminator.as_ref(),
        Some(MirInstruction::Jump { target, .. }) if *target == seed_header_block
    ) {
        return false;
    }
    let Some(seed_preheader_block) =
        single_non_latch_predecessor(function, seed_header_block, seed_body_block)
    else {
        return false;
    };
    let Some(seed_preheader) = function.blocks.get(&seed_preheader_block) else {
        return false;
    };
    if block_has_same_array_method_call(function, def_map, seed_preheader, array_value) {
        return false;
    }
    if !has_counted_loop_bound(
        function,
        def_map,
        seed_header,
        seed_preheader_block,
        seed_body_block,
        expected_bound,
    ) {
        return false;
    }
    body_pushes_single_ascii_literal(function, def_map, seed_body, array_value)
}

fn loop_body_that_exits_to(header: &BasicBlock, exit_block: BasicBlockId) -> Option<BasicBlockId> {
    let MirInstruction::Branch {
        then_bb, else_bb, ..
    } = header.terminator.as_ref()?
    else {
        return None;
    };
    if *then_bb == exit_block {
        Some(*else_bb)
    } else if *else_bb == exit_block {
        Some(*then_bb)
    } else {
        None
    }
}

fn single_non_latch_predecessor(
    function: &MirFunction,
    header_block: BasicBlockId,
    latch_block: BasicBlockId,
) -> Option<BasicBlockId> {
    let header = function.blocks.get(&header_block)?;
    let mut candidates = header
        .predecessors
        .iter()
        .copied()
        .filter(|pred| *pred != latch_block);
    let preheader = candidates.next()?;
    candidates.next().is_none().then_some(preheader)
}

fn has_counted_loop_bound(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &BasicBlock,
    preheader_block: BasicBlockId,
    body_block: BasicBlockId,
    expected_bound: i64,
) -> bool {
    header.instructions.iter().any(|inst| {
        let MirInstruction::Phi { dst, .. } = inst else {
            return false;
        };
        let Some((_bound_value, bound_const)) =
            match_header_bound(function, def_map, header, *dst, body_block)
        else {
            return false;
        };
        if bound_const != expected_bound {
            return false;
        }
        let Some((initial, next)) =
            match_loop_phi_inputs(header, *dst, preheader_block, body_block)
        else {
            return false;
        };
        const_i64(function, def_map, initial) == Some(0)
            && is_add_const_one_from(
                function,
                def_map,
                function.blocks.get(&body_block).expect("body checked"),
                next,
                *dst,
            )
    })
}

fn body_pushes_single_ascii_literal(
    function: &MirFunction,
    def_map: &ValueDefMap,
    body: &BasicBlock,
    array_value: ValueId,
) -> bool {
    let mut push_count = 0;
    for inst in &body.instructions {
        let Some(method) = same_array_method_call(function, def_map, inst, array_value) else {
            continue;
        };
        let ("push", args) = method else {
            return false;
        };
        if args.len() != 1 {
            return false;
        }
        let Some(text) = const_string(function, def_map, args[0]) else {
            return false;
        };
        if !text.is_ascii() {
            return false;
        }
        push_count += 1;
    }
    push_count == 1
}

fn block_has_same_array_method_call(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &BasicBlock,
    array_value: ValueId,
) -> bool {
    block
        .instructions
        .iter()
        .any(|inst| same_array_method_call(function, def_map, inst, array_value).is_some())
}

fn same_array_method_call<'a>(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &'a MirInstruction,
    array_value: ValueId,
) -> Option<(&'a str, &'a [ValueId])> {
    match inst {
        MirInstruction::Call {
            callee:
                Some(super::definitions::Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } if matches!(box_name.as_str(), "RuntimeDataBox" | "ArrayBox")
            && root(function, def_map, *receiver) == root(function, def_map, array_value) =>
        {
            Some((method.as_str(), args.as_slice()))
        }
        _ => None,
    }
}

fn match_outer_accumulator(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &BasicBlock,
    latch: &BasicBlock,
    begin_block: BasicBlockId,
    latch_block: BasicBlockId,
    outer_index_phi_value: ValueId,
) -> Option<(ValueId, ValueId, ValueId)> {
    header.instructions.iter().find_map(|inst| {
        let MirInstruction::Phi { dst, inputs, .. } = inst else {
            return None;
        };
        if *dst == outer_index_phi_value {
            return None;
        }
        let initial = inputs
            .iter()
            .find_map(|(block, value)| (*block == begin_block).then_some(*value))?;
        let next = inputs
            .iter()
            .find_map(|(block, value)| (*block == latch_block).then_some(*value))?;
        is_add_const_one_from(function, def_map, latch, next, *dst).then_some((*dst, initial, next))
    })
}
