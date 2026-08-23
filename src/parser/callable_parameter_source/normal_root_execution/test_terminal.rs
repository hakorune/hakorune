//! Named parser-product terminal used only by parser-owned tests.
//!
//! Tests may inspect one scoped parser-fact or syntax loan, but cannot silently
//! destructure or drop the total root relation and Script-A sibling.

use super::super::product::ParsedProgramWithCallableParameterSourceV1;
use super::super::syntax_loan::{
    ParserCallableDeclarationSyntaxLoanV1, ParserCallableSyntaxLoanErrorV1,
};
use super::{ParserNormalRootExecutionRoleV1, ParserNormalRootExecutionSourceDispositionV1};

/// Callback-scoped observation of one intact parser product.
///
/// This loan exposes parser facts only to tests. The HRTB callback cannot
/// return any borrowed field, and the product consumes every sibling at the
/// named terminal immediately after the callback returns.
pub(in crate::parser) struct ParserNormalRootExecutionTestLoanV1<'source> {
    parameter_source: &'source super::super::catalog::ParserCallableParameterSourceDispositionV1,
    static_box_parent_source:
        &'source super::super::static_box_source::ParserStaticBoxParentSourceDispositionV1,
    script_rows: &'source super::super::script_source_rows::CanonicalScriptSourceRowsDispositionV1,
    root_execution: &'source ParserNormalRootExecutionSourceDispositionV1,
    module_rows: Option<
        &'source super::super::script_source_authority::ParserNormalModuleSourceRowsDispositionV1,
    >,
}

/// The sole scoped observation of one retained parser owner.
///
/// Every reference and the AST-backed syntax loan end with the callback. The
/// retained owner then consumes all five fields through one common epilogue.
pub(in crate::parser) struct ParserRetainedCallableSemanticSourceTestLoanV1<'source> {
    parameter_source: &'source super::super::catalog::ParserCallableParameterSourceCatalogV1,
    source_authority:
        &'source super::super::script_source_authority::ParserNormalProgramSourceAuthorityDispositionV1,
    script_rows:
        &'source super::super::script_source_rows::CanonicalScriptSourceRowsDispositionV1,
    root_execution: &'source ParserNormalRootExecutionSourceDispositionV1,
    syntax: ParserCallableDeclarationSyntaxLoanV1<'source>,
}

pub(in crate::parser) struct ParserNormalRootExecutionTestTerminalV1;

impl ParserNormalRootExecutionTestTerminalV1 {
    pub(in crate::parser) fn observe_once<R>(
        source: ParsedProgramWithCallableParameterSourceV1,
        callback: impl for<'source> FnOnce(ParserNormalRootExecutionTestLoanV1<'source>) -> R,
    ) -> R {
        source.observe_test_terminal_once(callback)
    }

    pub(in crate::parser) fn consume_once<R>(
        source: ParsedProgramWithCallableParameterSourceV1,
        callback: impl for<'syntax> FnOnce(
            &'syntax super::super::catalog::ParserCallableParameterSourceCatalogV1,
            ParserCallableDeclarationSyntaxLoanV1<'syntax>,
        ) -> R,
    ) -> Result<R, ParserCallableSyntaxLoanErrorV1> {
        source.consume_test_terminal_once(callback)
    }

    pub(in crate::parser) fn consume_retained_once<R>(
        source: super::super::retained::RetainedParserCallableSemanticSourceV1,
        callback: impl for<'source> FnOnce(ParserRetainedCallableSemanticSourceTestLoanV1<'source>) -> R,
    ) -> Result<R, ParserCallableSyntaxLoanErrorV1> {
        source.consume_retained_test_terminal_once(callback)
    }
}

impl<'source> ParserNormalRootExecutionTestLoanV1<'source> {
    pub(in crate::parser) fn issue(
        parameter_source: &'source super::super::catalog::ParserCallableParameterSourceDispositionV1,
        static_box_parent_source: &'source super::super::static_box_source::ParserStaticBoxParentSourceDispositionV1,
        script_rows: &'source super::super::script_source_rows::CanonicalScriptSourceRowsDispositionV1,
        root_execution: &'source ParserNormalRootExecutionSourceDispositionV1,
        module_rows: Option<&'source super::super::script_source_authority::ParserNormalModuleSourceRowsDispositionV1>,
    ) -> Self {
        Self {
            parameter_source,
            static_box_parent_source,
            script_rows,
            root_execution,
            module_rows,
        }
    }

    pub(in crate::parser) fn callable_parameter_source(
        &self,
    ) -> &super::super::catalog::ParserCallableParameterSourceDispositionV1 {
        self.parameter_source
    }

    pub(in crate::parser) fn static_box_parent_source(
        &self,
    ) -> &super::super::static_box_source::ParserStaticBoxParentSourceDispositionV1 {
        self.static_box_parent_source
    }

    pub(in crate::parser) fn canonical_script_source_rows(
        &self,
    ) -> &super::super::script_source_rows::CanonicalScriptSourceRowsDispositionV1 {
        self.script_rows
    }

    pub(in crate::parser) fn normal_root_execution(
        &self,
    ) -> &ParserNormalRootExecutionSourceDispositionV1 {
        self.root_execution
    }

    pub(in crate::parser) fn normal_root_execution_role(
        &self,
    ) -> Option<ParserNormalRootExecutionRoleV1> {
        self.root_execution.ready().map(|source| source.role())
    }

    pub(in crate::parser) fn normal_module_source_rows(
        &self,
    ) -> Option<&super::super::script_source_authority::ParserNormalModuleSourceRowsDispositionV1>
    {
        self.module_rows
    }
}

impl<'source> ParserRetainedCallableSemanticSourceTestLoanV1<'source> {
    pub(in crate::parser) fn issue(
        parameter_source: &'source super::super::catalog::ParserCallableParameterSourceCatalogV1,
        source_authority: &'source super::super::script_source_authority::ParserNormalProgramSourceAuthorityDispositionV1,
        script_rows: &'source super::super::script_source_rows::CanonicalScriptSourceRowsDispositionV1,
        root_execution: &'source ParserNormalRootExecutionSourceDispositionV1,
        syntax: ParserCallableDeclarationSyntaxLoanV1<'source>,
    ) -> Self {
        Self {
            parameter_source,
            source_authority,
            script_rows,
            root_execution,
            syntax,
        }
    }

    pub(in crate::parser) fn callable_parameter_source(
        &self,
    ) -> &super::super::catalog::ParserCallableParameterSourceCatalogV1 {
        self.parameter_source
    }

    pub(in crate::parser) fn callable_declaration_syntax(
        &self,
    ) -> &ParserCallableDeclarationSyntaxLoanV1<'source> {
        &self.syntax
    }

    pub(in crate::parser) fn normal_root_execution(
        &self,
    ) -> &ParserNormalRootExecutionSourceDispositionV1 {
        self.root_execution
    }

    pub(in crate::parser) fn retains_script_source_rows(&self) -> bool {
        !matches!(
            self.script_rows,
            super::super::script_source_rows::CanonicalScriptSourceRowsDispositionV1::MovedToParallelHandoff
        )
    }

    pub(in crate::parser) fn source_authority_is_ready(&self) -> bool {
        matches!(
            self.source_authority,
            super::super::script_source_authority::ParserNormalProgramSourceAuthorityDispositionV1::Ready(_)
        )
    }

    pub(in crate::parser) fn parser_invocation_witness(
        &self,
    ) -> Option<&super::super::parser_invocation_witness::ParserInvocationWitnessV1> {
        self.root_execution
            .ready()
            .map(|root| root.bound().invocation())
    }
}
