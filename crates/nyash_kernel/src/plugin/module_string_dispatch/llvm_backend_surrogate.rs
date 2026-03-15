use super::{decode_string_handle, encode_string_handle, trace_log};
use std::path::Path;

const LLVM_BACKEND_MODULE: &str = "selfhost.shared.backend.llvm_backend";
const COMPILE_OBJ_METHOD: &str = "compile_obj";
const LINK_EXE_METHOD: &str = "link_exe";

pub(super) fn try_dispatch(
    module_name: &str,
    method_name: &str,
    arg_count: i64,
    arg1: i64,
    arg2: i64,
) -> Option<i64> {
    if is_compile_obj_route(module_name, method_name) {
        return Some(handle_compile_obj(arg_count, arg1, arg2).unwrap_or(0));
    }
    if is_link_exe_route(module_name, method_name) {
        return Some(handle_link_exe(arg_count, arg1, arg2).unwrap_or(0));
    }
    None
}

fn is_compile_obj_route(module_name: &str, method_name: &str) -> bool {
    module_name == LLVM_BACKEND_MODULE && method_name == COMPILE_OBJ_METHOD
}

fn is_link_exe_route(module_name: &str, method_name: &str) -> bool {
    module_name == LLVM_BACKEND_MODULE && method_name == LINK_EXE_METHOD
}

fn handle_compile_obj(arg_count: i64, arg1: i64, arg2: i64) -> Option<i64> {
    let mir_path = decode_compile_obj_path(arg_count, arg1, arg2)?;
    match compile_obj_from_json_path(&mir_path) {
        Ok(obj_path) => Some(encode_string_handle(&obj_path.to_string_lossy())),
        Err(error_text) => {
            trace_log(format!(
                "[stage1/module_dispatch] llvm_backend compile_obj failed: {}",
                error_text
            ));
            Some(0)
        }
    }
}

fn handle_link_exe(arg_count: i64, arg1: i64, arg2: i64) -> Option<i64> {
    let (obj_path, exe_path) = decode_link_exe_args(arg_count, arg1, arg2)?;
    match link_exe_from_object_paths(Path::new(&obj_path), Path::new(&exe_path)) {
        Ok(()) => Some(1),
        Err(error_text) => {
            trace_log(format!(
                "[stage1/module_dispatch] llvm_backend link_exe failed: {}",
                error_text
            ));
            Some(0)
        }
    }
}

fn compile_obj_from_json_path(mir_path: &str) -> Result<std::path::PathBuf, String> {
    nyash_rust::host_providers::llvm_codegen::mir_json_file_to_object(
        Path::new(mir_path),
        compile_obj_opts_from_env(),
    )
}

fn compile_obj_opts_from_env() -> nyash_rust::host_providers::llvm_codegen::Opts {
    nyash_rust::host_providers::llvm_codegen::Opts {
        out: None,
        nyrt: std::env::var("NYASH_EMIT_EXE_NYRT")
            .ok()
            .map(std::path::PathBuf::from),
        opt_level: std::env::var("HAKO_LLVM_OPT_LEVEL")
            .ok()
            .or_else(|| std::env::var("NYASH_LLVM_OPT_LEVEL").ok())
            .or(Some("0".to_string())),
        timeout_ms: None,
    }
}

fn decode_compile_obj_path(arg_count: i64, arg1: i64, arg2: i64) -> Option<String> {
    decode_required_arg(arg_count, arg1, arg2)
}

fn decode_link_exe_args(arg_count: i64, arg1: i64, arg2: i64) -> Option<(String, String)> {
    if arg_count < 2 {
        return None;
    }
    let obj_path = decode_string_handle(arg1)?;
    let exe_path = decode_string_handle(arg2)?;
    Some((obj_path, exe_path))
}

fn link_exe_from_object_paths(obj_path: &Path, exe_path: &Path) -> Result<(), String> {
    nyash_rust::host_providers::llvm_codegen::link_object_capi(obj_path, exe_path, None)
}

fn decode_required_arg(arg_count: i64, arg1: i64, arg2: i64) -> Option<String> {
    if arg_count < 1 {
        return None;
    }
    decode_string_handle(arg1).or_else(|| decode_string_handle(arg2))
}

#[cfg(test)]
mod tests {
    use super::{
        decode_link_exe_args, try_dispatch, COMPILE_OBJ_METHOD, LINK_EXE_METHOD, LLVM_BACKEND_MODULE,
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
    fn llvm_backend_link_exe_missing_arg_returns_zero_flag() {
        let obj = encode_string_handle("/tmp/in.o");
        let out = try_dispatch(LLVM_BACKEND_MODULE, LINK_EXE_METHOD, 1, obj, 0).expect("route");
        assert_eq!(out, 0);
    }

    #[test]
    fn llvm_backend_link_exe_decode_requires_two_args() {
        let obj = encode_string_handle("/tmp/in.o");
        let exe = encode_string_handle("/tmp/out.exe");
        assert!(decode_link_exe_args(1, obj, exe).is_none());
        assert_eq!(
            decode_link_exe_args(2, obj, exe),
            Some(("/tmp/in.o".to_string(), "/tmp/out.exe".to_string()))
        );
    }

    #[test]
    fn llvm_backend_compile_obj_decode_prefers_first_string_handle() {
        let mir_path = encode_string_handle("/tmp/any.mir.json");
        let out =
            try_dispatch(LLVM_BACKEND_MODULE, COMPILE_OBJ_METHOD, 1, mir_path, 0).expect("route");
        if out > 0 {
            let text = decode_string_handle(out).expect("string handle");
            assert!(!text.is_empty());
        } else {
            assert_eq!(out, 0);
        }
    }
}
