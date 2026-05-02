/*!
 * MIR-owned array/text residence session plans.
 *
 * H25 keeps this metadata-only: it proves where a future backend may hold an
 * array text residence session, but it does not change lowering or runtime
 * behavior. Runtime remains executor-only; legality lives here.
 */

use super::{BasicBlockId, MirFunction, MirModule, ValueId};

mod derive;

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
        .filter_map(|route| derive::derive_loopcarry_session(function, route))
        .collect();
    function.metadata.array_text_residence_sessions = routes;
}
