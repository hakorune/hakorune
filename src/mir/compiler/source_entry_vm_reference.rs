//! One-shot VM-reference view over a completed source-entry projection.
//!
//! This module does not execute MIR or select a production runner. It retains
//! the complete projected owner and exposes only the already-normalized
//! process outcome.

use super::canonical_core_dispatch::publication::PublishedCanonicalSourceEntryOwnerV1;
#[cfg(test)]
use super::canonical_core_dispatch::publication::CanonicalPublishedFamilyKindV1;
use super::raw_root_publication::RawPublishedInvocationV1;
use super::source_entry_projection::ProjectedSourceEntryV1;
use super::source_entry_result::{
    ProcessExitCodeV1, ProcessFaultV1, ProcessTerminationV1,
    SourceEntryResultV1, UnitOriginV1,
};
#[cfg(feature = "vm-reference")]
use super::source_entry_vm_invocation::VmReferenceExecutablePublishedOwnerV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum VmSourceEntryDecodePlanV1 {
    Unit {
        origin: UnitOriginV1,
        requires_void: bool,
    },
    Integer,
    Bool,
    Float,
    String,
}

#[derive(Debug)]
pub(in crate::mir) struct VmReferenceProcessOutcomeV1 {
    projected: VmReferenceProjectedOwnerV1,
    _seal: VmReferenceProcessOutcomeSealV1,
}

#[derive(Debug)]
enum VmReferenceProjectedOwnerV1 {
    Existing(ProjectedSourceEntryV1),
    Published {
        published: VmReferencePublishedOwnerV1,
        source_result: SourceEntryResultV1,
        termination: ProcessTerminationV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) enum VmReferencePublishedOwnerV1 {
    Raw(RawPublishedInvocationV1),
    Canonical(PublishedCanonicalSourceEntryOwnerV1),
}

impl From<PublishedCanonicalSourceEntryOwnerV1> for VmReferencePublishedOwnerV1 {
    fn from(owner: PublishedCanonicalSourceEntryOwnerV1) -> Self {
        Self::Canonical(owner)
    }
}

#[cfg(feature = "vm-reference")]
impl VmReferenceExecutablePublishedOwnerV1 for PublishedCanonicalSourceEntryOwnerV1 {
    fn execute_exact_vm_entry(
        &self,
        symbol: &str,
    ) -> Result<crate::backend::vm_types::VMValue, crate::backend::vm_types::VMError> {
        let mut interpreter = crate::backend::mir_interpreter::MirInterpreter::new();
        interpreter.execute_function_with_args(self.module(), symbol, &[])
    }
}

#[derive(Debug)]
struct VmReferenceProcessOutcomeSealV1;

impl ProjectedSourceEntryV1 {
    /// Consume the projected owner without reopening source-result policy.
    pub(in crate::mir) fn consume_vm_reference(self) -> VmReferenceProcessOutcomeV1 {
        VmReferenceProcessOutcomeV1 {
            projected: VmReferenceProjectedOwnerV1::Existing(self),
            _seal: VmReferenceProcessOutcomeSealV1,
        }
    }
}

impl VmReferenceProcessOutcomeV1 {
    pub(in crate::mir) fn status(&self) -> ProcessExitCodeV1 {
        match &self.projected {
            VmReferenceProjectedOwnerV1::Existing(projected) => {
                projected.termination().status_code()
            }
            VmReferenceProjectedOwnerV1::Published { termination, .. } => termination.status_code(),
        }
    }

    pub(in crate::mir) fn fault(&self) -> Option<&ProcessFaultV1> {
        match &self.projected {
            VmReferenceProjectedOwnerV1::Existing(projected) => projected.termination().fault(),
            VmReferenceProjectedOwnerV1::Published { termination, .. } => termination.fault(),
        }
    }

    pub(in crate::mir) fn discard(self) {}

    #[cfg(feature = "vm-reference")]
    pub(in crate::mir) fn into_run_report(self) -> RawVmReferenceRunReportV1 {
        let status = self.status();
        let diagnostic = self
            .fault()
            .map(VmReferenceProcessDiagnosticAdapterV1::project);
        self.discard();
        RawVmReferenceRunReportV1 { status, diagnostic }
    }

    #[cfg(test)]
    pub(in crate::mir) fn route_for_test(
        &self,
    ) -> super::source_entry_selection::SelectedSourceEntryRouteV1 {
        match &self.projected {
            VmReferenceProjectedOwnerV1::Existing(projected) => projected.carrier().route(),
            VmReferenceProjectedOwnerV1::Published { published, .. } => match published {
                VmReferencePublishedOwnerV1::Raw(published) => published.selected_entry().route(),
                VmReferencePublishedOwnerV1::Canonical(published) => {
                    match published.family_kind() {
                        CanonicalPublishedFamilyKindV1::Main => {
                            super::source_entry_selection::SelectedSourceEntryRouteV1::AppMain0
                        }
                        CanonicalPublishedFamilyKindV1::Script => {
                            super::source_entry_selection::SelectedSourceEntryRouteV1::Script
                        }
                        CanonicalPublishedFamilyKindV1::Callable => {
                            super::source_entry_selection::SelectedSourceEntryRouteV1::AppMain0
                        }
                    }
                }
            },
        }
    }
}
