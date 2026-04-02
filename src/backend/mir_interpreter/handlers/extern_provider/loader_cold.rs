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
            "env.codegen.emit_object"
            | "env.codegen.compile_ll_text"
            | "env.codegen.link_object" => {
                return self.dispatch_loader_cold_codegen_extern(extern_name, args);
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
