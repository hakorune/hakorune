//! S3-DIAGNOSTIC0: typed VM-reference fault presentation.
//!
//! This adapter formats a fault. It never owns or changes process status and
//! it has no retry/fallback path.

use super::source_entry_result::{
    ProcessFaultDiagnosticFieldsV1, ProcessFaultV1, SourceEntryResultKindV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum VmReferenceProcessDiagnosticReportV1 {
    ExitCodeOutOfRange {
        value: i64,
        accepted_min: u8,
        accepted_max: u8,
    },
    UnsupportedProcessResult {
        kind: SourceEntryResultKindV1,
    },
    SourceFault {
        code: &'static str,
        detail: Box<str>,
    },
}

pub(in crate::mir) struct VmReferenceProcessDiagnosticAdapterV1;

impl VmReferenceProcessDiagnosticAdapterV1 {
    pub(in crate::mir) fn project(fault: &ProcessFaultV1) -> VmReferenceProcessDiagnosticReportV1 {
        match fault.diagnostic_fields() {
            ProcessFaultDiagnosticFieldsV1::ExitCodeOutOfRange { value } => {
                VmReferenceProcessDiagnosticReportV1::ExitCodeOutOfRange {
                    value,
                    accepted_min: 0,
                    accepted_max: u8::MAX,
                }
            }
            ProcessFaultDiagnosticFieldsV1::UnsupportedProcessResult { kind } => {
                VmReferenceProcessDiagnosticReportV1::UnsupportedProcessResult { kind }
            }
            ProcessFaultDiagnosticFieldsV1::SourceFault { code, detail } => {
                VmReferenceProcessDiagnosticReportV1::SourceFault {
                    code,
                    detail: detail.into(),
                }
            }
        }
    }

    pub(in crate::mir) fn tag(report: &VmReferenceProcessDiagnosticReportV1) -> &'static str {
        match report {
            VmReferenceProcessDiagnosticReportV1::ExitCodeOutOfRange { .. } => {
                "[process/exit-code-out-of-range]"
            }
            VmReferenceProcessDiagnosticReportV1::UnsupportedProcessResult { .. } => {
                "[process/unsupported-result]"
            }
            VmReferenceProcessDiagnosticReportV1::SourceFault { .. } => "[process/source-fault]",
        }
    }

    pub(in crate::mir) fn line(report: &VmReferenceProcessDiagnosticReportV1) -> String {
        match report {
            VmReferenceProcessDiagnosticReportV1::ExitCodeOutOfRange {
                value,
                accepted_min,
                accepted_max,
            } => format!(
                "{} value={} accepted={}..={}",
                Self::tag(report),
                value,
                accepted_min,
                accepted_max
            ),
            VmReferenceProcessDiagnosticReportV1::UnsupportedProcessResult { kind } => {
                format!("{} kind={}", Self::tag(report), kind.stable_name())
            }
            VmReferenceProcessDiagnosticReportV1::SourceFault { code, detail } => format!(
                "{} code={} detail={}",
                Self::tag(report),
                code,
                sanitize_detail(detail)
            ),
        }
    }
}

fn sanitize_detail(detail: &str) -> String {
    detail.replace(['\r', '\n'], " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_preserves_typed_source_fault() {
        let fault = ProcessFaultV1::SourceFault {
            code: "vm-invalid-instruction",
            detail: "bad opcode".into(),
        };
        let report = VmReferenceProcessDiagnosticAdapterV1::project(&fault);
        assert_eq!(
            VmReferenceProcessDiagnosticAdapterV1::tag(&report),
            "[process/source-fault]"
        );
        assert_eq!(
            report,
            VmReferenceProcessDiagnosticReportV1::SourceFault {
                code: "vm-invalid-instruction",
                detail: "bad opcode".into(),
            }
        );
    }

    #[test]
    fn report_keeps_range_and_unsupported_kinds_structured() {
        let range = ProcessFaultV1::ExitCodeOutOfRange { value: 256 };
        let unsupported = ProcessFaultV1::UnsupportedProcessResult {
            kind: SourceEntryResultKindV1::Bool,
        };
        assert!(matches!(
            VmReferenceProcessDiagnosticAdapterV1::project(&range),
            VmReferenceProcessDiagnosticReportV1::ExitCodeOutOfRange {
                value: 256,
                accepted_min: 0,
                accepted_max: 255
            }
        ));
        assert!(matches!(
            VmReferenceProcessDiagnosticAdapterV1::project(&unsupported),
            VmReferenceProcessDiagnosticReportV1::UnsupportedProcessResult {
                kind: SourceEntryResultKindV1::Bool
            }
        ));
    }

    #[test]
    fn diagnostic_line_is_single_line_and_keeps_fault_identity() {
        let fault = ProcessFaultV1::SourceFault {
            code: "vm-fault",
            detail: "first\nsecond\rthird".into(),
        };
        let report = VmReferenceProcessDiagnosticAdapterV1::project(&fault);
        assert_eq!(
            VmReferenceProcessDiagnosticAdapterV1::line(&report),
            "[process/source-fault] code=vm-fault detail=first second third"
        );
    }
}
