use super::super::NyashRunner;
use nyash_rust::{ast::ASTNode, mir::MirCompiler, parser::NyashParser};
use std::{fs, process};

impl NyashRunner {
    /// Execute VM mode with full plugin initialization and AST prelude merge
    pub(crate) fn execute_vm_mode(&self, filename: &str) {
        // Note: hv1 direct route is now handled at main.rs entry point (before plugin initialization).
        // This function is only called after plugin initialization has already occurred.

        // Quiet mode for child pipelines (e.g., selfhost compiler JSON emit)
        let quiet_pipe = crate::config::env::env_bool("NYASH_JSON_ONLY");
        let emit_trace = std::env::var("NYASH_EMIT_MIR_TRACE").ok().as_deref() == Some("1");
        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.info(&format!(
                "[runner/vm:emit-trace] phase=entry file={} quiet_pipe={} mode=stage-b?{}",
                filename,
                quiet_pipe,
                std::env::var("HAKO_STAGEB_TRACE")
                    .ok()
                    .unwrap_or_else(|| "0".to_string())
            ));
        }

        // Enforce plugin-first policy for VM on this branch (deterministic):
        // - Initialize plugin host if not yet loaded
        // - Prefer plugin implementations for core boxes
        // - Optionally fail fast when plugins are missing (NYASH_VM_PLUGIN_STRICT=1)
        {
            // FileBox provider initialization (SSOT via ProviderFactory)
            use crate::runner::modes::common_util::provider_registry;
            use nyash_rust::runtime::provider_lock;

            // Register builtin FileBox factory (idempotent)
            #[cfg(feature = "builtin-filebox")]
            {
                nyash_rust::boxes::file::builtin_factory::register_builtin_filebox();
            }

            // Select provider based on mode (dynamic → builtin → core-ro)
            let filebox_mode = provider_registry::read_filebox_mode_from_env();
            let filebox_provider = provider_registry::select_file_provider(filebox_mode);
            if let Err(e) = provider_lock::set_filebox_provider(filebox_provider) {
                if !quiet_pipe {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0
                        .log
                        .warn(&format!("[warn] FileBox provider already set: {}", e));
                }
            }

            // Initialize unified registry globals (idempotent)
            nyash_rust::runtime::init_global_unified_registry();

            // Init plugin host from nyash.toml if not yet loaded
            let need_init = {
                let host = nyash_rust::runtime::get_global_plugin_host();
                host.read()
                    .map(|h| h.config_ref().is_none())
                    .unwrap_or(true)
            };
            if need_init {
                // Let init_bid_plugins resolve hakorune.toml/nyash.toml and configure
                crate::runner_plugin_init::init_bid_plugins();
            }

            // Phase 21.4: Deprecation warnings for old ENV variables
            // No longer auto-setting NYASH_USE_PLUGIN_BUILTINS or NYASH_PLUGIN_OVERRIDE_TYPES
            // Use NYASH_BOX_FACTORY_POLICY and NYASH_FILEBOX_MODE instead
            if std::env::var("NYASH_USE_PLUGIN_BUILTINS").is_ok() {
                if !quiet_pipe {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0.log.warn("[vm] warn: NYASH_USE_PLUGIN_BUILTINS is deprecated. Use NYASH_BOX_FACTORY_POLICY instead.");
                }
            }
            if std::env::var("NYASH_PLUGIN_OVERRIDE_TYPES").is_ok() {
                if !quiet_pipe {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0.log.warn("[vm] warn: NYASH_PLUGIN_OVERRIDE_TYPES is deprecated. Use NYASH_BOX_FACTORY_POLICY instead.");
                }
            }

            // Centralized plugin guard
            let strict = crate::config::env::env_bool("NYASH_VM_PLUGIN_STRICT");
            crate::runner::modes::common_util::plugin_guard::check_and_report(
                strict, quiet_pipe, "vm",
            );
        }

        let trace = crate::config::env::cli_verbose()
            || crate::config::env::env_bool("NYASH_RESOLVE_TRACE");

        let prepared = match crate::runner::modes::common_util::vm_source_prepare::prepare_vm_source(self, filename) {
            Some(prepared) => prepared,
            None => process::exit(1),
        };
        let code_final = prepared.code_final;
        let using_imports = prepared.using_imports;

        // Optional: dump merged Hako source after using/prelude merge and Hako normalization.
        // Guarded by env; defaultはOFF（Phase 25.1a selfhost builder デバッグ用）。
        if std::env::var("NYASH_VM_DUMP_MERGED_HAKO").ok().as_deref() == Some("1") {
            let default_path = {
                let mut tmp = std::env::temp_dir();
                tmp.push("nyash_merged_vm.hako");
                tmp
            };
            let path = std::env::var("NYASH_VM_DUMP_MERGED_HAKO_PATH")
                .ok()
                .filter(|p| !p.is_empty())
                .unwrap_or_else(|| default_path.to_string_lossy().into_owned());

            if let Err(e) = fs::write(&path, &code_final) {
                if trace {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0
                        .log
                        .warn(&format!("[vm/merged-hako] failed to write {}: {}", path, e));
                }
            } else if trace || crate::config::env::env_bool("NYASH_VM_DUMP_MERGED_HAKO_LOG") {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0
                    .log
                    .debug(&format!("[vm/merged-hako] dumped merged code to {}", path));
            }
        }

        if trace && crate::config::env::parser_stage3_enabled() {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.debug(&format!(
                "[vm] Stage-3: enabled (NYASH_FEATURES/legacy env) for {}",
                filename
            ));
        }

        crate::runner::modes::common_util::safety_gate::enforce_vm_source_safety_or_exit(
            &code_final,
            "vm",
        );

        // Parse main code (after text-merge and Hako normalization)
        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.info(&format!(
                "[runner/vm:emit-trace] phase=parse.begin merged_bytes={}",
                code_final.len()
            ));
        }
        let ast_combined = match NyashParser::parse_from_string(&code_final) {
            Ok(ast) => ast,
            Err(e) => {
                crate::runner::modes::common_util::diag::print_parse_error_with_context(
                    filename,
                    &code_final,
                    &e,
                );
                // Enhanced context: list merged prelude files if any
                let preludes =
                    crate::runner::modes::common_util::resolve::clone_last_merged_preludes();
                if !preludes.is_empty() {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[parse/context] merged prelude files ({}):",
                        preludes.len()
                    ));
                    let show = std::cmp::min(16, preludes.len());
                    for p in preludes.iter().take(show) {
                        ring0.log.debug(&format!("  - {}", p));
                    }
                    if preludes.len() > show {
                        ring0
                            .log
                            .debug(&format!("  ... ({} more)", preludes.len() - show));
                    }
                }
                process::exit(1);
            }
        };
        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            let top_level_stmt_count = if let ASTNode::Program { statements, .. } = &ast_combined {
                statements.len()
            } else {
                0
            };
            ring0.log.info(&format!(
                "[runner/vm:emit-trace] phase=parse.done top_level_statements={}",
                top_level_stmt_count
            ));
        }

        // Optional: dump AST statement kinds for quick diagnostics
        if std::env::var("NYASH_AST_DUMP").ok().as_deref() == Some("1") {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.debug("[ast] dump start (vm)");
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
                    ring0.log.debug(&format!("[ast] {}: {}", i, kind));
                }
            }
            ring0.log.debug("[ast] dump end");
        }

        // Macro expand (if enabled)
        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.info("[runner/vm:emit-trace] phase=macro.begin");
        }
        let ast = crate::r#macro::maybe_expand_and_dump(&ast_combined, false);
        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            let top_level_stmt_count = if let ASTNode::Program { statements, .. } = &ast {
                statements.len()
            } else {
                0
            };
            ring0.log.info(&format!(
                "[runner/vm:emit-trace] phase=macro.done top_level_statements={}",
                top_level_stmt_count
            ));
        }

        // Minimal user-defined Box support (inline factory)
        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0
                .log
                .info("[runner/vm:emit-trace] phase=user-factory.begin");
        }
        let vm_user_factory =
            crate::runner::modes::common_util::vm_user_factory::prepare_vm_user_factory(
                &ast, true, true,
            );
        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.info(&format!(
                "[runner/vm:emit-trace] phase=user-factory.done static_boxes={}",
                vm_user_factory.static_box_count()
            ));
        }

        // Compile to MIR
        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.info("[runner/vm:emit-trace] phase=compile.begin");
        }
        let mut compiler = MirCompiler::with_options(!self.config.no_optimize);
        let compile = match crate::runner::modes::common_util::source_hint::compile_with_source_hint_and_imports(&mut compiler, ast, Some(filename), using_imports) {
            Ok(c) => c,
            Err(e) => {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0.log.error(&format!("❌ MIR compilation error: {}", e));
                process::exit(1);
            }
        };
        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.info(&format!(
                "[runner/vm:emit-trace] phase=compile.done functions={}",
                compile.module.functions.len()
            ));
        }

        let module_vm = compile.module.clone();
        let groups = self.config.as_groups();
        crate::runner::modes::common_util::vm_execution::run_vm_compiled_module(
            "vm",
            quiet_pipe,
            emit_trace,
            groups.emit.emit_mir_json.as_deref(),
            groups.emit.emit_exe.as_deref(),
            groups.emit.emit_exe_nyrt.as_deref(),
            groups.emit.emit_exe_libs.as_deref(),
            &compile.verification_result,
            module_vm,
            &vm_user_factory,
            true,
        );

    }
}
