use super::super::builder::IRBuilder;
use super::LowerCore;
use crate::mir::{MirFunction, ValueId};

impl LowerCore {
    pub fn lower_extern_call(
        &mut self,
        b: &mut dyn IRBuilder,
        dst: &Option<ValueId>,
        iface_name: &str,
        method_name: &str,
        args: &Vec<ValueId>,
        _func: &MirFunction,
    ) -> Result<(), String> {
        // env.console.log/warn/error/println → ConsoleBox に委譲（host-bridge有効時は直接ログ）
        if iface_name == "env.console"
            && (method_name == "log"
                || method_name == "println"
                || method_name == "warn"
                || method_name == "error")
        {
            if std::env::var("NYASH_JIT_HOST_BRIDGE").ok().as_deref() == Some("1") {
                // a0: 先頭引数を最小限で積む
                if let Some(arg0) = args.get(0) {
                    self.push_value_if_known_or_param(b, arg0);
                } else {
                    b.emit_const_i64(0);
                }
                let sym = match method_name {
                    "warn" => crate::jit::r#extern::host_bridge::SYM_HOST_CONSOLE_WARN,
                    "error" => crate::jit::r#extern::host_bridge::SYM_HOST_CONSOLE_ERROR,
                    _ => crate::jit::r#extern::host_bridge::SYM_HOST_CONSOLE_LOG,
                };
                b.emit_host_call(sym, 1, false);
                return Ok(());
            }
            b.emit_host_call(
                crate::jit::r#extern::collections::SYM_CONSOLE_BIRTH_H,
                0,
                true,
            );
            if let Some(arg0) = args.get(0) {
                self.push_value_if_known_or_param(b, arg0);
            }
            let decision = crate::jit::policy::invoke::decide_box_method(
                "ConsoleBox",
                method_name,
                2,
                dst.is_some(),
            );
            if let crate::jit::policy::invoke::InvokeDecision::PluginInvoke {
                type_id,
                method_id,
                box_type,
                ..
            } = decision
            {
                b.emit_plugin_invoke(type_id, method_id, 2, dst.is_some());
                crate::jit::observe::lower_plugin_invoke(
                    &box_type,
                    method_name,
                    type_id,
                    method_id,
                    2,
                );
            } else if dst.is_some() {
                b.emit_const_i64(0);
            }
            return Ok(());
        }
        if iface_name == "env.future" && method_name == "await" {
            if let Some(arg0) = args.get(0) {
                if let Some(pidx) = self.param_index.get(arg0).copied() {
                    b.emit_param_i64(pidx);
                } else if let Some(slot) = self.local_index.get(arg0).copied() {
                    b.load_local_i64(slot);
                } else if let Some(v) = self.known_i64.get(arg0).copied() {
                    b.emit_const_i64(v);
                } else {
                    b.emit_const_i64(-1);
                }
            } else {
                b.emit_const_i64(-1);
            }
            b.emit_host_call(crate::jit::r#extern::r#async::SYM_FUTURE_AWAIT_H, 1, true);
            let hslot = {
                let id = self.next_local;
                self.next_local += 1;
                id
            };
            b.store_local_i64(hslot);
            b.load_local_i64(hslot);
            b.emit_host_call(crate::jit::r#extern::result::SYM_RESULT_OK_H, 1, true);
            let ok_slot = {
                let id = self.next_local;
                self.next_local += 1;
                id
            };
            b.store_local_i64(ok_slot);
            b.emit_const_i64(0);
            b.emit_host_call(crate::jit::r#extern::result::SYM_RESULT_ERR_H, 1, true);
            let err_slot = {
                let id = self.next_local;
                self.next_local += 1;
                id
            };
            b.store_local_i64(err_slot);
            b.load_local_i64(hslot);
            b.emit_const_i64(0);
            b.emit_compare(crate::jit::lower::builder::CmpKind::Eq);
            b.load_local_i64(err_slot);
            b.load_local_i64(ok_slot);
            b.emit_select_i64();
            if let Some(d) = dst {
                self.handle_values.insert(*d);
                let slot = *self.local_index.entry(*d).or_insert_with(|| {
                    let id = self.next_local;
                    self.next_local += 1;
                    id
                });
                b.store_local_i64(slot);
            }
            return Ok(());
        }
        if iface_name == "env.future" && method_name == "spawn_instance" {
            if let Some(recv) = args.get(0) {
                if let Some(pidx) = self.param_index.get(recv).copied() {
                    b.emit_param_i64(pidx);
                } else {
                    b.emit_const_i64(-1);
                }
            } else {
                b.emit_const_i64(-1);
            }
            if let Some(meth) = args.get(1) {
                self.push_value_if_known_or_param(b, meth);
            } else {
                b.emit_const_i64(0);
            }
            if let Some(a2) = args.get(2) {
                self.push_value_if_known_or_param(b, a2);
            } else {
                b.emit_const_i64(0);
            }
            let argc_total = args.len().saturating_sub(1).max(0);
            b.emit_const_i64(argc_total as i64);
            b.emit_host_call(
                crate::jit::r#extern::r#async::SYM_FUTURE_SPAWN_INSTANCE3_I64,
                4,
                true,
            );
            if let Some(d) = dst {
                self.handle_values.insert(*d);
                let slot = *self.local_index.entry(*d).or_insert_with(|| {
                    let id = self.next_local;
                    self.next_local += 1;
                    id
                });
                b.store_local_i64(slot);
            }
            return Ok(());
        }
        self.unsupported += 1;
        Ok(())
    }
}
