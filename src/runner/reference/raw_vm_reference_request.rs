//! Request boundary for the supported opt-in Raw VM-reference profile.
//!
//! This module converts already-parsed CLI facts into one typed request.  The
//! profile is consumed exactly once by the explicit reference runner; normal
//! and default routes remain disconnected by the accepted cutover decision.

use crate::ast::ASTNode;
use crate::cli::CliConfig;
use crate::mir::{
    RawVmReferenceInvocationV1, RawVmReferenceSupportProfileV1,
};

use super::cli_profile::seal_reference_cli_profile;

pub(crate) use super::cli_profile::ReferenceCliProfileErrorV1 as RawVmReferenceProfileErrorV1;

const RAW_VM_REFERENCE_BACKEND: &str = "raw-vm-reference";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawVmReferenceGrammarV1 {
    Canonical,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RawVmReferenceProductionRequestV1 {
    source_file: Box<str>,
    grammar: RawVmReferenceGrammarV1,
    support_profile: RawVmReferenceSupportProfileV1,
    optimize: bool,
}

impl RawVmReferenceProductionRequestV1 {
    pub(super) const fn backend_name() -> &'static str {
        RAW_VM_REFERENCE_BACKEND
    }

    /// Seal selected CLI facts exactly once.  This is a pure check/copy operation:
    /// it does not read the source file, initialize plugins, mutate env, or
    /// call any compiler/runner entry.
    pub(super) fn try_from_selected_cli(
        config: &CliConfig,
    ) -> Result<Self, RawVmReferenceProfileErrorV1> {
        let source_file = seal_reference_cli_profile(config)?.into_source_file();

        Ok(Self {
            source_file,
            grammar: RawVmReferenceGrammarV1::Canonical,
            support_profile: RawVmReferenceSupportProfileV1::canonical_v1(),
            optimize: !config.no_optimize,
        })
    }

    pub(crate) fn source_file(&self) -> &str {
        &self.source_file
    }

    pub(crate) const fn grammar(&self) -> RawVmReferenceGrammarV1 {
        self.grammar
    }

    pub(crate) const fn optimize(&self) -> bool {
        self.optimize
    }

    pub(crate) fn into_invocation(self, ast: ASTNode) -> RawVmReferenceInvocationV1 {
        let Self {
            source_file,
            support_profile,
            ..
        } = self;
        support_profile.into_invocation(ast, Some(source_file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hakorune_frontend_parser::parser::GrammarProfile;

    fn canonical_config() -> CliConfig {
        let mut config = CliConfig::default();
        config.backend = RAW_VM_REFERENCE_BACKEND.to_owned();
        config.file = Some("profile0.hako".to_owned());
        config
    }

    #[test]
    fn seals_independent_canonical_narrow_profile_fields_once() {
        let request = RawVmReferenceProductionRequestV1::try_from_selected_cli(&canonical_config())
            .expect("canonical profile facts should seal");

        assert_eq!(request.source_file(), "profile0.hako");
        assert_eq!(request.grammar(), RawVmReferenceGrammarV1::Canonical);
        assert_eq!(
            request.support_profile,
            RawVmReferenceSupportProfileV1::canonical_v1()
        );
        assert!(request.optimize());
    }

    #[test]
    fn preserves_no_optimize_as_a_typed_snapshot() {
        let mut config = canonical_config();
        config.no_optimize = true;
        let request = RawVmReferenceProductionRequestV1::try_from_selected_cli(&config)
            .expect("no-optimize is the only retained tuning fact");
        assert!(!request.optimize());
    }

    #[test]
    fn rejects_conflicts_without_source_or_runner_effects() {
        let mut config = canonical_config();
        config.cli_usings.push("pkg".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::UsingRequested)
        );

        let mut config = canonical_config();
        config.json_file = Some("program.json".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::JsonRouteRequested)
        );

        let mut config = canonical_config();
        config.macro_preexpand = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::MacroRouteRequested)
        );

        let mut config = canonical_config();
        config.emit_mir_json = Some("out.json".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::EmitRouteRequested)
        );

        let mut config = canonical_config();
        config.build_path = Some("hako.toml".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::BuildRouteRequested)
        );

        let mut config = canonical_config();
        config.dump_mir = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::DiagnosticRouteRequested)
        );

        let mut config = canonical_config();
        config.load_ny_plugins = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::PluginRequested)
        );

        let mut config = canonical_config();
        config.dev = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::DevelopmentRouteRequested)
        );

        let mut config = canonical_config();
        config.run_tests = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::TestRouteRequested)
        );

        let mut config = canonical_config();
        config.jit_stats = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::JitRequested)
        );

        let mut config = canonical_config();
        config.gc_mode = Some("rc".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::GcModeRequested)
        );

        let mut config = canonical_config();
        config.debug_fuel = Some(1);
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::NonDefaultDebugFuel)
        );
    }

    #[test]
    fn rejects_noncanonical_or_missing_source_before_file_io() {
        let mut config = CliConfig::default();
        config.backend = RAW_VM_REFERENCE_BACKEND.to_owned();
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::SourceFileRequired)
        );

        let mut config = canonical_config();
        config.grammar_profile = GrammarProfile::Compat2025;
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::NonCanonicalGrammar)
        );

        let mut config = canonical_config();
        config.script_args.push("runtime-arg".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(RawVmReferenceProfileErrorV1::ScriptArgsRequested)
        );
    }

}
