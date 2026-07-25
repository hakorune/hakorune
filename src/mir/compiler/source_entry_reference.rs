//! Reference fixtures for the pure source-result/process projection.
//!
//! These tests intentionally do not compile or execute a VM module. They
//! exercise only the typed projection contract owned by `source_entry_result`.

#[cfg(test)]
mod tests {
    use super::super::source_entry_result::{
        CanonicalProcessExitV1, ProcessExitCodeV1, ProcessExitProfileV1, ProcessExitProjectionV1,
        ProcessFaultV1, ProcessTerminationV1, SealedObjectResultV1, SealedSourceFaultV1,
        SourceEntryResultKindV1, SourceEntryResultV1, UnitOriginV1,
    };

    fn canonical(result: SourceEntryResultV1) -> ProcessTerminationV1 {
        ProcessExitProjectionV1::project(
            result,
            ProcessExitProfileV1::Canonical(CanonicalProcessExitV1::V1),
        )
        .expect("canonical profile is connected")
    }

    #[test]
    fn unit_and_byte_range_integer_are_reference_successes() {
        assert_eq!(
            canonical(SourceEntryResultV1::Unit(UnitOriginV1::EmptyBody)),
            ProcessTerminationV1::Exit(ProcessExitCodeV1::zero()),
        );
        assert_eq!(
            canonical(SourceEntryResultV1::Integer(255)),
            ProcessTerminationV1::Exit(ProcessExitCodeV1::from_byte(255)),
        );
    }

    #[test]
    fn out_of_range_integer_is_never_wrapped() {
        for value in [-1, 256] {
            assert_eq!(
                canonical(SourceEntryResultV1::Integer(value)),
                ProcessTerminationV1::Fault {
                    status: ProcessExitCodeV1::reserved_fault(),
                    fault: ProcessFaultV1::ExitCodeOutOfRange { value },
                },
            );
        }
    }

    #[test]
    fn unsupported_scalar_and_object_are_typed_faults() {
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
                    fault: ProcessFaultV1::UnsupportedProcessResult {
                        kind: SourceEntryResultKindV1::Bool
                            | SourceEntryResultKindV1::Float
                            | SourceEntryResultKindV1::String
                            | SourceEntryResultKindV1::Object,
                    },
                } if status == ProcessExitCodeV1::reserved_fault()
            ));
        }
    }

    #[test]
    fn source_fault_keeps_reserved_status_and_diagnostic() {
        let termination = canonical(SourceEntryResultV1::Fault(SealedSourceFaultV1::new(
            "source-fault",
            "reference fixture".into(),
        )));
        assert!(matches!(
            termination,
            ProcessTerminationV1::Fault {
                status,
                fault: ProcessFaultV1::SourceFault {
                    code: "source-fault",
                    ..
                },
            } if status == ProcessExitCodeV1::reserved_fault()
        ));
    }
}
