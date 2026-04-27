use crate::mir::string_corridor::{
    StringCorridorBorrowContract, StringPublishReason, StringPublishReprPolicy,
    StringStableViewProvenance,
};
use crate::mir::ValueId;

/// Placement/effect decision kinds that later passes may act on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorCandidateKind {
    BorrowCorridorFusion,
    PublicationSink,
    MaterializationSink,
    DirectKernelEntry,
}

impl std::fmt::Display for StringCorridorCandidateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowCorridorFusion => f.write_str("borrowed_corridor_fusion"),
            Self::PublicationSink => f.write_str("publication_sink"),
            Self::MaterializationSink => f.write_str("materialization_sink"),
            Self::DirectKernelEntry => f.write_str("direct_kernel_entry"),
        }
    }
}

/// Whether the candidate is a future transform or already satisfied by current MIR facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorCandidateState {
    Candidate,
    AlreadySatisfied,
}

impl std::fmt::Display for StringCorridorCandidateState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Candidate => f.write_str("candidate"),
            Self::AlreadySatisfied => f.write_str("already_satisfied"),
        }
    }
}

/// Non-widening boundary where the current specialized corridor may publish.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorPublicationBoundary {
    FirstExternalBoundary,
}

impl std::fmt::Display for StringCorridorPublicationBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FirstExternalBoundary => f.write_str("first_external_boundary"),
        }
    }
}

/// Non-widening publication contract proven for the current corridor plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorPublicationContract {
    PublishNowNotRequiredBeforeFirstExternalBoundary,
}

impl std::fmt::Display for StringCorridorPublicationContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PublishNowNotRequiredBeforeFirstExternalBoundary => {
                f.write_str("publish_now_not_required_before_first_external_boundary")
            }
        }
    }
}

/// Proof-bearing plan metadata for broader string corridor routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringCorridorCandidatePlan {
    /// The borrowed carrier value that this plan is about.
    pub corridor_root: ValueId,
    /// Shared source root when the corridor proves a single underlying source.
    pub source_root: Option<ValueId>,
    /// MIR/lowering-owned object -> text provenance contract.
    pub borrow_contract: Option<StringCorridorBorrowContract>,
    /// Explicit publication reason for `publish.text(reason, repr)` when the plan already
    /// knows the boundary demand.
    pub publish_reason: Option<StringPublishReason>,
    /// Public representation policy selected for `publish.text`.
    pub publish_repr_policy: Option<StringPublishReprPolicy>,
    /// Provenance witness required before `publish.text(..., stable_view)` may escape.
    pub stable_view_provenance: Option<StringStableViewProvenance>,
    /// Outer consumer window when the candidate is itself a substring consumer.
    pub start: Option<ValueId>,
    pub end: Option<ValueId>,
    /// Known constant length contribution already proven in the corridor.
    pub known_length: Option<i64>,
    /// MIR-owned publication contract for the active corridor only.
    pub publication_contract: Option<StringCorridorPublicationContract>,
    /// Shape proof that explains why this value is on the corridor.
    pub proof: StringCorridorCandidateProof,
}

impl StringCorridorCandidatePlan {
    pub fn summary(&self) -> String {
        let source = self
            .source_root
            .map(|value| format!("%{}", value.0))
            .unwrap_or_else(|| "-".to_string());
        let borrow_contract = self
            .borrow_contract
            .map(|contract| contract.to_string())
            .unwrap_or_else(|| "-".to_string());
        let publish_reason = self
            .publish_reason
            .map(|reason| reason.to_string())
            .unwrap_or_else(|| "-".to_string());
        let publish_repr_policy = self
            .publish_repr_policy
            .map(|repr| repr.to_string())
            .unwrap_or_else(|| "-".to_string());
        let stable_view_provenance = self
            .stable_view_provenance
            .map(|provenance| provenance.to_string())
            .unwrap_or_else(|| "-".to_string());
        let outer_window = match (self.start, self.end) {
            (Some(start), Some(end)) => format!("[%{}, %{}]", start.0, end.0),
            _ => "-".to_string(),
        };
        let known_len = self
            .known_length
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string());
        let publication_contract = self
            .publication_contract
            .map(|contract| contract.to_string())
            .unwrap_or_else(|| "-".to_string());
        format!(
            "plan(root=%{} source={} borrow_contract={} publish_reason={} publish_repr_policy={} stable_view_provenance={} outer={} known_len={} publication_contract={} proof={})",
            self.corridor_root.0,
            source,
            borrow_contract,
            publish_reason,
            publish_repr_policy,
            stable_view_provenance,
            outer_window,
            known_len,
            publication_contract,
            self.proof.summary()
        )
    }
}

/// Proof payload attached to a string corridor candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorCandidateProof {
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

impl StringCorridorCandidateProof {
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

/// Inspection-only candidate record derived from current string corridor facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringCorridorCandidate {
    pub kind: StringCorridorCandidateKind,
    pub state: StringCorridorCandidateState,
    pub reason: &'static str,
    pub plan: Option<StringCorridorCandidatePlan>,
    pub publication_boundary: Option<StringCorridorPublicationBoundary>,
}

impl StringCorridorCandidate {
    pub fn summary(&self) -> String {
        let publication_boundary = self
            .publication_boundary
            .map(|boundary| format!(" boundary={boundary}"))
            .unwrap_or_default();
        match self.plan {
            Some(plan) => format!(
                "{} [{}] {}{} | {}",
                self.kind,
                self.state,
                self.reason,
                publication_boundary,
                plan.summary()
            ),
            None => format!(
                "{} [{}] {}{}",
                self.kind, self.state, self.reason, publication_boundary
            ),
        }
    }
}
