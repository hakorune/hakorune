//! Phase 29ai P3: CandidateSet — SSOT implementation
//!
//! SSOT: "0 candidates = Ok(None), 1 = Ok(Some), 2+ = Freeze(ambiguous)"

#![allow(dead_code)]

use super::pattern_shadow;
use super::Freeze;
use crate::mir::builder::control_flow::plan::DomainPlan;
use crate::mir::builder::control_flow::plan::trace;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct PlanCandidate {
    pub plan: DomainPlan,
    pub rule: &'static str,
}

#[derive(Debug, Default)]
pub(in crate::mir::builder) struct CandidateSet {
    candidates: Vec<PlanCandidate>,
}

impl CandidateSet {
    pub(in crate::mir::builder) fn new() -> Self {
        Self {
            candidates: Vec::new(),
        }
    }

    pub(in crate::mir::builder) fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }

    pub(in crate::mir::builder) fn push(&mut self, candidate: PlanCandidate) {
        trace::trace_candidate_push(candidate.rule);
        self.candidates.push(candidate);
    }

    #[allow(unreachable_code, unused_variables)]
    pub(in crate::mir::builder) fn finalize(self) -> Result<Option<DomainPlan>, Freeze> {
        match self.candidates.len() {
            0 => {
                trace::trace_candidate_finalize_none();
                Ok(None)
            }
            1 => {
                let c = self.candidates.into_iter().next().expect("len == 1");
                trace::trace_candidate_finalize_some(c.rule);
                Ok(Some(c.plan))
            }
            n => {
                let rules = self
                    .candidates
                    .iter()
                    .map(|c| c.rule)
                    .collect::<Vec<_>>()
                    .join(", ");
                trace::trace_candidate_finalize_ambiguous(n, &rules);

                // Shadow pick: log what priority-based selection would choose
                pattern_shadow::trace_shadow_pick(&self.candidates);

                Err(Freeze::ambiguous(format!(
                    "multiple plan candidates (count={}): {}",
                    n, rules
                )))
            }
        }
    }
}
