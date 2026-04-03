use std::io::Read;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

use tempfile::NamedTempFile;

pub const ROUTE_RUNTIME_SELFHOST: &str = "SH-RUNTIME-SELFHOST";
pub const ROUTE_MODE_PIPELINE_ENTRY: &str = "pipeline-entry";
pub const ROUTE_MODE_STAGE_A: &str = "stage-a";
pub const ROUTE_MODE_EXE: &str = "exe";

// Shared result for the stage0 child route.
// Shell/process ownership stays here; callers only select program vs MIR payload.
pub struct CapturedJsonV0Lines {
    pub program_line: Option<String>,
    pub mir_line: Option<String>,
}

struct ChildCaptureFiles {
    stdout_tmp: NamedTempFile,
    stderr_tmp: NamedTempFile,
}

struct ChildCapturedOutput {
    stdout: String,
    stderr: String,
}

pub fn format_route_tag(route_id: &str, mode: &str, source: &str) -> String {
    format!(
        "[selfhost/route] id={} mode={} source={}",
        route_id, mode, source
    )
}

pub fn emit_route_tag(route_id: &str, mode: &str, source: &str) {
    eprintln!("{}", format_route_tag(route_id, mode, source));
}

pub fn emit_runtime_route_mode(mode: &str, source: &str) {
    emit_route_tag(ROUTE_RUNTIME_SELFHOST, mode, source);
}

fn build_stage0_child_command(
    exe: &Path,
    program: &Path,
    extra_args: &[&str],
    env_remove: &[&str],
    envs: &[(&str, &str)],
) -> Command {
    let mut cmd = Command::new(exe);
    crate::runner::child_env::apply_selfhost_compiler_env(&mut cmd);
    cmd.arg("--backend").arg("vm").arg(program);
    for a in extra_args {
        cmd.arg(a);
    }
    for k in env_remove {
        cmd.env_remove(k);
    }
    for (k, v) in envs {
        cmd.env(k, v);
    }
    cmd
}

fn create_capture_tempfile(label: &str) -> Option<NamedTempFile> {
    match NamedTempFile::new() {
        Ok(file) => Some(file),
        Err(e) => {
            crate::console_println!("[selfhost-child] temp {} file failed: {}", label, e);
            None
        }
    }
}

fn create_capture_files() -> Option<ChildCaptureFiles> {
    Some(ChildCaptureFiles {
        stdout_tmp: create_capture_tempfile("stdout")?,
        stderr_tmp: create_capture_tempfile("stderr")?,
    })
}

fn attach_capture_stdio(cmd: &mut Command, capture: &ChildCaptureFiles) -> Option<()> {
    let stdout_file = match capture.stdout_tmp.reopen() {
        Ok(file) => file,
        Err(e) => {
            crate::console_println!("[selfhost-child] reopen stdout temp failed: {}", e);
            return None;
        }
    };
    let stderr_file = match capture.stderr_tmp.reopen() {
        Ok(file) => file,
        Err(e) => {
            crate::console_println!("[selfhost-child] reopen stderr temp failed: {}", e);
            return None;
        }
    };
    cmd.stdout(Stdio::from(stdout_file));
    cmd.stderr(Stdio::from(stderr_file));
    Some(())
}

fn wait_for_child_or_timeout(child: &mut Child, timeout_ms: u64) -> Option<bool> {
    let ring0 = crate::runtime::ring0::get_global_ring0();
    let start = match ring0.time.monotonic_now() {
        Ok(t) => t,
        Err(e) => {
            crate::console_println!("[selfhost-child] monotonic_now failed: {}", e);
            return None;
        }
    };

    loop {
        match child.try_wait() {
            Ok(Some(_)) => return Some(false),
            Ok(None) => {
                if ring0.time.elapsed(start) >= Duration::from_millis(timeout_ms) {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Some(true);
                }
                sleep(Duration::from_millis(10));
            }
            Err(e) => {
                crate::console_println!("[selfhost-child] wait failed: {}", e);
                return None;
            }
        }
    }
}

fn read_capture_output(capture: &ChildCaptureFiles) -> ChildCapturedOutput {
    let mut stdout = String::new();
    if let Ok(mut file) = capture.stdout_tmp.reopen() {
        let _ = file.read_to_string(&mut stdout);
    }

    let mut stderr = String::new();
    if let Ok(mut file) = capture.stderr_tmp.reopen() {
        let _ = file.read_to_string(&mut stderr);
    }

    ChildCapturedOutput { stdout, stderr }
}

fn log_timed_out_capture(timeout_ms: u64, output: &ChildCapturedOutput) {
    let head = output.stdout.chars().take(200).collect::<String>();
    let err_head = output.stderr.chars().take(500).collect::<String>();
    crate::console_println!(
        "[selfhost-child] timeout after {} ms; stdout(head)='{}'",
        timeout_ms,
        head.replace('\n', "\\n")
    );
    if !err_head.is_empty() {
        crate::console_println!(
            "[selfhost-child] stderr(head)='{}'",
            err_head.replace('\n', "\\n")
        );
    }
}

fn extract_captured_json_lines(stdout: &str) -> CapturedJsonV0Lines {
    CapturedJsonV0Lines {
        program_line: crate::runner::modes::common_util::selfhost::json::first_json_v0_line(
            stdout,
        ),
        mir_line: crate::runner::modes::common_util::selfhost::json::first_mir_json_v0_line(
            stdout,
        ),
    }
}

/// Stage0 shell residue owner.
///
/// This function owns:
/// - child spawn under `--backend vm`
/// - timeout / kill / wait handling
/// - stdout/stderr temp-file capture
/// - first-line Program(JSON v0) / MIR(JSON v0) extraction
///
/// Callers must keep route policy out of this helper and only select which captured line they need.
/// - `exe`: path to nyash executable
/// - `program`: path to the Nyash script to run (e.g., lang/src/compiler/entry/compiler.hako)
/// - `timeout_ms`: kill child after this duration
/// - `extra_args`: additional args to pass after program (e.g., "--", "--read-tmp")
/// - `env_remove`: environment variable names to remove for the child
/// - `envs`: key/value pairs to set for the child
pub fn run_ny_program_capture_json_v0(
    exe: &Path,
    program: &Path,
    timeout_ms: u64,
    extra_args: &[&str],
    env_remove: &[&str],
    envs: &[(&str, &str)],
) -> Option<CapturedJsonV0Lines> {
    let mut cmd = build_stage0_child_command(exe, program, extra_args, env_remove, envs);
    let capture = create_capture_files()?;
    attach_capture_stdio(&mut cmd, &capture)?;
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            crate::console_println!("[selfhost-child] spawn failed: {}", e);
            return None;
        }
    };

    let timed_out = wait_for_child_or_timeout(&mut child, timeout_ms)?;
    let output = read_capture_output(&capture);

    if timed_out {
        log_timed_out_capture(timeout_ms, &output);
        return None;
    }

    Some(extract_captured_json_lines(&output.stdout))
}

pub fn run_ny_program_capture_json(
    exe: &Path,
    program: &Path,
    timeout_ms: u64,
    extra_args: &[&str],
    env_remove: &[&str],
    envs: &[(&str, &str)],
) -> Option<String> {
    run_ny_program_capture_json_v0(exe, program, timeout_ms, extra_args, env_remove, envs)
        .and_then(|captured| captured.program_line)
}

/// Thin MIR-line selector over the shared stage0 child capture owner.
pub fn run_ny_program_capture_mir_json(
    exe: &Path,
    program: &Path,
    timeout_ms: u64,
    extra_args: &[&str],
    env_remove: &[&str],
    envs: &[(&str, &str)],
) -> Option<String> {
    run_ny_program_capture_json_v0(exe, program, timeout_ms, extra_args, env_remove, envs)
        .and_then(|captured| captured.mir_line)
}

#[cfg(test)]
mod tests {
    #[test]
    fn route_tag_format_stable_pipeline_entry() {
        let line = super::format_route_tag(
            super::ROUTE_RUNTIME_SELFHOST,
            super::ROUTE_MODE_PIPELINE_ENTRY,
            "foo.hako",
        );
        assert_eq!(
            line,
            "[selfhost/route] id=SH-RUNTIME-SELFHOST mode=pipeline-entry source=foo.hako"
        );
    }

    #[test]
    fn route_tag_format_stable_stage_a() {
        let line = super::format_route_tag(
            super::ROUTE_RUNTIME_SELFHOST,
            super::ROUTE_MODE_STAGE_A,
            "foo.hako",
        );
        assert_eq!(
            line,
            "[selfhost/route] id=SH-RUNTIME-SELFHOST mode=stage-a source=foo.hako"
        );
    }

    #[test]
    fn route_tag_format_stable_exe() {
        let line = super::format_route_tag(
            super::ROUTE_RUNTIME_SELFHOST,
            super::ROUTE_MODE_EXE,
            "foo.hako",
        );
        assert_eq!(
            line,
            "[selfhost/route] id=SH-RUNTIME-SELFHOST mode=exe source=foo.hako"
        );
    }
}
