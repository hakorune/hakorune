use hakorune_mir_core::{BasicBlockId, ValueId};

/// Stage1 LoopRange lowering facts.
///
/// These facts are emitted by the executable LoopRange route so later verifier
/// and backend rows can consume a typed metadata surface instead of
/// rediscovering range-loop shape from raw CFG blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopRangeFact {
    pub index_name: String,
    pub start_value: ValueId,
    pub end_value: ValueId,
    pub index_phi: ValueId,
    pub preheader_bb: BasicBlockId,
    pub header_bb: BasicBlockId,
    pub body_bb: BasicBlockId,
    pub step_bb: BasicBlockId,
    pub exit_bb: BasicBlockId,
    pub step: i64,
    pub end_exclusive: bool,
    pub index_read_only: bool,
    pub body_local_writes_supported: bool,
    pub loop_carried_writes_supported: bool,
    pub body_writes_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountingLoopFact {
    pub index_name: String,
    pub lower_value: ValueId,
    pub upper_exclusive_value: ValueId,
    pub index_value: ValueId,
    pub preheader_bb: BasicBlockId,
    pub header_bb: BasicBlockId,
    pub body_bb: BasicBlockId,
    pub latch_bb: BasicBlockId,
    pub exit_bb: BasicBlockId,
    pub step: i64,
    pub end_exclusive: bool,
    pub index_body_read_only: bool,
    pub loop_carried_writes_supported: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeIndexFactOriginKind {
    RangeLoop,
    CountingLoop,
    ModuloOfRangeIndex,
    FastMemAssume,
}

impl RangeIndexFactOriginKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RangeLoop => "range_loop",
            Self::CountingLoop => "counting_loop",
            Self::ModuloOfRangeIndex => "modulo_of_range_index",
            Self::FastMemAssume => "fastmem_assume",
        }
    }
}

/// Canonical range-index view consumed by fast-path planners.
///
/// Producers such as `LoopRangeFact` and future counting-loop/induction
/// recognizers may feed this view.  Consumers must depend on this canonical
/// shape, not on source syntax or producer-specific fact types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeIndexFact {
    pub fact_id: u32,
    pub origin_kind: RangeIndexFactOriginKind,
    pub index_value: ValueId,
    pub lower_value: ValueId,
    pub upper_exclusive_value: ValueId,
    pub body_bb: BasicBlockId,
    pub step: i64,
    pub end_exclusive: bool,
    pub index_body_read_only: bool,
    pub loop_carried_writes_supported: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectArrayExtentProofKind {
    DefaultCapacity,
    ProducerInvariant,
}

impl DirectArrayExtentProofKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultCapacity => "default_capacity",
            Self::ProducerInvariant => "producer_invariant",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionStabilityProofKind {
    ProducerInvariant,
}

impl RegionStabilityProofKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProducerInvariant => "producer_invariant",
        }
    }
}

/// Region stability proof for a memory-like receiver value.
///
/// Extent facts prove that a receiver is large enough; this fact proves that
/// the receiver's storage base stays stable for the planned access region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionStabilityFact {
    pub fact_id: u32,
    pub region_value: ValueId,
    pub scope_bb: BasicBlockId,
    pub proof_kind: RegionStabilityProofKind,
    pub stable_in_region: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanBorrowMutability {
    Read,
    Write,
}

impl SpanBorrowMutability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanElementType {
    I64,
}

impl SpanElementType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::I64 => "i64",
        }
    }
}

/// No-escape borrow fact for a future Span view.
///
/// This is metadata-only until the Span access planner lands. The fact records
/// the lifetime and storage contract that access plans must consume instead of
/// deriving Span legality from source spelling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpanBorrowFact {
    pub span_id: u32,
    pub span_value: ValueId,
    pub region_value: ValueId,
    pub owner_value: ValueId,
    pub mutability: SpanBorrowMutability,
    pub element_type: SpanElementType,
    pub start_value: ValueId,
    pub length_value: ValueId,
    pub scope_bb: BasicBlockId,
    pub no_escape: bool,
    pub owner_stable: bool,
    pub region_stability_fact_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanAccessOp {
    Load,
    Store,
}

impl SpanAccessOp {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Load => "load",
            Self::Store => "store",
        }
    }
}

/// Metadata-only Span access plan.
///
/// The first real planner will derive this from `SpanBorrowFact` plus the same
/// range/extent/stability proof vocabulary used by DirectArray.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpanAccessPlan {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub span_id: u32,
    pub op: SpanAccessOp,
    pub index_value: ValueId,
    pub value_value: Option<ValueId>,
    pub result_value: Option<ValueId>,
    pub element_type: SpanElementType,
    pub route: &'static str,
    pub bounds_policy: &'static str,
    pub proof_ids: Vec<&'static str>,
    pub fallback_policy: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredFastPathRegion {
    pub region_id: u32,
    pub source_kind: &'static str,
    pub relevant_access_policy: &'static str,
    pub route_requirement: &'static str,
    pub bounds_requirement: &'static str,
    pub fallback_policy: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastPathObligation {
    pub obligation_id: u32,
    pub region_id: u32,
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub access_kind: &'static str,
    pub op: &'static str,
    pub expected: &'static str,
    pub actual_plan_kind: Option<&'static str>,
    pub actual_route: Option<&'static str>,
    pub bounds_policy: Option<&'static str>,
    pub proof_ids: Vec<&'static str>,
    pub status: &'static str,
    pub failure_code: Option<&'static str>,
    pub failure_reason: Option<&'static str>,
}

/// Lower-bound extent proof for a DirectArray receiver value.
///
/// This is intentionally separate from `RangeIndexFact`: range facts prove the
/// index interval, while extent facts prove that a specific receiver can cover
/// that interval. DirectArray consumers must require both for unchecked
/// lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectArrayExtentFact {
    pub receiver_value: ValueId,
    pub lower_bound_value: ValueId,
    pub proof_kind: DirectArrayExtentProofKind,
    pub region_stability_fact_id: u32,
    pub stable_in_region: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_kinds_keep_stable_report_names() {
        assert_eq!(RangeIndexFactOriginKind::RangeLoop.as_str(), "range_loop");
        assert_eq!(
            RangeIndexFactOriginKind::ModuloOfRangeIndex.as_str(),
            "modulo_of_range_index"
        );
        assert_eq!(SpanAccessOp::Store.as_str(), "store");
    }

    #[test]
    fn direct_array_extent_fact_stays_passive_metadata() {
        let fact = DirectArrayExtentFact {
            receiver_value: ValueId(1),
            lower_bound_value: ValueId(2),
            proof_kind: DirectArrayExtentProofKind::ProducerInvariant,
            region_stability_fact_id: 7,
            stable_in_region: true,
        };

        assert_eq!(fact.proof_kind.as_str(), "producer_invariant");
        assert!(fact.stable_in_region);
    }
}
