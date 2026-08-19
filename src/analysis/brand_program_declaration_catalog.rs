//! AST-free effective Brand declaration catalog.
//!
//! Source adapters record already-pruned top-level declarations. This owner
//! alone rejects duplicate effective names and seals deterministic lookup rows.

use std::collections::BTreeMap;

use crate::ast::ASTNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct BrandDeclarationSourceIdV1(u32);

impl BrandDeclarationSourceIdV1 {
    pub(crate) fn from_program_item_ordinal(ordinal: usize) -> Result<Self, BrandCatalogIssueV1> {
        u32::try_from(ordinal)
            .map(Self)
            .map_err(|_| BrandCatalogIssueV1::ProgramItemOrdinalOverflow)
    }

    pub(crate) const fn program_item_ordinal(self) -> u32 {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedBrandProgramDeclarationV1 {
    source: BrandDeclarationSourceIdV1,
    name: Box<str>,
    underlying_type: Box<str>,
}

impl VerifiedBrandProgramDeclarationV1 {
    pub(crate) const fn source(&self) -> BrandDeclarationSourceIdV1 {
        self.source
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn underlying_type(&self) -> &str {
        &self.underlying_type
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BrandCatalogIssueV1 {
    ProgramItemOrdinalOverflow,
    DeclarationOutsideProgram,
    DuplicateDeclaration {
        name: Box<str>,
        first: BrandDeclarationSourceIdV1,
        duplicate: BrandDeclarationSourceIdV1,
    },
}

impl std::fmt::Display for BrandCatalogIssueV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProgramItemOrdinalOverflow => write!(
                formatter,
                "[freeze:contract][brand/program-item-ordinal-overflow]"
            ),
            Self::DeclarationOutsideProgram => write!(
                formatter,
                "[freeze:contract][brand/declaration-outside-program]"
            ),
            Self::DuplicateDeclaration {
                name,
                first,
                duplicate,
            } => write!(
                formatter,
                "[freeze:contract][brand/duplicate-declaration] name={name} first={} duplicate={}",
                first.program_item_ordinal(),
                duplicate.program_item_ordinal()
            ),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct BrandProgramDeclarationCatalogDraftV1 {
    rows: Vec<VerifiedBrandProgramDeclarationV1>,
    by_name: BTreeMap<Box<str>, usize>,
}

impl BrandProgramDeclarationCatalogDraftV1 {
    pub(crate) fn record_effective_declaration(
        &mut self,
        source: BrandDeclarationSourceIdV1,
        name: &str,
        underlying_type: &str,
    ) -> Result<(), BrandCatalogIssueV1> {
        if let Some(first_ordinal) = self.by_name.get(name) {
            return Err(BrandCatalogIssueV1::DuplicateDeclaration {
                name: name.into(),
                first: self.rows[*first_ordinal].source,
                duplicate: source,
            });
        }
        let row_ordinal = self.rows.len();
        let name: Box<str> = name.into();
        self.rows.push(VerifiedBrandProgramDeclarationV1 {
            source,
            name: name.clone(),
            underlying_type: underlying_type.into(),
        });
        self.by_name.insert(name, row_ordinal);
        Ok(())
    }

    pub(crate) fn seal(self) -> VerifiedBrandProgramDeclarationCatalogV1 {
        VerifiedBrandProgramDeclarationCatalogV1 {
            rows: self.rows.into_boxed_slice(),
            by_name: self.by_name,
            _seal: VerifiedBrandProgramDeclarationCatalogSealV1,
        }
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedBrandProgramDeclarationCatalogV1 {
    rows: Box<[VerifiedBrandProgramDeclarationV1]>,
    by_name: BTreeMap<Box<str>, usize>,
    _seal: VerifiedBrandProgramDeclarationCatalogSealV1,
}

#[derive(Debug, Default)]
struct VerifiedBrandProgramDeclarationCatalogSealV1;

impl VerifiedBrandProgramDeclarationCatalogV1 {
    pub(crate) fn rows(&self) -> &[VerifiedBrandProgramDeclarationV1] {
        &self.rows
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub(crate) fn declaration(&self, name: &str) -> Option<&VerifiedBrandProgramDeclarationV1> {
        self.by_name
            .get(name)
            .and_then(|ordinal| self.rows.get(*ordinal))
    }

    pub(crate) fn contains_name(&self, name: &str) -> bool {
        self.declaration(name).is_some()
    }

    pub(crate) fn underlying_type(&self, name: &str) -> Option<&str> {
        self.declaration(name)
            .map(VerifiedBrandProgramDeclarationV1::underlying_type)
    }

    pub(crate) fn into_rows(self) -> Box<[VerifiedBrandProgramDeclarationV1]> {
        self.rows
    }
}

pub(crate) fn issue_brand_program_declaration_catalog_v1(
    root: &ASTNode,
) -> Result<VerifiedBrandProgramDeclarationCatalogV1, BrandCatalogIssueV1> {
    let mut draft = BrandProgramDeclarationCatalogDraftV1::default();
    let ASTNode::Program { statements, .. } = root else {
        return Ok(draft.seal());
    };
    for (ordinal, statement) in statements.iter().enumerate() {
        let ASTNode::BrandDeclaration {
            name,
            underlying_type_name,
            ..
        } = statement
        else {
            continue;
        };
        draft.record_effective_declaration(
            BrandDeclarationSourceIdV1::from_program_item_ordinal(ordinal)?,
            name,
            underlying_type_name,
        )?;
    }
    Ok(draft.seal())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_preserves_order_identity_and_lookup() {
        let mut draft = BrandProgramDeclarationCatalogDraftV1::default();
        draft
            .record_effective_declaration(
                BrandDeclarationSourceIdV1::from_program_item_ordinal(3).unwrap(),
                "PageId",
                "i64",
            )
            .unwrap();
        draft
            .record_effective_declaration(
                BrandDeclarationSourceIdV1::from_program_item_ordinal(8).unwrap(),
                "BlockId",
                "u64",
            )
            .unwrap();
        let catalog = draft.seal();

        assert_eq!(catalog.rows()[0].source().program_item_ordinal(), 3);
        assert_eq!(catalog.rows()[1].name(), "BlockId");
        assert_eq!(catalog.underlying_type("PageId"), Some("i64"));
    }

    #[test]
    fn duplicate_rejects_without_publishing_a_winner() {
        let mut draft = BrandProgramDeclarationCatalogDraftV1::default();
        draft
            .record_effective_declaration(
                BrandDeclarationSourceIdV1::from_program_item_ordinal(1).unwrap(),
                "PageId",
                "i64",
            )
            .unwrap();
        let error = draft
            .record_effective_declaration(
                BrandDeclarationSourceIdV1::from_program_item_ordinal(4).unwrap(),
                "PageId",
                "u64",
            )
            .unwrap_err();

        assert!(error.to_string().contains("[brand/duplicate-declaration]"));
        assert!(error.to_string().contains("first=1 duplicate=4"));
    }
}
