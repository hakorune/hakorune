use super::canonical_script_source_admission::issue_canonical_script_cohort;
use super::catalog::{
    ParserCallableParameterSourceCatalogV1, ParserCallableParameterSourceDispositionV1,
};
use super::composite_source::issue_parser_composite_source_v1;
use super::normal_root_execution::{
    ParserNormalRootExecutionIssuerV1, ParserNormalRootExecutionSourceDispositionV1,
};
use super::normal_source_plan_surface::ParserNormalSourcePlanSurfaceIssuerV1;
use super::retained::RetainedParserCallableSemanticSourceV1;
use super::script_source_authority::{
    issue_parser_normal_program_source_authority_v1,
    ParserNormalProgramSourceAuthorityDispositionV1,
};
use super::script_source_rows::{
    issue_canonical_script_source_rows, CanonicalScriptSourceRowsDispositionV1,
};
#[cfg(test)]
use super::syntax_loan::{
    borrow_callable_declaration_syntax_v1, ParserCallableDeclarationSyntaxLoanV1,
    ParserCallableSyntaxLoanErrorV1,
};
use crate::parser::postpass_envelope::CompletedParserPostpassV1;
use crate::parser::{NyashParser, ParseError, ParsedNormalCallableProgramV1, ParserBuildConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserCallableSourceRetentionErrorV1 {
    ParameterSourceUnavailable,
    CompositeSourceReadyCannotBeDiscarded,
}

#[derive(Debug)]
pub(crate) struct RejectedParserCallableSourceRetentionV1 {
    source: ParsedProgramWithCallableParameterSourceV1,
    error: ParserCallableSourceRetentionErrorV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRawVmSourceKindV1 {
    SourceBacked,
    Compatibility,
}

#[derive(Debug)]
pub(crate) enum PreparedParserNormalRawVmSourceRouteV1 {
    SourceBacked(PreparedParserNormalSourceBackedRawVmV1),
    Compatibility(PreparedParserNormalCompatibilityRawVmV1),
}

#[derive(Debug)]
pub(crate) struct PreparedParserNormalSourceBackedRawVmV1 {
    completed: CompletedParserPostpassV1,
    parameter_source: ParserCallableParameterSourceDispositionV1,
    source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
    root: super::normal_root_execution::ParserNormalRootExecutionSourceV1,
    _seal: PreparedParserNormalSourceBackedRawVmSealV1,
}

#[derive(Debug)]
struct PreparedParserNormalSourceBackedRawVmSealV1;

#[derive(Debug)]
pub(crate) struct PreparedParserNormalCompatibilityRawVmV1 {
    completed: CompletedParserPostpassV1,
    parameter_source: ParserCallableParameterSourceDispositionV1,
    source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
    compatibility: super::normal_root_execution::ParserNormalRootExecutionCompatibilityClosureV1,
    _seal: PreparedParserNormalCompatibilityRawVmSealV1,
}

#[derive(Debug)]
struct PreparedParserNormalCompatibilityRawVmSealV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRawVmSourceExtractionErrorV1 {
    SourceAuthorityUnavailable,
    Incomplete,
    IntegrityInvalid,
    CompatibilityClosure,
}

#[derive(Debug)]
pub(crate) struct RejectedParserNormalRawVmSourceExtractionV1 {
    source: ParsedProgramWithCallableParameterSourceV1,
    error: ParserNormalRawVmSourceExtractionErrorV1,
}

/// One-shot total parser result plus its sibling callable parameter source
/// catalog. Neither side can be paired with a product from another invocation.
#[derive(Debug)]
pub(crate) struct ParsedProgramWithCallableParameterSourceV1 {
    completed: CompletedParserPostpassV1,
    parameter_source: ParserCallableParameterSourceDispositionV1,
    canonical_script_source_rows: CanonicalScriptSourceRowsDispositionV1,
    source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
    normal_root_execution: ParserNormalRootExecutionSourceDispositionV1,
}

#[derive(Debug)]
pub(crate) struct PreparedParserNormalFileSourceV1 {
    source: ParsedProgramWithCallableParameterSourceV1,
    script_rows: CanonicalScriptSourceRowsDispositionV1,
    _seal: PreparedParserNormalFileSourceSealV1,
}

#[derive(Debug)]
struct PreparedParserNormalFileSourceSealV1;

impl NyashParser {
    pub(crate) fn parse_from_string_with_callable_parameter_source(
        input: impl Into<String>,
        build_config: ParserBuildConfig,
    ) -> Result<ParsedProgramWithCallableParameterSourceV1, ParseError> {
        crate::parser::string_postpass_entry::parse_with_callable_parameter_source(
            input.into(),
            Some(100_000),
            build_config,
        )
    }
}

impl ParsedProgramWithCallableParameterSourceV1 {
    pub(crate) fn prepare_raw_vm_source_route(
        self,
    ) -> Result<PreparedParserNormalRawVmSourceRouteV1, RejectedParserNormalRawVmSourceExtractionV1>
    {
        let Self {
            completed,
            parameter_source,
            canonical_script_source_rows,
            source_authority,
            normal_root_execution,
        } = self;
        if !matches!(
            &canonical_script_source_rows,
            CanonicalScriptSourceRowsDispositionV1::MovedToParallelHandoff
        ) {
            return Err(RejectedParserNormalRawVmSourceExtractionV1 {
                source: Self {
                    completed,
                    parameter_source,
                    canonical_script_source_rows,
                    source_authority,
                    normal_root_execution,
                },
                error: ParserNormalRawVmSourceExtractionErrorV1::IntegrityInvalid,
            });
        }
        match normal_root_execution {
            ParserNormalRootExecutionSourceDispositionV1::Ready(root) => Ok(
                PreparedParserNormalRawVmSourceRouteV1::SourceBacked(
                    PreparedParserNormalSourceBackedRawVmV1 {
                        completed,
                        parameter_source,
                        source_authority,
                        root,
                        _seal: PreparedParserNormalSourceBackedRawVmSealV1,
                    },
                ),
            ),
            compatibility @ ParserNormalRootExecutionSourceDispositionV1::
                SourceAuthorityUnavailable(
                    super::normal_source_plan_surface::
                        ParserNormalSourcePlanSurfaceUnavailableV1::PostpassNotSourceBacked,
                ) => {
                match super::normal_root_execution::
                    ParserNormalRootExecutionCompatibilityClosureV1::consume_once(compatibility)
                {
                    Ok(compatibility) => Ok(
                        PreparedParserNormalRawVmSourceRouteV1::Compatibility(
                            PreparedParserNormalCompatibilityRawVmV1 {
                                completed,
                                parameter_source,
                                source_authority,
                                compatibility,
                                _seal: PreparedParserNormalCompatibilityRawVmSealV1,
                            },
                        ),
                    ),
                    Err((normal_root_execution, _)) => {
                        Err(RejectedParserNormalRawVmSourceExtractionV1 {
                            source: Self {
                                completed,
                                parameter_source,
                                canonical_script_source_rows:
                                    CanonicalScriptSourceRowsDispositionV1::
                                        MovedToParallelHandoff,
                                source_authority,
                                normal_root_execution,
                            },
                            error: ParserNormalRawVmSourceExtractionErrorV1::
                                CompatibilityClosure,
                        })
                    }
                }
            }
            normal_root_execution @ ParserNormalRootExecutionSourceDispositionV1::
                SourceAuthorityUnavailable(_) => {
                Err(RejectedParserNormalRawVmSourceExtractionV1 {
                    source: Self {
                        completed,
                        parameter_source,
                        canonical_script_source_rows:
                            CanonicalScriptSourceRowsDispositionV1::MovedToParallelHandoff,
                        source_authority,
                        normal_root_execution,
                    },
                    error: ParserNormalRawVmSourceExtractionErrorV1::
                        SourceAuthorityUnavailable,
                })
            }
            normal_root_execution @ ParserNormalRootExecutionSourceDispositionV1::Incomplete(_) => {
                Err(RejectedParserNormalRawVmSourceExtractionV1 {
                    source: Self {
                        completed,
                        parameter_source,
                        canonical_script_source_rows:
                            CanonicalScriptSourceRowsDispositionV1::MovedToParallelHandoff,
                        source_authority,
                        normal_root_execution,
                    },
                    error: ParserNormalRawVmSourceExtractionErrorV1::Incomplete,
                })
            }
            normal_root_execution @ ParserNormalRootExecutionSourceDispositionV1::
                IntegrityInvalid(_) => {
                Err(RejectedParserNormalRawVmSourceExtractionV1 {
                    source: Self {
                        completed,
                        parameter_source,
                        canonical_script_source_rows:
                            CanonicalScriptSourceRowsDispositionV1::MovedToParallelHandoff,
                        source_authority,
                        normal_root_execution,
                    },
                    error: ParserNormalRawVmSourceExtractionErrorV1::IntegrityInvalid,
                })
            }
        }
    }

    pub(crate) fn into_normal_file_source(self) -> PreparedParserNormalFileSourceV1 {
        let Self {
            completed,
            parameter_source,
            canonical_script_source_rows,
            source_authority,
            normal_root_execution,
        } = self;
        PreparedParserNormalFileSourceV1 {
            source: Self {
                completed,
                parameter_source,
                canonical_script_source_rows:
                    CanonicalScriptSourceRowsDispositionV1::MovedToParallelHandoff,
                source_authority,
                normal_root_execution,
            },
            script_rows: canonical_script_source_rows,
            _seal: PreparedParserNormalFileSourceSealV1,
        }
    }

    pub(super) fn normal_root_execution_for_consumer(
        &self,
    ) -> &ParserNormalRootExecutionSourceDispositionV1 {
        &self.normal_root_execution
    }

    pub(super) fn source_ast_for_bound_terminal(&self) -> &crate::ast::ASTNode {
        self.completed.ast()
    }

    pub(super) fn into_ast_after_source_plan_terminal(self) -> crate::ast::ASTNode {
        let Self {
            completed,
            parameter_source,
            canonical_script_source_rows,
            source_authority,
            normal_root_execution,
        } = self;
        consume_parameter_source_at_named_terminal(parameter_source);
        consume_script_rows_for_normal_callable(canonical_script_source_rows);
        consume_program_source_authority_at_named_terminal(source_authority);
        consume_root_disposition_at_named_terminal(normal_root_execution);
        completed.into_ast()
    }

    pub(crate) fn discard_after_source_plan_rejection(self) {
        discard_parser_callable_source_at_named_terminal(self);
    }

    pub(in crate::parser) fn new(
        completed: CompletedParserPostpassV1,
        parameter_source: ParserCallableParameterSourceDispositionV1,
    ) -> Self {
        let mut completed = completed;
        let source_plan_seed = completed.consume_normal_source_plan_seed();
        let normal_source_plan_surface = ParserNormalSourcePlanSurfaceIssuerV1::issue_once(
            &completed,
            &parameter_source,
            source_plan_seed,
        );
        let normal_root_execution =
            ParserNormalRootExecutionIssuerV1::issue_once(normal_source_plan_surface);
        let canonical_script_admission =
            issue_canonical_script_cohort(&completed, &parameter_source);
        let canonical_script_source_rows = match &parameter_source {
            ParserCallableParameterSourceDispositionV1::Complete(catalog) => {
                issue_canonical_script_source_rows(&completed, catalog, &canonical_script_admission)
            }
            ParserCallableParameterSourceDispositionV1::SelectedBuildGateUnsupported => {
                CanonicalScriptSourceRowsDispositionV1::AdmissionMissing
            }
        };
        let composite_source = issue_parser_composite_source_v1(&completed, &parameter_source);
        let source_authority = issue_parser_normal_program_source_authority_v1(
            &completed,
            &parameter_source,
            composite_source,
        );
        Self {
            completed,
            parameter_source,
            canonical_script_source_rows,
            source_authority,
            normal_root_execution,
        }
    }

    /// Move the atomic parser result into the retained source owner used by
    /// the future sole callable semantic batch.
    ///
    /// This transition exposes neither the AST nor the parameter catalog as
    /// independently movable parts.
    pub(crate) fn into_retained_source(
        self,
    ) -> Result<RetainedParserCallableSemanticSourceV1, RejectedParserCallableSourceRetentionV1>
    {
        let error = if !matches!(
            self.parameter_source,
            ParserCallableParameterSourceDispositionV1::Complete(_)
        ) {
            Some(ParserCallableSourceRetentionErrorV1::ParameterSourceUnavailable)
        } else if self.source_authority.composite_source_is_ready() {
            Some(ParserCallableSourceRetentionErrorV1::CompositeSourceReadyCannotBeDiscarded)
        } else {
            None
        };
        if let Some(error) = error {
            return Err(RejectedParserCallableSourceRetentionV1 {
                source: self,
                error,
            });
        }
        let Self {
            completed,
            parameter_source,
            canonical_script_source_rows,
            source_authority,
            normal_root_execution,
        } = self;
        let ParserCallableParameterSourceDispositionV1::Complete(catalog) = parameter_source else {
            unreachable!("retention preflight admitted only a complete parameter source")
        };
        Ok(RetainedParserCallableSemanticSourceV1::new(
            completed,
            catalog,
            source_authority,
            canonical_script_source_rows,
            normal_root_execution,
        ))
    }

    pub(crate) fn into_normal_callable_program(
        self,
    ) -> Result<ParsedNormalCallableProgramV1, ParseError> {
        let Self {
            completed,
            parameter_source,
            canonical_script_source_rows,
            source_authority,
            normal_root_execution,
        } = self;
        consume_script_rows_for_normal_callable(canonical_script_source_rows);
        let parsed = if completed.is_source_backed() {
            completed.into_normal_callable_program_with_root_execution(
                parameter_source,
                source_authority,
                normal_root_execution,
            )
        } else {
            let compatibility = match super::normal_root_execution::
                ParserNormalRootExecutionCompatibilityClosureV1::consume_once(
                    normal_root_execution,
                ) {
                Ok(compatibility) => compatibility,
                Err((normal_root_execution, reject)) => {
                    discard_normal_callable_compatibility_attempt_at_named_terminal(
                        completed,
                        parameter_source,
                        source_authority,
                        normal_root_execution,
                        reject,
                    );
                    return Err(ParseError::GrammarContract {
                        stable_reject_tag: "parser/normal-callable-root-compatibility",
                        detail: "source-backed root state cannot enter compatibility".to_owned(),
                        line: 0,
                    });
                }
            };
            consume_compatibility_closure_at_named_terminal(compatibility);
            completed.into_normal_callable_program(parameter_source, source_authority)
        };
        parsed.map_err(normal_callable_source_reject)
    }

    #[cfg(test)]
    pub(super) fn observe_test_terminal_once<R>(
        self,
        callback: impl for<'source> FnOnce(
            super::normal_root_execution::ParserNormalRootExecutionTestLoanV1<'source>,
        ) -> R,
    ) -> R {
        let module_rows = match &self.source_authority {
            ParserNormalProgramSourceAuthorityDispositionV1::Ready(authority) => {
                Some(authority.module_rows())
            }
            ParserNormalProgramSourceAuthorityDispositionV1::SourceAuthorityUnavailable(_)
            | ParserNormalProgramSourceAuthorityDispositionV1::Incomplete(_)
            | ParserNormalProgramSourceAuthorityDispositionV1::IntegrityInvalid(_) => None,
        };
        let observation = callback(
            super::normal_root_execution::ParserNormalRootExecutionTestLoanV1::issue(
                &self.parameter_source,
                self.completed.static_box_parent_source(),
                &self.canonical_script_source_rows,
                &self.normal_root_execution,
                module_rows,
            ),
        );
        discard_parser_callable_source_at_named_terminal(self);
        observation
    }

    #[cfg(test)]
    pub(super) fn consume_test_terminal_once<R>(
        self,
        callback: impl for<'syntax> FnOnce(
            &'syntax ParserCallableParameterSourceCatalogV1,
            ParserCallableDeclarationSyntaxLoanV1<'syntax>,
        ) -> R,
    ) -> Result<R, ParserCallableSyntaxLoanErrorV1> {
        let Self {
            completed,
            parameter_source,
            canonical_script_source_rows,
            source_authority,
            normal_root_execution,
        } = self;
        if source_authority.composite_source_is_ready() {
            consume_parameter_source_at_named_terminal(parameter_source);
            consume_root_disposition_at_named_terminal(normal_root_execution);
            consume_script_rows_for_normal_callable(canonical_script_source_rows);
            consume_completed_postpass_at_test_terminal(completed);
            consume_program_source_authority_at_named_terminal(source_authority);
            return Err(ParserCallableSyntaxLoanErrorV1::CompositeSourceReadyCannotBeDiscarded);
        }
        let catalog = match parameter_source {
            ParserCallableParameterSourceDispositionV1::Complete(catalog) => catalog,
            parameter_source @ ParserCallableParameterSourceDispositionV1::SelectedBuildGateUnsupported => {
                consume_parameter_source_at_named_terminal(parameter_source);
                consume_root_disposition_at_named_terminal(normal_root_execution);
                consume_script_rows_for_normal_callable(canonical_script_source_rows);
                consume_completed_postpass_at_test_terminal(completed);
                consume_program_source_authority_at_named_terminal(source_authority);
                return Err(ParserCallableSyntaxLoanErrorV1::ParameterSourceUnavailable);
            }
        };
        let loan = match borrow_callable_declaration_syntax_v1(completed.ast(), &catalog) {
            Ok(loan) => loan,
            Err(error) => {
                consume_parameter_source_at_named_terminal(
                    ParserCallableParameterSourceDispositionV1::Complete(catalog),
                );
                consume_root_disposition_at_named_terminal(normal_root_execution);
                consume_script_rows_for_normal_callable(canonical_script_source_rows);
                consume_completed_postpass_at_test_terminal(completed);
                consume_program_source_authority_at_named_terminal(source_authority);
                return Err(error);
            }
        };
        let observation = callback(&catalog, loan);
        consume_parameter_source_at_named_terminal(
            ParserCallableParameterSourceDispositionV1::Complete(catalog),
        );
        consume_root_disposition_at_named_terminal(normal_root_execution);
        consume_script_rows_for_normal_callable(canonical_script_source_rows);
        consume_completed_postpass_at_test_terminal(completed);
        consume_program_source_authority_at_named_terminal(source_authority);
        Ok(observation)
    }
}

impl PreparedParserNormalSourceBackedRawVmV1 {
    pub(crate) fn into_ast_after_named_raw_discard(self) -> crate::ast::ASTNode {
        consume_parameter_source_at_named_terminal(self.parameter_source);
        consume_program_source_authority_at_named_terminal(self.source_authority);
        consume_ready_root_for_raw(self.root);
        self.completed.into_ast()
    }
}

impl PreparedParserNormalCompatibilityRawVmV1 {
    pub(crate) fn into_ast_after_named_compatibility_extraction(self) -> crate::ast::ASTNode {
        consume_parameter_source_at_named_terminal(self.parameter_source);
        consume_program_source_authority_at_named_terminal(self.source_authority);
        consume_compatibility_closure_at_named_terminal(self.compatibility);
        self.completed.into_ast()
    }
}

impl RejectedParserNormalRawVmSourceExtractionV1 {
    pub(crate) const fn error(&self) -> ParserNormalRawVmSourceExtractionErrorV1 {
        self.error
    }

    pub(crate) fn discard(self) {
        discard_parser_callable_source_at_named_terminal(self.source);
    }
}

impl RejectedParserCallableSourceRetentionV1 {
    pub(crate) const fn error(&self) -> ParserCallableSourceRetentionErrorV1 {
        self.error
    }

    pub(crate) fn discard(self) {
        discard_parser_callable_source_at_named_terminal(self.source);
    }
}

impl PreparedParserNormalFileSourceV1 {
    pub(crate) fn consume_once<T>(
        self,
        consume: impl FnOnce(
            ParsedProgramWithCallableParameterSourceV1,
            CanonicalScriptSourceRowsDispositionV1,
        ) -> T,
    ) -> T {
        consume(self.source, self.script_rows)
    }
}

fn consume_script_rows_for_normal_callable(rows: CanonicalScriptSourceRowsDispositionV1) {
    rows.discard_at_named_terminal();
}

fn discard_parser_callable_source_at_named_terminal(
    source: ParsedProgramWithCallableParameterSourceV1,
) {
    let ParsedProgramWithCallableParameterSourceV1 {
        completed,
        parameter_source,
        canonical_script_source_rows,
        source_authority,
        normal_root_execution,
    } = source;
    consume_parameter_source_at_named_terminal(parameter_source);
    consume_script_rows_for_normal_callable(canonical_script_source_rows);
    consume_program_source_authority_at_named_terminal(source_authority);
    consume_root_disposition_at_named_terminal(normal_root_execution);
    drop(completed.into_ast());
}

fn consume_parameter_source_at_named_terminal(source: ParserCallableParameterSourceDispositionV1) {
    match source {
        ParserCallableParameterSourceDispositionV1::Complete(catalog) => drop(catalog),
        ParserCallableParameterSourceDispositionV1::SelectedBuildGateUnsupported => {}
    }
}

fn consume_program_source_authority_at_named_terminal(
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

fn consume_root_disposition_at_named_terminal(root: ParserNormalRootExecutionSourceDispositionV1) {
    match root {
        ParserNormalRootExecutionSourceDispositionV1::Ready(source) => drop(source),
        ParserNormalRootExecutionSourceDispositionV1::SourceAuthorityUnavailable(error) => {
            let _consumed_error = error;
        }
        ParserNormalRootExecutionSourceDispositionV1::Incomplete(error) => {
            let _consumed_error = error;
        }
        ParserNormalRootExecutionSourceDispositionV1::IntegrityInvalid(error) => {
            let _consumed_error = error;
        }
    }
}

fn consume_compatibility_closure_at_named_terminal(
    closure: super::normal_root_execution::ParserNormalRootExecutionCompatibilityClosureV1,
) {
    drop(closure);
}

fn discard_normal_callable_compatibility_attempt_at_named_terminal(
    completed: CompletedParserPostpassV1,
    parameter_source: ParserCallableParameterSourceDispositionV1,
    source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
    normal_root_execution: ParserNormalRootExecutionSourceDispositionV1,
    reject: super::normal_root_execution::ParserNormalRootExecutionCompatibilityRejectV1,
) {
    consume_parameter_source_at_named_terminal(parameter_source);
    consume_program_source_authority_at_named_terminal(source_authority);
    consume_root_disposition_at_named_terminal(normal_root_execution);
    drop(completed.into_ast());
    let _consumed_reject = reject;
}

#[cfg(test)]
fn consume_completed_postpass_at_test_terminal(completed: CompletedParserPostpassV1) {
    drop(completed);
}

#[cfg(test)]
pub(super) fn consume_retained_fields_at_named_test_terminal(
    completed: CompletedParserPostpassV1,
    parameter_source: ParserCallableParameterSourceCatalogV1,
    source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
    canonical_script_source_rows: CanonicalScriptSourceRowsDispositionV1,
    normal_root_execution: ParserNormalRootExecutionSourceDispositionV1,
) {
    consume_parameter_source_at_named_terminal(
        ParserCallableParameterSourceDispositionV1::Complete(parameter_source),
    );
    consume_script_rows_for_normal_callable(canonical_script_source_rows);
    consume_program_source_authority_at_named_terminal(source_authority);
    consume_root_disposition_at_named_terminal(normal_root_execution);
    consume_completed_postpass_at_test_terminal(completed);
}

fn consume_ready_root_for_raw(
    root: super::normal_root_execution::ParserNormalRootExecutionSourceV1,
) {
    drop(root);
}

fn normal_callable_source_reject(
    error: super::super::normal_callable_program_source::NormalCallableParameterSourceRejectV1,
) -> ParseError {
    ParseError::GrammarContract {
        stable_reject_tag: "parser/normal-callable-parameter-source",
        detail: format!("normal callable parameter source rejected: {error:?}"),
        line: 0,
    }
}
