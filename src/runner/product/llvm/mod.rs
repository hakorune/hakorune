use super::super::NyashRunner;
use nyash_rust::parser::NyashParser;
use std::fs;

// Modularized boxes for LLVM mode
mod compile_options;
mod error;
mod exit_reporter;
mod fallback_executor;
mod harness_executor;
mod joinir_experiment;
mod method_id_injector;
mod mir_compiler;
mod pipeline_plan;
mod pipeline_report;
mod plugin_init;
mod pyvm_executor;
mod report;

// Re-export error types for convenience
use self::error::LlvmRunError;
use self::pipeline_plan::LlvmPipelinePlan;
use self::pipeline_report::{LlvmPipelineReport, PipelineReportBox};

impl NyashRunner {
    /// Execute LLVM mode (split)
    pub(crate) fn execute_llvm_mode(&self, filename: &str) {
        // Step 1: Plugin initialization
        if let Err(e) = plugin_init::PluginInitBox::init() {
            report::emit_error_and_exit(LlvmRunError::fatal(format!("Plugin init error: {}", e)));
        }

        // Read the file
        let code = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                report::emit_error_and_exit(LlvmRunError::fatal(format!(
                    "Error reading file {}: {}",
                    filename, e
                )));
            }
        };

        // Step 3: use the same source preparation contract as MIR emit mode.
        let prepared =
            match crate::runner::modes::common_util::source_hint::prepare_source_with_imports(
                self, filename, &code,
            ) {
                Ok(prepared) => prepared,
                Err(e) => {
                    report::emit_error_and_exit(LlvmRunError::fatal(format!("{}", e)));
                }
            };

        // Parse to AST (main)
        let ast = match NyashParser::parse_from_string(&prepared.code) {
            Ok(ast) => ast,
            Err(e) => {
                crate::runner::modes::common_util::diag::print_parse_error_with_context(
                    filename,
                    &prepared.code,
                    &e,
                );
                // Enhanced context: list merged prelude files if any.
                let preludes =
                    crate::runner::modes::common_util::resolve::clone_last_merged_preludes();
                if !preludes.is_empty() {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[parse/context] merged prelude files ({}):",
                        preludes.len()
                    ));
                    let show = std::cmp::min(16, preludes.len());
                    for p in preludes.iter().take(show) {
                        crate::runtime::get_global_ring0()
                            .log
                            .debug(&format!("  - {}", p));
                    }
                    if preludes.len() > show {
                        crate::runtime::get_global_ring0()
                            .log
                            .debug(&format!("  ... ({} more)", preludes.len() - show));
                    }
                }
                report::emit_error_and_exit(LlvmRunError::fatal(format!("Parse error: {}", e)));
            }
        };
        // Macro expansion (env-gated) after merge
        let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);
        let ast = crate::runner::modes::macro_child::normalize_core_pass(&ast);

        let pipeline_plan = LlvmPipelinePlan::current_default();
        let mut pipeline_report = LlvmPipelineReport::new(&pipeline_plan);

        // Compile to MIR
        let mut module = match compile_options::CompileOptionsBox::compile(
            ast,
            Some(filename),
            prepared.imports,
            pipeline_plan.compile_options,
        ) {
            Ok(m) => m,
            Err(e) => {
                report::emit_error_and_exit(LlvmRunError::fatal(format!("{}", e)));
            }
        };

        // Inject method_id for BoxCall where resolvable (by-id path)
        #[allow(unused_mut)]
        let _injected = if pipeline_plan.method_id_injector_enabled {
            method_id_injector::MethodIdInjectorBox::inject(&mut module)
        } else {
            0
        };
        pipeline_report.method_id_injector_mutation_count = _injected;

        // Phase 32 L-4.3a: JoinIR LLVM experiment hook
        let module = if pipeline_plan.joinir_experiment_hook_enabled {
            joinir_experiment::JoinIrExperimentBox::apply(module)
        } else {
            module
        };

        // Dev/Test helper: allow executing via PyVM harness when requested
        match pyvm_executor::PyVmExecutorBox::try_execute(&module) {
            Ok(code) => {
                pipeline_report.execution_backend = "pyvm";
                PipelineReportBox::emit_if_requested(&pipeline_report);
                exit_reporter::ExitReporterBox::emit_and_exit(code);
            }
            Err(e) if e.code == 0 && e.msg == "PyVM not requested" => {
                // Continue to next executor
            }
            Err(e) => {
                pipeline_report.execution_backend = "pyvm_error";
                PipelineReportBox::emit_if_requested(&pipeline_report);
                report::emit_error_and_exit(e);
            }
        }

        if let Some(out_path) = requested_object_output_path() {
            pipeline_report.execution_backend = "obj_out";
            PipelineReportBox::emit_if_requested(&pipeline_report);
            emit_requested_object_or_exit(&module, &out_path);
            return;
        }

        match execute_via_harness_or_fallback(&module) {
            Ok(outcome) => {
                pipeline_report.execution_backend = outcome.backend;
                pipeline_report.llvm_fallback_used = outcome.fallback_used;
                pipeline_report.llvm_fallback_reason = outcome.fallback_reason;
                pipeline_report.mock_fallback_used = outcome.backend == "mock";
                PipelineReportBox::emit_if_requested(&pipeline_report);
                exit_reporter::ExitReporterBox::emit_and_exit(outcome.code);
            }
            Err(e) => report::emit_error_and_exit(e),
        }

        // Execute via LLVM backend (mock or real)
        #[cfg(feature = "llvm-inkwell-legacy")]
        {
            use nyash_rust::backend::llvm_compile_and_execute;
            let temp_path = "nyash_llvm_temp";
            match llvm_compile_and_execute(&module, temp_path) {
                Ok(result) => {
                    if let Some(int_result) = result.as_any().downcast_ref::<IntegerBox>() {
                        let exit_code = int_result.value;
                        crate::console_println!("✅ LLVM execution completed!");
                        crate::console_println!("📊 Exit code: {}", exit_code);
                        exit_reporter::ExitReporterBox::emit_and_exit(exit_code as i32);
                    } else {
                        crate::console_println!(
                            "✅ LLVM execution completed (non-integer result)!"
                        );
                        crate::console_println!("📊 Result: {}", result.to_string_box().value);
                    }
                }
                Err(e) => {
                    report::emit_error_and_exit(LlvmRunError::fatal(format!(
                        "LLVM execution error: {}",
                        e
                    )));
                }
            }
        }
    }
}

struct LlvmExecutionOutcome {
    code: i32,
    backend: &'static str,
    fallback_used: bool,
    fallback_reason: &'static str,
}

fn execute_via_harness_or_fallback(
    module: &nyash_rust::mir::MirModule,
) -> Result<LlvmExecutionOutcome, LlvmRunError> {
    match harness_executor::HarnessExecutorBox::try_execute(module) {
        Ok(code) => Ok(LlvmExecutionOutcome {
            code,
            backend: "ny_llvmc_exe",
            fallback_used: false,
            fallback_reason: "none",
        }),
        Err(e) if crate::config::env::env_bool("NYASH_LLVM_USE_HARNESS") => Err(e),
        Err(_e) => {
            let code = fallback_executor::FallbackExecutorBox::execute(module)?;
            Ok(LlvmExecutionOutcome {
                code,
                backend: "mock",
                fallback_used: true,
                fallback_reason: "harness_unavailable_or_not_requested",
            })
        }
    }
}

fn requested_object_output_path() -> Option<String> {
    std::env::var("NYASH_LLVM_OBJ_OUT").ok()
}

fn emit_requested_object_or_exit(_module: &nyash_rust::mir::MirModule, _out_path: &str) {
    #[cfg(feature = "llvm-harness")]
    {
        if let Err(e) =
            crate::runner::modes::common_util::exec::ny_llvmc_emit_obj_lib(_module, _out_path)
        {
            report::emit_error_and_exit(LlvmRunError::fatal(format!("{}", e)));
        }
        return;
    }
    #[cfg(all(not(feature = "llvm-harness"), feature = "llvm-inkwell-legacy"))]
    {
        emit_requested_legacy_object_or_exit(_module, _out_path);
        return;
    }
    #[cfg(all(not(feature = "llvm-harness"), not(feature = "llvm-inkwell-legacy")))]
    {
        report::emit_error_and_exit(LlvmRunError::fatal(
            "LLVM backend not available (object emit)",
        ));
    }
}

#[cfg(all(not(feature = "llvm-harness"), feature = "llvm-inkwell-legacy"))]
fn emit_requested_legacy_object_or_exit(module: &nyash_rust::mir::MirModule, out_path: &str) {
    use nyash_rust::backend::llvm_compile_to_object;
    if let Err(e) =
        crate::mir::backend_capability::enforce_mir_backend_supported(module, "llvm-legacy-obj")
    {
        report::emit_error_and_exit(LlvmRunError::fatal(format!("{}", e)));
    }
    if let Some(parent) = std::path::Path::new(out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    crate::cli_v!(
        "[Runner/LLVM] emitting object to {} (cwd={})",
        out_path,
        std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default()
    );
    if let Err(e) = llvm_compile_to_object(module, out_path) {
        report::emit_error_and_exit(LlvmRunError::fatal(format!(
            "LLVM object emit error: {}",
            e
        )));
    }
    match std::fs::metadata(out_path) {
        Ok(meta) if meta.len() > 0 => {
            crate::cli_v!("[LLVM] object emitted: {} ({} bytes)", out_path, meta.len());
        }
        _ => {
            report::emit_error_and_exit(LlvmRunError::fatal(format!(
                "LLVM object not found or empty: {}",
                out_path
            )));
        }
    }
}

// emit_mir_json_for_harness moved to crate::runner::mir_json_emit
