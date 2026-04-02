//! External function implementations for `env.codegen.*`.

use crate::bid::{BidError, BidResult};
use crate::box_trait::{NyashBox, StringBox};

pub(super) fn handle_codegen(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    fn codegen_opts(
        out: Option<std::path::PathBuf>,
        compile_recipe: Option<String>,
        compat_replay: Option<String>,
    ) -> crate::host_providers::llvm_codegen::Opts {
        let (compile_recipe, compat_replay) =
            crate::config::env::backend_codegen_request_defaults(compile_recipe, compat_replay);
        crate::host_providers::llvm_codegen::Opts {
            out,
            nyrt: std::env::var("NYASH_EMIT_EXE_NYRT")
                .ok()
                .map(std::path::PathBuf::from),
            opt_level: std::env::var("HAKO_LLVM_OPT_LEVEL")
                .ok()
                .or_else(|| std::env::var("NYASH_LLVM_OPT_LEVEL").ok())
                .or(Some("0".to_string())),
            timeout_ms: None,
            compile_recipe,
            compat_replay,
        }
    }

    match method_name {
        "compile_ll_text" => {
            let ll_text = args
                .first()
                .map(|b| b.to_string_box().value)
                .unwrap_or_default();
            let out = args
                .get(1)
                .map(|b| b.to_string_box().value)
                .filter(|s| !s.is_empty() && s != "null")
                .map(std::path::PathBuf::from);
            match crate::host_providers::llvm_codegen::ll_text_to_object(
                &ll_text,
                codegen_opts(out, None, None),
            ) {
                Ok(p) => {
                    let s = p.to_string_lossy().into_owned();
                    Ok(Some(Box::new(StringBox::new(s)) as Box<dyn NyashBox>))
                }
                Err(_e) => Ok(None),
            }
        }
        "emit_object" => {
            let mir_json = args
                .get(0)
                .map(|b| b.to_string_box().value)
                .unwrap_or_default();
            match crate::host_providers::llvm_codegen::emit_object_from_mir_json(
                &mir_json,
                codegen_opts(None, None, None),
            ) {
                Ok(p) => {
                    let s = p.to_string_lossy().into_owned();
                    Ok(Some(Box::new(StringBox::new(s)) as Box<dyn NyashBox>))
                }
                Err(_e) => Ok(None),
            }
        }
        _ => Err(BidError::PluginError),
    }
}
