//! Canonical published-MIR object ingress for the first backend cohort.
//!
//! The JSON file here is only the existing physical body transport.  The
//! selected published-call target/site relation is passed separately through
//! the versioned typed C rows; a typed failure never retries the JSON route.

use std::path::{Path, PathBuf};

#[cfg(test)]
use crate::mir::emit_lifecycle_physical_abi_json;
use crate::mir::function::{
    PublishedMirBackendView, PublishedStaticMethodCFrameV1, PublishedStaticMethodRouteV1,
};
use crate::mir::MirModule;

use super::{
    boundary_default_object_opts, capi_transport, lifecycle_invocation::LifecycleInvocationInputV1,
    runtime_abi_descriptor::LifecycleRuntimeSessionV1, transport_io,
};

/// Compile the published view when it contains a selected typed row.  A
/// compatibility-only module returns `Ok(false)` so its explicit caller can
/// remain in control; no semantic fallback is performed here.
pub(crate) fn try_compile_published_static_method_object(
    module: &MirModule,
    obj_out: &str,
) -> Result<bool, String> {
    let view = PublishedMirBackendView::try_new(module)
        .map_err(|error| format!("published MIR backend admission failed: {error}"))?;
    try_compile_published_view_object(&view, obj_out)
}

pub(crate) fn try_compile_published_view_object(
    view: &PublishedMirBackendView<'_>,
    obj_out: &str,
) -> Result<bool, String> {
    if view.has_lifecycle_instructions() {
        crate::mir::typed_array_backend_capability::enforce_typed_array_backend_supported(
            view.module(),
            "ny-llvmc-obj",
        )?;
    }
    match view.route() {
        PublishedStaticMethodRouteV1::CanonicalTyped => {
            crate::mir::backend_capability::enforce_published_backend_supported(
                view,
                "ny-llvmc-obj",
            )?;
            compile_published_view_object(view, obj_out, None)?;
            Ok(true)
        }
        PublishedStaticMethodRouteV1::ExplicitCompatibility => Ok(false),
        PublishedStaticMethodRouteV1::UnsupportedBeforeObject => Err(
            "[freeze:contract][published-mir-backend-object] UnsupportedBeforeObject: canonical call family has no selected-C consumer"
                .to_owned(),
        ),
    }
}

fn compile_published_view_object<'session>(
    view: &PublishedMirBackendView<'_>,
    obj_out: &str,
    lifecycle_session: Option<&'session LifecycleRuntimeSessionV1>,
) -> Result<Option<&'session Path>, String> {
    if view.has_lifecycle_instructions() {
        crate::mir::typed_array_backend_capability::enforce_typed_array_backend_supported(
            view.module(),
            "ny-llvmc-obj",
        )?;
        if lifecycle_session.is_none() {
            return Err(
                "published lifecycle object ingress requires an explicit runtime session"
                    .to_owned(),
            );
        }
        let input = view.issue_lifecycle_physical_abi_input()?;
        crate::mir::backend_capability::enforce_published_lifecycle_backend_supported(
            view, &input,
        )?;
        let input = LifecycleInvocationInputV1::bind(
            input,
            lifecycle_session.expect("checked lifecycle session"),
        )?;
        capi_transport::compile_published_lifecycle_physical_v4(&input, Path::new(obj_out))?;
        return Ok(Some(input.runtime_archive()));
    }
    crate::mir::backend_capability::enforce_published_backend_supported(view, "ny-llvmc-obj")?;
    let frame = PublishedStaticMethodCFrameV1::from_view(view)
        .map_err(|error| format!("published MIR C frame rejected: {error}"))?;
    let mir_json_path = transport_io::prepare_backend_input_json_file(
        &crate::runner::mir_json_emit::emit_published_view_body(view)?,
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
    result.map(|()| None)
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
    emit_published_view_exe(&view, exe_out, nyrt_dir, extra_libs)
}

pub(crate) fn emit_published_view_exe(
    view: &PublishedMirBackendView<'_>,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<bool, String> {
    if view.has_lifecycle_instructions() {
        crate::mir::typed_array_backend_capability::enforce_typed_array_backend_supported(
            view.module(),
            "ny-llvmc-exe",
        )?;
    }
    match view.route() {
        PublishedStaticMethodRouteV1::CanonicalTyped => {
            crate::mir::backend_capability::enforce_published_backend_supported(
                view,
                "ny-llvmc-exe",
            )?;
        }
        PublishedStaticMethodRouteV1::ExplicitCompatibility => return Ok(false),
        PublishedStaticMethodRouteV1::UnsupportedBeforeObject => {
            return Err(
                "[freeze:contract][published-mir-backend-object] UnsupportedBeforeObject: canonical call family has no selected-C consumer"
                    .to_owned(),
            )
        }
    }
    let object_path = format!("{}.published-static-method.o", exe_out);
    let result = (|| {
        let runtime_dir = nyrt_dir.ok_or("published EXE requires an explicit runtime directory")?;
        let lifecycle_session = if !view.has_lifecycle_instructions() {
            None
        } else {
            Some(LifecycleRuntimeSessionV1::select(
                PathBuf::from(runtime_dir).join("libnyash_lifecycle_kernel.a"),
            )?)
        };
        let legacy_archive = PathBuf::from(runtime_dir).join("libnyash_kernel.a");
        let archive =
            compile_published_view_object(view, &object_path, lifecycle_session.as_ref())?
                .unwrap_or(&legacy_archive);
        super::link_object_capi_v2(
            Path::new(&object_path),
            Path::new(exe_out),
            archive,
            extra_libs,
        )?;
        Ok(true)
    })();
    let _ = std::fs::remove_file(&object_path);
    result
}

#[cfg(all(test, feature = "plugins"))]
#[path = "published_mir_object_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "published_array_source_tests.rs"]
mod array_source_tests;
