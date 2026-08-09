//! Exact selected-normal callable occurrences discovered by the live catalog scan.
//!
//! This inventory owns source identity only. It does not own bodies, semantic
//! owners, MIR symbols, or lowering policy. Program work planning borrows it
//! instead of independently issuing top-level declaration identities.

use super::{CanonicalSameModuleCallableKeyV1, SameModuleCallableCatalogBrandV1};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir) struct SelectedTopLevelFunctionKeyV1 {
    statement_index: usize,
    declared_name: Box<str>,
    declared_arity: usize,
}

impl SelectedTopLevelFunctionKeyV1 {
    pub(super) fn new(statement_index: usize, declared_name: &str, declared_arity: usize) -> Self {
        Self {
            statement_index,
            declared_name: declared_name.into(),
            declared_arity,
        }
    }

    pub(in crate::mir::builder) const fn statement_index(&self) -> usize {
        self.statement_index
    }

    pub(in crate::mir::builder) fn declared_name(&self) -> &str {
        &self.declared_name
    }

    pub(in crate::mir::builder) const fn declared_arity(&self) -> usize {
        self.declared_arity
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir) enum SelectedNormalCallableKeyV1 {
    TopLevel(SelectedTopLevelFunctionKeyV1),
    Cataloged(CanonicalSameModuleCallableKeyV1),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum SelectedNormalCallableSourceSiteV1 {
    ProgramFunction {
        statement_index: usize,
    },
    ProgramBoxMethod {
        statement_index: usize,
        method_key: Box<str>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SelectedNormalCallableSourceRowV1 {
    key: SelectedNormalCallableKeyV1,
    site: SelectedNormalCallableSourceSiteV1,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum SelectedCallableSemanticBlockerV1 {
    NonPlainInstanceBox { statement_index: usize },
}

#[derive(Debug)]
pub(crate) struct VerifiedSelectedNormalCallableSourceInventoryV1 {
    brand: SameModuleCallableCatalogBrandV1,
    rows: Box<[SelectedNormalCallableSourceRowV1]>,
    blockers: Box<[SelectedCallableSemanticBlockerV1]>,
}

impl VerifiedSelectedNormalCallableSourceInventoryV1 {
    pub(super) fn seal(
        brand: SameModuleCallableCatalogBrandV1,
        rows: Vec<(
            SelectedNormalCallableKeyV1,
            SelectedNormalCallableSourceSiteV1,
        )>,
        blockers: Vec<SelectedCallableSemanticBlockerV1>,
    ) -> Self {
        let mut rows = rows
            .into_iter()
            .map(|(key, site)| SelectedNormalCallableSourceRowV1 { key, site })
            .collect::<Vec<_>>();
        rows.sort_by(|left, right| left.key.cmp(&right.key));
        Self {
            brand,
            rows: rows.into_boxed_slice(),
            blockers: blockers.into_boxed_slice(),
        }
    }

    pub(in crate::mir) const fn brand(&self) -> &SameModuleCallableCatalogBrandV1 {
        &self.brand
    }

    pub(in crate::mir::builder) fn top_level_function(
        &self,
        statement_index: usize,
    ) -> Option<&SelectedTopLevelFunctionKeyV1> {
        self.rows.iter().find_map(|row| match &row.key {
            SelectedNormalCallableKeyV1::TopLevel(key)
                if key.statement_index() == statement_index =>
            {
                Some(key)
            }
            _ => None,
        })
    }

    pub(crate) fn site(
        &self,
        key: &SelectedNormalCallableKeyV1,
    ) -> Option<&SelectedNormalCallableSourceSiteV1> {
        self.rows
            .binary_search_by(|row| row.key.cmp(key))
            .ok()
            .map(|index| &self.rows[index].site)
    }

    pub(crate) fn len(&self) -> usize {
        self.rows.len()
    }

    pub(in crate::mir::builder) fn blockers(&self) -> &[SelectedCallableSemanticBlockerV1] {
        &self.blockers
    }

    pub(in crate::mir::builder) fn entries(
        &self,
    ) -> impl Iterator<
        Item = (
            &SelectedNormalCallableKeyV1,
            &SelectedNormalCallableSourceSiteV1,
        ),
    > {
        self.rows.iter().map(|row| (&row.key, &row.site))
    }
}

#[cfg(test)]
mod tests {
    use crate::mir::builder::callable_declaration_catalog::{
        CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
        VerifiedSameModuleCallableDeclarationCatalogV1,
    };
    use crate::mir::builder::module_draft_collector::FunctionDraftKeyV1;
    use crate::mir::builder::normal_top_level_function_admission::NormalTopLevelFunctionDraftAdmissionV1;
    use crate::parser::NyashParser;

    use super::{SelectedNormalCallableKeyV1, SelectedNormalCallableSourceSiteV1};

    #[test]
    fn one_catalog_scan_owns_top_level_and_box_method_occurrences() {
        let root = NyashParser::parse_from_string(
            "function helper(x) { return x }\n\
             static box Tools { add(x) { return x } }\n\
             box Page { show(x) { return x } }",
        )
        .expect("mixed callable source");
        let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
            .expect("selected callable catalog");
        let inventory = catalog.selected_source_inventory();

        assert_eq!(catalog.len(), 2);
        assert_eq!(inventory.len(), 3);
        let top_level = inventory.top_level_function(0).expect("top-level row");
        assert_eq!(top_level.declared_name(), "helper");
        assert_eq!(top_level.declared_arity(), 1);
        assert_eq!(
            inventory.site(&SelectedNormalCallableKeyV1::TopLevel(top_level.clone())),
            Some(&SelectedNormalCallableSourceSiteV1::ProgramFunction { statement_index: 0 })
        );

        let static_key = catalog
            .declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                "Tools",
                "add",
                1,
            )
            .expect("static row")
            .key()
            .clone();
        assert_eq!(
            inventory.site(&SelectedNormalCallableKeyV1::Cataloged(static_key)),
            Some(&SelectedNormalCallableSourceSiteV1::ProgramBoxMethod {
                statement_index: 1,
                method_key: "add".into(),
            })
        );

        let instance_key = catalog
            .declaration_for(
                SameModuleCallableNamespaceV1::InstanceBoxMethod,
                "Page",
                "show",
                1,
            )
            .expect("instance row")
            .key()
            .clone();
        assert_eq!(
            instance_key,
            CanonicalSameModuleCallableKeyV1::instance_box_method("Page", "show", 1)
        );
        assert_eq!(
            inventory.site(&SelectedNormalCallableKeyV1::Cataloged(instance_key)),
            Some(&SelectedNormalCallableSourceSiteV1::ProgramBoxMethod {
                statement_index: 2,
                method_key: "show".into(),
            })
        );
    }

    #[test]
    fn non_program_catalog_does_not_claim_program_occurrences() {
        let root = NyashParser::parse_from_string("static box Tools { add(x) { return x } }")
            .expect("Box source");
        let crate::ast::ASTNode::Program { mut statements, .. } = root else {
            panic!("parser root")
        };
        let catalog =
            VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&statements.remove(0))
                .expect("root catalog");
        assert_eq!(catalog.len(), 1);
        assert_eq!(catalog.selected_source_inventory().len(), 0);
    }

    #[test]
    fn static_main_methods_remain_transferred_to_main_expansion() {
        let root = NyashParser::parse_from_string(
            "static box Main { main() { return 0 } helper() { return 1 } }\n\
             static box Tools { helper() { return 2 } }",
        )
        .expect("Main transfer source");
        let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
            .expect("selected callable catalog");
        let inventory = catalog.selected_source_inventory();

        assert_eq!(catalog.len(), 3, "lookup catalog still owns Main methods");
        assert_eq!(
            inventory.len(),
            1,
            "Main methods are outside three-terminal loan"
        );
        let tools = catalog
            .declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                "Tools",
                "helper",
                0,
            )
            .expect("non-Main selected row")
            .key()
            .clone();
        assert!(inventory
            .site(&SelectedNormalCallableKeyV1::Cataloged(tools))
            .is_some());
    }

    #[test]
    fn distinct_top_level_occurrences_share_one_legacy_physical_projection() {
        let first = NormalTopLevelFunctionDraftAdmissionV1::from_catalog_key(
            super::SelectedTopLevelFunctionKeyV1::new(2, "same", 1),
        );
        let second = NormalTopLevelFunctionDraftAdmissionV1::from_catalog_key(
            super::SelectedTopLevelFunctionKeyV1::new(9, "same", 1),
        );

        assert_ne!(
            first.source_key().statement_index(),
            second.source_key().statement_index()
        );
        assert_eq!(first.physical_symbol(), "same/1");
        assert_eq!(second.physical_arity(), 1);
        let (key, symbol, arity) = second.into_legacy_collector_parts();
        assert_eq!(key, FunctionDraftKeyV1::LegacySymbol("same/1".to_owned()));
        assert_eq!(symbol, "same/1");
        assert_eq!(arity, 1);
    }
}
