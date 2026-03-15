use super::*;
use serde_json::Value as JsonValue;

impl MirInterpreter {
    #[inline]
    #[allow(dead_code)]
    fn ensure_mir_json_version_field(s: &str) -> String {
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
    #[allow(dead_code)]
    pub(super) fn handle_extern_call(
        &mut self,
        dst: Option<ValueId>,
        iface: &str,
        method: &str,
        args: &[ValueId],
    ) -> Result<(), VMError> {
        // Normalize method arity suffix (e.g., get/1 -> get)
        let mbase = super::super::utils::normalize_arity_suffix(method);
        match (iface, mbase) {
            ("env", "get") => {
                // Prefer provider-based resolution when available, fall back to process env.
                if let Some(provider_res) = self.extern_provider_dispatch("env.get", args) {
                    let result = provider_res?;
                    self.write_result(dst, result);
                    return Ok(());
                }
                if let Some(a0) = args.get(0) {
                    let key = self.reg_load(*a0)?.to_string();
                    let val = std::env::var(&key).ok();
                    let result = if let Some(s) = val {
                        VMValue::String(s)
                    } else {
                        // Represent missing env as null-equivalent (Void)
                        VMValue::Void
                    };
                    self.write_result(dst, result);
                }
                Ok(())
            }
            ("env", "mirbuilder_emit") => {
                if let Some(provider_res) =
                    self.extern_provider_dispatch("env.mirbuilder.emit", args)
                {
                    let result = provider_res?;
                    self.write_result(dst, result);
                    return Ok(());
                }
                Err(self.err_invalid("ExternCall env.mirbuilder_emit not supported".to_string()))
            }
            ("env.console", "log") => {
                if let Some(a0) = args.get(0) {
                    let v = self.reg_load(*a0)?;
                    // Dev-only: mirror print-trace for extern console.log
                    if Self::print_trace_enabled() {
                        self.print_trace_emit(&v);
                    }
                    // Treat VM Void and BoxRef(VoidBox) as JSON null for dev ergonomics
                    match &v {
                        VMValue::Void => {
                            println!("null");
                            self.write_void(dst);
                            return Ok(());
                        }
                        VMValue::BoxRef(bx) => {
                            if bx
                                .as_any()
                                .downcast_ref::<crate::box_trait::VoidBox>()
                                .is_some()
                            {
                                println!("null");
                                self.write_void(dst);
                                return Ok(());
                            }
                            if let Some(sb) =
                                bx.as_any().downcast_ref::<crate::box_trait::StringBox>()
                            {
                                println!("{}", sb.value);
                                self.write_void(dst);
                                return Ok(());
                            }
                        }
                        VMValue::String(s) => {
                            println!("{}", s);
                            self.write_void(dst);
                            return Ok(());
                        }
                        _ => {}
                    }
                    // Operator Box (Stringify) – dev flag gated
                    if std::env::var("NYASH_OPERATOR_BOX_STRINGIFY")
                        .ok()
                        .as_deref()
                        == Some("1")
                    {
                        if let Some(op) = self.functions.get("StringifyOperator.apply/1").cloned() {
                            let out = self.exec_function_inner(&op, Some(&[v.clone()]))?;
                            println!("{}", out.to_string());
                        } else {
                            println!("{}", v.to_string());
                        }
                    } else {
                        println!("{}", v.to_string());
                    }
                }
                self.write_void(dst);
                Ok(())
            }
            ("env.future", "new") => {
                let fut = crate::boxes::future::NyashFutureBox::new();
                if let Some(a0) = args.get(0) {
                    let v = self.load_as_box(*a0)?;
                    fut.set_result(v);
                }
                self.write_result(dst, VMValue::Future(fut));
                Ok(())
            }
            ("env.future", "set") => {
                if args.len() >= 2 {
                    let f = self.reg_load(args[0])?;
                    let v = self.load_as_box(args[1])?;
                    if let VMValue::Future(fut) = f {
                        fut.set_result(v);
                    } else {
                        return Err(VMError::TypeError("env.future.set expects Future".into()));
                    }
                }
                self.write_void(dst);
                Ok(())
            }
            ("env.future", "await") => {
                if let Some(a0) = args.get(0) {
                    let f = self.reg_load(*a0)?;
                    match f {
                        VMValue::Future(fut) => {
                            let v = fut.get();
                            self.write_result(dst, VMValue::from_nyash_box(v));
                        }
                        _ => {
                            return Err(VMError::TypeError("await expects Future".into()));
                        }
                    }
                }
                Ok(())
            }
            ("env.runtime", "checkpoint") => {
                crate::runtime::global_hooks::safepoint_and_poll();
                self.write_void(dst);
                Ok(())
            }
            ("env.modules", "set") => {
                if args.len() >= 2 {
                    let k = self.load_as_string(args[0])?;
                    let v = self.load_as_box(args[1])?;
                    crate::runtime::modules_registry::set(k, v);
                }
                self.write_void(dst);
                Ok(())
            }
            ("env.modules", "get") => {
                if let Some(a0) = args.get(0) {
                    let k = self.reg_load(*a0)?.to_string();
                    let vb = crate::runtime::modules_registry::get(&k)
                        .unwrap_or_else(|| Box::new(crate::box_trait::VoidBox::new()));
                    self.write_result(dst, VMValue::from_nyash_box(vb));
                }
                Ok(())
            }
            ("env", "set") => {
                // Delegate to provider
                let ret = self
                    .extern_provider_dispatch("env.set", args)
                    .unwrap_or(Ok(VMValue::Void))?;
                self.write_result(dst, ret);
                Ok(())
            }
            ("env.mirbuilder", "emit") => {
                let ret = self
                    .extern_provider_dispatch("env.mirbuilder.emit", args)
                    .unwrap_or(Ok(VMValue::Void))?;
                self.write_result(dst, ret);
                Ok(())
            }
            ("env.codegen", "emit_object") => {
                let ret = self
                    .extern_provider_dispatch("env.codegen.emit_object", args)
                    .unwrap_or(Ok(VMValue::Void))?;
                self.write_result(dst, ret);
                Ok(())
            }
            ("env.codegen", "link_object") => {
                // Args in third param (ArrayBox): [obj_path, exe_out?, extra_ldflags?]
                // Note: This branch is used for ExternCall form; provider toggles must be ON.
                if std::env::var("NYASH_LLVM_USE_CAPI").ok().as_deref() != Some("1")
                    || std::env::var("HAKO_V1_EXTERN_PROVIDER_C_ABI")
                        .ok()
                        .as_deref()
                        != Some("1")
                {
                    return Err(self.err_invalid("env.codegen.link_object: C-API route disabled"));
                }
                // Extract array payload
                let (obj_path, exe_out, extra) = if let Some(a2) = args.get(2) {
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
                                let mut extra: Option<String> = None;
                                let idx1: Box<dyn crate::box_trait::NyashBox> =
                                    Box::new(crate::box_trait::IntegerBox::new(1));
                                let e1 = ab.get(idx1).to_string_box().value;
                                if !e1.is_empty() {
                                    exe = Some(e1);
                                }
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
                    return Err(self
                        .err_invalid("extern_invoke env.codegen.link_object expects args array"));
                };
                let obj = std::path::PathBuf::from(obj_path);
                let exe = exe_out
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|| std::env::temp_dir().join("hako_link_out.exe"));
                crate::host_providers::llvm_codegen::link_object_capi(&obj, &exe, extra.as_deref())
                    .map_err(|e| {
                        self.err_with_context("env.codegen.link_object", &e.to_string())
                    })?;
                self.write_result(dst, VMValue::String(exe.to_string_lossy().into_owned()));
                Ok(())
            }
            ("env.box_introspect", "kind") => {
                // Route env.box_introspect.kind to extern provider (plugin_loader_v2)
                let ret = self
                    .extern_provider_dispatch("env.box_introspect.kind", args)
                    .unwrap_or(Ok(VMValue::Void))?;
                self.write_result(dst, ret);
                Ok(())
            }
            ("hostbridge", "extern_invoke") => {
                if let Some(res) = self.extern_provider_dispatch("hostbridge.extern_invoke", args) {
                    match res {
                        Ok(v) => {
                            self.write_result(dst, v);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                    return Ok(());
                }
                return Err(self.err_invalid("hostbridge.extern_invoke unsupported [externals]"));
            }

            // Phase 288.1: REPL session variable bridge
            ("__repl", "get") => {
                // args: [name: String]
                if args.len() != 1 {
                    return Err(self.err_invalid(format!(
                        "__repl.get expects 1 argument, got {}",
                        args.len()
                    )));
                }

                let name = self.reg_load(args[0])?.to_string();

                // REPL session から取得
                if let Some(session) = &self.repl_session {
                    // Clone the value before the borrow ends to avoid borrowing conflicts
                    let value_opt = session.borrow().get(&name).cloned();
                    match value_opt {
                        Some(value) => {
                            self.write_result(dst, value);
                            Ok(())
                        }
                        None => Err(self.err_invalid(format!(
                            "Undefined variable: '{}'\nHint: Variable not defined. Assign a value first.",
                            name
                        ))),
                    }
                } else {
                    Err(self.err_invalid("__repl.get called outside REPL mode".to_string()))
                }
            }

            ("__repl", "set") => {
                // args: [name: String, value: Any]
                if args.len() != 2 {
                    return Err(self.err_invalid(format!(
                        "__repl.set expects 2 arguments, got {}",
                        args.len()
                    )));
                }

                let name = self.reg_load(args[0])?.to_string();
                let value = self.reg_load(args[1])?.clone();

                // REPL session に保存
                if let Some(session) = &self.repl_session {
                    session.borrow_mut().set(name, value);
                    self.write_void(dst);
                    Ok(())
                } else {
                    Err(self.err_invalid("__repl.set called outside REPL mode".to_string()))
                }
            }

            _ => Err(self.err_invalid(format!("ExternCall {}.{} not supported", iface, method))),
        }
    }
}
