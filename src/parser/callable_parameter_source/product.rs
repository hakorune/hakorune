use super::canonical_script_source_admission::{
    issue_canonical_script_cohort, CanonicalScriptCohortDispositionV1,
};
use super::catalog::{
    ParserCallableParameterSourceCatalogV1, ParserCallableParameterSourceDispositionV1,
};
use super::composite_source::issue_parser_composite_source_v1;
use super::retained::RetainedParserCallableSemanticSourceV1;
use super::script_source_rows::{
    issue_canonical_script_source_rows, CanonicalScriptSourceRowsDispositionV1,
};
use super::script_source_authority::{
    issue_parser_normal_program_source_authority_v1,
    ParserNormalProgramSourceAuthorityDispositionV1,
};
use super::syntax_loan::{
    borrow_callable_declaration_syntax_v1, ParserCallableDeclarationSyntaxLoanV1,
    ParserCallableSyntaxLoanErrorV1,
};
use crate::parser::postpass_envelope::CompletedParserPostpassV1;
use crate::parser::{NyashParser, ParseError, ParsedNormalCallableProgramV1, ParserBuildConfig};

/// Total source-family disposition emitted by the one parser invocation.
/// Compatibility is explicit and never represented as an empty callable
/// catalog. Source-backed consumers receive the atomic product unchanged.
#[derive(Debug)]
pub(crate) enum ParserCallableSourceDispositionV1 {
    SourceBacked(ParsedProgramWithCallableParameterSourceV1),
    Compatibility(CompletedParserPostpassV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserCallableSourceRetentionErrorV1 {
    ParameterSourceUnavailable,
    CompositeSourceReadyCannotBeDiscarded,
}

/// One-shot total parser result plus its sibling callable parameter source
/// catalog. Neither side can be paired with a product from another invocation.
#[derive(Debug)]
pub(crate) struct ParsedProgramWithCallableParameterSourceV1 {
    completed: CompletedParserPostpassV1,
    parameter_source: ParserCallableParameterSourceDispositionV1,
    canonical_script_admission: CanonicalScriptCohortDispositionV1,
    canonical_script_source_rows: CanonicalScriptSourceRowsDispositionV1,
    source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
}

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
    pub(in crate::parser) fn new(
        completed: CompletedParserPostpassV1,
        parameter_source: ParserCallableParameterSourceDispositionV1,
    ) -> Self {
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
            canonical_script_admission,
            canonical_script_source_rows,
            source_authority,
        }
    }

    pub(crate) fn canonical_script_admission(&self) -> &CanonicalScriptCohortDispositionV1 {
        &self.canonical_script_admission
    }

    /// Move the atomic parser result into the retained source owner used by
    /// the future sole callable semantic batch.
    ///
    /// This transition exposes neither the AST nor the parameter catalog as
    /// independently movable parts.
    pub(crate) fn into_retained_source(
        self,
    ) -> Result<RetainedParserCallableSemanticSourceV1, ParserCallableSourceRetentionErrorV1> {
        let Self {
            completed,
            parameter_source,
            source_authority,
            ..
        } = self;
        let ParserCallableParameterSourceDispositionV1::Complete(catalog) = parameter_source
        else {
            return Err(ParserCallableSourceRetentionErrorV1::ParameterSourceUnavailable);
        };
        if source_authority.composite_source_is_ready() {
            return Err(
                ParserCallableSourceRetentionErrorV1::CompositeSourceReadyCannotBeDiscarded,
            );
        }
        Ok(RetainedParserCallableSemanticSourceV1::new(
            completed,
            catalog,
        ))
    }

    /// Keep compatibility explicit while preserving the atomic product for
    /// source-backed consumers. The catalog is dropped only on the explicit
    /// compatibility branch; it is never projected as an empty source fact.
    pub(crate) fn into_source_disposition(self) -> ParserCallableSourceDispositionV1 {
        self.into_source_disposition_with_script_rows().0
    }

    /// Move the callable disposition and the same-invocation Script rows
    /// together.  The front door uses this pair to prevent a second parser
    /// scan or cross-invocation re-pairing.
    pub(crate) fn into_source_disposition_with_script_rows(
        self,
    ) -> (
        ParserCallableSourceDispositionV1,
        CanonicalScriptSourceRowsDispositionV1,
    ) {
        let Self {
            completed,
            parameter_source,
            canonical_script_admission,
            canonical_script_source_rows,
            source_authority,
        } = self;
        // Preserve the pre-existing source-backed disposition for the
        // already-sealed callable cohort.  The old boolean is deliberately
        // not consulted by the admission issuer above; it is only a
        // compatibility-preserving projection here.
        let is_canonical_source = completed.is_source_backed()
            || matches!(
                &canonical_script_admission,
                CanonicalScriptCohortDispositionV1::CanonicalScriptCohortAdmitted(_)
            );
        if is_canonical_source {
            (
                ParserCallableSourceDispositionV1::SourceBacked(
                    ParsedProgramWithCallableParameterSourceV1 {
                        completed,
                        parameter_source,
                        canonical_script_admission,
                        canonical_script_source_rows:
                            CanonicalScriptSourceRowsDispositionV1::MovedToParallelHandoff,
                        source_authority,
                    },
                ),
                canonical_script_source_rows,
            )
        } else {
            (
                ParserCallableSourceDispositionV1::Compatibility(completed),
                canonical_script_source_rows,
            )
        }
    }

    pub(crate) fn into_normal_callable_program(
        self,
    ) -> Result<ParsedNormalCallableProgramV1, ParseError> {
        let disposition = self.into_source_disposition();
        disposition.into_normal_callable_program()
    }

    /// Borrow exact callable declarations while consuming the parser product.
    ///
    /// The loan cannot escape the callback. The owned catalog moves into the
    /// same callback, so another AST or parser invocation cannot be paired
    /// with its source rows after this boundary.
    pub(crate) fn with_callable_declaration_syntax<R>(
        self,
        callback: impl for<'ast> FnOnce(
            ParserCallableParameterSourceCatalogV1,
            ParserCallableDeclarationSyntaxLoanV1<'ast>,
        ) -> R,
    ) -> Result<R, ParserCallableSyntaxLoanErrorV1> {
        let Self {
            completed,
            parameter_source,
            canonical_script_admission: _,
            canonical_script_source_rows: _,
            source_authority,
        } = self;
        if source_authority.composite_source_is_ready() {
            return Err(ParserCallableSyntaxLoanErrorV1::CompositeSourceReadyCannotBeDiscarded);
        }
        let ParserCallableParameterSourceDispositionV1::Complete(catalog) = parameter_source else {
            return Err(ParserCallableSyntaxLoanErrorV1::ParameterSourceUnavailable);
        };
        let loan = borrow_callable_declaration_syntax_v1(completed.ast(), &catalog)?;
        Ok(callback(catalog, loan))
    }
}

impl ParserCallableSourceDispositionV1 {
    pub(crate) fn ast(&self) -> &crate::ast::ASTNode {
        match self {
            Self::SourceBacked(product) => product.completed.ast(),
            Self::Compatibility(postpass) => postpass.ast(),
        }
    }

    pub(crate) fn is_source_backed(&self) -> bool {
        matches!(self, Self::SourceBacked(_))
    }

    pub(crate) fn parser_postpass(&self) -> &CompletedParserPostpassV1 {
        match self {
            Self::SourceBacked(product) => &product.completed,
            Self::Compatibility(postpass) => postpass,
        }
    }

    pub(crate) fn into_ast(self) -> crate::ast::ASTNode {
        match self {
            Self::SourceBacked(product) => {
                assert!(
                    !product.source_authority.composite_source_is_ready(),
                    "ready composite source must not be discarded before its named consumer"
                );
                product.completed.into_ast()
            }
            Self::Compatibility(postpass) => postpass.into_ast(),
        }
    }

    pub(crate) fn into_normal_callable_program(
        self,
    ) -> Result<ParsedNormalCallableProgramV1, ParseError> {
        let parsed = match self {
            Self::SourceBacked(product) => {
                let ParsedProgramWithCallableParameterSourceV1 {
                    completed,
                    parameter_source,
                    canonical_script_admission: _,
                    canonical_script_source_rows: _,
                    source_authority,
                } = product;
                completed.into_normal_callable_program(parameter_source, source_authority)
            }
            Self::Compatibility(postpass) => postpass.into_normal_callable_program(
                ParserCallableParameterSourceDispositionV1::SelectedBuildGateUnsupported,
                ParserNormalProgramSourceAuthorityDispositionV1::SourceAuthorityUnavailable(
                    super::script_source_authority::ParserNormalProgramSourceAuthorityUnavailableV1::PostpassNotSourceBacked,
                ),
            ),
        };
        parsed.map_err(|error| ParseError::GrammarContract {
            stable_reject_tag: "parser/normal-callable-parameter-source",
            detail: format!("normal callable parameter source rejected: {error:?}"),
            line: 0,
        })
    }
}
