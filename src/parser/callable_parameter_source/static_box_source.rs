//! Parser-owned source preservation for the bounded static-Box parent cohort.
//!
//! This is deliberately a sibling of `ParserBoxSourceSealV1`.  The ordinary
//! seal owns ordinary-Box postpass relations; this module owns only the
//! parser-branded static parent header/member coverage needed by its first
//! source-only cohort.

use super::super::callable_source_anchor::{
    CallableDeclarationIdentityV1, DirectCallableDeclarationKindV1, PreparedCallableSourceV1,
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
        callable_identity: CallableDeclarationIdentityV1,
    },
    Unsupported {
        _site: SourceBoxMemberSiteV1,
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
    declaration_line: usize,
    syntax: ParserStaticBoxDeclarationSyntaxV1,
    cursor: ParserBoxMemberSourceCursorV1,
    rows: Vec<PreparedParserStaticBoxMemberSourceRowV1>,
}

impl OpenParserStaticBoxSourceTransactionV1 {
    pub(in crate::parser) fn open(
        brand: ParserInvocationBrandV1,
        path: SourceBoxDeclarationPathV1,
        name: String,
        declaration_line: usize,
    ) -> Self {
        let cursor = ParserBoxMemberSourceCursorV1::open_with_path(brand.clone(), path);
        Self {
            box_site: cursor.box_site().clone(),
            declaration_line,
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
            .push(PreparedParserStaticBoxMemberSourceRowV1::Unsupported { _site: site, kind });
        self.finish_member()
    }

    pub(in crate::parser) fn commit_direct_method(
        &mut self,
        site: SourceBoxMethodSiteV1,
        callable_identity: CallableDeclarationIdentityV1,
    ) -> Result<(), ParserStaticBoxSourceIssueV1> {
        let expected = self.current_member_site();
        if !site.is_direct() {
            return Err(ParserStaticBoxSourceIssueV1::NonDirectMethod);
        }
        if site.box_site() != &self.box_site || site.member_site() != &expected {
            return Err(ParserStaticBoxSourceIssueV1::ForeignOrStaleMethodSite);
        }
        self.rows
            .push(PreparedParserStaticBoxMemberSourceRowV1::DirectMethod {
                site,
                callable_identity,
            });
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
            declaration_line: self.declaration_line,
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
    declaration_line: usize,
    syntax: ParserStaticBoxDeclarationSyntaxV1,
    member_count: u32,
    rows: Box<[PreparedParserStaticBoxMemberSourceRowV1]>,
}

impl PreparedParserStaticBoxParentSourceV1 {
    pub(in crate::parser) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }

    pub(in crate::parser) fn declaration_syntax(&self) -> &ParserStaticBoxDeclarationSyntaxV1 {
        &self.syntax
    }

    pub(in crate::parser) fn member_count(&self) -> u32 {
        self.member_count
    }

    pub(in crate::parser) fn member_kinds(
        &self,
    ) -> impl ExactSizeIterator<Item = ParserStaticBoxMemberKindV1> + '_ {
        self.rows
            .iter()
            .map(PreparedParserStaticBoxMemberSourceRowV1::kind)
    }

    pub(in crate::parser) fn direct_method_relations(
        &self,
    ) -> impl Iterator<Item = (&SourceBoxMethodSiteV1, &CallableDeclarationIdentityV1)> + '_ {
        self.rows.iter().filter_map(|row| match row {
            PreparedParserStaticBoxMemberSourceRowV1::DirectMethod {
                site,
                callable_identity,
            } => Some((site, callable_identity)),
            PreparedParserStaticBoxMemberSourceRowV1::Unsupported { .. } => None,
        })
    }
}

#[derive(Debug)]
struct ParserStaticBoxMethodSourceSealV1 {
    site: SourceBoxMethodSiteV1,
    identity: CallableDeclarationIdentityV1,
}

#[derive(Debug)]
struct ParserStaticBoxParentSourceSealV1 {
    box_site: SourceBoxDeclarationSiteV1,
    declaration_line: usize,
    syntax: ParserStaticBoxDeclarationSyntaxV1,
    member_count: u32,
    member_kinds: Box<[ParserStaticBoxMemberKindV1]>,
    methods: Box<[ParserStaticBoxMethodSourceSealV1]>,
}

impl ParserStaticBoxParentSourceSealV1 {
    fn matches_prepared(&self, prepared: &PreparedParserStaticBoxParentSourceV1) -> bool {
        self.box_site == prepared.box_site
            && self.syntax == *prepared.declaration_syntax()
            && self.member_count == prepared.member_count
            && self.member_kinds.len() == prepared.rows.len()
            && self.member_kinds.iter().copied().eq(prepared
                .rows
                .iter()
                .map(PreparedParserStaticBoxMemberSourceRowV1::kind))
            && self.methods.len() == prepared.direct_method_relations().count()
            && self.methods.iter().all(|method| {
                prepared.direct_method_relations().any(|(site, identity)| {
                    site == &method.site && identity.same_as(&method.identity)
                })
            })
    }

    fn direct_method_relations(
        &self,
    ) -> impl Iterator<Item = (&SourceBoxMethodSiteV1, &CallableDeclarationIdentityV1)> + '_ {
        self.methods
            .iter()
            .map(|method| (&method.site, &method.identity))
    }
}

#[derive(Debug)]
pub(in crate::parser) struct ParserStaticBoxSourceSealV1 {
    parents: Box<[ParserStaticBoxParentSourceSealV1]>,
}

impl ParserStaticBoxSourceSealV1 {
    pub(in crate::parser) fn declaration_coordinates(
        &self,
    ) -> impl Iterator<Item = (&ParserInvocationBrandV1, &SourceBoxDeclarationPathV1, usize)> {
        self.parents.iter().map(|parent| {
            (
                parent.box_site.path().brand(),
                parent.box_site.path(),
                parent.declaration_line,
            )
        })
    }

    pub(in crate::parser) fn matches_prepared_parent(
        &self,
        prepared: &PreparedParserStaticBoxParentSourceV1,
    ) -> bool {
        self.parents
            .iter()
            .any(|parent| parent.matches_prepared(prepared))
    }

    pub(in crate::parser) fn declaration_syntax(&self) -> &ParserStaticBoxDeclarationSyntaxV1 {
        &self.parents[0].syntax
    }

    pub(in crate::parser) fn member_count(&self) -> u32 {
        self.parents[0].member_count
    }

    pub(in crate::parser) fn member_kinds(
        &self,
    ) -> impl Iterator<Item = ParserStaticBoxMemberKindV1> + '_ {
        self.parents[0].member_kinds.iter().copied()
    }

    pub(in crate::parser) fn method_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.parents[0].methods[0].identity
    }

    pub(in crate::parser) fn direct_method_relations(
        &self,
    ) -> impl Iterator<Item = (&SourceBoxMethodSiteV1, &CallableDeclarationIdentityV1)> + '_ {
        self.parents
            .iter()
            .flat_map(ParserStaticBoxParentSourceSealV1::direct_method_relations)
    }
}

#[derive(Debug)]
pub(in crate::parser) enum ParserStaticBoxParentSourceDispositionV1 {
    Ready(ParserStaticBoxSourceSealV1),
    Outside(ParserStaticBoxParentOutsideReasonV1),
    SourceAuthorityUnavailable(ParserStaticBoxParentSourceUnavailableV1),
    Incomplete(ParserStaticBoxParentSourceIncompleteV1),
    IntegrityInvalid {
        _error: ParserStaticBoxParentSourceIntegrityIssueV1,
    },
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
    MethodRelationMismatch,
    DuplicateStaticMethodSource,
    DuplicateStaticParentSource,
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
        prepared: &[PreparedParserStaticBoxParentSourceV1],
        callable_rows: &[PreparedCallableSourceV1],
    ) -> ParserStaticBoxParentSourceDispositionV1 {
        if !matches!(
            cohort,
            ParserPostpassProgramCohortV1::StaticBox | ParserPostpassProgramCohortV1::MixedProgram
        ) {
            return ParserStaticBoxParentSourceDispositionV1::Outside(
                ParserStaticBoxParentOutsideReasonV1::ProgramCohort,
            );
        }
        let Some(first) = prepared.first() else {
            return ParserStaticBoxParentSourceDispositionV1::SourceAuthorityUnavailable(
                ParserStaticBoxParentSourceUnavailableV1::NoPreparedParent,
            );
        };
        let parser_brand = first.brand.clone();
        let mut parent_paths = Vec::with_capacity(prepared.len());
        let mut parent_seals: Vec<ParserStaticBoxParentSourceSealV1> =
            Vec::with_capacity(prepared.len());
        for parent in prepared {
            if parent.box_site.path().segments().len() != 1 {
                return ParserStaticBoxParentSourceDispositionV1::Outside(
                    ParserStaticBoxParentOutsideReasonV1::BuildGatePath,
                );
            }
            if !parent.brand.same_as(&parser_brand)
                || !parent.brand.same_as(parent.box_site.path().brand())
            {
                return ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid {
                    _error: ParserStaticBoxParentSourceIntegrityIssueV1::ForeignParserBrand,
                };
            }
            if parent_paths
                .iter()
                .any(|path: &SourceBoxDeclarationPathV1| path == parent.box_site.path())
            {
                return ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid {
                    _error:
                        ParserStaticBoxParentSourceIntegrityIssueV1::DuplicateStaticParentSource,
                };
            }
            parent_paths.push(parent.box_site.path().clone());
            if usize::try_from(parent.member_count).ok() != Some(parent.rows.len()) {
                return ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid {
                    _error: ParserStaticBoxParentSourceIntegrityIssueV1::MemberCoverageMismatch,
                };
            }
            if parent
                .rows
                .iter()
                .any(|row| row.kind() != ParserStaticBoxMemberKindV1::DirectMethod)
            {
                return ParserStaticBoxParentSourceDispositionV1::Outside(
                    ParserStaticBoxParentOutsideReasonV1::UnsupportedMemberKind,
                );
            }
            let direct_methods = parent
                .rows
                .iter()
                .filter_map(|row| match row {
                    PreparedParserStaticBoxMemberSourceRowV1::DirectMethod {
                        site,
                        callable_identity,
                    } => Some((site, callable_identity)),
                    PreparedParserStaticBoxMemberSourceRowV1::Unsupported { .. } => None,
                })
                .collect::<Vec<_>>();
            if direct_methods.is_empty() {
                return ParserStaticBoxParentSourceDispositionV1::Outside(
                    ParserStaticBoxParentOutsideReasonV1::DirectMethodCohort,
                );
            }
            let mut method_seals = Vec::with_capacity(direct_methods.len());
            for (method_site, method_identity) in direct_methods {
                if parent_seals
                    .iter()
                    .flat_map(|parent_seal| parent_seal.methods.iter())
                    .any(|row| row.identity.same_as(method_identity))
                    || method_seals
                        .iter()
                        .any(|row: &ParserStaticBoxMethodSourceSealV1| {
                            row.identity.same_as(method_identity)
                        })
                {
                    return ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid {
                        _error:
                            ParserStaticBoxParentSourceIntegrityIssueV1::DuplicateStaticMethodSource,
                    };
                }
                let coordinate_matches = callable_rows
                    .iter()
                    .filter_map(PreparedCallableSourceV1::direct)
                    .filter(|row| row.kind() == DirectCallableDeclarationKindV1::StaticBoxMethod)
                    .filter(|row| {
                        callable_row_matches(row, parent.box_site.path(), method_site.member_site())
                    })
                    .count();
                if coordinate_matches > 1 {
                    return ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid {
                        _error:
                            ParserStaticBoxParentSourceIntegrityIssueV1::DuplicateStaticMethodSource,
                    };
                }
                let identity_matches = callable_rows
                    .iter()
                    .filter_map(PreparedCallableSourceV1::direct)
                    .filter(|row| row.kind() == DirectCallableDeclarationKindV1::StaticBoxMethod)
                    .filter(|row| row.anchor().identity().same_as(method_identity))
                    .collect::<Vec<_>>();
                match identity_matches.len() {
                    0 if coordinate_matches == 0 => {
                        return ParserStaticBoxParentSourceDispositionV1::Incomplete(
                            ParserStaticBoxParentSourceIncompleteV1::StaticMethodSourceMissing,
                        )
                    }
                    0 => {
                        return ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid {
                            _error:
                                ParserStaticBoxParentSourceIntegrityIssueV1::MethodRelationMismatch,
                        }
                    }
                    1 if callable_row_matches(
                        identity_matches[0],
                        parent.box_site.path(),
                        method_site.member_site(),
                    ) => {}
                    1 => {
                        return ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid {
                            _error:
                                ParserStaticBoxParentSourceIntegrityIssueV1::MethodRelationMismatch,
                        }
                    }
                    _ => return ParserStaticBoxParentSourceDispositionV1::IntegrityInvalid {
                        _error:
                            ParserStaticBoxParentSourceIntegrityIssueV1::DuplicateStaticMethodSource,
                    },
                }
                method_seals.push(ParserStaticBoxMethodSourceSealV1 {
                    site: method_site.clone(),
                    identity: method_identity.clone(),
                });
            }
            let member_kinds = parent
                .rows
                .iter()
                .map(PreparedParserStaticBoxMemberSourceRowV1::kind)
                .collect::<Vec<_>>()
                .into_boxed_slice();
            parent_seals.push(ParserStaticBoxParentSourceSealV1 {
                box_site: parent.box_site.clone(),
                declaration_line: parent.declaration_line,
                syntax: parent.syntax.clone(),
                member_count: parent.member_count,
                member_kinds,
                methods: method_seals.into_boxed_slice(),
            });
        }
        ParserStaticBoxParentSourceDispositionV1::Ready(ParserStaticBoxSourceSealV1 {
            parents: parent_seals.into_boxed_slice(),
        })
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
