use super::super::*;
use std::sync::Arc;

impl MirInterpreter {
    pub(super) fn dispatch_loader_hostbridge_extern_invoke(
        &mut self,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        if std::env::var("HAKO_CABI_TRACE").ok().as_deref() == Some("1") {
            crate::runtime::get_global_ring0()
                .log
                .debug("[hb:entry:provider] hostbridge.extern_invoke");
        }
        if args.len() < 2 {
            return Err(self.err_invalid("extern_invoke expects at least 2 args"));
        }
        let name = self.reg_load(args[0])?.to_string();
        let method = self.reg_load(args[1])?.to_string();
        let mut first_arg_str: Option<String> = None;
        if let Some(a2) = args.get(2) {
            let v = self.reg_load(*a2)?;
            match v {
                VMValue::BoxRef(b) => {
                    if let Some(ab) = b.as_any().downcast_ref::<crate::boxes::array::ArrayBox>() {
                        let idx: Box<dyn crate::box_trait::NyashBox> =
                            Box::new(crate::box_trait::IntegerBox::new(0));
                        let elem = ab.get(idx);
                        first_arg_str = Some(elem.to_string_box().value);
                    } else {
                        first_arg_str = Some(b.to_string_box().value);
                    }
                }
                _ => first_arg_str = Some(v.to_string()),
            }
        }
        if std::env::var("HAKO_CABI_TRACE").ok().as_deref() == Some("1") {
            crate::runtime::get_global_ring0()
                .log
                .debug(&format!("[hb:dispatch:provider] {} {}", name, method));
        }
        match (name.as_str(), method.as_str()) {
            ("env.mirbuilder", "emit") => {
                if crate::config::env::mirbuilder_delegate_forbidden() {
                    return Err(self.err_invalid(
                        &crate::config::env::mirbuilder_delegate_forbidden_message(
                            "extern_invoke env.mirbuilder.emit",
                        ),
                    ));
                }
                if let Some(s) = first_arg_str {
                    self.emit_mirbuilder_program_json(&s)
                } else {
                    Err(self.err_invalid("extern_invoke env.mirbuilder.emit expects 1 arg"))
                }
            }
            ("env.file", "read") => {
                if let Some(path) = first_arg_str {
                    match std::fs::read_to_string(&path) {
                        Ok(text) => Ok(VMValue::String(text)),
                        Err(_) => Ok(VMValue::Void),
                    }
                } else {
                    Err(self.err_invalid("extern_invoke env.file.read expects 1 arg"))
                }
            }
            ("env.codegen", "emit_object" | "compile_ll_text" | "link_object") => {
                return self.dispatch_loader_hostbridge_codegen_invoke(args);
            }
            ("env.box_introspect", "kind") => {
                use crate::box_trait::{NyashBox, StringBox};
                use crate::runtime::plugin_loader_v2;

                let mut collected: Vec<Box<dyn NyashBox>> = Vec::new();
                if let Some(arg_reg) = args.get(2) {
                    let v = self.reg_load(*arg_reg)?;
                    match v {
                        VMValue::BoxRef(b) => {
                            if let Some(ab) =
                                b.as_any().downcast_ref::<crate::boxes::array::ArrayBox>()
                            {
                                let idx0: Box<dyn NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(0));
                                let elem0 = ab.get(idx0);
                                if std::env::var("NYASH_BOX_INTROSPECT_TRACE").ok().as_deref()
                                    == Some("1")
                                {
                                    crate::runtime::get_global_ring0().log.debug(&format!(
                                        "[box_introspect:extern] using ArrayBox[0] value_type={}",
                                        elem0.type_name()
                                    ));
                                }
                                collected.push(elem0);
                            } else {
                                if std::env::var("NYASH_BOX_INTROSPECT_TRACE").ok().as_deref()
                                    == Some("1")
                                {
                                    crate::runtime::get_global_ring0().log.debug(&format!(
                                        "[box_introspect:extern] using BoxRef({}) directly",
                                        b.type_name()
                                    ));
                                }
                                collected.push(b.clone_box());
                            }
                        }
                        other => {
                            if std::env::var("NYASH_BOX_INTROSPECT_TRACE").ok().as_deref()
                                == Some("1")
                            {
                                crate::runtime::get_global_ring0().log.debug(&format!(
                                    "[box_introspect:extern] non-box arg kind={:?}",
                                    other
                                ));
                            }
                            collected.push(Box::new(StringBox::new(&other.to_string())));
                        }
                    }
                } else {
                    return Err(self
                        .err_invalid("extern_invoke env.box_introspect.kind expects args array"));
                }

                #[cfg(all(feature = "plugins", not(target_arch = "wasm32")))]
                let result = plugin_loader_v2::handle_box_introspect("kind", &collected);
                #[cfg(any(not(feature = "plugins"), target_arch = "wasm32"))]
                let result: crate::bid::BidResult<Option<Box<dyn NyashBox>>> =
                    Err(crate::bid::BidError::PluginError);

                match result {
                    Ok(Some(b)) => Ok(VMValue::BoxRef(Arc::from(b))),
                    Ok(None) => Ok(VMValue::Void),
                    Err(e) => {
                        Err(self.err_with_context("env.box_introspect.kind", &format!("{:?}", e)))
                    }
                }
            }
            _ => {
                if std::env::var("HAKO_CABI_TRACE").ok().as_deref() == Some("1") {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug(&format!("[hb:unsupported:provider] {}.{}", name, method));
                }
                Err(self.err_invalid(format!(
                    "hostbridge.extern_invoke unsupported for {}.{} [provider] (check extern_provider_dispatch / env.* handlers)",
                    name, method
                )))
            }
        }
    }
}
