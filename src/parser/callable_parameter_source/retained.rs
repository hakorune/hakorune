//! Retained parser source for one future resolved callable semantic batch.
//!
//! The completed postpass and complete parameter catalog stay atomic.  This
//! owner lends exact declaration syntax only within a callback and carries no
//! resolver, Home, Recipe, or physical meaning.

use super::catalog::ParserCallableParameterSourceCatalogV1;
use super::syntax_loan::{
    borrow_callable_declaration_syntax_v1, ParserCallableDeclarationSyntaxLoanV1,
    ParserCallableSyntaxLoanErrorV1,
};
use crate::parser::postpass_envelope::CompletedParserPostpassV1;

#[derive(Debug)]
pub(crate) struct RetainedParserCallableSemanticSourceV1 {
    completed: CompletedParserPostpassV1,
    parameter_source: ParserCallableParameterSourceCatalogV1,
}

impl RetainedParserCallableSemanticSourceV1 {
    pub(super) const fn new(
        completed: CompletedParserPostpassV1,
        parameter_source: ParserCallableParameterSourceCatalogV1,
    ) -> Self {
        Self {
            completed,
            parameter_source,
        }
    }

    /// Lend the exact parser source repeatedly without splitting its owners.
    ///
    /// The higher-ranked callback prevents AST-backed declaration references
    /// from escaping.  The future semantic-batch issuer may resolve once and
    /// retain only its owned forests/projections beside this source owner.
    pub(crate) fn with_callable_declaration_syntax<R>(
        &self,
        callback: impl for<'source> FnOnce(
            &'source ParserCallableParameterSourceCatalogV1,
            ParserCallableDeclarationSyntaxLoanV1<'source>,
        ) -> R,
    ) -> Result<R, ParserCallableSyntaxLoanErrorV1> {
        let loan =
            borrow_callable_declaration_syntax_v1(self.completed.ast(), &self.parameter_source)?;
        Ok(callback(&self.parameter_source, loan))
    }
}
