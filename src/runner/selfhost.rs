/*!
 * Runner selfhost helpers — Ny compiler pipeline (Ny -> JSON v0)
 *
 * Transitional shim: provides a stable entrypoint from callers, while the
 * heavy implementation currently lives in modes/common.rs. Next step will
 * migrate the full implementation here.
 */

use super::*;
use nyash_rust::{mir::MirCompiler, parser::NyashParser};

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

        // Keep `selfhost.rs` on route ordering and terminal accept.
        // Source extension/read/merge/preexpand/tmp staging lives in `source_prepare.rs`.
        let prepared = match crate::runner::modes::common_util::selfhost::source_prepare::prepare_selfhost_source(self, filename) {
            Some(prepared) => prepared,
            None => return false,
        };
        let source_name = prepared.source_name.as_str();
        crate::runner::modes::common_util::selfhost::child::emit_runtime_route_mode(
            crate::runner::modes::common_util::selfhost::child::ROUTE_MODE_PIPELINE_ENTRY,
            source_name,
        );

        // Optional macro pre-expand path for selfhost.
        // Keep this gate local: it is route ordering policy, not source materialization ownership.
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
                match NyashParser::parse_from_string(prepared.prepared_code.as_str()) {
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
        // Optional EXE-first route: when enabled, prefer external parser EXE.
        // If EXE parsing fails, fall back to the explicit stage-a-compat route.
        if crate::config::env::use_ny_compiler_exe() {
            crate::runner::modes::common_util::selfhost::child::emit_runtime_route_mode(
                crate::runner::modes::common_util::selfhost::child::ROUTE_MODE_MAINLINE,
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
                "[ny-compiler] exe route failed; falling back to stage-a-compat route: {}",
                filename
            );
        }

        // Preferred default: run Ny selfhost compiler program (lang/src/compiler/entry/compiler.hako).
        // This avoids inline embedding pitfalls and supports Stage-3 gating via args.
        {
            use crate::runner::modes::common_util::selfhost::stage_a_route;
            let verbose_level = crate::config::env::dump::cli_verbose_level();
            let exe = std::env::current_exe()
                .unwrap_or_else(|_| std::path::PathBuf::from("target/release/nyash"));
            let timeout_ms: u64 = crate::config::env::ny_compiler_timeout_ms();
            // Keep `selfhost.rs` on high-level route sequencing only.
            // Stage-A child spawn/setup and captured payload-family resolution live below.
            if let Some(resolved) = stage_a_route::try_capture_stage_a_module(
                &exe,
                source_name,
                &prepared.raw_code,
                timeout_ms,
                verbose_level,
            ) {
                return accept_stage_a_mir_module(
                    self,
                    source_name,
                    resolved.lane,
                    resolved.module,
                );
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
                    cmd.arg(py).arg(&prepared.tmp_path);
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
        // パイプライン（legacy selfhost archive helper など）を使う想定とし、ここでは常に Rust 既定
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
