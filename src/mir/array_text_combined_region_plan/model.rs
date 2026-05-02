use super::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ArrayTextCombinedRegionProof {
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
pub(super) enum ArrayTextCombinedRegionExecutionMode {
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
pub(super) enum ArrayTextCombinedRegionProofRegion {
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
pub(super) enum ArrayTextCombinedRegionEffect {
    LenHalfInsertMidStoreCell,
    ObserveIndexOf,
    ConstSuffixStoreCell,
    ScalarAccumulatorAddOne,
}

impl std::fmt::Display for ArrayTextCombinedRegionEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextCombinedRegionEffect {
    fn as_str(self) -> &'static str {
        match self {
            Self::LenHalfInsertMidStoreCell => "store.cell(lenhalf_insert_mid_const)",
            Self::ObserveIndexOf => "observe.indexof",
            Self::ConstSuffixStoreCell => "store.cell(const_suffix_append)",
            Self::ScalarAccumulatorAddOne => "scalar_accumulator(+1)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ArrayTextCombinedRegionConsumerCapability {
    SinkStore,
    CompareOnly,
    LengthOnlyResultCarry,
}

impl std::fmt::Display for ArrayTextCombinedRegionConsumerCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextCombinedRegionConsumerCapability {
    fn as_str(self) -> &'static str {
        match self {
            Self::SinkStore => "sink_store",
            Self::CompareOnly => "compare_only",
            Self::LengthOnlyResultCarry => "length_only_result_carry",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ArrayTextCombinedRegionByteBoundaryProof {
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
    pub(super) begin_block: BasicBlockId,
    pub(super) header_block: BasicBlockId,
    pub(super) edit_block: BasicBlockId,
    pub(super) observer_begin_block: BasicBlockId,
    pub(super) observer_header_block: BasicBlockId,
    pub(super) observer_block: BasicBlockId,
    pub(super) observer_store_block: BasicBlockId,
    pub(super) observer_latch_block: BasicBlockId,
    pub(super) observer_exit_block: BasicBlockId,
    pub(super) latch_block: BasicBlockId,
    pub(super) exit_block: BasicBlockId,
    pub(super) array_value: ValueId,
    pub(super) outer_index_phi_value: ValueId,
    pub(super) outer_index_initial_value: ValueId,
    pub(super) outer_index_initial_const: i64,
    pub(super) outer_index_next_value: ValueId,
    pub(super) loop_bound_value: ValueId,
    pub(super) loop_bound_const: i64,
    pub(super) row_index_value: ValueId,
    pub(super) row_modulus_value: ValueId,
    pub(super) row_modulus_const: i64,
    pub(super) observer_period_value: ValueId,
    pub(super) observer_period_const: i64,
    pub(super) accumulator_phi_value: ValueId,
    pub(super) accumulator_initial_value: ValueId,
    pub(super) accumulator_initial_const: i64,
    pub(super) accumulator_next_value: ValueId,
    pub(super) edit_middle_value: ValueId,
    pub(super) edit_middle_text: String,
    pub(super) edit_middle_byte_len: usize,
    pub(super) observer_bound_value: ValueId,
    pub(super) observer_bound_const: i64,
    pub(super) observer_needle_value: ValueId,
    pub(super) observer_needle_text: String,
    pub(super) observer_needle_byte_len: usize,
    pub(super) observer_suffix_value: ValueId,
    pub(super) observer_suffix_text: String,
    pub(super) observer_suffix_byte_len: usize,
    pub(super) execution_mode: ArrayTextCombinedRegionExecutionMode,
    pub(super) proof_region: ArrayTextCombinedRegionProofRegion,
    pub(super) proof: ArrayTextCombinedRegionProof,
    pub(super) byte_boundary_proof: Option<ArrayTextCombinedRegionByteBoundaryProof>,
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

    pub fn publication_boundary(&self) -> &'static str {
        "none"
    }

    pub fn carrier(&self) -> &'static str {
        "array_lane_text_cell"
    }

    pub fn effects(&self) -> Vec<&'static str> {
        [
            ArrayTextCombinedRegionEffect::LenHalfInsertMidStoreCell,
            ArrayTextCombinedRegionEffect::ObserveIndexOf,
            ArrayTextCombinedRegionEffect::ConstSuffixStoreCell,
            ArrayTextCombinedRegionEffect::ScalarAccumulatorAddOne,
        ]
        .into_iter()
        .map(ArrayTextCombinedRegionEffect::as_str)
        .collect()
    }

    pub fn consumer_capabilities(&self) -> Vec<&'static str> {
        [
            ArrayTextCombinedRegionConsumerCapability::SinkStore,
            ArrayTextCombinedRegionConsumerCapability::CompareOnly,
            ArrayTextCombinedRegionConsumerCapability::LengthOnlyResultCarry,
        ]
        .into_iter()
        .map(ArrayTextCombinedRegionConsumerCapability::as_str)
        .collect()
    }

    pub fn materialization_policy(&self) -> &'static str {
        "text_resident_or_stringlike_slot"
    }
}
