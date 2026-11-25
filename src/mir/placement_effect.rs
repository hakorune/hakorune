/*!
 * Folded generic placement/effect owner seam.
 *
 * This module keeps the placement/effect value types and delegates route
 * inventory assembly to a sibling module.
 */

use super::string_corridor::{
    StringPublishReason, StringPublishReprPolicy, StringStableViewProvenance,
};
use super::{BasicBlockId, ValueId};

mod routes;

pub use routes::{
    refresh_function_placement_effect_routes, refresh_module_placement_effect_routes,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementEffectSource {
    StringCorridor,
    SumPlacement,
    AggLocalScalarization,
    ThinEntry,
}

impl std::fmt::Display for PlacementEffectSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringCorridor => f.write_str("string_corridor"),
            Self::SumPlacement => f.write_str("sum_placement"),
            Self::AggLocalScalarization => f.write_str("agg_local_scalarization"),
            Self::ThinEntry => f.write_str("thin_entry"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementEffectDecision {
    StayBorrowed,
    PublishHandle,
    MaterializeOwned,
    DirectKernelEntry,
    LocalAggregate,
    CompatRuntimeBox,
    PublicEntry,
    ThinInternalEntry,
}

impl std::fmt::Display for PlacementEffectDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StayBorrowed => f.write_str("stay_borrowed"),
            Self::PublishHandle => f.write_str("publish_handle"),
            Self::MaterializeOwned => f.write_str("materialize_owned"),
            Self::DirectKernelEntry => f.write_str("direct_kernel_entry"),
            Self::LocalAggregate => f.write_str("local_aggregate"),
            Self::CompatRuntimeBox => f.write_str("compat_runtime_box"),
            Self::PublicEntry => f.write_str("public_entry"),
            Self::ThinInternalEntry => f.write_str("thin_internal_entry"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementEffectState {
    Candidate,
    Selected,
    AlreadySatisfied,
}

impl std::fmt::Display for PlacementEffectState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Candidate => f.write_str("candidate"),
            Self::Selected => f.write_str("selected"),
            Self::AlreadySatisfied => f.write_str("already_satisfied"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementEffectDemand {
    Unknown,
    ReadRef,
    OwnedPayload,
    CellResidence,
    Immediate,
    PublishHandle,
    StableObject,
    LocalAggregate,
}

impl std::fmt::Display for PlacementEffectDemand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => f.write_str("?"),
            Self::ReadRef => f.write_str("read_ref"),
            Self::OwnedPayload => f.write_str("owned_payload"),
            Self::CellResidence => f.write_str("cell_residence"),
            Self::Immediate => f.write_str("immediate"),
            Self::PublishHandle => f.write_str("publish_handle"),
            Self::StableObject => f.write_str("stable_object"),
            Self::LocalAggregate => f.write_str("local_aggregate"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementEffectPublicationBoundary {
    FirstExternalBoundary,
}

impl std::fmt::Display for PlacementEffectPublicationBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FirstExternalBoundary => f.write_str("first_external_boundary"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementEffectBorrowContract {
    BorrowTextFromObject,
}

impl std::fmt::Display for PlacementEffectBorrowContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowTextFromObject => f.write_str("borrow_text_from_obj"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementEffectStringProof {
    BorrowedSlice {
        source: ValueId,
        start: ValueId,
        end: ValueId,
    },
    ConcatTriplet {
        left_value: Option<ValueId>,
        left_source: ValueId,
        left_start: ValueId,
        left_end: ValueId,
        middle: ValueId,
        right_value: Option<ValueId>,
        right_source: ValueId,
        right_start: ValueId,
        right_end: ValueId,
        shared_source: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementEffectRoute {
    pub block: Option<BasicBlockId>,
    pub instruction_index: Option<usize>,
    pub value: Option<ValueId>,
    pub source_value: Option<ValueId>,
    pub window_start: Option<ValueId>,
    pub window_end: Option<ValueId>,
    pub borrow_contract: Option<PlacementEffectBorrowContract>,
    pub publish_reason: Option<StringPublishReason>,
    pub publish_repr_policy: Option<StringPublishReprPolicy>,
    pub stable_view_provenance: Option<StringStableViewProvenance>,
    pub string_proof: Option<PlacementEffectStringProof>,
    pub publication_boundary: Option<PlacementEffectPublicationBoundary>,
    pub source: PlacementEffectSource,
    pub subject: String,
    pub decision: PlacementEffectDecision,
    pub demand: PlacementEffectDemand,
    pub state: PlacementEffectState,
    pub detail: Option<String>,
    pub reason: String,
}

#[cfg(test)]
mod tests;
