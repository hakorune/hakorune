// Compiled-stage1 compat quarantine for llvm backend residue.
// This stays shrink-only until callers stop at the thin backend boundary directly.

use std::fs;
use super::{decode_string_handle, encode_string_handle, trace_log};
use std::path::Path;

const LLVM_BACKEND_MODULE: &str = "selfhost.shared.backend.llvm_backend";
const COMPILE_OBJ_METHOD: &str = "compile_obj";
const LINK_EXE_METHOD: &str = "link_exe";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LlvmBackendRoute {
    CompileObj,
    LinkExe,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CompileObjRequest {
    mir_path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LinkExeRequest {
    obj_path: String,
    exe_path: String,
}

pub(super) fn try_dispatch(
    module_name: &str,
    method_name: &str,
    arg_count: i64,
    arg1: i64,
    arg2: i64,
) -> Option<i64> {
    let route = match_route(module_name, method_name)?;
    Some(dispatch_route(route, arg_count, arg1, arg2).unwrap_or(0))
}

fn match_route(module_name: &str, method_name: &str) -> Option<LlvmBackendRoute> {
    if module_name != LLVM_BACKEND_MODULE {
        return None;
    }
    match method_name {
        COMPILE_OBJ_METHOD => Some(LlvmBackendRoute::CompileObj),
        LINK_EXE_METHOD => Some(LlvmBackendRoute::LinkExe),
        _ => None,
    }
}

fn dispatch_route(route: LlvmBackendRoute, arg_count: i64, arg1: i64, arg2: i64) -> Option<i64> {
    match route {
        LlvmBackendRoute::CompileObj => handle_compile_obj(arg_count, arg1),
        LlvmBackendRoute::LinkExe => handle_link_exe(arg_count, arg1, arg2),
    }
}

fn handle_compile_obj(arg_count: i64, arg1: i64) -> Option<i64> {
    let request = decode_compile_obj_request(arg_count, arg1)?;
    finish_compile_obj_result(execute_compile_obj_request(&request))
}

fn handle_link_exe(arg_count: i64, arg1: i64, arg2: i64) -> Option<i64> {
    let request = decode_link_exe_request(arg_count, arg1, arg2)?;
    finish_link_exe_result(execute_link_exe_request(&request))
}

fn finish_compile_obj_result(result: Result<std::path::PathBuf, String>) -> Option<i64> {
    match result {
        Ok(obj_path) => Some(encode_string_handle(&obj_path.to_string_lossy())),
        Err(error_text) => Some(trace_route_error_and_zero("compile_obj", error_text)),
    }
}

fn finish_link_exe_result(result: Result<(), String>) -> Option<i64> {
    match result {
        Ok(()) => Some(1),
        Err(error_text) => Some(trace_route_error_and_zero("link_exe", error_text)),
    }
}

fn trace_route_error_and_zero(route_label: &str, error_text: String) -> i64 {
    trace_log(format!(
        "[stage1/module_dispatch] llvm_backend {} failed: {}",
        route_label, error_text
    ));
    0
}

fn compile_obj_from_json_path(mir_path: &str) -> Result<std::path::PathBuf, String> {
    // Compiled-stage1/archive-later residue: keep only the file-path contract here.
    // The compile core now reuses the same no-helper text primitive as watch-1.
    let mir_json = fs::read_to_string(Path::new(mir_path))
        .map_err(|e| format!("[llvmemit/input/read-failed] {}", e))?;
    nyash_rust::host_providers::llvm_codegen::compat_text_primitive::compile_object_from_mir_json_text_no_helper(
        &mir_json,
        compile_obj_opts_from_env(),
    )
}

fn compile_obj_opts_from_env() -> nyash_rust::host_providers::llvm_codegen::Opts {
    let mut opts = nyash_rust::host_providers::llvm_codegen::boundary_default_object_opts(
        None,
        std::env::var("NYASH_EMIT_EXE_NYRT")
            .ok()
            .map(std::path::PathBuf::from),
        std::env::var("HAKO_LLVM_OPT_LEVEL")
            .ok()
            .or_else(|| std::env::var("NYASH_LLVM_OPT_LEVEL").ok())
            .or(Some("0".to_string())),
        None,
    );
    opts.compile_recipe = Some("pure-first".to_string());
    opts.compat_replay = Some("harness".to_string());
    opts
}

fn decode_compile_obj_request(arg_count: i64, arg1: i64) -> Option<CompileObjRequest> {
    if arg_count < 1 {
        return None;
    }
    Some(CompileObjRequest {
        mir_path: decode_string_handle(arg1)?,
    })
}

fn decode_link_exe_request(arg_count: i64, arg1: i64, arg2: i64) -> Option<LinkExeRequest> {
    if arg_count < 2 {
        return None;
    }
    Some(LinkExeRequest {
        obj_path: decode_string_handle(arg1)?,
        exe_path: decode_string_handle(arg2)?,
    })
}

fn execute_compile_obj_request(request: &CompileObjRequest) -> Result<std::path::PathBuf, String> {
    compile_obj_from_json_path(&request.mir_path)
}

fn execute_link_exe_request(request: &LinkExeRequest) -> Result<(), String> {
    link_exe_from_object_paths(Path::new(&request.obj_path), Path::new(&request.exe_path))
}

fn link_exe_from_object_paths(obj_path: &Path, exe_path: &Path) -> Result<(), String> {
    nyash_rust::host_providers::llvm_codegen::link_object_capi(obj_path, exe_path, None)
}

#[cfg(test)]
mod tests {
    use super::{
        decode_compile_obj_request, decode_link_exe_request, match_route, try_dispatch,
        CompileObjRequest, LinkExeRequest, LlvmBackendRoute, COMPILE_OBJ_METHOD, LINK_EXE_METHOD,
        LLVM_BACKEND_MODULE,
    };
    use crate::plugin::module_string_dispatch::{decode_string_handle, encode_string_handle};

    #[test]
    fn llvm_backend_route_contract_is_stable() {
        assert_eq!(LLVM_BACKEND_MODULE, "selfhost.shared.backend.llvm_backend");
        assert_eq!(COMPILE_OBJ_METHOD, "compile_obj");
        assert_eq!(LINK_EXE_METHOD, "link_exe");
    }

    #[test]
    fn llvm_backend_compile_obj_missing_arg_returns_zero_handle() {
        let out = try_dispatch(LLVM_BACKEND_MODULE, COMPILE_OBJ_METHOD, 0, 0, 0).expect("route");
        assert_eq!(out, 0);
    }

    #[test]
    fn llvm_backend_route_match_requires_known_module_and_method() {
        assert_eq!(
            match_route(LLVM_BACKEND_MODULE, COMPILE_OBJ_METHOD),
            Some(LlvmBackendRoute::CompileObj)
        );
        assert_eq!(
            match_route(LLVM_BACKEND_MODULE, LINK_EXE_METHOD),
            Some(LlvmBackendRoute::LinkExe)
        );
        assert_eq!(match_route("other.module", COMPILE_OBJ_METHOD), None);
        assert_eq!(match_route(LLVM_BACKEND_MODULE, "other_method"), None);
    }

    #[test]
    fn llvm_backend_link_exe_missing_arg_returns_zero_flag() {
        let obj = encode_string_handle("/tmp/in.o");
        let out = try_dispatch(LLVM_BACKEND_MODULE, LINK_EXE_METHOD, 1, obj, 0).expect("route");
        assert_eq!(out, 0);
    }

    #[test]
    fn llvm_backend_link_exe_request_requires_two_args() {
        let obj = encode_string_handle("/tmp/in.o");
        let exe = encode_string_handle("/tmp/out.exe");
        assert!(decode_link_exe_request(1, obj, exe).is_none());
        assert_eq!(
            decode_link_exe_request(2, obj, exe),
            Some(LinkExeRequest {
                obj_path: "/tmp/in.o".to_string(),
                exe_path: "/tmp/out.exe".to_string(),
            })
        );
    }

    #[test]
    fn llvm_backend_compile_obj_request_prefers_first_string_handle() {
        let mir_path = encode_string_handle("/tmp/any.mir.json");
        assert_eq!(
            decode_compile_obj_request(1, mir_path),
            Some(CompileObjRequest {
                mir_path: "/tmp/any.mir.json".to_string(),
            })
        );
        assert_eq!(decode_compile_obj_request(1, 0), None);
        assert_eq!(
            try_dispatch(LLVM_BACKEND_MODULE, COMPILE_OBJ_METHOD, 1, 0, mir_path),
            Some(0)
        );
        let out =
            try_dispatch(LLVM_BACKEND_MODULE, COMPILE_OBJ_METHOD, 1, mir_path, 0).expect("route");
        if out > 0 {
            let text = decode_string_handle(out).expect("string handle");
            assert!(!text.is_empty());
        } else {
            assert_eq!(out, 0);
        }
    }

    #[test]
    fn llvm_backend_unknown_method_returns_none() {
        assert_eq!(try_dispatch(LLVM_BACKEND_MODULE, "unknown", 0, 0, 0), None);
    }
}
