use super::*;
use crate::backend::mir_interpreter::utils::error_helpers::ErrorBuilder;
use serde_json::Value as JsonValue;
use std::sync::Arc;

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

    fn emit_mirbuilder_program_json(&mut self, program_json: &str) -> Result<VMValue, VMError> {
        match crate::runtime::mirbuilder_emit::emit_program_json_to_mir_json_with_env_imports(
            program_json,
        ) {
            Ok(out) => Ok(VMValue::String(Self::patch_mir_json_version(&out))),
            Err(e) => Err(self.err_with_context("env.mirbuilder.emit", &e.to_string())),
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
        match extern_name {
            // Console family (minimal)
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
                Some(Ok(VMValue::Void))
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
                Some(Ok(VMValue::Void))
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
                Some(Ok(VMValue::Void))
            }
            // Extern providers (env.mirbuilder / env.codegen)
            "env.mirbuilder.emit" | "env.mirbuilder_emit" => {
                // Guarded stub path for verify/Hakorune-primary bring-up
                if std::env::var("HAKO_V1_EXTERN_PROVIDER").ok().as_deref() == Some("1") {
                    return Some(Ok(VMValue::String(String::new())));
                }
                if crate::config::env::mirbuilder_delegate_forbidden() {
                    return Some(Err(ErrorBuilder::invalid_instruction(
                        crate::config::env::mirbuilder_delegate_forbidden_message(
                            "env.mirbuilder.emit",
                        ),
                    )));
                }
                if args.is_empty() {
                    return Some(Err(ErrorBuilder::arg_count_mismatch(
                        "env.mirbuilder.emit",
                        1,
                        args.len(),
                    )));
                }
                let program_json = match self.reg_load(args[0]) {
                    Ok(v) => v.to_string(),
                    Err(e) => return Some(Err(e)),
                };
                Some(self.emit_mirbuilder_program_json(&program_json))
            }
            "env.codegen.emit_object" => {
                // Guarded stub path for verify/Hakorune-primary bring-up
                if std::env::var("HAKO_V1_EXTERN_PROVIDER").ok().as_deref() == Some("1") {
                    return Some(Ok(VMValue::String(String::new())));
                }
                if args.is_empty() {
                    return Some(Err(ErrorBuilder::arg_count_mismatch(
                        "env.codegen.emit_object",
                        1,
                        args.len(),
                    )));
                }
                let mir_json_raw = match self.reg_load(args[0]) {
                    Ok(v) => v.to_string(),
                    Err(e) => return Some(Err(e)),
                };
                // Normalize to v1 shape if missing/legacy (prevents harness NoneType errors)
                let mir_json = Self::patch_mir_json_version(&mir_json_raw);
                let opts = crate::host_providers::llvm_codegen::Opts {
                    out: None,
                    nyrt: std::env::var("NYASH_EMIT_EXE_NYRT")
                        .ok()
                        .map(std::path::PathBuf::from),
                    opt_level: std::env::var("HAKO_LLVM_OPT_LEVEL")
                        .ok()
                        .or(Some("0".to_string())),
                    timeout_ms: None,
                };
                let res = match crate::host_providers::llvm_codegen::mir_json_to_object(
                    &mir_json, opts,
                ) {
                    Ok(p) => Ok(VMValue::String(p.to_string_lossy().into_owned())),
                    Err(e) => Err(ErrorBuilder::with_context(
                        "env.codegen.emit_object",
                        &e.to_string(),
                    )),
                };
                Some(res)
            }
            "env.codegen.link_object" => {
                // Only supported on C-API route; expect 1 or 2 args: obj_path [, exe_out]
                let obj_path = match args.get(0) {
                    Some(v) => match self.reg_load(*v) {
                        Ok(v) => v.to_string(),
                        Err(e) => return Some(Err(e)),
                    },
                    None => {
                        return Some(Err(
                            self.err_invalid("env.codegen.link_object expects 1+ args")
                        ))
                    }
                };
                let exe_out = match args.get(1) {
                    Some(v) => Some(match self.reg_load(*v) {
                        Ok(v) => v.to_string(),
                        Err(e) => return Some(Err(e)),
                    }),
                    None => None,
                };
                let extra = match args.get(2) {
                    Some(v) => Some(match self.reg_load(*v) {
                        Ok(v) => v.to_string(),
                        Err(e) => return Some(Err(e)),
                    }),
                    None => None,
                };
                // Require C-API toggles
                if std::env::var("NYASH_LLVM_USE_CAPI").ok().as_deref() != Some("1")
                    || std::env::var("HAKO_V1_EXTERN_PROVIDER_C_ABI")
                        .ok()
                        .as_deref()
                        != Some("1")
                {
                    return Some(Err(ErrorBuilder::invalid_instruction(
                        "env.codegen.link_object: C-API route disabled",
                    )));
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
                    Ok(()) => Some(Ok(VMValue::String(exe.to_string_lossy().into_owned()))),
                    Err(e) => Some(Err(ErrorBuilder::with_context(
                        "env.codegen.link_object",
                        &e.to_string(),
                    ))),
                }
            }
            // Environment
            "env.get" => {
                if args.is_empty() {
                    return Some(Err(ErrorBuilder::arg_count_mismatch(
                        "env.get",
                        1,
                        args.len(),
                    )));
                }
                let key = match self.reg_load(args[0]) {
                    Ok(v) => v.to_string(),
                    Err(e) => return Some(Err(e)),
                };
                let val = std::env::var(&key).ok();
                Some(Ok(match val {
                    Some(s) => VMValue::String(s),
                    None => VMValue::Void,
                }))
            }
            "env.now_ms" => {
                if !args.is_empty() {
                    return Some(Err(ErrorBuilder::arg_count_mismatch(
                        "env.now_ms",
                        0,
                        args.len(),
                    )));
                }
                let now_ms =
                    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                        Ok(d) => d.as_millis() as i64,
                        Err(_) => 0,
                    };
                Some(Ok(VMValue::Integer(now_ms)))
            }
            "env.set" => {
                if args.len() < 2 {
                    return Some(Err(ErrorBuilder::arg_count_mismatch(
                        "env.set",
                        2,
                        args.len(),
                    )));
                }
                let key = match self.reg_load(args[0]) {
                    Ok(v) => v.to_string(),
                    Err(e) => return Some(Err(e)),
                };
                let val = match self.reg_load(args[1]) {
                    Ok(VMValue::Void) => String::new(),
                    Ok(v) => v.to_string(),
                    Err(e) => return Some(Err(e)),
                };
                std::env::set_var(&key, &val);
                Some(Ok(VMValue::Void))
            }
            // Direct env.box_introspect.kind extern (ExternCall form)
            "env.box_introspect.kind" => {
                use crate::box_trait::{NyashBox, StringBox};
                use crate::runtime::plugin_loader_v2;

                let mut collected: Vec<Box<dyn NyashBox>> = Vec::new();
                if let Some(arg_reg) = args.get(0) {
                    let v = match self.reg_load(*arg_reg) {
                        Ok(v) => v,
                        Err(e) => return Some(Err(e)),
                    };
                    match v {
                        VMValue::BoxRef(b) => collected.push(b.clone_box()),
                        other => {
                            collected.push(Box::new(StringBox::new(&other.to_string())));
                        }
                    }
                } else {
                    return Some(Err(
                        self.err_invalid("env.box_introspect.kind expects 1 arg")
                    ));
                }

                #[cfg(all(feature = "plugins", not(target_arch = "wasm32")))]
                let result = plugin_loader_v2::handle_box_introspect("kind", &collected);
                #[cfg(any(not(feature = "plugins"), target_arch = "wasm32"))]
                let result: crate::bid::BidResult<
                    Option<Box<dyn crate::box_trait::NyashBox>>,
                > = Err(crate::bid::BidError::PluginError);

                match result {
                    Ok(Some(b)) => Some(Ok(VMValue::BoxRef(Arc::from(b)))),
                    Ok(None) => Some(Ok(VMValue::Void)),
                    Err(e) => Some(Err(
                        self.err_with_context("env.box_introspect.kind", &format!("{:?}", e))
                    )),
                }
            }
            "hostbridge.extern_invoke" => {
                if std::env::var("HAKO_CABI_TRACE").ok().as_deref() == Some("1") {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug("[hb:entry:provider] hostbridge.extern_invoke");
                }
                if args.len() < 2 {
                    return Some(Err(
                        self.err_invalid("extern_invoke expects at least 2 args")
                    ));
                }
                let name = match self.reg_load(args[0]) {
                    Ok(v) => v.to_string(),
                    Err(e) => return Some(Err(e)),
                };
                let method = match self.reg_load(args[1]) {
                    Ok(v) => v.to_string(),
                    Err(e) => return Some(Err(e)),
                };
                // Extract first payload arg (optional)
                let mut first_arg_str: Option<String> = None;
                if let Some(a2) = args.get(2) {
                    let v = match self.reg_load(*a2) {
                        Ok(v) => v,
                        Err(e) => return Some(Err(e)),
                    };
                    match v {
                        VMValue::BoxRef(b) => {
                            if let Some(ab) =
                                b.as_any().downcast_ref::<crate::boxes::array::ArrayBox>()
                            {
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
                // Dispatch to known providers
                if std::env::var("HAKO_CABI_TRACE").ok().as_deref() == Some("1") {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug(&format!("[hb:dispatch:provider] {} {}", name, method));
                }
                let out = match (name.as_str(), method.as_str()) {
                    ("env.codegen", "link_object")
                        if std::env::var("HAKO_CABI_TRACE").ok().as_deref() == Some("1") =>
                    {
                        // Trace payload shape before actual handling
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
                        // fallthrough to real handler below by duplicating arm
                        // Args in third param (ArrayBox): [obj_path, exe_out?, extra_ldflags?]
                        let (objs, exe_out, extra) = if let Some(a2) = args.get(2) {
                            let v = match self.reg_load(*a2) {
                                Ok(v) => v,
                                Err(e) => return Some(Err(e)),
                            };
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
                                        if !e1.is_empty() {
                                            exe = Some(e1);
                                        }
                                        let mut extra: Option<String> = None;
                                        let idx2: Box<dyn crate::box_trait::NyashBox> =
                                            Box::new(crate::box_trait::IntegerBox::new(2));
                                        let e2 = ab.get(idx2).to_string_box().value;
                                        if !e2.is_empty() {
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
                            return Some(Err(self.err_invalid(
                                "extern_invoke env.codegen.link_object expects args array",
                            )));
                        };
                        if std::env::var("NYASH_LLVM_USE_CAPI").ok().as_deref() != Some("1")
                            || std::env::var("HAKO_V1_EXTERN_PROVIDER_C_ABI")
                                .ok()
                                .as_deref()
                                != Some("1")
                        {
                            return Some(Err(ErrorBuilder::invalid_instruction(
                                "env.codegen.link_object: C-API route disabled",
                            )));
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
                            return Some(Err(self.err_invalid(
                                &crate::config::env::mirbuilder_delegate_forbidden_message(
                                    "extern_invoke env.mirbuilder.emit",
                                ),
                            )));
                        }
                        if let Some(s) = first_arg_str {
                            self.emit_mirbuilder_program_json(&s)
                        } else {
                            Err(self.err_invalid("extern_invoke env.mirbuilder.emit expects 1 arg"))
                        }
                    }
                    ("env.codegen", "emit_object") => {
                        if let Some(s) = first_arg_str {
                            let opts = crate::host_providers::llvm_codegen::Opts {
                                out: None,
                                nyrt: std::env::var("NYASH_EMIT_EXE_NYRT")
                                    .ok()
                                    .map(std::path::PathBuf::from),
                                opt_level: std::env::var("HAKO_LLVM_OPT_LEVEL").ok(),
                                timeout_ms: None,
                            };
                            match crate::host_providers::llvm_codegen::mir_json_to_object(&s, opts)
                            {
                                Ok(p) => Ok(VMValue::String(p.to_string_lossy().into_owned())),
                                Err(e) => Err(self
                                    .err_with_context("env.codegen.emit_object", &e.to_string())),
                            }
                        } else {
                            Err(self
                                .err_invalid("extern_invoke env.codegen.emit_object expects 1 arg"))
                        }
                    }
                    ("env.codegen", "link_object") => {
                        // Unify both shapes:
                        // 1) third arg is ArrayBox [obj, exe?]
                        // 2) first_arg_str has obj and third arg has optional exe
                        let mut obj_s: Option<String> = None;
                        let mut exe_s: Option<String> = None;
                        let mut extra_s: Option<String> = None;
                        if let Some(a2) = args.get(2) {
                            let v = match self.reg_load(*a2) {
                                Ok(v) => v,
                                Err(e) => return Some(Err(e)),
                            };
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
                                        if !s1.is_empty() {
                                            exe_s = Some(s1);
                                        }
                                        let idx2: Box<dyn crate::box_trait::NyashBox> =
                                            Box::new(crate::box_trait::IntegerBox::new(2));
                                        let s2 = ab.get(idx2).to_string_box().value;
                                        if !s2.is_empty() {
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
                                return Some(Err(self.err_invalid(
                                    "extern_invoke env.codegen.link_object expects args",
                                )))
                            }
                        };
                        if std::env::var("NYASH_LLVM_USE_CAPI").ok().as_deref() != Some("1")
                            || std::env::var("HAKO_V1_EXTERN_PROVIDER_C_ABI")
                                .ok()
                                .as_deref()
                                != Some("1")
                        {
                            return Some(Err(ErrorBuilder::invalid_instruction(
                                "env.codegen.link_object: C-API route disabled",
                            )));
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
                        // hostbridge.extern_invoke("env.box_introspect","kind",[value])
                        // args layout (regs): [name, method, array_box_or_value, ...]
                        // For BoxTypeInspectorBox we only care about the first element of the ArrayBox.
                        use crate::box_trait::{NyashBox, StringBox};
                        use crate::runtime::plugin_loader_v2;

                        let mut collected: Vec<Box<dyn NyashBox>> = Vec::new();
                        if let Some(arg_reg) = args.get(2) {
                            let v = match self.reg_load(*arg_reg) {
                                Ok(v) => v,
                                Err(e) => return Some(Err(e)),
                            };
                            match v {
                                VMValue::BoxRef(b) => {
                                    if let Some(ab) =
                                        b.as_any().downcast_ref::<crate::boxes::array::ArrayBox>()
                                    {
                                        let idx0: Box<dyn NyashBox> =
                                            Box::new(crate::box_trait::IntegerBox::new(0));
                                        let elem0 = ab.get(idx0);
                                        if std::env::var("NYASH_BOX_INTROSPECT_TRACE")
                                            .ok()
                                            .as_deref()
                                            == Some("1")
                                        {
                                            crate::runtime::get_global_ring0().log.debug(&format!(
                                                "[box_introspect:extern] using ArrayBox[0] value_type={}",
                                                elem0.type_name()
                                            ));
                                        }
                                        collected.push(elem0);
                                    } else {
                                        if std::env::var("NYASH_BOX_INTROSPECT_TRACE")
                                            .ok()
                                            .as_deref()
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
                            return Some(Err(self.err_invalid(
                                "extern_invoke env.box_introspect.kind expects args array",
                            )));
                        }

                        #[cfg(all(feature = "plugins", not(target_arch = "wasm32")))]
                        let result = plugin_loader_v2::handle_box_introspect("kind", &collected);
                        #[cfg(any(not(feature = "plugins"), target_arch = "wasm32"))]
                        let result: crate::bid::BidResult<
                            Option<Box<dyn NyashBox>>,
                        > = Err(crate::bid::BidError::PluginError);

                        match result {
                            Ok(Some(b)) => Ok(VMValue::BoxRef(Arc::from(b))),
                            Ok(None) => Ok(VMValue::Void),
                            Err(e) => Err(self
                                .err_with_context("env.box_introspect.kind", &format!("{:?}", e))),
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
                };
                Some(out)
            }
            _ => None,
        }
    }
}
