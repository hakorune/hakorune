// Process entry point for NyRT.

// ---- Process entry (driver) ----
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main() -> i32 {
    crate::rss_observe::checkpoint("entry_start");
    let ring0_init_mode = match nyash_rust::config::env::stage1::nyrt_ring0_init_mode() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            return 70;
        }
    };
    let entry_path_prep_mode = match nyash_rust::config::env::stage1::nyrt_entry_path_prep_mode() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            return 70;
        }
    };
    let runtime_build_mode = match nyash_rust::config::env::stage1::nyrt_runtime_build_mode() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            return 70;
        }
    };
    let runtime_hooks_mode = match nyash_rust::config::env::stage1::nyrt_runtime_hooks_mode() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            return 70;
        }
    };
    let plugin_host_mode = match nyash_rust::config::env::stage1::nyrt_plugin_host_mode() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            return 70;
        }
    };
    if matches!(
        runtime_build_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Off
    ) && matches!(
        runtime_hooks_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Auto
    ) {
        eprintln!(
            "[freeze:contract][nyrt/runtime-build-off] NYASH_NYRT_RUNTIME_BUILD=off requires NYASH_NYRT_RUNTIME_HOOKS=off"
        );
        return 70;
    }
    if matches!(
        ring0_init_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Off
    ) && (!matches!(
        runtime_hooks_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Off
    ) || !matches!(
        runtime_build_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Off
    ) || !matches!(
        plugin_host_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Off
    )) {
        eprintln!(
            "[freeze:contract][nyrt/ring0-init-off] NYASH_NYRT_RING0_INIT=off requires HAKO_NYRT_PLUGIN_HOST=off, NYASH_NYRT_RUNTIME_HOOKS=off, and NYASH_NYRT_RUNTIME_BUILD=off"
        );
        return 70;
    }

    // AOT 実行器でも Ring0Context は必須（PluginHost/ログなどが依存する）。
    // EXE 直起動では host 側の init が存在しないため、ここで先に初期化する。
    if matches!(
        ring0_init_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Auto
    ) {
        if nyash_rust::runtime::ring0::GLOBAL_RING0.get().is_none() {
            nyash_rust::runtime::ring0::init_global_ring0(
                nyash_rust::runtime::ring0::default_ring0(),
            );
        }
    }
    crate::rss_observe::checkpoint("after_ring0");

    // Initialize plugin host: prefer nyash.toml next to the executable; fallback to CWD
    let exe_dir = if matches!(
        entry_path_prep_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Auto
    ) {
        nyash_rust::config::env::paths::nyrt_entry_exe_dir()
    } else {
        None
    };

    // Windows: assist DLL/plugin discovery by extending PATH and normalizing PYTHONHOME
    #[cfg(target_os = "windows")]
    if let Some(dir) = &exe_dir {
        nyash_rust::config::env::paths::nyrt_entry_apply_windows_path_shaping(dir);
    }

    // Initialize a minimal runtime to back global hooks (GC/scheduler) for safepoints.
    // Diagnostic floor probes can skip this when runtime hooks and metrics are off.
    let rt_hooks = if matches!(
        runtime_build_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Auto
    ) {
        let mut rt_builder = if nyash_rust::config::env::stage1::nyrt_minimal_startup_enabled() {
            let registry = std::sync::Arc::new(std::sync::Mutex::new(
                nyash_rust::box_factory::UnifiedBoxRegistry::with_policy(
                    nyash_rust::box_factory::FactoryPolicy::StrictPluginFirst,
                ),
            ));
            nyash_rust::runtime::NyashRuntimeBuilder::new().with_box_registry(registry)
        } else {
            nyash_rust::runtime::NyashRuntimeBuilder::new()
        };
        let gc_mode = nyash_rust::runtime::gc_mode::GcMode::from_env();
        let controller = std::sync::Arc::new(
            nyash_rust::runtime::gc_controller::GcController::new(gc_mode),
        );
        rt_builder = rt_builder.with_gc_hooks(controller);
        Some((rt_builder.build(), gc_mode))
    } else {
        None
    };
    if matches!(
        runtime_hooks_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Auto
    ) {
        if let Some((rt, _gc_mode)) = &rt_hooks {
            nyash_rust::runtime::global_hooks::set_from_runtime(rt);
        }
    } else if crate::env_flags::cli_verbose_enabled() {
        println!("🔌 nyrt: runtime hooks init skipped (NYASH_NYRT_RUNTIME_HOOKS=off)");
    }
    crate::rss_observe::checkpoint("after_runtime_hooks");

    if matches!(
        entry_path_prep_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Off
    ) && !matches!(
        plugin_host_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Off
    ) {
        eprintln!(
            "[freeze:contract][nyrt/entry-path-prep-off] NYASH_NYRT_ENTRY_PATH_PREP=off requires HAKO_NYRT_PLUGIN_HOST=off"
        );
        return 70;
    }
    let plugin_host_enabled = matches!(
        plugin_host_mode,
        nyash_rust::config::env::stage1::NyrtAutoOffMode::Auto
    );
    if plugin_host_enabled {
        let mut inited = false;
        if let Some(dir) = &exe_dir {
            let candidate = dir.join("nyash.toml");
            if candidate.exists() {
                let _ = nyash_rust::runtime::init_global_plugin_host(
                    candidate.to_string_lossy().as_ref(),
                );
                inited = true;
            }
        }
        if !inited {
            let _ = nyash_rust::runtime::init_global_plugin_host("nyash.toml");
        }
    };
    crate::rss_observe::checkpoint("after_plugin_host");
    // Optional verbosity
    if crate::env_flags::cli_verbose_enabled() {
        if plugin_host_enabled {
            println!(
                "🔌 nyrt: plugin host init attempted (exe_dir={}, cwd={})",
                exe_dir
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "?".into()),
                nyash_rust::config::env::paths::nyrt_entry_current_dir_display()
                    .unwrap_or_else(|| "?".into())
            );
        } else {
            println!("🔌 nyrt: plugin host init skipped (HAKO_NYRT_PLUGIN_HOST=off)");
        }
    }
    // Call exported Nyash entry if linked: `ny_main` (i64 -> return code normalized)
    unsafe {
        extern "C" {
            fn ny_main() -> i64;
        }
        // SAFETY: if not linked, calling will be an unresolved symbol at link-time; we rely on link step to include ny_main.
        crate::rss_observe::checkpoint("before_ny_main");
        let v = ny_main();
        crate::rss_observe::checkpoint("after_ny_main");
        let exit_code: i64 = {
            use nyash_rust::{box_trait::IntegerBox, runtime::host_handles as handles};
            if v > 0 {
                if let Some(obj) = handles::get(v as u64) {
                    if let Some(ib) = obj.as_any().downcast_ref::<IntegerBox>() {
                        ib.value as i64
                    } else {
                        // Avoid “raw integer vs handle id” collision:
                        // if the handle exists but isn't an IntegerBox, treat `v` as a raw i64.
                        v
                    }
                } else {
                    v
                }
            } else {
                v
            }
        };
        // Print standardized result line for golden comparisons (can be silenced for tests)
        let silent = nyash_rust::config::env::stage1::nyrt_silent_result_enabled();
        if !silent {
            println!("Result: {}", exit_code);
        }
        // Optional GC metrics after program completes
        let want_json = nyash_rust::config::env::stage1::nyrt_gc_metrics_json_enabled();
        let want_text = nyash_rust::config::env::stage1::nyrt_gc_metrics_text_enabled();
        if want_json || want_text {
            let Some((rt_hooks, gc_mode)) = &rt_hooks else {
                eprintln!(
                    "[freeze:contract][nyrt/runtime-build-off] GC metrics require NYASH_NYRT_RUNTIME_BUILD=auto"
                );
                return 70;
            };
            let (sp, br, bw) = rt_hooks.gc.snapshot_counters().unwrap_or((0, 0, 0));
            let handles = 0u64; // Handles tracking is not available in this kernel entry path.
            let gc_mode_s = gc_mode.as_str();
            // Include allocation totals if controller is used
            let any_gc: &dyn std::any::Any = &*rt_hooks.gc;
            let (
                alloc_count,
                alloc_bytes,
                trial_nodes,
                trial_edges,
                collect_total,
                collect_sp,
                collect_alloc,
                last_ms,
                last_reason,
            ) = if let Some(ctrl) =
                any_gc.downcast_ref::<nyash_rust::runtime::gc_controller::GcController>()
            {
                let (ac, ab) = ctrl.alloc_totals();
                let (tn, te) = ctrl.trial_reachability_last();
                let (ct, csp, calloc) = ctrl.collection_totals();
                let lms = ctrl.trial_duration_last_ms();
                let lrf = ctrl.trial_reason_last_bits();
                (ac, ab, tn, te, ct, csp, calloc, lms, lrf)
            } else {
                (0, 0, 0, 0, 0, 0, 0, 0, 0)
            };
            // Settings snapshot (env)
            let sp_interval =
                nyash_rust::config::env::stage1::nyrt_gc_collect_sp_interval().unwrap_or(0);
            let alloc_thresh =
                nyash_rust::config::env::stage1::nyrt_gc_collect_alloc_bytes().unwrap_or(0);
            let auto_sp = nyash_rust::config::env::stage1::nyrt_llvm_auto_safepoint_enabled();
            if want_json {
                // Minimal JSON assembly to avoid extra deps in nyrt
                println!(
                    "{{\"kind\":\"gc_metrics\",\"safepoints\":{},\"barrier_reads\":{},\"barrier_writes\":{},\"jit_handles\":{},\"alloc_count\":{},\"alloc_bytes\":{},\"trial_nodes\":{},\"trial_edges\":{},\"collections\":{},\"collect_by_sp\":{},\"collect_by_alloc\":{},\"last_collect_ms\":{},\"last_reason_bits\":{},\"sp_interval\":{},\"alloc_threshold\":{},\"auto_safepoint\":{},\"gc_mode\":\"{}\"}}",
                    sp, br, bw, handles, alloc_count, alloc_bytes, trial_nodes, trial_edges, collect_total, collect_sp, collect_alloc, last_ms, last_reason, sp_interval, alloc_thresh, if auto_sp {1} else {0}, gc_mode_s
                );
            } else if want_text {
                eprintln!(
                    "[GC] metrics: safepoints={} read_barriers={} write_barriers={} jit_handles={} allocs={} bytes={} collections={} (sp={} alloc={}) last_ms={} mode={}",
                    sp, br, bw, handles, alloc_count, alloc_bytes, collect_total, collect_sp, collect_alloc, last_ms, gc_mode_s
                );
            }
            // Threshold warning
            let alloc_warn_threshold =
                nyash_rust::config::env::stage1::nyrt_gc_alloc_threshold_bytes().unwrap_or(0);
            if alloc_warn_threshold > 0 && alloc_bytes > alloc_warn_threshold {
                eprintln!(
                    "[GC][warn] allocation bytes {} exceeded threshold {}",
                    alloc_bytes, alloc_warn_threshold
                );
            }
        }

        crate::observe::flush();
        crate::observe::flush_trace();
        exit_code as i32
    }
}
