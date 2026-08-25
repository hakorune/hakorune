//! Affine conversion from one completed postpass into the normal-callable
//! parser owner. Every rejection closes the Program, coverage, parameter,
//! source-authority, and total-root siblings through one named terminal.

use super::super::callable_parameter_source::{
    ParserCallableParameterSourceDispositionV1, ParserNormalProgramSourceAuthorityDispositionV1,
    ParserNormalRootExecutionSourceDispositionV1,
};
use super::super::normal_callable_program_source::{
    NormalCallableParameterSourceRejectV1, NormalCallableParserCompatibilityV1 as Compatibility,
    ParsedNormalCallableProgramV1 as Program, ParserNormalRootExecutionPreservationIssuerV1,
    PreparedNormalCallableProgramSourceV1,
};
use super::{
    consume_box_coverage_at_named_terminal, consume_compatibility_callable_rows_at_named_terminal,
    consume_explain_at_named_terminal, consume_metadata_at_named_terminal,
    consume_static_box_parent_source_at_named_terminal, CompletedParserPostpassV1,
    CompletedParserProgramV1, NormalCallableProgramAdmissionV1, ParserBoxPostpassCoverageV1,
    ParserPostpassProgramCohortV1,
};

impl CompletedParserPostpassV1 {
    pub(in crate::parser) fn into_normal_callable_program(
        self,
        parameter_source: ParserCallableParameterSourceDispositionV1,
        source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
    ) -> Result<Program, NormalCallableParameterSourceRejectV1> {
        self.into_normal_callable_program_with_admission(
            parameter_source,
            source_authority,
            NormalCallableProgramAdmissionV1::Compatibility,
        )
    }

    pub(in crate::parser) fn into_normal_callable_program_with_root_execution(
        self,
        parameter_source: ParserCallableParameterSourceDispositionV1,
        source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
        normal_root_execution: ParserNormalRootExecutionSourceDispositionV1,
    ) -> Result<Program, NormalCallableParameterSourceRejectV1> {
        self.into_normal_callable_program_with_admission(
            parameter_source,
            source_authority,
            NormalCallableProgramAdmissionV1::SourceBacked(normal_root_execution),
        )
    }

    fn into_normal_callable_program_with_admission(
        self,
        parameter_source: ParserCallableParameterSourceDispositionV1,
        source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
        admission: NormalCallableProgramAdmissionV1,
    ) -> Result<Program, NormalCallableParameterSourceRejectV1> {
        let Self {
            program,
            metadata,
            explain,
            box_coverage,
            static_box_parent_source,
            normal_source_plan_seed,
        } = self;
        consume_metadata_at_named_terminal(metadata);
        consume_explain_at_named_terminal(explain);
        consume_static_box_parent_source_at_named_terminal(static_box_parent_source);
        normal_source_plan_seed.discard_unconnected();

        if source_authority.composite_source_is_ready() && program.is_compatibility() {
            discard_pretransform_normal_callable_at_named_terminal(
                program,
                box_coverage,
                parameter_source,
                source_authority,
                admission,
            );
            return Err(NormalCallableParameterSourceRejectV1::CompositeSourceCompatibilityLoss);
        }

        match (program, admission) {
            (
                CompletedParserProgramV1::Initial(program),
                NormalCallableProgramAdmissionV1::SourceBacked(normal_root_execution),
            ) => {
                let ordinary_box_coverage = box_coverage.into_source_backed_ordinary_coverage();
                PreparedNormalCallableProgramSourceV1::issue(
                    program,
                    parameter_source,
                    source_authority,
                    normal_root_execution,
                    ordinary_box_coverage,
                )
                .map(Program::SourceBacked)
            }
            (
                program @ CompletedParserProgramV1::Initial(_),
                admission @ NormalCallableProgramAdmissionV1::Compatibility,
            ) => {
                discard_pretransform_normal_callable_at_named_terminal(
                    program,
                    box_coverage,
                    parameter_source,
                    source_authority,
                    admission,
                );
                Err(NormalCallableParameterSourceRejectV1::MainAppEntryCompatibilityLoss)
            }
            (
                CompletedParserProgramV1::Compatibility { ast, callable_rows },
                NormalCallableProgramAdmissionV1::Compatibility,
            ) => {
                let cohort = compatibility_from_cohort(box_coverage.program_cohort);
                consume_compatibility_callable_rows_at_named_terminal(callable_rows);
                consume_box_coverage_at_named_terminal(box_coverage);
                consume_parameter_source_at_named_terminal(parameter_source);
                consume_source_authority_at_named_terminal(source_authority);
                Ok(Program::Compatibility { ast, cohort })
            }
            (
                program @ CompletedParserProgramV1::Compatibility { .. },
                admission @ NormalCallableProgramAdmissionV1::SourceBacked(_),
            ) => {
                discard_pretransform_normal_callable_at_named_terminal(
                    program,
                    box_coverage,
                    parameter_source,
                    source_authority,
                    admission,
                );
                Err(NormalCallableParameterSourceRejectV1::MainAppEntryCompatibilityLoss)
            }
        }
    }
}

fn compatibility_from_cohort(cohort: ParserPostpassProgramCohortV1) -> Compatibility {
    match cohort {
        ParserPostpassProgramCohortV1::InterfaceBox => Compatibility::InterfaceBox,
        ParserPostpassProgramCohortV1::RecordBox => Compatibility::RecordBox,
        ParserPostpassProgramCohortV1::MixedProgram | ParserPostpassProgramCohortV1::StaticBox => {
            Compatibility::MixedProgram
        }
        ParserPostpassProgramCohortV1::TopLevelBuildGate => Compatibility::TopLevelBuildGate,
        ParserPostpassProgramCohortV1::NoBoxDeclarations => Compatibility::NoBoxDeclarations,
        ParserPostpassProgramCohortV1::NonProgram => Compatibility::NonProgram,
        ParserPostpassProgramCohortV1::OrdinaryTopLevelBox => {
            Compatibility::UnsupportedCallableSource
        }
    }
}

fn discard_pretransform_normal_callable_at_named_terminal(
    program: CompletedParserProgramV1,
    box_coverage: ParserBoxPostpassCoverageV1,
    parameter_source: ParserCallableParameterSourceDispositionV1,
    source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
    admission: NormalCallableProgramAdmissionV1,
) {
    drop(program.into_ast_at_named_terminal());
    consume_box_coverage_at_named_terminal(box_coverage);
    consume_parameter_source_at_named_terminal(parameter_source);
    consume_source_authority_at_named_terminal(source_authority);
    match admission {
        NormalCallableProgramAdmissionV1::Compatibility => {}
        NormalCallableProgramAdmissionV1::SourceBacked(root) => {
            ParserNormalRootExecutionPreservationIssuerV1::
                discard_at_named_transform_reject_terminal(root);
        }
    }
}

fn consume_parameter_source_at_named_terminal(source: ParserCallableParameterSourceDispositionV1) {
    match source {
        ParserCallableParameterSourceDispositionV1::Complete(catalog) => drop(catalog),
        ParserCallableParameterSourceDispositionV1::SelectedBuildGateUnsupported => {}
    }
}

fn consume_source_authority_at_named_terminal(
    source: ParserNormalProgramSourceAuthorityDispositionV1,
) {
    match source {
        ParserNormalProgramSourceAuthorityDispositionV1::Ready(authority) => drop(authority),
        ParserNormalProgramSourceAuthorityDispositionV1::SourceAuthorityUnavailable(error) => {
            let _consumed_error = error;
        }
        ParserNormalProgramSourceAuthorityDispositionV1::Incomplete(error) => {
            let _consumed_error = error;
        }
        ParserNormalProgramSourceAuthorityDispositionV1::IntegrityInvalid(error) => {
            let _consumed_error = error;
        }
    }
}
