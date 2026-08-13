use std::path::{Path, PathBuf};

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
        .or_else(|| crate::config::env::hako_root().map(|r| format!("{}/target/release", r)))
        .unwrap_or_else(|| "target/release".to_string())
}

/// Resolve and verify the runtime archive directory for the selected Boundary
/// lane.  Unlike ordinary compatibility execution, this path must always
/// pass an explicit `--nyrt` value to `ny-llvmc`; the harness environment must
/// not suppress the link input.
#[cfg(feature = "llvm-harness")]
pub(crate) fn selected_dynamic_nyrt_dir() -> Result<String, String> {
    let dir = default_nyrt_dir();
    verify_nyrt_dir(&dir)?;
    Ok(dir)
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

fn resolve_python3() -> Option<PathBuf> {
    if let Ok(p) = which::which("python3") {
        return Some(p);
    }
    if let Ok(p) = which::which("python") {
        return Some(p);
    }
    None
}

fn resolve_llvmlite_harness() -> Option<PathBuf> {
    if let Some(root) = crate::config::env::hako_root() {
        let p = PathBuf::from(root).join("tools/llvmlite_harness.py");
        if p.exists() {
            return Some(p);
        }
    }
    let p = PathBuf::from("tools/llvmlite_harness.py");
    if p.exists() {
        return Some(p);
    }
    let p2 = PathBuf::from("../tools/llvmlite_harness.py");
    if p2.exists() {
        return Some(p2);
    }
    None
}

fn prepare_llvmlite_emit_json_path() -> PathBuf {
    let tmp_dir = Path::new("tmp");
    let _ = std::fs::create_dir_all(tmp_dir);
    tmp_dir.join("nyash_cli_emit_harness.json")
}

fn spawn_llvmlite_emit_obj_command(
    python: &Path,
    harness: &Path,
    json_path: &Path,
    obj_out: &str,
) -> Result<(), String> {
    let status = std::process::Command::new(python)
        .arg(harness)
        .arg("--in")
        .arg(json_path)
        .arg("--out")
        .arg(obj_out)
        .status()
        .map_err(|e| format!("[llvmemit/llvmlite/spawn/error] {}", e))?;
    if !status.success() {
        return Err(format!(
            "[llvmemit/llvmlite/failed status={}]",
            status.code().unwrap_or(1)
        ));
    }
    Ok(())
}

fn prepare_ny_llvmc_emit_json_path() -> std::path::PathBuf {
    let tmp_dir = std::path::Path::new("tmp");
    let _ = std::fs::create_dir_all(tmp_dir);
    tmp_dir.join(format!("nyash_cli_emit_{}.json", std::process::id()))
}

/// Return the deterministic receipt path for one selected Dynamic executable
/// attempt.  The path is transport-only: the receipt is issued by ny-llvmc
/// and consumed by `emit_json_and_run_ny_llvmc_emit_exe` before this function
/// returns.  It is not a route or semantic authority.
pub(crate) fn selected_dynamic_receipt_path(exe_out: &str) -> PathBuf {
    PathBuf::from(format!(
        "{}.selected-dynamic-receipt-{}.json",
        exe_out,
        std::process::id()
    ))
}

/// Census the already-sealed selected Dynamic metadata pair on a candidate
/// module.  This is a physical route query, not a second semantic issuer:
/// the package adapter/session already issued both slots.  A partial pair or
/// more than one pair is rejected before any backend process is spawned.
pub(crate) fn selected_dynamic_aot_metadata_present(
    module: &crate::mir::MirModule,
) -> Result<bool, String> {
    use crate::mir::function::DynamicV2MetadataPairObservation;

    let mut selected = 0usize;
    for (name, function) in &module.functions {
        match function.metadata.selected_dynamic_metadata_observation() {
            DynamicV2MetadataPairObservation::Ordinary => {}
            DynamicV2MetadataPairObservation::Selected { .. } => {
                selected += 1;
            }
            DynamicV2MetadataPairObservation::Scrubbed => {
                return Err(format!(
                    "selected Dynamic metadata pair is scrubbed for function {name}"
                ));
            }
            DynamicV2MetadataPairObservation::Partial => {
                return Err(format!(
                    "selected Dynamic metadata pair is partial for function {name}"
                ));
            }
        }
    }
    match selected {
        0 => Ok(false),
        1 => Ok(true),
        count => Err(format!(
            "selected Dynamic metadata pair count={count} expected=1"
        )),
    }
}

fn validate_selected_dynamic_boundary_route_values(
    compile_recipe: Option<&str>,
    compat_replay: Option<&str>,
    emit_provider: Option<&str>,
    legacy_capi_pure: Option<&str>,
) -> Result<(), String> {
    if let Some(recipe) = compile_recipe {
        if recipe != "pure-first" {
            return Err(format!(
                "selected Dynamic Boundary rejects HAKO_BACKEND_COMPILE_RECIPE={recipe:?}; expected pure-first or unset"
            ));
        }
    }
    if let Some(replay) = compat_replay {
        if replay != "none" {
            return Err(format!(
                "selected Dynamic Boundary rejects HAKO_BACKEND_COMPAT_REPLAY={replay:?}; expected none or unset"
            ));
        }
    }
    if let Some(provider) = emit_provider {
        return Err(format!(
            "selected Dynamic Boundary rejects explicit HAKO_LLVM_EMIT_PROVIDER={provider:?}"
        ));
    }
    if matches!(legacy_capi_pure, Some("1" | "on" | "true" | "yes")) {
        return Err(
            "selected Dynamic Boundary rejects deprecated HAKO_CAPI_PURE; use the fixed pure-first route"
                .to_owned(),
        );
    }
    Ok(())
}

fn validate_selected_dynamic_boundary_route_request() -> Result<(), String> {
    validate_selected_dynamic_boundary_route_values(
        std::env::var("HAKO_BACKEND_COMPILE_RECIPE").ok().as_deref(),
        std::env::var("HAKO_BACKEND_COMPAT_REPLAY").ok().as_deref(),
        std::env::var("HAKO_LLVM_EMIT_PROVIDER").ok().as_deref(),
        std::env::var("HAKO_CAPI_PURE").ok().as_deref(),
    )
}

fn build_ny_llvmc_emit_obj_command(
    ny_llvmc: &std::path::Path,
    json_path: &std::path::Path,
    obj_out: &str,
) -> Result<std::process::Command, String> {
    let mut cmd = std::process::Command::new(ny_llvmc);
    cmd.arg("--in")
        .arg(json_path)
        .arg("--emit")
        .arg("obj")
        .arg("--out")
        .arg(obj_out);
    apply_ny_llvmc_driver_arg(&mut cmd)?;
    Ok(cmd)
}

fn build_ny_llvmc_emit_exe_command(
    ny_llvmc: &std::path::Path,
    json_path: &std::path::Path,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
    receipt_json: Option<&std::path::Path>,
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
    if let Some(receipt_json) = receipt_json {
        cmd.arg("--receipt-json").arg(receipt_json);
    }
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

fn spawn_ny_llvmc_emit_obj_command(
    ny_llvmc: &std::path::Path,
    cmd: &mut std::process::Command,
    obj_out: &str,
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
            "ny-llvmc object emit failed with status: {:?} (out={})",
            status.code(),
            obj_out
        ));
    }
    let metadata = std::fs::metadata(obj_out)
        .map_err(|e| format!("ny-llvmc object not found after emit: {} ({})", obj_out, e))?;
    if metadata.len() == 0 {
        return Err(format!("ny-llvmc object is empty: {}", obj_out));
    }
    Ok(())
}

fn run_ny_llvmc_emit_exe(
    json_path: &std::path::Path,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
    receipt_json: Option<&std::path::Path>,
) -> Result<(), String> {
    let ny_llvmc = resolve_ny_llvmc();
    if !ny_llvmc.exists() {
        return Err(hint_ny_llvmc_missing(&ny_llvmc));
    }
    let mut cmd = build_ny_llvmc_emit_exe_command(
        &ny_llvmc,
        json_path,
        exe_out,
        nyrt_dir,
        extra_libs,
        receipt_json,
    )?;
    spawn_ny_llvmc_emit_exe_command(&ny_llvmc, &mut cmd)
}

fn run_ny_llvmc_emit_obj(json_path: &std::path::Path, obj_out: &str) -> Result<(), String> {
    let ny_llvmc = resolve_ny_llvmc();
    if !ny_llvmc.exists() {
        return Err(hint_ny_llvmc_missing(&ny_llvmc));
    }
    if let Some(parent) = Path::new(obj_out).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("[llvmemit/ny-llvmc/out-parent-failed] {}", e))?;
    }
    let mut cmd = build_ny_llvmc_emit_obj_command(&ny_llvmc, json_path, obj_out)?;
    spawn_ny_llvmc_emit_obj_command(&ny_llvmc, &mut cmd, obj_out)
}

fn with_retained_mir_path(err: String, json_path: &std::path::Path) -> String {
    format!("{}\nretained_mir={}", err, json_path.display())
}

fn emit_json_and_run_ny_llvmc_emit_exe(
    emit_json: impl FnOnce(&std::path::Path) -> Result<(), String>,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
    receipt_json: Option<&std::path::Path>,
) -> Result<(), String> {
    emit_json_and_run_ny_llvmc_emit_exe_with_receipt(
        emit_json,
        exe_out,
        nyrt_dir,
        extra_libs,
        receipt_json,
    )
    .map(|_| ())
}

/// Execute one Boundary artifact attempt and return its consumed receipt fence
/// when the caller supplied a receipt path.  The selected Dynamic runner keeps
/// that fence alive through process execution; ordinary callers intentionally
/// discard the optional transport fence at this compatibility boundary.
fn emit_json_and_run_ny_llvmc_emit_exe_with_receipt(
    emit_json: impl FnOnce(&std::path::Path) -> Result<(), String>,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
    receipt_json: Option<&std::path::Path>,
) -> Result<Option<crate::mir::StaticArtifactReceiptConsumedFenceV1>, String> {
    let json_path = prepare_ny_llvmc_emit_json_path();
    emit_json(&json_path)?;
    let result = run_ny_llvmc_emit_exe(&json_path, exe_out, nyrt_dir, extra_libs, receipt_json);
    match result {
        Ok(()) => {
            let consumed_receipt = if let Some(receipt_json) = receipt_json {
                Some(crate::runner::modes::common_util::static_artifact_receipt::consume_static_artifact_receipt(
                    receipt_json,
                    &json_path,
                    Some(Path::new(exe_out)),
                ).map_err(|err| with_retained_mir_path(err, &json_path))?)
            } else {
                None
            };
            let _ = std::fs::remove_file(&json_path);
            Ok(consumed_receipt)
        }
        Err(err) => Err(with_retained_mir_path(err, &json_path)),
    }
}

/// Emit native executable via ny-llvmc (lib-side MIR)
pub fn ny_llvmc_emit_exe_lib(
    module: &nyash_rust::mir::MirModule,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<(), String> {
    let mut backend_ready = module.clone();
    crate::mir::semantic_refresh::refresh_module_semantic_metadata(&mut backend_ready);
    crate::mir::backend_capability::enforce_mir_backend_supported(&backend_ready, "ny-llvmc-exe")?;
    let backend_ready =
        crate::mir::array_element_write::project_module_to_legacy_calls(&backend_ready)?;
    emit_json_and_run_ny_llvmc_emit_exe(
        |json_path| {
            crate::runner::mir_json_emit::emit_mir_json_for_harness(&backend_ready, json_path)
                .map_err(|e| format!("MIR JSON emit error: {}", e))
        },
        exe_out,
        nyrt_dir,
        extra_libs,
        None,
    )
}

/// Emit a native object via the llvmlite keep lane (lib-side MIR).
pub fn llvmlite_emit_obj_lib(
    module: &nyash_rust::mir::MirModule,
    obj_out: &str,
) -> Result<(), String> {
    crate::mir::backend_capability::enforce_mir_backend_supported(module, "llvmlite-obj")?;
    let backend_ready = crate::mir::array_element_write::project_module_to_legacy_calls(module)?;
    let json_path = prepare_llvmlite_emit_json_path();
    crate::runner::mir_json_emit::emit_mir_json_for_harness(&backend_ready, &json_path)
        .map_err(|e| format!("MIR JSON emit error: {}", e))?;

    let result = (|| {
        let python = resolve_python3()
            .ok_or_else(|| "[llvmemit/llvmlite/python-not-found] python3 not found".to_string())?;
        let harness = resolve_llvmlite_harness().ok_or_else(|| {
            "[llvmemit/llvmlite/harness-not-found] tools/llvmlite_harness.py".to_string()
        })?;
        if let Some(parent) = Path::new(obj_out).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("[llvmemit/llvmlite/out-parent-failed] {}", e))?;
        }
        spawn_llvmlite_emit_obj_command(&python, &harness, &json_path, obj_out)?;
        let metadata = std::fs::metadata(obj_out)
            .map_err(|e| format!("harness object not found after emit: {} ({})", obj_out, e))?;
        if metadata.len() == 0 {
            return Err(format!("harness object is empty: {}", obj_out));
        }
        Ok(())
    })();
    let _ = std::fs::remove_file(&json_path);
    result
}

/// Deprecated compatibility alias for older internal call sites.
pub fn ny_llvmc_emit_obj_lib(
    module: &nyash_rust::mir::MirModule,
    obj_out: &str,
) -> Result<(), String> {
    crate::mir::backend_capability::enforce_mir_backend_supported(module, "ny-llvmc-obj")?;
    let backend_ready = crate::mir::array_element_write::project_module_to_legacy_calls(module)?;
    let json_path = prepare_ny_llvmc_emit_json_path();
    crate::runner::mir_json_emit::emit_mir_json_for_harness(&backend_ready, &json_path)
        .map_err(|e| format!("MIR JSON emit error: {}", e))?;
    let result = run_ny_llvmc_emit_obj(&json_path, obj_out);
    let _ = std::fs::remove_file(&json_path);
    result
}

/// Emit native executable via ny-llvmc (bin-side MIR)
pub fn ny_llvmc_emit_exe_bin(
    module: &crate::mir::MirModule,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<(), String> {
    let mut backend_ready = module.clone();
    crate::mir::semantic_refresh::refresh_module_semantic_metadata(&mut backend_ready);
    crate::mir::backend_capability::enforce_mir_backend_supported(&backend_ready, "ny-llvmc-exe")?;
    let backend_ready =
        crate::mir::array_element_write::project_module_to_legacy_calls(&backend_ready)?;
    emit_json_and_run_ny_llvmc_emit_exe(
        |json_path| {
            crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(&backend_ready, json_path)
                .map_err(|e| format!("MIR JSON emit error: {}", e))
        },
        exe_out,
        nyrt_dir,
        extra_libs,
        None,
    )
}

/// Emit the selected Dynamic module through the dedicated Boundary receipt
/// channel. The selected runner consumes this fence before launching the
/// published temporary artifact; ordinary compatibility uses its own route.
pub fn ny_llvmc_emit_exe_selected_dynamic_bin(
    module: &crate::mir::MirModule,
    exe_out: &str,
    receipt_json: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<crate::mir::StaticArtifactReceiptConsumedFenceV1, String> {
    validate_selected_dynamic_boundary_route_request()?;
    let nyrt_dir = nyrt_dir.ok_or_else(|| {
        "selected Dynamic Boundary requires an explicit --nyrt archive directory".to_owned()
    })?;
    crate::mir::backend_capability::enforce_mir_backend_supported(
        module,
        "ny-llvmc-selected-dynamic-exe",
    )?;
    let receipt_path = Path::new(receipt_json);
    emit_json_and_run_ny_llvmc_emit_exe_with_receipt(
        |json_path| {
            crate::runner::mir_json_emit::emit_mir_json_for_selected_dynamic_candidate(
                module, json_path,
            )
            .map_err(|e| format!("MIR JSON emit error: {}", e))
        },
        exe_out,
        Some(nyrt_dir),
        extra_libs,
        Some(receipt_path),
    )?
    .ok_or_else(|| "selected Dynamic Boundary receipt fence missing".to_owned())
}

/// Run an executable with arguments and a timeout.
/// Returns (exit_code, timed_out, stdout_text).
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
    use super::{
        append_ny_llvmc_extra_libs_arg, build_ny_llvmc_emit_exe_command,
        ny_llvmc_driver_arg_from_backend, selected_dynamic_aot_metadata_present,
        selected_dynamic_receipt_path, validate_selected_dynamic_boundary_route_values,
        with_retained_mir_path,
    };

    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType,
    };

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
    fn selected_dynamic_census_keeps_ordinary_module_on_generic_route() {
        let module = MirModule::new("ordinary".to_owned());
        assert!(!selected_dynamic_aot_metadata_present(&module).unwrap());
    }

    #[test]
    fn selected_dynamic_census_rejects_scrubbed_clone() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "ParserScanLoopBox.skip_while/4".to_owned(),
                params: vec![MirType::Unknown; 4],
                return_type: MirType::Integer,
                effects: EffectMask::READ,
            },
            BasicBlockId::new(0),
        );
        function
            .metadata
            .install_a_prime_i64_physical_receipt_for_test(
                crate::mir::test_support::a_prime_receipt(),
            )
            .expect("receipt install");
        function
            .metadata
            .install_dynamic_v2_aot_metadata_for_test(
                crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1::for_test(),
            )
            .expect("admission install");

        let scrubbed_function = function.clone();
        let mut module = MirModule::new("selected".to_owned());
        module.add_function(function);
        assert!(selected_dynamic_aot_metadata_present(&module).unwrap());

        let mut cloned_module = MirModule::new("scrubbed-clone".to_owned());
        cloned_module.add_function(scrubbed_function);
        let error = selected_dynamic_aot_metadata_present(&cloned_module).unwrap_err();
        assert!(error.contains("scrubbed"));
    }

    #[test]
    fn selected_dynamic_census_rejects_partial_pair() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "partial/4".to_owned(),
                params: vec![MirType::Unknown; 4],
                return_type: MirType::Integer,
                effects: EffectMask::READ,
            },
            BasicBlockId::new(0),
        );
        function
            .metadata
            .install_a_prime_i64_physical_receipt_for_test(
                crate::mir::test_support::a_prime_receipt(),
            )
            .expect("receipt install");
        let mut module = MirModule::new("partial".to_owned());
        module.add_function(function);
        let error = selected_dynamic_aot_metadata_present(&module).unwrap_err();
        assert!(error.contains("partial"));
    }

    #[test]
    fn selected_dynamic_boundary_accepts_only_fixed_route_values() {
        assert!(validate_selected_dynamic_boundary_route_values(None, None, None, None).is_ok());
        assert!(validate_selected_dynamic_boundary_route_values(
            Some("pure-first"),
            Some("none"),
            None,
            Some("0"),
        )
        .is_ok());
    }

    #[test]
    fn selected_dynamic_boundary_rejects_compat_route_inheritance() {
        for (recipe, replay, provider, legacy) in [
            (Some("harness"), None, None, None),
            (None, Some("harness"), None, None),
            (None, None, Some("llvmlite"), None),
            (None, None, None, Some("1")),
        ] {
            assert!(validate_selected_dynamic_boundary_route_values(
                recipe, replay, provider, legacy
            )
            .is_err());
        }
    }

    #[test]
    fn selected_dynamic_receipt_path_is_process_scoped() {
        let path = selected_dynamic_receipt_path("tmp/out");
        assert!(path
            .to_string_lossy()
            .contains("tmp/out.selected-dynamic-receipt-"));
        assert!(path.to_string_lossy().ends_with(".json"));
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

    #[test]
    fn retained_mir_path_is_reported_on_emit_failure() {
        let err = with_retained_mir_path(
            "ny-llvmc failed".to_string(),
            std::path::Path::new("tmp/nyash_cli_emit_123.json"),
        );
        assert!(err.contains("ny-llvmc failed"));
        assert!(err.contains("retained_mir=tmp/nyash_cli_emit_123.json"));
    }

    #[test]
    fn selected_receipt_flag_is_forwarded_to_boundary_command() {
        let cmd = build_ny_llvmc_emit_exe_command(
            std::path::Path::new("ny-llvmc"),
            std::path::Path::new("candidate.json"),
            "candidate.exe",
            Some("target/release"),
            None,
            Some(std::path::Path::new("receipt.json")),
        )
        .expect("command");
        let args: Vec<_> = cmd
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();
        assert!(args
            .windows(2)
            .any(|pair| pair == ["--receipt-json", "receipt.json"]));
    }
}
