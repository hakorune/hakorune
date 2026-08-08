use crate::{ASTNode, Span};

use super::{BoxMethodGeneratedProvenanceV1, BoxMethodInventoryErrorV1, BoxMethodInventoryV1};

#[derive(Debug, Clone, PartialEq)]
pub struct PreparedGeneratedBoxMethodV1 {
    pub(super) name: Box<str>,
    pub(super) declaration: ASTNode,
    pub(super) provenance: BoxMethodGeneratedProvenanceV1,
    pub(super) diagnostic_span: Span,
}

impl PreparedGeneratedBoxMethodV1 {
    pub fn new(
        name: impl Into<Box<str>>,
        declaration: ASTNode,
        provenance: BoxMethodGeneratedProvenanceV1,
        diagnostic_span: Span,
    ) -> Result<Self, BoxMethodInventoryErrorV1> {
        let name = name.into();
        BoxMethodInventoryV1::validate_declaration_name(&name, &declaration)?;
        Ok(Self {
            name,
            declaration,
            provenance,
            diagnostic_span,
        })
    }
}

/// A complete unpublished generated-method transaction.
///
/// Construction rejects duplicate names inside the batch. Committing the
/// batch separately preflights every collision and ordinal before mutating the
/// destination inventory.
#[derive(Debug, Clone, PartialEq)]
pub struct PreparedGeneratedBoxMethodBatchV1 {
    pub(super) rows: Box<[PreparedGeneratedBoxMethodV1]>,
}

impl PreparedGeneratedBoxMethodBatchV1 {
    pub fn try_new(
        rows: impl IntoIterator<Item = PreparedGeneratedBoxMethodV1>,
    ) -> Result<Self, BoxMethodInventoryErrorV1> {
        let rows = rows.into_iter().collect::<Vec<_>>();
        let mut names = std::collections::HashMap::<&str, Span>::new();
        for row in &rows {
            if let Some(first_span) = names.insert(row.name.as_ref(), row.diagnostic_span) {
                return Err(BoxMethodInventoryErrorV1::DuplicateMethod {
                    name: row.name.clone(),
                    first_span,
                    duplicate_span: row.diagnostic_span,
                });
            }
        }
        Ok(Self {
            rows: rows.into_boxed_slice(),
        })
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn names_in_order(&self) -> impl ExactSizeIterator<Item = &str> {
        self.rows.iter().map(|row| row.name.as_ref())
    }
}
