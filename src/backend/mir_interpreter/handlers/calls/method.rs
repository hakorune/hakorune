use super::*;
use crate::backend::mir_interpreter::handlers::string_method_helpers::{
    parse_index_of_args, parse_last_index_of_args, parse_substring_args,
    try_eval_string_char_predicate, ArgParsePolicy,
};
use crate::backend::mir_interpreter::handlers::temp_dispatch::{
    with_temp_receiver_dispatch, TMP_OUT_INSTANCE_FIELD_OP, TMP_OUT_INSTANCE_METHOD_BRIDGE,
    TMP_OUT_OBJECT_FIELD_METHOD_BRIDGE, TMP_RECV_INSTANCE_FIELD_OP,
    TMP_RECV_INSTANCE_METHOD_BRIDGE, TMP_RECV_OBJECT_FIELD_METHOD_BRIDGE,
};
use crate::box_trait::{NyashBox, StringBox as CoreStringBox};
use crate::boxes::array::{ArrayBox, ArrayMethodId, ArraySurfaceInvokeResult};
use crate::boxes::basic::{StringMethodId, StringSurfaceInvokeResult};
use crate::boxes::string_ops;
use crate::boxes::{MapBox, MapMethodId, MapSurfaceInvokeResult};
use crate::config::env;
use crate::runtime::get_global_ring0;

#[path = "method/dispatch.rs"]
mod dispatch;
#[cfg(test)]
#[path = "method/tests.rs"]
mod tests;

impl MirInterpreter {
    fn load_array_surface_args(
        &mut self,
        method_id: ArrayMethodId,
        args: &[ValueId],
    ) -> Result<Vec<Box<dyn NyashBox>>, VMError> {
        let expected = method_id.arity();
        if args.len() < expected {
            return Err(self.err_invalid(format!(
                "ArrayBox.{}: invalid receiver or missing arguments",
                method_id.canonical_name()
            )));
        }

        args.iter()
            .take(expected)
            .map(|arg| self.load_as_box(*arg))
            .collect()
    }

    fn invoke_array_surface(
        &mut self,
        array: &ArrayBox,
        method_id: ArrayMethodId,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        let surface_args = self.load_array_surface_args(method_id, args)?;
        let result = array
            .invoke_surface(method_id, surface_args)
            .map_err(|err| self.err_invalid(err.to_string()))?;
        Ok(match result {
            ArraySurfaceInvokeResult::Value(value) => VMValue::from_nyash_box(value),
            ArraySurfaceInvokeResult::Void => VMValue::Void,
        })
    }

    fn load_map_surface_args(
        &mut self,
        method_id: MapMethodId,
        args: &[ValueId],
    ) -> Result<Vec<Box<dyn NyashBox>>, VMError> {
        let expected = method_id.arity();
        if args.len() < expected {
            return Err(self.err_invalid(format!(
                "MapBox.{}: invalid receiver or missing arguments",
                method_id.canonical_name()
            )));
        }

        args.iter()
            .take(expected)
            .map(|arg| self.load_as_box(*arg))
            .collect()
    }

    fn invoke_map_surface(
        &mut self,
        map: &MapBox,
        method_id: MapMethodId,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        let surface_args = self.load_map_surface_args(method_id, args)?;
        let result = map
            .invoke_surface(method_id, surface_args)
            .map_err(|err| self.err_invalid(err.to_string()))?;
        Ok(match result {
            MapSurfaceInvokeResult::Value(value) => VMValue::from_nyash_box(value),
        })
    }

    fn load_string_surface_args(
        &mut self,
        method_id: StringMethodId,
        args: &[ValueId],
    ) -> Result<Vec<Box<dyn NyashBox>>, VMError> {
        let expected = method_id.arity();
        if args.len() < expected {
            return Err(self.err_invalid(format!(
                "StringBox.{}: invalid receiver or missing arguments",
                method_id.canonical_name()
            )));
        }

        args.iter()
            .take(expected)
            .map(|arg| self.load_as_box(*arg))
            .collect()
    }

    fn invoke_string_surface(
        &mut self,
        receiver: &VMValue,
        method_id: StringMethodId,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        let string = match receiver {
            VMValue::String(value) => CoreStringBox::new(value.clone()),
            VMValue::BoxRef(bx) if bx.type_name() == "StringBox" => bx.to_string_box(),
            _ => {
                return Err(self.err_invalid(format!(
                    "StringBox.{}: invalid receiver",
                    method_id.canonical_name()
                )));
            }
        };
        let surface_args = self.load_string_surface_args(method_id, args)?;
        let result = string
            .invoke_surface(method_id, surface_args)
            .map_err(|err| self.err_invalid(err.to_string()))?;
        Ok(match result {
            StringSurfaceInvokeResult::Value(value) => VMValue::from_nyash_box(value),
        })
    }

    fn is_duplicate_receiver_arg(
        &mut self,
        receiver: &VMValue,
        receiver_id: ValueId,
        arg: ValueId,
    ) -> bool {
        if arg == receiver_id {
            return true;
        }

        let Ok(VMValue::BoxRef(arg_bx)) = self.reg_load(arg) else {
            return false;
        };
        let VMValue::BoxRef(recv_bx) = receiver else {
            return false;
        };

        if std::sync::Arc::ptr_eq(&arg_bx, recv_bx) {
            return true;
        }

        let Some(recv_plugin) = recv_bx
            .as_any()
            .downcast_ref::<crate::runtime::plugin_loader_v2::PluginBoxV2>()
        else {
            return false;
        };
        arg_bx
            .as_any()
            .downcast_ref::<crate::runtime::plugin_loader_v2::PluginBoxV2>()
            .map(|arg_plugin| {
                arg_plugin.inner.instance_id == recv_plugin.inner.instance_id
                    && arg_plugin.box_type == recv_plugin.box_type
            })
            .unwrap_or(false)
    }

    fn strip_duplicate_receiver_arg_for_arity<'a>(
        &mut self,
        receiver: &VMValue,
        receiver_id: ValueId,
        args: &'a [ValueId],
        expected_arity: usize,
    ) -> &'a [ValueId] {
        if args.len() == expected_arity + 1
            && args
                .first()
                .copied()
                .is_some_and(|arg| self.is_duplicate_receiver_arg(receiver, receiver_id, arg))
        {
            &args[1..]
        } else {
            args
        }
    }

    fn core_surface_expected_arity(receiver: &VMValue, method: &str) -> Option<usize> {
        match receiver {
            VMValue::String(_) => StringMethodId::from_name(method).map(|id| id.arity()),
            VMValue::BoxRef(bx) if bx.as_any().downcast_ref::<ArrayBox>().is_some() => {
                ArrayMethodId::from_name(method).map(|id| id.arity())
            }
            VMValue::BoxRef(bx) if bx.as_any().downcast_ref::<MapBox>().is_some() => {
                MapMethodId::from_name(method).map(|id| id.arity())
            }
            VMValue::BoxRef(bx) if bx.type_name() == "StringBox" => {
                StringMethodId::from_name(method).map(|id| id.arity())
            }
            _ => None,
        }
    }

    pub(super) fn execute_method_callee(
        &mut self,
        box_name: &str,
        method: &str,
        receiver: &Option<ValueId>,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        if let Some(recv_id) = receiver {
            // Primary: load receiver by id. If undefined due to builder localization gap,
            // try to auto-locate the most recent `NewBox <box_name>` in the current block
            // (same fn/last_block) and use its dst as the receiver. This is a structural
            // recovery, not a by-name fallback, and keeps semantics stable for plugin boxes.
            let recv_val = match self.reg_load(*recv_id) {
                Ok(v) => v,
                Err(e) => {
                    // Attempt structured autoscan for receiver in current block
                    if let (Some(cur_fn), Some(bb)) = (self.cur_fn.clone(), self.last_block) {
                        if let Some(func) = self.functions.get(&cur_fn) {
                            if let Some(block) = func.blocks.get(&bb) {
                                let mut last_recv: Option<ValueId> = None;
                                for inst in &block.instructions {
                                    if let crate::mir::MirInstruction::NewBox {
                                        dst,
                                        box_type,
                                        ..
                                    } = inst
                                    {
                                        if box_type == box_name {
                                            last_recv = Some(*dst);
                                        }
                                    }
                                }
                                if let Some(rid) = last_recv {
                                    if let Ok(v) = self.reg_load(rid) {
                                        v
                                    } else {
                                        return Err(e);
                                    }
                                } else {
                                    // Dev fallback (guarded): use args[0] as surrogate receiver if explicitly allowed
                                    let tolerate = env::env_bool("NYASH_VM_RECV_ARG_FALLBACK")
                                        || env::env_bool("NYASH_VM_TOLERATE_VOID");
                                    if tolerate {
                                        if let Some(a0) = args.get(0) {
                                            self.reg_load(*a0)?
                                        } else {
                                            return Err(e);
                                        }
                                    } else {
                                        return Err(e);
                                    }
                                }
                            } else {
                                return Err(e);
                            }
                        } else {
                            return Err(e);
                        }
                    } else {
                        // Dev fallback (guarded): use args[0] as surrogate receiver if explicitly allowed
                        let tolerate = env::env_bool("NYASH_VM_RECV_ARG_FALLBACK")
                            || env::env_bool("NYASH_VM_TOLERATE_VOID");
                        if tolerate {
                            if let Some(a0) = args.get(0) {
                                self.reg_load(*a0)?
                            } else {
                                return Err(e);
                            }
                        } else {
                            return Err(e);
                        }
                    }
                }
            };
            let dev_trace = env::env_bool("NYASH_VM_TRACE");
            // Fast bridge for builtin boxes (Array) and common methods.
            // Preserve legacy semantics when plugins are absent.
            if let VMValue::BoxRef(bx) = &recv_val {
                // ArrayBox bridge
                if let Some(arr) = bx.as_any().downcast_ref::<ArrayBox>() {
                    if method == "birth" {
                        return Ok(VMValue::Void);
                    }
                    if let Some(method_id) = ArrayMethodId::from_name(method) {
                        let method_args = self.strip_duplicate_receiver_arg_for_arity(
                            &recv_val,
                            *recv_id,
                            args,
                            method_id.arity(),
                        );
                        return self.invoke_array_surface(arr, method_id, method_args);
                    }
                }
            }
            // Minimal bridge for birth(): delegate to BoxCall handler and return Void
            if method == "birth" {
                let method_args =
                    self.strip_duplicate_receiver_arg_for_arity(&recv_val, *recv_id, args, 0);
                let _ = self.handle_box_call(None, *recv_id, &method.to_string(), method_args)?;
                return Ok(VMValue::Void);
            }
            let is_kw = method == "keyword_to_token_type";
            let method_args =
                if let Some(expected) = Self::core_surface_expected_arity(&recv_val, method) {
                    self.strip_duplicate_receiver_arg_for_arity(&recv_val, *recv_id, args, expected)
                } else {
                    args
                };
            if dev_trace && is_kw {
                let a0 = method_args.get(0).and_then(|id| self.reg_load(*id).ok());
                get_global_ring0()
                    .log
                    .debug(&format!("[vm-trace] mcall {} argv0={:?}", method, a0));
            }
            let out = self.execute_method_call(&recv_val, method, method_args)?;
            if dev_trace && is_kw {
                get_global_ring0()
                    .log
                    .debug(&format!("[vm-trace] mret  {} -> {:?}", method, out));
            }
            Ok(out)
        } else {
            // Receiver not provided: try static singleton instance for the box (methodize PoC fallback)
            if self.static_box_registry.exists(box_name) {
                // 🎯 Phase 173-B: Static box methods are in MIR function table
                // Try calling the MIR function directly (BoxName.method/arity pattern)
                let func_name = format!("{}.{}/{}", box_name, method, args.len());
                if self.functions.contains_key(&func_name) {
                    // Call MIR function directly via execute_global_function
                    return self.execute_global_function(&func_name, args);
                }

                // Fallback: try InstanceBox method dispatch (for builtin static boxes)
                let instance = self.ensure_static_box_instance(box_name)?;
                let recv_val = VMValue::from_nyash_box(Box::new(instance.clone()));
                return self.execute_method_call(&recv_val, method, args);
            }
            Err(self.err_with_context("Method call", &format!("missing receiver for {}", method)))
        }
    }

    fn execute_method_call(
        &mut self,
        receiver: &VMValue,
        method: &str,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        // Phase 124: Unified dispatch using TypeRegistry
        // 1. Get type_name from receiver
        let type_name = match receiver {
            VMValue::String(_) => "String",
            VMValue::Integer(_) => "Integer",
            VMValue::Bool(_) => "Bool",
            VMValue::Float(_) => "Float",
            VMValue::Void => "Void",
            VMValue::Future(_) => "Future",
            VMValue::BoxRef(bx) => bx.type_name(),
            VMValue::WeakBox(_) => "WeakRef", // Phase 285A0
        };

        // 2. Lookup type in TypeRegistry and get slot
        // Note: Try exact arity first, then try with args.len()-1 (in case receiver is duplicated in args)
        let slot =
            crate::runtime::type_registry::resolve_slot_by_name(type_name, method, args.len())
                .or_else(|| {
                    // Fallback: try with one less argument (receiver might be in args)
                    if args.len() > 0 {
                        crate::runtime::type_registry::resolve_slot_by_name(
                            type_name,
                            method,
                            args.len() - 1,
                        )
                    } else {
                        None
                    }
                });

        if let Some(slot) = slot {
            // 3. Use unified dispatch
            return self.dispatch_by_slot(receiver, type_name, slot, args);
        }

        // Fallback: Special methods not in TypeRegistry yet
        if let VMValue::BoxRef(box_ref) = receiver {
            if box_ref.type_name() == "StringBox" {
                if let Some(out) = try_eval_string_char_predicate(self, method, args)? {
                    return Ok(out);
                }
            }

            // InstanceBox method bridge for canonical method-call lane.
            if box_ref
                .as_any()
                .downcast_ref::<crate::instance_v2::InstanceBox>()
                .is_some()
            {
                if let Some(out) = with_temp_receiver_dispatch(
                    self,
                    receiver,
                    TMP_RECV_INSTANCE_METHOD_BRIDGE,
                    TMP_OUT_INSTANCE_METHOD_BRIDGE,
                    |this, recv_tmp, out_tmp| {
                        super::boxes_instance::try_handle_instance_box(
                            this,
                            Some(out_tmp),
                            recv_tmp,
                            method,
                            args,
                        )
                    },
                )? {
                    return Ok(out);
                }
            }

            // Field bridge for canonical method-call lane.
            // `setField/getField` may arrive as Call(callee=Method), so reuse object-field
            // handler semantics even outside legacy BoxCall path.
            if method == "getField" || method == "setField" {
                if let Some(out) = with_temp_receiver_dispatch(
                    self,
                    receiver,
                    TMP_RECV_OBJECT_FIELD_METHOD_BRIDGE,
                    TMP_OUT_OBJECT_FIELD_METHOD_BRIDGE,
                    |this, recv_tmp, out_tmp| {
                        super::boxes_object_fields::try_handle_object_fields(
                            this,
                            Some(out_tmp),
                            recv_tmp,
                            method,
                            args,
                        )
                    },
                )? {
                    return Ok(out);
                }
            }

            // FileBox fallback (builtin/core-ro): open/read/close via shared provider dispatch.
            if let Some(out) =
                super::boxes_file::try_handle_file_box_methodcall(self, receiver, method, args)?
            {
                return Ok(out);
            }
            // BufferBox fallback: typed binary read/write helpers.
            if let Some(out) =
                super::boxes_buffer::try_handle_buffer_box_methodcall(self, receiver, method, args)?
            {
                return Ok(out);
            }
            // PathBox fallback: join/dirname/basename/extname/isAbs/normalize via provider dispatch.
            if let Some(out) =
                super::boxes_path::try_handle_path_box_methodcall(self, receiver, method, args)?
            {
                return Ok(out);
            }

            // Plugin Box fallback
            if let Some(p) = box_ref
                .as_any()
                .downcast_ref::<crate::runtime::plugin_loader_v2::PluginBoxV2>()
            {
                let host = crate::runtime::plugin_loader_unified::get_global_plugin_host();
                let host = host.read().unwrap();
                let argv = self.load_args_as_boxes(args)?;
                match host.invoke_instance_method(&p.box_type, method, p.inner.instance_id, &argv) {
                    Ok(Some(ret)) => return Ok(VMValue::from_nyash_box(ret)),
                    Ok(None) => return Ok(VMValue::Void),
                    Err(e) => {
                        return Err(self.err_with_context(
                            &format!("Plugin method {}.{}", p.box_type, method),
                            &format!("{:?}", e),
                        ))
                    }
                }
            }
        }

        // No slot found and no fallback matched. Emit a focused trace so
        // selfhost-first builder failures can be tied back to the current fn.
        if crate::config::env::vm_trace_enabled() {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[vm-trace] unknown-method type={} method={} argc={} fn={:?} last_block={:?} last_inst={:?}",
                type_name,
                method,
                args.len(),
                self.cur_fn,
                self.last_block,
                self.last_inst
            ));
        }
        Err(self.err_method_not_found(type_name, method))
    }
}
