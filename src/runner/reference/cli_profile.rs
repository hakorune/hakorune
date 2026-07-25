//! Shared CLI admission for explicit reference runners.
//!
//! This module owns only common, pre-I/O CLI facts. Route-specific request
//! owners decide their backend spelling and optimization policy afterwards.

use crate::cli::CliConfig;
use hakorune_frontend_parser::parser::GrammarProfile;

const DEFAULT_DEBUG_FUEL: usize = 100_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReferenceCliProfileFactsV1 {
    source_file: Box<str>,
}

impl ReferenceCliProfileFactsV1 {
    pub(crate) fn into_source_file(self) -> Box<str> {
        self.source_file
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ReferenceCliProfileErrorV1 {
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

impl ReferenceCliProfileErrorV1 {
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

/// Seal common explicit-reference facts without reading source or selecting a
/// route-specific compiler/runtime policy.
pub(crate) fn seal_reference_cli_profile(
    config: &CliConfig,
) -> Result<ReferenceCliProfileFactsV1, ReferenceCliProfileErrorV1> {
    let source_file = match config.file.as_deref() {
        None => return Err(ReferenceCliProfileErrorV1::SourceFileRequired),
        Some("") => return Err(ReferenceCliProfileErrorV1::EmptySourceFile),
        Some(path) => path.to_owned().into_boxed_str(),
    };

    if !matches!(config.grammar_profile, GrammarProfile::Canonical) {
        return Err(ReferenceCliProfileErrorV1::NonCanonicalGrammar);
    }
    if !config.cli_usings.is_empty() {
        return Err(ReferenceCliProfileErrorV1::UsingRequested);
    }
    if config.repl {
        return Err(ReferenceCliProfileErrorV1::ReplRequested);
    }
    if config.dev || config.stage3 || config.ny_compiler_args.is_some() {
        return Err(ReferenceCliProfileErrorV1::DevelopmentRouteRequested);
    }
    if config.run_tests
        || config.test_filter.is_some()
        || config.test_entry.is_some()
        || config.test_return.is_some()
    {
        return Err(ReferenceCliProfileErrorV1::TestRouteRequested);
    }
    if !config.script_args.is_empty() {
        return Err(ReferenceCliProfileErrorV1::ScriptArgsRequested);
    }
    if config.ny_parser_pipe || config.json_file.is_some() || config.mir_json_file.is_some() {
        return Err(ReferenceCliProfileErrorV1::JsonRouteRequested);
    }
    if has_emit_route(config) {
        return Err(ReferenceCliProfileErrorV1::EmitRouteRequested);
    }
    if has_build_route(config) {
        return Err(ReferenceCliProfileErrorV1::BuildRouteRequested);
    }
    if has_diagnostic_route(config) {
        return Err(ReferenceCliProfileErrorV1::DiagnosticRouteRequested);
    }
    if has_macro_route(config) {
        return Err(ReferenceCliProfileErrorV1::MacroRouteRequested);
    }
    if config.load_ny_plugins || has_provider_route(config) {
        return Err(ReferenceCliProfileErrorV1::PluginRequested);
    }
    if has_jit_route(config) {
        return Err(ReferenceCliProfileErrorV1::JitRequested);
    }
    if config.gc_mode.is_some() {
        return Err(ReferenceCliProfileErrorV1::GcModeRequested);
    }
    if config.debug_fuel != Some(DEFAULT_DEBUG_FUEL) {
        return Err(ReferenceCliProfileErrorV1::NonDefaultDebugFuel);
    }

    Ok(ReferenceCliProfileFactsV1 { source_file })
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
