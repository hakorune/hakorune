//! The one explicit `raw-vm-reference` supported opt-in consumer.
//!
//! This module is entered from the first statement of `run_refactored`. A
//! `NotSelected` result falls through to the existing runner unchanged. The
//! selected branch owns only source-file read, canonical parse, and the
//! already-sealed Raw VM-reference compiler entry.

use super::raw_vm_reference_request::{
    RawVmReferenceGrammarV1, RawVmReferenceProductionRequestV1, RawVmReferenceProfileSelectionV1,
};
use crate::cli::CliConfig;
#[cfg(feature = "vm-reference")]
use crate::mir::RawVmReferenceRunReportV1;
use std::io::Write;

#[derive(Debug)]
pub(crate) enum RawVmReferenceRunOutcome {
    Usage(String),
    Invocation(String),
    #[cfg(feature = "vm-reference")]
    Program(RawVmReferenceRunReportV1),
}

impl RawVmReferenceRunOutcome {
    pub(crate) fn finish(self) -> ! {
        let code = match self {
            Self::Usage(line) => {
                write_stderr_line(&line);
                2
            }
            Self::Invocation(line) => {
                write_stderr_line(&line);
                1
            }
            #[cfg(feature = "vm-reference")]
            Self::Program(report) => {
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

/// Select and run the supported reference lane exactly once. `None` is the byte-for-byte
/// default-route fallthrough; `Some` is terminal and must call `finish`.
pub(crate) fn select_and_run(config: &CliConfig) -> Option<RawVmReferenceRunOutcome> {
    let selection = match RawVmReferenceProductionRequestV1::select_from_cli(config) {
        Ok(selection) => selection,
        Err(error) => {
            return Some(RawVmReferenceRunOutcome::Usage(format!(
                "[raw-vm-reference/profile/rejected] {}",
                error.code()
            )))
        }
    };
    let request = match selection {
        RawVmReferenceProfileSelectionV1::NotSelected => return None,
        RawVmReferenceProfileSelectionV1::Selected(request) => request,
    };

    #[cfg(not(feature = "vm-reference"))]
    {
        let _ = request;
        return Some(RawVmReferenceRunOutcome::Usage(
            "[raw-vm-reference/feature-unavailable] build with --features vm-reference".to_owned(),
        ));
    }

    #[cfg(feature = "vm-reference")]
    {
        let source_file = request.source_file().to_owned();
        let source = match std::fs::read_to_string(&source_file) {
            Ok(source) => source,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Some(RawVmReferenceRunOutcome::Usage(format!(
                    "[raw-vm-reference/source/missing] file={source_file} error={error}"
                )))
            }
            Err(error) => {
                return Some(RawVmReferenceRunOutcome::Invocation(format!(
                    "[raw-vm-reference/source/read] file={source_file} error={error}"
                )))
            }
        };
        let grammar_profile = match request.grammar() {
            RawVmReferenceGrammarV1::Canonical => {
                hakorune_frontend_parser::parser::GrammarProfile::Canonical
            }
        };
        let parser_config = crate::parser::ParserBuildConfig {
            grammar_profile,
            ..Default::default()
        };
        let ast = match crate::parser::NyashParser::parse_from_string_with_build_config(
            source,
            parser_config,
        ) {
            Ok(ast) => ast,
            Err(error) => {
                return Some(RawVmReferenceRunOutcome::Invocation(format!(
                    "[raw-vm-reference/source/parse] {error:?}"
                )))
            }
        };
        let optimize = request.optimize();
        let invocation = request.into_invocation(ast);
        let mut compiler = crate::mir::MirCompiler::with_options(optimize);
        match compiler.run_raw_vm_reference_v1(invocation) {
            Ok(report) => Some(RawVmReferenceRunOutcome::Program(report)),
            Err(error) => Some(RawVmReferenceRunOutcome::Invocation(format!(
                "[raw-vm-reference/invocation] {error}"
            ))),
        }
    }
}
