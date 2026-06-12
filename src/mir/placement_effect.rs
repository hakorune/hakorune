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

impl PlacementEffectStringProof {
    pub fn summary(&self) -> String {
        match self {
            Self::BorrowedSlice { source, start, end } => format!(
                "borrowed_slice(src=%{} start=%{} end=%{})",
                source.0, start.0, end.0
            ),
            Self::ConcatTriplet {
                left_value,
                left_source,
                left_start,
                left_end,
                middle,
                right_value,
                right_source,
                right_start,
                right_end,
                shared_source,
            } => format!(
                "concat_triplet(shared_source={} left_value={} left=%{}[%{},%{}] middle=%{} right_value={} right=%{}[%{},%{}])",
                shared_source,
                left_value
                    .map(|value| format!("%{}", value.0))
                    .unwrap_or_else(|| "-".to_string()),
                left_source.0,
                left_start.0,
                left_end.0,
                middle.0,
                right_value
                    .map(|value| format!("%{}", value.0))
                    .unwrap_or_else(|| "-".to_string()),
                right_source.0,
                right_start.0,
                right_end.0
            ),
        }
    }
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

impl PlacementEffectRoute {
    pub fn summary(&self) -> String {
        let block_suffix = self
            .block
            .map(|block| format!(" bb{}", block.as_u32()))
            .unwrap_or_else(|| " module".to_string());
        let instruction_suffix = self
            .instruction_index
            .map(|index| format!("#{index}"))
            .unwrap_or_default();
        let value_suffix = self
            .value
            .map(|value| format!(" value=%{}", value.as_u32()))
            .unwrap_or_default();
        let source_value_suffix = self
            .source_value
            .map(|value| format!(" source_value=%{}", value.as_u32()))
            .unwrap_or_default();
        let window_suffix = match (self.window_start, self.window_end) {
            (Some(start), Some(end)) => {
                format!(" window=[%{}, %{}]", start.as_u32(), end.as_u32())
            }
            _ => String::new(),
        };
        let borrow_contract_suffix = self
            .borrow_contract
            .map(|contract| format!(" borrow_contract={contract}"))
            .unwrap_or_default();
        let publish_reason_suffix = self
            .publish_reason
            .map(|reason| format!(" publish_reason={reason}"))
            .unwrap_or_default();
        let publish_repr_policy_suffix = self
            .publish_repr_policy
            .map(|repr| format!(" publish_repr_policy={repr}"))
            .unwrap_or_default();
        let stable_view_provenance_suffix = self
            .stable_view_provenance
            .map(|provenance| format!(" stable_view_provenance={provenance}"))
            .unwrap_or_default();
        let string_proof_suffix = self
            .string_proof
            .as_ref()
            .map(|proof| format!(" string_proof={}", proof.summary()))
            .unwrap_or_default();
        let publication_boundary_suffix = self
            .publication_boundary
            .map(|boundary| format!(" publication_boundary={boundary}"))
            .unwrap_or_default();
        let detail_suffix = self
            .detail
            .as_ref()
            .map(|detail| format!(" detail={detail}"))
            .unwrap_or_default();
        format!(
            "{}{} {} {} {} demand={} [{}]{}{}{}{}{}{}{}{}{}{} reason={}",
            block_suffix,
            instruction_suffix,
            self.source,
            self.subject,
            self.decision,
            self.demand,
            self.state,
            value_suffix,
            source_value_suffix,
            window_suffix,
            borrow_contract_suffix,
            publish_reason_suffix,
            publish_repr_policy_suffix,
            stable_view_provenance_suffix,
            string_proof_suffix,
            publication_boundary_suffix,
            detail_suffix,
            self.reason
        )
    }
}

#[cfg(test)]
mod tests;
