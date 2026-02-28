use super::super::NyashRunner;
#[cfg(feature = "wasm-backend")]
use crate::config::env::WasmRoutePolicyMode;
#[cfg(feature = "wasm-backend")]
use nyash_rust::{
    backend::wasm::{compile_hako_native_shape_emit, WasmBackend},
    mir::MirCompiler,
    parser::NyashParser,
};
#[cfg(feature = "wasm-backend")]
use std::{fs, process};

#[cfg(feature = "wasm-backend")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WasmCompileRoute {
    HakoDefaultBridge,
    RustNativeForced,
}

#[cfg(feature = "wasm-backend")]
fn select_wasm_compile_route(policy: WasmRoutePolicyMode) -> WasmCompileRoute {
    match policy {
        WasmRoutePolicyMode::Default => WasmCompileRoute::HakoDefaultBridge,
        WasmRoutePolicyMode::RustNative => WasmCompileRoute::RustNativeForced,
    }
}

#[cfg(feature = "wasm-backend")]
fn wasm_route_policy_name(policy: WasmRoutePolicyMode) -> &'static str {
    match policy {
        WasmRoutePolicyMode::Default => "default",
        WasmRoutePolicyMode::RustNative => "rust_native",
    }
}

#[cfg(feature = "wasm-backend")]
fn wasm_route_name_for_plan(plan: &'static str) -> &'static str {
    match plan {
        "native-shape-table" => "hako_native",
        "bridge-rust-backend" => "rust_native",
        _ => panic!(
            "[freeze:contract][wasm/route-trace] unknown plan '{}': route must be hako_native|rust_native",
            plan
        ),
    }
}

#[cfg(feature = "wasm-backend")]
fn emit_wasm_route_trace_with_route(
    policy: WasmRoutePolicyMode,
    plan: &'static str,
    shape_id: Option<&str>,
) {
    let shape = shape_id.unwrap_or("-");
    eprintln!(
        "[wasm/route-trace] policy={} plan={} shape_id={} route={}",
        wasm_route_policy_name(policy),
        plan,
        shape,
        wasm_route_name_for_plan(plan)
    );
}

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
        let compile_route = select_wasm_compile_route(route_policy);
        let compile_result = match compile_route {
            // P5-min6: default(hako-lane) tries native shape-table helper first.
            // Fallback to bridge route only when outside native shape-table contract.
            WasmCompileRoute::HakoDefaultBridge => {
                match compile_hako_native_shape_emit(&mir_module) {
                    Ok(Some(emitted)) => {
                        emit_wasm_route_trace_with_route(
                            route_policy,
                            "native-shape-table",
                            Some(emitted.shape_id),
                        );
                        Ok(emitted.bytes)
                    }
                    Ok(None) => {
                        emit_wasm_route_trace_with_route(
                            route_policy,
                            "bridge-rust-backend",
                            None,
                        );
                        wasm_backend.compile_module(mir_module)
                    }
                    Err(err) => Err(err),
                }
            }
            // Freeze-1: explicit rust_native route for parity/diagnostic only.
            WasmCompileRoute::RustNativeForced => {
                emit_wasm_route_trace_with_route(route_policy, "bridge-rust-backend", None);
                wasm_backend.compile_module(mir_module)
            }
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

#[cfg(all(test, feature = "wasm-backend"))]
mod tests {
    use super::*;

    #[test]
    fn wasm_compile_route_policy_default_maps_to_hako_bridge_contract() {
        let route = select_wasm_compile_route(WasmRoutePolicyMode::Default);
        assert_eq!(route, WasmCompileRoute::HakoDefaultBridge);
    }

    #[test]
    fn wasm_compile_route_policy_rust_native_maps_to_forced_rust_contract() {
        let route = select_wasm_compile_route(WasmRoutePolicyMode::RustNative);
        assert_eq!(route, WasmCompileRoute::RustNativeForced);
    }

    #[test]
    fn wasm_route_policy_name_contract() {
        assert_eq!(wasm_route_policy_name(WasmRoutePolicyMode::Default), "default");
        assert_eq!(
            wasm_route_policy_name(WasmRoutePolicyMode::RustNative),
            "rust_native"
        );
    }

    #[test]
    fn wasm_route_name_for_plan_contract() {
        assert_eq!(wasm_route_name_for_plan("native-shape-table"), "hako_native");
        assert_eq!(wasm_route_name_for_plan("bridge-rust-backend"), "rust_native");
    }
}
