//! Passive VM-reference projection over a published source-entry invocation.
//!
//! This layer selects no entry and executes no MIR. It converts the sealed
//! backend-neutral result contract into the existing VM decode vocabulary.

use super::source_entry_published_invocation::{
    PublishedSourceEntryInvocationV1, PublishedSourceEntryResultContractV1,
    PublishedUnitPhysicalContractV1,
};
#[cfg(feature = "vm-reference")]
use super::source_entry_result::{
    ProcessExitProjectionV1, SealedSourceFaultV1, SourceEntryResultV1,
};
use super::source_entry_vm_reference::VmSourceEntryDecodePlanV1;
#[cfg(feature = "vm-reference")]
use super::source_entry_vm_reference::{VmReferenceProcessOutcomeV1, VmReferencePublishedOwnerV1};
#[cfg(feature = "vm-reference")]
use crate::backend::vm_types::{VMError, VMValue};

#[derive(Debug)]
pub(in crate::mir) struct PreparedVmReferenceSourceEntryInvocationV1<O> {
    published: PublishedSourceEntryInvocationV1<O>,
    decode: VmSourceEntryDecodePlanV1,
    _seal: PreparedVmReferenceSourceEntryInvocationSealV1,
}

#[derive(Debug)]
struct PreparedVmReferenceSourceEntryInvocationSealV1;

impl<O> PublishedSourceEntryInvocationV1<O> {
    pub(in crate::mir) fn prepare_vm_reference(
        self,
    ) -> PreparedVmReferenceSourceEntryInvocationV1<O> {
        let decode = match self.result() {
            PublishedSourceEntryResultContractV1::Unit { origin, physical } => {
                VmSourceEntryDecodePlanV1::Unit {
                    origin,
                    requires_void: matches!(physical, PublishedUnitPhysicalContractV1::ExactVoid),
                }
            }
            PublishedSourceEntryResultContractV1::Integer => VmSourceEntryDecodePlanV1::Integer,
            PublishedSourceEntryResultContractV1::Bool => VmSourceEntryDecodePlanV1::Bool,
            PublishedSourceEntryResultContractV1::Float => VmSourceEntryDecodePlanV1::Float,
            PublishedSourceEntryResultContractV1::String => VmSourceEntryDecodePlanV1::String,
        };
        PreparedVmReferenceSourceEntryInvocationV1 {
            published: self,
            decode,
            _seal: PreparedVmReferenceSourceEntryInvocationSealV1,
        }
    }
}

impl<O> PreparedVmReferenceSourceEntryInvocationV1<O> {
    pub(in crate::mir) const fn decode_plan(&self) -> VmSourceEntryDecodePlanV1 {
        self.decode
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        PublishedSourceEntryInvocationV1<O>,
        VmSourceEntryDecodePlanV1,
    ) {
        (self.published, self.decode)
    }
}

#[cfg(feature = "vm-reference")]
pub(in crate::mir) trait VmReferenceExecutablePublishedOwnerV1 {
    fn execute_exact_vm_entry(&self, symbol: &str) -> Result<VMValue, VMError>;
}

#[cfg(feature = "vm-reference")]
#[derive(Debug)]
pub(in crate::mir) struct CompletedVmReferenceSourceEntryInvocationV1<O> {
    published: PublishedSourceEntryInvocationV1<O>,
    source_result: SourceEntryResultV1,
    _seal: CompletedVmReferenceSourceEntryInvocationSealV1,
}

#[cfg(feature = "vm-reference")]
#[derive(Debug)]
struct CompletedVmReferenceSourceEntryInvocationSealV1;

#[cfg(feature = "vm-reference")]
impl<O: VmReferenceExecutablePublishedOwnerV1> PreparedVmReferenceSourceEntryInvocationV1<O> {
    pub(in crate::mir) fn execute(self) -> CompletedVmReferenceSourceEntryInvocationV1<O> {
        let symbol = self.published.target().symbol().to_owned();
        let execution = self.published.owner().execute_exact_vm_entry(&symbol);
        let source_result = match execution {
            Ok(value) => decode_vm_value(self.decode, value),
            Err(error) => vm_error_to_source_fault(error),
        };
        CompletedVmReferenceSourceEntryInvocationV1 {
            published: self.published,
            source_result,
            _seal: CompletedVmReferenceSourceEntryInvocationSealV1,
        }
    }
}

#[cfg(feature = "vm-reference")]
impl<O: Into<VmReferencePublishedOwnerV1>> CompletedVmReferenceSourceEntryInvocationV1<O> {
    pub(in crate::mir) fn complete_canonical_source_entry(self) -> VmReferenceProcessOutcomeV1 {
        let (owner, _, _, _) = self.published.into_parts();
        let termination = ProcessExitProjectionV1::project_canonical(&self.source_result);
        VmReferenceProcessOutcomeV1::from_published_vm_reference(
            owner.into(),
            self.source_result,
            termination,
        )
    }
}

#[cfg(feature = "vm-reference")]
pub(super) fn decode_vm_value(
    plan: VmSourceEntryDecodePlanV1,
    value: VMValue,
) -> SourceEntryResultV1 {
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

#[cfg(feature = "vm-reference")]
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

#[cfg(feature = "vm-reference")]
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

#[cfg(feature = "vm-reference")]
fn abi_mismatch(expected: &'static str, actual: &'static str) -> SourceEntryResultV1 {
    SourceEntryResultV1::Fault(SealedSourceFaultV1::new(
        "vm-entry-result-abi-mismatch",
        format!("expected={}, actual={}", expected, actual).into_boxed_str(),
    ))
}

#[cfg(feature = "vm-reference")]
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
    use crate::mir::compiler::source_entry_published_invocation::{
        CanonicalPublishedSourceEntryMembershipV1, PendingPublishedSourceEntryTargetV1,
        PublishedSourceEntryMembershipV1,
    };
    use crate::mir::compiler::source_entry_result::UnitOriginV1;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    fn prepare(
        result: PublishedSourceEntryResultContractV1,
    ) -> PreparedVmReferenceSourceEntryInvocationV1<&'static str> {
        let target = PendingPublishedSourceEntryTargetV1::new("main", 0)
            .seal()
            .expect("exact target");
        PublishedSourceEntryInvocationV1::from_verified_parts(
            "owner",
            target,
            result,
            PublishedSourceEntryMembershipV1::Canonical(
                CanonicalPublishedSourceEntryMembershipV1::Main {
                    source_owner: FunctionOwnerIssuerV1::new_for_compilation()
                        .expect("test owner issuer")
                        .issue()
                        .expect("test owner"),
                },
            ),
        )
        .prepare_vm_reference()
    }

    #[test]
    fn all_source_result_contracts_project_without_execution() {
        for origin in [
            UnitOriginV1::EmptyBody,
            UnitOriginV1::ImplicitFallthrough,
            UnitOriginV1::BareReturn,
            UnitOriginV1::ExplicitVoid,
            UnitOriginV1::ExplicitNull,
        ] {
            let prepared = prepare(PublishedSourceEntryResultContractV1::Unit {
                origin,
                physical: PublishedUnitPhysicalContractV1::ExactVoid,
            });
            assert_eq!(
                prepared.decode_plan(),
                VmSourceEntryDecodePlanV1::Unit {
                    origin,
                    requires_void: true,
                }
            );
        }

        let cases = [
            (
                PublishedSourceEntryResultContractV1::Unit {
                    origin: UnitOriginV1::PrintStatement,
                    physical: PublishedUnitPhysicalContractV1::CompatiblePayload,
                },
                VmSourceEntryDecodePlanV1::Unit {
                    origin: UnitOriginV1::PrintStatement,
                    requires_void: false,
                },
            ),
            (
                PublishedSourceEntryResultContractV1::Integer,
                VmSourceEntryDecodePlanV1::Integer,
            ),
            (
                PublishedSourceEntryResultContractV1::Bool,
                VmSourceEntryDecodePlanV1::Bool,
            ),
            (
                PublishedSourceEntryResultContractV1::Float,
                VmSourceEntryDecodePlanV1::Float,
            ),
            (
                PublishedSourceEntryResultContractV1::String,
                VmSourceEntryDecodePlanV1::String,
            ),
        ];
        for (result, expected) in cases {
            let prepared = prepare(result);
            assert_eq!(prepared.decode_plan(), expected);
            let (published, decode) = prepared.into_parts();
            assert_eq!(published.result(), result);
            assert_eq!(decode, expected);
        }
    }
}
