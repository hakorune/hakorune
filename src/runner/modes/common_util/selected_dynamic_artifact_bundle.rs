//! Selected Dynamic Boundary bundle transport.
//!
//! This facade owns only the selected physical route: one attempt-unique
//! bundle path is emitted, root-validated, and returned as a path-bound fence.

use std::path::PathBuf;

use super::exec;
use super::static_artifact_receipt::VerifiedStaticArtifactBundleLaunchFenceV1;

pub(crate) fn selected_dynamic_bundle_path(exe_out: &str) -> PathBuf {
    PathBuf::from(format!(
        "{}.selected-dynamic-bundle-{}",
        exe_out,
        std::process::id()
    ))
}

pub(crate) fn emit_selected_dynamic(
    module: &crate::mir::MirModule,
    artifact_bundle: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<VerifiedStaticArtifactBundleLaunchFenceV1, String> {
    exec::validate_selected_dynamic_boundary_route_request()?;
    let nyrt_dir = nyrt_dir.ok_or_else(|| {
        "selected Dynamic Boundary requires an explicit --nyrt archive directory".to_owned()
    })?;
    crate::mir::backend_capability::enforce_mir_backend_supported(
        module,
        "ny-llvmc-selected-dynamic-exe",
    )?;
    exec::emit_json_and_run_ny_llvmc_emit_exe_with_bundle(
        |json_path| {
            crate::runner::mir_json_emit::emit_mir_json_for_selected_dynamic_candidate(
                module, json_path,
            )
            .map_err(|e| format!("MIR JSON emit error: {}", e))
        },
        artifact_bundle,
        Some(nyrt_dir),
        extra_libs,
    )
}

#[cfg(test)]
mod tests {
    use super::selected_dynamic_bundle_path;

    #[test]
    fn path_is_attempt_scoped_bundle_not_receipt_file() {
        let path = selected_dynamic_bundle_path("tmp/out");
        assert!(path
            .to_string_lossy()
            .contains("tmp/out.selected-dynamic-bundle-"));
        assert!(!path.to_string_lossy().ends_with(".json"));
    }
}
