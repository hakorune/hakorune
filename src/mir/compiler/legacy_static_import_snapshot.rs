use std::collections::BTreeMap;

use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::source_call_target::{
    StaticImportAliasViewErrorV1, VerifiedStaticImportAliasViewV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CompilerSuppliedStaticImportSnapshotErrorV1 {
    EmptyAlias,
    EmptyCanonicalOwner { alias: Box<str> },
    DuplicateAlias { alias: Box<str> },
}

/// One compiler-supplied, immutable import authority.
///
/// `None` and an explicitly supplied empty table remain distinct. The
/// snapshot is deliberately non-Clone and owns no Builder installation API in
/// the disconnected request row.
#[derive(Debug)]
pub(super) enum CompilerSuppliedStaticImportSnapshotV1 {
    None,
    Explicit(BTreeMap<Box<str>, Box<str>>),
}

impl CompilerSuppliedStaticImportSnapshotV1 {
    pub(super) const fn none() -> Self {
        Self::None
    }

    pub(super) fn explicit(
        rows: impl IntoIterator<Item = (String, String)>,
    ) -> Result<Self, CompilerSuppliedStaticImportSnapshotErrorV1> {
        let mut imports = BTreeMap::new();
        for (alias, canonical_owner) in rows {
            if alias.is_empty() {
                return Err(CompilerSuppliedStaticImportSnapshotErrorV1::EmptyAlias);
            }
            if canonical_owner.is_empty() {
                return Err(
                    CompilerSuppliedStaticImportSnapshotErrorV1::EmptyCanonicalOwner {
                        alias: alias.into(),
                    },
                );
            }
            if imports
                .insert(
                    alias.clone().into_boxed_str(),
                    canonical_owner.into_boxed_str(),
                )
                .is_some()
            {
                return Err(
                    CompilerSuppliedStaticImportSnapshotErrorV1::DuplicateAlias {
                        alias: alias.into(),
                    },
                );
            }
        }
        Ok(Self::Explicit(imports))
    }

    pub(super) const fn is_explicit(&self) -> bool {
        matches!(self, Self::Explicit(_))
    }

    pub(super) fn len(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Explicit(imports) => imports.len(),
        }
    }

    pub(super) fn entries(&self) -> Box<dyn Iterator<Item = (&str, &str)> + '_> {
        match self {
            Self::None => Box::new(std::iter::empty()),
            Self::Explicit(imports) => Box::new(
                imports
                    .iter()
                    .map(|(alias, owner)| (alias.as_ref(), owner.as_ref())),
            ),
        }
    }

    pub(super) fn verify_alias_view<'catalog>(
        &self,
        declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> Result<VerifiedStaticImportAliasViewV1<'catalog>, StaticImportAliasViewErrorV1> {
        VerifiedStaticImportAliasViewV1::seal(
            declarations,
            self.entries()
                .map(|(alias, owner)| (alias.to_owned(), owner.to_owned())),
        )
    }

    pub(super) fn discard(self) {}
}

#[cfg(test)]
mod tests {
    use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
    use crate::parser::NyashParser;

    use super::{
        CompilerSuppliedStaticImportSnapshotErrorV1, CompilerSuppliedStaticImportSnapshotV1,
    };

    fn catalog() -> VerifiedSameModuleCallableDeclarationCatalogV1 {
        let source = r#"
static box Alpha { run() { return 1 } }
static box Beta { run() { return 2 } }
"#;
        let ast = NyashParser::parse_from_string(source).expect("snapshot fixture");
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&ast)
            .expect("snapshot catalog")
    }

    #[test]
    fn none_and_explicit_empty_remain_distinct() {
        let none = CompilerSuppliedStaticImportSnapshotV1::none();
        let explicit =
            CompilerSuppliedStaticImportSnapshotV1::explicit(std::iter::empty()).unwrap();
        assert!(!none.is_explicit());
        assert!(explicit.is_explicit());
        assert_eq!(none.len(), 0);
        assert_eq!(explicit.len(), 0);
    }

    #[test]
    fn explicit_snapshot_is_sorted_and_seals_one_borrowed_alias_view() {
        let snapshot = CompilerSuppliedStaticImportSnapshotV1::explicit([
            ("z".to_owned(), "Beta".to_owned()),
            ("a".to_owned(), "Alpha".to_owned()),
        ])
        .unwrap();
        assert_eq!(
            snapshot.entries().collect::<Vec<_>>(),
            vec![("a", "Alpha"), ("z", "Beta")]
        );
        let declarations = catalog();
        let view = snapshot.verify_alias_view(&declarations).unwrap();
        assert!(view.is_branded_by(&declarations));
        assert_eq!(view.canonical_owner("a"), Some("Alpha"));
        assert_eq!(view.canonical_owner("z"), Some("Beta"));
    }

    #[test]
    fn malformed_or_duplicate_rows_never_form_a_snapshot() {
        assert_eq!(
            CompilerSuppliedStaticImportSnapshotV1::explicit([(String::new(), "Alpha".to_owned())])
                .unwrap_err(),
            CompilerSuppliedStaticImportSnapshotErrorV1::EmptyAlias
        );
        assert_eq!(
            CompilerSuppliedStaticImportSnapshotV1::explicit([("a".to_owned(), String::new())])
                .unwrap_err(),
            CompilerSuppliedStaticImportSnapshotErrorV1::EmptyCanonicalOwner { alias: "a".into() }
        );
        assert_eq!(
            CompilerSuppliedStaticImportSnapshotV1::explicit([
                ("a".to_owned(), "Alpha".to_owned()),
                ("a".to_owned(), "Beta".to_owned()),
            ])
            .unwrap_err(),
            CompilerSuppliedStaticImportSnapshotErrorV1::DuplicateAlias { alias: "a".into() }
        );
    }
}
