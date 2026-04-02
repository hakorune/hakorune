use super::super::*;
use crate::backend::mir_interpreter::utils::error_helpers::ErrorBuilder;

impl MirInterpreter {
    pub(super) fn dispatch_loader_hostbridge_codegen_invoke(
        &mut self,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        let name = "env.codegen";
        let method = "dispatch";
        if std::env::var("HAKO_CABI_TRACE").ok().as_deref() == Some("1") {
            crate::runtime::get_global_ring0()
                .log
                .debug("[hb:entry:provider] hostbridge.codegen dispatch");
        }
        if args.len() < 2 {
            return Err(self.err_invalid("extern_invoke expects at least 2 args"));
        }
        let iface = self.reg_load(args[0])?.to_string();
        let method_name = self.reg_load(args[1])?.to_string();
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
                .debug(&format!("[hb:dispatch:provider] {} {}", iface, method_name));
        }
        match (iface.as_str(), method_name.as_str()) {
            ("env.codegen", "emit_object") => {
                if let Some(s) = first_arg_str {
                    let opts = Self::codegen_object_opts(None, None, None);
                    match crate::host_providers::llvm_codegen::emit_object_from_mir_json(&s, opts)
                    {
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
                        );
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
            _ => Err(self.err_invalid(format!(
                "unsupported hostbridge.codegen dispatch: {}.{}",
                name, method
            ))),
        }
    }

    pub(super) fn dispatch_loader_cold_codegen_extern(
        &mut self,
        extern_name: &str,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        match extern_name {
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
            other => Err(self.err_invalid(format!(
                "loader-cold extern routed to unsupported codegen name: {}",
                other
            ))),
        }
    }
}
