//! The one explicit `raw-vm-reference` supported opt-in consumer.
//!
//! The central explicit-reference selector hands this owner one already-sealed
//! Raw request. It owns only source-file read, canonical parse, and the
//! already-sealed Raw VM-reference compiler entry.

use super::raw_vm_reference_request::{RawVmReferenceGrammarV1, RawVmReferenceProductionRequestV1};
use super::terminal::{ReferenceInvocationReportV1, ReferenceRunOutcomeV1, ReferenceUsageReportV1};

/// Run the supported reference lane from its one sealed request.
pub(crate) fn run(request: RawVmReferenceProductionRequestV1) -> ReferenceRunOutcomeV1 {
    #[cfg(not(feature = "vm-reference"))]
    {
        let _ = request;
        return ReferenceRunOutcomeV1::Usage(ReferenceUsageReportV1::new(
            "[raw-vm-reference/feature-unavailable] build with --features vm-reference",
        ));
    }

    #[cfg(feature = "vm-reference")]
    {
        let source_file = request.source_file().to_owned();
        let source = match std::fs::read_to_string(&source_file) {
            Ok(source) => source,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return ReferenceRunOutcomeV1::Usage(ReferenceUsageReportV1::new(format!(
                    "[raw-vm-reference/source/missing] file={source_file} error={error}"
                )));
            }
            Err(error) => {
                return ReferenceRunOutcomeV1::Invocation(ReferenceInvocationReportV1::new(
                    format!("[raw-vm-reference/source/read] file={source_file} error={error}"),
                ));
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
                return ReferenceRunOutcomeV1::Invocation(ReferenceInvocationReportV1::new(
                    format!("[raw-vm-reference/source/parse] {error:?}"),
                ));
            }
        };
        let optimize = request.optimize();
        let invocation = request.into_invocation(ast);
        let mut compiler = crate::mir::MirCompiler::with_options(optimize);
        match compiler.run_raw_vm_reference_v1(invocation) {
            Ok(report) => ReferenceRunOutcomeV1::Program(report),
            Err(error) => ReferenceRunOutcomeV1::Invocation(ReferenceInvocationReportV1::new(
                format!("[raw-vm-reference/invocation] {error}"),
            )),
        }
    }
}
