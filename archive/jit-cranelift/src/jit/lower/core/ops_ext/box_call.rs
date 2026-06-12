use super::super::builder::IRBuilder;
use super::LowerCore;
use crate::mir::{MirFunction, ValueId};

impl LowerCore {
    pub fn lower_box_call(
        &mut self,
        func: &MirFunction,
        b: &mut dyn IRBuilder,
        array: &ValueId,
        method: &str,
        args: &Vec<ValueId>,
        dst: Option<ValueId>,
    ) -> Result<bool, String> {
        if matches!(method, "sin" | "cos" | "abs" | "min" | "max") {
            super::super::core_hostcall::lower_math_call(
                func,
                b,
                &self.known_i64,
                &self.known_f64,
                &self.float_box_values,
                method,
                args,
                dst.clone(),
            );
            return Ok(true);
        }

        if let Some(bt) = self.box_type_map.get(array).cloned() {
            let is_core =
                bt == "StringBox" || bt == "ArrayBox" || bt == "MapBox" || bt == "PyRuntimeBox";
            if !is_core {
                if let Some(slot) = self.local_index.get(array).copied() {
                    b.load_local_i64(slot);
                } else if let Some(pidx) = self.param_index.get(array).copied() {
                    b.emit_param_i64(pidx);
                    b.emit_host_call(crate::jit::r#extern::handles::SYM_HANDLE_OF, 1, true);
                } else {
                    self.push_value_if_known_or_param(b, array);
                    b.emit_host_call(crate::jit::r#extern::handles::SYM_HANDLE_OF, 1, true);
                }
                let take_n = core::cmp::min(args.len(), 2);
                for i in 0..take_n {
                    if let Some(v) = args.get(i) {
                        self.push_value_if_known_or_param(b, v);
                    }
                }
                let argc = 1 + take_n;
                b.emit_plugin_invoke_by_name(method, argc, dst.is_some());
                if std::env::var("NYASH_JIT_TRACE_LOWER").ok().as_deref() == Some("1") {
                    crate::jit::events::emit_lower(
                        serde_json::json!({
                            "id": format!("plugin_name:{}:{}", bt, method),
                            "decision": "allow",
                            "reason": "plugin_invoke_by_name",
                            "argc": argc
                        }),
                        "plugin",
                        "<jit>",
                    );
                }
                if let Some(d) = dst {
                    self.handle_values.insert(d);
                    let slot = *self.local_index.entry(d).or_insert_with(|| {
                        let id = self.next_local;
                        self.next_local += 1;
                        id
                    });
                    b.store_local_i64(slot);
                }
                if std::env::var("NYASH_JIT_TRACE_LOWER").ok().as_deref() == Some("1") {
                    eprintln!("[LOWER] {}.{} via name-invoke (argc={})", bt, method, argc);
                }
                return Ok(true);
            }
        }

        if super::string_ops::lower_string_box_method(self, func, b, array, method, args, dst)? {
            return Ok(true);
        }
        if super::collections::lower_collection_box_method(self, func, b, array, method, args, dst)? {
            return Ok(true);
        }

        if std::env::var("NYASH_JIT_TRACE_LOWER").ok().as_deref() == Some("1") {
            let bt = self.box_type_map.get(array).cloned().unwrap_or_default();
            let is_param = self.param_index.contains_key(array);
            let has_local = self.local_index.contains_key(array);
            let is_handle = self.handle_values.contains(array);
            let mut arg_kinds: Vec<&'static str> = Vec::new();
            for a in args.iter().take(3) {
                let k = if self.known_i64.contains_key(a) {
                    "i"
                } else if self.known_str.contains_key(a) {
                    "s"
                } else if self.param_index.contains_key(a) {
                    "p"
                } else if self.local_index.contains_key(a) {
                    "l"
                } else if self.handle_values.contains(a) {
                    "h"
                } else {
                    "-"
                };
                arg_kinds.push(k);
            }
            let policy = crate::jit::policy::invoke::decide_box_method(
                &bt,
                method,
                1 + args.len(),
                dst.is_some(),
            );
            let policy_str = match policy {
                crate::jit::policy::invoke::InvokeDecision::HostCall { ref symbol, .. } => {
                    format!("hostcall:{}", symbol)
                }
                crate::jit::policy::invoke::InvokeDecision::PluginInvoke { .. } => {
                    "plugin_invoke".to_string()
                }
                crate::jit::policy::invoke::InvokeDecision::Fallback { ref reason } => {
                    format!("fallback:{}", reason)
                }
            };
            eprintln!(
                "[LOWER] unhandled BoxCall: box_type='{}' method='{}' recv[param?{} local?{} handle?{}] args={:?} policy={}",
                bt, method, is_param, has_local, is_handle, arg_kinds, policy_str
            );
        }
        Ok(false)
    }
}
