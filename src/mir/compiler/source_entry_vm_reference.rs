//! One-shot VM-reference view over a completed source-entry projection.
//!
//! This module does not execute MIR or select a production runner. It retains
//! the complete projected owner and exposes only the already-normalized
//! process outcome.

use super::raw_root_publication::RawPublishedInvocationV1;
use super::canonical_core_dispatch::publication::{
    CanonicalPublishedFamilyKindV1, PublishedCanonicalSourceEntryOwnerV1,
};
use super::source_entry_projection::ProjectedSourceEntryV1;
use super::source_entry_result::{
    ProcessExitCodeV1, ProcessFaultV1, ProcessTerminationV1, SourceEntryResultKindV1,
    SourceEntryResultV1, UnitOriginV1,
};
use super::source_entry_vm_diagnostic::{
    VmReferenceProcessDiagnosticAdapterV1, VmReferenceProcessDiagnosticReportV1,
};
use crate::mir::builder::RawVmSourceEntryDecodeKindV1;
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

impl VmSourceEntryDecodePlanV1 {
    pub(in crate::mir) fn from_builder(plan: RawVmSourceEntryDecodeKindV1) -> Self {
        match plan {
            RawVmSourceEntryDecodeKindV1::Unit {
                origin,
                requires_void,
            } => Self::Unit {
                origin: match origin {
                    crate::mir::builder::RawVmUnitOriginV1::EmptyBody => UnitOriginV1::EmptyBody,
                    crate::mir::builder::RawVmUnitOriginV1::ImplicitFallthrough => {
                        UnitOriginV1::ImplicitFallthrough
                    }
                    crate::mir::builder::RawVmUnitOriginV1::PrintStatement => {
                        UnitOriginV1::PrintStatement
                    }
                    crate::mir::builder::RawVmUnitOriginV1::LocalStatement => {
                        UnitOriginV1::LocalStatement
                    }
                    crate::mir::builder::RawVmUnitOriginV1::AssignmentStatement => {
                        UnitOriginV1::AssignmentStatement
                    }
                    crate::mir::builder::RawVmUnitOriginV1::CompoundAssignmentStatement => {
                        UnitOriginV1::CompoundAssignmentStatement
                    }
                    crate::mir::builder::RawVmUnitOriginV1::ExplicitVoid => {
                        UnitOriginV1::ExplicitVoid
                    }
                },
                requires_void,
            },
            RawVmSourceEntryDecodeKindV1::Integer => Self::Integer,
            RawVmSourceEntryDecodeKindV1::Bool => Self::Bool,
            RawVmSourceEntryDecodeKindV1::Float => Self::Float,
            RawVmSourceEntryDecodeKindV1::String => Self::String,
        }
    }
}

impl VmSourceEntryDecodePlanV1 {
    pub(in crate::mir) const fn result_kind(self) -> SourceEntryResultKindV1 {
        match self {
            Self::Unit { .. } => SourceEntryResultKindV1::Unit,
            Self::Integer => SourceEntryResultKindV1::Integer,
            Self::Bool => SourceEntryResultKindV1::Bool,
            Self::Float => SourceEntryResultKindV1::Float,
            Self::String => SourceEntryResultKindV1::String,
        }
    }
}

#[derive(Debug)]
pub(in crate::mir) struct VmReferenceProcessOutcomeV1 {
    projected: VmReferenceProjectedOwnerV1,
    _seal: VmReferenceProcessOutcomeSealV1,
}

#[derive(Debug)]
pub(crate) struct RawVmReferenceRunReportV1 {
    status: ProcessExitCodeV1,
    diagnostic: Option<VmReferenceProcessDiagnosticReportV1>,
}

impl RawVmReferenceRunReportV1 {
    pub(crate) fn status_code(&self) -> u8 {
        self.status.value()
    }

    pub(crate) fn diagnostic_tag(&self) -> Option<&'static str> {
        self.diagnostic
            .as_ref()
            .map(VmReferenceProcessDiagnosticAdapterV1::tag)
    }

    pub(crate) fn diagnostic_line(&self) -> Option<String> {
        self.diagnostic
            .as_ref()
            .map(VmReferenceProcessDiagnosticAdapterV1::line)
    }
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
    pub(in crate::mir) fn from_published_vm_reference(
        published: VmReferencePublishedOwnerV1,
        source_result: SourceEntryResultV1,
        termination: ProcessTerminationV1,
    ) -> Self {
        Self {
            projected: VmReferenceProjectedOwnerV1::Published {
                published,
                source_result,
                termination,
            },
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
                VmReferencePublishedOwnerV1::Canonical(published) => match published.family_kind() {
                    CanonicalPublishedFamilyKindV1::Main => {
                        super::source_entry_selection::SelectedSourceEntryRouteV1::AppMain0
                    }
                    CanonicalPublishedFamilyKindV1::Script => {
                        super::source_entry_selection::SelectedSourceEntryRouteV1::Script
                    }
                    CanonicalPublishedFamilyKindV1::Callable => {
                        super::source_entry_selection::SelectedSourceEntryRouteV1::AppMain0
                    }
                },
            },
        }
    }
}
