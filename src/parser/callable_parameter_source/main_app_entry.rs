//! Parser-only admission for the first static `Main.main/0` cohort.
//!
//! This module consumes already-issued parser products.  It does not inspect
//! the AST, select a Builder route, or write root state.  The callable anchor
//! remains owned by the direct callable source session; this product carries
//! only its comparison identity and exact source coverage.

use super::catalog::{
    ParserCallableParameterSourceCatalogV1, ParserCallableParameterSourceDispositionV1,
};
use super::model::ParserCallableDeclarationKindV1;
use super::static_box_source::{
    ParserStaticBoxMemberKindV1, ParserStaticBoxParentOutsideReasonV1,
    ParserStaticBoxParentSourceDispositionV1, ParserStaticBoxParentSourceIncompleteV1,
    ParserStaticBoxParentSourceIntegrityIssueV1, ParserStaticBoxParentSourceUnavailableV1,
    ParserStaticBoxSourceSealV1,
};
use crate::parser::callable_source_anchor::CallableDeclarationIdentityV1;
use crate::parser::postpass_envelope::{
    CompletedParserPostpassV1, ParserPostpassProgramCohortV1,
};
use crate::parser::source_authority::{SourceBoxDeclarationSiteV1, SourceBoxMethodSiteV1};

#[derive(Debug)]
pub(in crate::parser) struct ParserMainAppEntrySealV1 {
    box_site: SourceBoxDeclarationSiteV1,
    method_site: SourceBoxMethodSiteV1,
    callable_identity: CallableDeclarationIdentityV1,
}

impl ParserMainAppEntrySealV1 {
    pub(in crate::parser) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }

    pub(in crate::parser) fn method_site(&self) -> &SourceBoxMethodSiteV1 {
        &self.method_site
    }

    pub(in crate::parser) fn callable_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.callable_identity
    }
}

#[derive(Debug)]
pub(in crate::parser) enum ParserMainAppEntryDispositionV1 {
    AppMainReady(ParserMainAppEntrySealV1),
    Outside(ParserMainAppEntryOutsideReasonV1),
    SourceAuthorityUnavailable(ParserMainAppEntryUnavailableV1),
    Incomplete(ParserMainAppEntryIncompleteV1),
    IntegrityInvalid(ParserMainAppEntryIntegrityIssueV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserMainAppEntryOutsideReasonV1 {
    ProgramCohort,
    StaticParent(ParserStaticBoxParentOutsideReasonV1),
    NonMainStaticBox,
    NonMainMethod,
    NonZeroMainArity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserMainAppEntryUnavailableV1 {
    StaticParent(ParserStaticBoxParentSourceUnavailableV1),
    SelectedBuildGateUnsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserMainAppEntryIncompleteV1 {
    StaticParent(ParserStaticBoxParentSourceIncompleteV1),
    CallableCatalogRowMissing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserMainAppEntryIntegrityIssueV1 {
    ForeignParameterCatalog,
    StaticParent(ParserStaticBoxParentSourceIntegrityIssueV1),
    DuplicateCallableCatalogRow,
    CallableRelationMismatch,
    CallableKindMismatch,
    CallableSiteMismatch,
    StaticParentMemberCoverageMismatch,
}

pub(in crate::parser) fn issue_parser_main_app_entry_v1(
    completed: &CompletedParserPostpassV1,
    parameter_source: &ParserCallableParameterSourceDispositionV1,
) -> ParserMainAppEntryDispositionV1 {
    if completed.program_cohort_for_admission() != ParserPostpassProgramCohortV1::StaticBox {
        return ParserMainAppEntryDispositionV1::Outside(
            ParserMainAppEntryOutsideReasonV1::ProgramCohort,
        );
    }

    let parent = match completed.static_box_parent_source() {
        ParserStaticBoxParentSourceDispositionV1::Ready(seal) => seal,
        ParserStaticBoxParentSourceDispositionV1::Outside(reason) => {
            return ParserMainAppEntryDispositionV1::Outside(
                ParserMainAppEntryOutsideReasonV1::StaticParent(*reason),
            )
        }
        ParserStaticBoxParentSourceDispositionV1::SourceAuthorityUnavailable(reason) => {
            return ParserMainAppEntryDispositionV1::SourceAuthorityUnavailable(
                ParserMainAppEntryUnavailableV1::StaticParent(*reason),
            )
        }
        ParserStaticBoxParentSourceDispositionV1::Incomplete(reason) => {
            return ParserMainAppEntryDispositionV1::Incomplete(
                ParserMainAppEntryIncompleteV1::StaticParent(*reason),
            )
        }
        ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid(reason) => {
            return ParserMainAppEntryDispositionV1::IntegrityInvalid(
                ParserMainAppEntryIntegrityIssueV1::StaticParent(*reason),
            )
        }
    };

    if parent.declaration_syntax().name() != "Main" || parent.declaration_syntax().is_sync() {
        return ParserMainAppEntryDispositionV1::Outside(
            ParserMainAppEntryOutsideReasonV1::NonMainStaticBox,
        );
    }
    let mut member_kinds = parent.member_kinds();
    if parent.member_count() != 1
        || member_kinds.next() != Some(ParserStaticBoxMemberKindV1::DirectMethod)
        || member_kinds.next().is_some()
    {
        return ParserMainAppEntryDispositionV1::IntegrityInvalid(
            ParserMainAppEntryIntegrityIssueV1::StaticParentMemberCoverageMismatch,
        );
    }

    let ParserCallableParameterSourceDispositionV1::Complete(catalog) = parameter_source else {
        return ParserMainAppEntryDispositionV1::SourceAuthorityUnavailable(
            ParserMainAppEntryUnavailableV1::SelectedBuildGateUnsupported,
        );
    };
    issue_from_complete_catalog(parent, catalog)
}

fn issue_from_complete_catalog(
    parent: &ParserStaticBoxSourceSealV1,
    catalog: &ParserCallableParameterSourceCatalogV1,
) -> ParserMainAppEntryDispositionV1 {
    if !catalog.same_parser_brand(parent.method_site().box_site().path().brand()) {
        return ParserMainAppEntryDispositionV1::IntegrityInvalid(
            ParserMainAppEntryIntegrityIssueV1::ForeignParameterCatalog,
        );
    }

    let identity_matches = catalog
        .declarations()
        .iter()
        .filter(|row| row.callable_identity().same_as(parent.method_identity()))
        .collect::<Vec<_>>();
    let coordinate_matches = catalog
        .declarations()
        .iter()
        .filter(|row| row.source_site() == parent.method_site())
        .count();

    let row = match identity_matches.as_slice() {
        [] if coordinate_matches == 0 => {
            return ParserMainAppEntryDispositionV1::Incomplete(
                ParserMainAppEntryIncompleteV1::CallableCatalogRowMissing,
            )
        }
        [] => {
            return ParserMainAppEntryDispositionV1::IntegrityInvalid(
                ParserMainAppEntryIntegrityIssueV1::CallableRelationMismatch,
            )
        }
        [_, _, ..] => {
            return ParserMainAppEntryDispositionV1::IntegrityInvalid(
                ParserMainAppEntryIntegrityIssueV1::DuplicateCallableCatalogRow,
            )
        }
        [row] => *row,
    };

    if row.source_site() != parent.method_site() {
        return ParserMainAppEntryDispositionV1::IntegrityInvalid(
            ParserMainAppEntryIntegrityIssueV1::CallableSiteMismatch,
        );
    }
    if row.kind() != ParserCallableDeclarationKindV1::StaticBoxMethod {
        return ParserMainAppEntryDispositionV1::IntegrityInvalid(
            ParserMainAppEntryIntegrityIssueV1::CallableKindMismatch,
        );
    }
    if row.diagnostic_name() != "main" {
        return ParserMainAppEntryDispositionV1::Outside(
            ParserMainAppEntryOutsideReasonV1::NonMainMethod,
        );
    }
    if !row.parameters().is_empty() {
        return ParserMainAppEntryDispositionV1::Outside(
            ParserMainAppEntryOutsideReasonV1::NonZeroMainArity,
        );
    }

    ParserMainAppEntryDispositionV1::AppMainReady(ParserMainAppEntrySealV1 {
        box_site: parent.box_site().clone(),
        method_site: parent.method_site().clone(),
        callable_identity: parent.method_identity().clone(),
    })
}
