use super::model::ParserCallableParameterDeclarationSourceV1;
use crate::parser::source_authority::ParserInvocationBrandV1;

/// Complete parser-issued callable parameter source truth for one invocation.
///
/// This is a sibling of `ParserBoxSourceSealV1`, not an extension of it.
/// Static and ordinary instance methods therefore share source coordinates
/// without sharing Box postpass/delegate policy.
#[derive(Debug)]
pub(crate) struct ParserCallableParameterSourceCatalogV1 {
    brand: ParserInvocationBrandV1,
    declarations: Box<[ParserCallableParameterDeclarationSourceV1]>,
}

impl ParserCallableParameterSourceCatalogV1 {
    pub(super) fn new(
        brand: ParserInvocationBrandV1,
        declarations: Box<[ParserCallableParameterDeclarationSourceV1]>,
    ) -> Self {
        Self {
            brand,
            declarations,
        }
    }

    pub(crate) fn declarations(&self) -> &[ParserCallableParameterDeclarationSourceV1] {
        &self.declarations
    }

    pub(crate) fn same_parser_source(&self, other: &Self) -> bool {
        self.brand.same_as(&other.brand)
    }
}
