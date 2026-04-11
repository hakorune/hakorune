/*!
 * Backend-consumable string kernel plan seam.
 *
 * This module owns the thin derived view that backend/export consumers read.
 * It is downstream of string corridor candidates and upstream of JSON/shim
 * transport. Placement remains the owner of candidate metadata itself.
 */

use super::{
    string_corridor_placement::{
        StringCorridorCandidate, StringCorridorCandidateKind, StringCorridorCandidateProof,
        StringCorridorCandidateState,
    },
    ValueId,
};

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

/// Backend-consumable string kernel plan part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    },
}

/// Thin legality facts that backend consumers may check before emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringKernelPlanLegality {
    pub byte_exact: bool,
    pub no_publish_inside: bool,
}

/// Thin backend-consumable kernel plan derived from the current candidate set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringKernelPlan {
    pub version: u32,
    pub family: StringKernelPlanFamily,
    pub corridor_root: ValueId,
    pub source_root: Option<ValueId>,
    pub known_length: Option<i64>,
    pub retained_form: StringKernelPlanRetainedForm,
    pub publication: Option<StringCorridorCandidateState>,
    pub materialization: Option<StringCorridorCandidateState>,
    pub direct_kernel_entry: Option<StringCorridorCandidateState>,
    pub consumer: Option<StringKernelPlanConsumer>,
    pub proof: StringCorridorCandidateProof,
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
        StringKernelPlanLegality {
            byte_exact: true,
            no_publish_inside: self.publication.is_some(),
        }
    }
}

fn candidate_priority(kind: StringCorridorCandidateKind) -> u8 {
    match kind {
        StringCorridorCandidateKind::DirectKernelEntry => 0,
        StringCorridorCandidateKind::PublicationSink => 1,
        StringCorridorCandidateKind::MaterializationSink => 2,
        StringCorridorCandidateKind::BorrowCorridorFusion => 3,
    }
}

/// Derive a backend-consumable string kernel plan from current candidate metadata.
pub fn derive_string_kernel_plan(
    candidates: &[StringCorridorCandidate],
) -> Option<StringKernelPlan> {
    let mut representative: Option<StringCorridorCandidate> = None;
    let mut publication = None;
    let mut materialization = None;
    let mut direct_kernel_entry = None;

    for candidate in candidates {
        match candidate.kind {
            StringCorridorCandidateKind::PublicationSink => publication = Some(candidate.state),
            StringCorridorCandidateKind::MaterializationSink => {
                materialization = Some(candidate.state)
            }
            StringCorridorCandidateKind::DirectKernelEntry => {
                direct_kernel_entry = Some(candidate.state)
            }
            StringCorridorCandidateKind::BorrowCorridorFusion => {}
        }

        let Some(plan) = candidate.plan else {
            continue;
        };
        representative = match representative {
            Some(current)
                if current.plan.is_some()
                    && candidate_priority(current.kind) <= candidate_priority(candidate.kind) =>
            {
                Some(current)
            }
            _ => Some(StringCorridorCandidate {
                kind: candidate.kind,
                state: candidate.state,
                reason: candidate.reason,
                plan: Some(plan),
            }),
        };
    }

    let representative = representative?;
    let plan = representative.plan?;
    let family = match plan.proof {
        StringCorridorCandidateProof::BorrowedSlice { .. } => {
            StringKernelPlanFamily::BorrowedSliceWindow
        }
        StringCorridorCandidateProof::ConcatTriplet { .. } => {
            StringKernelPlanFamily::ConcatTripletWindow
        }
    };

    Some(StringKernelPlan {
        version: 1,
        family,
        corridor_root: plan.corridor_root,
        source_root: plan.source_root,
        known_length: plan.known_length,
        retained_form: StringKernelPlanRetainedForm::BorrowedText,
        publication,
        materialization,
        direct_kernel_entry,
        consumer: direct_kernel_entry.map(|_| StringKernelPlanConsumer::DirectKernelEntry),
        proof: plan.proof,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_string_kernel_plan_prefers_direct_entry_and_collects_barriers() {
        let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(7),
            source_root: Some(ValueId::new(1)),
            start: Some(ValueId::new(2)),
            end: Some(ValueId::new(3)),
            known_length: Some(2),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(4)),
                left_source: ValueId::new(1),
                left_start: ValueId::new(4),
                left_end: ValueId::new(5),
                middle: ValueId::new(6),
                right_value: Some(ValueId::new(8)),
                right_source: ValueId::new(1),
                right_start: ValueId::new(5),
                right_end: ValueId::new(9),
                shared_source: true,
            },
        };
        let candidates = vec![
            StringCorridorCandidate {
                kind: StringCorridorCandidateKind::PublicationSink,
                state: StringCorridorCandidateState::AlreadySatisfied,
                reason: "publish boundary is already sunk at the current corridor exit",
                plan: Some(plan),
            },
            StringCorridorCandidate {
                kind: StringCorridorCandidateKind::MaterializationSink,
                state: StringCorridorCandidateState::Candidate,
                reason: "slice result may stay borrowed until a later boundary",
                plan: Some(plan),
            },
            StringCorridorCandidate {
                kind: StringCorridorCandidateKind::DirectKernelEntry,
                state: StringCorridorCandidateState::Candidate,
                reason:
                    "borrowed slice corridor can target a direct kernel entry before publication",
                plan: Some(plan),
            },
        ];

        let kernel_plan = derive_string_kernel_plan(&candidates).expect("kernel plan");

        assert_eq!(kernel_plan.version, 1);
        assert_eq!(
            kernel_plan.family,
            StringKernelPlanFamily::ConcatTripletWindow
        );
        assert_eq!(kernel_plan.corridor_root, ValueId::new(7));
        assert_eq!(kernel_plan.source_root, Some(ValueId::new(1)));
        assert_eq!(kernel_plan.known_length, Some(2));
        assert_eq!(
            kernel_plan.retained_form,
            StringKernelPlanRetainedForm::BorrowedText
        );
        assert_eq!(
            kernel_plan.publication,
            Some(StringCorridorCandidateState::AlreadySatisfied)
        );
        assert_eq!(
            kernel_plan.materialization,
            Some(StringCorridorCandidateState::Candidate)
        );
        assert_eq!(
            kernel_plan.direct_kernel_entry,
            Some(StringCorridorCandidateState::Candidate)
        );
        assert_eq!(
            kernel_plan.consumer,
            Some(StringKernelPlanConsumer::DirectKernelEntry)
        );
        let parts = kernel_plan.parts();
        assert_eq!(parts.len(), 3);
        assert_eq!(
            kernel_plan.legality(),
            StringKernelPlanLegality {
                byte_exact: true,
                no_publish_inside: true,
            }
        );
    }
}
