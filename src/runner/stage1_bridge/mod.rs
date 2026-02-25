/*!
 * Stage-1 CLI bridge — delegate to Hako Stage1 stub when explicitly enabled.
 *
 * - Entry: NYASH_USE_STAGE1_CLI=1 (default OFF).
 * - Toggle guard for child recursion: NYASH_STAGE1_CLI_CHILD=1 (set by bridge).
 * - Entry path override: STAGE1_CLI_ENTRY or HAKORUNE_STAGE1_ENTRY (optional).
 * - Mode toggles:
 *     - STAGE1_EMIT_PROGRAM_JSON=1 : emit program-json <source.hako>
 *     - STAGE1_EMIT_MIR_JSON=1     : emit mir-json (<source.hako> or STAGE1_PROGRAM_JSON)
 *     - STAGE1_BACKEND={vm|llvm} hint for run path (default: CLI backend)
 *
 * Notes
 * - This bridge aims to keep Rust Stage0 thin: it only invokes the Stage1 stub
 *   (lang/src/runner/stage1_cli.hako) with script args and exits with the stub's code.
 * - When toggles are unset or this is a child invocation, the bridge is a no-op.
 */

mod args;
mod direct_route;
mod env;
mod modules;
mod plan;

use super::NyashRunner;
use crate::cli::CliGroups;
use crate::config;
use crate::config::env::stage1;
use crate::mir::MirPrinter;
use std::io::Write;
use std::path::{Path, PathBuf};

const EMBEDDED_STAGE1_ENTRY_SRC: &str = include_str!("../../../lang/src/runner/stage1_cli.hako");
const EMBEDDED_STAGE1_ENTRY_FILE: &str = "stage1_cli.embedded.hako";

fn resolve_stage1_entry_path() -> Result<String, String> {
    if let Some(entry) = stage1::entry_override() {
        if Path::new(&entry).exists() {
            return Ok(entry);
        }
        return Err(format!("[stage1-cli] entry not found: {}", entry));
    }

    // Default route: materialize embedded Stage1 CLI into tmpdir.
    // This removes runtime dependency on lang/src/runner/stage1_cli.hako.
    let base = std::env::var("NYASH_STAGE1_EMBED_DIR")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("hakorune_stage1_embedded"));

    std::fs::create_dir_all(&base).map_err(|e| {
        format!(
            "[stage1-cli] embedded entry mkdir failed: {} ({})",
            base.display(),
            e
        )
    })?;

    let entry_path = base.join(EMBEDDED_STAGE1_ENTRY_FILE);
    match std::fs::read_to_string(&entry_path) {
        Ok(existing) if existing == EMBEDDED_STAGE1_ENTRY_SRC => {}
        _ => {
            let tmp_path = base.join(format!(
                "{}.tmp-{}",
                EMBEDDED_STAGE1_ENTRY_FILE,
                std::process::id()
            ));
            std::fs::write(&tmp_path, EMBEDDED_STAGE1_ENTRY_SRC).map_err(|e| {
                format!(
                    "[stage1-cli] embedded entry write failed: {} ({})",
                    tmp_path.display(),
                    e
                )
            })?;
            std::fs::rename(&tmp_path, &entry_path)
                .or_else(|_| {
                    let _ = std::fs::remove_file(&entry_path);
                    std::fs::rename(&tmp_path, &entry_path)
                })
                .map_err(|e| {
                    format!(
                        "[stage1-cli] embedded entry install failed: {} ({})",
                        entry_path.display(),
                        e
                    )
                })?;
        }
    }
    Ok(entry_path.to_string_lossy().to_string())
}

impl NyashRunner {
    /// Emit Program(JSON v0) using Stage-1 stub and write to a file.
    pub(crate) fn emit_program_json_v0(
        &self,
        groups: &CliGroups,
        out_path: &str,
    ) -> Result<(), String> {
        let source = stage1::input_path()
            .or_else(|| groups.input.file.as_ref().cloned())
            .ok_or_else(|| "emit-program-json-v0 requires an input file".to_string())?;

        let code = std::fs::read_to_string(&source)
            .map_err(|e| format!("emit-program-json-v0 read error: {}: {}", source, e))?;
        let out = crate::stage1::program_json_v0::source_to_program_json_v0(&code)
            .map_err(|e| format!("emit-program-json-v0: {}", e))?;
        std::fs::write(out_path, out)
            .map_err(|e| format!("emit-program-json-v0 write {} failed: {}", out_path, e))?;
        Ok(())
    }

    /// If enabled, run the Stage-1 CLI stub as a child process and return its exit code.
    /// Returns None when the bridge is not engaged.
    pub(crate) fn maybe_run_stage1_cli_stub(&self, groups: &CliGroups) -> Option<i32> {
        // Temporary trace: confirm the bridge is evaluated
        if config::env::cli_verbose_level() == 2 {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[stage1-bridge/trace] maybe_run_stage1_cli_stub invoked"
            ));
        }

        // Guard: skip if child invocation
        if stage1::child_invocation() {
            if config::env::cli_verbose_level() == 2 {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[stage1-bridge/trace] skip: NYASH_STAGE1_CLI_CHILD=1"
                ));
            }
            return None;
        }

        // Guard: skip if not enabled
        if !stage1::enabled() {
            if config::env::cli_verbose_level() == 2 {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[stage1-bridge/trace] skip: NYASH_USE_STAGE1_CLI!=1"
                ));
            }
            return None;
        }

        if config::env::cli_verbose() || config::env::cli_verbose_level() == 2 {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[stage1-bridge/debug] NYASH_USE_STAGE1_CLI=1 detected"
            ));
        }

        // Locate Stage-1 CLI entry
        let entry = match resolve_stage1_entry_path() {
            Ok(entry) => entry,
            Err(msg) => {
                crate::runtime::get_global_ring0().log.error(&msg);
                return Some(97);
            }
        };

        // Build args
        let args_result = args::build_stage1_args(groups);
        let route_plan = plan::decide(&args_result);
        match route_plan.route {
            plan::Stage1BridgeRoute::BinaryOnlyEmitMirDirect => {
                crate::runtime::get_global_ring0().log.warn(&format!(
                    "[stage1-cli] emit-mir: binary-only direct route engaged ({})",
                    route_plan.reason
                ));
                match direct_route::emit_mir_binary_only_direct(self, groups) {
                    Ok(()) => return Some(0),
                    Err(e) => {
                        crate::runtime::get_global_ring0()
                            .log
                            .error(&format!("[stage1-cli] emit-mir(binary-only): {}", e));
                        return Some(98);
                    }
                }
            }
            plan::Stage1BridgeRoute::BinaryOnlyRunDirect => {
                crate::runtime::get_global_ring0().log.warn(&format!(
                    "[stage1-cli] run: binary-only direct route engaged ({})",
                    route_plan.reason
                ));
                match direct_route::run_binary_only_direct(self, groups) {
                    Ok(rc) => return Some(rc),
                    Err(e) => {
                        crate::runtime::get_global_ring0()
                            .log
                            .error(&format!("[stage1-cli] run(binary-only): {}", e));
                        return Some(98);
                    }
                }
            }
            plan::Stage1BridgeRoute::Stage1Stub => {}
        }

        // Collect modules list and module_roots list (single TOML parse path)
        let module_env_lists = modules::collect_module_env_lists();

        // Prepare command
        let exe = std::env::current_exe().unwrap_or_else(|_| {
            // Fallback to release binary path when current_exe is unavailable
            std::path::PathBuf::from("target/release/nyash")
        });
        let mut cmd = std::process::Command::new(exe);
        let entry_fn =
            std::env::var("NYASH_ENTRY").unwrap_or_else(|_| "Stage1CliMain.main/0".to_string());
        cmd.arg(&entry).arg("--");
        for a in &args_result.args {
            cmd.arg(a);
        }

        // Set environment variables for args
        if let Some(json) = args_result.env_script_args.as_ref() {
            cmd.env("NYASH_SCRIPT_ARGS_JSON", json);
        }
        if let Some(src) = args_result.source_env.as_ref() {
            cmd.env("STAGE1_SOURCE", src);
        }
        if let Some(pjson) = args_result.progjson_env.as_ref() {
            cmd.env("STAGE1_PROGRAM_JSON", pjson);
        }

        // Pass source text inline to avoid FileBox dependency when possible.
        if args_result.source_env.is_none() {
            if let Some(src_path) = groups.input.file.as_ref() {
                if let Ok(text) = std::fs::read_to_string(&src_path) {
                    cmd.env("STAGE1_SOURCE_TEXT", text);
                }
            }
        } else if let Some(src_path) = args_result.source_env.as_ref() {
            if let Ok(text) = std::fs::read_to_string(src_path) {
                cmd.env("STAGE1_SOURCE_TEXT", text);
            }
        }

        // Configure environment
        env::configure_stage1_env(
            &mut cmd,
            &entry_fn,
            &args_result.args,
            module_env_lists.modules_list,
            module_env_lists.module_roots_list,
        );

        let emit_program = stage1::emit_program_json();
        // Emit routes: capture stdout for JSON v0 produced by Stage-1 CLI.
        if args_result.emit_mir || emit_program {
            let timeout_ms = crate::config::env::ny_compiler_emit_timeout_ms();
            let output =
                match crate::runner::modes::common_util::io::spawn_with_timeout(cmd, timeout_ms) {
                    Ok(o) => o,
                    Err(e) => {
                        crate::runtime::get_global_ring0()
                            .log
                            .error(&format!("[stage1-cli] failed to spawn stub: {}", e));
                        return Some(97);
                    }
                };
            if output.timed_out {
                let mode_tag = if args_result.emit_mir {
                    "emit-mir"
                } else {
                    "emit-program"
                };
                crate::runtime::get_global_ring0().log.error(&format!(
                    "[stage1-cli] {}: stage1 stub timed out after {} ms",
                    mode_tag, timeout_ms
                ));
                return Some(98);
            }
            let code = output.exit_code.unwrap_or(1);
            if code != 0 {
                if !output.stderr.is_empty() {
                    let _ = std::io::stderr().write_all(&output.stderr);
                }
                return Some(code);
            }
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            if args_result.emit_mir {
                let line =
                    match crate::runner::modes::common_util::selfhost::json::first_mir_json_v0_line(
                        &stdout,
                    ) {
                        Some(l) => l,
                        None => {
                            crate::runtime::get_global_ring0().log.error(
                                "[stage1-cli] emit-mir: no MIR(JSON v0) found in stub output",
                            );
                            return Some(98);
                        }
                    };
                let module =
                    match crate::runner::modes::common_util::selfhost::json::parse_mir_json_v0_line(
                        &line,
                    ) {
                        Ok(m) => m,
                        Err(e) => {
                            crate::runtime::get_global_ring0().log.error(&format!(
                                "[stage1-cli] emit-mir: MIR(JSON v0) parse error: {}",
                                e
                            ));
                            return Some(98);
                        }
                    };
                super::json_v0_bridge::maybe_dump_mir(&module);

                if groups.debug.dump_mir {
                    let mut printer = if groups.debug.mir_verbose {
                        MirPrinter::verbose()
                    } else {
                        MirPrinter::new()
                    };
                    if groups.debug.mir_verbose_effects {
                        printer.set_show_effects_inline(true);
                    }
                    println!("{}", printer.print_module(&module));
                }
                let out_path = groups.emit.emit_mir_json.clone().or_else(|| {
                    std::env::var("NYASH_STAGE1_EMIT_MIR_OUT")
                        .ok()
                        .or_else(|| std::env::var("HAKO_STAGE1_EMIT_MIR_OUT").ok())
                });
                if let Some(path) = out_path {
                    let p = std::path::Path::new(&path);
                    if let Err(e) =
                        crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(&module, p)
                    {
                        eprintln!("❌ MIR JSON emit error: {}", e);
                        return Some(98);
                    }
                    println!("MIR JSON written: {}", p.display());
                } else {
                    println!("{}", line);
                }
                return Some(0);
            }

            let line = match crate::runner::modes::common_util::selfhost::json::first_json_v0_line(
                &stdout,
            ) {
                Some(l) => l,
                None => {
                    crate::runtime::get_global_ring0().log.error(
                        "[stage1-cli] emit-program: no Program(JSON v0) found in stub output",
                    );
                    return Some(98);
                }
            };
            if let Err(e) =
                crate::runner::modes::common_util::selfhost::json::parse_json_v0_line(&line)
            {
                crate::runtime::get_global_ring0().log.error(&format!(
                    "[stage1-cli] emit-program: Program(JSON v0) parse error: {}",
                    e
                ));
                return Some(98);
            }
            let out_path = groups.emit.emit_program_json_v0.clone().or_else(|| {
                std::env::var("NYASH_STAGE1_EMIT_PROGRAM_OUT")
                    .ok()
                    .or_else(|| std::env::var("HAKO_STAGE1_EMIT_PROGRAM_OUT").ok())
            });
            if let Some(path) = out_path {
                let p = std::path::Path::new(&path);
                if let Err(e) = std::fs::write(p, line.as_bytes()) {
                    eprintln!("❌ Program JSON emit error: {}", e);
                    return Some(98);
                };
                println!("Program JSON written: {}", p.display());
            } else {
                println!("{}", line);
            }
            return Some(0);
        }

        crate::cli_v!(
            "[stage1-cli] delegating to stub: {} -- {}",
            entry,
            args_result.args.join(" ")
        );

        // Execute
        let status = match cmd.status() {
            Ok(s) => s,
            Err(e) => {
                crate::runtime::get_global_ring0()
                    .log
                    .error(&format!("[stage1-cli] failed to spawn stub: {}", e));
                return Some(97);
            }
        };
        Some(status.code().unwrap_or(1))
    }
}
