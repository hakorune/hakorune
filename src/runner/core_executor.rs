/*!
 * CoreExecutor — JSON v0 → Execute (boxed)
 *
 * Responsibility
 * - Single entry to execute a MIR(JSON) payload under Gate‑C/Core policy.
 * - Encapsulates: optional canonicalize, v1-bridge try, v0-parse fallback,
 *   OOB strict observation, and rc mapping via MIR Interpreter.
 *
 * Notes
 * - For now, execution uses the existing MIR Interpreter runner
 *   (execute_mir_module_quiet_exit). Later we can swap internals to call
 *   the Core Dispatcher directly without touching callers.
 */

use super::NyashRunner;
use std::io::Write;

pub fn run_json_v0(runner: &NyashRunner, json: &str) -> i32 {
    // Optional: direct Core Dispatcher via child nyash (boxed)
    // Toggle: HAKO_CORE_DIRECT=1 (alias: NYASH_CORE_DIRECT)
    let core_direct = std::env::var("HAKO_CORE_DIRECT").ok().as_deref() == Some("1")
        || std::env::var("NYASH_CORE_DIRECT").ok().as_deref() == Some("1");
    if core_direct {
        // Only attempt Core-Direct when payload already looks like MIR(JSON v0)
        // i.e., has functions/blocks keys. Stage‑B Program(JSON v0) must go through bridge first.
        let looks_like_mir = json.contains("\"functions\"") && json.contains("\"blocks\"");
        if looks_like_mir {
            // In-proc prototype (opt-in): HAKO_CORE_DIRECT_INPROC=1 (alias NYASH_CORE_DIRECT_INPROC)
            let core_direct_inproc = std::env::var("HAKO_CORE_DIRECT_INPROC").ok().as_deref()
                == Some("1")
                || std::env::var("NYASH_CORE_DIRECT_INPROC").ok().as_deref() == Some("1");
            if core_direct_inproc {
                if let Some(rc) = try_run_core_direct_inproc(runner, json) {
                    return rc;
                }
                crate::runtime::get_global_ring0()
                    .log
                    .warn("[core-exec] direct Core (inproc) failed; trying child wrapper");
            }
            if let Some(rc) = try_run_core_direct(json) {
                return rc;
            }
            crate::runtime::get_global_ring0()
                .log
                .warn("[core-exec] direct Core (child) failed; falling back to VM interpreter");
        }
        // else: skip direct Core and continue to bridge/VM path
    }
    let mut payload = json.to_string();

    // Optional: downconvert/canonicalize even for v1 when requested (dev diagnostics)
    if crate::config::env::nyvm_v1_downconvert() {
        if let Ok(j) =
            crate::runner::modes::common_util::core_bridge::canonicalize_module_json(&payload)
        {
            payload = j;
        }
    }

    // Prefer v1 bridge when schema_version is present (JSON v1). This must run
    // before the v0 fast-path because v1 payloads also contain `functions` and
    // `blocks`, which would otherwise be misrouted to the v0 loader.
    if payload.contains("\"schema_version\"") {
        match crate::runner::json_v1_bridge::try_parse_v1_to_module(&payload) {
            Ok(Some(module)) => {
                super::json_v0_bridge::maybe_dump_mir(&module);
                crate::runner::child_env::pre_run_reset_oob_if_strict();
                let rc = runner.execute_mir_module_quiet_exit(&module);
                if crate::config::env::oob_strict_fail() && crate::runtime::observe::oob_seen() {
                    crate::runtime::get_global_ring0()
                        .log
                        .error("[gate-c][oob-strict] Out-of-bounds observed → exit(1)");
                    return 1;
                }
                return rc;
            }
            Ok(None) => { /* fall through to v0 path */ }
            Err(e) => {
                eprintln!("❌ JSON v1 bridge error: {}", e);
                return 1;
            }
        }
    }

    // Fast-path: accept MIR(JSON v0) directly when it looks like a module (functions/blocks)
    if payload.contains("\"functions\"") && payload.contains("\"blocks\"") {
        match super::mir_json_v0::parse_mir_v0_to_module(&payload) {
            Ok(module) => {
                super::json_v0_bridge::maybe_dump_mir(&module);
                crate::runner::child_env::pre_run_reset_oob_if_strict();
                let rc = runner.execute_mir_module_quiet_exit(&module);
                if crate::config::env::oob_strict_fail() && crate::runtime::observe::oob_seen() {
                    crate::runtime::get_global_ring0()
                        .log
                        .error("[gate-c][oob-strict] Out-of-bounds observed → exit(1)");
                    return 1;
                }
                return rc;
            }
            Err(e) => {
                eprintln!("❌ MIR JSON v0 parse error: {}", e);
                return 1;
            }
        }
    }

    // For non‑v1 input, attempt canonicalization and v1 bridge (Stage‑B program → MIR).
    if let Ok(j) =
        crate::runner::modes::common_util::core_bridge::canonicalize_module_json(&payload)
    {
        payload = j;
    }
    match crate::runner::json_v1_bridge::try_parse_v1_to_module(&payload) {
        Ok(Some(module)) => {
            super::json_v0_bridge::maybe_dump_mir(&module);
            crate::runner::child_env::pre_run_reset_oob_if_strict();
            let rc = runner.execute_mir_module_quiet_exit(&module);
            if crate::config::env::oob_strict_fail() && crate::runtime::observe::oob_seen() {
                crate::runtime::get_global_ring0()
                    .log
                    .error("[gate-c][oob-strict] Out-of-bounds observed → exit(1)");
                return 1;
            }
            return rc;
        }
        Ok(None) => { /* fall through to v0 parse/execute */ }
        Err(e) => {
            eprintln!("❌ JSON v1 bridge error: {}", e);
            return 1;
        }
    }

    match super::json_v0_bridge::parse_json_v0_to_module(&payload) {
        Ok(mut module) => {
            if let Err(e) = maybe_merge_program_json_v0_imports(runner, &payload, &mut module) {
                eprintln!("{e}");
                return 1;
            }
            super::json_v0_bridge::maybe_dump_mir(&module);
            crate::runner::child_env::pre_run_reset_oob_if_strict();
            let rc = runner.execute_mir_module_quiet_exit(&module);
            if crate::config::env::oob_strict_fail() && crate::runtime::observe::oob_seen() {
                crate::runtime::get_global_ring0()
                    .log
                    .error("[gate-c][oob-strict] Out-of-bounds observed → exit(1)");
                return 1;
            }
            rc
        }
        Err(e) => {
            eprintln!("❌ JSON v0 bridge error: {}", e);
            1
        }
    }
}

fn maybe_merge_program_json_v0_imports(
    runner: &NyashRunner,
    json: &str,
    module: &mut crate::mir::MirModule,
) -> Result<(), String> {
    let import_targets = extract_program_json_v0_used_import_targets(json)?;
    if import_targets.is_empty() {
        return Ok(());
    }

    let import_module = compile_program_json_v0_imports_bundle(runner, &import_targets)?;
    for (name, f) in import_module.functions {
        if name == "main" {
            continue;
        }
        if module.functions.contains_key(&name) {
            return Err(format!(
                "[freeze:contract][json_v0/imports] duplicate function: {}",
                name
            ));
        }
        module.functions.insert(name, f);
    }
    Ok(())
}

fn extract_program_json_v0_used_import_targets(json: &str) -> Result<Vec<String>, String> {
    let v: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| format!("[freeze:contract][json_v0/imports] invalid JSON: {e}"))?;
    let obj = v
        .as_object()
        .ok_or_else(|| "[freeze:contract][json_v0/imports] expected object".to_string())?;
    let version = obj
        .get("version")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| "[freeze:contract][json_v0/imports] missing version".to_string())?;
    let kind = obj
        .get("kind")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "[freeze:contract][json_v0/imports] missing kind".to_string())?;
    if version != 0 || kind != "Program" {
        return Ok(Vec::new());
    }

    let Some(imports) = obj.get("imports") else {
        return Ok(Vec::new());
    };
    let Some(map) = imports.as_object() else {
        return Err("[freeze:contract][json_v0/imports] imports must be an object".to_string());
    };

    let mut used_vars: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    if let Some(body) = obj.get("body") {
        collect_program_json_v0_var_names(body, &mut used_vars);
    }

    let mut out: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for (alias, target) in map.iter() {
        let Some(s) = target.as_str() else {
            return Err(
                "[freeze:contract][json_v0/imports] imports value must be string".to_string(),
            );
        };
        let s = s.trim();
        if s.is_empty() {
            continue;
        }
        // Only compile+merge imports that are actually referenced by the Program(JSON v0) body.
        // This keeps the execution surface minimal and avoids pulling in unrelated selfhost modules.
        if used_vars.contains(alias) {
            out.insert(s.to_string());
        }
    }
    Ok(out.into_iter().collect())
}

fn collect_program_json_v0_var_names(
    v: &serde_json::Value,
    out: &mut std::collections::BTreeSet<String>,
) {
    match v {
        serde_json::Value::Object(obj) => {
            if obj.get("type").and_then(|t| t.as_str()) == Some("Var") {
                if let Some(name) = obj.get("name").and_then(|n| n.as_str()) {
                    out.insert(name.to_string());
                }
            }
            for (_, child) in obj.iter() {
                collect_program_json_v0_var_names(child, out);
            }
        }
        serde_json::Value::Array(arr) => {
            for child in arr {
                collect_program_json_v0_var_names(child, out);
            }
        }
        _ => {}
    }
}

fn compile_program_json_v0_imports_bundle(
    runner: &NyashRunner,
    targets: &[String],
) -> Result<crate::mir::MirModule, String> {
    use crate::mir::MirCompiler;
    use crate::parser::NyashParser;
    use crate::runner::modes::common_util::resolve::prelude_manager::PreludeManagerBox;
    use crate::runner::modes::common_util::resolve::strip::resolve_prelude_paths_profiled;
    use crate::using::resolver::resolve_using_target_common;

    struct EnvVarRestore {
        key: &'static str,
        prev: Option<std::ffi::OsString>,
    }
    impl EnvVarRestore {
        fn set(key: &'static str, value: &str) -> Self {
            let prev = std::env::var_os(key);
            std::env::set_var(key, value);
            Self { key, prev }
        }
    }
    impl Drop for EnvVarRestore {
        fn drop(&mut self) {
            match self.prev.take() {
                Some(v) => std::env::set_var(self.key, v),
                None => std::env::remove_var(self.key),
            }
        }
    }

    // Program(JSON v0) imports are compiled on-demand during `--json-file` execution.
    // This is an internal bridge path; force planner-required lowering so BoxCount rules
    // (which are dev/strict-gated) can be used without relying on caller env.
    let _guard_strict = EnvVarRestore::set("HAKO_JOINIR_STRICT", "1");
    let _guard_planner_required = EnvVarRestore::set("HAKO_JOINIR_PLANNER_REQUIRED", "1");

    let using_ctx = runner.init_using_context();
    let verbose = crate::config::env::cli_verbose();

    let mut prelude_paths: Vec<String> = Vec::new();
    let mut seen_prelude: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut cleaned_roots: Vec<String> = Vec::new();

    for tgt in targets {
        let resolved = resolve_using_target_common(
            tgt,
            &using_ctx.pending_modules,
            &using_ctx.module_roots,
            &using_ctx.using_paths,
            &using_ctx.packages,
            None,
            true,
            verbose,
        )
        .map_err(|e| format!("[freeze:contract][json_v0/imports] {e}"))?;

        let abs = std::fs::canonicalize(&resolved).map_err(|e| {
            format!(
                "[freeze:contract][json_v0/imports] canonicalize {}: {e}",
                resolved
            )
        })?;
        let path = abs.to_string_lossy().to_string();
        let src = std::fs::read_to_string(&path)
            .map_err(|e| format!("[freeze:contract][json_v0/imports] read {}: {e}", path))?;

        let (cleaned, nested_preludes) = resolve_prelude_paths_profiled(runner, &src, &path)
            .map_err(|e| format!("[freeze:contract][json_v0/imports] using: {e}"))?;
        cleaned_roots.push(cleaned);
        for p in nested_preludes {
            if seen_prelude.insert(p.clone()) {
                prelude_paths.push(p);
            }
        }
    }

    let mut main_src = String::new();
    for (idx, s) in cleaned_roots.iter().enumerate() {
        if idx > 0 {
            main_src.push_str("\n\n");
        }
        main_src.push_str(s);
    }

    let prelude_manager = PreludeManagerBox::new(runner);
    let merged = if crate::config::env::using_ast_enabled() {
        prelude_manager
            .merge_ast(&main_src, "<json_v0/imports>", &prelude_paths)
            .map_err(|e| format!("[freeze:contract][json_v0/imports] merge_ast: {e}"))?
    } else {
        prelude_manager
            .merge_text(&main_src, "<json_v0/imports>", &prelude_paths)
            .map_err(|e| format!("[freeze:contract][json_v0/imports] merge_text: {e}"))?
    };

    let ast = NyashParser::parse_from_string(&merged.merged_content)
        .map_err(|e| format!("[freeze:contract][json_v0/imports] parse: {e}"))?;
    let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);

    let mut compiler = MirCompiler::with_options(true);
    let compile = crate::runner::modes::common_util::source_hint::compile_with_source_hint(
        &mut compiler,
        ast,
        Some("<json_v0/imports>"),
    )
    .map_err(|e| format!("[freeze:contract][json_v0/imports] compile: {e}"))?;
    Ok(compile.module)
}

fn try_run_core_direct(json: &str) -> Option<i32> {
    // Generate a temporary Hako program that includes the Core dispatcher
    // and calls NyVmDispatcher.run(json), printing the numeric result.
    let tmp_dir = std::path::Path::new("tmp");
    let _ = std::fs::create_dir_all(tmp_dir);
    let script_path = tmp_dir.join("core_exec_direct.hako");
    // Escape JSON into Hako string literal (simple backslash+quote escaping)
    let mut j = String::new();
    for ch in json.chars() {
        match ch {
            '\\' => j.push_str("\\\\"),
            '"' => j.push_str("\\\""),
            _ => j.push(ch),
        }
    }
    let code = format!(
        "include \"lang/src/vm/core/dispatcher.hako\"\nstatic box Main {{ method main(args) {{ local j=\"{}\"; local r=NyVmDispatcher.run(j); return r }} }}\n",
        j
    );
    if let Ok(mut f) = std::fs::File::create(&script_path) {
        let _ = f.write_all(code.as_bytes());
    } else {
        return None;
    }
    // Determine nyash binary (current executable)
    let exe = std::env::current_exe().ok()?;
    let mut cmd = std::process::Command::new(exe);
    crate::runner::child_env::apply_core_wrapper_env(&mut cmd);
    let out = cmd
        .args(["--backend", "vm", script_path.to_string_lossy().as_ref()])
        .output()
        .ok()?;
    if !out.stdout.is_empty() {
        let _ = std::io::stdout().write_all(&out.stdout);
    }
    let rc = out.status.code().unwrap_or(1);
    Some(rc)
}

fn try_run_core_direct_inproc(runner: &NyashRunner, json: &str) -> Option<i32> {
    // Parse MIR(JSON v0) in-proc and execute via MIR Interpreter quietly.
    // This bypasses the child Hako wrapper and reduces latency/recursion risks.
    match crate::runner::json_v0_bridge::parse_json_v0_to_module(json) {
        Ok(module) => {
            crate::runner::child_env::pre_run_reset_oob_if_strict();
            let rc = runner.execute_mir_module_quiet_exit(&module);
            if crate::config::env::oob_strict_fail() && crate::runtime::observe::oob_seen() {
                crate::runtime::get_global_ring0()
                    .log
                    .error("[gate-c][oob-strict] Out-of-bounds observed → exit(1)");
                return Some(1);
            }
            Some(rc)
        }
        Err(_) => None,
    }
}
