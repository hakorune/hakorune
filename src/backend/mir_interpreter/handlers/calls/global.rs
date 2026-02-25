use super::*;
use crate::mir::naming::StaticMethodId;

impl MirInterpreter {
    pub(super) fn execute_global_function(
        &mut self,
        func_name: &str,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        // 🎯 Phase 21.7++ Phase 2: StaticMethodId SSOT 実装
        let canonical = if let Some(mut id) = StaticMethodId::parse(func_name) {
            // 1. Parse success - this is a static method call (e.g., "Box.method" or "Box.method/N")
            // 2. Complement arity if not specified
            if id.arity.is_none() {
                id = id.with_arity(args.len());
            }
            // 3. Format to canonical name
            id.format()
        } else {
            // Parse failed - not a static method format.
            // Keep plain global names as-is (e.g. "id"), and let explicit "name/N"
            // stay explicit when provided by the caller.
            crate::mir::naming::normalize_static_global_name(func_name)
        };

        // Normalize arity suffix for extern-like dispatch, but keep canonical/original name
        // for module-local function table lookup (functions may carry arity suffix).
        let base = super::super::utils::normalize_arity_suffix(&canonical);

        // 🔍 Debug: Check function lookup (Phase 21.7++ Phase 2.2: StaticMethodId info)
        if std::env::var("NYASH_DEBUG_FUNCTION_LOOKUP").ok().as_deref() == Some("1") {
            crate::runtime::get_global_ring0().log.debug(&format!("[DEBUG/vm] Looking up function: '{}'", func_name));

            // Phase 2.2: Show parsed StaticMethodId info
            if let Some(id) = StaticMethodId::parse(func_name) {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[DEBUG/vm]   Parsed: box='{}', method='{}', arity={:?}",
                    id.box_name, id.method, id.arity
                ));
            } else {
                crate::runtime::get_global_ring0().log.debug("[DEBUG/vm]   Not a static method (builtin?)");
            }

            crate::runtime::get_global_ring0().log.debug(&format!("[DEBUG/vm]   canonical: '{}'", canonical));
            crate::runtime::get_global_ring0().log.debug(&format!("[DEBUG/vm]   base: '{}'", base));
            crate::runtime::get_global_ring0().log.debug(&format!("[DEBUG/vm]   Available functions: {}", self.functions.len()));
            if !self.functions.contains_key(&canonical) {
                crate::runtime::get_global_ring0().log.debug(&format!("[DEBUG/vm]   ❌ '{}' NOT found in functions", canonical));
                // List functions starting with same prefix
                let prefix = if let Some(idx) = canonical.find('.') {
                    &canonical[..idx]
                } else {
                    &canonical
                };
                let matching: Vec<_> = self
                    .functions
                    .keys()
                    .filter(|k| k.starts_with(prefix))
                    .collect();
                if !matching.is_empty() {
                    crate::runtime::get_global_ring0().log.debug("[DEBUG/vm]   Similar functions:");
                    for k in matching.iter().take(10) {
                        crate::runtime::get_global_ring0().log.debug(&format!("[DEBUG/vm]     - {}", k));
                    }
                }
            } else {
                crate::runtime::get_global_ring0().log.debug(&format!("[DEBUG/vm]   ✅ '{}' found", canonical));
            }
        }

        // Module-local/global function: execute by function table if present.
        // まず canonical 名で探す（Main._nop/0 など）。Phase 25.x 時点では
        // レガシー名での再探索は廃止し、NamingBox 側の正規化に一本化する。
        if let Some(func) = self.functions.get(&canonical).cloned() {
            let mut argv: Vec<VMValue> = Vec::with_capacity(args.len());
            for a in args {
                argv.push(self.reg_load(*a)?);
            }
            return self.exec_function_inner(&func, Some(&argv));
        }

        match base {
            // Console-like globals
            "print" | "nyash.builtin.print" => {
                // Reuse extern handler for consistency with other console names
                return self.execute_extern_function("print", args);
            }
            "error" => {
                if let Some(arg_id) = args.get(0) {
                    let val = self.reg_load(*arg_id)?;
                    eprintln!("Error: {}", val.to_string());
                }
                return Ok(VMValue::Void);
            }
            "panic" => {
                return self.execute_extern_function("panic", args);
            }
            "exit" => {
                return self.execute_extern_function("exit", args);
            }
            "env.get" => {
                // Route env.get global to extern handler
                return self.execute_extern_function("env.get", args);
            }
            "env.now_ms" => {
                // Route env.now_ms global to extern handler
                return self.execute_extern_function("env.now_ms", args);
            }
            "hostbridge.extern_invoke" => {
                // SSOT: delegate to extern dispatcher (provider)
                return self.execute_extern_function("hostbridge.extern_invoke", args);
            }
            // LLVM harness providers (direct)
            "env.mirbuilder.emit" | "env.mirbuilder.emit/1" => {
                if let Some(a0) = args.get(0) {
                    let s = self.reg_load(*a0)?.to_string();
                    match crate::host_providers::mir_builder::program_json_to_mir_json(&s) {
                        Ok(out) => Ok(VMValue::String(out)),
                        Err(e) => Err(self.err_with_context("env.mirbuilder.emit", &e.to_string())),
                    }
                } else {
                    Err(self.err_invalid("env.mirbuilder.emit expects 1 arg"))
                }
            }
            "env.codegen.emit_object" | "env.codegen.emit_object/1" => {
                if let Some(a0) = args.get(0) {
                    let s = self.reg_load(*a0)?.to_string();
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
                    match crate::host_providers::llvm_codegen::mir_json_to_object(&s, opts) {
                        Ok(p) => Ok(VMValue::String(p.to_string_lossy().into_owned())),
                        Err(e) => {
                            Err(self.err_with_context("env.codegen.emit_object", &e.to_string()))
                        }
                    }
                } else {
                    Err(self.err_invalid("env.codegen.emit_object expects 1 arg"))
                }
            }
            "env.codegen.link_object" | "env.codegen.link_object/3" => {
                // C-API route only; args[2] is expected to be an ArrayBox [obj_path, exe_out?]
                if std::env::var("NYASH_LLVM_USE_CAPI").ok().as_deref() != Some("1")
                    || std::env::var("HAKO_V1_EXTERN_PROVIDER_C_ABI")
                        .ok()
                        .as_deref()
                        != Some("1")
                {
                    return Err(self.err_invalid("env.codegen.link_object: C-API route disabled"));
                }
                if args.len() < 3 {
                    return Err(self.err_arg_count("env.codegen.link_object", 3, args.len()));
                }
                let v = self.reg_load(args[2])?;
                let (obj_path, exe_out) = match v {
                    VMValue::BoxRef(b) => {
                        if let Some(ab) = b.as_any().downcast_ref::<crate::boxes::array::ArrayBox>()
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
                            (elem0, exe)
                        } else {
                            (b.to_string_box().value, None)
                        }
                    }
                    _ => (v.to_string(), None),
                };
                let extra = std::env::var("HAKO_AOT_LDFLAGS").ok();
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
                    Err(e) => Err(self.err_with_context("env.codegen.link_object", &e.to_string())),
                }
            }
            "nyash.builtin.error" => {
                if let Some(arg_id) = args.get(0) {
                    let val = self.reg_load(*arg_id)?;
                    eprintln!("Error: {}", val.to_string());
                }
                Ok(VMValue::Void)
            }
            _ => {
                // ⚠️ Phase 0.2: User-friendly "Did you mean?" suggestions
                let prefix = if let Some(idx) = canonical.find('.') {
                    &canonical[..idx]
                } else {
                    &canonical
                };

                let similar: Vec<_> = self
                    .functions
                    .keys()
                    .filter(|k| k.starts_with(prefix))
                    .take(5)
                    .collect();

                let mut err_msg = format!("Function not found: {}", func_name);

                if !similar.is_empty() {
                    err_msg.push_str("\n\n💡 Did you mean:");
                    for s in similar {
                        err_msg.push_str(&format!("\n   - {}", s));
                    }
                }

                err_msg
                    .push_str("\n\n🔍 Debug: NYASH_DEBUG_FUNCTION_LOOKUP=1 for full lookup trace");

                // NamingBox SSOT: ここで canonical に失敗したら素直に Unknown とする。
                // レガシーフォールバック（functions.get(func_name) 再探索）は Phase 25.x で廃止済み。
                Err(self.err_with_context("global function", &err_msg))
            }
        }
    }
}
