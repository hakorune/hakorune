//! One retained C document/query/frame/compile lifetime for the static host.
use super::*;
use crate::mir::function::{PublishedMirBackendView, PublishedStaticMethodCFrameV2};

#[cfg(feature = "plugins")]
pub(in crate::host_providers::llvm_codegen) fn compile_published_static_v2(
    view: &PublishedMirBackendView<'_>, body: &str, output: &Path, opts: &Opts,
) -> Result<(), String> {
    use crate::mir::function::{NamedAllocationConsumer as Consumer, PublishedStaticFrameHeaderV2};
    use std::os::raw::{c_char, c_int, c_void};
    type Open = unsafe extern "C" fn(*const c_char, usize, *mut *mut c_void, *mut *mut c_char) -> c_int;
    type Query = unsafe extern "C" fn(*mut c_void, *const c_char, usize, u32, u32, *mut u32) -> c_int;
    type Compile = unsafe extern "C" fn(*mut c_void, *const PublishedStaticFrameHeaderV2, *const c_char, *mut *mut c_char) -> c_int;
    type Close = unsafe extern "C" fn(*mut c_void);
    struct Invocation<'a> { handle: *mut c_void, close: Close, _library: &'a libloading::Library }
    impl Drop for Invocation<'_> {
        fn drop(&mut self) { unsafe { (self.close)(self.handle) } }
    }
    // Existing process settings remain scoped across planning too. This is not
    // a parallel-compile guarantee; C call rows are still process-global.
    struct Settings(Vec<(&'static str, Option<std::ffi::OsString>)>);
    impl Drop for Settings {
        fn drop(&mut self) {
            for (name, value) in self.0.iter().rev() {
                match value { Some(v) => std::env::set_var(name, v), None => std::env::remove_var(name) }
            }
        }
    }
    let mut settings = Settings(Vec::new());
    for (name, value, inherit) in [
        ("HAKO_BACKEND_COMPILE_RECIPE", opts.compile_recipe.as_deref(), false),
        ("HAKO_BACKEND_COMPAT_REPLAY", opts.compat_replay.as_deref(), false),
        ("HAKO_LLVM_OPT_LEVEL", opts.opt_level.as_deref(), true),
        ("NYASH_LLVM_OPT_LEVEL", opts.opt_level.as_deref(), true),
    ] {
        if inherit && value.is_none() { continue; }
        settings.0.push((name, std::env::var_os(name)));
        match value { Some(v) => std::env::set_var(name, v), None => std::env::remove_var(name) }
    }
    unsafe fn result(rc: c_int, error: *mut c_char) -> Result<(), String> {
        extern "C" { fn free(pointer: *mut c_void); }
        let message = if error.is_null() { None } else {
            let message = CStr::from_ptr(error).to_string_lossy().into_owned();
            free(error.cast());
            Some(message)
        };
        if rc == 0 { Ok(()) } else { Err(message.unwrap_or_else(|| "static V2 compilation failed".into())) }
    }
    unsafe {
        let library = load_ffi_library()?;
        let open: Open = *library.get(b"hako_llvmc_static_open_v2\0").map_err(|e| e.to_string())?;
        let query: Query = *library.get(b"hako_llvmc_static_query_v2\0").map_err(|e| e.to_string())?;
        let compile: Compile = *library.get(b"hako_llvmc_static_compile_v2\0").map_err(|e| e.to_string())?;
        let close: Close = *library.get(b"hako_llvmc_static_close_v2\0").map_err(|e| e.to_string())?;
        let mut handle = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        result(open(body.as_ptr().cast(), body.len(), &mut handle, &mut error), error)?;
        let invocation = Invocation { handle, close, _library: &library };
        let frame = PublishedStaticMethodCFrameV2::from_view_with_query(view, |name, block, ordinal| {
            let mut consumer = u32::MAX;
            match query(invocation.handle, name.as_ptr().cast(), name.len(), block, ordinal, &mut consumer) {
                0 => Ok(Some(match consumer {
                    0 => Consumer::Array, 1 => Consumer::DirectArray, 2 => Consumer::Map,
                    3 => Consumer::File, 4 => Consumer::AliasOperandZero, 5 => Consumer::TypedObject,
                    6 => Consumer::InvalidPlan, 7 => Consumer::Unsupported,
                    _ => return Err("[freeze:contract][static-v2/query-consumer]".into()),
                })),
                1..=4 => Ok(None), // Required missing observations reject in the existing planner.
                _ => Err("[freeze:contract][static-v2/query-status]".into()),
            }
        })?;
        let output_c = CString::new(output.to_string_lossy().as_bytes()).map_err(|_| "invalid output path")?;
        error = std::ptr::null_mut();
        result(compile(invocation.handle, &frame.header(), output_c.as_ptr(), &mut error), error)?;
        transport_io::ensure_backend_artifact_written(output, "object")
    }
}

#[cfg(not(feature = "plugins"))]
pub(in crate::host_providers::llvm_codegen) fn compile_published_static_v2(
    _view: &PublishedMirBackendView<'_>, _body: &str, _output: &Path, _opts: &Opts,
) -> Result<(), String> {
    Err("capi not available (plugins feature disabled)".into())
}
