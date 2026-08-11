use super::super::NyashRunner;
use nyash_rust::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
use std::{fs, process};

impl NyashRunner {
    /// Execute MIR compilation and processing mode (split)
    pub(crate) fn execute_mir_mode(&self, filename: &str) {
        let code = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Error reading file {}: {}", filename, e);
                process::exit(1);
            }
        };

        let prepared =
            match crate::runner::modes::common_util::source_hint::prepare_source_with_imports(
                self, filename, &code,
            ) {
                Ok(prepared) => prepared,
                Err(e) => {
                    eprintln!("❌ {}", e);
                    process::exit(1);
                }
            };

        let parsed =
            match crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                &prepared.code,
                self.parser_build_config(),
            ) {
                Ok(parsed) => parsed,
                Err(e) => {
                    crate::runner::modes::common_util::diag::print_parse_error_with_context(
                        filename,
                        &prepared.code,
                        &e,
                    );
                    process::exit(1);
                }
            };
        let transformed = match crate::r#macro::transform_normal_callable_program_v1(parsed) {
            Ok(transformed) => transformed,
            Err(rejected) => {
                eprintln!("❌ MIR source transform error: {:?}", rejected);
                process::exit(1);
            }
        };
        let request = match transformed {
            crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) => {
                NormalCompileRequestV1::for_mir_mode_callable_source(
                    source,
                    Some(filename),
                    prepared.imports,
                )
            }
            crate::r#macro::NormalCallableTransformOutcomeV1::Compatibility {
                ast,
                reason: _reason,
            } => {
                match NormalCompileRequestV1::for_mir_mode(ast, Some(filename), prepared.imports) {
                    Ok(request) => request,
                    Err(rejected) => {
                        eprintln!("❌ MIR compilation error: {}", rejected);
                        process::exit(1);
                    }
                }
            }
        };
        let mut mir_compiler = MirCompiler::with_options(!self.config.no_optimize);
        let compile_result = match mir_compiler.compile_normal(request) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("❌ MIR compilation error: {}", e);
                process::exit(1);
            }
        };

        let groups = self.config.as_groups();
        if groups.debug.verify_mir {
            println!("🔍 Verifying MIR...");
            match &compile_result.verification_result {
                Ok(()) => println!("✅ MIR verification passed!"),
                Err(errors) => {
                    eprintln!("❌ MIR verification failed:");
                    for error in errors {
                        eprintln!("  • {}", error);
                    }
                    process::exit(1);
                }
            }
        }

        if groups.debug.dump_mir {
            let mut printer = if groups.debug.mir_verbose {
                MirPrinter::verbose()
            } else {
                MirPrinter::new()
            };
            if groups.debug.mir_verbose_effects {
                printer.set_show_effects_inline(true);
            }
            println!("🚀 MIR Output for {}:", filename);
            println!("{}", printer.print_module(&compile_result.module));
        }

        crate::runner::modes::common_util::emit_direct::maybe_emit_mir_json_and_exit(
            groups.emit.emit_mir_json.as_deref(),
            &compile_result.verification_result,
            "mir",
            false,
            |out_path| {
                crate::runner::mir_json_emit::emit_mir_json_for_harness(
                    &compile_result.module,
                    out_path,
                )
            },
        );

        crate::runner::modes::common_util::emit_direct::maybe_emit_exe_and_exit(
            groups.emit.emit_exe.as_deref(),
            &compile_result.verification_result,
            "mir",
            false,
            |exe_out| {
                crate::runner::modes::common_util::exec::ny_llvmc_emit_exe_lib(
                    &compile_result.module,
                    exe_out,
                    groups.emit.emit_exe_nyrt.as_deref(),
                    groups.emit.emit_exe_libs.as_deref(),
                )
            },
        );

        // Normal `backend=mir` execution should run the compiled MIR module.
        // The emit/diagnostic routes above exit early, so only the plain
        // interpreter path reaches here.
        if !groups.debug.dump_mir && !groups.debug.verify_mir {
            std::process::exit(self.execute_mir_module_quiet_exit(&compile_result.module));
        }
    }

    /// Minimal MIR emit mode for perf-sensitive startup measurements.
    ///
    /// This path intentionally skips using/prelude resolution and plugin init.
    /// It keeps the lightweight parser-side normalization needed by current
    /// benchmark fixtures, then compiles and writes MIR JSON directly.
    pub(crate) fn execute_mir_json_minimal(&self, filename: &str, out_path: &str) {
        let code = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Error reading file {}: {}", filename, e);
                process::exit(1);
            }
        };

        let prepared = match crate::runner::modes::common_util::source_hint::prepare_source_minimal(
            &code, filename,
        ) {
            Ok(prepared) => prepared,
            Err(e) => {
                eprintln!("❌ {}", e);
                process::exit(1);
            }
        };

        let ast = match self.parse_source(&prepared) {
            Ok(ast) => ast,
            Err(e) => {
                crate::runner::modes::common_util::diag::print_parse_error_with_context(
                    filename, &prepared, &e,
                );
                process::exit(1);
            }
        };

        let ast = if crate::r#macro::enabled() {
            crate::r#macro::maybe_expand_and_dump(&ast, false)
        } else {
            ast
        };

        let request = match NormalCompileRequestV1::for_minimal_mir_json(ast, Some(filename)) {
            Ok(request) => request,
            Err(rejected) => {
                eprintln!("❌ MIR compilation error: {}", rejected);
                process::exit(1);
            }
        };
        let mut mir_compiler = MirCompiler::with_options(!self.config.no_optimize);
        let compile_result = match mir_compiler.compile_normal(request) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("❌ MIR compilation error: {}", e);
                process::exit(1);
            }
        };

        let out = std::path::Path::new(out_path);
        if let Err(e) =
            crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(&compile_result.module, out)
        {
            eprintln!("❌ MIR JSON emit error: {}", e);
            process::exit(1);
        }
        println!("MIR JSON written: {}", out.display());
        process::exit(0);
    }
}
