//! Production caller for the canonical published-MIR backend ingress.
//!
//! A module with no selected StaticBoxMethod calls returns `Ok(false)` so the
//! caller may use its explicitly named compatibility path.  Once a selected
//! call is present, typed admission errors are returned and never retried.

pub(crate) fn try_emit_published_static_method_exe(
    module: &crate::mir::MirModule,
    exe_out: &str,
    nyrt_dir: Option<&str>,
    extra_libs: Option<&str>,
) -> Result<bool, String> {
    crate::host_providers::llvm_codegen::emit_published_static_method_exe(
        module, exe_out, nyrt_dir, extra_libs,
    )
}
