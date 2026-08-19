//! Parser-owned constructor occurrence transport for normal callable source.
//!
//! Final AST ordinals and constructor keys validate placement only. The opaque
//! source id is downstream identity; consumers never reconstruct it from a
//! Box name, key, arity, or Builder work item.

use std::collections::BTreeSet;

use crate::ast::ASTNode;

use super::source_authority::{ConstructorSourceRelationV1, ParserInvocationBrandV1};
use super::source_seal::ParserBoxSourceSealV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstructorSourceIdV1 {
    parser_brand: ParserInvocationBrandV1,
    catalog_ordinal: u32,
}

impl ConstructorSourceIdV1 {
    pub(crate) fn same_as(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParserConstructorSourceRowV1 {
    source_id: ConstructorSourceIdV1,
    final_box_ordinal: u32,
    relation: ConstructorSourceRelationV1,
}

/// Non-Clone parser source catalog carried through final callable source.
#[derive(Debug)]
pub(crate) struct ParserConstructorSourceCatalogV1 {
    rows: Box<[ParserConstructorSourceRowV1]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConstructorSourceCatalogIssueErrorV1 {
    CoverageMismatch,
    FinalBoxMissing,
    DuplicateFinalBox,
    DuplicateConstructor,
    OrdinalOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinalConstructorSemanticSyntaxLoanErrorV1 {
    ProgramMissing,
    FinalBoxMissing,
    ConstructorMissing,
    ConstructorChanged,
    DuplicateSourceId,
}

#[derive(Debug)]
pub(crate) struct FinalConstructorSemanticSyntaxRowRefV1<'source> {
    source_id: &'source ConstructorSourceIdV1,
    box_name: &'source str,
    key: &'source str,
    declaration: &'source ASTNode,
}

#[derive(Debug)]
pub(crate) struct FinalConstructorSemanticSyntaxLoanV1<'source> {
    rows: Box<[FinalConstructorSemanticSyntaxRowRefV1<'source>]>,
}

impl FinalConstructorSemanticSyntaxLoanV1<'_> {
    pub(crate) fn rows(&self) -> &[FinalConstructorSemanticSyntaxRowRefV1<'_>] {
        &self.rows
    }
}

impl FinalConstructorSemanticSyntaxRowRefV1<'_> {
    pub(crate) fn source_id(&self) -> &ConstructorSourceIdV1 {
        self.source_id
    }

    pub(crate) fn box_name(&self) -> &str {
        self.box_name
    }

    pub(crate) fn key(&self) -> &str {
        self.key
    }

    pub(crate) fn declaration(&self) -> &ASTNode {
        self.declaration
    }
}

impl ParserConstructorSourceCatalogV1 {
    pub(super) fn issue(
        ast: &ASTNode,
        seals: &[ParserBoxSourceSealV1],
        final_box_ordinals: &[usize],
    ) -> Result<Self, ConstructorSourceCatalogIssueErrorV1> {
        if seals.len() != final_box_ordinals.len() {
            return Err(ConstructorSourceCatalogIssueErrorV1::CoverageMismatch);
        }
        let statements =
            program_statements(ast).ok_or(ConstructorSourceCatalogIssueErrorV1::FinalBoxMissing)?;
        let mut final_boxes = BTreeSet::new();
        let mut rows = Vec::new();
        for (seal, final_box_ordinal) in seals.iter().zip(final_box_ordinals.iter().copied()) {
            if !final_boxes.insert(final_box_ordinal) {
                return Err(ConstructorSourceCatalogIssueErrorV1::DuplicateFinalBox);
            }
            let Some(ASTNode::BoxDeclaration { constructors, .. }) =
                statements.get(final_box_ordinal)
            else {
                return Err(ConstructorSourceCatalogIssueErrorV1::FinalBoxMissing);
            };
            if constructors.len() != seal.constructor_relations().len() {
                return Err(ConstructorSourceCatalogIssueErrorV1::CoverageMismatch);
            }
            for relation in seal.constructor_relations() {
                if !constructors.contains_key(relation.key()) {
                    return Err(ConstructorSourceCatalogIssueErrorV1::CoverageMismatch);
                }
                let catalog_ordinal = u32::try_from(rows.len())
                    .map_err(|_| ConstructorSourceCatalogIssueErrorV1::OrdinalOverflow)?;
                if rows.iter().any(|row: &ParserConstructorSourceRowV1| {
                    row.final_box_ordinal == final_box_ordinal as u32
                        && row.relation.key() == relation.key()
                }) {
                    return Err(ConstructorSourceCatalogIssueErrorV1::DuplicateConstructor);
                }
                rows.push(ParserConstructorSourceRowV1 {
                    source_id: ConstructorSourceIdV1 {
                        parser_brand: seal.box_site().path().brand().clone(),
                        catalog_ordinal,
                    },
                    final_box_ordinal: u32::try_from(final_box_ordinal)
                        .map_err(|_| ConstructorSourceCatalogIssueErrorV1::OrdinalOverflow)?,
                    relation: relation.clone(),
                });
            }
        }
        Ok(Self {
            rows: rows.into_boxed_slice(),
        })
    }

    pub(super) fn validate_transform(
        &self,
        initial: &ASTNode,
        transformed: &ASTNode,
    ) -> Result<(), FinalConstructorSemanticSyntaxLoanErrorV1> {
        for row in &self.rows {
            if constructor_at(initial, row)? != constructor_at(transformed, row)? {
                return Err(FinalConstructorSemanticSyntaxLoanErrorV1::ConstructorChanged);
            }
        }
        Ok(())
    }

    pub(super) fn syntax_loan<'source>(
        &'source self,
        ast: &'source ASTNode,
    ) -> Result<
        FinalConstructorSemanticSyntaxLoanV1<'source>,
        FinalConstructorSemanticSyntaxLoanErrorV1,
    > {
        let statements = program_statements(ast)
            .ok_or(FinalConstructorSemanticSyntaxLoanErrorV1::ProgramMissing)?;
        let mut source_ids = Vec::new();
        let mut rows = Vec::with_capacity(self.rows.len());
        for row in &self.rows {
            if source_ids
                .iter()
                .any(|source_id| source_id == &row.source_id)
            {
                return Err(FinalConstructorSemanticSyntaxLoanErrorV1::DuplicateSourceId);
            }
            source_ids.push(row.source_id.clone());
            let Some(ASTNode::BoxDeclaration {
                name, constructors, ..
            }) = statements.get(row.final_box_ordinal as usize)
            else {
                return Err(FinalConstructorSemanticSyntaxLoanErrorV1::FinalBoxMissing);
            };
            let declaration = constructors
                .get(row.relation.key())
                .ok_or(FinalConstructorSemanticSyntaxLoanErrorV1::ConstructorMissing)?;
            rows.push(FinalConstructorSemanticSyntaxRowRefV1 {
                source_id: &row.source_id,
                box_name: name,
                key: row.relation.key(),
                declaration,
            });
        }
        Ok(FinalConstructorSemanticSyntaxLoanV1 {
            rows: rows.into_boxed_slice(),
        })
    }
}

fn constructor_at<'source>(
    ast: &'source ASTNode,
    row: &ParserConstructorSourceRowV1,
) -> Result<&'source ASTNode, FinalConstructorSemanticSyntaxLoanErrorV1> {
    let statements =
        program_statements(ast).ok_or(FinalConstructorSemanticSyntaxLoanErrorV1::ProgramMissing)?;
    let Some(ASTNode::BoxDeclaration { constructors, .. }) =
        statements.get(row.final_box_ordinal as usize)
    else {
        return Err(FinalConstructorSemanticSyntaxLoanErrorV1::FinalBoxMissing);
    };
    constructors
        .get(row.relation.key())
        .ok_or(FinalConstructorSemanticSyntaxLoanErrorV1::ConstructorMissing)
}

fn program_statements(ast: &ASTNode) -> Option<&[ASTNode]> {
    let ASTNode::Program { statements, .. } = ast else {
        return None;
    };
    Some(statements)
}
