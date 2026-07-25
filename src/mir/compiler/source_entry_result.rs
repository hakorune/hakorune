//! Typed source-entry results and canonical process projection.
//!
//! This module is deliberately disconnected from runner/backend entry points.
//! It owns the semantic boundary only:
//!
//! ```text
//! SourceEntryResultV1 -> ProcessExitProjectionV1 -> ProcessTerminationV1
//! ```
//!
//! Legacy runner conversions (including Box-to-status and mock 42/0 rules)
//! remain outside this module until an explicit cutover row.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum UnitOriginV1 {
    EmptyBody,
    ImplicitFallthrough,
    ExpressionStatementDiscard,
    PrintStatement,
    LocalStatement,
    AssignmentStatement,
    CompoundAssignmentStatement,
    ExplicitVoid,
    ExplicitNull,
    BareReturn,
}

/// A source object result whose concrete runtime representation is owned by a
/// later source-entry producer. The process boundary only needs its kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct SealedObjectResultV1 {
    kind: Box<str>,
}

impl SealedObjectResultV1 {
    pub(in crate::mir) fn new(kind: Box<str>) -> Self {
        Self { kind }
    }

    pub(in crate::mir) fn kind_name(&self) -> &str {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct SealedSourceFaultV1 {
    code: &'static str,
    detail: Box<str>,
}

impl SealedSourceFaultV1 {
    pub(in crate::mir) fn new(code: &'static str, detail: Box<str>) -> Self {
        Self { code, detail }
    }

    pub(in crate::mir) fn code(&self) -> &'static str {
        self.code
    }

    pub(in crate::mir) fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum SourceEntryResultKindV1 {
    Unit,
    Integer,
    Bool,
    Float,
    String,
    Object,
    Fault,
}

#[derive(Debug, PartialEq)]
pub(in crate::mir) enum SourceEntryResultV1 {
    Unit(UnitOriginV1),
    Integer(i64),
    Bool(bool),
    Float(f64),
    String(Box<str>),
    Object(SealedObjectResultV1),
    Fault(SealedSourceFaultV1),
}

impl SourceEntryResultV1 {
    pub(in crate::mir) fn kind(&self) -> SourceEntryResultKindV1 {
        match self {
            Self::Unit(_) => SourceEntryResultKindV1::Unit,
            Self::Integer(_) => SourceEntryResultKindV1::Integer,
            Self::Bool(_) => SourceEntryResultKindV1::Bool,
            Self::Float(_) => SourceEntryResultKindV1::Float,
            Self::String(_) => SourceEntryResultKindV1::String,
            Self::Object(_) => SourceEntryResultKindV1::Object,
            Self::Fault(_) => SourceEntryResultKindV1::Fault,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct ProcessExitCodeV1(u8);

impl ProcessExitCodeV1 {
    pub(in crate::mir) const fn from_byte(value: u8) -> Self {
        Self(value)
    }

    pub(in crate::mir) const fn zero() -> Self {
        Self(0)
    }

    pub(in crate::mir) const fn reserved_fault() -> Self {
        Self(70)
    }

    fn try_from_integer(value: i64) -> Result<Self, ProcessFaultV1> {
        u8::try_from(value)
            .map(Self)
            .map_err(|_| ProcessFaultV1::ExitCodeOutOfRange { value })
    }

    pub(in crate::mir) const fn value(self) -> u8 {
        self.0
    }

    pub(in crate::mir) fn normalized_i64(self) -> i64 {
        i64::from(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalProcessExitV1 {
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ProcessExitProfileV1 {
    Canonical(CanonicalProcessExitV1),
    LegacyRunnerExitProjectionV1,
}

#[derive(Debug, PartialEq)]
pub(in crate::mir) enum ProcessFaultV1 {
    ExitCodeOutOfRange {
        value: i64,
    },
    UnsupportedProcessResult {
        kind: SourceEntryResultKindV1,
    },
    SourceFault {
        code: &'static str,
        detail: Box<str>,
    },
}

impl ProcessFaultV1 {
    pub(in crate::mir) fn diagnostic_fields(&self) -> ProcessFaultDiagnosticFieldsV1<'_> {
        match self {
            Self::ExitCodeOutOfRange { value } => {
                ProcessFaultDiagnosticFieldsV1::ExitCodeOutOfRange { value: *value }
            }
            Self::UnsupportedProcessResult { kind } => {
                ProcessFaultDiagnosticFieldsV1::UnsupportedProcessResult { kind: *kind }
            }
            Self::SourceFault { code, detail } => ProcessFaultDiagnosticFieldsV1::SourceFault {
                code,
                detail,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ProcessFaultDiagnosticFieldsV1<'a> {
    ExitCodeOutOfRange { value: i64 },
    UnsupportedProcessResult { kind: SourceEntryResultKindV1 },
    SourceFault { code: &'static str, detail: &'a str },
}

#[derive(Debug, PartialEq)]
pub(in crate::mir) enum ProcessTerminationV1 {
    Exit(ProcessExitCodeV1),
    Fault {
        status: ProcessExitCodeV1,
        fault: ProcessFaultV1,
    },
}

impl ProcessTerminationV1 {
    pub(in crate::mir) const fn status_code(&self) -> ProcessExitCodeV1 {
        match self {
            Self::Exit(status) | Self::Fault { status, .. } => *status,
        }
    }

    pub(in crate::mir) const fn fault(&self) -> Option<&ProcessFaultV1> {
        match self {
            Self::Exit(_) => None,
            Self::Fault { fault, .. } => Some(fault),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ProcessExitProjectionErrorV1 {
    LegacyProfileDisconnected,
}

pub(in crate::mir) struct ProcessExitProjectionV1;

impl ProcessExitProjectionV1 {
    pub(in crate::mir) fn project_borrowed(
        result: &SourceEntryResultV1,
        profile: ProcessExitProfileV1,
    ) -> Result<ProcessTerminationV1, ProcessExitProjectionErrorV1> {
        let ProcessExitProfileV1::Canonical(CanonicalProcessExitV1::V1) = profile else {
            return Err(ProcessExitProjectionErrorV1::LegacyProfileDisconnected);
        };

        let termination = match result {
            SourceEntryResultV1::Unit(_) => ProcessTerminationV1::Exit(ProcessExitCodeV1::zero()),
            SourceEntryResultV1::Integer(value) => {
                match ProcessExitCodeV1::try_from_integer(*value) {
                    Ok(code) => ProcessTerminationV1::Exit(code),
                    Err(fault) => ProcessTerminationV1::Fault {
                        status: ProcessExitCodeV1::reserved_fault(),
                        fault,
                    },
                }
            }
            SourceEntryResultV1::Bool(_) => ProcessTerminationV1::Fault {
                status: ProcessExitCodeV1::reserved_fault(),
                fault: ProcessFaultV1::UnsupportedProcessResult {
                    kind: SourceEntryResultKindV1::Bool,
                },
            },
            SourceEntryResultV1::Float(_) => ProcessTerminationV1::Fault {
                status: ProcessExitCodeV1::reserved_fault(),
                fault: ProcessFaultV1::UnsupportedProcessResult {
                    kind: SourceEntryResultKindV1::Float,
                },
            },
            SourceEntryResultV1::String(_) => ProcessTerminationV1::Fault {
                status: ProcessExitCodeV1::reserved_fault(),
                fault: ProcessFaultV1::UnsupportedProcessResult {
                    kind: SourceEntryResultKindV1::String,
                },
            },
            SourceEntryResultV1::Object(object) => {
                let _object_kind = object.kind_name();
                ProcessTerminationV1::Fault {
                    status: ProcessExitCodeV1::reserved_fault(),
                    fault: ProcessFaultV1::UnsupportedProcessResult {
                        kind: SourceEntryResultKindV1::Object,
                    },
                }
            }
            SourceEntryResultV1::Fault(fault) => ProcessTerminationV1::Fault {
                status: ProcessExitCodeV1::reserved_fault(),
                fault: ProcessFaultV1::SourceFault {
                    code: fault.code(),
                    detail: fault.detail().into(),
                },
            },
        };
        Ok(termination)
    }

    pub(in crate::mir) fn project(
        result: SourceEntryResultV1,
        profile: ProcessExitProfileV1,
    ) -> Result<ProcessTerminationV1, ProcessExitProjectionErrorV1> {
        Self::project_borrowed(&result, profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canonical(result: SourceEntryResultV1) -> ProcessTerminationV1 {
        ProcessExitProjectionV1::project(
            result,
            ProcessExitProfileV1::Canonical(CanonicalProcessExitV1::V1),
        )
        .expect("canonical profile is connected")
    }

    #[test]
    fn unit_and_integer_byte_values_project_without_wrapping() {
        assert_eq!(
            canonical(SourceEntryResultV1::Unit(UnitOriginV1::EmptyBody)),
            ProcessTerminationV1::Exit(ProcessExitCodeV1::zero())
        );
        assert_eq!(
            canonical(SourceEntryResultV1::Integer(0)),
            ProcessTerminationV1::Exit(ProcessExitCodeV1::zero())
        );
        assert_eq!(
            canonical(SourceEntryResultV1::Integer(255)),
            ProcessTerminationV1::Exit(ProcessExitCodeV1(255))
        );
    }

    #[test]
    fn out_of_range_integer_is_typed_fault() {
        for value in [-1, 256] {
            assert_eq!(
                canonical(SourceEntryResultV1::Integer(value)),
                ProcessTerminationV1::Fault {
                    status: ProcessExitCodeV1::reserved_fault(),
                    fault: ProcessFaultV1::ExitCodeOutOfRange { value },
                }
            );
        }
    }

    #[test]
    fn unsupported_values_never_become_success_zero() {
        for result in [
            SourceEntryResultV1::Bool(true),
            SourceEntryResultV1::Float(1.5),
            SourceEntryResultV1::String("text".into()),
            SourceEntryResultV1::Object(SealedObjectResultV1::new("ArrayBox".into())),
        ] {
            assert!(matches!(
                canonical(result),
                ProcessTerminationV1::Fault {
                    status,
                    fault: ProcessFaultV1::UnsupportedProcessResult { .. },
                } if status == ProcessExitCodeV1::reserved_fault()
            ));
        }
    }

    #[test]
    fn source_fault_keeps_diagnostic_and_uses_reserved_status() {
        let result = canonical(SourceEntryResultV1::Fault(SealedSourceFaultV1::new(
            "body-fault",
            "failed body".into(),
        )));
        assert_eq!(
            result,
            ProcessTerminationV1::Fault {
                status: ProcessExitCodeV1::reserved_fault(),
                fault: ProcessFaultV1::SourceFault {
                    code: "body-fault",
                    detail: "failed body".into(),
                },
            }
        );
    }

    #[test]
    fn legacy_profile_is_not_implicitly_connected() {
        let result = ProcessExitProjectionV1::project(
            SourceEntryResultV1::Integer(42),
            ProcessExitProfileV1::LegacyRunnerExitProjectionV1,
        );
        assert_eq!(
            result,
            Err(ProcessExitProjectionErrorV1::LegacyProfileDisconnected)
        );
    }
}
