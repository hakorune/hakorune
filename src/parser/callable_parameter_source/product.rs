use crate::ast::ASTNode;

use super::catalog::ParserCallableParameterSourceCatalogV1;
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

    pub(crate) fn into_ast_and_catalog(self) -> (ASTNode, ParserCallableParameterSourceCatalogV1) {
        (self.completed.into_ast(), self.catalog)
    }
}
