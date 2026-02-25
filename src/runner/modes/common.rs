use super::super::NyashRunner;
use crate::runner::json_v0_bridge;
#[cfg(not(feature = "interpreter-legacy"))]
use nyash_rust::parser::NyashParser;
#[cfg(feature = "interpreter-legacy")]
use nyash_rust::{interpreter::NyashInterpreter, parser::NyashParser};
// Use the library crate's plugin init module rather than the bin crate root
use crate::cli_v;
use crate::runner::pipeline::{resolve_using_target, suggest_in_base};
use crate::runner::trace::cli_verbose;
use crate::runtime::get_global_ring0;
use std::io::Read;
use std::process::Stdio;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{fs, process};

// (moved) suggest_in_base is now in runner/pipeline.rs

#[cfg(feature = "interpreter-legacy")]
impl NyashRunner {
    // legacy run_file_legacy removed (was commented out)

    /// Helper: try external selfhost compiler EXE to parse Ny -> JSON v0 and return MIR module
    /// Returns Some(module) on success, None on failure (timeout/invalid output/missing exe)
    fn exe_try_parse_json_v0(
        &self,
        filename: &str,
        timeout_ms: u64,
    ) -> Option<nyash_rust::mir::MirModule> {
        super::common_util::selfhost_exe::exe_try_parse_json_v0(filename, timeout_ms)
    }

    /// Phase-15.3: Attempt Ny compiler pipeline (Ny -> JSON v0 via Ny program), then execute MIR
    pub(crate) fn try_run_ny_compiler_pipeline(&self, filename: &str) -> bool {
        // Delegate to centralized selfhost pipeline to avoid drift
        self.try_run_selfhost_pipeline(filename)
    }

    /// Execute Nyash file with interpreter (common helper)
    pub(crate) fn execute_nyash_file(&self, filename: &str) {
        let quiet_pipe = crate::config::env::env_bool("NYASH_JSON_ONLY");
        // Ensure runtime and plugins are initialized via unified helper (idempotent)
        let groups = self.config.as_groups();
        self.init_runtime_and_plugins(&groups);
        // Read the file
        let code = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Error reading file {}: {}", filename, e);
                process::exit(1);
            }
        };
        if crate::config::env::cli_verbose() && !quiet_pipe {
            println!("📝 File contents:\n{}", code);
            println!("\n🚀 Parsing and executing...\n");
        }

        // Using handling: AST-based prelude collection (legacy inlining removed)
        let use_ast = crate::config::env::using_ast_enabled();
        let mut code_ref: &str = &code;
        let cleaned_code_owned;
        let mut prelude_asts: Vec<nyash_rust::ast::ASTNode> = Vec::new();
        if crate::config::env::enable_using() {
            match crate::runner::modes::common_util::resolve::resolve_prelude_paths_profiled(
                self, &code, filename,
            ) {
                Ok((clean, paths)) => {
                    cleaned_code_owned = clean;
                    code_ref = &cleaned_code_owned;
                    if !paths.is_empty() && !use_ast {
                        eprintln!("❌ using: AST prelude merge is disabled in this profile. Enable NYASH_USING_AST=1 or remove 'using' lines.");
                        std::process::exit(1);
                    }
                    if use_ast && !paths.is_empty() {
                        match crate::runner::modes::common_util::resolve::parse_preludes_to_asts(
                            self, &paths,
                        ) {
                            Ok(v) => prelude_asts = v,
                            Err(e) => {
                                if e.starts_with("[freeze:contract][module_registry]") {
                                    eprintln!("{}", e);
                                } else {
                                    eprintln!("❌ {}", e);
                                }
                                std::process::exit(1);
                            }
                        }
                    }
                }
                Err(e) => {
                    if e.starts_with("[freeze:contract][module_registry]") {
                        eprintln!("{}", e);
                    } else {
                        eprintln!("❌ {}", e);
                    }
                    std::process::exit(1);
                }
            }
        }
        // Optional dev sugar: @name[:T] = expr → local name[:T] = expr (line-head only)
        let preexpanded_owned;
        {
            let s = crate::runner::modes::common_util::resolve::preexpand_at_local(code_ref);
            preexpanded_owned = s;
            code_ref = &preexpanded_owned;
        }

        // Parse the code with debug fuel limit
        let groups = self.config.as_groups();
        get_global_ring0().log.debug(&format!(
            "🔍 DEBUG: Starting parse with fuel: {:?}...",
            groups.debug.debug_fuel
        ));
        let main_ast =
            match NyashParser::parse_from_string_with_fuel(code_ref, groups.debug.debug_fuel) {
                Ok(ast) => {
                    get_global_ring0()
                        .log
                        .debug("🔍 DEBUG: Parse completed, AST created");
                    ast
                }
                Err(e) => {
                    crate::runner::modes::common_util::diag::print_parse_error_with_context(
                        filename, code_ref, &e,
                    );
                    process::exit(1);
                }
            };
        // When using AST prelude mode, combine prelude ASTs + main AST into one Program
        let ast = if use_ast && !prelude_asts.is_empty() {
            crate::runner::modes::common_util::resolve::merge_prelude_asts_with_main(
                prelude_asts,
                &main_ast,
            )
        } else {
            main_ast
        };

        // Optional: dump AST statement kinds for quick diagnostics
        if crate::config::env::env_bool("NYASH_AST_DUMP") {
            use nyash_rust::ast::ASTNode;
            get_global_ring0().log.debug("[ast] dump start");
            if let ASTNode::Program { statements, .. } = &ast {
                for (i, st) in statements.iter().enumerate().take(50) {
                    let kind = match st {
                        ASTNode::BoxDeclaration {
                            is_static, name, ..
                        } => {
                            if *is_static {
                                format!("StaticBox({})", name)
                            } else {
                                format!("Box({})", name)
                            }
                        }
                        ASTNode::FunctionDeclaration { name, .. } => format!("FuncDecl({})", name),
                        ASTNode::FunctionCall { name, .. } => format!("FuncCall({})", name),
                        ASTNode::MethodCall { method, .. } => format!("MethodCall({})", method),
                        ASTNode::ScopeBox { .. } => "ScopeBox".to_string(),
                        ASTNode::ImportStatement { path, .. } => format!("Import({})", path),
                        ASTNode::UsingStatement { namespace_name, .. } => {
                            format!("Using({})", namespace_name)
                        }
                        _ => format!("{:?}", st),
                    };
                    get_global_ring0()
                        .log
                        .debug(&format!("[ast] {}: {}", i, kind));
                }
            }
            get_global_ring0().log.debug("[ast] dump end");
        }

        // Stage-0: import loader (top-level only) — resolve path and register in modules registry
        if let nyash_rust::ast::ASTNode::Program { statements, .. } = &ast {
            for st in statements {
                if let nyash_rust::ast::ASTNode::ImportStatement { path, alias, .. } = st {
                    // resolve path relative to current file if not absolute
                    let mut p = std::path::PathBuf::from(path);
                    if p.is_relative() {
                        if let Some(dir) = std::path::Path::new(filename).parent() {
                            p = dir.join(&p);
                        }
                    }
                    let exists = p.exists();
                    if !exists {
                        if crate::config::env::env_bool("NYASH_USING_STRICT") {
                            eprintln!(
                                "❌ import: path not found: {} (from {})",
                                p.display(),
                                filename
                            );
                            process::exit(1);
                        } else if crate::config::env::cli_verbose()
                            || crate::config::env::env_bool("NYASH_IMPORT_TRACE")
                        {
                            crate::runtime::get_global_ring0().log.debug(&format!(
                                "[import] path not found (continuing): {}",
                                p.display()
                            ));
                        }
                    }
                    let key = if let Some(a) = alias {
                        a.clone()
                    } else {
                        std::path::Path::new(path)
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or(path)
                            .to_string()
                    };
                    let value = if exists {
                        p.to_string_lossy().to_string()
                    } else {
                        path.clone()
                    };
                    let sb = nyash_rust::box_trait::StringBox::new(value);
                    nyash_rust::runtime::modules_registry::set(key, Box::new(sb));
                }
            }
        }

        if crate::config::env::cli_verbose() && !quiet_pipe {
            if crate::config::env::cli_verbose() {
                println!("✅ Parse successful!");
            }
        }

        // Execute the AST
        let mut interpreter = NyashInterpreter::new();
        get_global_ring0()
            .log
            .debug("🔍 DEBUG: Starting execution...");
        match interpreter.execute(ast) {
            Ok(result) => {
                if crate::config::env::cli_verbose() && !quiet_pipe {
                    if crate::config::env::cli_verbose() {
                        println!("✅ Execution completed successfully!");
                    }
                }
                // Normalize display via shared semantics: prefer numeric, then string, then fallback.
                let disp = nyash_rust::runtime::semantics::coerce_to_i64(result.as_ref())
                    .map(|i| i.to_string())
                    .or_else(|| nyash_rust::runtime::semantics::coerce_to_string(result.as_ref()))
                    .unwrap_or_else(|| result.to_string_box().value);
                if !quiet_pipe {
                    println!("Result: {}", disp);
                }
            }
            Err(e) => {
                eprintln!("❌ Runtime error:\n{}", e.detailed_message(Some(&code)));
                process::exit(1);
            }
        }
    }
}

#[cfg(not(feature = "interpreter-legacy"))]
impl NyashRunner {
    /// Interpreter backend is disabled in default builds. Use `--backend vm` or `--backend llvm`.
    pub(crate) fn execute_nyash_file(&self, _filename: &str) {
        eprintln!("❌ Interpreter backend (AST) is disabled. Build with --features interpreter-legacy to enable, or use --backend vm/llvm.");
        std::process::exit(1);
    }
}
