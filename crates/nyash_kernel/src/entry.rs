// Process entry point for NyRT.

enum PluginHostMode {
    Auto,
    Off,
}

enum RuntimeHooksMode {
    Auto,
    Off,
}

enum RuntimeBuildMode {
    Auto,
    Off,
}

enum EntryPathPrepMode {
    Auto,
    Off,
}

enum Ring0InitMode {
    Auto,
    Off,
}

fn minimal_startup_enabled_from_env() -> bool {
    crate::env_flags::flag_on("NYASH_NYRT_MINIMAL_STARTUP")
}

fn plugin_host_mode_from_env() -> Result<PluginHostMode, String> {
    let Ok(raw) = std::env::var("HAKO_NYRT_PLUGIN_HOST") else {
        return Ok(PluginHostMode::Auto);
    };
    let value = raw.trim();
    if value.is_empty()
        || value.eq_ignore_ascii_case("auto")
        || value.eq_ignore_ascii_case("on")
        || value.eq_ignore_ascii_case("1")
        || value.eq_ignore_ascii_case("true")
    {
        return Ok(PluginHostMode::Auto);
    }
    if value.eq_ignore_ascii_case("off")
        || value.eq_ignore_ascii_case("0")
        || value.eq_ignore_ascii_case("false")
        || value.eq_ignore_ascii_case("none")
    {
        return Ok(PluginHostMode::Off);
    }
    Err(format!(
        "[freeze:contract][nyrt/plugin-host-mode] expected=auto|on|1|true|off|0|false|none got={}",
        value
    ))
}

fn runtime_hooks_mode_from_env() -> Result<RuntimeHooksMode, String> {
    let Ok(raw) = std::env::var("NYASH_NYRT_RUNTIME_HOOKS") else {
        return Ok(RuntimeHooksMode::Auto);
    };
    let value = raw.trim();
    if value.is_empty()
        || value.eq_ignore_ascii_case("auto")
        || value.eq_ignore_ascii_case("on")
        || value.eq_ignore_ascii_case("1")
        || value.eq_ignore_ascii_case("true")
    {
        return Ok(RuntimeHooksMode::Auto);
    }
    if value.eq_ignore_ascii_case("off")
        || value.eq_ignore_ascii_case("0")
        || value.eq_ignore_ascii_case("false")
        || value.eq_ignore_ascii_case("none")
    {
        return Ok(RuntimeHooksMode::Off);
    }
    Err(format!(
        "[freeze:contract][nyrt/runtime-hooks-mode] expected=auto|on|1|true|off|0|false|none got={}",
        value
    ))
}

fn runtime_build_mode_from_env() -> Result<RuntimeBuildMode, String> {
    let Ok(raw) = std::env::var("NYASH_NYRT_RUNTIME_BUILD") else {
        return Ok(RuntimeBuildMode::Auto);
    };
    let value = raw.trim();
    if value.is_empty()
        || value.eq_ignore_ascii_case("auto")
        || value.eq_ignore_ascii_case("on")
        || value.eq_ignore_ascii_case("1")
        || value.eq_ignore_ascii_case("true")
    {
        return Ok(RuntimeBuildMode::Auto);
    }
    if value.eq_ignore_ascii_case("off")
        || value.eq_ignore_ascii_case("0")
        || value.eq_ignore_ascii_case("false")
        || value.eq_ignore_ascii_case("none")
    {
        return Ok(RuntimeBuildMode::Off);
    }
    Err(format!(
        "[freeze:contract][nyrt/runtime-build-mode] expected=auto|on|1|true|off|0|false|none got={}",
        value
    ))
}

fn entry_path_prep_mode_from_env() -> Result<EntryPathPrepMode, String> {
    let Ok(raw) = std::env::var("NYASH_NYRT_ENTRY_PATH_PREP") else {
        return Ok(EntryPathPrepMode::Auto);
    };
    let value = raw.trim();
    if value.is_empty()
        || value.eq_ignore_ascii_case("auto")
        || value.eq_ignore_ascii_case("on")
        || value.eq_ignore_ascii_case("1")
        || value.eq_ignore_ascii_case("true")
    {
        return Ok(EntryPathPrepMode::Auto);
    }
    if value.eq_ignore_ascii_case("off")
        || value.eq_ignore_ascii_case("0")
        || value.eq_ignore_ascii_case("false")
        || value.eq_ignore_ascii_case("none")
    {
        return Ok(EntryPathPrepMode::Off);
    }
    Err(format!(
        "[freeze:contract][nyrt/entry-path-prep-mode] expected=auto|on|1|true|off|0|false|none got={}",
        value
    ))
}

fn ring0_init_mode_from_env() -> Result<Ring0InitMode, String> {
    let Ok(raw) = std::env::var("NYASH_NYRT_RING0_INIT") else {
        return Ok(Ring0InitMode::Auto);
    };
    let value = raw.trim();
    if value.is_empty()
        || value.eq_ignore_ascii_case("auto")
        || value.eq_ignore_ascii_case("on")
        || value.eq_ignore_ascii_case("1")
        || value.eq_ignore_ascii_case("true")
    {
        return Ok(Ring0InitMode::Auto);
    }
    if value.eq_ignore_ascii_case("off")
        || value.eq_ignore_ascii_case("0")
        || value.eq_ignore_ascii_case("false")
        || value.eq_ignore_ascii_case("none")
    {
        return Ok(Ring0InitMode::Off);
    }
    Err(format!(
        "[freeze:contract][nyrt/ring0-init-mode] expected=auto|on|1|true|off|0|false|none got={}",
        value
    ))
}

// ---- Process entry (driver) ----
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main() -> i32 {
    crate::rss_observe::checkpoint("entry_start");
    let ring0_init_mode = match ring0_init_mode_from_env() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            return 70;
        }
    };
    let entry_path_prep_mode = match entry_path_prep_mode_from_env() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            return 70;
        }
    };
    let runtime_build_mode = match runtime_build_mode_from_env() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            return 70;
        }
    };
    let runtime_hooks_mode = match runtime_hooks_mode_from_env() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            return 70;
        }
    };
    let plugin_host_mode = match plugin_host_mode_from_env() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("{}", message);
            return 70;
        }
    };
    if matches!(runtime_build_mode, RuntimeBuildMode::Off)
        && matches!(runtime_hooks_mode, RuntimeHooksMode::Auto)
    {
        eprintln!(
            "[freeze:contract][nyrt/runtime-build-off] NYASH_NYRT_RUNTIME_BUILD=off requires NYASH_NYRT_RUNTIME_HOOKS=off"
        );
        return 70;
    }
    if matches!(ring0_init_mode, Ring0InitMode::Off)
        && (!matches!(runtime_hooks_mode, RuntimeHooksMode::Off)
            || !matches!(runtime_build_mode, RuntimeBuildMode::Off)
            || !matches!(plugin_host_mode, PluginHostMode::Off))
    {
        eprintln!(
            "[freeze:contract][nyrt/ring0-init-off] NYASH_NYRT_RING0_INIT=off requires HAKO_NYRT_PLUGIN_HOST=off, NYASH_NYRT_RUNTIME_HOOKS=off, and NYASH_NYRT_RUNTIME_BUILD=off"
        );
        return 70;
    }

    // AOT 実行器でも Ring0Context は必須（PluginHost/ログなどが依存する）。
    // EXE 直起動では host 側の init が存在しないため、ここで先に初期化する。
    if matches!(ring0_init_mode, Ring0InitMode::Auto) {
        if nyash_rust::runtime::ring0::GLOBAL_RING0.get().is_none() {
            nyash_rust::runtime::ring0::init_global_ring0(
                nyash_rust::runtime::ring0::default_ring0(),
            );
        }
    }
    crate::rss_observe::checkpoint("after_ring0");

    // Initialize plugin host: prefer nyash.toml next to the executable; fallback to CWD
    let exe_dir = if matches!(entry_path_prep_mode, EntryPathPrepMode::Auto) {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
    } else {
        None
    };

    // Windows: assist DLL/plugin discovery by extending PATH and normalizing PYTHONHOME
    #[cfg(target_os = "windows")]
    if let Some(dir) = &exe_dir {
        use std::path::PathBuf;
        // Extend PATH with exe_dir and exe_dir\plugins if not already present
        let mut path_val = std::env::var("PATH").unwrap_or_default();
        let add_path = |pv: &mut String, p: &PathBuf| {
            let ps = p.display().to_string();
            if !pv.split(';').any(|seg| seg.eq_ignore_ascii_case(&ps)) {
                if !pv.is_empty() {
                    pv.push(';');
                }
                pv.push_str(&ps);
            }
        };
        add_path(&mut path_val, dir);
        let plug = dir.join("plugins");
        if plug.is_dir() {
            add_path(&mut path_val, &plug);
        }
        std::env::set_var("PATH", &path_val);

        // Normalize PYTHONHOME: if unset, point to exe_dir\python when present.
        match std::env::var("PYTHONHOME") {
            Ok(v) => {
                // If relative, make absolute under exe_dir
                let pb = PathBuf::from(&v);
                if pb.is_relative() {
                    let abs = dir.join(pb);
                    std::env::set_var("PYTHONHOME", abs.display().to_string());
                }
            }
            Err(_) => {
                let cand = dir.join("python");
                if cand.is_dir() {
                    std::env::set_var("PYTHONHOME", cand.display().to_string());
                }
            }
        }
    }

    // Initialize a minimal runtime to back global hooks (GC/scheduler) for safepoints.
    // Diagnostic floor probes can skip this when runtime hooks and metrics are off.
    let rt_hooks = if matches!(runtime_build_mode, RuntimeBuildMode::Auto) {
        let mut rt_builder = if minimal_startup_enabled_from_env() {
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
    if matches!(runtime_hooks_mode, RuntimeHooksMode::Auto) {
        if let Some((rt, _gc_mode)) = &rt_hooks {
            nyash_rust::runtime::global_hooks::set_from_runtime(rt);
        }
    } else if crate::env_flags::cli_verbose_enabled() {
        println!("🔌 nyrt: runtime hooks init skipped (NYASH_NYRT_RUNTIME_HOOKS=off)");
    }
    crate::rss_observe::checkpoint("after_runtime_hooks");

    if matches!(entry_path_prep_mode, EntryPathPrepMode::Off)
        && !matches!(plugin_host_mode, PluginHostMode::Off)
    {
        eprintln!(
            "[freeze:contract][nyrt/entry-path-prep-off] NYASH_NYRT_ENTRY_PATH_PREP=off requires HAKO_NYRT_PLUGIN_HOST=off"
        );
        return 70;
    }
    let plugin_host_enabled = matches!(plugin_host_mode, PluginHostMode::Auto);
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
                std::env::current_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| "?".into())
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
        let silent = crate::env_flags::flag_on("NYASH_NYRT_SILENT_RESULT");
        if !silent {
            println!("Result: {}", exit_code);
        }
        // Optional GC metrics after program completes
        let want_json = crate::env_flags::flag_on("NYASH_GC_METRICS_JSON");
        let want_text = crate::env_flags::flag_on("NYASH_GC_METRICS");
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
            let sp_interval = crate::env_flags::u64_or("NYASH_GC_COLLECT_SP", 0);
            let alloc_thresh = crate::env_flags::u64_or("NYASH_GC_COLLECT_ALLOC", 0);
            let auto_sp = crate::env_flags::flag_default_on("NYASH_LLVM_AUTO_SAFEPOINT");
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
            let alloc_warn_threshold = crate::env_flags::u64_or("NYASH_GC_ALLOC_THRESHOLD", 0);
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
