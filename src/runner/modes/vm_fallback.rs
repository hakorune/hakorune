use super::super::NyashRunner;
use crate::runtime::get_global_ring0;
use crate::{backend::MirInterpreter, mir::MirCompiler, parser::NyashParser};
use std::{fs, process};

impl NyashRunner {
    /// Lightweight VM fallback using the in-crate MIR interpreter.
    /// - Respects using preprocessing done earlier in the pipeline
    /// - Relies on global plugin host initialized by runner
    pub(crate) fn execute_vm_fallback_interpreter(&self, filename: &str) {
        crate::runner::route_orchestrator::enforce_vm_compat_fallback_guard_or_exit("vm-fallback");
        // Note: hv1 direct route is now handled at main.rs entry point (before plugin initialization).
        // This function is only called after plugin initialization has already occurred.

        // Read source
        let code = match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("❌ Error reading file {}: {}", filename, e);
                process::exit(1);
            }
        };

        let trace = crate::config::env::cli_verbose()
            || crate::config::env::env_bool("NYASH_RESOLVE_TRACE");

        // Unified using/prelude handling (SSOT, parity with vm.rs):
        // - resolve_prelude_paths_profiled で preludes を発見
        // - merge_prelude_text で text-merge（.hako は AST parse しない）
        let mut code2 = if crate::config::env::enable_using() {
            match crate::runner::modes::common_util::resolve::resolve_prelude_paths_profiled(
                self, &code, filename,
            ) {
                Ok((_, prelude_paths)) => {
                    if !prelude_paths.is_empty() {
                        match crate::runner::modes::common_util::resolve::merge_prelude_text(
                            self, &code, filename,
                        ) {
                            Ok(merged) => {
                                if trace {
                                    eprintln!(
                                        "[using/text-merge] preludes={} (vm-fallback)",
                                        prelude_paths.len()
                                    );
                                }
                                merged
                            }
                            Err(e) => {
                                if e.starts_with("[freeze:contract][module_registry]") {
                                    eprintln!("{}", e);
                                } else {
                                    eprintln!("❌ {}", e);
                                }
                                process::exit(1);
                            }
                        }
                    } else {
                        code.clone()
                    }
                }
                Err(e) => {
                    if e.starts_with("[freeze:contract][module_registry]") {
                        eprintln!("{}", e);
                    } else {
                        eprintln!("❌ {}", e);
                    }
                    process::exit(1);
                }
            }
        } else {
            // using disabled: detect and fail fast if present
            if code.contains("\nusing ") || code.trim_start().starts_with("using ") {
                eprintln!(
                    "❌ using: prelude merge is disabled in this profile. Enable NYASH_USING_AST=1 or remove 'using' lines."
                );
                process::exit(1);
            }
            code
        };

        // Dev sugar pre-expand: @name = expr → local name = expr
        code2 = crate::runner::modes::common_util::resolve::preexpand_at_local(&code2);

        // Hako-friendly normalize
        if crate::runner::modes::common_util::hako::looks_like_hako_code(&code2)
            || filename.ends_with(".hako")
        {
            code2 = crate::runner::modes::common_util::hako::strip_local_decl(&code2);
        }

        if trace && crate::config::env::parser_stage3_enabled() {
            eprintln!(
                "[vm-fallback] Stage-3: enabled (NYASH_FEATURES/legacy env) for {}",
                filename
            );
        }

        crate::runner::modes::common_util::safety_gate::enforce_vm_source_safety_or_exit(
            &code2,
            "vm-fallback",
        );

        // Parse main code
        let main_ast = match NyashParser::parse_from_string(&code2) {
            Ok(ast) => ast,
            Err(e) => {
                crate::runner::modes::common_util::diag::print_parse_error_with_context(
                    filename, &code2, &e,
                );
                process::exit(1);
            }
        };
        // No AST preludes (text path or no using) → use the parsed main AST as-is
        let ast_combined = main_ast;
        // Optional: dump AST statement kinds for quick diagnostics
        if std::env::var("NYASH_AST_DUMP").ok().as_deref() == Some("1") {
            use nyash_rust::ast::ASTNode;
            get_global_ring0()
                .log
                .debug("[ast] dump start (vm-fallback)");
            if let ASTNode::Program { statements, .. } = &ast_combined {
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
        let ast = crate::r#macro::maybe_expand_and_dump(&ast_combined, false);

        let _vm_user_factory =
            crate::runner::modes::common_util::vm_user_factory::prepare_vm_user_factory(
                &ast, false, true,
            );
        let mut compiler = MirCompiler::with_options(!self.config.no_optimize);
        let compile = match crate::runner::modes::common_util::source_hint::compile_with_source_hint(
            &mut compiler,
            ast,
            Some(filename),
        ) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("❌ MIR compilation error: {}", e);
                process::exit(1);
            }
        };

        // Optional barrier-elision for parity with VM path
        let mut module_vm = compile.module.clone();
        if crate::config::env::env_bool("NYASH_VM_ESCAPE_ANALYSIS") {
            let removed = crate::mir::passes::escape::escape_elide_barriers_vm(&mut module_vm);
            if removed > 0 {
                crate::cli_v!(
                    "[VM-fallback] escape_elide_barriers: removed {} barriers",
                    removed
                );
            }
        }

        // Optional: dump MIR for diagnostics (parity with vm path)
        // Phase 25.1: File dump for offline analysis (ParserBox等)
        if let Ok(path) = std::env::var("RUST_MIR_DUMP_PATH") {
            if let Ok(mut f) = std::fs::File::create(&path) {
                let p = crate::mir::MirPrinter::new();
                let _ = std::io::Write::write_all(&mut f, p.print_module(&module_vm).as_bytes());
                get_global_ring0()
                    .log
                    .debug(&format!("[vm-fallback] MIR dumped to: {}", path));
            }
        }
        // Existing: NYASH_VM_DUMP_MIR dumps to stderr
        if crate::config::env::env_bool("NYASH_VM_DUMP_MIR") {
            let p = crate::mir::MirPrinter::new();
            get_global_ring0().log.debug(&p.print_module(&module_vm));
        }

        // Execute via MIR interpreter
        let mut vm = MirInterpreter::new();
        // Centralized plugin guard (non-strict by default on fallback route)
        crate::runner::modes::common_util::plugin_guard::check_and_report(
            false,
            crate::config::env::env_bool("NYASH_JSON_ONLY"),
            "vm-fallback",
        );
        // Optional verifier gate (single-entry SSOT across VM lanes)
        crate::runner::modes::common_util::verifier_gate::enforce_vm_verify_gate_or_exit(
            &module_vm,
            "vm-fallback",
        );
        crate::runner::modes::common_util::safety_gate::enforce_vm_lifecycle_safety_or_exit(
            &module_vm,
            "vm-fallback",
        );
        if std::env::var("NYASH_DUMP_FUNCS").ok().as_deref() == Some("1") {
            get_global_ring0().log.debug("[vm] functions available:");
            for k in module_vm.functions.keys() {
                get_global_ring0().log.debug(&format!("  - {}", k));
            }
        }
        match vm.execute_module(&module_vm) {
            Ok(ret) => {
                use crate::box_trait::{BoolBox, IntegerBox};

                // Extract exit code from return value
                let exit_code = if let Some(ib) = ret.as_any().downcast_ref::<IntegerBox>() {
                    ib.value as i32
                } else if let Some(bb) = ret.as_any().downcast_ref::<BoolBox>() {
                    if bb.value {
                        1
                    } else {
                        0
                    }
                } else {
                    // For non-integer/bool returns, default to 0 (success)
                    0
                };

                // Exit with the return value as exit code
                process::exit(exit_code);
            }
            Err(e) => {
                eprintln!("❌ VM fallback error: {}", e);
                process::exit(1);
            }
        }
    }
}

impl NyashRunner {
    /// Small helper to continue fallback execution once AST is prepared
    #[allow(dead_code)]
    fn execute_vm_fallback_from_ast(&self, filename: &str, ast: nyash_rust::ast::ASTNode) {
        use crate::{backend::MirInterpreter, mir::MirCompiler};
        use std::process;

        // Macro expand (if enabled)
        let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);
        let _vm_user_factory =
            crate::runner::modes::common_util::vm_user_factory::prepare_vm_user_factory(
                &ast, false, true,
            );
        // Compile to MIR and execute via interpreter
        let mut compiler = MirCompiler::with_options(!self.config.no_optimize);
        let module = match crate::runner::modes::common_util::source_hint::compile_with_source_hint(
            &mut compiler,
            ast,
            Some(filename),
        ) {
            Ok(r) => r.module,
            Err(e) => {
                eprintln!("❌ MIR compilation error: {}", e);
                process::exit(1);
            }
        };
        let mut interp = MirInterpreter::new();
        match interp.execute_module(&module) {
            Ok(result) => {
                use nyash_rust::box_trait::{BoolBox, IntegerBox};
                let rc = if let Some(ib) = result.as_any().downcast_ref::<IntegerBox>() {
                    ib.value as i32
                } else if let Some(bb) = result.as_any().downcast_ref::<BoolBox>() {
                    if bb.value {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                };
                // For C‑API pure pipeline, suppress "RC:" text to keep last line = exe path
                let capi = std::env::var("NYASH_LLVM_USE_CAPI").ok().as_deref() == Some("1");
                let pure = crate::config::env::backend_recipe_requests_pure_first();
                if !(capi && pure) {
                    println!("RC: {}", rc);
                }
                process::exit(rc);
            }
            Err(e) => {
                eprintln!("❌ VM fallback runtime error: {}", e);
                process::exit(1);
            }
        }
    }
}
