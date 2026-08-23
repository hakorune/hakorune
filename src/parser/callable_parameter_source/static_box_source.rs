//! Parser-owned source preservation for the bounded static-Box parent cohort.
//!
//! This is deliberately a sibling of `ParserBoxSourceSealV1`.  The ordinary
//! seal owns ordinary-Box postpass relations; this module owns only the
//! parser-branded static parent header/member coverage needed by its first
//! source-only cohort.

use super::super::callable_source_anchor::{
    DirectCallableDeclarationKindV1, PreparedCallableSourceV1,
};
use super::super::postpass_envelope::ParserPostpassProgramCohortV1;
use super::super::source_authority::{
    ParserInvocationBrandV1, SourceBoxDeclarationSiteV1, SourceBoxMemberSiteV1,
    SourceBoxMethodSiteV1,
};
use super::super::source_member_cursor::{
    ParserBoxMemberSourceCursorErrorV1, ParserBoxMemberSourceCursorV1,
};
use super::super::source_path::SourceBoxDeclarationPathV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserStaticBoxMemberKindV1 {
    DirectMethod,
    Field,
    InitBlock,
    StaticInitializer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::parser) struct ParserStaticBoxDeclarationSyntaxV1 {
    name: Box<str>,
    is_sync: bool,
}

impl ParserStaticBoxDeclarationSyntaxV1 {
    pub(in crate::parser) fn static_box(name: String) -> Self {
        Self {
            name: name.into_boxed_str(),
            is_sync: false,
        }
    }

    pub(in crate::parser) fn name(&self) -> &str {
        &self.name
    }

    pub(in crate::parser) fn is_sync(&self) -> bool {
        self.is_sync
    }
}

#[derive(Debug)]
enum PreparedParserStaticBoxMemberSourceRowV1 {
    DirectMethod {
        site: SourceBoxMethodSiteV1,
    },
    Unsupported {
        site: SourceBoxMemberSiteV1,
        kind: ParserStaticBoxMemberKindV1,
    },
}

impl PreparedParserStaticBoxMemberSourceRowV1 {
    fn kind(&self) -> ParserStaticBoxMemberKindV1 {
        match self {
            Self::DirectMethod { .. } => ParserStaticBoxMemberKindV1::DirectMethod,
            Self::Unsupported { kind, .. } => *kind,
        }
    }
}

#[derive(Debug)]
pub(in crate::parser) struct OpenParserStaticBoxSourceTransactionV1 {
    brand: ParserInvocationBrandV1,
    box_site: SourceBoxDeclarationSiteV1,
    syntax: ParserStaticBoxDeclarationSyntaxV1,
    cursor: ParserBoxMemberSourceCursorV1,
    rows: Vec<PreparedParserStaticBoxMemberSourceRowV1>,
}

impl OpenParserStaticBoxSourceTransactionV1 {
    pub(in crate::parser) fn open(
        brand: ParserInvocationBrandV1,
        path: SourceBoxDeclarationPathV1,
        name: String,
    ) -> Self {
        let cursor = ParserBoxMemberSourceCursorV1::open_with_path(brand.clone(), path);
        Self {
            box_site: cursor.box_site().clone(),
            brand,
            syntax: ParserStaticBoxDeclarationSyntaxV1::static_box(name),
            cursor,
            rows: Vec::new(),
        }
    }

    pub(in crate::parser) fn current_member_site(&self) -> SourceBoxMemberSiteV1 {
        self.cursor.current_member_site()
    }

    pub(in crate::parser) fn current_program_callable_path(
        &self,
    ) -> super::super::source_path::SourceProgramCallablePathV1 {
        self.cursor.current_program_callable_path()
    }

    pub(in crate::parser) fn commit_unsupported_member(
        &mut self,
        kind: ParserStaticBoxMemberKindV1,
    ) -> Result<(), ParserStaticBoxSourceIssueV1> {
        if kind == ParserStaticBoxMemberKindV1::DirectMethod {
            return Err(ParserStaticBoxSourceIssueV1::DirectMethodNeedsRelation);
        }
        let site = self.current_member_site();
        self.rows
            .push(PreparedParserStaticBoxMemberSourceRowV1::Unsupported { site, kind });
        self.finish_member()
    }

    pub(in crate::parser) fn commit_direct_method(
        &mut self,
        site: SourceBoxMethodSiteV1,
    ) -> Result<(), ParserStaticBoxSourceIssueV1> {
        let expected = self.current_member_site();
        if !site.is_direct() {
            return Err(ParserStaticBoxSourceIssueV1::NonDirectMethod);
        }
        if site.box_site() != &self.box_site || site.member_site() != &expected {
            return Err(ParserStaticBoxSourceIssueV1::ForeignOrStaleMethodSite);
        }
        self.rows
            .push(PreparedParserStaticBoxMemberSourceRowV1::DirectMethod { site });
        self.finish_member()
    }

    fn finish_member(&mut self) -> Result<(), ParserStaticBoxSourceIssueV1> {
        self.cursor
            .finish_member()
            .map_err(ParserStaticBoxSourceIssueV1::Cursor)
    }

    pub(in crate::parser) fn finish(
        self,
    ) -> Result<PreparedParserStaticBoxParentSourceV1, ParserStaticBoxSourceIssueV1> {
        let member_count = self.cursor.current_member_ordinal();
        if usize::try_from(member_count).ok() != Some(self.rows.len()) {
            return Err(ParserStaticBoxSourceIssueV1::MemberCoverageMismatch);
        }
        Ok(PreparedParserStaticBoxParentSourceV1 {
            brand: self.brand,
            box_site: self.box_site,
            syntax: self.syntax,
            member_count,
            rows: self.rows.into_boxed_slice(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserStaticBoxSourceIssueV1 {
    Cursor(ParserBoxMemberSourceCursorErrorV1),
    DirectMethodNeedsRelation,
    NonDirectMethod,
    ForeignOrStaleMethodSite,
    MemberCoverageMismatch,
}

#[derive(Debug)]
pub(in crate::parser) struct PreparedParserStaticBoxParentSourceV1 {
    brand: ParserInvocationBrandV1,
    box_site: SourceBoxDeclarationSiteV1,
    syntax: ParserStaticBoxDeclarationSyntaxV1,
    member_count: u32,
    rows: Box<[PreparedParserStaticBoxMemberSourceRowV1]>,
}

impl PreparedParserStaticBoxParentSourceV1 {
    pub(in crate::parser) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }
}

#[derive(Debug)]
pub(in crate::parser) struct ParserStaticBoxSourceSealV1 {
    prepared: PreparedParserStaticBoxParentSourceV1,
}

impl ParserStaticBoxSourceSealV1 {
    pub(in crate::parser) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.prepared.box_site
    }

    pub(in crate::parser) fn declaration_syntax(&self) -> &ParserStaticBoxDeclarationSyntaxV1 {
        &self.prepared.syntax
    }

    pub(in crate::parser) fn member_count(&self) -> u32 {
        self.prepared.member_count
    }

    pub(in crate::parser) fn member_kinds(
        &self,
    ) -> impl Iterator<Item = ParserStaticBoxMemberKindV1> + '_ {
        self.prepared
            .rows
            .iter()
            .map(PreparedParserStaticBoxMemberSourceRowV1::kind)
    }
}

#[derive(Debug)]
pub(in crate::parser) enum ParserStaticBoxParentSourceDispositionV1 {
    Ready(ParserStaticBoxSourceSealV1),
    Outside(ParserStaticBoxParentOutsideReasonV1),
    SourceAuthorityUnavailable(ParserStaticBoxParentSourceUnavailableV1),
    Incomplete(ParserStaticBoxParentSourceIncompleteV1),
    IntegrityInvalid(ParserStaticBoxParentSourceIntegrityIssueV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserStaticBoxParentOutsideReasonV1 {
    ProgramCohort,
    MultipleParentRows,
    BuildGatePath,
    UnsupportedMemberKind,
    DirectMethodCohort,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserStaticBoxParentSourceUnavailableV1 {
    NoPreparedParent,
    OrdinarySourcePath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserStaticBoxParentSourceIncompleteV1 {
    StaticMethodSourceMissing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserStaticBoxParentSourceIntegrityIssueV1 {
    ForeignParserBrand,
    MemberCoverageMismatch,
    DuplicateStaticMethodSource,
}

impl ParserStaticBoxParentSourceDispositionV1 {
    pub(in crate::parser) fn unavailable_for_ordinary() -> Self {
        Self::SourceAuthorityUnavailable(
            ParserStaticBoxParentSourceUnavailableV1::OrdinarySourcePath,
        )
    }
}

pub(in crate::parser) struct ParserStaticBoxParentSourceAuthorityIssuerV1;

impl ParserStaticBoxParentSourceAuthorityIssuerV1 {
    pub(in crate::parser) fn issue_once(
        cohort: ParserPostpassProgramCohortV1,
        prepared: Vec<PreparedParserStaticBoxParentSourceV1>,
        callable_rows: &[PreparedCallableSourceV1],
    ) -> ParserStaticBoxParentSourceDispositionV1 {
        if !matches!(cohort, ParserPostpassProgramCohortV1::StaticBox) {
            return ParserStaticBoxParentSourceDispositionV1::Outside(
                ParserStaticBoxParentOutsideReasonV1::ProgramCohort,
            );
        }
        let Some(prepared) = (match prepared.len() {
            0 => {
                return ParserStaticBoxParentSourceDispositionV1::SourceAuthorityUnavailable(
                    ParserStaticBoxParentSourceUnavailableV1::NoPreparedParent,
                )
            }
            1 => prepared.into_iter().next(),
            _ => {
                return ParserStaticBoxParentSourceDispositionV1::Outside(
                    ParserStaticBoxParentOutsideReasonV1::MultipleParentRows,
                )
            }
        }) else {
            unreachable!("static parent row length one was checked")
        };

        if prepared.box_site.path().segments().len() != 1 {
            return ParserStaticBoxParentSourceDispositionV1::Outside(
                ParserStaticBoxParentOutsideReasonV1::BuildGatePath,
            );
        }
        if !prepared.brand.same_as(prepared.box_site.path().brand()) {
            return ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid(
                ParserStaticBoxParentSourceIntegrityIssueV1::ForeignParserBrand,
            );
        }
        if usize::try_from(prepared.member_count).ok() != Some(prepared.rows.len()) {
            return ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid(
                ParserStaticBoxParentSourceIntegrityIssueV1::MemberCoverageMismatch,
            );
        }
        if prepared
            .rows
            .iter()
            .any(|row| row.kind() != ParserStaticBoxMemberKindV1::DirectMethod)
        {
            return ParserStaticBoxParentSourceDispositionV1::Outside(
                ParserStaticBoxParentOutsideReasonV1::UnsupportedMemberKind,
            );
        }
        let direct_methods = prepared
            .rows
            .iter()
            .filter_map(|row| match row {
                PreparedParserStaticBoxMemberSourceRowV1::DirectMethod { site } => Some(site),
                PreparedParserStaticBoxMemberSourceRowV1::Unsupported { .. } => None,
            })
            .collect::<Vec<_>>();
        if direct_methods.len() != 1 {
            return ParserStaticBoxParentSourceDispositionV1::Outside(
                ParserStaticBoxParentOutsideReasonV1::DirectMethodCohort,
            );
        }
        let method_site = direct_methods[0].member_site();
        let matches = callable_rows
            .iter()
            .filter_map(PreparedCallableSourceV1::direct)
            .filter(|row| row.kind() == DirectCallableDeclarationKindV1::StaticBoxMethod)
            .filter(|row| callable_row_matches(row, prepared.box_site.path(), method_site))
            .count();
        match matches {
            0 => ParserStaticBoxParentSourceDispositionV1::Incomplete(
                ParserStaticBoxParentSourceIncompleteV1::StaticMethodSourceMissing,
            ),
            1 => ParserStaticBoxParentSourceDispositionV1::Ready(ParserStaticBoxSourceSealV1 {
                prepared,
            }),
            _ => ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid(
                ParserStaticBoxParentSourceIntegrityIssueV1::DuplicateStaticMethodSource,
            ),
        }
    }
}

fn callable_row_matches(
    row: &super::super::callable_source_anchor::PreparedDirectCallableSourceV1,
    box_path: &SourceBoxDeclarationPathV1,
    member_site: &SourceBoxMemberSiteV1,
) -> bool {
    let Some((declaration, gate_path, member_ordinal)) = row.path().box_method_parts() else {
        return false;
    };
    gate_path.is_empty()
        && declaration.compatibility_box_path() == box_path
        && member_ordinal == member_site.member_ordinal()
}
