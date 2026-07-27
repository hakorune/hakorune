//! Disconnected selected-root installation and post-install-kernel handoff.
//!
//! This is the sole C4 owner chain. It consumes named source/activation
//! projections, uses the atomic context transaction, retains one armed row on
//! the stack, and calls the existing post-install root kernel. It has no
//! production caller, function capture, retry, fallback, or publication.

use crate::mir::builder::MirBuilder;
use crate::mir::builder::{
    CompletedPreloopStageBFunctionActivationV1, RejectedPreloopStageBFunctionActivationV1,
};
use crate::mir::preloop_stageb_candidate_shell::VerifiedPreloopStageBCandidateShellReadinessV1;
use crate::mir::preloop_stageb_carrier::{
    PreparedPreloopStageBActivationContextInstallV1,
    RejectedPreloopStageBActivationContextInstallV1,
};
use crate::mir::ValueId;

use super::ledger::PreloopStageBFunctionActivationLedgerV1;
use super::PreparedPreloopStageBModuleActivationV1;
use crate::mir::compiler::legacy_whole_source_request::{
    PreparedPreloopStageBSourceInstallPartsV1, RetainedPreloopStageBSourceOwnerV1,
};
use crate::mir::compiler::lowering_input::LegacyModuleOriginV1;

#[derive(Debug)]
pub(super) struct PreparedPreloopStageBPreinstalledRootV1 {
    source: RetainedPreloopStageBSourceOwnerV1,
    activation: PreparedPreloopStageBActivationContextInstallV1,
    readiness: VerifiedPreloopStageBCandidateShellReadinessV1,
}

impl PreparedPreloopStageBPreinstalledRootV1 {
    pub(super) fn from_module_activation(
        prepared: PreparedPreloopStageBModuleActivationV1,
    ) -> Self {
        let selected = prepared.selected.into_install_parts_v1();
        let source_parts = selected.source.into_source_install_parts_v1();
        debug_assert_eq!(source_parts.origin, LegacyModuleOriginV1::BareAst);
        let PreparedPreloopStageBSourceInstallPartsV1 {
            ast,
            origin,
            aliases,
            diagnostic_source_hint,
        } = source_parts;
        let source = RetainedPreloopStageBSourceOwnerV1 {
            ast,
            origin,
            diagnostic_source_hint,
        };
        let aliases = aliases.into_builder_projection();
        let activation = selected
            .activation
            .into_module_install_parts_v1()
            .attach_aliases(aliases);
        Self {
            source,
            activation,
            readiness: prepared.readiness,
        }
    }

    pub(super) fn commit(
        self,
        builder: &mut MirBuilder,
    ) -> Result<InstalledPreloopStageBModuleActivationV1, RejectedPreloopStageBPreinstalledRootV1>
    {
        match self.activation.commit(builder) {
            Ok(installed) => Ok(InstalledPreloopStageBModuleActivationV1 {
                source: self.source,
                readiness: self.readiness,
                ledger: PreloopStageBFunctionActivationLedgerV1::armed(
                    installed.into_ledger_parts(),
                ),
            }),
            Err(activation) => Err(RejectedPreloopStageBPreinstalledRootV1 {
                source: self.source,
                readiness: self.readiness,
                cause: RejectedPreloopStageBPreinstalledRootCauseV1::Context(activation),
            }),
        }
    }
}

#[derive(Debug)]
pub(super) struct InstalledPreloopStageBModuleActivationV1 {
    source: RetainedPreloopStageBSourceOwnerV1,
    readiness: VerifiedPreloopStageBCandidateShellReadinessV1,
    ledger: PreloopStageBFunctionActivationLedgerV1,
}

impl InstalledPreloopStageBModuleActivationV1 {
    pub(super) fn lower_root(
        self,
        builder: &mut MirBuilder,
    ) -> Result<CompletedPreloopStageBPreinstalledRootV1, RejectedPreloopStageBPreinstalledRootV1>
    {
        match builder.lower_root_with_preloop_stageb_function_activation_v1(
            &self.source.ast,
            self.ledger.into_prepared(),
        ) {
            Ok(activation) => Ok(CompletedPreloopStageBPreinstalledRootV1 {
                activation,
                source: self.source,
                readiness: self.readiness,
            }),
            Err(activation) => Err(RejectedPreloopStageBPreinstalledRootV1 {
                source: self.source,
                readiness: self.readiness,
                cause: RejectedPreloopStageBPreinstalledRootCauseV1::FunctionActivation(activation),
            }),
        }
    }
}

#[derive(Debug)]
pub(super) struct CompletedPreloopStageBPreinstalledRootV1 {
    activation: CompletedPreloopStageBFunctionActivationV1,
    source: RetainedPreloopStageBSourceOwnerV1,
    readiness: VerifiedPreloopStageBCandidateShellReadinessV1,
}

impl CompletedPreloopStageBPreinstalledRootV1 {
    pub(super) const fn result_value(&self) -> ValueId {
        self.activation.result_value()
    }

    pub(super) fn activation(&self) -> &CompletedPreloopStageBFunctionActivationV1 {
        &self.activation
    }

    pub(super) fn discard(self) {
        let _ = (self.source, self.readiness);
        self.activation.discard();
    }
}

#[derive(Debug)]
enum RejectedPreloopStageBPreinstalledRootCauseV1 {
    Context(RejectedPreloopStageBActivationContextInstallV1),
    FunctionActivation(RejectedPreloopStageBFunctionActivationV1),
}

#[derive(Debug)]
pub(super) struct RejectedPreloopStageBPreinstalledRootV1 {
    source: RetainedPreloopStageBSourceOwnerV1,
    readiness: VerifiedPreloopStageBCandidateShellReadinessV1,
    cause: RejectedPreloopStageBPreinstalledRootCauseV1,
}

impl RejectedPreloopStageBPreinstalledRootV1 {
    pub(super) fn diagnostic_source_hint(&self) -> Option<&str> {
        self.source.diagnostic_source_hint.as_deref()
    }

    pub(super) fn bounded_report(&self) -> Box<str> {
        match &self.cause {
            RejectedPreloopStageBPreinstalledRootCauseV1::Context(cause) => cause.bounded_report(),
            RejectedPreloopStageBPreinstalledRootCauseV1::FunctionActivation(activation) => {
                activation.bounded_report()
            }
        }
    }

    pub(super) fn discard(self) {
        match self.cause {
            RejectedPreloopStageBPreinstalledRootCauseV1::Context(cause) => cause.discard(),
            RejectedPreloopStageBPreinstalledRootCauseV1::FunctionActivation(activation) => {
                activation.discard()
            }
        }
        let _ = (self.source, self.readiness);
    }

    #[cfg(test)]
    fn retained_completed_caller(
        &self,
    ) -> Option<&crate::mir::builder::CanonicalSameModuleCallableKeyV1> {
        match &self.cause {
            RejectedPreloopStageBPreinstalledRootCauseV1::FunctionActivation(activation) => {
                activation.retained_completed_caller()
            }
            _ => None,
        }
    }

    #[cfg(test)]
    fn retains_invocation_state(&self) -> bool {
        match &self.cause {
            RejectedPreloopStageBPreinstalledRootCauseV1::FunctionActivation(activation) => {
                activation.retains_invocation_state()
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::mir::builder::MirBuilder;
    use crate::mir::compiler::legacy_source_selection::PreparedSelectedPreloopStageBWholeSourceV1;
    use crate::mir::compiler::legacy_static_import_snapshot::CompilerSuppliedStaticImportSnapshotV1;
    use crate::mir::compiler::legacy_whole_source_request::LegacyWholeSourceCompileRequestV1;
    use crate::mir::compiler::lowering_input::LegacyModuleLoweringInputV1;
    use crate::parser::NyashParser;

    fn selected() -> PreparedSelectedPreloopStageBWholeSourceV1 {
        let source = crate::mir::callable_result_representation::actual_parser_add_fixture::
            stageb_source_for_lowering();
        let ast = NyashParser::parse_from_string(&source).expect("preinstalled-root source");
        let request = LegacyWholeSourceCompileRequestV1::new(
            LegacyModuleLoweringInputV1::bare_ast(ast),
            CompilerSuppliedStaticImportSnapshotV1::none(),
            Some("preinstalled-root.hako".into()),
        );
        PreparedSelectedPreloopStageBWholeSourceV1::from_exact_test_parts(
            request,
            crate::mir::preloop_stageb_carrier::test_support::actual_parser_activation_plan(),
        )
    }

    fn prepared_builder() -> MirBuilder {
        let mut builder = MirBuilder::new();
        builder
            .prepare_module_for_preloop_stageb_shell_test_v1()
            .expect("candidate shell");
        builder
    }

    #[test]
    fn selected_owner_installs_once_and_reaches_the_existing_root_kernel() {
        let mut builder = prepared_builder();
        let prepared = selected()
            .prepare_module_activation(&builder)
            .expect("readiness")
            .into_preinstalled_root_request_v1();
        let installed = prepared.commit(&mut builder).expect("atomic context");
        assert!(builder.comp_ctx.callable_declaration_catalog().is_ok());
        assert!(builder
            .comp_ctx
            .callable_declaration_catalog_is_shared_with(installed.ledger.context().catalog()));
        assert_eq!(installed.ledger.row().caller().owner(), "ParserBox");

        let rejected = installed
            .lower_root(&mut builder)
            .expect_err("real suffix remains the current Stage-B frontier");
        let caller = rejected
            .retained_completed_caller()
            .expect("selected F6 draft was collected before the suffix frontier");
        assert_eq!(caller.owner(), "ParserBox");
        assert_eq!(caller.name(), "static_const_parse_add");
        assert!(rejected.retains_invocation_state());
        assert!(rejected.bounded_report().contains("MissingTransientType"));
        rejected.discard();
    }

    #[test]
    fn stale_context_conflict_retains_the_exact_source_owner() {
        let mut builder = prepared_builder();
        let prepared = selected()
            .prepare_module_activation(&builder)
            .expect("readiness")
            .into_preinstalled_root_request_v1();
        builder.comp_ctx.set_using_import_boxes(HashMap::from([(
            "Alias".to_owned(),
            "ChangedAfterReadiness".to_owned(),
        )]));
        let rejected = prepared.commit(&mut builder).expect_err("stale readiness");
        assert_eq!(
            rejected.diagnostic_source_hint(),
            Some("preinstalled-root.hako")
        );
        assert!(builder.comp_ctx.callable_declaration_catalog().is_err());
        rejected.discard();
    }
}
