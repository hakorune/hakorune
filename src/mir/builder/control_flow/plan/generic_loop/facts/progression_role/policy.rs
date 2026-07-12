//! Neutral closed progression-proof policy.
//!
//! Candidate names are removed before this policy is invoked. C1 proves only
//! structural role hypotheses; Recipe verification and acceptance are later.

use super::branch_control::{CandidateControlAnchorsV0, LoopControlAnchorV0};
use super::observation::{CandidateObservationV0, CandidateSiteV0};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct NormalizedCandidateObservationV0 {
    pub condition_anchored: bool,
    pub writes: Vec<CandidateSiteV0>,
    pub canonical_step_sites: Vec<CandidateSiteV0>,
    pub conditional_writes: Vec<CandidateSiteV0>,
}

impl From<&CandidateObservationV0> for NormalizedCandidateObservationV0 {
    fn from(observation: &CandidateObservationV0) -> Self {
        Self {
            condition_anchored: observation.condition_anchored,
            writes: observation.writes.clone(),
            canonical_step_sites: observation.canonical_step_sites.clone(),
            conditional_writes: observation.conditional_writes.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopProgressionProofV0 {
    ConditionAnchoredCanonical {
        step_site: CandidateSiteV0,
    },
    ConditionAnchoredBodyManaged {
        write_sites: Vec<CandidateSiteV0>,
    },
    ControlAnchoredCanonical {
        control_site: LoopControlAnchorV0,
        step_site: CandidateSiteV0,
    },
    ControlAnchoredBodyManaged {
        control_site: LoopControlAnchorV0,
        write_sites: Vec<CandidateSiteV0>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CandidateProofOutcomeV0 {
    Proven(LoopProgressionProofV0),
    Excluded(&'static str),
    Unproven(&'static str),
}

pub(in crate::mir::builder) fn prove_candidate_progression_v0(
    observation: &NormalizedCandidateObservationV0,
    control: &CandidateControlAnchorsV0,
) -> CandidateProofOutcomeV0 {
    if observation.writes.is_empty() {
        return CandidateProofOutcomeV0::Excluded("candidate.write_free");
    }

    let canonical_step =
        if observation.writes.len() == 1 && observation.canonical_step_sites.len() == 1 {
            observation.canonical_step_sites.first().cloned()
        } else {
            None
        };

    if observation.condition_anchored {
        return CandidateProofOutcomeV0::Proven(match canonical_step {
            Some(step_site) => LoopProgressionProofV0::ConditionAnchoredCanonical { step_site },
            None => LoopProgressionProofV0::ConditionAnchoredBodyManaged {
                write_sites: observation.writes.clone(),
            },
        });
    }

    let Some(control_site) = control.anchors.first().copied() else {
        return CandidateProofOutcomeV0::Unproven("candidate.control_anchor_missing");
    };
    CandidateProofOutcomeV0::Proven(match canonical_step {
        Some(step_site) => LoopProgressionProofV0::ControlAnchoredCanonical {
            control_site,
            step_site,
        },
        None => LoopProgressionProofV0::ControlAnchoredBodyManaged {
            control_site,
            write_sites: observation.writes.clone(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::{
        prove_candidate_progression_v0, CandidateProofOutcomeV0, NormalizedCandidateObservationV0,
    };
    use crate::mir::builder::control_flow::plan::generic_loop::facts::progression_role::{
        branch_control::{
            CandidateControlAnchorsV0, ControlConditionSiteV0, GuardBranchV0, LoopControlAnchorV0,
        },
        observation::CandidateSiteV0,
    };

    fn site(index: usize) -> CandidateSiteV0 {
        CandidateSiteV0 {
            preorder_index: index,
            top_level_stmt_index: index,
            conditional: false,
        }
    }

    fn observation(condition_anchored: bool) -> NormalizedCandidateObservationV0 {
        NormalizedCandidateObservationV0 {
            condition_anchored,
            writes: vec![site(1)],
            canonical_step_sites: vec![site(1)],
            conditional_writes: Vec::new(),
        }
    }

    fn exit_anchor() -> CandidateControlAnchorsV0 {
        CandidateControlAnchorsV0 {
            anchors: vec![LoopControlAnchorV0::CurrentLoopExitGuard {
                condition_site: ControlConditionSiteV0 {
                    top_level_stmt_index: 0,
                    condition_preorder_index: 0,
                },
                branch: GuardBranchV0::Then,
            }],
        }
    }

    #[test]
    fn condition_anchor_proves_canonical_candidate() {
        assert!(matches!(
            prove_candidate_progression_v0(
                &observation(true),
                &CandidateControlAnchorsV0 {
                    anchors: Vec::new()
                }
            ),
            CandidateProofOutcomeV0::Proven(_)
        ));
    }

    #[test]
    fn control_anchor_proves_body_managed_candidate() {
        let mut observed = observation(false);
        observed.writes.push(site(2));
        observed.canonical_step_sites.clear();
        assert!(matches!(
            prove_candidate_progression_v0(&observed, &exit_anchor()),
            CandidateProofOutcomeV0::Proven(_)
        ));
    }

    #[test]
    fn missing_anchor_is_unproven() {
        assert_eq!(
            prove_candidate_progression_v0(
                &observation(false),
                &CandidateControlAnchorsV0 {
                    anchors: Vec::new()
                }
            ),
            CandidateProofOutcomeV0::Unproven("candidate.control_anchor_missing")
        );
    }

    #[test]
    fn write_free_candidate_is_excluded() {
        let mut observed = observation(true);
        observed.writes.clear();
        observed.canonical_step_sites.clear();
        assert_eq!(
            prove_candidate_progression_v0(&observed, &exit_anchor()),
            CandidateProofOutcomeV0::Excluded("candidate.write_free")
        );
    }
}
