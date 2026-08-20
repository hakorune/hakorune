use super::catalog::{
    ParserCallableParameterSourceCatalogV1, ParserCallableParameterSourceDispositionV1,
};
use super::retained::RetainedParserCallableSemanticSourceV1;
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
}

/// One-shot total parser result plus its sibling callable parameter source
/// catalog. Neither side can be paired with a product from another invocation.
#[derive(Debug)]
pub(crate) struct ParsedProgramWithCallableParameterSourceV1 {
    completed: CompletedParserPostpassV1,
    parameter_source: ParserCallableParameterSourceDispositionV1,
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
        Self {
            completed,
            parameter_source,
        }
    }

    /// Move the atomic parser result into the retained source owner used by
    /// the future sole callable semantic batch.
    ///
    /// This transition exposes neither the AST nor the parameter catalog as
    /// independently movable parts.
    pub(crate) fn into_retained_source(
        self,
    ) -> Result<RetainedParserCallableSemanticSourceV1, ParserCallableSourceRetentionErrorV1> {
        let ParserCallableParameterSourceDispositionV1::Complete(catalog) = self.parameter_source
        else {
            return Err(ParserCallableSourceRetentionErrorV1::ParameterSourceUnavailable);
        };
        Ok(RetainedParserCallableSemanticSourceV1::new(
            self.completed,
            catalog,
        ))
    }

    /// Keep compatibility explicit while preserving the atomic product for
    /// source-backed consumers. The catalog is dropped only on the explicit
    /// compatibility branch; it is never projected as an empty source fact.
    pub(crate) fn into_source_disposition(self) -> ParserCallableSourceDispositionV1 {
        if self.completed.is_source_backed() {
            ParserCallableSourceDispositionV1::SourceBacked(self)
        } else {
            ParserCallableSourceDispositionV1::Compatibility(self.completed)
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
        } = self;
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
            Self::SourceBacked(product) => product.completed.into_ast(),
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
                } = product;
                completed.into_normal_callable_program(parameter_source)
            }
            Self::Compatibility(postpass) => postpass.into_normal_callable_program(
                ParserCallableParameterSourceDispositionV1::SelectedBuildGateUnsupported,
            ),
        };
        parsed.map_err(|error| ParseError::GrammarContract {
            stable_reject_tag: "parser/normal-callable-parameter-source",
            detail: format!("normal callable parameter source rejected: {error:?}"),
            line: 0,
        })
    }
}
