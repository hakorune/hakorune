use super::super::NyashRunner;
#[cfg(feature = "wasm-backend")]
use nyash_rust::{parser::NyashParser, mir::MirCompiler, backend::wasm::WasmBackend};
#[cfg(feature = "wasm-backend")]
use std::{fs, process};

impl NyashRunner {
    #[cfg(feature = "wasm-backend")]
    fn compile_file_to_wat_text(&self, filename: &str) -> String {
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
        // Keep WASM emit route macro-free for deterministic parity checks against
        // fixture-level compile_to_wat contracts.

        // Compile to MIR
        let mut mir_compiler = MirCompiler::new();
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

        // Compile to WAT text
        let mut wasm_backend = WasmBackend::new();
        match wasm_backend.compile_to_wat(compile_result.module) {
            Ok(wat) => wat,
            Err(e) => {
                eprintln!("❌ WASM compilation error: {}", e);
                process::exit(1);
            }
        }
    }

    /// Execute WASM compilation mode (split)
    #[cfg(feature = "wasm-backend")]
    pub(crate) fn execute_wasm_mode(&self, filename: &str) {
        let wat_text = self.compile_file_to_wat_text(filename);

        // Determine output file
        let groups = self.config.as_groups();
        let output = groups.output_file.as_deref().unwrap_or_else(|| {
            if filename.ends_with(".hako") {
                filename.strip_suffix(".hako").unwrap_or(filename)
            } else if filename.ends_with(".nyash") {
                filename.strip_suffix(".nyash").unwrap_or(filename)
            } else {
                filename
            }
        });
        let output_file = format!("{}.wat", output);

        match fs::write(&output_file, wat_text) {
            Ok(()) => { println!("✅ WASM compilation successful!\nOutput written to: {}", output_file); },
            Err(e) => { eprintln!("❌ Error writing WASM file {}: {}", output_file, e); process::exit(1); }
        }
    }

    /// Emit WAT to explicit file path and exit.
    #[cfg(feature = "wasm-backend")]
    pub(crate) fn execute_emit_wat_mode(&self, filename: &str, output_file: &str) {
        let wat_text = self.compile_file_to_wat_text(filename);
        match fs::write(output_file, wat_text) {
            Ok(()) => {
                println!(
                    "✅ WAT emit successful!\nOutput written to: {}",
                    output_file
                );
            }
            Err(e) => {
                eprintln!("❌ Error writing WAT file {}: {}", output_file, e);
                process::exit(1);
            }
        }
    }
}
