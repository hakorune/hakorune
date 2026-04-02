use crate::bid::{BidError, BidResult};
use crate::box_trait::{NyashBox, StringBox};
use serde_json::Value as JsonValue;
use std::path::PathBuf;

pub(super) fn handle_codegen(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "compile_ll_text" => {
            let ll_text = args
                .first()
                .map(|b| b.to_string_box().value)
                .unwrap_or_default();
            let out = args.get(1).map(|b| b.to_string_box().value);
            match compile_ll_text(&ll_text, out) {
                Ok(p) => Ok(Some(Box::new(StringBox::new(p)) as Box<dyn NyashBox>)),
                Err(_e) => Ok(None),
            }
        }
        "emit_object" => {
            let mir_json = args
                .first()
                .map(|b| b.to_string_box().value)
                .unwrap_or_default();
            match emit_object(&mir_json, false) {
                Ok(p) => Ok(Some(Box::new(StringBox::new(p)) as Box<dyn NyashBox>)),
                Err(_e) => Ok(None),
            }
        }
        "link_object" => {
            let obj_path = args
                .first()
                .map(|b| b.to_string_box().value)
                .unwrap_or_default();
            let exe_out = args.get(1).map(|b| b.to_string_box().value);
            let extra = args.get(2).map(|b| b.to_string_box().value);
            match link_object(&obj_path, exe_out, extra) {
                Ok(p) => Ok(Some(Box::new(StringBox::new(p)) as Box<dyn NyashBox>)),
                Err(_e) => Ok(None),
            }
        }
        _ => Err(BidError::PluginError),
    }
}

pub(crate) fn emit_object(mir_json: &str, patch_version: bool) -> Result<String, String> {
    let input = if patch_version {
        patch_mir_json_version(mir_json)
    } else {
        mir_json.to_string()
    };
    crate::host_providers::llvm_codegen::emit_object_from_mir_json(&input, codegen_opts(None))
        .map(|p| p.to_string_lossy().into_owned())
        .map_err(|e| e.to_string())
}

pub(crate) fn compile_ll_text(ll_text: &str, out: Option<String>) -> Result<String, String> {
    let out = out.and_then(optional_codegen_text).map(PathBuf::from);
    crate::host_providers::llvm_codegen::ll_text_to_object(ll_text, codegen_opts(out))
        .map(|p| p.to_string_lossy().into_owned())
        .map_err(|e| e.to_string())
}

pub(crate) fn link_object(
    obj_path: &str,
    exe_out: Option<String>,
    extra: Option<String>,
) -> Result<String, String> {
    if std::env::var("NYASH_LLVM_USE_CAPI").ok().as_deref() != Some("1")
        || std::env::var("HAKO_V1_EXTERN_PROVIDER_C_ABI").ok().as_deref() != Some("1")
    {
        return Err("env.codegen.link_object: C-API route disabled".to_string());
    }
    let obj = PathBuf::from(obj_path);
    let exe = exe_out
        .and_then(optional_codegen_text)
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("hako_link_out.exe"));
    crate::host_providers::llvm_codegen::link_object_capi(&obj, &exe, extra.as_deref())
        .map(|()| exe.to_string_lossy().into_owned())
        .map_err(|e| e.to_string())
}

pub(crate) fn optional_codegen_text(text: String) -> Option<String> {
    if text.is_empty() || text == "null" {
        None
    } else {
        Some(text)
    }
}

fn codegen_opts(out: Option<PathBuf>) -> crate::host_providers::llvm_codegen::Opts {
    let (compile_recipe, compat_replay) =
        crate::config::env::backend_codegen_request_defaults(None, None);
    crate::host_providers::llvm_codegen::Opts {
        out,
        nyrt: std::env::var("NYASH_EMIT_EXE_NYRT")
            .ok()
            .map(PathBuf::from),
        opt_level: std::env::var("HAKO_LLVM_OPT_LEVEL")
            .ok()
            .or_else(|| std::env::var("NYASH_LLVM_OPT_LEVEL").ok())
            .or(Some("0".to_string())),
        timeout_ms: None,
        compile_recipe,
        compat_replay,
    }
}

fn patch_mir_json_version(s: &str) -> String {
    match serde_json::from_str::<JsonValue>(s) {
        Ok(mut v) => {
            if let JsonValue::Object(ref mut m) = v {
                if !m.contains_key("version") {
                    m.insert("version".to_string(), JsonValue::from(0));
                    if let Ok(out) = serde_json::to_string(&v) {
                        return out;
                    }
                }
            }
            s.to_string()
        }
        Err(_) => s.to_string(),
    }
}
