use super::super::NyashRunner;
use crate::runtime::get_global_ring0;
use crate::{mir::MirCompiler, parser::NyashParser};
use std::{fs, process};

impl NyashRunner {
    /// Explicit VM compat keep using the in-crate MIR interpreter.
    /// Keep this path narrow and non-growing.
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

        // Centralized plugin guard (non-strict by default on fallback route)
        crate::runner::modes::common_util::plugin_guard::check_and_report(
            false,
            crate::config::env::env_bool("NYASH_JSON_ONLY"),
            "vm-fallback",
        );
        let module_vm = compile.module.clone();
        crate::runner::modes::common_util::vm_execution::run_vm_compiled_module(
            "vm-fallback",
            true,
            false,
            None,
            None,
            None,
            None,
            &compile.verification_result,
            module_vm,
            &_vm_user_factory,
            false,
        );
    }
}
