//! Complete final-callable syntax loan with an exact parameter-source subset.
//!
//! This is parser transport, not semantic authority. Final callable anchors
//! own total membership; direct-method parameter rows are joined only as a
//! partial projection. Neither names nor catalog ordinals repair identity.

use crate::ast::ASTNode;

use super::super::callable_parameter_source::{
    ParserCallableParameterDeclarationSourceV1, ParserCallableParameterSourceDispositionV1,
};
use super::super::callable_source_anchor::{
    CallableDeclarationIdentityV1, DirectCallableDeclarationKindV1, PreparedCallableSourceV1,
};
use super::super::initial_callable_program_source::{declaration_at, InitialCallableFinalSlotV1};
use super::model::direct_source_matches_parameter_declaration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinalCallableDeclarationModeV1 {
    TopLevel,
    StaticBoxMethod,
    InstanceBoxMethod,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FinalCallableParameterSourceRefV1<'source> {
    ordinal: u32,
    name: &'source str,
    ordinary: bool,
}

#[derive(Debug)]
pub(crate) struct FinalCallableSemanticSyntaxRowRefV1<'source> {
    batch_slot: u32,
    identity: CallableDeclarationIdentityV1,
    final_slot: InitialCallableFinalSlotV1,
    mode: FinalCallableDeclarationModeV1,
    declaration: &'source ASTNode,
    owner_name: Option<&'source str>,
    parameters: Option<Box<[FinalCallableParameterSourceRefV1<'source>]>>,
}

#[derive(Debug)]
pub(crate) struct FinalCallableSemanticSyntaxLoanV1<'source> {
    rows: Box<[FinalCallableSemanticSyntaxRowRefV1<'source>]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinalCallableSemanticSyntaxLoanErrorV1 {
    CoverageMismatch,
    BatchSlotOverflow,
    DeclarationMissing,
    DuplicateParameterProjection,
}

impl FinalCallableSemanticSyntaxLoanV1<'_> {
    pub(crate) fn rows(&self) -> &[FinalCallableSemanticSyntaxRowRefV1<'_>] {
        &self.rows
    }
}

impl FinalCallableSemanticSyntaxRowRefV1<'_> {
    pub(crate) const fn batch_slot(&self) -> u32 {
        self.batch_slot
    }

    pub(crate) fn identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.identity
    }

    pub(crate) const fn final_slot(&self) -> InitialCallableFinalSlotV1 {
        self.final_slot
    }

    pub(crate) const fn mode(&self) -> FinalCallableDeclarationModeV1 {
        self.mode
    }

    pub(crate) const fn declaration(&self) -> &ASTNode {
        self.declaration
    }

    pub(crate) const fn owner_name(&self) -> Option<&str> {
        self.owner_name
    }

    pub(crate) fn parameters(&self) -> Option<&[FinalCallableParameterSourceRefV1<'_>]> {
        self.parameters.as_deref()
    }
}

impl FinalCallableParameterSourceRefV1<'_> {
    pub(crate) const fn ordinal(self) -> u32 {
        self.ordinal
    }

    pub(crate) const fn is_ordinary(self) -> bool {
        self.ordinary
    }

    pub(crate) const fn name(&self) -> &str {
        self.name
    }
}

pub(super) fn build_final_callable_semantic_syntax_loan_v1<'source>(
    ast: &'source ASTNode,
    sources: &'source [PreparedCallableSourceV1],
    slots: &[InitialCallableFinalSlotV1],
    parameter_source: &'source ParserCallableParameterSourceDispositionV1,
) -> Result<FinalCallableSemanticSyntaxLoanV1<'source>, FinalCallableSemanticSyntaxLoanErrorV1> {
    if sources.len() != slots.len() {
        return Err(FinalCallableSemanticSyntaxLoanErrorV1::CoverageMismatch);
    }
    let parameter_catalog = match parameter_source {
        ParserCallableParameterSourceDispositionV1::Complete(catalog) => Some(catalog),
        ParserCallableParameterSourceDispositionV1::SelectedBuildGateUnsupported => None,
    };
    let mut rows = Vec::with_capacity(sources.len());
    for (index, (source, slot)) in sources.iter().zip(slots.iter().copied()).enumerate() {
        let batch_slot = u32::try_from(index)
            .map_err(|_| FinalCallableSemanticSyntaxLoanErrorV1::BatchSlotOverflow)?;
        let declaration = declaration_at(ast, slot);
        if !matches!(declaration, ASTNode::FunctionDeclaration { .. }) {
            return Err(FinalCallableSemanticSyntaxLoanErrorV1::DeclarationMissing);
        }
        let projected = exact_parameter_projection(source, parameter_catalog)?;
        let parameters = projected.map(|declaration| {
            declaration
                .parameters()
                .iter()
                .map(|parameter| FinalCallableParameterSourceRefV1 {
                    ordinal: parameter.ordinal(),
                    name: parameter.name(),
                    ordinary: parameter.transfer().is_ordinary(),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice()
        });
        rows.push(FinalCallableSemanticSyntaxRowRefV1 {
            batch_slot,
            identity: source.anchor().identity(),
            final_slot: slot,
            mode: declaration_mode(ast, slot)?,
            declaration,
            owner_name: declaration_owner_name(ast, slot)?,
            parameters,
        });
    }
    Ok(FinalCallableSemanticSyntaxLoanV1 {
        rows: rows.into_boxed_slice(),
    })
}

fn declaration_owner_name(
    ast: &ASTNode,
    slot: InitialCallableFinalSlotV1,
) -> Result<Option<&str>, FinalCallableSemanticSyntaxLoanErrorV1> {
    let InitialCallableFinalSlotV1::BoxMethod { statement, .. } = slot else {
        return Ok(None);
    };
    let ASTNode::Program { statements, .. } = ast else {
        return Err(FinalCallableSemanticSyntaxLoanErrorV1::DeclarationMissing);
    };
    let Some(ASTNode::BoxDeclaration { name, .. }) = statements.get(statement as usize) else {
        return Err(FinalCallableSemanticSyntaxLoanErrorV1::DeclarationMissing);
    };
    Ok(Some(name))
}

fn exact_parameter_projection<'source>(
    source: &PreparedCallableSourceV1,
    catalog: Option<
        &'source super::super::callable_parameter_source::ParserCallableParameterSourceCatalogV1,
    >,
) -> Result<
    Option<&'source ParserCallableParameterDeclarationSourceV1>,
    FinalCallableSemanticSyntaxLoanErrorV1,
> {
    let Some(direct) = source.direct() else {
        return Ok(None);
    };
    let Some(catalog) = catalog else {
        return Ok(None);
    };
    let mut matches = catalog
        .declarations()
        .iter()
        .filter(|declaration| direct_source_matches_parameter_declaration(direct, declaration));
    let matched = matches.next();
    if matches.next().is_some() {
        return Err(FinalCallableSemanticSyntaxLoanErrorV1::DuplicateParameterProjection);
    }
    if matched.is_none()
        && matches!(
            direct.kind(),
            DirectCallableDeclarationKindV1::StaticBoxMethod
                | DirectCallableDeclarationKindV1::InstanceBoxMethod
        )
    {
        return Err(FinalCallableSemanticSyntaxLoanErrorV1::CoverageMismatch);
    }
    Ok(matched)
}

fn declaration_mode(
    ast: &ASTNode,
    slot: InitialCallableFinalSlotV1,
) -> Result<FinalCallableDeclarationModeV1, FinalCallableSemanticSyntaxLoanErrorV1> {
    match slot {
        InitialCallableFinalSlotV1::TopLevel { .. } => Ok(FinalCallableDeclarationModeV1::TopLevel),
        InitialCallableFinalSlotV1::BoxMethod { statement, .. } => {
            let ASTNode::Program { statements, .. } = ast else {
                return Err(FinalCallableSemanticSyntaxLoanErrorV1::DeclarationMissing);
            };
            let Some(ASTNode::BoxDeclaration { is_static, .. }) =
                statements.get(statement as usize)
            else {
                return Err(FinalCallableSemanticSyntaxLoanErrorV1::DeclarationMissing);
            };
            Ok(if *is_static {
                FinalCallableDeclarationModeV1::StaticBoxMethod
            } else {
                FinalCallableDeclarationModeV1::InstanceBoxMethod
            })
        }
    }
}
