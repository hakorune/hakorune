//! Canonical published-MIR object ingress for the first backend cohort.
//!
//! The JSON file here is only the existing physical body transport.  The
//! selected published-call target/site relation is passed separately through
//! the versioned typed C rows; a typed failure never retries the JSON route.

use std::path::{Path, PathBuf};

use crate::mir::function::{
    PublishedMirBackendView, PublishedStaticMethodCFrameV1, PublishedStaticMethodRouteV1,
};
use crate::mir::MirModule;

use super::{boundary_default_object_opts, capi_transport, transport_io};

pub(crate) fn compile_published_static_method_object(
    module: &MirModule,
    obj_out: &str,
) -> Result<(), String> {
    let view = PublishedMirBackendView::try_new(module)
        .map_err(|error| format!("published MIR backend admission failed: {error}"))?;
    if view.route() != PublishedStaticMethodRouteV1::CanonicalTyped {
        return Err("published MIR module has no canonical typed call".to_owned());
    }
    let frame = PublishedStaticMethodCFrameV1::from_view(&view)
        .map_err(|error| format!("published MIR C frame rejected: {error}"))?;

    let mir_json_path = transport_io::prepare_backend_input_json_file(
        &crate::runner::mir_json_emit::emit_mir_json_string_for_harness_bin(module)?,
    )?;
    let output = PathBuf::from(obj_out);
    transport_io::ensure_backend_output_parent(&output);
    let opts = boundary_default_object_opts(Some(output.clone()), None, None, None);
    let result = capi_transport::compile_published_static_method_v1(
        &mir_json_path,
        &output,
        frame.as_slice(),
        &opts,
    );
    transport_io::remove_backend_temp_file(&mir_json_path);
    result
}

/// Emit and link one canonical published-call module. `false` means the
/// module has no selected-family call and must use its explicit compatibility
/// caller; `true` means the typed path owned the emission.
pub(crate) fn emit_published_static_method_exe(
    module: &MirModule,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<bool, String> {
    let view = PublishedMirBackendView::try_new(module)
        .map_err(|error| format!("published MIR backend admission failed: {error}"))?;
    if view.route() != PublishedStaticMethodRouteV1::CanonicalTyped {
        return Ok(false);
    }
    let object_path = format!("{}.published-static-method.o", exe_out);
    let result = (|| {
        compile_published_static_method_object(module, &object_path)?;
        let runtime_dir = nyrt_dir
            .map(PathBuf::from)
            .or_else(|| std::env::var("NYASH_EMIT_EXE_NYRT").ok().map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("target/release"));
        let runtime_archive = runtime_dir.join("libnyash_kernel.a");
        super::link_object_capi_v2(
            Path::new(&object_path),
            Path::new(exe_out),
            &runtime_archive,
            extra_libs,
        )?;
        Ok(true)
    })();
    let _ = std::fs::remove_file(&object_path);
    result
}
