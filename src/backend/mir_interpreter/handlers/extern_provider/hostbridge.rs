use super::super::*;
use crate::backend::mir_interpreter::utils::error_helpers::ErrorBuilder;
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
            ("env.codegen", "link_object")
                if std::env::var("HAKO_CABI_TRACE").ok().as_deref() == Some("1") =>
            {
                if let Some(a2) = args.get(2) {
                    let v = match self.reg_load(*a2) {
                        Ok(v) => v,
                        Err(_) => VMValue::Void,
                    };
                    match &v {
                        VMValue::BoxRef(b) => {
                            if b.as_any()
                                .downcast_ref::<crate::boxes::array::ArrayBox>()
                                .is_some()
                            {
                                crate::runtime::get_global_ring0()
                                    .log
                                    .debug("[hb:provider:args] link_object third=ArrayBox");
                            } else {
                                crate::runtime::get_global_ring0().log.debug(&format!(
                                    "[hb:provider:args] link_object third=BoxRef({})",
                                    b.type_name()
                                ));
                            }
                        }
                        other => {
                            crate::runtime::get_global_ring0().log.debug(&format!(
                                "[hb:provider:args] link_object third={:?}",
                                other
                            ));
                        }
                    }
                } else {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug("[hb:provider:args] link_object third=<none>");
                }
                let (objs, exe_out, extra) = if let Some(a2) = args.get(2) {
                    let v = self.reg_load(*a2)?;
                    match v {
                        VMValue::BoxRef(b) => {
                            if let Some(ab) =
                                b.as_any().downcast_ref::<crate::boxes::array::ArrayBox>()
                            {
                                let idx0: Box<dyn crate::box_trait::NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(0));
                                let elem0 = ab.get(idx0).to_string_box().value;
                                let mut exe: Option<String> = None;
                                let idx1: Box<dyn crate::box_trait::NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(1));
                                let e1 = ab.get(idx1).to_string_box().value;
                                if let Some(e1) = Self::optional_codegen_text(e1) {
                                    exe = Some(e1);
                                }
                                let mut extra: Option<String> = None;
                                let idx2: Box<dyn crate::box_trait::NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(2));
                                let e2 = ab.get(idx2).to_string_box().value;
                                if let Some(e2) = Self::optional_codegen_text(e2) {
                                    extra = Some(e2);
                                }
                                (elem0, exe, extra)
                            } else {
                                (b.to_string_box().value, None, None)
                            }
                        }
                        _ => (v.to_string(), None, None),
                    }
                } else {
                    return Err(self
                        .err_invalid("extern_invoke env.codegen.link_object expects args array"));
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
                let obj = std::path::PathBuf::from(objs);
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
            ("env.codegen", "emit_object") => {
                if let Some(s) = first_arg_str {
                    let opts = Self::codegen_object_opts(None, None, None);
                    match crate::host_providers::llvm_codegen::emit_object_from_mir_json(&s, opts) {
                        Ok(p) => Ok(VMValue::String(p.to_string_lossy().into_owned())),
                        Err(e) => {
                            Err(self.err_with_context("env.codegen.emit_object", &e.to_string()))
                        }
                    }
                } else {
                    Err(self.err_invalid("extern_invoke env.codegen.emit_object expects 1 arg"))
                }
            }
            ("env.codegen", "compile_ll_text") => {
                let ll_text = match first_arg_str {
                    Some(s) => s,
                    None => {
                        return Err(self.err_invalid(
                            "extern_invoke env.codegen.compile_ll_text expects 1+ args",
                        ))
                    }
                };
                let out = if let Some(a2) = args.get(2) {
                    let v = self.reg_load(*a2)?;
                    match v {
                        VMValue::BoxRef(b) => {
                            if let Some(ab) =
                                b.as_any().downcast_ref::<crate::boxes::array::ArrayBox>()
                            {
                                let idx1: Box<dyn crate::box_trait::NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(1));
                                let s1 = ab.get(idx1).to_string_box().value;
                                Self::optional_codegen_text(s1).map(std::path::PathBuf::from)
                            } else {
                                let text = b.to_string_box().value;
                                Self::optional_codegen_text(text).map(std::path::PathBuf::from)
                            }
                        }
                        other => {
                            let text = other.to_string();
                            Self::optional_codegen_text(text).map(std::path::PathBuf::from)
                        }
                    }
                } else {
                    None
                };
                let opts = Self::codegen_object_opts(out, None, None);
                match crate::host_providers::llvm_codegen::ll_text_to_object(&ll_text, opts) {
                    Ok(p) => Ok(VMValue::String(p.to_string_lossy().into_owned())),
                    Err(e) => {
                        Err(self.err_with_context("env.codegen.compile_ll_text", &e.to_string()))
                    }
                }
            }
            ("env.codegen", "link_object") => {
                let mut obj_s: Option<String> = None;
                let mut exe_s: Option<String> = None;
                let mut extra_s: Option<String> = None;
                if let Some(a2) = args.get(2) {
                    let v = self.reg_load(*a2)?;
                    match v {
                        VMValue::BoxRef(b) => {
                            if let Some(ab) =
                                b.as_any().downcast_ref::<crate::boxes::array::ArrayBox>()
                            {
                                let idx0: Box<dyn crate::box_trait::NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(0));
                                obj_s = Some(ab.get(idx0).to_string_box().value);
                                let idx1: Box<dyn crate::box_trait::NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(1));
                                let s1 = ab.get(idx1).to_string_box().value;
                                if let Some(s1) = Self::optional_codegen_text(s1) {
                                    exe_s = Some(s1);
                                }
                                let idx2: Box<dyn crate::box_trait::NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(2));
                                let s2 = ab.get(idx2).to_string_box().value;
                                if let Some(s2) = Self::optional_codegen_text(s2) {
                                    extra_s = Some(s2);
                                }
                            } else {
                                obj_s = Some(b.to_string_box().value);
                            }
                        }
                        _ => obj_s = Some(v.to_string()),
                    }
                }
                if obj_s.is_none() {
                    obj_s = first_arg_str;
                }
                let objs = match obj_s {
                    Some(s) => s,
                    None => {
                        return Err(
                            self.err_invalid("extern_invoke env.codegen.link_object expects args")
                        )
                    }
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
                let obj = std::path::PathBuf::from(objs);
                let exe = exe_s
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|| std::env::temp_dir().join("hako_link_out.exe"));
                match crate::host_providers::llvm_codegen::link_object_capi(
                    &obj,
                    &exe,
                    extra_s.as_deref(),
                ) {
                    Ok(()) => Ok(VMValue::String(exe.to_string_lossy().into_owned())),
                    Err(e) => Err(ErrorBuilder::with_context(
                        "env.codegen.link_object",
                        &e.to_string(),
                    )),
                }
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
