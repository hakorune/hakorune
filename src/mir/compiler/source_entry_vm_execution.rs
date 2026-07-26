//! S3 exact Raw VM-reference execution.
//!
//! This lane consumes a published Raw owner, executes only the sealed Main
//! target, and converts the VM result into a source result before the shared
//! process projection.  It never discovers an entry from the module or
//! reconstructs a status from a VM value.

use super::raw_published_compile::RejectedRawPublishedCompileV1;
use super::source_entry_vm_raw_adapter::RejectedRawPublishedVmAdapterV1;
use super::source_entry_vm_reference::RawVmReferenceRunReportV1;
use crate::mir::{RawVmReferenceExecutionProfileV1, RawVmReferenceInvocationV1};

/// The only typed failures after a Raw VM-reference invocation owns a Raw
/// compile request. Runtime faults intentionally become source results.
#[derive(Debug)]
pub(in crate::mir) enum RejectedRawVmReferenceRunV1 {
    Compile(Box<RejectedRawPublishedCompileV1>),
    Activation(Box<RejectedRawPublishedVmAdapterV1>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawVmReferenceRunStageV1 {
    Compile,
    Activation,
}

pub(in crate::mir) enum RawVmReferenceRunEvidenceV1<'a> {
    Compile(&'a RejectedRawPublishedCompileV1),
    Activation(&'a RejectedRawPublishedVmAdapterV1),
}

impl RejectedRawVmReferenceRunV1 {
    pub(in crate::mir) const fn stage(&self) -> RawVmReferenceRunStageV1 {
        match self {
            Self::Compile(_) => RawVmReferenceRunStageV1::Compile,
            Self::Activation(_) => RawVmReferenceRunStageV1::Activation,
        }
    }

    pub(in crate::mir) fn evidence(&self) -> RawVmReferenceRunEvidenceV1<'_> {
        match self {
            Self::Compile(rejected) => RawVmReferenceRunEvidenceV1::Compile(rejected),
            Self::Activation(rejected) => RawVmReferenceRunEvidenceV1::Activation(rejected),
        }
    }

    pub(in crate::mir) fn discard(self) {
        match self {
            Self::Compile(rejected) => rejected.discard(),
            Self::Activation(rejected) => rejected.discard(),
        }
    }

    pub(in crate::mir) fn into_public_string(self) -> String {
        match self {
            Self::Compile(rejected) => rejected.into_public_string(),
            Self::Activation(rejected) => rejected.into_public_string(),
        }
    }
}

impl super::MirCompiler {
    /// Test-only compatibility helper for the fixed NarrowV1 profile.
    #[cfg(test)]
    pub(crate) fn run_raw_vm_reference(
        &mut self,
        ast: crate::ast::ASTNode,
        source_file: Option<&str>,
    ) -> Result<RawVmReferenceRunReportV1, String> {
        self.run_raw_vm_reference_v1(RawVmReferenceInvocationV1::narrow_v1(ast, source_file))
    }

    /// Supported opt-in Raw VM-reference production entry.  The invocation
    /// owns the selected compile and execution profiles; this owner does not
    /// reconstruct NarrowV1 or the process policy.
    pub(crate) fn run_raw_vm_reference_v1(
        &mut self,
        invocation: RawVmReferenceInvocationV1,
    ) -> Result<RawVmReferenceRunReportV1, String> {
        if self.builder.repl_mode {
            return Err("[raw-vm-reference/source-binding/repl-unsupported] NarrowV1".to_owned());
        }
        self.run_raw_vm_reference_owned_v1(invocation)
            .map_err(RejectedRawVmReferenceRunV1::into_public_string)
    }

    pub(in crate::mir) fn run_raw_vm_reference_owned_v1(
        &mut self,
        invocation: RawVmReferenceInvocationV1,
    ) -> Result<RawVmReferenceRunReportV1, RejectedRawVmReferenceRunV1> {
        let RawVmReferenceInvocationV1 { compile, execution } = invocation;
        let RawVmReferenceExecutionProfileV1::CanonicalV1 = execution;
        let published = self
            .compile_raw_published_v1(compile)
            .map_err(|rejected| RejectedRawVmReferenceRunV1::Compile(Box::new(rejected)))?;
        let prepared = published
            .prepare_neutral_vm_reference()
            .map_err(|rejected| RejectedRawVmReferenceRunV1::Activation(Box::new(rejected)))?;
        Ok(prepared
            .execute()
            .complete_canonical_source_entry()
            .into_run_report())
    }
}

#[cfg(test)]
#[path = "source_entry_vm_execution_tests.rs"]
mod tests;
