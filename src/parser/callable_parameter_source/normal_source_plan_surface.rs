//! Parser-owned normal source-plan surface for the bounded I0-A handoff.
//!
//! This module co-seals final placement, parser source relation, and the
//! static-parent/member payload once.  It emits no policy, Recipe, or MIR
//! meaning.  The compiler must consume this product instead of rebuilding a
//! surface from the AST.

use crate::ast::ASTNode;

use super::super::callable_source_anchor::{
    CallableDeclarationIdentityV1, PreparedCallableSourceV1,
};
use super::catalog::ParserCallableParameterSourceDispositionV1;
use super::model::ParserCallableDeclarationKindV1;
use super::normal_source_plan_seed::ParserNormalSourcePlanSeedDispositionV1;
use super::parser_invocation_witness::ParserInvocationWitnessV1;
use super::static_box_source::PreparedParserStaticBoxParentSourceV1;
use crate::parser::build_cfg::program_item_slots::ProjectedProgramItemSlotV1;
use crate::parser::postpass_envelope::CompletedParserPostpassV1;
use crate::parser::source_authority::{SourceBoxDeclarationSiteV1, SourceBoxMethodSiteV1};
use crate::parser::source_path::SourceProgramCallablePathV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalSourcePlanUnsupportedKindV1 {
    NestedProgram,
    BuildGate,
    Using,
    Import,
    Enum,
    Brand,
    TypeAlias,
    Global,
    StaticConstTable,
}

#[derive(Debug)]
pub(crate) enum ParserNormalSourcePlanTopLevelRowV1 {
    Executable {
        slot: ProjectedProgramItemSlotV1,
    },
    TopLevelCallable {
        slot: ProjectedProgramItemSlotV1,
        callable_identity: CallableDeclarationIdentityV1,
    },
    StaticBox {
        slot: ProjectedProgramItemSlotV1,
        source: PreparedParserStaticBoxParentSourceV1,
    },
    OrdinaryBox {
        slot: ProjectedProgramItemSlotV1,
        source: ParserNormalSourcePlanOrdinaryBoxV1,
    },
    Unsupported {
        slot: ProjectedProgramItemSlotV1,
        kind: ParserNormalSourcePlanUnsupportedKindV1,
    },
}

impl ParserNormalSourcePlanTopLevelRowV1 {
    pub(crate) fn slot(&self) -> &ProjectedProgramItemSlotV1 {
        match self {
            Self::Executable { slot }
            | Self::TopLevelCallable { slot, .. }
            | Self::StaticBox { slot, .. }
            | Self::OrdinaryBox { slot, .. }
            | Self::Unsupported { slot, .. } => slot,
        }
    }
}

/// Policy-total projection of one parser-sealed ordinary Box.
///
/// This is issued only while the sole source-surface issuer can compare the
/// postpass seal, parameter catalog, and callable anchors from one parser
/// invocation. Names and source sites remain diagnostics/coverage; opaque
/// callable identities are the only cross-product relation.
#[derive(Debug)]
pub(crate) struct ParserNormalSourcePlanOrdinaryBoxV1 {
    box_site: SourceBoxDeclarationSiteV1,
    diagnostic_name: Box<str>,
    is_sync: bool,
    observed_member_count: u32,
    direct_methods: Box<[ParserNormalSourcePlanBoxMethodRelationV1]>,
}

#[derive(Debug)]
pub(crate) struct ParserNormalSourcePlanBoxMethodRelationV1 {
    source_site: SourceBoxMethodSiteV1,
    callable_identity: CallableDeclarationIdentityV1,
}

impl ParserNormalSourcePlanOrdinaryBoxV1 {
    pub(crate) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }

    pub(crate) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }

    pub(crate) const fn is_sync(&self) -> bool {
        self.is_sync
    }

    pub(crate) const fn observed_member_count(&self) -> u32 {
        self.observed_member_count
    }

    pub(crate) fn direct_method_relations(&self) -> &[ParserNormalSourcePlanBoxMethodRelationV1] {
        &self.direct_methods
    }
}

impl ParserNormalSourcePlanBoxMethodRelationV1 {
    pub(crate) fn source_site(&self) -> &SourceBoxMethodSiteV1 {
        &self.source_site
    }

    pub(crate) fn callable_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.callable_identity
    }
}

#[derive(Debug)]
pub(crate) struct NonEmptyParserNormalSourcePlanRowsV1 {
    rows: Box<[ParserNormalSourcePlanTopLevelRowV1]>,
}

impl NonEmptyParserNormalSourcePlanRowsV1 {
    fn issue(rows: Vec<ParserNormalSourcePlanTopLevelRowV1>) -> Option<Self> {
        if rows.is_empty() {
            None
        } else {
            Some(Self {
                rows: rows.into_boxed_slice(),
            })
        }
    }

    pub(crate) fn rows(&self) -> &[ParserNormalSourcePlanTopLevelRowV1] {
        &self.rows
    }
}

#[derive(Debug)]
pub(crate) enum ParserNormalSourcePlanSurfaceV1 {
    CompleteEmpty,
    CompleteRows(NonEmptyParserNormalSourcePlanRowsV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalSourcePlanCallableKindV1 {
    TopLevel,
    StaticBoxMethod,
    InstanceBoxMethod,
}

#[derive(Debug)]
pub(crate) struct ParserNormalSourcePlanCallableSyntaxV1 {
    identity: CallableDeclarationIdentityV1,
    diagnostic_name: Box<str>,
    arity: u32,
    kind: ParserNormalSourcePlanCallableKindV1,
}

impl ParserNormalSourcePlanCallableSyntaxV1 {
    pub(crate) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }

    pub(crate) const fn kind(&self) -> ParserNormalSourcePlanCallableKindV1 {
        self.kind
    }
}

#[derive(Debug)]
pub(crate) struct ParserBackedNormalSourcePlanBoundV1 {
    invocation: ParserInvocationWitnessV1,
    surface: ParserNormalSourcePlanSurfaceV1,
    callable_syntax: Box<[ParserNormalSourcePlanCallableSyntaxV1]>,
    _seal: ParserBackedNormalSourcePlanBoundSealV1,
}

#[derive(Debug)]
pub(in crate::parser) struct ParserBackedNormalSourcePlanBoundSealV1;

#[derive(Debug)]
pub(in crate::parser) enum ParserNormalSourcePlanSurfaceDispositionV1 {
    Ready(ParserBackedNormalSourcePlanBoundV1),
    SourceAuthorityUnavailable(ParserNormalSourcePlanSurfaceUnavailableV1),
    Incomplete(ParserNormalSourcePlanSurfaceIncompleteV1),
    IntegrityInvalid(ParserNormalSourcePlanSurfaceIntegrityIssueV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalSourcePlanSurfaceUnavailableV1 {
    PostpassNotSourceBacked,
    ParameterSourceUnavailable,
    SeedAlreadyConsumed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalSourcePlanSurfaceIncompleteV1 {
    ProgramMissing,
    SlotCoverageMismatch,
    SlotOutOfRange,
    CallableSourceMissing,
    StaticParentSourceMissing,
    OrdinaryParentSourceMissing,
    OrdinaryCallableCoverageMismatch,
    CallableSyntaxMissing,
    CallableArityOverflow,
    OrdinaryMemberCountOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalSourcePlanSurfaceIntegrityIssueV1 {
    ForeignParserRelation,
    DuplicateCallableSource,
    DuplicateCallableSyntax,
    CallableSyntaxRelationMismatch,
    DuplicateOrdinaryParentSource,
    OrdinaryParentSourceRelationMismatch,
    OrphanStaticParentSource,
    OrphanOrdinaryParentSource,
}

impl ParserBackedNormalSourcePlanBoundV1 {
    pub(in crate::parser) fn invocation(&self) -> &ParserInvocationWitnessV1 {
        &self.invocation
    }

    pub(in crate::parser) fn surface(&self) -> &ParserNormalSourcePlanSurfaceV1 {
        &self.surface
    }

    pub(in crate::parser) fn callable_syntax(
        &self,
        identity: &CallableDeclarationIdentityV1,
    ) -> Option<&ParserNormalSourcePlanCallableSyntaxV1> {
        self.callable_syntax
            .iter()
            .find(|row| row.identity.same_as(identity))
    }
}

pub(in crate::parser) struct ParserNormalSourcePlanSurfaceIssuerV1;

impl ParserNormalSourcePlanSurfaceIssuerV1 {
    pub(in crate::parser) fn issue_once(
        completed: &CompletedParserPostpassV1,
        parameter_source: &ParserCallableParameterSourceDispositionV1,
        seed: ParserNormalSourcePlanSeedDispositionV1,
    ) -> ParserNormalSourcePlanSurfaceDispositionV1 {
        if !completed.is_source_backed() {
            return ParserNormalSourcePlanSurfaceDispositionV1::SourceAuthorityUnavailable(
                ParserNormalSourcePlanSurfaceUnavailableV1::PostpassNotSourceBacked,
            );
        }
        let ParserCallableParameterSourceDispositionV1::Complete(catalog) = parameter_source else {
            return ParserNormalSourcePlanSurfaceDispositionV1::SourceAuthorityUnavailable(
                ParserNormalSourcePlanSurfaceUnavailableV1::ParameterSourceUnavailable,
            );
        };
        let ParserNormalSourcePlanSeedDispositionV1::Ready(seed) = seed else {
            return match seed {
                ParserNormalSourcePlanSeedDispositionV1::CompatibilityOutside => {
                    ParserNormalSourcePlanSurfaceDispositionV1::SourceAuthorityUnavailable(
                        ParserNormalSourcePlanSurfaceUnavailableV1::PostpassNotSourceBacked,
                    )
                }
                ParserNormalSourcePlanSeedDispositionV1::Consumed => {
                    ParserNormalSourcePlanSurfaceDispositionV1::SourceAuthorityUnavailable(
                        ParserNormalSourcePlanSurfaceUnavailableV1::SeedAlreadyConsumed,
                    )
                }
                ParserNormalSourcePlanSeedDispositionV1::Ready(_) => unreachable!(),
            };
        };
        let (slot_set, static_parent_sources) = seed.into_parts();
        let invocation = ParserInvocationWitnessV1::from_brand(slot_set.brand());
        if !catalog.same_parser_brand(slot_set.brand()) {
            return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                ParserNormalSourcePlanSurfaceIntegrityIssueV1::ForeignParserRelation,
            );
        }
        let slots = slot_set.into_rows();
        let ASTNode::Program { statements, .. } = completed.ast() else {
            return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                ParserNormalSourcePlanSurfaceIncompleteV1::ProgramMissing,
            );
        };
        if slots.len() != statements.len() {
            return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                ParserNormalSourcePlanSurfaceIncompleteV1::SlotCoverageMismatch,
            );
        }

        let mut static_parent_sources = static_parent_sources.into_vec();
        let ordinary_parent_count = completed
            .box_coverage()
            .rows()
            .iter()
            .filter_map(|row| row.source_sealed())
            .count();
        let mut consumed_ordinary_parent_count = 0usize;
        let mut rows = Vec::with_capacity(slots.len());
        let mut callable_syntax = Vec::new();
        for slot in slots {
            let Some(statement) = statements.get(slot.final_statement_slot() as usize) else {
                return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                    ParserNormalSourcePlanSurfaceIncompleteV1::SlotOutOfRange,
                );
            };
            let row = match statement {
                ASTNode::FunctionDeclaration { name, params, .. } => {
                    let callable_matches = completed
                        .callable_rows()
                        .iter()
                        .filter_map(PreparedCallableSourceV1::direct)
                        .filter(|source| {
                            matches!(
                                source.path(),
                                SourceProgramCallablePathV1::TopLevel { declaration }
                                    if *declaration == *slot.source_path()
                            )
                        })
                        .collect::<Vec<_>>();
                    let Some(source) = callable_matches.first() else {
                        return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                            ParserNormalSourcePlanSurfaceIncompleteV1::CallableSourceMissing,
                        );
                    };
                    if callable_matches.len() != 1 {
                        return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                            ParserNormalSourcePlanSurfaceIntegrityIssueV1::DuplicateCallableSource,
                        );
                    }
                    if !source.parser_brand().same_as(slot.source_path().brand()) {
                        return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                            ParserNormalSourcePlanSurfaceIntegrityIssueV1::ForeignParserRelation,
                        );
                    }
                    let arity = match u32::try_from(params.len()) {
                        Ok(arity) => arity,
                        Err(_) => {
                            return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                                ParserNormalSourcePlanSurfaceIncompleteV1::CallableArityOverflow,
                            )
                        }
                    };
                    let identity = source.anchor().identity();
                    if callable_syntax
                        .iter()
                        .any(|row: &ParserNormalSourcePlanCallableSyntaxV1| {
                            row.identity.same_as(&identity)
                        })
                    {
                        return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                            ParserNormalSourcePlanSurfaceIntegrityIssueV1::DuplicateCallableSyntax,
                        );
                    }
                    callable_syntax.push(ParserNormalSourcePlanCallableSyntaxV1 {
                        identity: identity.clone(),
                        diagnostic_name: name.as_str().into(),
                        arity,
                        kind: ParserNormalSourcePlanCallableKindV1::TopLevel,
                    });
                    ParserNormalSourcePlanTopLevelRowV1::TopLevelCallable {
                        slot,
                        callable_identity: identity,
                    }
                }
                ASTNode::BoxDeclaration { is_static, .. } if *is_static => {
                    let Some(index) = static_parent_sources.iter().position(|source| {
                        source.box_site().path() == slot.source_path().compatibility_box_path()
                    }) else {
                        return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                            ParserNormalSourcePlanSurfaceIncompleteV1::StaticParentSourceMissing,
                        );
                    };
                    let source = static_parent_sources.swap_remove(index);
                    for (method_site, method_identity) in source.direct_method_relations() {
                        let mut matches = catalog
                            .declarations()
                            .iter()
                            .filter(|row| row.callable_identity().same_as(method_identity));
                        let Some(method) = matches.next() else {
                            return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                                ParserNormalSourcePlanSurfaceIncompleteV1::CallableSyntaxMissing,
                            );
                        };
                        if matches.next().is_some() || method.source_site() != method_site {
                            return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                                ParserNormalSourcePlanSurfaceIntegrityIssueV1::CallableSyntaxRelationMismatch,
                            );
                        }
                        if callable_syntax.iter().any(
                            |row: &ParserNormalSourcePlanCallableSyntaxV1| {
                                row.identity.same_as(method_identity)
                            },
                        ) {
                            return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                                ParserNormalSourcePlanSurfaceIntegrityIssueV1::DuplicateCallableSyntax,
                            );
                        }
                        let arity = match u32::try_from(method.parameters().len()) {
                            Ok(arity) => arity,
                            Err(_) => {
                                return ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(
                                    ParserNormalSourcePlanSurfaceIncompleteV1::CallableArityOverflow,
                                )
                            }
                        };
                        callable_syntax.push(ParserNormalSourcePlanCallableSyntaxV1 {
                            identity: method_identity.clone(),
                            diagnostic_name: method.diagnostic_name().into(),
                            arity,
                            kind: match method.kind() {
                                ParserCallableDeclarationKindV1::StaticBoxMethod => {
                                    ParserNormalSourcePlanCallableKindV1::StaticBoxMethod
                                }
                                ParserCallableDeclarationKindV1::InstanceBoxMethod => {
                                    ParserNormalSourcePlanCallableKindV1::InstanceBoxMethod
                                }
                            },
                        });
                    }
                    ParserNormalSourcePlanTopLevelRowV1::StaticBox { slot, source }
                }
                ASTNode::BoxDeclaration { .. } => {
                    let source = match issue_ordinary_box_source(
                        completed,
                        catalog,
                        &slot,
                        &mut callable_syntax,
                    ) {
                        Ok(source) => source,
                        Err(issue) => return issue.into_disposition(),
                    };
                    consumed_ordinary_parent_count += 1;
                    ParserNormalSourcePlanTopLevelRowV1::OrdinaryBox { slot, source }
                }
                ASTNode::Program { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::NestedProgram)
                }
                ASTNode::BuildGate { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::BuildGate)
                }
                ASTNode::UsingStatement { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::Using)
                }
                ASTNode::ImportStatement { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::Import)
                }
                ASTNode::EnumDeclaration { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::Enum)
                }
                ASTNode::BrandDeclaration { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::Brand)
                }
                ASTNode::TypeAliasDeclaration { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::TypeAlias)
                }
                ASTNode::GlobalVar { .. } => {
                    unsupported(slot, ParserNormalSourcePlanUnsupportedKindV1::Global)
                }
                ASTNode::StaticConstTable { .. } => unsupported(
                    slot,
                    ParserNormalSourcePlanUnsupportedKindV1::StaticConstTable,
                ),
                _ => ParserNormalSourcePlanTopLevelRowV1::Executable { slot },
            };
            rows.push(row);
        }
        if !static_parent_sources.is_empty() {
            return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                ParserNormalSourcePlanSurfaceIntegrityIssueV1::OrphanStaticParentSource,
            );
        }
        if consumed_ordinary_parent_count != ordinary_parent_count {
            return ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(
                ParserNormalSourcePlanSurfaceIntegrityIssueV1::OrphanOrdinaryParentSource,
            );
        }
        let surface = match NonEmptyParserNormalSourcePlanRowsV1::issue(rows) {
            None => ParserNormalSourcePlanSurfaceV1::CompleteEmpty,
            Some(rows) => ParserNormalSourcePlanSurfaceV1::CompleteRows(rows),
        };
        ParserNormalSourcePlanSurfaceDispositionV1::Ready(ParserBackedNormalSourcePlanBoundV1 {
            invocation,
            surface,
            callable_syntax: callable_syntax.into_boxed_slice(),
            _seal: ParserBackedNormalSourcePlanBoundSealV1,
        })
    }
}

enum ParserNormalSourcePlanSurfaceIssueV1 {
    Incomplete(ParserNormalSourcePlanSurfaceIncompleteV1),
    IntegrityInvalid(ParserNormalSourcePlanSurfaceIntegrityIssueV1),
}

impl ParserNormalSourcePlanSurfaceIssueV1 {
    fn into_disposition(self) -> ParserNormalSourcePlanSurfaceDispositionV1 {
        match self {
            Self::Incomplete(error) => {
                ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(error)
            }
            Self::IntegrityInvalid(error) => {
                ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(error)
            }
        }
    }
}

fn issue_ordinary_box_source(
    completed: &CompletedParserPostpassV1,
    catalog: &super::catalog::ParserCallableParameterSourceCatalogV1,
    slot: &ProjectedProgramItemSlotV1,
    callable_syntax: &mut Vec<ParserNormalSourcePlanCallableSyntaxV1>,
) -> Result<ParserNormalSourcePlanOrdinaryBoxV1, ParserNormalSourcePlanSurfaceIssueV1> {
    let final_slot = usize::try_from(slot.final_statement_slot()).map_err(|_| {
        ParserNormalSourcePlanSurfaceIssueV1::Incomplete(
            ParserNormalSourcePlanSurfaceIncompleteV1::SlotOutOfRange,
        )
    })?;
    let mut matches = completed
        .box_coverage()
        .rows()
        .iter()
        .filter_map(|row| row.source_sealed())
        .filter(|(ordinal, _)| *ordinal == final_slot);
    let (_, seal) = matches
        .next()
        .ok_or(ParserNormalSourcePlanSurfaceIssueV1::Incomplete(
            ParserNormalSourcePlanSurfaceIncompleteV1::OrdinaryParentSourceMissing,
        ))?;
    if matches.next().is_some() {
        return Err(ParserNormalSourcePlanSurfaceIssueV1::IntegrityInvalid(
            ParserNormalSourcePlanSurfaceIntegrityIssueV1::DuplicateOrdinaryParentSource,
        ));
    }
    if seal.box_site().path() != slot.source_path().compatibility_box_path() {
        return Err(ParserNormalSourcePlanSurfaceIssueV1::IntegrityInvalid(
            ParserNormalSourcePlanSurfaceIntegrityIssueV1::OrdinaryParentSourceRelationMismatch,
        ));
    }
    let observed_member_count = u32::try_from(seal.method_relations().len()).map_err(|_| {
        ParserNormalSourcePlanSurfaceIssueV1::Incomplete(
            ParserNormalSourcePlanSurfaceIncompleteV1::OrdinaryMemberCountOverflow,
        )
    })?;

    let mut direct_methods = Vec::new();
    for relation in seal.method_relations() {
        let Some(source_site) = relation.source_site() else {
            continue;
        };
        let mut declarations = catalog
            .declarations()
            .iter()
            .filter(|row| row.source_site() == source_site);
        let declaration =
            declarations
                .next()
                .ok_or(ParserNormalSourcePlanSurfaceIssueV1::Incomplete(
                    ParserNormalSourcePlanSurfaceIncompleteV1::CallableSyntaxMissing,
                ))?;
        if declarations.next().is_some()
            || declaration.kind() != ParserCallableDeclarationKindV1::InstanceBoxMethod
        {
            return Err(ParserNormalSourcePlanSurfaceIssueV1::IntegrityInvalid(
                ParserNormalSourcePlanSurfaceIntegrityIssueV1::CallableSyntaxRelationMismatch,
            ));
        }
        let identity = declaration.callable_identity().clone();
        push_callable_syntax(
            callable_syntax,
            identity.clone(),
            declaration.diagnostic_name(),
            declaration.parameters().len(),
            ParserNormalSourcePlanCallableKindV1::InstanceBoxMethod,
        )?;
        direct_methods.push(ParserNormalSourcePlanBoxMethodRelationV1 {
            source_site: source_site.clone(),
            callable_identity: identity,
        });
    }
    let catalog_method_count = catalog
        .declarations()
        .iter()
        .filter(|row| row.source_site().box_site() == seal.box_site())
        .count();
    if catalog_method_count != direct_methods.len() {
        return Err(ParserNormalSourcePlanSurfaceIssueV1::Incomplete(
            ParserNormalSourcePlanSurfaceIncompleteV1::OrdinaryCallableCoverageMismatch,
        ));
    }

    Ok(ParserNormalSourcePlanOrdinaryBoxV1 {
        box_site: seal.box_site().clone(),
        diagnostic_name: seal.declaration_syntax().name().into(),
        is_sync: seal.declaration_syntax().is_sync(),
        observed_member_count,
        direct_methods: direct_methods.into_boxed_slice(),
    })
}

fn push_callable_syntax(
    callable_syntax: &mut Vec<ParserNormalSourcePlanCallableSyntaxV1>,
    identity: CallableDeclarationIdentityV1,
    diagnostic_name: &str,
    arity: usize,
    kind: ParserNormalSourcePlanCallableKindV1,
) -> Result<(), ParserNormalSourcePlanSurfaceIssueV1> {
    if callable_syntax
        .iter()
        .any(|row| row.identity.same_as(&identity))
    {
        return Err(ParserNormalSourcePlanSurfaceIssueV1::IntegrityInvalid(
            ParserNormalSourcePlanSurfaceIntegrityIssueV1::DuplicateCallableSyntax,
        ));
    }
    let arity = u32::try_from(arity).map_err(|_| {
        ParserNormalSourcePlanSurfaceIssueV1::Incomplete(
            ParserNormalSourcePlanSurfaceIncompleteV1::CallableArityOverflow,
        )
    })?;
    callable_syntax.push(ParserNormalSourcePlanCallableSyntaxV1 {
        identity,
        diagnostic_name: diagnostic_name.into(),
        arity,
        kind,
    });
    Ok(())
}

fn unsupported(
    slot: ProjectedProgramItemSlotV1,
    kind: ParserNormalSourcePlanUnsupportedKindV1,
) -> ParserNormalSourcePlanTopLevelRowV1 {
    ParserNormalSourcePlanTopLevelRowV1::Unsupported { slot, kind }
}
