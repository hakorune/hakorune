use super::types::{
    ArrayTextObserverExecutorCarrier, ArrayTextObserverExecutorConsumerCapability,
    ArrayTextObserverExecutorEffect, ArrayTextObserverExecutorExecutionMode,
    ArrayTextObserverExecutorMaterializationPolicy, ArrayTextObserverExecutorProofRegion,
};
use crate::mir::array_text_observer_plan::ArrayTextObserverPublicationBoundary;
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextObserverStoreRegionMapping {
    array_root_value: ValueId,
    loop_index_phi_value: ValueId,
    loop_index_initial_value: ValueId,
    loop_index_initial_const: i64,
    loop_index_next_value: ValueId,
    loop_bound_value: ValueId,
    loop_bound_const: i64,
    begin_block: BasicBlockId,
    begin_to_header_block: BasicBlockId,
    header_block: BasicBlockId,
    observer_block: BasicBlockId,
    observer_instruction_index: usize,
    predicate_value: ValueId,
    then_store_block: BasicBlockId,
    store_instruction_index: usize,
    suffix_value: ValueId,
    suffix_text: String,
    suffix_byte_len: usize,
    row_index_value: Option<ValueId>,
    row_modulus_value: Option<ValueId>,
    row_modulus_const: Option<i64>,
    length_result_value: Option<ValueId>,
    accumulator_phi_value: Option<ValueId>,
    accumulator_next_value: Option<ValueId>,
    latch_block: BasicBlockId,
    exit_block: BasicBlockId,
}

impl ArrayTextObserverStoreRegionMapping {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        array_root_value: ValueId,
        loop_index_phi_value: ValueId,
        loop_index_initial_value: ValueId,
        loop_index_initial_const: i64,
        loop_index_next_value: ValueId,
        loop_bound_value: ValueId,
        loop_bound_const: i64,
        begin_block: BasicBlockId,
        begin_to_header_block: BasicBlockId,
        header_block: BasicBlockId,
        observer_block: BasicBlockId,
        observer_instruction_index: usize,
        predicate_value: ValueId,
        then_store_block: BasicBlockId,
        store_instruction_index: usize,
        suffix_value: ValueId,
        suffix_text: String,
        suffix_byte_len: usize,
        latch_block: BasicBlockId,
        exit_block: BasicBlockId,
    ) -> Self {
        Self {
            array_root_value,
            loop_index_phi_value,
            loop_index_initial_value,
            loop_index_initial_const,
            loop_index_next_value,
            loop_bound_value,
            loop_bound_const,
            begin_block,
            begin_to_header_block,
            header_block,
            observer_block,
            observer_instruction_index,
            predicate_value,
            then_store_block,
            store_instruction_index,
            suffix_value,
            suffix_text,
            suffix_byte_len,
            row_index_value: None,
            row_modulus_value: None,
            row_modulus_const: None,
            length_result_value: None,
            accumulator_phi_value: None,
            accumulator_next_value: None,
            latch_block,
            exit_block,
        }
    }

    pub(super) fn with_len_sum_payload(
        mut self,
        row_index_value: ValueId,
        row_modulus_value: ValueId,
        row_modulus_const: i64,
        length_result_value: ValueId,
        accumulator_phi_value: ValueId,
        accumulator_next_value: ValueId,
    ) -> Self {
        self.row_index_value = Some(row_index_value);
        self.row_modulus_value = Some(row_modulus_value);
        self.row_modulus_const = Some(row_modulus_const);
        self.length_result_value = Some(length_result_value);
        self.accumulator_phi_value = Some(accumulator_phi_value);
        self.accumulator_next_value = Some(accumulator_next_value);
        self
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

    pub fn begin_block(&self) -> BasicBlockId {
        self.begin_block
    }

    pub fn begin_to_header_block(&self) -> BasicBlockId {
        self.begin_to_header_block
    }

    pub fn header_block(&self) -> BasicBlockId {
        self.header_block
    }

    pub fn observer_block(&self) -> BasicBlockId {
        self.observer_block
    }

    pub fn observer_instruction_index(&self) -> usize {
        self.observer_instruction_index
    }

    pub fn predicate_value(&self) -> ValueId {
        self.predicate_value
    }

    pub fn then_store_block(&self) -> BasicBlockId {
        self.then_store_block
    }

    pub fn store_instruction_index(&self) -> usize {
        self.store_instruction_index
    }

    pub fn suffix_value(&self) -> ValueId {
        self.suffix_value
    }

    pub fn suffix_text(&self) -> &str {
        &self.suffix_text
    }

    pub fn suffix_byte_len(&self) -> usize {
        self.suffix_byte_len
    }

    pub fn row_index_value(&self) -> Option<ValueId> {
        self.row_index_value
    }

    pub fn row_modulus_value(&self) -> Option<ValueId> {
        self.row_modulus_value
    }

    pub fn row_modulus_const(&self) -> Option<i64> {
        self.row_modulus_const
    }

    pub fn length_result_value(&self) -> Option<ValueId> {
        self.length_result_value
    }

    pub fn accumulator_phi_value(&self) -> Option<ValueId> {
        self.accumulator_phi_value
    }

    pub fn accumulator_next_value(&self) -> Option<ValueId> {
        self.accumulator_next_value
    }

    pub fn latch_block(&self) -> BasicBlockId {
        self.latch_block
    }

    pub fn exit_block(&self) -> BasicBlockId {
        self.exit_block
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextObserverExecutorContract {
    execution_mode: ArrayTextObserverExecutorExecutionMode,
    proof_region: ArrayTextObserverExecutorProofRegion,
    publication_boundary: ArrayTextObserverPublicationBoundary,
    carrier: ArrayTextObserverExecutorCarrier,
    effects: Vec<ArrayTextObserverExecutorEffect>,
    consumer_capabilities: Vec<ArrayTextObserverExecutorConsumerCapability>,
    materialization_policy: ArrayTextObserverExecutorMaterializationPolicy,
    region_mapping: Option<ArrayTextObserverStoreRegionMapping>,
}

impl ArrayTextObserverExecutorContract {
    pub fn execution_mode(&self) -> &'static str {
        self.execution_mode.as_str()
    }

    pub fn is_single_region_executor(&self) -> bool {
        self.execution_mode == ArrayTextObserverExecutorExecutionMode::SingleRegionExecutor
    }

    pub fn proof_region(&self) -> &'static str {
        self.proof_region.as_str()
    }

    pub fn publication_boundary(&self) -> &'static str {
        self.publication_boundary.as_str()
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

    pub fn region_mapping(&self) -> Option<&ArrayTextObserverStoreRegionMapping> {
        self.region_mapping.as_ref()
    }

    pub(crate) fn conditional_suffix_store_single_region(
        region_mapping: ArrayTextObserverStoreRegionMapping,
    ) -> Self {
        Self {
            execution_mode: ArrayTextObserverExecutorExecutionMode::SingleRegionExecutor,
            proof_region: ArrayTextObserverExecutorProofRegion::LoopBackedgeSingleBody,
            publication_boundary: ArrayTextObserverPublicationBoundary::None,
            carrier: ArrayTextObserverExecutorCarrier::ArrayLaneTextCell,
            effects: vec![
                ArrayTextObserverExecutorEffect::ObserveIndexOf,
                ArrayTextObserverExecutorEffect::StoreCell,
            ],
            consumer_capabilities: vec![
                ArrayTextObserverExecutorConsumerCapability::CompareOnly,
                ArrayTextObserverExecutorConsumerCapability::SinkStore,
            ],
            materialization_policy:
                ArrayTextObserverExecutorMaterializationPolicy::TextResidentOrStringlikeSlot,
            region_mapping: Some(region_mapping),
        }
    }

    pub(crate) fn conditional_suffix_store_len_sum_single_region(
        region_mapping: ArrayTextObserverStoreRegionMapping,
    ) -> Self {
        Self {
            execution_mode: ArrayTextObserverExecutorExecutionMode::SingleRegionExecutor,
            proof_region: ArrayTextObserverExecutorProofRegion::LoopBackedgeSingleBody,
            publication_boundary: ArrayTextObserverPublicationBoundary::None,
            carrier: ArrayTextObserverExecutorCarrier::ArrayLaneTextCell,
            effects: vec![
                ArrayTextObserverExecutorEffect::ObserveIndexOf,
                ArrayTextObserverExecutorEffect::StoreCell,
                ArrayTextObserverExecutorEffect::LengthResultCarry,
                ArrayTextObserverExecutorEffect::ScalarAccumulator,
            ],
            consumer_capabilities: vec![
                ArrayTextObserverExecutorConsumerCapability::CompareOnly,
                ArrayTextObserverExecutorConsumerCapability::SinkStoreLenSum,
            ],
            materialization_policy:
                ArrayTextObserverExecutorMaterializationPolicy::TextResidentOrStringlikeSlot,
            region_mapping: Some(region_mapping),
        }
    }
}
