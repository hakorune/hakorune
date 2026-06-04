// Compiled-stage1 llvm backend route helper.

use super::trace_log_args;
use std::path::Path;

const LLVM_BACKEND_MODULE: &str = "selfhost.shared.backend.llvm_backend";
const COMPILE_OBJ_METHOD: &str = "compile_obj";
const LINK_EXE_METHOD: &str = "link_exe";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LlvmBackendRoute {
    CompileObj,
    LinkExe,
}

pub(super) fn try_dispatch(
    module_name: &str,
    method_name: &str,
    arg_count: i64,
    arg1: i64,
    arg2: i64,
) -> Option<i64> {
    let route = if module_name != LLVM_BACKEND_MODULE {
        None
    } else {
        match method_name {
            COMPILE_OBJ_METHOD => Some(LlvmBackendRoute::CompileObj),
            LINK_EXE_METHOD => Some(LlvmBackendRoute::LinkExe),
            _ => None,
        }
    }?;

    match route {
        LlvmBackendRoute::CompileObj => {
            if arg_count < 1 {
                return Some(0);
            }
            let Some(mir_path) = crate::plugin::owned_string_from_handle(arg1) else {
                return Some(0);
            };
            let mir_json = match std::fs::read_to_string(Path::new(&mir_path)) {
                Ok(text) => text,
                Err(error) => {
                    trace_log_args(format_args!(
                        "[stage1/module_dispatch] llvm_backend compile_obj failed: {}",
                        error
                    ));
                    return Some(0);
                }
            };
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
            Some(
                match nyash_rust::host_providers::llvm_codegen::mir_json_text_object::compile_object_from_mir_json_text_boundary(&mir_json, opts) {
                    Ok(obj_path) => crate::plugin::materialize_owned_string(
                        obj_path.to_string_lossy().into_owned(),
                    ),
                    Err(error_text) => {
                        trace_log_args(format_args!(
                            "[stage1/module_dispatch] llvm_backend compile_obj failed: {}",
                            error_text
                        ));
                        0
                    }
                },
            )
        }
        LlvmBackendRoute::LinkExe => {
            if arg_count < 2 {
                return Some(0);
            }
            let Some(obj_path) = crate::plugin::owned_string_from_handle(arg1) else {
                return Some(0);
            };
            let Some(exe_path) = crate::plugin::owned_string_from_handle(arg2) else {
                return Some(0);
            };
            Some(
                match nyash_rust::host_providers::llvm_codegen::link_object_capi(
                    Path::new(&obj_path),
                    Path::new(&exe_path),
                    None,
                ) {
                    Ok(()) => 1,
                    Err(error_text) => {
                        trace_log_args(format_args!(
                            "[stage1/module_dispatch] llvm_backend link_exe failed: {}",
                            error_text
                        ));
                        0
                    }
                },
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{try_dispatch, COMPILE_OBJ_METHOD, LINK_EXE_METHOD, LLVM_BACKEND_MODULE};
    use crate::plugin::{materialize_owned_string, owned_string_from_handle};

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
            try_dispatch(LLVM_BACKEND_MODULE, COMPILE_OBJ_METHOD, 0, 0, 0),
            Some(0)
        );
        assert_eq!(
            try_dispatch(LLVM_BACKEND_MODULE, LINK_EXE_METHOD, 0, 0, 0),
            Some(0)
        );
        assert_eq!(
            try_dispatch("other.module", COMPILE_OBJ_METHOD, 0, 0, 0),
            None
        );
        assert_eq!(
            try_dispatch(LLVM_BACKEND_MODULE, "other_method", 0, 0, 0),
            None
        );
    }

    #[test]
    fn llvm_backend_link_exe_missing_arg_returns_zero_flag() {
        let obj = materialize_owned_string("/tmp/in.o".to_owned());
        let out = try_dispatch(LLVM_BACKEND_MODULE, LINK_EXE_METHOD, 1, obj, 0).expect("route");
        assert_eq!(out, 0);
    }

    #[test]
    fn llvm_backend_link_exe_request_requires_two_args() {
        let obj = materialize_owned_string("/tmp/in.o".to_owned());
        let exe = materialize_owned_string("/tmp/out.exe".to_owned());
        assert_eq!(
            try_dispatch(LLVM_BACKEND_MODULE, LINK_EXE_METHOD, 1, obj, exe),
            Some(0)
        );
        let out = try_dispatch(LLVM_BACKEND_MODULE, LINK_EXE_METHOD, 2, obj, exe).expect("route");
        assert!(out == 0 || out == 1);
    }

    #[test]
    fn llvm_backend_compile_obj_request_prefers_first_string_handle() {
        let mir_path = materialize_owned_string("/tmp/any.mir.json".to_owned());
        assert_eq!(
            try_dispatch(LLVM_BACKEND_MODULE, COMPILE_OBJ_METHOD, 1, 0, mir_path),
            Some(0)
        );
        let out =
            try_dispatch(LLVM_BACKEND_MODULE, COMPILE_OBJ_METHOD, 1, mir_path, 0).expect("route");
        if out > 0 {
            let text = owned_string_from_handle(out).expect("string handle");
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
