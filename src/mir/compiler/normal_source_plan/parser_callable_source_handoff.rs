//! Builder-free transport for one parser-issued callable source product.
//!
//! This handoff carries parser meaning and front-door identity only. It does
//! not issue declaration Facts, resolver products, Recipe/Join, or physical
//! state. Compatibility remains an explicit disposition; it is never an empty
//! source-backed candidate.

use crate::parser::{
    NormalParserSourceLineageV1, ParsedNormalCallableProgramV1, ParserCallableSourceDispositionV1,
};

#[derive(Debug)]
pub(crate) struct NormalParserCallableSourceHandoffV1 {
    disposition: ParserCallableSourceDispositionV1,
    lineage: NormalParserSourceLineageV1,
    _seal: NormalParserCallableSourceHandoffSealV1,
}

#[derive(Debug)]
struct NormalParserCallableSourceHandoffSealV1;

impl NormalParserCallableSourceHandoffV1 {
    pub(crate) fn new(
        disposition: ParserCallableSourceDispositionV1,
        lineage: NormalParserSourceLineageV1,
    ) -> Self {
        Self {
            disposition,
            lineage,
            _seal: NormalParserCallableSourceHandoffSealV1,
        }
    }

    pub(crate) fn ast(&self) -> &crate::ast::ASTNode {
        self.disposition.ast()
    }

    pub(crate) fn is_source_backed(&self) -> bool {
        self.disposition.is_source_backed()
    }

    pub(crate) fn parser_postpass(
        &self,
    ) -> &crate::parser::postpass_envelope::CompletedParserPostpassV1 {
        self.disposition.parser_postpass()
    }

    pub(crate) fn lineage(&self) -> &NormalParserSourceLineageV1 {
        &self.lineage
    }

    pub(crate) fn into_normal_callable_program(
        self,
    ) -> Result<
        (ParsedNormalCallableProgramV1, NormalParserSourceLineageV1),
        crate::parser::ParseError,
    > {
        let Self {
            disposition,
            lineage,
            _seal: _,
        } = self;
        Ok((disposition.into_normal_callable_program()?, lineage))
    }

    pub(crate) fn into_ast(self) -> crate::ast::ASTNode {
        self.disposition.into_ast()
    }
}
