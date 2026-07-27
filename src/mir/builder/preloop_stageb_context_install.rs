//! Atomic callable-catalog and alias installation for one selected Stage-B shell.
//!
//! This module owns only the candidate `CompilationContext` transaction.
//! Source selection, activation rows, root lowering, function ledgers, retry,
//! and fallback remain outside this boundary.

use std::collections::HashMap;

use super::{MirBuilder, VerifiedSameModuleCallableDeclarationCatalogV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PreloopStageBContextInstallErrorV1 {
    CallableCatalogLaneOccupied,
    ImportAliasLaneConflict,
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedPreloopStageBContextInstallV1 {
    catalog: VerifiedSameModuleCallableDeclarationCatalogV1,
    aliases: HashMap<String, String>,
    aliases_are_explicit: bool,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedPreloopStageBContextInstallV1 {
    owner: PreparedPreloopStageBContextInstallV1,
    cause: PreloopStageBContextInstallErrorV1,
}

impl RejectedPreloopStageBContextInstallV1 {
    pub(in crate::mir) const fn cause(&self) -> PreloopStageBContextInstallErrorV1 {
        self.cause
    }

    pub(in crate::mir) fn discard(self) {
        let _ = self.owner;
    }
}

#[derive(Debug)]
pub(in crate::mir) struct InstalledPreloopStageBContextV1 {
    _seal: InstalledPreloopStageBContextSealV1,
}

#[derive(Debug)]
struct InstalledPreloopStageBContextSealV1(());

impl PreparedPreloopStageBContextInstallV1 {
    pub(in crate::mir) fn new(
        catalog: VerifiedSameModuleCallableDeclarationCatalogV1,
        aliases: HashMap<String, String>,
        aliases_are_explicit: bool,
    ) -> Self {
        Self {
            catalog,
            aliases,
            aliases_are_explicit,
        }
    }

    pub(in crate::mir) fn commit(
        self,
        builder: &mut MirBuilder,
    ) -> Result<InstalledPreloopStageBContextV1, RejectedPreloopStageBContextInstallV1> {
        if !builder
            .comp_ctx
            .callable_declaration_catalog_lane_is_vacant()
        {
            return Err(RejectedPreloopStageBContextInstallV1 {
                owner: self,
                cause: PreloopStageBContextInstallErrorV1::CallableCatalogLaneOccupied,
            });
        }
        let aliases_are_compatible = builder.comp_ctx.using_import_boxes_are_vacant()
            || (self.aliases_are_explicit
                && builder
                    .comp_ctx
                    .using_import_boxes_match(self.aliases.len(), self.alias_entries()));
        if !aliases_are_compatible {
            return Err(RejectedPreloopStageBContextInstallV1 {
                owner: self,
                cause: PreloopStageBContextInstallErrorV1::ImportAliasLaneConflict,
            });
        }

        builder.comp_ctx.install_preloop_stageb_context_preflighted(
            self.catalog,
            self.aliases,
            self.aliases_are_explicit,
        );
        Ok(InstalledPreloopStageBContextV1 {
            _seal: InstalledPreloopStageBContextSealV1(()),
        })
    }

    fn alias_entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.aliases
            .iter()
            .map(|(alias, owner)| (alias.as_str(), owner.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::NyashParser;

    use super::{
        MirBuilder, PreloopStageBContextInstallErrorV1, PreparedPreloopStageBContextInstallV1,
        VerifiedSameModuleCallableDeclarationCatalogV1,
    };

    const SOURCE: &str = r#"
static box Helper {
  pick(a, b) { return b }
}
"#;

    fn catalog() -> VerifiedSameModuleCallableDeclarationCatalogV1 {
        let ast = NyashParser::parse_from_string(SOURCE).expect("context-install source");
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&ast)
            .expect("context-install catalog")
    }

    fn aliases(owner: &str) -> HashMap<String, String> {
        HashMap::from([("Alias".to_owned(), owner.to_owned())])
    }

    #[test]
    fn vacant_context_commits_none_or_explicit_aliases_atomically() {
        let mut none = MirBuilder::new();
        PreparedPreloopStageBContextInstallV1::new(catalog(), HashMap::new(), false)
            .commit(&mut none)
            .expect("vacant None context");
        assert!(none.comp_ctx.callable_declaration_catalog().is_ok());
        assert!(none.comp_ctx.using_import_boxes.is_empty());

        let mut explicit = MirBuilder::new();
        PreparedPreloopStageBContextInstallV1::new(catalog(), aliases("Helper"), true)
            .commit(&mut explicit)
            .expect("vacant explicit context");
        assert!(explicit.comp_ctx.callable_declaration_catalog().is_ok());
        assert_eq!(
            explicit.comp_ctx.resolve_imported_static_box("Alias"),
            Some("Helper")
        );
    }

    #[test]
    fn exact_preinstalled_aliases_are_accepted_without_reclassification() {
        let mut builder = MirBuilder::new();
        builder.comp_ctx.set_using_import_boxes(aliases("Helper"));
        PreparedPreloopStageBContextInstallV1::new(catalog(), aliases("Helper"), true)
            .commit(&mut builder)
            .expect("exact alias lane");
        assert!(builder.comp_ctx.callable_declaration_catalog().is_ok());
        assert_eq!(
            builder.comp_ctx.resolve_imported_static_box("Alias"),
            Some("Helper")
        );
    }

    #[test]
    fn occupied_catalog_rejects_without_changing_aliases() {
        let mut builder = MirBuilder::new();
        builder
            .comp_ctx
            .install_callable_declaration_catalog(catalog())
            .expect("first catalog");
        builder.comp_ctx.set_using_import_boxes(aliases("Existing"));

        let rejected =
            PreparedPreloopStageBContextInstallV1::new(catalog(), aliases("Helper"), true)
                .commit(&mut builder)
                .expect_err("occupied catalog");
        assert_eq!(
            rejected.cause(),
            PreloopStageBContextInstallErrorV1::CallableCatalogLaneOccupied
        );
        assert_eq!(
            builder.comp_ctx.resolve_imported_static_box("Alias"),
            Some("Existing")
        );
        rejected.discard();
    }

    #[test]
    fn stale_alias_conflict_rejects_without_installing_catalog() {
        let mut builder = MirBuilder::new();
        let prepared =
            PreparedPreloopStageBContextInstallV1::new(catalog(), aliases("Helper"), true);
        builder
            .comp_ctx
            .set_using_import_boxes(aliases("ChangedAfterReadiness"));

        let rejected = prepared
            .commit(&mut builder)
            .expect_err("stale readiness must be rechecked");
        assert_eq!(
            rejected.cause(),
            PreloopStageBContextInstallErrorV1::ImportAliasLaneConflict
        );
        assert!(builder
            .comp_ctx
            .callable_declaration_catalog_lane_is_vacant());
        assert_eq!(
            builder.comp_ctx.resolve_imported_static_box("Alias"),
            Some("ChangedAfterReadiness")
        );
        rejected.discard();
    }
}
