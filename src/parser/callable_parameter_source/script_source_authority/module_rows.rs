//! Parser-owned ordinary module source rows.
//!
//! This module only co-seals already-issued parser products. It does not scan
//! the AST, select a runtime entry, or issue a semantic/physical product.

use crate::parser::callable_parameter_source::catalog::ParserCallableParameterSourceCatalogV1;
use crate::parser::callable_parameter_source::parser_invocation_witness::ParserInvocationWitnessV1;
use crate::parser::callable_source_anchor::{
    CallableDeclarationIdentityV1, DirectCallableDeclarationKindV1,
};
use crate::parser::postpass_envelope::CompletedParserPostpassV1;
use crate::parser::source_authority::{
    ParserBoxDeclarationKindV1, SourceBoxDeclarationSiteV1, SourceBoxMethodSiteV1,
};

use super::super::model::ParserCallableDeclarationKindV1;
use super::model::{ParserNormalProgramBodySourceRowV1, ParserNormalProgramBodySyntaxKindV1};

#[derive(Debug)]
pub(in crate::parser) enum ParserNormalModuleSourceRowsDispositionV1 {
    Ready(ParserNormalModuleSourceRowsV1),
    SourceAuthorityUnavailable(ParserNormalModuleSourceRowsUnavailableV1),
    Incomplete(ParserNormalModuleSourceRowsIncompleteV1),
    IntegrityInvalid(ParserNormalModuleSourceRowsIntegrityIssueV1),
    Outside(ParserNormalModuleSourceRowsOutsideReasonV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalModuleSourceRowsUnavailableV1 {
    PostpassNotSourceBacked,
    ParameterSourceUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalModuleSourceRowsIncompleteV1 {
    ProgramBodyCoverage,
    BoxSourceSealMissing,
    MethodSourceRelationMissing,
    CallableSourceMissing,
    ParameterArityOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalModuleSourceRowsIntegrityIssueV1 {
    BoxCoverageMismatch,
    ForeignParser,
    MethodSourceRelationMismatch,
    CallableSourceMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalModuleSourceRowsOutsideReasonV1 {
    UnsupportedProgramBody,
    MultipleBoxSourceSeals,
    UnsupportedBoxKind,
    MultipleMethodRows,
    StaticMethod,
    SelectedBuildGateMethod,
    GeneratedCallableRow,
    NonDirectCallablePath,
    NonOrdinaryParameterTransfer,
}

#[derive(Debug)]
pub(in crate::parser) struct ParserNormalModuleSourceRowsV1 {
    invocation: ParserInvocationWitnessV1,
    box_row: ParserNormalModuleBoxSourceRowV1,
    _seal: ParserNormalModuleSourceRowsSealV1,
}

#[derive(Debug)]
pub(in crate::parser) struct ParserNormalModuleBoxSourceRowV1 {
    program_position: u32,
    final_box_ordinal: usize,
    box_site: SourceBoxDeclarationSiteV1,
    declaration_syntax: crate::parser::source_authority::ParserBoxDeclarationSyntaxV1,
    method: ParserNormalModuleMethodSourceRowV1,
}

#[derive(Debug)]
pub(in crate::parser) struct ParserNormalModuleMethodSourceRowV1 {
    source_site: SourceBoxMethodSiteV1,
    diagnostic_name: Box<str>,
    arity: u32,
    callable_identity: CallableDeclarationIdentityV1,
}

#[derive(Debug)]
pub(in crate::parser) struct ParserNormalModuleSourceRowsSealV1;

impl ParserNormalModuleSourceRowsDispositionV1 {
    pub(in crate::parser) fn is_ready(&self) -> bool {
        matches!(self, Self::Ready(_))
    }
}

impl ParserNormalModuleSourceRowsV1 {
    pub(in crate::parser) fn invocation(&self) -> &ParserInvocationWitnessV1 {
        &self.invocation
    }

    pub(in crate::parser) fn box_row(&self) -> &ParserNormalModuleBoxSourceRowV1 {
        &self.box_row
    }
}

impl ParserNormalModuleBoxSourceRowV1 {
    pub(in crate::parser) fn program_position(&self) -> u32 {
        self.program_position
    }

    pub(in crate::parser) fn final_box_ordinal(&self) -> usize {
        self.final_box_ordinal
    }

    pub(in crate::parser) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }

    pub(in crate::parser) fn declaration_syntax(
        &self,
    ) -> &crate::parser::source_authority::ParserBoxDeclarationSyntaxV1 {
        &self.declaration_syntax
    }

    pub(in crate::parser) fn method(&self) -> &ParserNormalModuleMethodSourceRowV1 {
        &self.method
    }
}

impl ParserNormalModuleMethodSourceRowV1 {
    pub(in crate::parser) fn source_site(&self) -> &SourceBoxMethodSiteV1 {
        &self.source_site
    }

    pub(in crate::parser) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }

    pub(in crate::parser) fn arity(&self) -> u32 {
        self.arity
    }

    pub(in crate::parser) fn callable_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.callable_identity
    }
}

pub(super) struct ParserNormalModuleSourceAuthorityIssuerV1;

impl ParserNormalModuleSourceAuthorityIssuerV1 {
    pub(super) fn issue_once(
        completed: &CompletedParserPostpassV1,
        catalog: &ParserCallableParameterSourceCatalogV1,
        body_rows: &[ParserNormalProgramBodySourceRowV1],
        invocation: ParserInvocationWitnessV1,
    ) -> ParserNormalModuleSourceRowsDispositionV1 {
        if !completed.is_source_backed() {
            return ParserNormalModuleSourceRowsDispositionV1::SourceAuthorityUnavailable(
                ParserNormalModuleSourceRowsUnavailableV1::PostpassNotSourceBacked,
            );
        }
        if body_rows.len() != 1
            || body_rows[0].position() != 0
            || body_rows[0].kind() != ParserNormalProgramBodySyntaxKindV1::BoxDeclaration
        {
            return ParserNormalModuleSourceRowsDispositionV1::Outside(
                ParserNormalModuleSourceRowsOutsideReasonV1::UnsupportedProgramBody,
            );
        }

        let rows = completed.box_coverage().rows();
        let mut sealed_rows = rows.iter().filter_map(|row| row.source_sealed());
        let Some((final_box_ordinal, seal)) = sealed_rows.next() else {
            return ParserNormalModuleSourceRowsDispositionV1::Incomplete(
                ParserNormalModuleSourceRowsIncompleteV1::BoxSourceSealMissing,
            );
        };
        if sealed_rows.next().is_some() {
            return ParserNormalModuleSourceRowsDispositionV1::Outside(
                ParserNormalModuleSourceRowsOutsideReasonV1::MultipleBoxSourceSeals,
            );
        }
        if rows.len() != 1 || final_box_ordinal != body_rows[0].position() as usize {
            return ParserNormalModuleSourceRowsDispositionV1::IntegrityInvalid(
                ParserNormalModuleSourceRowsIntegrityIssueV1::BoxCoverageMismatch,
            );
        }
        if seal.declaration_syntax().kind() != ParserBoxDeclarationKindV1::Ordinary {
            return ParserNormalModuleSourceRowsDispositionV1::Outside(
                ParserNormalModuleSourceRowsOutsideReasonV1::UnsupportedBoxKind,
            );
        }
        if !catalog.same_parser_brand(seal.box_site().path().brand()) {
            return ParserNormalModuleSourceRowsDispositionV1::IntegrityInvalid(
                ParserNormalModuleSourceRowsIntegrityIssueV1::ForeignParser,
            );
        }

        let Some(catalog_row) = catalog.declarations().first() else {
            return ParserNormalModuleSourceRowsDispositionV1::Incomplete(
                ParserNormalModuleSourceRowsIncompleteV1::CallableSourceMissing,
            );
        };
        if catalog.declarations().len() != 1 {
            return ParserNormalModuleSourceRowsDispositionV1::Outside(
                ParserNormalModuleSourceRowsOutsideReasonV1::MultipleMethodRows,
            );
        }
        if catalog_row.kind() != ParserCallableDeclarationKindV1::InstanceBoxMethod {
            return ParserNormalModuleSourceRowsDispositionV1::Outside(
                ParserNormalModuleSourceRowsOutsideReasonV1::StaticMethod,
            );
        }
        if !catalog_row.source_site().is_direct() {
            return ParserNormalModuleSourceRowsDispositionV1::Outside(
                ParserNormalModuleSourceRowsOutsideReasonV1::SelectedBuildGateMethod,
            );
        }
        if catalog_row
            .parameters()
            .iter()
            .any(|row| !row.transfer().is_ordinary())
        {
            return ParserNormalModuleSourceRowsDispositionV1::Outside(
                ParserNormalModuleSourceRowsOutsideReasonV1::NonOrdinaryParameterTransfer,
            );
        }
        let Ok(arity) = u32::try_from(catalog_row.parameters().len()) else {
            return ParserNormalModuleSourceRowsDispositionV1::Incomplete(
                ParserNormalModuleSourceRowsIncompleteV1::ParameterArityOverflow,
            );
        };
        if catalog_row.source_site().box_site() != seal.box_site() {
            return ParserNormalModuleSourceRowsDispositionV1::IntegrityInvalid(
                ParserNormalModuleSourceRowsIntegrityIssueV1::MethodSourceRelationMismatch,
            );
        }

        let Some(relation) = seal.method_relations().first() else {
            return ParserNormalModuleSourceRowsDispositionV1::Incomplete(
                ParserNormalModuleSourceRowsIncompleteV1::MethodSourceRelationMissing,
            );
        };
        if seal.method_relations().len() != 1
            || relation.source_site() != Some(catalog_row.source_site())
            || relation.inventory_ordinal() != catalog_row.inventory_ordinal()
        {
            return ParserNormalModuleSourceRowsDispositionV1::IntegrityInvalid(
                ParserNormalModuleSourceRowsIntegrityIssueV1::MethodSourceRelationMismatch,
            );
        }

        if completed.callable_rows().len() != 1 {
            return if completed
                .callable_rows()
                .iter()
                .any(|row| row.generated().is_some())
            {
                ParserNormalModuleSourceRowsDispositionV1::Outside(
                    ParserNormalModuleSourceRowsOutsideReasonV1::GeneratedCallableRow,
                )
            } else {
                ParserNormalModuleSourceRowsDispositionV1::Outside(
                    ParserNormalModuleSourceRowsOutsideReasonV1::MultipleMethodRows,
                )
            };
        }
        let Some(direct) = completed.callable_rows()[0].direct() else {
            return ParserNormalModuleSourceRowsDispositionV1::Outside(
                ParserNormalModuleSourceRowsOutsideReasonV1::NonDirectCallablePath,
            );
        };
        if direct.kind() != DirectCallableDeclarationKindV1::InstanceBoxMethod {
            return ParserNormalModuleSourceRowsDispositionV1::Outside(
                ParserNormalModuleSourceRowsOutsideReasonV1::StaticMethod,
            );
        }
        let Some((declaration, gate_path, member_ordinal)) = direct.path().box_method_parts()
        else {
            return ParserNormalModuleSourceRowsDispositionV1::IntegrityInvalid(
                ParserNormalModuleSourceRowsIntegrityIssueV1::CallableSourceMismatch,
            );
        };
        if !gate_path.is_empty()
            || declaration.compatibility_box_path() != seal.box_site().path()
            || member_ordinal != catalog_row.source_site().source_member_ordinal()
        {
            return ParserNormalModuleSourceRowsDispositionV1::IntegrityInvalid(
                ParserNormalModuleSourceRowsIntegrityIssueV1::CallableSourceMismatch,
            );
        }

        ParserNormalModuleSourceRowsDispositionV1::Ready(ParserNormalModuleSourceRowsV1 {
            invocation,
            box_row: ParserNormalModuleBoxSourceRowV1 {
                program_position: body_rows[0].position(),
                final_box_ordinal,
                box_site: seal.box_site().clone(),
                declaration_syntax: seal.declaration_syntax().clone(),
                method: ParserNormalModuleMethodSourceRowV1 {
                    source_site: catalog_row.source_site().clone(),
                    diagnostic_name: catalog_row.diagnostic_name().into(),
                    arity,
                    callable_identity: direct.anchor().identity(),
                },
            },
            _seal: ParserNormalModuleSourceRowsSealV1,
        })
    }
}
