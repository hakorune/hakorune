//! Acceptance-neutral candidate observation reports.
//!
//! C0 records evidence captured on an isolated probe branch. These report-only
//! classifications are not imported by Facts, Recipe, selection, or Lower.

use std::collections::BTreeMap;

use super::observation::{CandidateObservationV0, CandidateSiteV0};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum CandidateDiscoverySourceV0 {
    HeaderCondition,
    ExistingTrueLoopIncrement,
    BodyAssignment,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) struct CandidateIdV0 {
    pub discovery_source: CandidateDiscoverySourceV0,
    pub first_write_site: Option<CandidateSiteV0>,
    pub discovery_ordinal_within_path: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum ProvisionalBodyManagedProfileV0 {
    Rebased,
    MultipleWrites,
    PostUpdateUse,
    Mixed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ProvisionalUpdateShapeV0 {
    CanonicalInduction,
    BodyManagedCursor {
        update_profile: ProvisionalBodyManagedProfileV0,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ProvisionalRoleOutcomeV0 {
    Accepted(ProvisionalUpdateShapeV0),
    NotCandidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum ProvisionalEvidenceRankV0 {
    LocalUpdateOnly,
    BodyStateAcrossStatements,
    CanonicalRecurrenceObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CandidateComparisonV0 {
    Tied,
    Unproven,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CandidateDecisionRowV0 {
    pub diagnostic_label: String,
    pub candidate_id: CandidateIdV0,
    pub discovery_source: CandidateDiscoverySourceV0,
    pub observation: CandidateObservationV0,
    pub provisional_role: ProvisionalRoleOutcomeV0,
    pub provisional_rank: ProvisionalEvidenceRankV0,
    pub comparison: CandidateComparisonV0,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CandidateSelectionReportV0 {
    pub rows: Vec<CandidateDecisionRowV0>,
    pub final_outcome: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CandidateStructureV0 {
    condition_anchored: bool,
    existing_true_loop_increment_derived: bool,
    writes: Vec<CandidateSiteV0>,
    canonical_step_sites: Vec<CandidateSiteV0>,
    uses_outside_canonical_step: Vec<CandidateSiteV0>,
    post_update_uses: Vec<CandidateSiteV0>,
    conditional_writes: Vec<CandidateSiteV0>,
}

impl CandidateStructureV0 {
    fn from_observation(observation: &CandidateObservationV0) -> Self {
        Self {
            condition_anchored: observation.condition_anchored,
            existing_true_loop_increment_derived: observation.existing_true_loop_increment_derived,
            writes: observation.writes.clone(),
            canonical_step_sites: observation.canonical_step_sites.clone(),
            uses_outside_canonical_step: observation.uses_outside_canonical_step.clone(),
            post_update_uses: observation.post_update_uses.clone(),
            conditional_writes: observation.conditional_writes.clone(),
        }
    }
}

/// Builds a normalized report for a captured Multiple outcome.
///
/// This API intentionally accepts no selector, Recipe, or lowering result.
pub(in crate::mir::builder) fn capture_multiple_candidate_report_v0(
    observations: &[CandidateObservationV0],
) -> CandidateSelectionReportV0 {
    let identities = structural_identities(observations);
    let mut rows: Vec<_> = observations
        .iter()
        .enumerate()
        .map(|(index, observation)| {
            let provisional_role = provisional_update_shape(observation);
            CandidateDecisionRowV0 {
                diagnostic_label: observation.candidate.clone(),
                candidate_id: identities[index].clone(),
                discovery_source: discovery_source(observation),
                observation: observation.clone(),
                provisional_rank: provisional_evidence_rank(observation),
                comparison: if matches!(provisional_role, ProvisionalRoleOutcomeV0::Accepted(_)) {
                    CandidateComparisonV0::Tied
                } else {
                    CandidateComparisonV0::Unproven
                },
                provisional_role,
            }
        })
        .collect();
    rows.sort_by(|left, right| {
        left.candidate_id
            .cmp(&right.candidate_id)
            .then_with(|| {
                CandidateStructureV0::from_observation(&left.observation)
                    .cmp(&CandidateStructureV0::from_observation(&right.observation))
            })
            // Diagnostic presentation only; policy never imports this module.
            .then_with(|| left.diagnostic_label.cmp(&right.diagnostic_label))
    });
    CandidateSelectionReportV0 {
        rows,
        final_outcome: "Multiple",
    }
}

#[cfg(test)]
impl CandidateSelectionReportV0 {
    pub(in crate::mir::builder) fn stable_text(&self) -> String {
        let rows = self
            .rows
            .iter()
            .map(stable_row_text)
            .collect::<Vec<_>>()
            .join(";");
        format!("outcome={} rows=[{}]", self.final_outcome, rows)
    }
}

fn provisional_update_shape(observation: &CandidateObservationV0) -> ProvisionalRoleOutcomeV0 {
    if (!observation.condition_anchored && !observation.existing_true_loop_increment_derived)
        || observation.writes.is_empty()
    {
        return ProvisionalRoleOutcomeV0::NotCandidate;
    }
    if observation.writes.len() == 1
        && observation.canonical_step_sites.len() == 1
        && observation.post_update_uses.is_empty()
        && observation.conditional_writes.is_empty()
    {
        return ProvisionalRoleOutcomeV0::Accepted(ProvisionalUpdateShapeV0::CanonicalInduction);
    }

    let rebased = observation.canonical_step_sites.is_empty();
    let multiple_writes = observation.writes.len() > 1;
    let post_update_use = !observation.post_update_uses.is_empty();
    let conditional_write = !observation.conditional_writes.is_empty();
    let signal_count = usize::from(rebased)
        + usize::from(multiple_writes)
        + usize::from(post_update_use)
        + usize::from(conditional_write);
    let update_profile = if signal_count > 1 {
        ProvisionalBodyManagedProfileV0::Mixed
    } else if multiple_writes || conditional_write {
        ProvisionalBodyManagedProfileV0::MultipleWrites
    } else if post_update_use {
        ProvisionalBodyManagedProfileV0::PostUpdateUse
    } else {
        ProvisionalBodyManagedProfileV0::Rebased
    };
    ProvisionalRoleOutcomeV0::Accepted(ProvisionalUpdateShapeV0::BodyManagedCursor {
        update_profile,
    })
}

fn provisional_evidence_rank(observation: &CandidateObservationV0) -> ProvisionalEvidenceRankV0 {
    if !observation.canonical_step_sites.is_empty()
        && !observation.uses_outside_canonical_step.is_empty()
    {
        return ProvisionalEvidenceRankV0::CanonicalRecurrenceObserved;
    }
    let mut statement_indices: Vec<_> = observation
        .uses_outside_canonical_step
        .iter()
        .map(|site| site.top_level_stmt_index)
        .collect();
    statement_indices.sort_unstable();
    statement_indices.dedup();
    if statement_indices.len() > 1 {
        ProvisionalEvidenceRankV0::BodyStateAcrossStatements
    } else {
        ProvisionalEvidenceRankV0::LocalUpdateOnly
    }
}

fn structural_identities(observations: &[CandidateObservationV0]) -> Vec<CandidateIdV0> {
    let mut groups: BTreeMap<
        (CandidateDiscoverySourceV0, Option<CandidateSiteV0>),
        Vec<CandidateStructureV0>,
    > = BTreeMap::new();
    for observation in observations {
        groups
            .entry((
                discovery_source(observation),
                observation.writes.first().cloned(),
            ))
            .or_default()
            .push(CandidateStructureV0::from_observation(observation));
    }
    for structures in groups.values_mut() {
        structures.sort();
        structures.dedup();
    }
    observations
        .iter()
        .map(|observation| {
            let source = discovery_source(observation);
            let first_write_site = observation.writes.first().cloned();
            let structure = CandidateStructureV0::from_observation(observation);
            let ordinal = groups
                .get(&(source, first_write_site.clone()))
                .and_then(|structures| structures.binary_search(&structure).ok())
                .expect("captured candidate structure must have a structural identity");
            CandidateIdV0 {
                discovery_source: source,
                first_write_site,
                discovery_ordinal_within_path: ordinal,
            }
        })
        .collect()
}

fn discovery_source(observation: &CandidateObservationV0) -> CandidateDiscoverySourceV0 {
    if observation.condition_anchored {
        CandidateDiscoverySourceV0::HeaderCondition
    } else if observation.existing_true_loop_increment_derived {
        CandidateDiscoverySourceV0::ExistingTrueLoopIncrement
    } else {
        CandidateDiscoverySourceV0::BodyAssignment
    }
}

#[cfg(test)]
fn stable_row_text(row: &CandidateDecisionRowV0) -> String {
    format!(
        "label={}|id={:?}:{:?}:{}|source={:?}|cond={}|true_inc={}|writes={}|canon={}|nonstep={}|post={}|conditional={}|role={:?}|rank={:?}|comparison={:?}",
        row.diagnostic_label,
        row.candidate_id.discovery_source,
        row.candidate_id.first_write_site,
        row.candidate_id.discovery_ordinal_within_path,
        row.discovery_source,
        row.observation.condition_anchored,
        row.observation.existing_true_loop_increment_derived,
        sites_text(&row.observation.writes),
        sites_text(&row.observation.canonical_step_sites),
        sites_text(&row.observation.uses_outside_canonical_step),
        sites_text(&row.observation.post_update_uses),
        sites_text(&row.observation.conditional_writes),
        row.provisional_role,
        row.provisional_rank,
        row.comparison,
    )
}

#[cfg(test)]
fn sites_text(sites: &[CandidateSiteV0]) -> String {
    sites
        .iter()
        .map(|site| {
            format!(
                "{}:{}:{}",
                site.top_level_stmt_index,
                site.preorder_index,
                usize::from(site.conditional)
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::capture_multiple_candidate_report_v0;
    use crate::mir::builder::control_flow::plan::generic_loop::facts::progression_role::observation::{
        CandidateObservationV0, CandidateSiteV0,
    };

    fn observation(label: &str, stmt: usize) -> CandidateObservationV0 {
        let site = CandidateSiteV0 {
            preorder_index: 1,
            top_level_stmt_index: stmt,
            conditional: false,
        };
        CandidateObservationV0 {
            candidate: label.to_string(),
            condition_anchored: true,
            existing_true_loop_increment_derived: false,
            writes: vec![site.clone()],
            uses: Vec::new(),
            canonical_step_sites: vec![site],
            uses_outside_canonical_step: Vec::new(),
            post_update_uses: Vec::new(),
            conditional_writes: Vec::new(),
        }
    }

    #[test]
    fn report_is_independent_of_evaluation_order() {
        let left = vec![observation("z", 3), observation("a", 1)];
        let right = vec![observation("a", 1), observation("z", 3)];
        let left_report = capture_multiple_candidate_report_v0(&left);
        let right_report = capture_multiple_candidate_report_v0(&right);
        assert_eq!(left_report, right_report);
        assert!(left_report
            .stable_text()
            .starts_with("outcome=Multiple rows=["));
    }

    #[test]
    fn diagnostic_label_does_not_change_structural_identity() {
        let first = capture_multiple_candidate_report_v0(&[observation("left", 2)]);
        let second = capture_multiple_candidate_report_v0(&[observation("renamed", 2)]);
        assert_eq!(first.rows[0].candidate_id, second.rows[0].candidate_id);
    }
}
