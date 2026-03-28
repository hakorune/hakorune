use std::path::Path;

/// Resolve ny-llvmc executable path with env/PATH fallbacks
fn resolve_ny_llvmc() -> std::path::PathBuf {
    std::env::var("NYASH_NY_LLVM_COMPILER")
        .ok()
        .and_then(|s| {
            if !s.is_empty() {
                Some(std::path::PathBuf::from(s))
            } else {
                None
            }
        })
        .or_else(|| which::which("ny-llvmc").ok())
        .unwrap_or_else(|| std::path::PathBuf::from("target/release/ny-llvmc"))
}

fn hint_ny_llvmc_missing(path: &std::path::Path) -> String {
    format!(
        "ny-llvmc not found (tried: {}).\nHints:\n  - Build it: cargo build -p nyash-llvm-compiler --release\n  - Use the built binary: target/release/ny-llvmc\n  - Or set env NYASH_NY_LLVM_COMPILER=/full/path/to/ny-llvmc\n  - Or add it to PATH\n",
        path.display()
    )
}

fn hint_nyrt_missing(dir: &str) -> String {
    let lib = Path::new(dir).join("libnyash_kernel.a");
    format!(
        "nyrt runtime not found (missing: {}).\nHints:\n  - Build it: cargo build -p nyash_kernel --release\n  - Or set env NYASH_EMIT_EXE_NYRT=/path/to/nyash_kernel/target/release\n",
        lib.display()
    )
}

fn verify_nyrt_dir(dir: &str) -> Result<(), String> {
    let lib = Path::new(dir).join("libnyash_kernel.a");
    if lib.exists() {
        return Ok(());
    }
    Err(hint_nyrt_missing(dir))
}

#[inline(always)]
fn skip_nyrt_precheck() -> bool {
    // Keep default behavior unchanged. Harness/dev route can opt out of
    // runner-side precheck and let ny-llvmc decide its own runtime path.
    std::env::var("NYASH_LLVM_USE_HARNESS").ok().as_deref() == Some("1")
}

fn default_nyrt_dir() -> String {
    std::env::var("NYASH_EMIT_EXE_NYRT")
        .ok()
        .or_else(|| {
            std::env::var("NYASH_ROOT")
                .ok()
                .map(|r| format!("{}/target/release", r))
        })
        .unwrap_or_else(|| "target/release".to_string())
}

fn apply_nyrt_arg(cmd: &mut std::process::Command, nyrt_dir: Option<&str>) -> Result<(), String> {
    let default_nyrt = default_nyrt_dir();
    let nyrt_dir_final = nyrt_dir.unwrap_or(&default_nyrt);
    if !skip_nyrt_precheck() {
        verify_nyrt_dir(nyrt_dir_final)?;
        cmd.arg("--nyrt").arg(nyrt_dir_final);
    } else if let Some(explicit_nyrt) = nyrt_dir {
        cmd.arg("--nyrt").arg(explicit_nyrt);
    }
    Ok(())
}

fn ny_llvmc_driver_arg_from_backend(backend: Option<&str>) -> Result<Option<&'static str>, String> {
    match backend.map(str::trim).filter(|value| !value.is_empty()) {
        Some("native") => Err(
            "NYASH_LLVM_BACKEND=native is canary-only now; invoke ny-llvmc --driver native directly instead of routing it through hakorune".to_string(),
        ),
        _ => Ok(None),
    }
}

fn apply_ny_llvmc_driver_arg(cmd: &mut std::process::Command) -> Result<(), String> {
    if let Some(driver) =
        ny_llvmc_driver_arg_from_backend(std::env::var("NYASH_LLVM_BACKEND").ok().as_deref())?
    {
        cmd.arg("--driver").arg(driver);
    }
    Ok(())
}

fn append_ny_llvmc_extra_libs_arg(cmd: &mut std::process::Command, extra_libs: Option<&str>) {
    if let Some(flags) = extra_libs {
        if !flags.trim().is_empty() {
            cmd.arg("--libs").arg(flags);
        }
    }
}

fn prepare_ny_llvmc_emit_json_path() -> std::path::PathBuf {
    let tmp_dir = std::path::Path::new("tmp");
    let _ = std::fs::create_dir_all(tmp_dir);
    tmp_dir.join("nyash_cli_emit.json")
}

fn build_ny_llvmc_emit_exe_command(
    ny_llvmc: &std::path::Path,
    json_path: &std::path::Path,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<std::process::Command, String> {
    let mut cmd = std::process::Command::new(ny_llvmc);
    cmd.arg("--in")
        .arg(json_path)
        .arg("--emit")
        .arg("exe")
        .arg("--out")
        .arg(exe_out);
    apply_ny_llvmc_driver_arg(&mut cmd)?;
    apply_nyrt_arg(&mut cmd, nyrt_dir)?;
    append_ny_llvmc_extra_libs_arg(&mut cmd, extra_libs);
    Ok(cmd)
}

fn spawn_ny_llvmc_emit_exe_command(
    ny_llvmc: &std::path::Path,
    cmd: &mut std::process::Command,
) -> Result<(), String> {
    let status = cmd.status().map_err(|e| {
        format!(
            "failed to spawn ny-llvmc: {}\n{}",
            e,
            hint_ny_llvmc_missing(ny_llvmc)
        )
    })?;
    if !status.success() {
        return Err(format!(
            "ny-llvmc failed with status: {:?}.\nTry adding --emit-exe-libs (e.g. \"-ldl -lpthread -lm\") or set --emit-exe-nyrt to NyRT dir (e.g. target/release).",
            status.code()
        ));
    }
    Ok(())
}

fn run_ny_llvmc_emit_exe(
    json_path: &std::path::Path,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<(), String> {
    let ny_llvmc = resolve_ny_llvmc();
    if !ny_llvmc.exists() {
        return Err(hint_ny_llvmc_missing(&ny_llvmc));
    }
    let mut cmd =
        build_ny_llvmc_emit_exe_command(&ny_llvmc, json_path, exe_out, nyrt_dir, extra_libs)?;
    spawn_ny_llvmc_emit_exe_command(&ny_llvmc, &mut cmd)
}

fn emit_json_and_run_ny_llvmc_emit_exe(
    emit_json: impl FnOnce(&std::path::Path) -> Result<(), String>,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<(), String> {
    let json_path = prepare_ny_llvmc_emit_json_path();
    emit_json(&json_path)?;
    run_ny_llvmc_emit_exe(&json_path, exe_out, nyrt_dir, extra_libs)
}

/// Emit native executable via ny-llvmc (lib-side MIR)
#[allow(dead_code)]
pub fn ny_llvmc_emit_exe_lib(
    module: &nyash_rust::mir::MirModule,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<(), String> {
    emit_json_and_run_ny_llvmc_emit_exe(
        |json_path| {
            crate::runner::mir_json_emit::emit_mir_json_for_harness(module, json_path)
                .map_err(|e| format!("MIR JSON emit error: {}", e))
        },
        exe_out,
        nyrt_dir,
        extra_libs,
    )
}

/// Emit a native object via the shared harness helper (lib-side MIR)
#[allow(dead_code)]
pub fn ny_llvmc_emit_obj_lib(
    module: &nyash_rust::mir::MirModule,
    obj_out: &str,
) -> Result<(), String> {
    let mir_json = {
        let tmp_path = std::env::temp_dir().join(format!(
            "llvm_object_emitter-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or_default()
        ));
        crate::runner::mir_json_emit::emit_mir_json_for_harness(module, &tmp_path)
            .map_err(|e| format!("MIR JSON emit error: {}", e))?;
        let contents = std::fs::read_to_string(&tmp_path)
            .map_err(|e| format!("read harness MIR JSON: {}", e))?;
        let _ = std::fs::remove_file(&tmp_path);
        contents
    };
    let mut opts = crate::host_providers::llvm_codegen::boundary_default_object_opts(
        Some(std::path::PathBuf::from(obj_out)),
        None,
        crate::config::env::llvm_opt_level_env(),
        Some(20_000),
    );
    opts.compile_recipe = Some("pure-first".to_string());
    opts.compat_replay = Some("harness".to_string());
    let out_path = crate::host_providers::llvm_codegen::mir_json_to_object(&mir_json, opts)?;
    if std::fs::metadata(&out_path)
        .map_err(|e| format!("harness object not found after emit: {} ({})", out_path.display(), e))?
        .len()
        == 0
    {
        return Err(format!("harness object is empty: {}", out_path.display()));
    }
    Ok(())
}

/// Emit native executable via ny-llvmc (bin-side MIR)
#[allow(dead_code)]
pub fn ny_llvmc_emit_exe_bin(
    module: &crate::mir::MirModule,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<(), String> {
    emit_json_and_run_ny_llvmc_emit_exe(
        |json_path| {
            crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(module, json_path)
                .map_err(|e| format!("MIR JSON emit error: {}", e))
        },
        exe_out,
        nyrt_dir,
        extra_libs,
    )
}

/// Run an executable with arguments and a timeout.
/// Returns (exit_code, timed_out, stdout_text).
#[allow(dead_code)]
pub fn run_executable(
    exe_path: &str,
    args: &[&str],
    timeout_ms: u64,
) -> Result<(i32, bool, String), String> {
    let mut cmd = std::process::Command::new(exe_path);
    for a in args {
        cmd.arg(a);
    }
    let out =
        super::io::spawn_with_timeout(cmd, timeout_ms).map_err(|e| format!("spawn exe: {}", e))?;
    let code = out.exit_code.unwrap_or(1);
    let stdout_text = String::from_utf8_lossy(&out.stdout).into_owned();
    Ok((code, out.timed_out, stdout_text))
}

#[cfg(test)]
mod tests {
    use super::{append_ny_llvmc_extra_libs_arg, ny_llvmc_driver_arg_from_backend};

    #[test]
    fn rejects_native_backend_selector_for_runner_route() {
        let err = ny_llvmc_driver_arg_from_backend(Some("native")).unwrap_err();
        assert!(err.contains("canary-only"));
        let err = ny_llvmc_driver_arg_from_backend(Some(" native ")).unwrap_err();
        assert!(err.contains("ny-llvmc --driver native"));
    }

    #[test]
    fn ignores_empty_or_non_native_backend_values() {
        assert_eq!(ny_llvmc_driver_arg_from_backend(None).unwrap(), None);
        assert_eq!(ny_llvmc_driver_arg_from_backend(Some("")).unwrap(), None);
        assert_eq!(
            ny_llvmc_driver_arg_from_backend(Some("crate")).unwrap(),
            None
        );
        assert_eq!(
            ny_llvmc_driver_arg_from_backend(Some("llvmlite")).unwrap(),
            None
        );
    }

    #[test]
    fn appends_non_empty_extra_libs_as_single_arg() {
        let mut cmd = std::process::Command::new("ny-llvmc");
        append_ny_llvmc_extra_libs_arg(&mut cmd, Some("-ldl -lpthread"));
        let args: Vec<_> = cmd
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            args,
            vec!["--libs".to_string(), "-ldl -lpthread".to_string()]
        );
    }

    #[test]
    fn ignores_blank_extra_libs() {
        let mut cmd = std::process::Command::new("ny-llvmc");
        append_ny_llvmc_extra_libs_arg(&mut cmd, Some("   "));
        assert!(cmd.get_args().next().is_none());
    }
}
