use super::catalog::ParserCallableParameterSourceCatalogV1;
use super::retained::RetainedParserCallableSemanticSourceV1;
use super::syntax_loan::{
    borrow_callable_declaration_syntax_v1, ParserCallableDeclarationSyntaxLoanV1,
    ParserCallableSyntaxLoanErrorV1,
};
use crate::parser::postpass_envelope::CompletedParserPostpassV1;
use crate::parser::{NyashParser, ParseError, ParserBuildConfig};

/// One-shot total parser result plus its sibling callable parameter source
/// catalog. Neither side can be paired with a product from another invocation.
#[derive(Debug)]
pub(crate) struct ParsedProgramWithCallableParameterSourceV1 {
    completed: CompletedParserPostpassV1,
    catalog: ParserCallableParameterSourceCatalogV1,
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
        catalog: ParserCallableParameterSourceCatalogV1,
    ) -> Self {
        Self { completed, catalog }
    }

    /// Move the atomic parser result into the retained source owner used by
    /// the future sole callable semantic batch.
    ///
    /// This transition exposes neither the AST nor the parameter catalog as
    /// independently movable parts.
    pub(crate) fn into_retained_source(self) -> RetainedParserCallableSemanticSourceV1 {
        RetainedParserCallableSemanticSourceV1::new(self.completed, self.catalog)
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
        let Self { completed, catalog } = self;
        let loan = borrow_callable_declaration_syntax_v1(completed.ast(), &catalog)?;
        Ok(callback(catalog, loan))
    }
}
