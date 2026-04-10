use super::super::NyashRunner;
use nyash_rust::{
    mir::{MirCompiler, MirPrinter},
    parser::NyashParser,
};
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

        let ast = match NyashParser::parse_from_string(&prepared.code) {
            Ok(ast) => ast,
            Err(e) => {
                crate::runner::modes::common_util::diag::print_parse_error_with_context(
                    filename,
                    &prepared.code,
                    &e,
                );
                process::exit(1);
            }
        };
        let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);

        let mut mir_compiler = MirCompiler::with_options(!self.config.no_optimize);
        let compile_result =
            match crate::runner::modes::common_util::source_hint::compile_with_source_hint_and_imports(
                &mut mir_compiler,
                ast,
                Some(filename),
                prepared.imports,
            ) {
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

        let ast = match NyashParser::parse_from_string(&prepared) {
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

        let mut mir_compiler = MirCompiler::with_options(!self.config.no_optimize);
        let compile_result =
            match crate::runner::modes::common_util::source_hint::compile_with_source_hint(
                &mut mir_compiler,
                ast,
                Some(filename),
            ) {
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
