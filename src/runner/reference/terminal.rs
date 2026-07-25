//! Shared terminal for explicit reference runners.

#[cfg(feature = "vm-reference")]
use crate::mir::RawVmReferenceRunReportV1;
use std::io::Write;

#[derive(Debug)]
pub(crate) struct ReferenceUsageReportV1 {
    line: Box<str>,
}

impl ReferenceUsageReportV1 {
    pub(crate) fn new(line: impl Into<Box<str>>) -> Self {
        Self { line: line.into() }
    }

    #[cfg(test)]
    pub(crate) fn line(&self) -> &str {
        &self.line
    }
}

#[derive(Debug)]
pub(crate) struct ReferenceInvocationReportV1 {
    line: Box<str>,
}

impl ReferenceInvocationReportV1 {
    pub(crate) fn new(line: impl Into<Box<str>>) -> Self {
        Self { line: line.into() }
    }

    #[cfg(test)]
    pub(crate) fn line(&self) -> &str {
        &self.line
    }
}

#[derive(Debug)]
pub(crate) enum ReferenceRunOutcomeV1 {
    Usage(ReferenceUsageReportV1),
    Invocation(ReferenceInvocationReportV1),
    #[cfg(feature = "vm-reference")]
    Program(RawVmReferenceRunReportV1),
}

pub(crate) struct ReferenceRunTerminalV1;

impl ReferenceRunTerminalV1 {
    pub(crate) fn finish(outcome: ReferenceRunOutcomeV1) -> ! {
        let code = match outcome {
            ReferenceRunOutcomeV1::Usage(report) => {
                write_stderr_line(&report.line);
                2
            }
            ReferenceRunOutcomeV1::Invocation(report) => {
                write_stderr_line(&report.line);
                1
            }
            #[cfg(feature = "vm-reference")]
            ReferenceRunOutcomeV1::Program(report) => {
                if let Some(line) = report.diagnostic_line() {
                    write_stderr_line(&line);
                }
                report.status_code()
            }
        };
        std::process::exit(i32::from(code));
    }
}

fn write_stderr_line(line: &str) {
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(stderr, "{line}");
}
