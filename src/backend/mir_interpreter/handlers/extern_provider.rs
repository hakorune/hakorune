use super::*;
use crate::backend::mir_interpreter::utils::error_helpers::ErrorBuilder;
use serde_json::Value as JsonValue;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExternProviderLane {
    RuntimeDirect,
    LoaderCold,
}

fn classify_extern_provider_lane(extern_name: &str) -> Option<ExternProviderLane> {
    match extern_name {
        "nyash.console.log"
        | "env.console.log"
        | "print"
        | "nyash.builtin.print"
        | "env.console.warn"
        | "nyash.console.warn"
        | "env.error"
        | "env.error/1"
        | "env.console.error"
        | "env.console.error/1"
        | "nyash.console.error"
        | "env.get"
        | "env.now_ms"
        | "env.set" => Some(ExternProviderLane::RuntimeDirect),
        "env.mirbuilder.emit"
        | "env.mirbuilder_emit"
        | "env.codegen.emit_object"
        | "env.codegen.compile_json_path"
        | "env.codegen.compile_ll_text"
        | "env.codegen.link_object"
        | "env.box_introspect.kind"
        | "hostbridge.extern_invoke" => Some(ExternProviderLane::LoaderCold),
        _ => None,
    }
}

impl MirInterpreter {
    #[inline]
    fn should_trace_call_extern(target: &str, method: &str) -> bool {
        if let Ok(flt) = std::env::var("HAKO_CALL_TRACE_FILTER") {
            let key = format!("{}.{}", target, method);
            for pat in flt.split(',') {
                let p = pat.trim();
                if p.is_empty() {
                    continue;
                }
                if p == method || p == key {
                    return true;
                }
            }
            return false;
        }
        true
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

    fn codegen_object_opts(
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

    fn optional_codegen_text(text: String) -> Option<String> {
        if text.is_empty() || text == "null" {
            None
        } else {
            Some(text)
        }
    }

    fn emit_mirbuilder_program_json(&mut self, program_json: &str) -> Result<VMValue, VMError> {
        match crate::runtime::mirbuilder_emit::emit_program_json_to_mir_json_with_env_imports(
            program_json,
        ) {
            Ok(out) => Ok(VMValue::String(Self::patch_mir_json_version(&out))),
            Err(e) => Err(self.err_with_context("env.mirbuilder.emit", &e.to_string())),
        }
    }

    fn dispatch_runtime_direct_extern(
        &mut self,
        extern_name: &str,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        match extern_name {
            "nyash.console.log" | "env.console.log" | "print" | "nyash.builtin.print" => {
                if let Some(a0) = args.get(0) {
                    let v = self.reg_load(*a0).ok();
                    if let Some(v) = v {
                        match &v {
                            VMValue::Void => println!("null"),
                            VMValue::String(s) => println!("{}", s),
                            VMValue::BoxRef(bx) => {
                                if bx
                                    .as_any()
                                    .downcast_ref::<crate::box_trait::VoidBox>()
                                    .is_some()
                                {
                                    println!("null");
                                } else if let Some(sb) =
                                    bx.as_any().downcast_ref::<crate::box_trait::StringBox>()
                                {
                                    println!("{}", sb.value);
                                } else {
                                    println!("{}", v.to_string());
                                }
                            }
                            _ => println!("{}", v.to_string()),
                        }
                    } else {
                        println!("");
                    }
                } else {
                    println!("");
                }
                Ok(VMValue::Void)
            }
            "env.console.warn" | "nyash.console.warn" => {
                if let Some(a0) = args.get(0) {
                    let v = self.reg_load(*a0).ok();
                    if let Some(v) = v {
                        match &v {
                            VMValue::Void => eprintln!("[warn] null"),
                            VMValue::String(s) => eprintln!("[warn] {}", s),
                            VMValue::BoxRef(bx) => {
                                if bx
                                    .as_any()
                                    .downcast_ref::<crate::box_trait::VoidBox>()
                                    .is_some()
                                {
                                    eprintln!("[warn] null");
                                } else if let Some(sb) =
                                    bx.as_any().downcast_ref::<crate::box_trait::StringBox>()
                                {
                                    eprintln!("[warn] {}", sb.value);
                                } else {
                                    eprintln!("[warn] {}", v.to_string());
                                }
                            }
                            _ => eprintln!("[warn] {}", v.to_string()),
                        }
                    } else {
                        eprintln!("[warn]");
                    }
                } else {
                    eprintln!("[warn]");
                }
                Ok(VMValue::Void)
            }
            "env.error"
            | "env.error/1"
            | "env.console.error"
            | "env.console.error/1"
            | "nyash.console.error" => {
                if let Some(a0) = args.get(0) {
                    let v = self.reg_load(*a0).ok();
                    if let Some(v) = v {
                        match &v {
                            VMValue::Void => eprintln!("[error] null"),
                            VMValue::String(s) => eprintln!("[error] {}", s),
                            VMValue::BoxRef(bx) => {
                                if bx
                                    .as_any()
                                    .downcast_ref::<crate::box_trait::VoidBox>()
                                    .is_some()
                                {
                                    eprintln!("[error] null");
                                } else if let Some(sb) =
                                    bx.as_any().downcast_ref::<crate::box_trait::StringBox>()
                                {
                                    eprintln!("[error] {}", sb.value);
                                } else {
                                    eprintln!("[error] {}", v.to_string());
                                }
                            }
                            _ => eprintln!("[error] {}", v.to_string()),
                        }
                    } else {
                        eprintln!("[error]");
                    }
                } else {
                    eprintln!("[error]");
                }
                Ok(VMValue::Void)
            }
            "env.get" => {
                if args.is_empty() {
                    return Err(ErrorBuilder::arg_count_mismatch("env.get", 1, args.len()));
                }
                let key = self.reg_load(args[0])?.to_string();
                let val = std::env::var(&key).ok();
                Ok(match val {
                    Some(s) => VMValue::String(s),
                    None => VMValue::Void,
                })
            }
            "env.now_ms" => {
                if !args.is_empty() {
                    return Err(ErrorBuilder::arg_count_mismatch(
                        "env.now_ms",
                        0,
                        args.len(),
                    ));
                }
                let now_ms =
                    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                        Ok(d) => d.as_millis() as i64,
                        Err(_) => 0,
                    };
                Ok(VMValue::Integer(now_ms))
            }
            "env.set" => {
                if args.len() < 2 {
                    return Err(ErrorBuilder::arg_count_mismatch("env.set", 2, args.len()));
                }
                let key = self.reg_load(args[0])?.to_string();
                let val = match self.reg_load(args[1])? {
                    VMValue::Void => String::new(),
                    v => v.to_string(),
                };
                std::env::set_var(&key, &val);
                Ok(VMValue::Void)
            }
            _ => Err(self.err_invalid(format!(
                "runtime-direct extern routed to unsupported name: {}",
                extern_name
            ))),
        }
    }

    fn dispatch_loader_cold_extern(
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
                match crate::host_providers::llvm_codegen::mir_json_to_object(&mir_json, opts) {
                    Ok(p) => Ok(VMValue::String(p.to_string_lossy().into_owned())),
                    Err(e) => Err(ErrorBuilder::with_context(
                        "env.codegen.emit_object",
                        &e.to_string(),
                    )),
                }
            }
            "env.codegen.compile_json_path" => {
                let compile_recipe = match args.get(2) {
                    Some(v) => Some(self.reg_load(*v)?.to_string()),
                    None => None,
                };
                if crate::config::env::backend_compile_json_path_is_daily_owner(
                    compile_recipe.as_deref(),
                ) {
                    if crate::config::env::cabi_trace() {
                        crate::runtime::get_global_ring0().log.debug(
                            "[extern/c-abi:codegen.compile_json_path-retired]",
                        );
                    }
                    return Ok(VMValue::Void);
                }
                if std::env::var("HAKO_V1_EXTERN_PROVIDER").ok().as_deref() == Some("1") {
                    return Ok(VMValue::String(String::new()));
                }
                let json_path = match args.get(0) {
                    Some(v) => self.reg_load(*v)?.to_string(),
                    None => {
                        return Err(
                            self.err_invalid("env.codegen.compile_json_path expects 1+ args")
                        )
                    }
                };
                let out = match args.get(1) {
                    Some(v) => Self::optional_codegen_text(self.reg_load(*v)?.to_string())
                        .map(std::path::PathBuf::from),
                    None => None,
                };
                let compile_recipe = match args.get(2) {
                    Some(v) => Self::optional_codegen_text(self.reg_load(*v)?.to_string()),
                    None => None,
                };
                let compat_replay = match args.get(3) {
                    Some(v) => Self::optional_codegen_text(self.reg_load(*v)?.to_string()),
                    None => None,
                };
                let opts = Self::codegen_object_opts(out, compile_recipe, compat_replay);
                match crate::host_providers::llvm_codegen::mir_json_file_to_object(
                    std::path::Path::new(&json_path),
                    opts,
                ) {
                    Ok(p) => Ok(VMValue::String(p.to_string_lossy().into_owned())),
                    Err(e) => Err(ErrorBuilder::with_context(
                        "env.codegen.compile_json_path",
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

    fn dispatch_loader_hostbridge_extern_invoke(
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
                    match crate::host_providers::llvm_codegen::mir_json_to_object(&s, opts) {
                        Ok(p) => Ok(VMValue::String(p.to_string_lossy().into_owned())),
                        Err(e) => {
                            Err(self.err_with_context("env.codegen.emit_object", &e.to_string()))
                        }
                    }
                } else {
                    Err(self.err_invalid("extern_invoke env.codegen.emit_object expects 1 arg"))
                }
            }
            ("env.codegen", "compile_json_path") => {
                let compile_recipe = args.get(2).map(|v| self.reg_load(*v)).transpose()?;
                let compile_recipe = compile_recipe
                    .map(|value| value.to_string())
                    .filter(|s| !s.is_empty());
                if crate::config::env::backend_compile_json_path_is_daily_owner(
                    compile_recipe.as_deref(),
                ) {
                    if crate::config::env::cabi_trace() {
                        crate::runtime::get_global_ring0().log.debug(
                            "[extern/c-abi:codegen.compile_json_path-retired]",
                        );
                    }
                    return Ok(VMValue::Void);
                }
                let json_path = match first_arg_str {
                    Some(s) => s,
                    None => {
                        return Err(self.err_invalid(
                            "extern_invoke env.codegen.compile_json_path expects 1+ args",
                        ))
                    }
                };
                let (out, compile_recipe, compat_replay) = if let Some(a2) = args.get(2) {
                    let v = self.reg_load(*a2)?;
                    match v {
                        VMValue::BoxRef(b) => {
                            if let Some(ab) =
                                b.as_any().downcast_ref::<crate::boxes::array::ArrayBox>()
                            {
                                let idx1: Box<dyn crate::box_trait::NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(1));
                                let s1 = ab.get(idx1).to_string_box().value;
                                let idx2: Box<dyn crate::box_trait::NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(2));
                                let s2 = ab.get(idx2).to_string_box().value;
                                let idx3: Box<dyn crate::box_trait::NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(3));
                                let s3 = ab.get(idx3).to_string_box().value;
                                (
                                    Self::optional_codegen_text(s1).map(std::path::PathBuf::from),
                                    Self::optional_codegen_text(s2),
                                    Self::optional_codegen_text(s3),
                                )
                            } else {
                                let text = b.to_string_box().value;
                                (
                                    Self::optional_codegen_text(text).map(std::path::PathBuf::from),
                                    None,
                                    None,
                                )
                            }
                        }
                        other => {
                            let text = other.to_string();
                            (
                                Self::optional_codegen_text(text).map(std::path::PathBuf::from),
                                None,
                                None,
                            )
                        }
                    }
                } else {
                    (None, None, None)
                };
                let opts = Self::codegen_object_opts(out, compile_recipe, compat_replay);
                match crate::host_providers::llvm_codegen::mir_json_file_to_object(
                    std::path::Path::new(&json_path),
                    opts,
                ) {
                    Ok(p) => Ok(VMValue::String(p.to_string_lossy().into_owned())),
                    Err(e) => {
                        Err(self.err_with_context("env.codegen.compile_json_path", &e.to_string()))
                    }
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

    /// Central extern dispatcher used by both execute_extern_function (calls.rs)
    /// and handle_extern_call (externals.rs). Returns a VMValue; callers are
    /// responsible for writing it to registers when needed.
    pub(super) fn extern_provider_dispatch(
        &mut self,
        extern_name: &str,
        args: &[ValueId],
    ) -> Option<Result<VMValue, VMError>> {
        // Unified call trace (optional)
        if std::env::var("HAKO_CALL_TRACE").ok().as_deref() == Some("1") {
            // Split iface.method for filtering
            if let Some((iface, method)) = extern_name.rsplit_once('.') {
                if Self::should_trace_call_extern(iface, method) {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug(&format!("[call:{}.{}]", iface, method));
                }
            } else {
                // Fallback: no dot in extern name (e.g., 'print')
                if Self::should_trace_call_extern("", extern_name) {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug(&format!("[call:{}]", extern_name));
                }
            }
        }
        match classify_extern_provider_lane(extern_name) {
            Some(ExternProviderLane::RuntimeDirect) => {
                Some(self.dispatch_runtime_direct_extern(extern_name, args))
            }
            Some(ExternProviderLane::LoaderCold) => {
                Some(self.dispatch_loader_cold_extern(extern_name, args))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_extern_provider_lane, ExternProviderLane};

    #[test]
    fn classify_runtime_direct_lane_for_console_and_env() {
        for extern_name in [
            "print",
            "env.console.log",
            "env.console.warn",
            "env.console.error",
            "env.get",
            "env.now_ms",
            "env.set",
        ] {
            assert_eq!(
                classify_extern_provider_lane(extern_name),
                Some(ExternProviderLane::RuntimeDirect),
                "expected runtime-direct lane for {}",
                extern_name
            );
        }
    }

    #[test]
    fn classify_loader_cold_lane_for_provider_and_hostbridge() {
        for extern_name in [
            "env.mirbuilder.emit",
            "env.mirbuilder_emit",
            "env.codegen.emit_object",
            "env.codegen.compile_json_path",
            "env.codegen.compile_ll_text",
            "env.codegen.link_object",
            "env.box_introspect.kind",
            "hostbridge.extern_invoke",
        ] {
            assert_eq!(
                classify_extern_provider_lane(extern_name),
                Some(ExternProviderLane::LoaderCold),
                "expected loader-cold lane for {}",
                extern_name
            );
        }
    }

    #[test]
    fn classify_non_provider_names_as_none() {
        for extern_name in ["nyash.string.concat_hh", "exit", "panic"] {
            assert_eq!(classify_extern_provider_lane(extern_name), None);
        }
    }
}
