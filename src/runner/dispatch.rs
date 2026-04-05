/*!
 * Runner dispatch helpers — execute MIR module and report result
 */

use super::*;
use nyash_rust::parser::NyashParser;
use std::{fs, process};

/// Thin file dispatcher: select backend and delegate to mode executors
pub(crate) fn execute_file_with_backend(runner: &NyashRunner, filename: &str) {
    // Note: hv1 direct route is now handled at main.rs entry point (before NyashRunner creation).
    // This function is only called after plugins and runner initialization have already occurred.

    // Selfhost pipeline (Ny -> JSON v0) — consolidated env toggle
    // Primary: NYASH_USE_NY_COMPILER=0|1; legacy disables accepted with deprecation warning
    if crate::config::env::use_ny_compiler() {
        if runner.try_run_selfhost_pipeline(filename) {
            return;
        } else {
            crate::cli_v!(
                "[ny-compiler] fallback to default path (MVP unavailable for this input)"
            );
        }
    }

    // Direct v0 bridge when requested via CLI/env
    let groups = runner.config.as_groups();
    // Diagnostic/Exec: accept MIR JSON file direct (experimental; default OFF)
    if let Some(path) = groups.parser.mir_json_file.as_ref() {
        // Phase 90-A: fs 系移行
        let ring0 = crate::runtime::ring0::get_global_ring0();
        match ring0.fs.read_to_string(std::path::Path::new(path)) {
            Ok(text) => {
                // Try schema v1 first (preferred by emitter)
                match crate::runner::json_v1_bridge::try_parse_v1_to_module(&text) {
                    Ok(Some(module)) => {
                        crate::cli_v!(
                            "[mir-json] schema=v1 executing {} (len={})",
                            path,
                            text.len()
                        );
                        let rc = runner.execute_mir_module_quiet_exit(&module);
                        std::process::exit(rc);
                    }
                    Ok(None) => {
                        // Not v1 schema; attempt minimal v0 loader
                        if text.contains("\"functions\"") && text.contains("\"blocks\"") {
                            match crate::runner::mir_json_v0::parse_mir_v0_to_module(&text) {
                                Ok(module) => {
                                    crate::cli_v!(
                                        "[mir-json] schema=v0 executing {} (len={})",
                                        path,
                                        text.len()
                                    );
                                    let rc = runner.execute_mir_module_quiet_exit(&module);
                                    std::process::exit(rc);
                                }
                                Err(e) => {
                                    eprintln!("❌ MIR JSON v0 parse error: {}", e);
                                    std::process::exit(1);
                                }
                            }
                        }
                        eprintln!("❌ MIR JSON invalid or unsupported shape: {}", path);
                        std::process::exit(1);
                    }
                    Err(e) => {
                        eprintln!("❌ MIR JSON parse error (v1): {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error reading MIR JSON {}: {}", path, e);
                std::process::exit(1);
            }
        }
    }
    // AST dump mode
    if groups.debug.dump_ast {
        println!("🧠 Hakorune AST Dump - Processing file: {}", filename);
        let code = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Error reading file {}: {}", filename, e);
                process::exit(1);
            }
        };
        let ast = match NyashParser::parse_from_string(&code) {
            Ok(ast) => ast,
            Err(e) => {
                crate::runner::modes::common_util::diag::print_parse_error_with_context(
                    filename, &code, &e,
                );
                process::exit(1);
            }
        };
        // Optional macro expansion dump (no-op expansion for now)
        let ast2 = if crate::r#macro::enabled() {
            let a = crate::r#macro::maybe_expand_and_dump(&ast, true);
            crate::runner::modes::macro_child::normalize_core_pass(&a)
        } else {
            ast.clone()
        };
        println!("{:#?}", ast2);
        return;
    }

    // Dump expanded AST as JSON v0 and exit
    if runner.config.dump_expanded_ast_json {
        let code = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Error reading file {}: {}", filename, e);
                process::exit(1);
            }
        };
        let ast = match NyashParser::parse_from_string(&code) {
            Ok(ast) => ast,
            Err(e) => {
                crate::runner::modes::common_util::diag::print_parse_error_with_context(
                    filename, &code, &e,
                );
                process::exit(1);
            }
        };
        let expanded = if crate::r#macro::enabled() {
            let a = crate::r#macro::maybe_expand_and_dump(&ast, false);
            crate::runner::modes::macro_child::normalize_core_pass(&a)
        } else {
            ast
        };
        let j = crate::r#macro::ast_json::ast_to_json_roundtrip(&expanded);
        println!("{}", j.to_string());
        return;
    }

    // MIR dump/verify
    if groups.debug.dump_mir || groups.debug.verify_mir {
        crate::cli_v!(
            "🚀 Hakorune MIR Compiler - Processing file: {} 🚀",
            filename
        );
        runner.execute_mir_mode(filename);
        return;
    }

    if groups.emit.emit_wat.is_some() {
        #[cfg(feature = "wasm-backend")]
        {
            let output_path = groups
                .emit
                .emit_wat
                .as_ref()
                .expect("emit_wat presence checked");
            runner.execute_emit_wat_mode(filename, output_path);
            return;
        }
        #[cfg(not(feature = "wasm-backend"))]
        {
            eprintln!("❌ WAT emit not available. Please rebuild with: cargo build --features wasm-backend");
            process::exit(1);
        }
    }

    // WASM / AOT (feature-gated)
    if groups.compile_wasm {
        #[cfg(feature = "wasm-backend")]
        {
            runner.execute_wasm_mode(filename);
            return;
        }
        #[cfg(not(feature = "wasm-backend"))]
        {
            eprintln!("❌ WASM backend not available. Please rebuild with: cargo build --features wasm-backend");
            process::exit(1);
        }
    }
    if groups.compile_native {
        #[cfg(feature = "cranelift-jit")]
        {
            runner.execute_aot_mode(filename);
            return;
        }
        #[cfg(not(feature = "cranelift-jit"))]
        {
            eprintln!("❌ Native AOT compilation requires Cranelift. Please rebuild: cargo build --features cranelift-jit");
            process::exit(1);
        }
    }

    // Backend selection
    if std::env::var("NYASH_EMIT_MIR_TRACE").ok().as_deref() == Some("1") {
        eprintln!(
            "[dispatch] backend={} file={} path=backend-select",
            groups.backend.backend, filename
        );
    }
    match groups.backend.backend.as_str() {
        "mir" => {
            crate::cli_v!(
                "🚀 Hakorune MIR Interpreter - Executing file: {} 🚀",
                filename
            );
            runner.execute_mir_mode(filename);
        }
        "vm" => {
            crate::cli_v!(
                "🚀 Hakorune Legacy VM Keep/Debug Override (explicit only) - Executing file: {} 🚀",
                filename
            );
            if !super::route_orchestrator::execute_vm_family_route(runner, "vm", filename) {
                eprintln!("❌ VM route orchestration error: backend=vm");
                std::process::exit(2);
            }
        }
        "vm-hako" => {
            crate::cli_v!(
                "🚀 Hakorune Explicit VM-Hako Reference Override (explicit only) - Executing file: {} 🚀",
                filename
            );
            if !super::route_orchestrator::execute_vm_family_route(runner, "vm-hako", filename) {
                eprintln!("❌ VM route orchestration error: backend=vm-hako");
                std::process::exit(2);
            }
        }
        #[cfg(feature = "cranelift-jit")]
        "jit-direct" => {
            crate::cli_v!(
                "⚡ Hakorune JIT-Direct Backend - Executing file: {} ⚡",
                filename
            );
            #[cfg(feature = "cranelift-jit")]
            {
                // Use independent JIT-direct runner method (no VM execute loop)
                runner.run_file_jit_direct(filename);
            }
            #[cfg(not(feature = "cranelift-jit"))]
            {
                eprintln!("❌ Cranelift backend not available. Please rebuild with: cargo build --features cranelift-jit");
                process::exit(1);
            }
        }
        "llvm" => {
            crate::cli_v!("⚡ Hakorune LLVM Backend - Executing file: {} ⚡", filename);
            runner.execute_llvm_mode(filename);
        }
        other => {
            eprintln!(
                "❌ Unknown backend: {}. Use mainline/default mir, or an explicit override family (product/native: llvm; legacy keep/debug: vm; reference/conformance: vm-hako).",
                other
            );
            std::process::exit(2);
        }
    }
}

impl NyashRunner {
    /// Execute a MIR module quietly and return an OS exit code derived from the
    /// program's return value (Integer/Bool/Float). Strings and other values map to 0.
    /// - Integer: value as i64, truncated to 0..=255 (two's complement wrap)
    /// - Bool:    true=1, false=0
    /// - Float:   floor toward zero (as i64), truncated to 0..=255
    pub(crate) fn execute_mir_module_quiet_exit(&self, module: &crate::mir::MirModule) -> i32 {
        use crate::backend::MirInterpreter;
        use crate::box_trait::{BoolBox, IntegerBox, StringBox};
        use crate::boxes::FloatBox;
        use crate::mir::MirType;

        fn to_rc(val: i64) -> i32 {
            let m = ((val % 256) + 256) % 256; // normalize into 0..=255
            m as i32
        }

        let mut interp = MirInterpreter::new();
        match interp.execute_module(module) {
            Ok(result) => {
                if let Some(func) = module.functions.get("main") {
                    match &func.signature.return_type {
                        MirType::Integer => {
                            if let Some(ib) = result.as_any().downcast_ref::<IntegerBox>() {
                                return to_rc(ib.value);
                            }
                        }
                        MirType::Bool => {
                            if let Some(bb) = result.as_any().downcast_ref::<BoolBox>() {
                                return if bb.value { 1 } else { 0 };
                            }
                        }
                        MirType::Float => {
                            if let Some(fb) = result.as_any().downcast_ref::<FloatBox>() {
                                return to_rc(fb.value as i64);
                            }
                            if let Some(ib) = result.as_any().downcast_ref::<IntegerBox>() {
                                return to_rc(ib.value);
                            }
                        }
                        _ => {}
                    }
                    // Fallbacks by inspecting boxed value kinds (robust to imprecise return types)
                    if let Some(ib) = result.as_any().downcast_ref::<IntegerBox>() {
                        return to_rc(ib.value);
                    }
                    if let Some(bb) = result.as_any().downcast_ref::<BoolBox>() {
                        return if bb.value { 1 } else { 0 };
                    }
                    if let Some(fb) = result.as_any().downcast_ref::<FloatBox>() {
                        return to_rc(fb.value as i64);
                    }
                    if let Some(_sb) = result.as_any().downcast_ref::<StringBox>() {
                        return 0; // strings do not define rc semantics yet
                    }
                }
                // Global fallbacks when signature is missing or imprecise
                if let Some(ib) = result.as_any().downcast_ref::<IntegerBox>() {
                    return to_rc(ib.value);
                }
                if let Some(bb) = result.as_any().downcast_ref::<BoolBox>() {
                    return if bb.value { 1 } else { 0 };
                }
                if let Some(fb) = result.as_any().downcast_ref::<FloatBox>() {
                    return to_rc(fb.value as i64);
                }
                if let Some(_sb) = result.as_any().downcast_ref::<StringBox>() {
                    return 0;
                }
                0
            }
            Err(e) => {
                // Quiet-exit path still prints a single-line error to keep smoke logs actionable.
                // (stdout is still reserved for program output; this goes to stderr.)
                let msg = e.to_string().replace('\n', "\\n");
                crate::runtime::get_global_ring0()
                    .log
                    .error(&format!("[vm/error] {msg}"));
                1
            }
        }
    }
    pub(crate) fn execute_mir_module(&self, module: &crate::mir::MirModule) {
        // If CLI requested MIR JSON emit, write to file and exit immediately.
        let groups = self.config.as_groups();
        if let Some(path) = groups.emit.emit_mir_json.as_ref() {
            let p = std::path::Path::new(path);
            if let Err(e) = crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(module, p) {
                eprintln!("❌ MIR JSON emit error: {}", e);
                std::process::exit(1);
            }
            println!("MIR JSON written: {}", p.display());
            std::process::exit(0);
        }
        // If CLI requested EXE emit, generate JSON then invoke ny-llvmc to link NyRT and exit.
        if let Some(exe_out) = groups.emit.emit_exe.as_ref() {
            if let Err(e) = crate::runner::modes::common_util::exec::ny_llvmc_emit_exe_bin(
                module,
                exe_out,
                groups.emit.emit_exe_nyrt.as_deref(),
                groups.emit.emit_exe_libs.as_deref(),
            ) {
                eprintln!("❌ {}", e);
                std::process::exit(1);
            }
            println!("EXE written: {}", exe_out);
            std::process::exit(0);
        }
        use crate::backend::MirInterpreter;
        use crate::box_trait::{BoolBox, IntegerBox, StringBox};
        use crate::boxes::FloatBox;
        use crate::mir::MirType;

        let mut interp = MirInterpreter::new();
        match interp.execute_module(module) {
            Ok(result) => {
                println!("✅ MIR interpreter execution completed!");
                if let Some(func) = module.functions.get("main") {
                    let (ety, sval) = match &func.signature.return_type {
                        MirType::Float => {
                            if let Some(fb) = result.as_any().downcast_ref::<FloatBox>() {
                                ("Float", format!("{}", fb.value))
                            } else if let Some(ib) = result.as_any().downcast_ref::<IntegerBox>() {
                                ("Float", format!("{}", ib.value as f64))
                            } else {
                                ("Float", result.to_string_box().value)
                            }
                        }
                        MirType::Integer => {
                            if let Some(ib) = result.as_any().downcast_ref::<IntegerBox>() {
                                ("Integer", ib.value.to_string())
                            } else {
                                ("Integer", result.to_string_box().value)
                            }
                        }
                        MirType::Bool => {
                            if let Some(bb) = result.as_any().downcast_ref::<BoolBox>() {
                                ("Bool", bb.value.to_string())
                            } else if let Some(ib) = result.as_any().downcast_ref::<IntegerBox>() {
                                ("Bool", (ib.value != 0).to_string())
                            } else {
                                ("Bool", result.to_string_box().value)
                            }
                        }
                        MirType::String => {
                            if let Some(sb) = result.as_any().downcast_ref::<StringBox>() {
                                ("String", sb.value.clone())
                            } else {
                                ("String", result.to_string_box().value)
                            }
                        }
                        _ => (result.type_name(), result.to_string_box().value),
                    };
                    println!("ResultType(MIR): {}", ety);
                    println!("Result: {}", sval);
                } else {
                    println!("Result: {:?}", result);
                }
            }
            Err(e) => {
                eprintln!("❌ MIR interpreter error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
