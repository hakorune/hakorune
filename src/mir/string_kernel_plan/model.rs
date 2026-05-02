use crate::mir::string_corridor::{
    StringPublishReason, StringPublishReprPolicy, StringStableViewProvenance,
};
use crate::mir::string_corridor_placement::{
    StringCorridorCandidateProof, StringCorridorCandidateState,
};
use crate::mir::ValueId;

/// Backend-consumable family names derived from string corridor candidate plans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanFamily {
    BorrowedSliceWindow,
    ConcatTripletWindow,
}

impl std::fmt::Display for StringKernelPlanFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowedSliceWindow => f.write_str("borrowed_slice_window"),
            Self::ConcatTripletWindow => f.write_str("concat_triplet_window"),
        }
    }
}

/// Current retained-form names exported to backend consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanRetainedForm {
    BorrowedText,
}

impl std::fmt::Display for StringKernelPlanRetainedForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowedText => f.write_str("borrowed_text"),
        }
    }
}

/// Backend consumer role selected from current candidate families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanConsumer {
    DirectKernelEntry,
}

impl std::fmt::Display for StringKernelPlanConsumer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirectKernelEntry => f.write_str("direct_kernel_entry"),
        }
    }
}

/// Direct-kernel text consumer rule derived from the current MIR uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanTextConsumer {
    SlotText,
    ExplicitColdPublish,
}

impl std::fmt::Display for StringKernelPlanTextConsumer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SlotText => f.write_str("slot_text"),
            Self::ExplicitColdPublish => f.write_str("explicit_cold_publish"),
        }
    }
}

/// Runtime-private direct-kernel carrier selected by MIR/lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanCarrier {
    KernelTextSlot,
    RegistryBackedHandle,
}

impl std::fmt::Display for StringKernelPlanCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KernelTextSlot => f.write_str("kernel_text_slot"),
            Self::RegistryBackedHandle => f.write_str("registry_backed_handle"),
        }
    }
}

/// Backend-consumable borrow/provenance contract for object -> text entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanBorrowContract {
    BorrowTextFromObject,
}

impl std::fmt::Display for StringKernelPlanBorrowContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowTextFromObject => f.write_str("borrow_text_from_obj"),
        }
    }
}

/// Owner responsible for legality verification on the current direct-kernel lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanVerifierOwner {
    LoweringDirectKernelEntry,
}

impl std::fmt::Display for StringKernelPlanVerifierOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoweringDirectKernelEntry => f.write_str("lowering_direct_kernel_entry"),
        }
    }
}

/// Backend-consumable publication boundary for a string kernel plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanPublicationBoundary {
    FirstExternalBoundary,
}

impl std::fmt::Display for StringKernelPlanPublicationBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FirstExternalBoundary => f.write_str("first_external_boundary"),
        }
    }
}

/// Backend-consumable MIR proof that publication may stay deferred on this plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanPublicationContract {
    PublishNowNotRequiredBeforeFirstExternalBoundary,
}

impl std::fmt::Display for StringKernelPlanPublicationContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PublishNowNotRequiredBeforeFirstExternalBoundary => {
                f.write_str("publish_now_not_required_before_first_external_boundary")
            }
        }
    }
}

/// Backend-consumable string kernel plan part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringKernelPlanPart {
    Slice {
        value: Option<ValueId>,
        source: ValueId,
        start: ValueId,
        end: ValueId,
    },
    Const {
        value: ValueId,
        known_length: Option<i64>,
        literal: Option<String>,
    },
}

/// Narrow scalar payload for the current substring-concat exact loop route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringKernelPlanLoopPayload {
    pub seed_value: ValueId,
    pub seed_literal: String,
    pub seed_length: i64,
    pub loop_bound: i64,
    pub split_length: i64,
}

/// Thin legality facts that backend consumers may check before emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringKernelPlanLegality {
    pub byte_exact: bool,
    pub no_publish_inside: bool,
    pub requires_kernel_text_slot: bool,
    pub rejects_early_stable_box_now: bool,
    pub rejects_early_fresh_registry_handle: bool,
    pub rejects_registry_backed_carrier: bool,
}

/// MIR-owned read-side alias continuation facts.
///
/// Backend shims may consume these facts, but must not re-own legality for
/// follow-up substring / piecewise / shared-receiver continuation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StringKernelPlanReadAliasFacts {
    pub same_receiver: bool,
    pub source_window: bool,
    pub followup_substring: bool,
    pub piecewise_subrange: bool,
    pub direct_set_consumer: bool,
    pub shared_receiver: bool,
}

/// MIR-owned slot-hop continuation route.
///
/// This is the last same-block consumer that backend shims used to rediscover
/// with JSON/callee matching. The shim may still apply the skip list, but the
/// continuation decision and substring window are MIR metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringKernelPlanSlotHopSubstring {
    pub consumer_value: ValueId,
    pub start: ValueId,
    pub end: ValueId,
    pub instruction_index: usize,
    pub copy_instruction_indices: Vec<usize>,
}

/// Thin backend-consumable kernel plan derived from the current candidate set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringKernelPlan {
    pub plan_value: ValueId,
    pub version: u32,
    pub family: StringKernelPlanFamily,
    pub corridor_root: ValueId,
    pub source_root: Option<ValueId>,
    pub borrow_contract: Option<StringKernelPlanBorrowContract>,
    pub publish_reason: Option<StringPublishReason>,
    pub publish_repr_policy: Option<StringPublishReprPolicy>,
    pub stable_view_provenance: Option<StringStableViewProvenance>,
    pub known_length: Option<i64>,
    pub retained_form: StringKernelPlanRetainedForm,
    pub publication_boundary: Option<StringKernelPlanPublicationBoundary>,
    pub publication_contract: Option<StringKernelPlanPublicationContract>,
    pub publication: Option<StringCorridorCandidateState>,
    pub materialization: Option<StringCorridorCandidateState>,
    pub direct_kernel_entry: Option<StringCorridorCandidateState>,
    pub consumer: Option<StringKernelPlanConsumer>,
    pub text_consumer: Option<StringKernelPlanTextConsumer>,
    pub carrier: Option<StringKernelPlanCarrier>,
    pub verifier_owner: Option<StringKernelPlanVerifierOwner>,
    pub read_alias: StringKernelPlanReadAliasFacts,
    pub slot_hop_substring: Option<StringKernelPlanSlotHopSubstring>,
    pub proof: StringCorridorCandidateProof,
    pub middle_literal: Option<String>,
    pub loop_payload: Option<StringKernelPlanLoopPayload>,
}

impl StringKernelPlan {
    pub fn parts(&self) -> Vec<StringKernelPlanPart> {
        match self.proof {
            StringCorridorCandidateProof::BorrowedSlice { source, start, end } => {
                vec![StringKernelPlanPart::Slice {
                    value: None,
                    source,
                    start,
                    end,
                }]
            }
            StringCorridorCandidateProof::ConcatTriplet {
                left_value,
                left_source,
                left_start,
                left_end,
                middle,
                right_value,
                right_source,
                right_start,
                right_end,
                shared_source: _,
            } => vec![
                StringKernelPlanPart::Slice {
                    value: left_value,
                    source: left_source,
                    start: left_start,
                    end: left_end,
                },
                StringKernelPlanPart::Const {
                    value: middle,
                    known_length: self.known_length,
                    literal: self.middle_literal.clone(),
                },
                StringKernelPlanPart::Slice {
                    value: right_value,
                    source: right_source,
                    start: right_start,
                    end: right_end,
                },
            ],
        }
    }

    pub fn legality(&self) -> StringKernelPlanLegality {
        let requires_kernel_text_slot = self.text_consumer.is_some();
        let reject_early_publish = self.publication_contract.is_some() && requires_kernel_text_slot;
        StringKernelPlanLegality {
            byte_exact: true,
            no_publish_inside: self.publication_contract.is_some(),
            requires_kernel_text_slot,
            rejects_early_stable_box_now: reject_early_publish,
            rejects_early_fresh_registry_handle: reject_early_publish,
            rejects_registry_backed_carrier: reject_early_publish,
        }
    }
}
