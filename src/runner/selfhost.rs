/*!
 * Runner selfhost helpers — Ny compiler pipeline (Ny -> JSON v0)
 *
 * Transitional shim: provides a stable entrypoint from callers, while the
 * heavy implementation currently lives in modes/common.rs. Next step will
 * migrate the full implementation here.
 */

use super::*;
use nyash_rust::{mir::MirCompiler, parser::NyashParser};
use std::fs;

// ============================================================================
// Selfhost pipeline helpers
// ============================================================================

/// Dump MIR if NYASH_CLI_VERBOSE is enabled.
fn maybe_dump_mir_verbose(module: &crate::mir::MirModule) {
    if crate::config::env::cli_verbose() {
        super::json_v0_bridge::maybe_dump_mir(module);
    }
}

/// Check OOB strict mode and exit(1) if out-of-bounds was observed.
fn check_oob_strict_exit() {
    if crate::config::env::oob_strict_fail() && crate::runtime::observe::oob_seen() {
        // Phase 88: Ring0Context 経由でエラーログ出力
        let ring0 = crate::runtime::ring0::get_global_ring0();
        ring0
            .log
            .error("[selfhost][oob-strict] Out-of-bounds observed → exit(1)");
        std::process::exit(1);
    }
}

/// Run module and check OOB, with pre-run reset.
fn execute_with_oob_check(runner: &NyashRunner, module: &crate::mir::MirModule) {
    crate::runner::child_env::pre_run_reset_oob_if_strict();
    runner.execute_mir_module(module);
    check_oob_strict_exit();
}

fn accept_stage_a_mir_module(
    runner: &NyashRunner,
    source_name: &str,
    lane: &str,
    module: crate::mir::MirModule,
) -> bool {
    use crate::runner::modes::common_util::selfhost::runtime_route_contract;

    runtime_route_contract::emit_accepted_mir(source_name, lane);
    super::json_v0_bridge::maybe_dump_mir(&module);
    if crate::config::env::ny_compiler_emit_only() {
        return false;
    }
    runner.execute_mir_module(&module);
    true
}

impl NyashRunner {
    /// Selfhost (Ny -> JSON v0) pipeline: EXE/VM/Python フォールバック含む
    pub(crate) fn try_run_selfhost_pipeline(&self, filename: &str) -> bool {
        use std::io::Write;

        // Phase 95: PluginHost 初期化（環境変数で制御）+ ConsoleService 使用デモ
        if std::env::var("NYASH_USE_PLUGIN_HOST").ok().as_deref() == Some("1") {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            match crate::runtime::initialize_runtime(ring0) {
                Ok(()) => {
                    // Phase 105.5: Unified console output via macro
                    crate::console_println!("[selfhost] PluginHost initialized successfully");
                }
                Err(e) => {
                    // Phase 105.5: Unified console output via macro
                    crate::console_println!("[selfhost] CoreInitError: {}", e);
                    return false;
                }
            }
        }

        // Phase 25.1b: guard selfhost pipeline to Ny-only sources.
        // `.hako` / other extensionsは Stage‑B / JSON v0 bridge 側の責務なので、
        // ここでは Ny/Nyash 拡張子以外は即座にスキップする。
        let path = std::path::Path::new(filename);
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            match ext {
                "ny" | "nyash" => { /* continue */ }
                "hako" => {
                    // Opt-in: allow .hako to flow through Ny compiler when explicitly requested.
                    if !crate::config::env::use_ny_compiler() {
                        crate::cli_v!(
                            "[ny-compiler] skip selfhost pipeline for .hako (ext={}); enable NYASH_USE_NY_COMPILER=1 to force",
                            ext
                        );
                        return false;
                    }
                }
                _ => {
                    crate::cli_v!(
                        "[ny-compiler] skip selfhost pipeline for non-Ny source: {} (ext={})",
                        filename,
                        ext
                    );
                    return false;
                }
            }
        } else {
            // No extension: treat as non-Ny for safety
            crate::cli_v!(
                "[ny-compiler] skip selfhost pipeline for source without extension: {}",
                filename
            );
            return false;
        }

        let source_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(filename);
        crate::runner::modes::common_util::selfhost::child::emit_runtime_route_mode(
            crate::runner::modes::common_util::selfhost::child::ROUTE_MODE_PIPELINE_ENTRY,
            source_name,
        );

        // Read input source
        let code = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0.log.error(&format!("[ny-compiler] read error: {}", e));
                return false;
            }
        };
        // Optional Phase-15: using prelude merge (text-based for speed)
        let mut code_ref: std::borrow::Cow<'_, str> = std::borrow::Cow::Borrowed(&code);
        if crate::config::env::enable_using() {
            let using_ast = crate::config::env::using_ast_enabled();
            if using_ast {
                // Text-based merge: faster for inline/selfhost execution
                match crate::runner::modes::common_util::resolve::merge_prelude_text(
                    self, &code, filename,
                ) {
                    Ok(merged) => {
                        code_ref = std::borrow::Cow::Owned(merged);
                    }
                    Err(e) => {
                        let ring0 = crate::runtime::ring0::get_global_ring0();
                        ring0
                            .log
                            .error(&format!("[ny-compiler] using text merge error: {}", e));
                        return false;
                    }
                }
            } else {
                // Legacy: strip only (no prelude merge)
                match crate::runner::modes::common_util::resolve::resolve_prelude_paths_profiled(
                    self, &code, filename,
                ) {
                    Ok((clean, paths)) => {
                        if !paths.is_empty() {
                            let ring0 = crate::runtime::ring0::get_global_ring0();
                            ring0.log.error("[ny-compiler] using: AST prelude merge is disabled in this profile. Enable NYASH_USING_AST=1 or remove 'using' lines.");
                            return false;
                        }
                        code_ref = std::borrow::Cow::Owned(clean);
                    }
                    Err(e) => {
                        let ring0 = crate::runtime::ring0::get_global_ring0();
                        ring0.log.error(&format!("[ny-compiler] {}", e));
                        return false;
                    }
                }
            }
        }

        // Promote dev sugar to standard: pre-expand line-head '@name[:T] = expr' to 'local name[:T] = expr'
        {
            let expanded =
                crate::runner::modes::common_util::resolve::preexpand_at_local(code_ref.as_ref());
            code_ref = std::borrow::Cow::Owned(expanded);
        }

        // Write to tmp/ny_parser_input.ny (as expected by Ny parser v0), unless forced to reuse existing tmp
        let use_tmp_only = crate::config::env::ny_compiler_use_tmp_only();
        let tmp_dir = std::path::Path::new("tmp");
        if let Err(e) = std::fs::create_dir_all(tmp_dir) {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0
                .log
                .error(&format!("[ny-compiler] mkdir tmp failed: {}", e));
            return false;
        }

        // Optional macro pre-expand path for selfhost
        // Gate: NYASH_MACRO_SELFHOST_PRE_EXPAND={1|auto|0}
        // `auto` is currently disabled for stability; use explicit `1` to opt in.
        {
            let preenv = crate::config::env::macro_selfhost_pre_expand().or_else(|| {
                if crate::r#macro::enabled() {
                    Some("auto".to_string())
                } else {
                    None
                }
            });
            let do_pre = match preenv.as_deref() {
                Some("1") => true,
                Some("auto") => false,
                _ => false,
            };
            if do_pre && crate::r#macro::enabled() {
                crate::cli_v!(
                    "[ny-compiler] selfhost macro pre-expand: engaging (mode={:?})",
                    preenv
                );
                match NyashParser::parse_from_string(code_ref.as_ref()) {
                    Ok(ast0) => {
                        let ast = crate::r#macro::maybe_expand_and_dump(&ast0, false);
                        // Compile to MIR and execute on the unified runtime path.
                        let mut mir_compiler = MirCompiler::with_options(true);
                        match mir_compiler.compile(ast) {
                            Ok(result) => {
                                execute_with_oob_check(self, &result.module);
                                return true;
                            }
                            Err(e) => {
                                let ring0 = crate::runtime::ring0::get_global_ring0();
                                ring0.log.error(&format!(
                                    "[ny-compiler] pre-expand compile error: {}",
                                    e
                                ));
                                return false;
                            }
                        }
                    }
                    Err(e) => {
                        let ring0 = crate::runtime::ring0::get_global_ring0();
                        ring0
                            .log
                            .error(&format!("[ny-compiler] pre-expand parse error: {}", e));
                        return false;
                    }
                }
            }
        }
        let tmp_path = tmp_dir.join("ny_parser_input.ny");
        if !use_tmp_only {
            match std::fs::File::create(&tmp_path) {
                Ok(mut f) => {
                    if let Err(e) = f.write_all(code_ref.as_bytes()) {
                        let ring0 = crate::runtime::ring0::get_global_ring0();
                        ring0
                            .log
                            .error(&format!("[ny-compiler] write tmp failed: {}", e));
                        return false;
                    }
                }
                Err(e) => {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0
                        .log
                        .error(&format!("[ny-compiler] open tmp failed: {}", e));
                    return false;
                }
            }
        }
        // Optional EXE-first route: when enabled, prefer external parser EXE.
        // If EXE parsing fails, fall back to stage-a route.
        if crate::config::env::use_ny_compiler_exe() {
            crate::runner::modes::common_util::selfhost::child::emit_runtime_route_mode(
                crate::runner::modes::common_util::selfhost::child::ROUTE_MODE_EXE,
                source_name,
            );
            let timeout_ms = crate::config::env::ny_compiler_timeout_ms();
            if let Some(module) =
                super::modes::common_util::selfhost_exe::exe_try_parse_json_v0(filename, timeout_ms)
            {
                maybe_dump_mir_verbose(&module);
                if crate::config::env::ny_compiler_emit_only() {
                    return false;
                }
                execute_with_oob_check(self, &module);
                return true;
            }
            crate::cli_v!(
                "[ny-compiler] exe route failed; falling back to stage-a route: {}",
                filename
            );
        }

        // Preferred default: run Ny selfhost compiler program (lang/src/compiler/entry/compiler.hako).
        // This avoids inline embedding pitfalls and supports Stage-3 gating via args.
        {
            use crate::runner::modes::common_util::selfhost::{
                child, stage_a_compat_bridge, stage_a_policy, stage_a_spawn,
            };
            let verbose_level = crate::config::env::dump::cli_verbose_level();
            let exe = std::env::current_exe()
                .unwrap_or_else(|_| std::path::PathBuf::from("target/release/nyash"));
            // Phase 28.2: selfhost compiler entry moved under lang/src/compiler/entry
            let parser_prog = std::path::Path::new("lang/src/compiler/entry/compiler.hako");
            if parser_prog.exists() {
                crate::runner::modes::common_util::selfhost::child::emit_runtime_route_mode(
                    crate::runner::modes::common_util::selfhost::child::ROUTE_MODE_STAGE_A,
                    source_name,
                );
                // Phase 29x X21 contract:
                // non-strict stage-a compat lanes are explicit-only.
                stage_a_policy::enforce_stage_a_compat_policy_or_exit(source_name);
                // Phase 28.2: observation log (NYASH_CLI_VERBOSE>=2)
                if verbose_level >= 2 {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0.log.info(&format!(
                        "[selfhost/ny] spawning Ny compiler child process: {}",
                        parser_prog.display()
                    ));
                }
                // Build args/env through the Stage-A spawn contract box.
                // D5-min1: runtime route compiles through Stage-B surface (`--stage-b --stage3`)
                // so that Program(JSON v0) emission is stable under the same contracts as selfhost gates.
                let extra_owned = stage_a_spawn::build_stage_a_child_extra_args();
                let extra: Vec<&str> = extra_owned.iter().map(|s| s.as_str()).collect();
                let child_env_owned = stage_a_spawn::build_stage_a_child_env(&code);
                let child_env: Vec<(&str, &str)> = child_env_owned
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                let timeout_ms: u64 = crate::config::env::ny_compiler_timeout_ms();
                // Phase 28.2 fix: Don't override HAKO_ALLOW_USING_FILE here.
                // apply_selfhost_compiler_env() already sets HAKO_ALLOW_USING_FILE=1 to enable
                // module resolution for `using lang.compiler.parser.box`, etc.
                // The extra envs here previously overrode it back to "0", breaking module loading.
                if let Some(captured) = child::run_ny_program_capture_json_v0(
                    &exe,
                    parser_prog,
                    timeout_ms,
                    &extra,
                    &["NYASH_USE_NY_COMPILER", "NYASH_CLI_VERBOSE"],
                    &child_env,
                ) {
                    // Keep `selfhost.rs` on Stage-A spawn/route sequencing only.
                    // Captured payload-family resolution now lives in `stage_a_compat_bridge.rs`.
                    if let Some(resolved) = stage_a_compat_bridge::resolve_captured_payload_to_mir(
                        &exe,
                        source_name,
                        timeout_ms,
                        verbose_level,
                        captured.mir_line.as_deref(),
                        captured.program_line.as_deref(),
                    ) {
                        return accept_stage_a_mir_module(
                            self,
                            source_name,
                            resolved.lane,
                            resolved.module,
                        );
                    }
                }
            }
        }

        // Python MVP (optional): lightweight harness to produce JSON v0.
        // Phase 25.1b: default OFF（NYASH_NY_COMPILER_USE_PY=1 のときだけ有効）。
        if crate::config::env::ny_compiler_use_py() {
            if let Ok(py3) = which::which("python3") {
                let py = std::path::Path::new("tools/ny_parser_mvp.py");
                if py.exists() {
                    let mut cmd = std::process::Command::new(&py3);
                    // Phase 25.1b: Use selfhost compiler env for consistency
                    crate::runner::child_env::apply_selfhost_compiler_env(&mut cmd);
                    cmd.arg(py).arg(&tmp_path);
                    let timeout_ms = crate::config::env::ny_compiler_timeout_ms();
                    let out =
                        match super::modes::common_util::io::spawn_with_timeout(cmd, timeout_ms) {
                            Ok(o) => o,
                            Err(e) => {
                                let ring0 = crate::runtime::ring0::get_global_ring0();
                                ring0
                                    .log
                                    .error(&format!("[ny-compiler] python harness failed: {}", e));
                                return false;
                            }
                        };
                    if !out.timed_out {
                        if let Ok(s) = String::from_utf8(out.stdout) {
                            if let Some(line) = crate::runner::modes::common_util::selfhost::json::first_json_v0_line(&s) {
                                match super::json_v0_bridge::parse_json_v0_to_module(&line) {
                                    Ok(module) => {
                                        maybe_dump_mir_verbose(&module);
                                        if crate::config::env::ny_compiler_emit_only() {
                                            return false;
                                        }
                                        execute_with_oob_check(self, &module);
                                        return true;
                                    }
                                    Err(e) => {
                                        let ring0 = crate::runtime::ring0::get_global_ring0();
                                        ring0.log.error(&format!("[ny-compiler] json parse error: {}", e));
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // Fallback: inline VM run (embed source into a tiny wrapper that prints JSON)
        // Phase 25.1b: この経路は Ny selfhost 実験用だったが、現在は不安定かつ .hako 側 selfhost builder の
        // デバッグを阻害するため、既定で無効化する。Ny selfhost が必要な場合は別の .sh ベースの
        // パイプライン（tools/ny_selfhost_inline.sh など）を使う想定とし、ここでは常に Rust 既定
        // パスへフォールバックする。
        crate::cli_v!("[ny-compiler] inline selfhost pipeline disabled (Phase 25.1b); falling back to default path");

        // Dev-only escape hatch: allow forcing the old inline path when explicitly requested.
        if crate::config::env::selfhost_inline_force() {
            match super::json_v0_bridge::parse_json_v0_to_module("") {
                Ok(module) => {
                    maybe_dump_mir_verbose(&module);
                    if crate::config::env::ny_compiler_emit_only() {
                        return false;
                    }
                    execute_with_oob_check(self, &module);
                    return true;
                }
                Err(e) => {
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    ring0.log.error(&format!("❌ JSON v0 bridge error: {}", e));
                    return false;
                }
            }
        }

        // Default path: always fall back to existing Rust runner.
        return false;
    }
}
