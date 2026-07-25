//! One-shot VM-reference view over a completed source-entry projection.
//!
//! This module does not execute MIR or select a production runner. It retains
//! the complete projected owner and exposes only the already-normalized
//! process outcome.

use super::source_entry_projection::ProjectedSourceEntryV1;
use super::source_entry_result::{ProcessExitCodeV1, ProcessFaultV1};

#[derive(Debug)]
pub(in crate::mir) struct VmReferenceProcessOutcomeV1 {
    projected: ProjectedSourceEntryV1,
    _seal: VmReferenceProcessOutcomeSealV1,
}

#[derive(Debug)]
struct VmReferenceProcessOutcomeSealV1;

impl ProjectedSourceEntryV1 {
    /// Consume the projected owner without reopening source-result policy.
    pub(in crate::mir) fn consume_vm_reference(self) -> VmReferenceProcessOutcomeV1 {
        VmReferenceProcessOutcomeV1 {
            projected: self,
            _seal: VmReferenceProcessOutcomeSealV1,
        }
    }
}

impl VmReferenceProcessOutcomeV1 {
    pub(in crate::mir) fn status(&self) -> ProcessExitCodeV1 {
        self.projected.termination().status_code()
    }

    pub(in crate::mir) fn fault(&self) -> Option<&ProcessFaultV1> {
        self.projected.termination().fault()
    }

    pub(in crate::mir) fn discard(self) {}

    #[cfg(test)]
    pub(in crate::mir) fn route_for_test(
        &self,
    ) -> super::source_entry_selection::SelectedSourceEntryRouteV1 {
        self.projected.carrier().route()
    }
}
