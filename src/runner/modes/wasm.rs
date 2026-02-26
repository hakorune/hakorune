use super::super::NyashRunner;
#[cfg(feature = "wasm-backend")]
use crate::config::env::WasmRoutePolicyMode;
#[cfg(feature = "wasm-backend")]
use nyash_rust::{parser::NyashParser, mir::MirCompiler, backend::wasm::WasmBackend};
#[cfg(feature = "wasm-backend")]
use std::{fs, process};

impl NyashRunner {
    #[cfg(feature = "wasm-backend")]
    fn parse_ast_for_wasm_emit(&self, filename: &str, code: &str) -> nyash_rust::ast::ASTNode {
        let ast = match NyashParser::parse_from_string(code) {
            Ok(ast) => ast,
            Err(e) => {
                crate::runner::modes::common_util::diag::print_parse_error_with_context(
                    filename, code, &e,
                );
                process::exit(1);
            }
        };
        // Keep WASM emit route macro-free for deterministic parity checks against
        // fixture-level compile_to_wat contracts.
        ast
    }

    #[cfg(feature = "wasm-backend")]
    fn compile_file_to_mir_module(&self, filename: &str) -> nyash_rust::mir::MirModule {
        // Read the file
        let code = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Error reading file {}: {}", filename, e);
                process::exit(1);
            }
        };

        // Parse to AST
        let ast = self.parse_ast_for_wasm_emit(filename, &code);

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

        compile_result.module
    }

    #[cfg(feature = "wasm-backend")]
    fn compile_file_to_wat_text(&self, filename: &str) -> String {
        // Compile to WAT text
        let mir_module = self.compile_file_to_mir_module(filename);
        let mut wasm_backend = WasmBackend::new();
        match wasm_backend.compile_to_wat(mir_module) {
            Ok(wat) => wat,
            Err(e) => {
                eprintln!("❌ WASM compilation error: {}", e);
                process::exit(1);
            }
        }
    }

    #[cfg(feature = "wasm-backend")]
    fn compile_file_to_wasm_bytes(&self, filename: &str) -> Vec<u8> {
        let mir_module = self.compile_file_to_mir_module(filename);
        let mut wasm_backend = WasmBackend::new();
        let route_policy = crate::config::env::wasm_route_policy_mode();
        let compile_result = match route_policy {
            // Current default keeps Rust backend route while P5 cutover is staged.
            WasmRoutePolicyMode::Default => wasm_backend.compile_module(mir_module),
            // Explicit compatibility lane for phased default cutover.
            WasmRoutePolicyMode::LegacyWasmRust => wasm_backend.compile_module(mir_module),
        };
        match compile_result {
            Ok(wasm) => wasm,
            Err(e) => {
                eprintln!("❌ WASM compilation error: {}", e);
                process::exit(1);
            }
        }
    }

    /// Execute WASM compilation mode (split)
    #[cfg(feature = "wasm-backend")]
    pub(crate) fn execute_wasm_mode(&self, filename: &str) {
        let wasm_bytes = self.compile_file_to_wasm_bytes(filename);

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
        let output_file = format!("{}.wasm", output);

        match fs::write(&output_file, wasm_bytes) {
            Ok(()) => {
                println!(
                    "✅ WASM binary compilation successful!\nOutput written to: {}",
                    output_file
                );
            }
            Err(e) => {
                eprintln!("❌ Error writing WASM file {}: {}", output_file, e);
                process::exit(1);
            }
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
