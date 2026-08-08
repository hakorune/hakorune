use std::collections::HashMap;

use super::{BoxMethodEntryV1, BoxMethodInventoryErrorV1, BoxMethodInventoryV1};

/// A complete, unpublished append prepared by the parser source transaction.
///
/// The AST carrier validates declaration/name identity and duplicate rows. It
/// deliberately does not know parser brands, source sites, or gate-path
/// ownership. The destination inventory assigns selected placement ordinals
/// only when this append is committed.
#[derive(Debug, Clone, PartialEq)]
pub struct PreparedBoxMethodInventoryAppendV1 {
    pub(super) entries: Box<[BoxMethodEntryV1]>,
}

impl PreparedBoxMethodInventoryAppendV1 {
    pub fn try_new(
        entries: impl IntoIterator<Item = BoxMethodEntryV1>,
    ) -> Result<Self, BoxMethodInventoryErrorV1> {
        let entries = entries.into_iter().collect::<Vec<_>>();
        let mut names = HashMap::<&str, _>::new();
        for entry in &entries {
            BoxMethodInventoryV1::validate_declaration_name(entry.name(), entry.declaration())?;
            entry.provenance().validate_transport()?;
            if let Some(first_span) = names.insert(entry.name(), entry.diagnostic_span()) {
                return Err(BoxMethodInventoryErrorV1::DuplicateMethod {
                    name: entry.name().into(),
                    first_span,
                    duplicate_span: entry.diagnostic_span(),
                });
            }
        }
        Ok(Self {
            entries: entries.into_boxed_slice(),
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(super) fn into_entries(self) -> Box<[BoxMethodEntryV1]> {
        self.entries
    }
}
