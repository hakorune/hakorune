use super::super::*;
use crate::backend::mir_interpreter::utils::error_helpers::ErrorBuilder;
use std::sync::Arc;

impl MirInterpreter {
    pub(super) fn dispatch_loader_cold_extern(
        &mut self,
        extern_name: &str,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        match extern_name {
            "env.mirbuilder.emit" | "env.mirbuilder_emit" => {
                if std::env::var("HAKO_V1_EXTERN_PROVIDER").ok().as_deref() == Some("1") {
                    return Ok(VMValue::String(String::new()));
                }
                if crate::config::env::mirbuilder_delegate_forbidden() {
                    return Err(ErrorBuilder::invalid_instruction(
                        crate::config::env::mirbuilder_delegate_forbidden_message(
                            "env.mirbuilder.emit",
                        ),
                    ));
                }
                if args.is_empty() {
                    return Err(ErrorBuilder::arg_count_mismatch(
                        "env.mirbuilder.emit",
                        1,
                        args.len(),
                    ));
                }
                let program_json = self.reg_load(args[0])?.to_string();
                self.emit_mirbuilder_program_json(&program_json)
            }
            "env.codegen.emit_object" => {
                if std::env::var("HAKO_V1_EXTERN_PROVIDER").ok().as_deref() == Some("1") {
                    return Ok(VMValue::String(String::new()));
                }
                if args.is_empty() {
                    return Err(ErrorBuilder::arg_count_mismatch(
                        "env.codegen.emit_object",
                        1,
                        args.len(),
                    ));
                }
                let mir_json_raw = self.reg_load(args[0])?.to_string();
                let mir_json = Self::patch_mir_json_version(&mir_json_raw);
                let opts = Self::codegen_object_opts(None, None, None);
                match crate::host_providers::llvm_codegen::emit_object_from_mir_json(
                    &mir_json, opts,
                ) {
                    Ok(p) => Ok(VMValue::String(p.to_string_lossy().into_owned())),
                    Err(e) => Err(ErrorBuilder::with_context(
                        "env.codegen.emit_object",
                        &e.to_string(),
                    )),
                }
            }
            "env.codegen.compile_ll_text" => {
                if std::env::var("HAKO_V1_EXTERN_PROVIDER").ok().as_deref() == Some("1") {
                    return Ok(VMValue::String(String::new()));
                }
                let ll_text = match args.get(0) {
                    Some(v) => self.reg_load(*v)?.to_string(),
                    None => {
                        return Err(self.err_invalid("env.codegen.compile_ll_text expects 1+ args"))
                    }
                };
                let out = match args.get(1) {
                    Some(v) => Self::optional_codegen_text(self.reg_load(*v)?.to_string())
                        .map(std::path::PathBuf::from),
                    None => None,
                };
                let opts = Self::codegen_object_opts(out, None, None);
                match crate::host_providers::llvm_codegen::ll_text_to_object(&ll_text, opts) {
                    Ok(p) => Ok(VMValue::String(p.to_string_lossy().into_owned())),
                    Err(e) => Err(ErrorBuilder::with_context(
                        "env.codegen.compile_ll_text",
                        &e.to_string(),
                    )),
                }
            }
            "env.codegen.link_object" => {
                let obj_path = match args.get(0) {
                    Some(v) => self.reg_load(*v)?.to_string(),
                    None => return Err(self.err_invalid("env.codegen.link_object expects 1+ args")),
                };
                let exe_out = match args.get(1) {
                    Some(v) => Some(self.reg_load(*v)?.to_string()),
                    None => None,
                };
                let extra = match args.get(2) {
                    Some(v) => Some(self.reg_load(*v)?.to_string()),
                    None => None,
                };
                if std::env::var("NYASH_LLVM_USE_CAPI").ok().as_deref() != Some("1")
                    || std::env::var("HAKO_V1_EXTERN_PROVIDER_C_ABI")
                        .ok()
                        .as_deref()
                        != Some("1")
                {
                    return Err(ErrorBuilder::invalid_instruction(
                        "env.codegen.link_object: C-API route disabled",
                    ));
                }
                let obj = std::path::PathBuf::from(obj_path);
                let exe = exe_out
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|| std::env::temp_dir().join("hako_link_out.exe"));
                match crate::host_providers::llvm_codegen::link_object_capi(
                    &obj,
                    &exe,
                    extra.as_deref(),
                ) {
                    Ok(()) => Ok(VMValue::String(exe.to_string_lossy().into_owned())),
                    Err(e) => Err(ErrorBuilder::with_context(
                        "env.codegen.link_object",
                        &e.to_string(),
                    )),
                }
            }
            "env.box_introspect.kind" => {
                use crate::box_trait::{NyashBox, StringBox};
                use crate::runtime::plugin_loader_v2;

                let mut collected: Vec<Box<dyn NyashBox>> = Vec::new();
                if let Some(arg_reg) = args.get(0) {
                    let v = self.reg_load(*arg_reg)?;
                    match v {
                        VMValue::BoxRef(b) => collected.push(b.clone_box()),
                        other => collected.push(Box::new(StringBox::new(&other.to_string()))),
                    }
                } else {
                    return Err(self.err_invalid("env.box_introspect.kind expects 1 arg"));
                }

                #[cfg(all(feature = "plugins", not(target_arch = "wasm32")))]
                let result = plugin_loader_v2::handle_box_introspect("kind", &collected);
                #[cfg(any(not(feature = "plugins"), target_arch = "wasm32"))]
                let result: crate::bid::BidResult<
                    Option<Box<dyn crate::box_trait::NyashBox>>,
                > = Err(crate::bid::BidError::PluginError);

                match result {
                    Ok(Some(b)) => Ok(VMValue::BoxRef(Arc::from(b))),
                    Ok(None) => Ok(VMValue::Void),
                    Err(e) => {
                        Err(self.err_with_context("env.box_introspect.kind", &format!("{:?}", e)))
                    }
                }
            }
            "hostbridge.extern_invoke" => self.dispatch_loader_hostbridge_extern_invoke(args),
            _ => Err(self.err_invalid(format!(
                "loader-cold extern routed to unsupported name: {}",
                extern_name
            ))),
        }
    }
}
