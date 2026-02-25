use super::super::NyashRunner;
use nyash_rust::{
    mir::{MirCompiler, MirPrinter},
    parser::NyashParser,
};
use std::{fs, process};

impl NyashRunner {
    /// Execute MIR compilation and processing mode (split)
    pub(crate) fn execute_mir_mode(&self, filename: &str) {
        // Read the file
        let code = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Error reading file {}: {}", filename, e);
                process::exit(1);
            }
        };

        // Parse to AST
        let ast = match NyashParser::parse_from_string(&code) {
            Ok(ast) => ast,
            Err(e) => {
                crate::runner::modes::common_util::diag::print_parse_error_with_context(
                    filename, &code, &e,
                );
                process::exit(1);
            }
        };
        // Macro expansion (env-gated)
        let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);

        // Compile to MIR (opt passes configurable)
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

        let groups = self.config.as_groups();
        // Verify MIR if requested
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

        // Dump MIR if requested
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

        // Emit MIR JSON if requested and exit
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

        // Emit native executable via ny-llvmc (crate) and exit
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
    }
}
