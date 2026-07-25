//! Bounded MIR-to-runner adapter for the explicit VM-reference lanes.
//!
//! The runner receives a stable failure report, never an unpublished compile
//! or activation owner. Runtime faults remain successful process terminals.

use super::source_entry_vm_execution::{RawVmReferenceRunStageV1, RejectedRawVmReferenceRunV1};
use crate::mir::{MirCompiler, RawVmReferenceInvocationV1, RawVmReferenceRunReportV1};

#[derive(Debug)]
pub(crate) struct RawVmReferenceInvocationFailureReportV1 {
    stage: RawVmReferenceRunnerFailureStageV1,
    code: &'static str,
    detail: Box<str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawVmReferenceRunnerFailureStageV1 {
    Compile,
    Activation,
}

impl RawVmReferenceInvocationFailureReportV1 {
    pub(crate) const fn stage(&self) -> RawVmReferenceRunnerFailureStageV1 {
        self.stage
    }

    pub(crate) const fn code(&self) -> &'static str {
        self.code
    }

    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }
}

impl From<RejectedRawVmReferenceRunV1> for RawVmReferenceInvocationFailureReportV1 {
    fn from(rejected: RejectedRawVmReferenceRunV1) -> Self {
        let (stage, code) = match rejected.stage() {
            RawVmReferenceRunStageV1::Compile => (
                RawVmReferenceRunnerFailureStageV1::Compile,
                "raw-compile-rejected",
            ),
            RawVmReferenceRunStageV1::Activation => (
                RawVmReferenceRunnerFailureStageV1::Activation,
                "raw-activation-rejected",
            ),
        };
        let detail = rejected.into_public_string().into_boxed_str();
        Self {
            stage,
            code,
            detail,
        }
    }
}

impl MirCompiler {
    /// Consume the private typed rejection into a bounded runner-facing report.
    pub(crate) fn run_raw_vm_reference_for_runner_v1(
        &mut self,
        invocation: RawVmReferenceInvocationV1,
    ) -> Result<RawVmReferenceRunReportV1, RawVmReferenceInvocationFailureReportV1> {
        self.run_raw_vm_reference_owned_v1(invocation)
            .map_err(RawVmReferenceInvocationFailureReportV1::from)
    }
}
