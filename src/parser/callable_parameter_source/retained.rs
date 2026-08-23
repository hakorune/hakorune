//! Retained parser source for one future resolved callable semantic batch.
//!
//! The completed postpass and complete parameter catalog stay atomic. This
//! owner reaches one consuming test terminal without issuing resolver, Home,
//! Recipe, or physical meaning.

use super::catalog::ParserCallableParameterSourceCatalogV1;
use super::normal_root_execution::ParserNormalRootExecutionSourceDispositionV1;
use super::script_source_authority::ParserNormalProgramSourceAuthorityDispositionV1;
use super::script_source_rows::CanonicalScriptSourceRowsDispositionV1;
#[cfg(test)]
use super::syntax_loan::{borrow_callable_declaration_syntax_v1, ParserCallableSyntaxLoanErrorV1};
use crate::parser::postpass_envelope::CompletedParserPostpassV1;

#[derive(Debug)]
pub(crate) struct RetainedParserCallableSemanticSourceV1 {
    completed: CompletedParserPostpassV1,
    parameter_source: ParserCallableParameterSourceCatalogV1,
    source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
    canonical_script_source_rows: CanonicalScriptSourceRowsDispositionV1,
    normal_root_execution: ParserNormalRootExecutionSourceDispositionV1,
}

impl RetainedParserCallableSemanticSourceV1 {
    pub(super) const fn new(
        completed: CompletedParserPostpassV1,
        parameter_source: ParserCallableParameterSourceCatalogV1,
        source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
        canonical_script_source_rows: CanonicalScriptSourceRowsDispositionV1,
        normal_root_execution: ParserNormalRootExecutionSourceDispositionV1,
    ) -> Self {
        Self {
            completed,
            parameter_source,
            source_authority,
            canonical_script_source_rows,
            normal_root_execution,
        }
    }

    #[cfg(test)]
    pub(super) fn consume_retained_test_terminal_once<R>(
        self,
        callback: impl for<'source> FnOnce(
            super::normal_root_execution::ParserRetainedCallableSemanticSourceTestLoanV1<'source>,
        ) -> R,
    ) -> Result<R, ParserCallableSyntaxLoanErrorV1> {
        let Self {
            completed,
            parameter_source,
            source_authority,
            canonical_script_source_rows,
            normal_root_execution,
        } = self;
        let observation = match borrow_callable_declaration_syntax_v1(
            completed.ast(),
            &parameter_source,
        ) {
            Ok(syntax) => Ok(callback(
                super::normal_root_execution::ParserRetainedCallableSemanticSourceTestLoanV1::issue(
                    &parameter_source,
                    &source_authority,
                    &canonical_script_source_rows,
                    &normal_root_execution,
                    syntax,
                ),
            )),
            Err(error) => Err(error),
        };
        super::product::consume_retained_fields_at_named_test_terminal(
            completed,
            parameter_source,
            source_authority,
            canonical_script_source_rows,
            normal_root_execution,
        );
        observation
    }
}
