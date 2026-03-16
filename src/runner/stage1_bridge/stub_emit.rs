/*!
 * Stage-1 bridge stub emit output helper.
 *
 * Keeps Stage1 stub emit orchestration thin while delegating stdout
 * parse/validation and writeback policy to focused helpers.
 */

mod parse;
mod writeback;

use super::args::Stage1StubEmitMode;
use crate::cli::CliGroups;
use crate::runner::modes::common_util::io::ChildOutput;
use std::io::Write;
use std::process::Command;

impl Stage1StubEmitMode {
    fn log_tag(self) -> &'static str {
        match self {
            Self::MirJson => "emit-mir",
            Self::ProgramJsonV0 => "emit-program",
        }
    }
}

pub(super) fn run_capture(cmd: Command, groups: &CliGroups, mode: Stage1StubEmitMode) -> i32 {
    let timeout_ms = crate::config::env::ny_compiler_emit_timeout_ms();
    let output = match crate::runner::modes::common_util::io::spawn_with_timeout(cmd, timeout_ms) {
        Ok(output) => output,
        Err(error) => {
            crate::runtime::get_global_ring0()
                .log
                .error(&format!("[stage1-cli] failed to spawn stub: {}", error));
            return 97;
        }
    };
    handle_child_output(output, groups, mode, timeout_ms)
}

fn handle_child_output(
    output: ChildOutput,
    groups: &CliGroups,
    mode: Stage1StubEmitMode,
    timeout_ms: u64,
) -> i32 {
    if let Some(code) = timeout_exit_code(&output, mode, timeout_ms) {
        return code;
    }
    if let Some(code) = failed_exit_code(&output) {
        return code;
    }
    parse_and_write_payload(groups, &output, mode)
}

fn timeout_exit_code(
    output: &ChildOutput,
    mode: Stage1StubEmitMode,
    timeout_ms: u64,
) -> Option<i32> {
    if !output.timed_out {
        return None;
    }
    crate::runtime::get_global_ring0().log.error(&format!(
        "[stage1-cli] {}: stage1 stub timed out after {} ms",
        mode.log_tag(),
        timeout_ms
    ));
    Some(98)
}

fn failed_exit_code(output: &ChildOutput) -> Option<i32> {
    let code = output.exit_code.unwrap_or(1);
    if code == 0 {
        return None;
    }
    if !output.stderr.is_empty() {
        let _ = std::io::stderr().write_all(&output.stderr);
    }
    Some(code)
}

fn parse_and_write_payload(
    groups: &CliGroups,
    output: &ChildOutput,
    mode: Stage1StubEmitMode,
) -> i32 {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let payload = match parse::parse_payload(&stdout, mode) {
        Ok(payload) => payload,
        Err(error) => {
            crate::runtime::get_global_ring0().log.error(&error);
            return 98;
        }
    };
    match writeback::write(groups, payload) {
        Ok(()) => 0,
        Err(error) => {
            crate::runtime::get_global_ring0().log.error(&error);
            98
        }
    }
}
