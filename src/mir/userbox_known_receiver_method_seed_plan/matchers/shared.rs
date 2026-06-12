use super::BasicBlockId;

pub(super) struct CounterStepMethodFacts {
    pub(super) block_count: usize,
    pub(super) block: BasicBlockId,
    pub(super) delta_i64: i64,
}

pub(super) struct PointSumMethodFacts {
    pub(super) block_count: usize,
    pub(super) block: BasicBlockId,
}

pub(super) struct ChainForwardFacts {
    pub(super) block_count: usize,
    pub(super) block: BasicBlockId,
}
