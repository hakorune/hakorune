use super::{decode_string_handle, encode_string_handle, trace_log};
use serde_json::json;
use std::fs;
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
    let mir_path = decode_required_arg(arg_count, arg1, arg2)?;
    let mir_json = fs::read_to_string(&mir_path).ok()?;
    let mir_payload = inject_v1_meta_externs(&mir_json)?;
    let opts = nyash_rust::host_providers::llvm_codegen::Opts {
        out: None,
        nyrt: std::env::var("NYASH_EMIT_EXE_NYRT")
            .ok()
            .map(std::path::PathBuf::from),
        opt_level: std::env::var("HAKO_LLVM_OPT_LEVEL")
            .ok()
            .or_else(|| std::env::var("NYASH_LLVM_OPT_LEVEL").ok())
            .or(Some("0".to_string())),
        timeout_ms: None,
    };
    match nyash_rust::host_providers::llvm_codegen::mir_json_to_object(&mir_payload, opts) {
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
    if arg_count < 2 {
        return Some(0);
    }
    let obj_path = decode_string_handle(arg1)?;
    let exe_path = decode_string_handle(arg2)?;
    match nyash_rust::host_providers::llvm_codegen::link_object_capi(
        Path::new(&obj_path),
        Path::new(&exe_path),
        None,
    ) {
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

fn decode_required_arg(arg_count: i64, arg1: i64, arg2: i64) -> Option<String> {
    if arg_count < 1 {
        return None;
    }
    decode_string_handle(arg1).or_else(|| decode_string_handle(arg2))
}

fn inject_v1_meta_externs(mir_json: &str) -> Option<String> {
    let mut value: serde_json::Value = serde_json::from_str(mir_json).ok()?;
    let root = value.as_object_mut()?;
    root.insert("kind".to_string(), serde_json::Value::String("MIR".to_string()));
    root.insert(
        "schema_version".to_string(),
        serde_json::Value::String("1.0".to_string()),
    );

    match root.get_mut("metadata") {
        Some(serde_json::Value::Object(metadata)) => {
            metadata
                .entry("extern_c".to_string())
                .or_insert_with(|| json!([]));
        }
        _ => {
            root.insert("metadata".to_string(), json!({ "extern_c": [] }));
        }
    }

    serde_json::to_string(&value).ok()
}

#[cfg(test)]
mod tests {
    use super::{
        inject_v1_meta_externs, try_dispatch, COMPILE_OBJ_METHOD, LINK_EXE_METHOD,
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
    fn llvm_backend_link_exe_missing_arg_returns_zero_flag() {
        let obj = encode_string_handle("/tmp/in.o");
        let out = try_dispatch(LLVM_BACKEND_MODULE, LINK_EXE_METHOD, 1, obj, 0).expect("route");
        assert_eq!(out, 0);
    }

    #[test]
    fn llvm_backend_inject_meta_adds_schema_and_metadata() {
        let mir_json = r#"{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"ret","value":1}]}]}]}"#;
        let out = inject_v1_meta_externs(mir_json).expect("json");
        let value: serde_json::Value = serde_json::from_str(&out).expect("valid json");
        assert_eq!(value["kind"], "MIR");
        assert_eq!(value["schema_version"], "1.0");
        assert_eq!(value["metadata"]["extern_c"], serde_json::json!([]));
        assert!(value["functions"].is_array());
    }

    #[test]
    fn llvm_backend_compile_obj_decode_prefers_first_string_handle() {
        let mir_path = encode_string_handle("/tmp/any.mir.json");
        let out = try_dispatch(LLVM_BACKEND_MODULE, COMPILE_OBJ_METHOD, 1, mir_path, 0)
            .expect("route");
        if out > 0 {
            let text = decode_string_handle(out).expect("string handle");
            assert!(!text.is_empty());
        } else {
            assert_eq!(out, 0);
        }
    }
}
