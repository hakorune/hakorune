use std::io::Read;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

pub struct ChildOutput {
    pub stdout: Vec<u8>,
    #[allow(dead_code)]
    pub stderr: Vec<u8>,
    #[allow(dead_code)]
    pub status_ok: bool,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
}

/// Spawn command with timeout (ms), capture stdout/stderr, and return ChildOutput.
pub fn spawn_with_timeout(mut cmd: Command, timeout_ms: u64) -> std::io::Result<ChildOutput> {
    let cmd = cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn()?;
    let ch_stdout = child.stdout.take();
    let ch_stderr = child.stderr.take();
    // Phase 90-C: time 系移行
    let ring0 = crate::runtime::ring0::get_global_ring0();
    let start = ring0
        .time
        .monotonic_now()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut timed_out = false;
    let mut exit_status: Option<std::process::ExitStatus> = None;
    loop {
        match child.try_wait()? {
            Some(status) => {
                exit_status = Some(status);
                break;
            }
            None => {
                if ring0.time.elapsed(start) >= Duration::from_millis(timeout_ms) {
                    let _ = child.kill();
                    let _ = child.wait();
                    timed_out = true;
                    break;
                }
                sleep(Duration::from_millis(10));
            }
        }
    }
    let mut out_buf = Vec::new();
    let mut err_buf = Vec::new();
    if let Some(mut s) = ch_stdout {
        let _ = s.read_to_end(&mut out_buf);
    }
    if let Some(mut s) = ch_stderr {
        let _ = s.read_to_end(&mut err_buf);
    }
    let (status_ok, exit_code) = if let Some(st) = exit_status {
        (st.success(), st.code())
    } else {
        (false, None)
    };
    Ok(ChildOutput {
        stdout: out_buf,
        stderr: err_buf,
        status_ok,
        exit_code,
        timed_out,
    })
}
