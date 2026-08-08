use std::fmt;

use crate::ASTNode;

use super::{BoxMethodEntryV1, BoxMethodInventoryErrorV1, BoxMethodInventoryV1};

#[derive(Debug)]
pub enum BoxMethodDeclarationTransformErrorV1<E> {
    Transform(E),
    InvalidInventory(BoxMethodInventoryErrorV1),
}

impl<E: fmt::Display> fmt::Display for BoxMethodDeclarationTransformErrorV1<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transform(error) => write!(formatter, "Box method transform failed: {error}"),
            Self::InvalidInventory(error) => {
                write!(
                    formatter,
                    "Box method transform produced invalid inventory: {error}"
                )
            }
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for BoxMethodDeclarationTransformErrorV1<E> {}

impl BoxMethodInventoryV1 {
    /// Transforms only method declarations while preserving every inventory
    /// authority field exactly. The operation consumes the unpublished
    /// inventory, so a failure cannot expose a partially transformed carrier.
    pub fn try_map_declarations_preserving_metadata<E, F>(
        self,
        mut transform: F,
    ) -> Result<Self, BoxMethodDeclarationTransformErrorV1<E>>
    where
        F: FnMut(ASTNode) -> Result<ASTNode, E>,
    {
        let mut entries = Vec::with_capacity(self.entries.len());
        for entry in self.entries {
            let BoxMethodEntryV1 {
                name,
                declaration,
                provenance,
                site,
                diagnostic_span,
            } = entry;
            let transformed =
                transform(declaration).map_err(BoxMethodDeclarationTransformErrorV1::Transform)?;
            Self::validate_declaration_name(&name, &transformed)
                .map_err(BoxMethodDeclarationTransformErrorV1::InvalidInventory)?;
            entries.push(BoxMethodEntryV1 {
                name,
                declaration: transformed,
                provenance,
                site,
                diagnostic_span,
            });
        }
        Ok(Self {
            entries,
            lookup: self.lookup,
        })
    }
}
