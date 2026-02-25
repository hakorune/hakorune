use super::super::NyashRunner;
use nyash_rust::parser::NyashParser;
use std::fs;

// Modularized boxes for LLVM mode
mod plugin_init;
mod exit_reporter;
mod method_id_injector;
mod using_resolver;
mod mir_compiler;
mod pyvm_executor;
mod joinir_experiment;
mod object_emitter;
mod harness_executor;
mod fallback_executor;
mod error;
mod report;

// Re-export error types for convenience
use self::error::LlvmRunError;

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

        // Step 3: Using resolution and prelude merge
        let (clean_code, prelude_asts) = match using_resolver::UsingResolverBox::resolve(self, &code, filename) {
            Ok(result) => result,
            Err(e) => {
                report::emit_error_and_exit(LlvmRunError::fatal(format!("{}", e)));
            }
        };

        // Parse to AST (main)
        let main_ast = match NyashParser::parse_from_string(&clean_code) {
            Ok(ast) => ast,
            Err(e) => {
                crate::runner::modes::common_util::diag::print_parse_error_with_context(
                    filename, &clean_code, &e,
                );
                // Enhanced context: list merged prelude files if any (from text-merge path)
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
        // Merge preludes + main when enabled
        let use_ast = crate::config::env::using_ast_enabled();
        let ast = if use_ast && !prelude_asts.is_empty() {
            crate::runner::modes::common_util::resolve::merge_prelude_asts_with_main(
                prelude_asts,
                &main_ast,
            )
        } else {
            main_ast
        };
        // Macro expansion (env-gated) after merge
        let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);
        let ast = crate::runner::modes::macro_child::normalize_core_pass(&ast);

        // Compile to MIR
        let mut module = match mir_compiler::MirCompilerBox::compile(ast, Some(filename)) {
            Ok(m) => m,
            Err(e) => {
                report::emit_error_and_exit(LlvmRunError::fatal(format!("{}", e)));
            }
        };

        // Inject method_id for BoxCall where resolvable (by-id path)
        #[allow(unused_mut)]
        let _injected = method_id_injector::MethodIdInjectorBox::inject(&mut module);

        // Phase 32 L-4.3a: JoinIR LLVM experiment hook
        let module = joinir_experiment::JoinIrExperimentBox::apply(module);

        // Dev/Test helper: allow executing via PyVM harness when requested
        match pyvm_executor::PyVmExecutorBox::try_execute(&module) {
            Ok(code) => exit_reporter::ExitReporterBox::emit_and_exit(code),
            Err(e) if e.code == 0 && e.msg == "PyVM not requested" => {
                // Continue to next executor
            }
            Err(e) => report::emit_error_and_exit(e),
        }

        // If explicit object path is requested, emit object only
        if let Ok(_out_path) = std::env::var("NYASH_LLVM_OBJ_OUT") {
            #[cfg(feature = "llvm-harness")]
            {
                // Harness path (optional): if NYASH_LLVM_USE_HARNESS=1, try Python/llvmlite first.
                if crate::config::env::llvm_use_harness() {
                    if let Err(e) = object_emitter::ObjectEmitterBox::try_emit(&module) {
                        report::emit_error_and_exit(LlvmRunError::fatal(format!("{}", e)));
                    }
                    return;
                }
                // Verify object presence and size (>0)
                match std::fs::metadata(&_out_path) {
                    Ok(meta) => {
                        if meta.len() == 0 {
                            report::emit_error_and_exit(LlvmRunError::fatal(format!("harness object is empty: {}", _out_path)));
                        }
                        if std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1") {
                            crate::console_println!(
                                "[LLVM] object emitted: {} ({} bytes)",
                                _out_path,
                                meta.len()
                            );
                        }
                    }
                    Err(e) => {
                        report::emit_error_and_exit(LlvmRunError::fatal(format!("harness output not found after emit: {} ({})", _out_path, e)));
                    }
                }
                return;
            }
            #[cfg(all(not(feature = "llvm-harness"), feature = "llvm-inkwell-legacy"))]
            {
                use nyash_rust::backend::llvm_compile_to_object;
                // Ensure parent directory exists for the object file
                if let Some(parent) = std::path::Path::new(&_out_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                crate::cli_v!(
                    "[Runner/LLVM] emitting object to {} (cwd={})",
                    _out_path,
                    std::env::current_dir()
                        .map(|p| p.display().to_string())
                        .unwrap_or_default()
                );
                if let Err(e) = llvm_compile_to_object(&module, &_out_path) {
                    report::emit_error_and_exit(LlvmRunError::fatal(format!("LLVM object emit error: {}", e)));
                }
                match std::fs::metadata(&_out_path) {
                    Ok(meta) if meta.len() > 0 => {
                        crate::cli_v!(
                            "[LLVM] object emitted: {} ({} bytes)",
                            _out_path,
                            meta.len()
                        );
                    }
                    _ => {
                        report::emit_error_and_exit(LlvmRunError::fatal(format!("LLVM object not found or empty: {}", _out_path)));
                    }
                }
                return;
            }
            #[cfg(all(not(feature = "llvm-harness"), not(feature = "llvm-inkwell-legacy")))]
            {
                report::emit_error_and_exit(LlvmRunError::fatal("LLVM backend not available (object emit)"));
            }
        }

        // Execute via LLVM backend (harness preferred)
        match harness_executor::HarnessExecutorBox::try_execute(&module) {
            Ok(code) => exit_reporter::ExitReporterBox::emit_and_exit(code),
            Err(_e) => {
                // If harness failed, try fallback path
                match fallback_executor::FallbackExecutorBox::execute(&module) {
                    Ok(code) => exit_reporter::ExitReporterBox::emit_and_exit(code),
                    Err(fallback_err) => report::emit_error_and_exit(fallback_err),
                }
            }
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
                    report::emit_error_and_exit(LlvmRunError::fatal(format!("LLVM execution error: {}", e)));
                }
            }
        }
    }
}

// emit_mir_json_for_harness moved to crate::runner::mir_json_emit
