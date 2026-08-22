//! Pre-effect source-package issuer for the selected Script lookup relation.
//!
//! The issuer keeps the parser loan, neutral source window, declaration
//! catalog, import relation, and generic result catalog in one scope.  Only
//! owned AST-free rows leave that scope.  The existing static publication
//! owner is also issued here for the callable package so the generic catalogs
//! are not re-observed after Builder effects.

use super::normal_script_neutral_window::PreparedCanonicalScriptNeutralProgramWindowV1;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationOwnerV1;
use crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
use crate::mir::source_call_target::{
    ScriptDirectStaticCallCoverageIssueV1, ScriptDirectStaticCallLookupErrorV1,
    VerifiedScriptDirectStaticCallLookupV1, VerifiedStaticImportAliasViewV1,
    VerifiedWholeSourceStaticCallTargetInventoryV1,
};

#[derive(Debug)]
pub(super) enum NormalScriptDirectStaticLookupIssueV1 {
    Import(Box<str>),
    Targets(Box<str>),
    Results(Box<str>),
    PublicationOwner(Box<str>),
    SourceLoan(Box<str>),
    Lookup(ScriptDirectStaticCallLookupErrorV1),
}

/// The only production lookup issuer.  The returned relation and existing
/// publication owner are both owned, so all temporary catalog borrows end
/// before the semantic package is moved into Builder.
pub(super) struct ScriptDirectStaticCallLookupIssuerV1;

impl ScriptDirectStaticCallLookupIssuerV1 {
    pub(super) fn issue(
        package: &VerifiedNormalCallableSemanticPackageV1,
        neutral_window: Option<&PreparedCanonicalScriptNeutralProgramWindowV1>,
        import_rows: &[(String, String)],
    ) -> Result<
        (
            Option<VerifiedScriptDirectStaticCallLookupV1>,
            VerifiedStaticCallResultPublicationOwnerV1,
        ),
        NormalScriptDirectStaticLookupIssueV1,
    > {
        let declarations = package.declaration_catalog();
        let imports =
            VerifiedStaticImportAliasViewV1::seal(declarations, import_rows.iter().cloned())
                .map_err(|error| {
                    NormalScriptDirectStaticLookupIssueV1::Import(format!("{error:?}").into())
                })?;
        let inventory =
            VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, &imports)
                .map_err(|error| {
                    NormalScriptDirectStaticLookupIssueV1::Targets(format!("{error:?}").into())
                })?;
        let targets = inventory.into_targets();
        let results = crate::mir::callable_result_representation::VerifiedSameModuleCallableResultCatalogV1::verify(
            declarations,
            &targets,
        )
        .map_err(|error| NormalScriptDirectStaticLookupIssueV1::Results(format!("{error:?}").into()))?;
        let publication_owner =
            VerifiedStaticCallResultPublicationOwnerV1::issue(declarations, &targets, &results)
                .map_err(|error| {
                    NormalScriptDirectStaticLookupIssueV1::PublicationOwner(
                        format!("{error:?}").into(),
                    )
                })?;

        let Some(neutral_window) = neutral_window else {
            return Ok((None, publication_owner));
        };
        let lookup = package
            .with_normal_program_source_loan(|loan| {
                if !neutral_window.is_from_invocation(loan.invocation_witness()) {
                    return Err(NormalScriptDirectStaticLookupIssueV1::Lookup(
                        ScriptDirectStaticCallLookupErrorV1::Coverage(
                            ScriptDirectStaticCallCoverageIssueV1::ForeignInvocation,
                        ),
                    ));
                }
                VerifiedScriptDirectStaticCallLookupV1::issue_from_program_loan(
                    &loan,
                    neutral_window.window(),
                    declarations,
                    &imports,
                    &targets,
                    &results,
                )
                .map_err(NormalScriptDirectStaticLookupIssueV1::Lookup)
            })
            .map_err(|error| {
                NormalScriptDirectStaticLookupIssueV1::SourceLoan(format!("{error:?}").into())
            })??;

        Ok((Some(lookup), publication_owner))
    }
}

#[cfg(test)]
#[path = "normal_script_direct_static_lookup_tests.rs"]
mod tests;
