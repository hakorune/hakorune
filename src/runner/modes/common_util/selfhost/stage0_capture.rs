use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

use tempfile::NamedTempFile;

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
        program_line: crate::runner::modes::common_util::selfhost::json::first_json_v0_line(stdout),
        mir_line: crate::runner::modes::common_util::selfhost::json::first_mir_json_v0_line(stdout),
    }
}

/// Route-neutral Stage0 capture plumbing.
///
/// This function owns:
/// - timeout / kill / wait handling
/// - stdout/stderr temp-file capture
/// - first-line Program(JSON v0) / MIR(JSON v0) extraction
///
/// Callers must build the route-specific child command elsewhere and keep route policy out of this helper.
/// - `cmd`: prebuilt child command, including the backend/route selection
/// - `timeout_ms`: kill child after this duration
pub fn run_captured_json_v0_command(
    mut cmd: Command,
    timeout_ms: u64,
) -> Option<CapturedJsonV0Lines> {
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
