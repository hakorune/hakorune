use super::*;

fn select_unique_tail_method_key<'a, I>(
    method: &str,
    args_len: usize,
    recv_cls: Option<&str>,
    keys: I,
) -> Option<String>
where
    I: Iterator<Item = &'a str>,
{
    let tail = format!(".{}/{}", method, args_len);
    let mut cands: Vec<&str> = keys.filter(|k| k.ends_with(&tail)).collect();
    if let Some(want) = recv_cls {
        let prefix = format!("{}.", want);
        cands.retain(|k| k.starts_with(&prefix));
    }
    if cands.len() == 1 {
        Some(cands[0].to_string())
    } else {
        None
    }
}

fn fallback_wants_receiver(callee_params: usize, args_len: usize) -> bool {
    callee_params == args_len + 1
}

impl MirInterpreter {
    pub(super) fn handle_new_box(
        &mut self,
        dst: ValueId,
        box_type: &str,
        args: &[ValueId],
    ) -> Result<(), VMError> {
        // Provider Lock guard (受け口・既定は挙動不変)
        if let Err(e) = crate::runtime::provider_lock::guard_before_new_box(box_type) {
            return Err(self.err_invalid(e));
        }

        // Temporary phase-29ck seam: allow placeholder `newbox(hostbridge)` so the
        // vm-hako backend bridge can be hosted by the regular Rust VM during proof.
        if box_type == "hostbridge" {
            let instance = crate::instance_v2::InstanceBox::from_declaration(
                "hostbridge".to_string(),
                vec![],
                std::collections::HashMap::new(),
            );
            let created_vm = VMValue::from_nyash_box(Box::new(instance));
            self.write_reg(dst, created_vm);
            return Ok(());
        }

        // Fast path (bench/profile-only): new StringBox(...) without registry roundtrip.
        // Under NYASH_VM_FAST we keep StringBox payload as VMValue::String to avoid Arc/Box churn
        // on tight loops (e.g., box_create_destroy_small).
        if self.vm_fast_enabled && box_type == "StringBox" {
            // new StringBox() -> ""
            if args.is_empty() {
                self.write_reg(dst, VMValue::String(String::new()));
                if Self::box_trace_enabled() {
                    self.box_trace_emit_new(box_type, args.len());
                }
                return Ok(());
            }
            // new StringBox(x) where x is already string-like.
            if args.len() == 1 {
                let v0 = self.reg_load(args[0])?;
                let s_opt: Option<String> = match v0 {
                    VMValue::String(s) => Some(s),
                    VMValue::BoxRef(b) => b
                        .as_any()
                        .downcast_ref::<crate::boxes::basic::StringBox>()
                        .map(|sb| sb.value.clone()),
                    _ => None,
                };
                if let Some(s) = s_opt {
                    self.write_reg(dst, VMValue::String(s));
                    if Self::box_trace_enabled() {
                        self.box_trace_emit_new(box_type, args.len());
                    }
                    return Ok(());
                }
            }
        }

        // User-defined boxes (Program/MIR): create InstanceBox directly from compiler metadata.
        //
        // Why:
        // - `--json-file` execution path uses MirInterpreter directly (no VM runner),
        //   so UnifiedBoxRegistry may not have a user-defined factory registered.
        // - We must still allow NewBox for user-defined boxes even when plugins are disabled.
        if let Some(fields) = self.user_box_field_decls.get(box_type).cloned() {
            let argv = self.load_args_as_boxes(args)?;
            let mut instance = crate::instance_v2::InstanceBox::from_declaration(
                box_type.to_string(),
                fields,
                std::collections::HashMap::new(),
            );
            let _ = instance.init(&argv);
            let created_vm = VMValue::from_nyash_box(Box::new(instance));
            self.write_reg(dst, created_vm);
            if Self::box_trace_enabled() {
                self.box_trace_emit_new(box_type, args.len());
            }
            return Ok(());
        }

        // Static box singleton path (Program JSON imports, etc.).
        // If a box name is detected via MIR functions, treat `NewBox BoxName` as
        // acquiring the singleton instance (not a plugin/builtin provider).
        if self.static_box_registry.exists(box_type) {
            let inst = self.ensure_static_box_instance(box_type)?;
            let created_vm = VMValue::from_nyash_box(Box::new(inst.clone()));
            self.write_reg(dst, created_vm);
            if Self::box_trace_enabled() {
                self.box_trace_emit_new(box_type, args.len());
            }
            return Ok(());
        }

        let converted = self.load_args_as_boxes(args)?;
        let reg = crate::runtime::unified_registry::get_global_unified_registry();
        let created = reg
            .lock()
            .unwrap()
            .create_box(box_type, &converted)
            .map_err(|e| self.err_with_context(&format!("NewBox {}", box_type), &e.to_string()))?;

        // Store created instance first so 'me' can be passed to birth
        let created_vm = VMValue::from_nyash_box(created);
        self.write_reg(dst, created_vm.clone());

        // Trace: new box event (dev-only)
        if Self::box_trace_enabled() {
            self.box_trace_emit_new(box_type, args.len());
        }

        // Note: birth の自動呼び出しは削除。
        // 正しい設計は Builder が NewBox 後に明示的に birth 呼び出しを生成すること。
        Ok(())
    }

    fn boxcall_receiver_class_name(&self, box_val: ValueId) -> Option<String> {
        match self.reg_load(box_val).ok() {
            Some(VMValue::BoxRef(b)) => {
                if let Some(inst) = b.as_any().downcast_ref::<crate::instance_v2::InstanceBox>() {
                    Some(inst.class_name.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn fallback_resolve_user_method(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        method: &str,
        args: &[ValueId],
    ) -> Result<bool, VMError> {
        let recv_cls = self.boxcall_receiver_class_name(box_val);
        let func_key = match select_unique_tail_method_key(
            method,
            args.len(),
            recv_cls.as_deref(),
            self.functions.keys().map(|k| k.as_str()),
        ) {
            Some(k) => k,
            None => return Ok(false),
        };
        let Some(func) = self.functions.get(&func_key).cloned() else {
            return Ok(false);
        };

        // Build argv for user-defined methods:
        // - If callee arity matches args.len(), treat as static-box style (no implicit receiver).
        // - If callee arity matches args.len() + 1, treat as instance style (prepend receiver as `me`).
        // This avoids argument shifting bugs for `static box Foo { m(a,b) }` functions compiled as Foo.m/2.
        let recv_vm = self.reg_load(box_val)?;
        let callee_params = func.signature.params.len();
        let want_me = fallback_wants_receiver(callee_params, args.len());
        let mut argv: Vec<VMValue> = Vec::with_capacity(args.len() + if want_me { 1 } else { 0 });
        if want_me {
            argv.push(recv_vm);
        }
        for a in args {
            argv.push(self.reg_load(*a)?);
        }
        let ret = self.exec_function_inner(&func, Some(&argv))?;
        self.write_result(dst, ret);
        Ok(true)
    }

    /// S-5.2-improved: Made public for JoinIR Runner integration via execute_box_call wrapper
    pub fn handle_box_call(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        method: &str,
        args: &[ValueId],
    ) -> Result<(), VMError> {
        if super::boxcall_prelude::run_boxcall_prelude(self, dst, box_val, method, args)? {
            return Ok(());
        }
        if super::boxcall_dispatch::dispatch_box_call_handlers(self, dst, box_val, method, args)? {
            return Ok(());
        }
        // Fallback: unique-tail dynamic resolution for user-defined methods
        // Narrowing: restrict to receiver's class when available to avoid
        // accidentally binding methods from unrelated boxes that happen to
        // share the same method name/arity (e.g., JsonScanner.is_eof vs JsonToken.is_eof).
        if self.fallback_resolve_user_method(dst, box_val, method, args)? {
            return Ok(());
        }

        super::boxes_plugin::invoke_plugin_box(self, dst, box_val, method, args)
    }

    // moved: try_handle_map_box → handlers/boxes_map.rs
    #[allow(dead_code)]
    fn try_handle_map_box(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        method: &str,
        args: &[ValueId],
    ) -> Result<bool, VMError> {
        super::boxes_map::try_handle_map_box(self, dst, box_val, method, args)
    }

    // moved: try_handle_string_box → handlers/boxes_string.rs
    #[allow(dead_code)]
    fn try_handle_string_box(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        method: &str,
        args: &[ValueId],
    ) -> Result<bool, VMError> {
        super::boxes_string::try_handle_string_box(self, dst, box_val, method, args)
    }

    // moved: try_handle_array_box → handlers/boxes_array.rs
    #[allow(dead_code)]
    fn try_handle_array_box(
        &mut self,
        dst: Option<ValueId>,
        box_val: ValueId,
        method: &str,
        args: &[ValueId],
    ) -> Result<bool, VMError> {
        super::boxes_array::try_handle_array_box(self, dst, box_val, method, args)
    }
}

#[cfg(test)]
mod tests {
    use super::{fallback_wants_receiver, select_unique_tail_method_key};

    #[test]
    fn fallback_wants_receiver_contract() {
        assert!(!fallback_wants_receiver(2, 2));
        assert!(fallback_wants_receiver(3, 2));
        assert!(!fallback_wants_receiver(4, 2));
    }

    #[test]
    fn select_unique_tail_method_key_respects_receiver_class() {
        let keys = vec!["Foo.bar/2", "Bar.bar/2", "Foo.baz/2"];
        let pick = select_unique_tail_method_key("bar", 2, None, keys.iter().copied());
        assert!(pick.is_none());

        let pick = select_unique_tail_method_key("bar", 2, Some("Foo"), keys.iter().copied());
        assert_eq!(pick.as_deref(), Some("Foo.bar/2"));

        let pick = select_unique_tail_method_key("bar", 2, Some("Baz"), keys.iter().copied());
        assert!(pick.is_none());
    }
}
