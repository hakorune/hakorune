/*!
 * Stage0 capture route builders.
 *
 * Purpose:
 * - Keep `stage0_capture.rs` route-neutral.
 * - Own the backend-specific child command construction separately.
 */

use std::path::Path;
use std::process::Command;

/// Build a stage0 capture command for the requested backend.
#[allow(dead_code)]
pub(crate) fn build_stage0_capture_command(
    backend: &str,
    exe: &Path,
    program: &Path,
    extra_args: &[&str],
    env_remove: &[&str],
    envs: &[(&str, &str)],
) -> Command {
    let mut cmd = Command::new(exe);
    crate::runner::child_env::apply_selfhost_compiler_env(&mut cmd);
    cmd.arg("--backend").arg(backend).arg(program);
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

/// Build a stage0 capture command for non-VM routes.
///
/// This keeps the capture plumbing route-neutral while letting callers
/// supply direct/core or compat-only route bodies without reintroducing a
/// `--backend vm` default.
#[allow(dead_code)]
pub(crate) fn build_stage0_non_vm_capture_command(
    exe: &Path,
    program: &Path,
    extra_args: &[&str],
    env_remove: &[&str],
    envs: &[(&str, &str)],
) -> Command {
    let mut cmd = Command::new(exe);
    crate::runner::child_env::apply_selfhost_compiler_env(&mut cmd);
    cmd.arg(program);
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

/// Convenience builder for the current VM-backed stage0 capture route.
#[allow(dead_code)]
pub(crate) fn build_stage0_vm_capture_command(
    exe: &Path,
    program: &Path,
    extra_args: &[&str],
    env_remove: &[&str],
    envs: &[(&str, &str)],
) -> Command {
    build_stage0_capture_command("vm", exe, program, extra_args, env_remove, envs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_capture_builder_sets_backend_vm() {
        let cmd = build_stage0_vm_capture_command(
            Path::new("/tmp/nyash"),
            Path::new("foo.hako"),
            &["--", "--read-tmp"],
            &["NYASH_USE_NY_COMPILER"],
            &[("NYASH_SAMPLE", "1")],
        );
        let args: Vec<String> = cmd
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        assert_eq!(args[0], "--backend");
        assert_eq!(args[1], "vm");
        assert_eq!(args[2], "foo.hako");
        assert!(args.contains(&"--read-tmp".to_string()));
    }

    #[test]
    fn non_vm_capture_builder_leaves_backend_unset() {
        let cmd = build_stage0_non_vm_capture_command(
            Path::new("/tmp/nyash"),
            Path::new("foo.hako"),
            &["--emit-mir-json", "/tmp/out.json"],
            &["NYASH_USE_NY_COMPILER"],
            &[("NYASH_SAMPLE", "1")],
        );
        let args: Vec<String> = cmd
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        assert_eq!(args[0], "foo.hako");
        assert_eq!(args[1], "--emit-mir-json");
        assert_eq!(args[2], "/tmp/out.json");
        assert!(!args.iter().any(|arg| arg == "--backend"));
    }
}
