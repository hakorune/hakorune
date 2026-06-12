use super::super::builder::IRBuilder;
use super::LowerCore;
use crate::mir::{MirFunction, ValueId};

impl LowerCore {
    pub fn lower_plugin_invoke(
        &mut self,
        b: &mut dyn IRBuilder,
        dst: &Option<ValueId>,
        box_val: &ValueId,
        method: &str,
        args: &Vec<ValueId>,
        _func: &MirFunction,
    ) -> Result<(), String> {
        // Copied logic from core.rs PluginInvoke arm (scoped to PyRuntimeBox path)
        let bt = self.box_type_map.get(box_val).cloned().unwrap_or_default();
        let m = method;
        if bt == "PyRuntimeBox" && (m == "import") {
            let argc = 1 + args.len();
            if let Some(pidx) = self.param_index.get(box_val).copied() {
                b.emit_param_i64(pidx);
            } else {
                self.push_value_if_known_or_param(b, box_val);
            }
            let decision =
                crate::jit::policy::invoke::decide_box_method(&bt, m, argc, dst.is_some());
            if let crate::jit::policy::invoke::InvokeDecision::PluginInvoke {
                type_id,
                method_id,
                box_type,
                ..
            } = decision
            {
                b.emit_plugin_invoke(type_id, method_id, argc, dst.is_some());
                crate::jit::observe::lower_plugin_invoke(&box_type, m, type_id, method_id, argc);
                if let Some(d) = dst {
                    self.handle_values.insert(*d);
                }
            } else if dst.is_some() {
                b.emit_const_i64(0);
            }
        } else if bt == "PyRuntimeBox" && (m == "getattr" || m == "call") {
            let argc = 1 + args.len();
            if let Some(pidx) = self.param_index.get(box_val).copied() {
                b.emit_param_i64(pidx);
            } else {
                b.emit_const_i64(-1);
            }
            for a in args.iter() {
                self.push_value_if_known_or_param(b, a);
            }
            b.emit_plugin_invoke_by_name(m, argc, dst.is_some());
            if let Some(d) = dst {
                self.handle_values.insert(*d);
                let slot = *self.local_index.entry(*d).or_insert_with(|| {
                    let id = self.next_local;
                    self.next_local += 1;
                    id
                });
                b.store_local_i64(slot);
            }
        } else if self.handle_values.contains(box_val) && (m == "getattr" || m == "call") {
            let argc = 1 + args.len();
            if let Some(slot) = self.local_index.get(box_val).copied() {
                b.load_local_i64(slot);
            } else {
                b.emit_const_i64(-1);
            }
            for a in args.iter() {
                self.push_value_if_known_or_param(b, a);
            }
            b.emit_plugin_invoke_by_name(m, argc, dst.is_some());
            if let Some(d) = dst {
                self.handle_values.insert(*d);
                let slot = *self.local_index.entry(*d).or_insert_with(|| {
                    let id = self.next_local;
                    self.next_local += 1;
                    id
                });
                b.store_local_i64(slot);
            }
        } else if (bt == "PyRuntimeBox" && (m == "birth" || m == "eval"))
            || (bt == "IntegerBox" && m == "birth")
            || (bt == "StringBox" && m == "birth")
            || (bt == "ConsoleBox" && m == "birth")
        {
            if dst.is_some() {
                b.emit_const_i64(0);
            }
        } else {
            self.unsupported += 1;
        }
        Ok(())
    }
}
