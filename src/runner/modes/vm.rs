use super::super::NyashRunner;
use nyash_rust::{ast::ASTNode, mir::MirCompiler, parser::NyashParser};
use std::{fs, process};

// Phase 30 F-4.4: JoinIR VM Bridge dispatch (experimental)
// Used only when NYASH_JOINIR_EXPERIMENT=1 AND NYASH_JOINIR_VM_BRIDGE=1
use crate::mir::join_ir_vm_bridge_dispatch::try_run_joinir_vm_bridge;

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

        // Read the file
        let code = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0
                    .log
                    .error(&format!("❌ Error reading file {}: {}", filename, e));
                process::exit(1);
            }
        };

        // Unified using/prelude handling (SSOT):
        // - resolve_prelude_paths_profiled: discover preludes (DFS, operator boxes, etc.)
        // - merge_prelude_text: text-based merge for all VM paths
        //   * .hako プレリュードは AST に突っ込まない（Parse error防止）
        //   * .hako は text-merge + Stage-3 parser で一貫処理
        let trace = crate::config::env::cli_verbose()
            || crate::config::env::env_bool("NYASH_RESOLVE_TRACE");

        // When using is enabled, resolve preludes/profile; otherwise, keep original code.
        let mut code_final = if crate::config::env::enable_using() {
            match crate::runner::modes::common_util::resolve::resolve_prelude_paths_profiled(
                self, &code, filename,
            ) {
                Ok((_, prelude_paths)) => {
                    if !prelude_paths.is_empty() {
                        // SSOT: always text-merge for VM (includes .hako-safe handling inside)
                        match crate::runner::modes::common_util::resolve::merge_prelude_text(
                            self, &code, filename,
                        ) {
                            Ok(merged) => {
                                if trace {
                                    let ring0 = crate::runtime::ring0::get_global_ring0();
                                    ring0.log.debug(&format!(
                                        "[using/text-merge] preludes={} (vm)",
                                        prelude_paths.len()
                                    ));
                                }
                                merged
                            }
                            Err(e) => {
                                let ring0 = crate::runtime::ring0::get_global_ring0();
                                let msg = if e.starts_with("[freeze:contract][module_registry]") {
                                    e
                                } else {
                                    format!("❌ {}", e)
                                };
                                ring0.log.error(&msg);
                                process::exit(1);
                            }
                        }
                    } else {
                        code.clone()
                    }
                }
                Err(e) => {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    let msg = if e.starts_with("[freeze:contract][module_registry]") {
                        e
                    } else {
                        format!("❌ {}", e)
                    };
                    ring0.log.error(&msg);
                    process::exit(1);
                }
            }
        } else {
            // using disabled: detect and fail fast if present
            if code.contains("\nusing ") || code.trim_start().starts_with("using ") {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0.log.error("❌ using: prelude merge is disabled in this profile. Enable NYASH_USING_AST=1 or remove 'using' lines.");
                process::exit(1);
            }
            code
        };

        // Dev sugar pre-expand: @name = expr → local name = expr
        code_final = crate::runner::modes::common_util::resolve::preexpand_at_local(&code_final);

        // Hako-friendly normalize: strip leading `local ` at line head for Nyash parser compatibility.
        if crate::runner::modes::common_util::hako::looks_like_hako_code(&code_final)
            || filename.ends_with(".hako")
        {
            code_final = crate::runner::modes::common_util::hako::strip_local_decl(&code_final);
        }

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
        let static_box_decls = {
            use crate::{
                box_factory::{BoxFactory, RuntimeError},
                core::model::BoxDeclaration as CoreBoxDecl,
                instance_v2::InstanceBox,
            };
            use std::sync::{Arc, RwLock};

            // Collect user-defined (non-static) box declarations at program level.
            // Additionally, record static box names so we can alias
            // `StaticBoxName` -> `StaticBoxNameInstance` when such a
            // concrete instance box exists (common pattern in libs).
            // Also collect static box declarations for VM singleton persistence.
            let mut nonstatic_decls: std::collections::HashMap<String, CoreBoxDecl> =
                std::collections::HashMap::new();
            let mut static_names: Vec<String> = Vec::new();
            let mut static_box_decls: std::collections::HashMap<String, CoreBoxDecl> =
                std::collections::HashMap::new();
            if let ASTNode::Program { statements, .. } = &ast {
                for st in statements {
                    if let ASTNode::BoxDeclaration {
                        name,
                        fields,
                        public_fields,
                        private_fields,
                        methods,
                        constructors,
                        init_fields,
                        weak_fields,
                        is_interface,
                        extends,
                        implements,
                        type_parameters,
                        is_static,
                        ..
                    } = st
                    {
                        if *is_static {
                            static_names.push(name.clone());
                            // Store static box declaration for VM singleton persistence
                            let static_decl = CoreBoxDecl {
                                name: name.clone(),
                                fields: fields.clone(),
                                public_fields: public_fields.clone(),
                                private_fields: private_fields.clone(),
                                methods: methods.clone(),
                                constructors: constructors.clone(),
                                init_fields: init_fields.clone(),
                                weak_fields: weak_fields.clone(),
                                is_interface: *is_interface,
                                extends: extends.clone(),
                                implements: implements.clone(),
                                type_parameters: type_parameters.clone(),
                            };
                            static_box_decls.insert(name.clone(), static_decl);
                            continue; // modules/static boxes are not user-instantiable directly
                        }
                        let decl = CoreBoxDecl {
                            name: name.clone(),
                            fields: fields.clone(),
                            public_fields: public_fields.clone(),
                            private_fields: private_fields.clone(),
                            methods: methods.clone(),
                            constructors: constructors.clone(),
                            init_fields: init_fields.clone(),
                            weak_fields: weak_fields.clone(),
                            is_interface: *is_interface,
                            extends: extends.clone(),
                            implements: implements.clone(),
                            type_parameters: type_parameters.clone(),
                        };
                        nonstatic_decls.insert(name.clone(), decl);
                    }
                }
            }
            // Phase 25.1b: Include static boxes in user factory for NewBox support
            // Static boxes are singletons (via VM's register_static_box_decl) but must also
            // be advertised by user factory so NewBox doesn't fall through to plugin route.
            //
            // Build final map:
            // 1. Start with nonstatic_decls (regular user boxes)
            // 2. Add static_box_decls (for NewBox support, VM will ensure singleton)
            // 3. Add StaticName -> StaticNameInstance aliases if present
            let mut decls = nonstatic_decls.clone();

            // Add static box declarations (VM singleton instances)
            // nonstatic takes precedence if name conflicts (rare but possible)
            for (name, sdecl) in static_box_decls.iter() {
                if !decls.contains_key(name) {
                    decls.insert(name.clone(), sdecl.clone());
                }
            }

            // Add StaticName -> StaticNameInstance aliases if present
            for s in static_names.into_iter() {
                let inst = format!("{}Instance", s);
                if let Some(d) = nonstatic_decls.get(&inst) {
                    decls.insert(s, d.clone());
                }
            }

            if !decls.is_empty() {
                // Inline factory: minimal User factory backed by collected declarations
                struct InlineUserBoxFactory {
                    decls: Arc<RwLock<std::collections::HashMap<String, CoreBoxDecl>>>,
                }
                impl BoxFactory for InlineUserBoxFactory {
                    fn create_box(
                        &self,
                        name: &str,
                        args: &[Box<dyn crate::box_trait::NyashBox>],
                    ) -> Result<Box<dyn crate::box_trait::NyashBox>, RuntimeError>
                    {
                        let guard = self.decls.read().unwrap();
                        let opt = guard.get(name).cloned();
                        let decl = match opt {
                            Some(d) => {
                                drop(guard);
                                d
                            }
                            None => {
                                // Quick Win 1: Show available boxes for easier debugging
                                let mut available: Vec<_> = guard.keys().cloned().collect();
                                available.sort();
                                drop(guard);
                                let hint = if available.is_empty() {
                                    "No user-defined boxes available".to_string()
                                } else if available.len() <= 10 {
                                    format!("Available: {}", available.join(", "))
                                } else {
                                    format!(
                                        "Available ({} boxes): {}, ...",
                                        available.len(),
                                        available[..10].join(", ")
                                    )
                                };
                                return Err(RuntimeError::InvalidOperation {
                                    message: format!("Unknown Box type: {}. {}", name, hint),
                                });
                            }
                        };
                        let mut inst = InstanceBox::from_declaration(
                            decl.name.clone(),
                            decl.fields.clone(),
                            decl.methods.clone(),
                        );
                        let _ = inst.init(args);
                        Ok(Box::new(inst))
                    }

                    fn box_types(&self) -> Vec<&str> {
                        // Can't return borrowed strings from temporary RwLock guard
                        // Registry will try create_box() regardless of this list
                        vec![]
                    }

                    fn is_available(&self) -> bool {
                        true
                    }

                    fn factory_type(&self) -> crate::box_factory::FactoryType {
                        crate::box_factory::FactoryType::User
                    }
                }
                let factory = InlineUserBoxFactory {
                    decls: Arc::new(RwLock::new(decls)),
                };
                crate::runtime::unified_registry::register_user_defined_factory(
                    std::sync::Arc::new(factory),
                );
            }

            // Return static_box_decls for VM registration
            static_box_decls
        };
        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.info(&format!(
                "[runner/vm:emit-trace] phase=user-factory.done static_boxes={}",
                static_box_decls.len()
            ));
        }

        // Compile to MIR
        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.info("[runner/vm:emit-trace] phase=compile.begin");
        }
        let mut compiler = MirCompiler::with_options(!self.config.no_optimize);
        let compile = match crate::runner::modes::common_util::source_hint::compile_with_source_hint(
            &mut compiler,
            ast,
            Some(filename),
        ) {
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

        // Optional barrier-elision for parity with fallback path
        let mut module_vm = compile.module.clone();
        if crate::config::env::env_bool("NYASH_VM_ESCAPE_ANALYSIS") {
            let removed = crate::mir::passes::escape::escape_elide_barriers_vm(&mut module_vm);
            if removed > 0 {
                crate::cli_v!("[VM] escape_elide_barriers: removed {} barriers", removed);
            }
        }

        // CLI emit: MIR JSON / EXE
        // NOTE: These flags are CLI-level and should work regardless of selected backend.
        // The VM runner is a common default backend, so we honor them here and exit early.
        {
            let groups = self.config.as_groups();
            crate::runner::modes::common_util::emit_direct::maybe_emit_mir_json_and_exit(
                groups.emit.emit_mir_json.as_deref(),
                &compile.verification_result,
                "vm",
                quiet_pipe,
                |out_path| {
                    crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(
                        &module_vm, out_path,
                    )
                },
            );
            crate::runner::modes::common_util::emit_direct::maybe_emit_exe_and_exit(
                groups.emit.emit_exe.as_deref(),
                &compile.verification_result,
                "vm",
                quiet_pipe,
                |exe_out| {
                    crate::runner::modes::common_util::exec::ny_llvmc_emit_exe_bin(
                        &module_vm,
                        exe_out,
                        groups.emit.emit_exe_nyrt.as_deref(),
                        groups.emit.emit_exe_libs.as_deref(),
                    )
                },
            );
        }

        // Optional: dump MIR for diagnostics
        // Phase 25.1: File dump for offline analysis (ParserBox等)
        if let Ok(path) = std::env::var("RUST_MIR_DUMP_PATH") {
            if let Ok(mut f) = std::fs::File::create(&path) {
                let p = crate::mir::MirPrinter::new();
                let _ = std::io::Write::write_all(&mut f, p.print_module(&module_vm).as_bytes());
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0.log.info(&format!("[vm] MIR dumped to: {}", path));
            }
        }
        // Existing: NYASH_VM_DUMP_MIR dumps to stderr
        if crate::config::env::env_bool("NYASH_VM_DUMP_MIR") {
            let p = crate::mir::MirPrinter::new();
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.debug(&p.print_module(&module_vm));
        }

        // Execute via MIR interpreter
        use crate::backend::MirInterpreter;
        let mut vm = MirInterpreter::new();

        // Register static box declarations for singleton persistence (AST-based)
        for (name, decl) in static_box_decls {
            vm.register_static_box_decl(name, decl);
        }

        // Optional verifier gate (single-entry SSOT across VM lanes)
        crate::runner::modes::common_util::verifier_gate::enforce_vm_verify_gate_or_exit(
            &module_vm, "vm",
        );
        crate::runner::modes::common_util::safety_gate::enforce_vm_lifecycle_safety_or_exit(
            &module_vm, "vm",
        );

        if std::env::var("NYASH_DUMP_FUNCS").ok().as_deref() == Some("1") {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.debug("[vm] functions available:");
            for k in module_vm.functions.keys() {
                ring0.log.debug(&format!("  - {}", k));
            }
        }

        // Phase 33-10.0: If lowering ドライラン統合（箱化版）
        // JoinIR dev + IfSelect 有効時に IfLoweringDryRunner を使用
        if crate::config::env::joinir_dev_enabled()
            && crate::config::env::joinir_if_select_enabled()
        {
            let debug_level = crate::config::env::joinir_debug_level();
            let runner =
                crate::mir::join_ir::lowering::if_dry_runner::IfLoweringDryRunner::new(debug_level);
            let stats = runner.scan_module(&module_vm.functions);
            runner.print_stats(&stats);
        }

        // Phase 30 F-4.4: JoinIR VM Bridge experimental path (consolidated dispatch)
        // Activated when NYASH_JOINIR_EXPERIMENT=1 AND NYASH_JOINIR_VM_BRIDGE=1
        // Routing logic is centralized in join_ir_vm_bridge_dispatch module
        try_run_joinir_vm_bridge(&module_vm, quiet_pipe);

        if emit_trace {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.info("[runner/vm:emit-trace] phase=execute.begin");
        }
        match vm.execute_module(&module_vm) {
            Ok(ret) => {
                use crate::box_trait::{BoolBox, IntegerBox};

                if emit_trace {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0.log.info(&format!(
                        "[runner/vm:emit-trace] phase=execute.done result={}",
                        ret.to_string_box().value
                    ));
                }

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

                // Optional: print lightweight VM counters for diagnostics
                if crate::config::env::env_bool("NYASH_VM_STATS") {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    let (inst, br, cmp) = vm.stats_counters();
                    ring0.log.debug(&format!(
                        "[vm/stats] inst={} compare={} branch={}",
                        inst, cmp, br
                    ));
                }

                // Quiet mode: suppress "RC:" output for JSON-only pipelines
                if !quiet_pipe {
                    // Phase 105.5: Unified console output via macro
                    crate::console_println!("RC: {}", exit_code);
                }
                if std::env::var("NYASH_EMIT_MIR_TRACE").ok().as_deref() == Some("1") {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0
                        .log
                        .debug(&format!("[runner/vm] exit_code={}", exit_code));
                }

                // Phase 285: Emit leak report before exit (if enabled)
                crate::runtime::leak_tracker::emit_leak_report();

                // Exit with the return value as exit code
                process::exit(exit_code);
            }
            Err(e) => {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                if std::env::var("NYASH_EMIT_MIR_TRACE").ok().as_deref() == Some("1") {
                    ring0.log.debug(&format!("[runner/vm] vm_error={}", e));
                }
                ring0.log.error(&format!("❌ [rust-vm] VM error: {}", e));
                process::exit(1);
            }
        }
    }
}
