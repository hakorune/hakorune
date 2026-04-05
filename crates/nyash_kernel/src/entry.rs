// Process entry point for NyRT.

// ---- Process entry (driver) ----
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main() -> i32 {
    // Phase 88: AOT 実行器でも Ring0Context は必須（PluginHost/ログなどが依存する）。
    // EXE 直起動では host 側の init が存在しないため、ここで先に初期化する。
    if nyash_rust::runtime::ring0::GLOBAL_RING0.get().is_none() {
        nyash_rust::runtime::ring0::init_global_ring0(nyash_rust::runtime::ring0::default_ring0());
    }

    // Initialize plugin host: prefer nyash.toml next to the executable; fallback to CWD
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));

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
    // Initialize a minimal runtime to back global hooks (GC/scheduler) for safepoints
    // Choose GC hooks based on env (default dev: Counting for observability unless explicitly off)
    let mut rt_builder = nyash_rust::runtime::NyashRuntimeBuilder::new();
    let gc_mode = nyash_rust::runtime::gc_mode::GcMode::from_env();
    let controller = std::sync::Arc::new(nyash_rust::runtime::gc_controller::GcController::new(
        gc_mode,
    ));
    rt_builder = rt_builder.with_gc_hooks(controller);
    let rt_hooks = rt_builder.build();
    nyash_rust::runtime::global_hooks::set_from_runtime(&rt_hooks);

    let mut inited = false;
    if let Some(dir) = &exe_dir {
        let candidate = dir.join("nyash.toml");
        if candidate.exists() {
            let _ =
                nyash_rust::runtime::init_global_plugin_host(candidate.to_string_lossy().as_ref());
            inited = true;
        }
    }
    if !inited {
        let _ = nyash_rust::runtime::init_global_plugin_host("nyash.toml");
    }
    // Optional verbosity
    if std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1") {
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
    }
    // Call exported Nyash entry if linked: `ny_main` (i64 -> return code normalized)
    unsafe {
        extern "C" {
            fn ny_main() -> i64;
        }
        // SAFETY: if not linked, calling will be an unresolved symbol at link-time; we rely on link step to include ny_main.
        let v = ny_main();
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
        let silent = std::env::var("NYASH_NYRT_SILENT_RESULT").ok().as_deref() == Some("1");
        if !silent {
            println!("Result: {}", exit_code);
        }
        // Optional GC metrics after program completes
        let want_json = std::env::var("NYASH_GC_METRICS_JSON").ok().as_deref() == Some("1");
        let want_text = std::env::var("NYASH_GC_METRICS").ok().as_deref() == Some("1");
        if want_json || want_text {
            let (sp, br, bw) = rt_hooks.gc.snapshot_counters().unwrap_or((0, 0, 0));
            // ✂️ REMOVED: Legacy JIT handles::len() - part of 42% deletable functions
            let handles = 0u64; // Placeholder: handles tracking removed with JIT archival
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
            let sp_interval = std::env::var("NYASH_GC_COLLECT_SP")
                .ok()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            let alloc_thresh = std::env::var("NYASH_GC_COLLECT_ALLOC")
                .ok()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            let auto_sp = std::env::var("NYASH_LLVM_AUTO_SAFEPOINT")
                .ok()
                .map(|v| v == "1")
                .unwrap_or(true);
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
            if let Ok(s) = std::env::var("NYASH_GC_ALLOC_THRESHOLD") {
                if let Ok(th) = s.parse::<u64>() {
                    if alloc_bytes > th {
                        eprintln!(
                            "[GC][warn] allocation bytes {} exceeded threshold {}",
                            alloc_bytes, th
                        );
                    }
                }
            }
        }

        // ✂️ REMOVED: Legacy JIT leak diagnostics - part of 42% deletable functions
        // Leak diagnostics functionality removed with JIT archival
        // handles::type_tally() no longer available in Plugin-First architecture
        crate::observe::flush();
        exit_code as i32
    }
}
