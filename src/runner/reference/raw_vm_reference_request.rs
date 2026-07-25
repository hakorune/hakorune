//! Request boundary for the supported opt-in Raw VM-reference profile.
//!
//! This module converts already-parsed CLI facts into one typed request.  The
//! profile is consumed exactly once by the explicit reference runner; normal
//! and default routes remain disconnected by the accepted cutover decision.

use crate::cli::CliConfig;
use crate::ast::ASTNode;
use crate::mir::{
    RawPublishedCompileProfileV1, RawPublishedCompileRequestV1,
    RawVmReferenceExecutionProfileV1, RawVmReferenceInvocationV1,
};
use hakorune_frontend_parser::parser::GrammarProfile;

const RAW_VM_REFERENCE_BACKEND: &str = "raw-vm-reference";
const DEFAULT_DEBUG_FUEL: usize = 100_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawVmReferenceGrammarV1 {
    Canonical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RawVmReferenceProfileErrorV1 {
    SourceFileRequired,
    EmptySourceFile,
    NonCanonicalGrammar,
    UsingRequested,
    ReplRequested,
    JsonRouteRequested,
    EmitRouteRequested,
    BuildRouteRequested,
    DiagnosticRouteRequested,
    MacroRouteRequested,
    PluginRequested,
    JitRequested,
    GcModeRequested,
    NonDefaultDebugFuel,
    DevelopmentRouteRequested,
    TestRouteRequested,
    ScriptArgsRequested,
}

impl RawVmReferenceProfileErrorV1 {
    pub(crate) const fn code(&self) -> &'static str {
        match self {
            Self::SourceFileRequired => "source-file-required",
            Self::EmptySourceFile => "empty-source-file",
            Self::NonCanonicalGrammar => "non-canonical-grammar",
            Self::UsingRequested => "using-requested",
            Self::ReplRequested => "repl-requested",
            Self::JsonRouteRequested => "json-route-requested",
            Self::EmitRouteRequested => "emit-route-requested",
            Self::BuildRouteRequested => "build-route-requested",
            Self::DiagnosticRouteRequested => "diagnostic-route-requested",
            Self::MacroRouteRequested => "macro-route-requested",
            Self::PluginRequested => "plugin-requested",
            Self::JitRequested => "jit-requested",
            Self::GcModeRequested => "gc-mode-requested",
            Self::NonDefaultDebugFuel => "non-default-debug-fuel",
            Self::DevelopmentRouteRequested => "development-route-requested",
            Self::TestRouteRequested => "test-route-requested",
            Self::ScriptArgsRequested => "script-args-requested",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RawVmReferenceProductionRequestV1 {
    source_file: Box<str>,
    grammar: RawVmReferenceGrammarV1,
    compile_profile: RawPublishedCompileProfileV1,
    execution_profile: RawVmReferenceExecutionProfileV1,
    optimize: bool,
}

impl RawVmReferenceProductionRequestV1 {
    /// Select and seal CLI facts exactly once.  A non-target backend is
    /// explicitly not selected, so the default runner remains unchanged.
    pub(crate) fn select_from_cli(
        config: &CliConfig,
    ) -> Result<RawVmReferenceProfileSelectionV1, RawVmReferenceProfileErrorV1> {
        if config.backend != RAW_VM_REFERENCE_BACKEND {
            return Ok(RawVmReferenceProfileSelectionV1::NotSelected);
        }
        Self::try_from_selected_cli(config).map(RawVmReferenceProfileSelectionV1::Selected)
    }

    /// Seal selected CLI facts exactly once.  This is a pure check/copy operation:
    /// it does not read the source file, initialize plugins, mutate env, or
    /// call any compiler/runner entry.
    fn try_from_selected_cli(config: &CliConfig) -> Result<Self, RawVmReferenceProfileErrorV1> {
        let source_file = match config.file.as_deref() {
            None => return Err(RawVmReferenceProfileErrorV1::SourceFileRequired),
            Some("") => return Err(RawVmReferenceProfileErrorV1::EmptySourceFile),
            Some(path) => path.to_owned().into_boxed_str(),
        };

        if !matches!(config.grammar_profile, GrammarProfile::Canonical) {
            return Err(RawVmReferenceProfileErrorV1::NonCanonicalGrammar);
        }
        if !config.cli_usings.is_empty() {
            return Err(RawVmReferenceProfileErrorV1::UsingRequested);
        }
        if config.repl {
            return Err(RawVmReferenceProfileErrorV1::ReplRequested);
        }
        if config.dev || config.stage3 || config.ny_compiler_args.is_some() {
            return Err(RawVmReferenceProfileErrorV1::DevelopmentRouteRequested);
        }
        if config.run_tests
            || config.test_filter.is_some()
            || config.test_entry.is_some()
            || config.test_return.is_some()
        {
            return Err(RawVmReferenceProfileErrorV1::TestRouteRequested);
        }
        if !config.script_args.is_empty() {
            return Err(RawVmReferenceProfileErrorV1::ScriptArgsRequested);
        }
        if config.ny_parser_pipe || config.json_file.is_some() || config.mir_json_file.is_some() {
            return Err(RawVmReferenceProfileErrorV1::JsonRouteRequested);
        }
        if has_emit_route(config) {
            return Err(RawVmReferenceProfileErrorV1::EmitRouteRequested);
        }
        if has_build_route(config) {
            return Err(RawVmReferenceProfileErrorV1::BuildRouteRequested);
        }
        if has_diagnostic_route(config) {
            return Err(RawVmReferenceProfileErrorV1::DiagnosticRouteRequested);
        }
        if has_macro_route(config) {
            return Err(RawVmReferenceProfileErrorV1::MacroRouteRequested);
        }
        if config.load_ny_plugins || has_provider_route(config) {
            return Err(RawVmReferenceProfileErrorV1::PluginRequested);
        }
        if has_jit_route(config) {
            return Err(RawVmReferenceProfileErrorV1::JitRequested);
        }
        if config.gc_mode.is_some() {
            return Err(RawVmReferenceProfileErrorV1::GcModeRequested);
        }
        if config.debug_fuel != Some(DEFAULT_DEBUG_FUEL) {
            return Err(RawVmReferenceProfileErrorV1::NonDefaultDebugFuel);
        }

        Ok(Self {
            source_file,
            grammar: RawVmReferenceGrammarV1::Canonical,
            compile_profile: RawPublishedCompileProfileV1::narrow_v1(),
            execution_profile: RawVmReferenceExecutionProfileV1::CanonicalV1,
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
            compile_profile,
            execution_profile,
            ..
        } = self;
        let compile = RawPublishedCompileRequestV1::new(
            ast,
            Some(source_file),
            "main",
            compile_profile,
        );
        RawVmReferenceInvocationV1::new(compile, execution_profile)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RawVmReferenceProfileSelectionV1 {
    NotSelected,
    Selected(RawVmReferenceProductionRequestV1),
}

fn has_emit_route(config: &CliConfig) -> bool {
    config.emit_cfg.is_some()
        || config.emit_mir_json.is_some()
        || config.emit_mir_json_minimal.is_some()
        || config.emit_ast_json.is_some()
        || config.emit_parser_evidence_ast_json.is_some()
        || config.emit_program_json_v0.is_some()
        || config.hako_emit_mir_json
        || config.hako_run
        || config.emit_exe.is_some()
        || config.emit_exe_nyrt.is_some()
        || config.emit_exe_libs.is_some()
        || config.emit_wat.is_some()
        || config.compile_wasm
        || config.compile_native
}

fn has_build_route(config: &CliConfig) -> bool {
    config.build_path.is_some()
        || config.build_app.is_some()
        || config.build_out.is_some()
        || config.build_aot.is_some()
        || config.build_profile.is_some()
        || config.build_target.is_some()
        || config.output_file.is_some()
        || config.run_task.is_some()
        || config.benchmark
}

fn has_diagnostic_route(config: &CliConfig) -> bool {
    config.dump_ast
        || config.dump_mir
        || config.verify_mir
        || config.mir_verbose
        || config.mir_verbose_effects
        || config.vm_stats
        || config.vm_stats_json
        || config.cli_verbose
}

fn has_macro_route(config: &CliConfig) -> bool {
    config.macro_expand_child.is_some()
        || config.macro_preexpand
        || config.macro_preexpand_auto
        || config.macro_top_level_allow
        || config.macro_profile.is_some()
        || config.macro_ctx_json.is_some()
        || config.dump_expanded_ast_json
}

fn has_provider_route(config: &CliConfig) -> bool {
    config.allocator_hook_dry_run
        || config.allocator_hook_dry_run_plan.is_some()
        || config.allocator_hook_dry_run_proof.is_some()
        || config.allocator_provider_manifest.is_some()
        || config.allocator_provider_activation_safety_gate.is_some()
        || config.allocator_provider_activation_decision.is_some()
        || config.allocator_provider_registry_snapshot.is_some()
        || config.allocator_provider_selection_decision.is_some()
        || config.allocator_provider_proof_bundle_consumption.is_some()
        || config.provider_package_selected_binary_build_fixture
        || config.provider_package_hako_derived_build_fixture.is_some()
        || config.provider_package_existing_binary.is_some()
        || config.provider_package_out_dir.is_some()
        || config.provider_package_artifact_name.is_some()
        || config.provider_package_id.is_some()
        || config.provider_package_kind.is_some()
        || config.provider_package_name.is_some()
        || config.provider_package_version.is_some()
        || config.provider_package_target_triple.is_some()
        || config.provider_package_platform.is_some()
        || config.provider_package_profile.is_some()
        || config.provider_package_hako_semantic_codegen.is_some()
        || config.provider_package_provider_call_allowed
        || config.provider_package_force
}

fn has_jit_route(config: &CliConfig) -> bool {
    config.jit_exec
        || config.jit_stats
        || config.jit_stats_json
        || config.jit_dump
        || config.jit_events
        || config.jit_events_compile
        || config.jit_events_runtime
        || config.jit_events_path.is_some()
        || config.jit_threshold.is_some()
        || config.jit_phi_min
        || config.jit_hostcall
        || config.jit_handle_debug
        || config.jit_native_f64
        || config.jit_native_bool
        || config.jit_only
        || config.jit_direct
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canonical_config() -> CliConfig {
        let mut config = CliConfig::default();
        config.backend = RAW_VM_REFERENCE_BACKEND.to_owned();
        config.file = Some("profile0.hako".to_owned());
        config
    }

    #[test]
    fn seals_independent_canonical_narrow_profile_fields_once() {
        let selection = RawVmReferenceProductionRequestV1::select_from_cli(&canonical_config())
            .expect("canonical profile facts should seal");
        let RawVmReferenceProfileSelectionV1::Selected(request) = selection else {
            panic!("raw-vm-reference must be selected");
        };

        assert_eq!(request.source_file(), "profile0.hako");
        assert_eq!(request.grammar(), RawVmReferenceGrammarV1::Canonical);
        assert_eq!(
            request.compile_profile,
            RawPublishedCompileProfileV1::narrow_v1()
        );
        assert!(request.optimize());
    }

    #[test]
    fn preserves_no_optimize_as_a_typed_snapshot() {
        let mut config = canonical_config();
        config.no_optimize = true;
        let selection = RawVmReferenceProductionRequestV1::select_from_cli(&config)
            .expect("no-optimize is the only retained tuning fact");
        let RawVmReferenceProfileSelectionV1::Selected(request) = selection else {
            panic!("raw-vm-reference must be selected");
        };
        assert!(!request.optimize());
    }

    #[test]
    fn default_backend_families_are_not_selected() {
        for backend in ["mir", "vm", "vm-hako", "llvm"] {
            let mut config = CliConfig::default();
            config.backend = backend.to_owned();
            assert_eq!(
                RawVmReferenceProductionRequestV1::select_from_cli(&config),
                Ok(RawVmReferenceProfileSelectionV1::NotSelected),
                "default backend {backend} must fall through"
            );
        }
    }

    #[test]
    fn rejects_conflicts_without_source_or_runner_effects() {
        let mut config = canonical_config();
        config.cli_usings.push("pkg".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::UsingRequested)
        );

        let mut config = canonical_config();
        config.json_file = Some("program.json".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::JsonRouteRequested)
        );

        let mut config = canonical_config();
        config.macro_preexpand = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::MacroRouteRequested)
        );

        let mut config = canonical_config();
        config.emit_mir_json = Some("out.json".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::EmitRouteRequested)
        );

        let mut config = canonical_config();
        config.build_path = Some("hako.toml".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::BuildRouteRequested)
        );

        let mut config = canonical_config();
        config.dump_mir = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::DiagnosticRouteRequested)
        );

        let mut config = canonical_config();
        config.load_ny_plugins = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::PluginRequested)
        );

        let mut config = canonical_config();
        config.dev = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::DevelopmentRouteRequested)
        );

        let mut config = canonical_config();
        config.run_tests = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::TestRouteRequested)
        );

        let mut config = canonical_config();
        config.jit_stats = true;
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::JitRequested)
        );

        let mut config = canonical_config();
        config.gc_mode = Some("rc".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::GcModeRequested)
        );

        let mut config = canonical_config();
        config.debug_fuel = Some(1);
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::NonDefaultDebugFuel)
        );
    }

    #[test]
    fn rejects_noncanonical_or_missing_source_before_file_io() {
        let mut config = CliConfig::default();
        config.backend = RAW_VM_REFERENCE_BACKEND.to_owned();
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::SourceFileRequired)
        );

        let mut config = canonical_config();
        config.grammar_profile = GrammarProfile::Compat2025;
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::NonCanonicalGrammar)
        );

        let mut config = canonical_config();
        config.script_args.push("runtime-arg".to_owned());
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&config),
            Err(RawVmReferenceProfileErrorV1::ScriptArgsRequested)
        );
    }

    #[test]
    fn default_backend_is_not_selected_and_has_no_profile_side_effect() {
        assert_eq!(
            RawVmReferenceProductionRequestV1::select_from_cli(&CliConfig::default())
                .expect("non-target backend is not an error"),
            RawVmReferenceProfileSelectionV1::NotSelected
        );
    }
}
