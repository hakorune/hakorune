//! Disconnected preparation for one selected Legacy Stage-B module shell.
//!
//! The complete selected owner remains intact. This row only pairs it with a
//! read-only receipt from an already-open Builder candidate. Catalog/import
//! installation, root lowering, function capture, retry, and fallback remain
//! unavailable.

mod install;
mod ledger;

use crate::mir::builder::MirBuilder;
use crate::mir::preloop_stageb_candidate_shell::{
    PreloopStageBCandidateShellReadinessErrorV1, VerifiedPreloopStageBCandidateShellReadinessV1,
};

use super::legacy_source_selection::PreparedSelectedPreloopStageBWholeSourceV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopStageBModuleActivationStageV1 {
    CandidateShell,
}

#[derive(Debug)]
pub(super) struct PreparedPreloopStageBModuleActivationV1 {
    selected: PreparedSelectedPreloopStageBWholeSourceV1,
    readiness: VerifiedPreloopStageBCandidateShellReadinessV1,
}

impl PreparedPreloopStageBModuleActivationV1 {
    #[cfg(test)]
    const fn selected(&self) -> &PreparedSelectedPreloopStageBWholeSourceV1 {
        &self.selected
    }

    pub(super) fn discard(self) {
        let Self {
            selected,
            readiness,
        } = self;
        selected.discard();
        let _ = readiness;
    }

    pub(super) fn into_preinstalled_root_request_v1(
        self,
    ) -> install::PreparedPreloopStageBPreinstalledRootV1 {
        install::PreparedPreloopStageBPreinstalledRootV1::from_module_activation(self)
    }
}

#[derive(Debug)]
pub(super) struct RejectedPreloopStageBModuleActivationV1 {
    selected: PreparedSelectedPreloopStageBWholeSourceV1,
    stage: PreloopStageBModuleActivationStageV1,
    cause: PreloopStageBCandidateShellReadinessErrorV1,
}

impl RejectedPreloopStageBModuleActivationV1 {
    pub(super) const fn stage(&self) -> PreloopStageBModuleActivationStageV1 {
        self.stage
    }

    pub(super) const fn cause(&self) -> PreloopStageBCandidateShellReadinessErrorV1 {
        self.cause
    }

    pub(super) fn bounded_report(&self) -> Box<str> {
        format!(
            "[mir/preloop-stageb/module-activation/{:?}] {:?}",
            self.stage, self.cause
        )
        .into_boxed_str()
    }

    pub(super) fn discard(self) {
        self.selected.discard();
    }
}

pub(super) fn prepare_preloop_stageb_module_activation_v1(
    selected: PreparedSelectedPreloopStageBWholeSourceV1,
    builder: &MirBuilder,
) -> Result<PreparedPreloopStageBModuleActivationV1, RejectedPreloopStageBModuleActivationV1> {
    let (imports_are_explicit, import_count, imports) = selected.import_expectation();
    match builder.verify_preloop_stageb_candidate_shell_readiness_v1(
        imports_are_explicit,
        import_count,
        imports,
    ) {
        Ok(readiness) => Ok(PreparedPreloopStageBModuleActivationV1 {
            selected,
            readiness,
        }),
        Err(cause) => Err(RejectedPreloopStageBModuleActivationV1 {
            selected,
            stage: PreloopStageBModuleActivationStageV1::CandidateShell,
            cause,
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::mir::builder::{MirBuilder, VerifiedSameModuleCallableDeclarationCatalogV1};
    use crate::mir::preloop_stageb_candidate_shell::PreloopStageBCandidateShellReadinessErrorV1;
    use crate::parser::NyashParser;

    use super::super::legacy_source_selection::{
        PreloopStageBWholeSourceDispositionV1, PreloopStageBWholeSourceProducerV1,
    };
    use super::super::legacy_static_import_snapshot::CompilerSuppliedStaticImportSnapshotV1;
    use super::super::legacy_whole_source_request::LegacyWholeSourceCompileRequestV1;
    use super::super::lowering_input::LegacyModuleLoweringInputV1;

    const SOURCE: &str = r#"
static box Helper {
  pick(a, b) { return b }
}
box Caller {
  run(text, ret) {
    local pos
    pos = Helper.pick(text, me.value(ret))
    return pos
  }
  value(ret) { return 1 }
}
"#;

    fn selected(
        imports: CompilerSuppliedStaticImportSnapshotV1,
    ) -> super::PreparedSelectedPreloopStageBWholeSourceV1 {
        let ast = NyashParser::parse_from_string(SOURCE).expect("Stage-B activation source");
        let request = LegacyWholeSourceCompileRequestV1::new(
            LegacyModuleLoweringInputV1::bare_ast(ast),
            imports,
            Some("fixture.hako".into()),
        );
        match PreloopStageBWholeSourceProducerV1::select(request).expect("unique selection") {
            PreloopStageBWholeSourceDispositionV1::Selected(selected) => selected,
            other => panic!("expected selected source, got {other:?}"),
        }
    }

    fn prepared_builder() -> MirBuilder {
        let mut builder = MirBuilder::new();
        builder
            .prepare_module_for_preloop_stageb_shell_test_v1()
            .expect("candidate shell");
        builder
    }

    #[test]
    fn vacant_candidate_shell_prepares_without_mutation() {
        let builder = prepared_builder();
        let before_name = builder.current_function_name().map(str::to_owned);
        let before_entry = builder.current_function_entry_block();
        let prepared = selected(CompilerSuppliedStaticImportSnapshotV1::none())
            .prepare_module_activation(&builder)
            .expect("vacant shell");
        assert_eq!(
            prepared.selected().diagnostic_source_hint(),
            Some("fixture.hako")
        );
        assert_eq!(
            builder.current_function_name().map(str::to_owned),
            before_name
        );
        assert_eq!(builder.current_function_entry_block(), before_entry);
        prepared.discard();
    }

    #[test]
    fn missing_module_retains_the_complete_selected_owner() {
        let rejected = selected(CompilerSuppliedStaticImportSnapshotV1::none())
            .prepare_module_activation(&MirBuilder::new())
            .expect_err("module must be prepared");
        assert_eq!(
            rejected.cause(),
            PreloopStageBCandidateShellReadinessErrorV1::CandidateModuleMissing
        );
        assert!(rejected.bounded_report().contains("CandidateModuleMissing"));
        rejected.discard();
    }

    #[test]
    fn occupied_catalog_lane_rejects_before_install() {
        let ast = NyashParser::parse_from_string(SOURCE).unwrap();
        let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&ast).unwrap();
        let mut builder = prepared_builder();
        builder.install_callable_catalog_for_preloop_stageb_shell_test_v1(catalog);
        let rejected = selected(CompilerSuppliedStaticImportSnapshotV1::none())
            .prepare_module_activation(&builder)
            .expect_err("occupied catalog");
        assert_eq!(
            rejected.cause(),
            PreloopStageBCandidateShellReadinessErrorV1::CallableCatalogLaneOccupied
        );
        rejected.discard();
    }

    #[test]
    fn exact_installed_aliases_are_compatible_but_conflicts_reject() {
        let imports = || {
            CompilerSuppliedStaticImportSnapshotV1::explicit([(
                "Alias".to_owned(),
                "Helper".to_owned(),
            )])
            .unwrap()
        };
        let mut exact = prepared_builder();
        exact
            .comp_ctx
            .set_using_import_boxes(HashMap::from([("Alias".to_owned(), "Helper".to_owned())]));
        selected(imports())
            .prepare_module_activation(&exact)
            .expect("exact aliases")
            .discard();

        let mut conflict = prepared_builder();
        conflict
            .comp_ctx
            .set_using_import_boxes(HashMap::from([("Alias".to_owned(), "Other".to_owned())]));
        let rejected = selected(imports())
            .prepare_module_activation(&conflict)
            .expect_err("conflicting aliases");
        assert_eq!(
            rejected.cause(),
            PreloopStageBCandidateShellReadinessErrorV1::ImportAliasLaneConflict
        );
        rejected.discard();
    }
}
