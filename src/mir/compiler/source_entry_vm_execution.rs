//! S3 exact Raw VM-reference execution.
//!
//! This lane consumes a published Raw owner, executes only the sealed Main
//! target, and converts the VM result into a source result before the shared
//! process projection.  It never discovers an entry from the module or
//! reconstructs a status from a VM value.

use super::raw_root_publication::RawPublishedInvocationV1;
use super::source_entry_result::{
    CanonicalProcessExitV1, ProcessExitProfileV1, ProcessExitProjectionV1, SealedSourceFaultV1,
    SourceEntryResultV1,
};
use super::source_entry_vm_reference::{VmReferenceProcessOutcomeV1, VmSourceEntryDecodePlanV1};
use crate::backend::vm_types::{VMError, VMValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawVmReferenceActivationStageV1 {
    Target,
    DecodePlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawVmReferenceActivationErrorV1 {
    NonRawTarget,
    EntryTargetMismatch,
    DecodePlanUnavailable,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawVmReferenceActivationV1 {
    owner: RawPublishedInvocationV1,
    stage: RawVmReferenceActivationStageV1,
    error: RawVmReferenceActivationErrorV1,
}

impl RejectedRawVmReferenceActivationV1 {
    pub(in crate::mir) const fn stage(&self) -> RawVmReferenceActivationStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &RawVmReferenceActivationErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedRawVmReferenceActivationV1 {
    published: RawPublishedInvocationV1,
    plan: VmSourceEntryDecodePlanV1,
}

impl RawPublishedInvocationV1 {
    pub(in crate::mir) fn prepare_vm_reference_activation(
        self,
    ) -> Result<PreparedRawVmReferenceActivationV1, RejectedRawVmReferenceActivationV1> {
        if !self.main_entry_target_matches()
            || !self.selected_entry().is_main_target()
            || self.selected_entry().arity() != 0
        {
            return Err(RejectedRawVmReferenceActivationV1 {
                owner: self,
                stage: RawVmReferenceActivationStageV1::Target,
                error: RawVmReferenceActivationErrorV1::EntryTargetMismatch,
            });
        }
        let plan = match self.vm_decode_plan() {
            Ok(plan) => plan,
            Err(()) => {
                return Err(RejectedRawVmReferenceActivationV1 {
                    owner: self,
                    stage: RawVmReferenceActivationStageV1::DecodePlan,
                    error: RawVmReferenceActivationErrorV1::DecodePlanUnavailable,
                })
            }
        };
        Ok(PreparedRawVmReferenceActivationV1 {
            published: self,
            plan,
        })
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RawVmReferenceExecutionReceiptV1;

#[derive(Debug)]
pub(in crate::mir) struct CompletedRawVmReferenceExecutionV1 {
    published: RawPublishedInvocationV1,
    source_result: SourceEntryResultV1,
    _receipt: RawVmReferenceExecutionReceiptV1,
}

impl PreparedRawVmReferenceActivationV1 {
    pub(in crate::mir) fn execute(self) -> CompletedRawVmReferenceExecutionV1 {
        let Self { published, plan } = self;
        let symbol = published.selected_entry().symbol().to_owned();
        let (published, execution) = published.execute_exact_vm_entry(&symbol);
        let source_result = match execution {
            Ok(value) => decode_vm_value(plan, value),
            Err(error) => vm_error_to_source_fault(error),
        };
        CompletedRawVmReferenceExecutionV1 {
            published,
            source_result,
            _receipt: RawVmReferenceExecutionReceiptV1,
        }
    }
}

impl CompletedRawVmReferenceExecutionV1 {
    pub(in crate::mir) fn complete_source_entry(
        self,
    ) -> Result<VmReferenceProcessOutcomeV1, RawVmReferenceActivationErrorV1> {
        let Self {
            published,
            source_result,
            ..
        } = self;
        let termination = ProcessExitProjectionV1::project_borrowed(
            &source_result,
            ProcessExitProfileV1::Canonical(CanonicalProcessExitV1::V1),
        )
        .map_err(|_| RawVmReferenceActivationErrorV1::NonRawTarget)?;
        // The published owner is consumed only here and remains inside the
        // process outcome until the terminal consumes that outcome by value.
        Ok(VmReferenceProcessOutcomeV1::from_raw_vm_reference(
            published,
            source_result,
            termination,
        ))
    }
}

fn decode_vm_value(plan: VmSourceEntryDecodePlanV1, value: VMValue) -> SourceEntryResultV1 {
    match plan {
        VmSourceEntryDecodePlanV1::Unit {
            origin,
            requires_void,
        } => {
            if requires_void && !matches!(value, VMValue::Void) {
                return abi_mismatch("Void", vm_value_kind(&value));
            }
            if !requires_void && !is_supported_unit_payload(&value) {
                return abi_mismatch("unit-compatible VM value", vm_value_kind(&value));
            }
            SourceEntryResultV1::Unit(origin)
        }
        VmSourceEntryDecodePlanV1::Integer => match value {
            VMValue::Integer(value) => SourceEntryResultV1::Integer(value),
            other => abi_mismatch("Integer", vm_value_kind(&other)),
        },
        VmSourceEntryDecodePlanV1::Bool => match value {
            VMValue::Bool(value) => SourceEntryResultV1::Bool(value),
            other => abi_mismatch("Bool", vm_value_kind(&other)),
        },
        VmSourceEntryDecodePlanV1::Float => match value {
            VMValue::Float(value) => SourceEntryResultV1::Float(value),
            other => abi_mismatch("Float", vm_value_kind(&other)),
        },
        VmSourceEntryDecodePlanV1::String => match value {
            VMValue::String(value) => SourceEntryResultV1::String(value.into_boxed_str()),
            other => abi_mismatch("String", vm_value_kind(&other)),
        },
    }
}

fn is_supported_unit_payload(value: &VMValue) -> bool {
    matches!(
        value,
        VMValue::Integer(_)
            | VMValue::Float(_)
            | VMValue::Bool(_)
            | VMValue::String(_)
            | VMValue::Void
    )
}

fn vm_value_kind(value: &VMValue) -> &'static str {
    match value {
        VMValue::Integer(_) => "Integer",
        VMValue::ExactNumeric(_) => "ExactNumeric",
        VMValue::Float(_) => "Float",
        VMValue::Bool(_) => "Bool",
        VMValue::String(_) => "String",
        VMValue::Future(_) => "Future",
        VMValue::Void => "Void",
        VMValue::BoxRef(_) => "BoxRef",
        VMValue::WeakBox(_) => "WeakBox",
    }
}

fn abi_mismatch(expected: &'static str, actual: &'static str) -> SourceEntryResultV1 {
    SourceEntryResultV1::Fault(SealedSourceFaultV1::new(
        "vm-entry-result-abi-mismatch",
        format!("expected={}, actual={}", expected, actual).into_boxed_str(),
    ))
}

fn vm_error_to_source_fault(error: VMError) -> SourceEntryResultV1 {
    let (code, detail) = match &error {
        VMError::DivisionByZero => ("vm-division-by-zero", error.to_string()),
        VMError::StepBudgetExceeded { .. } => ("vm-step-budget-exceeded", error.to_string()),
        VMError::DuringFrameRestore { .. } => ("vm-frame-restore-failed", error.to_string()),
        VMError::FrameRestoreFailed { .. } => ("vm-frame-restore-failed", error.to_string()),
        VMError::InvalidValue(_) => ("vm-invalid-value", error.to_string()),
        VMError::InvalidInstruction(_) => ("vm-invalid-instruction", error.to_string()),
        VMError::InvalidBasicBlock(_) => ("vm-invalid-basic-block", error.to_string()),
        VMError::StackUnderflow => ("vm-stack-underflow", error.to_string()),
        VMError::TypeError(_) => ("vm-type-error", error.to_string()),
        VMError::TaskFailed(_) => ("vm-task-failed", error.to_string()),
        VMError::TaskCancelled(_) => ("vm-task-cancelled", error.to_string()),
    };
    SourceEntryResultV1::Fault(SealedSourceFaultV1::new(code, detail.into_boxed_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::source_entry_result::UnitOriginV1;

    #[test]
    fn unit_plan_does_not_promote_print_payload_to_a_value() {
        let result = decode_vm_value(
            VmSourceEntryDecodePlanV1::Unit {
                origin: UnitOriginV1::PrintStatement,
                requires_void: false,
            },
            VMValue::Integer(1),
        );
        assert!(matches!(
            result,
            SourceEntryResultV1::Unit(UnitOriginV1::PrintStatement)
        ));
    }

    #[test]
    fn integer_plan_preserves_exact_integer_value() {
        assert!(matches!(
            decode_vm_value(VmSourceEntryDecodePlanV1::Integer, VMValue::Integer(255)),
            SourceEntryResultV1::Integer(255)
        ));
    }

    #[test]
    fn synthetic_unit_requires_void_payload() {
        let result = decode_vm_value(
            VmSourceEntryDecodePlanV1::Unit {
                origin: UnitOriginV1::EmptyBody,
                requires_void: true,
            },
            VMValue::Integer(0),
        );
        assert!(matches!(result, SourceEntryResultV1::Fault(_)));
    }
}
