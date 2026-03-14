use super::*;
use crate::backend::mir_interpreter::handlers::string_method_helpers::{
    parse_index_of_args, parse_last_index_of_args, parse_substring_args, ArgParsePolicy,
};
use crate::backend::mir_interpreter::handlers::temp_dispatch::{
    with_temp_receiver_dispatch, TMP_OUT_INSTANCE_FIELD_OP, TMP_OUT_INSTANCE_METHOD_BRIDGE,
    TMP_OUT_OBJECT_FIELD_METHOD_BRIDGE, TMP_RECV_INSTANCE_FIELD_OP,
    TMP_RECV_INSTANCE_METHOD_BRIDGE, TMP_RECV_OBJECT_FIELD_METHOD_BRIDGE,
};
use crate::boxes::string_ops;
use crate::config::env;
use crate::runtime::get_global_ring0;

impl MirInterpreter {
    fn normalize_plugin_method_args<'a>(
        &mut self,
        receiver: &VMValue,
        args: &'a [ValueId],
    ) -> &'a [ValueId] {
        let Some((recv_box_type, recv_instance_id)) = (match receiver {
            VMValue::BoxRef(bx) => bx
                .as_any()
                .downcast_ref::<crate::runtime::plugin_loader_v2::PluginBoxV2>()
                .map(|p| (p.box_type.as_str(), p.inner.instance_id)),
            _ => None,
        }) else {
            return args;
        };

        let Some(first_arg) = args.first() else {
            return args;
        };

        let is_duplicate_receiver = match self.reg_load(*first_arg) {
            Ok(VMValue::BoxRef(arg_bx)) => arg_bx
                .as_any()
                .downcast_ref::<crate::runtime::plugin_loader_v2::PluginBoxV2>()
                .map(|p| p.inner.instance_id == recv_instance_id && p.box_type == recv_box_type)
                .unwrap_or(false),
            _ => false,
        };

        if is_duplicate_receiver {
            &args[1..]
        } else {
            args
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
                if let Some(arr) = bx.as_any().downcast_ref::<crate::boxes::array::ArrayBox>() {
                    match method {
                        "birth" => {
                            return Ok(VMValue::Void);
                        }
                        "push" => {
                            if let Some(a0) = args.get(0) {
                                let v = self.load_as_box(*a0)?;
                                let _ = arr.push(v);
                                return Ok(VMValue::Void);
                            }
                        }
                        "len" | "length" | "size" => {
                            let ret = arr.length();
                            return Ok(VMValue::from_nyash_box(ret));
                        }
                        "get" => {
                            if let Some(a0) = args.get(0) {
                                let idx = self.load_as_box(*a0)?;
                                let ret = arr.get(idx);
                                return Ok(VMValue::from_nyash_box(ret));
                            }
                        }
                        "set" => {
                            if args.len() >= 2 {
                                let idx = self.load_as_box(args[0])?;
                                let val = self.load_as_box(args[1])?;
                                let _ = arr.set(idx, val);
                                return Ok(VMValue::Void);
                            }
                        }
                        _ => {}
                    }
                }
            }
            // Minimal bridge for birth(): delegate to BoxCall handler and return Void
            if method == "birth" {
                let _ = self.handle_box_call(None, *recv_id, &method.to_string(), args)?;
                return Ok(VMValue::Void);
            }
            let is_kw = method == "keyword_to_token_type";
            if dev_trace && is_kw {
                let a0 = args.get(0).and_then(|id| self.reg_load(*id).ok());
                get_global_ring0()
                    .log
                    .debug(&format!("[vm-trace] mcall {} argv0={:?}", method, a0));
            }
            let method_args = self.normalize_plugin_method_args(&recv_val, args);
            let out = self.execute_method_call(&recv_val, method, method_args)?;
            if dev_trace && is_kw {
                get_global_ring0()
                    .log
                    .debug(&format!("[vm-trace] mret  {} -> {:?}", method, out));
            }
            Ok(out)
        } else {
            if box_name == "hostbridge" && method == "extern_invoke" {
                return self.execute_extern_function("hostbridge.extern_invoke", args);
            }
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

    /// Phase 124: Unified dispatch using TypeRegistry slot numbers
    /// This function replaces the old pattern-matching dispatch with a slot-based approach
    fn dispatch_by_slot(
        &mut self,
        receiver: &VMValue,
        type_name: &str,
        slot: u16,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        match (type_name, slot) {
            // String methods (slot 300+)
            ("String", 300) => {
                // length
                if let VMValue::String(s) = receiver {
                    Ok(VMValue::Integer(s.len() as i64))
                } else {
                    Err(self.err_invalid("String.length: invalid receiver"))
                }
            }
            ("String", 302) => {
                // concat
                if let VMValue::String(s) = receiver {
                    if let Some(arg_id) = args.get(0) {
                        let arg_val = self.reg_load(*arg_id)?;
                        let new_str = format!("{}{}", s, arg_val.to_string());
                        Ok(VMValue::String(new_str))
                    } else {
                        Err(self.err_invalid("String.concat: requires 1 argument"))
                    }
                } else {
                    Err(self.err_invalid("String.concat: invalid receiver"))
                }
            }
            ("String", 304) => {
                // replace
                if let VMValue::String(s) = receiver {
                    if args.len() == 2 {
                        let old = self.reg_load(args[0])?.to_string();
                        let new = self.reg_load(args[1])?.to_string();
                        Ok(VMValue::String(s.replace(&old, &new)))
                    } else {
                        Err(self.err_invalid("String.replace: requires 2 arguments"))
                    }
                } else {
                    Err(self.err_invalid("String.replace: invalid receiver"))
                }
            }
            ("String", 305) => {
                // trim
                if let VMValue::String(s) = receiver {
                    if args.is_empty() {
                        Ok(VMValue::String(s.trim().to_string()))
                    } else {
                        Err(self.err_invalid("String.trim: requires 0 arguments"))
                    }
                } else {
                    Err(self.err_invalid("String.trim: invalid receiver"))
                }
            }
            ("String", 303) => {
                // indexOf
                if let VMValue::String(s) = receiver {
                    let (needle, start) = parse_index_of_args(
                        self,
                        args,
                        ArgParsePolicy::LENIENT,
                        "String.indexOf: requires 1 argument",
                    )?;
                    let mode = string_ops::index_mode_from_env();
                    let idx = string_ops::index_of(s, &needle, start, mode);
                    Ok(VMValue::Integer(idx))
                } else {
                    Err(self.err_invalid("String.indexOf: invalid receiver"))
                }
            }
            ("String", 308) => {
                // lastIndexOf
                if let VMValue::String(s) = receiver {
                    let needle = parse_last_index_of_args(
                        self,
                        args,
                        ArgParsePolicy::LENIENT,
                        "String.lastIndexOf: requires 1 argument",
                    )?;
                    let mode = string_ops::index_mode_from_env();
                    let idx = string_ops::last_index_of(s, &needle, mode);
                    Ok(VMValue::Integer(idx))
                } else {
                    Err(self.err_invalid("String.lastIndexOf: invalid receiver"))
                }
            }
            ("String", 309) => {
                // contains
                if let VMValue::String(s) = receiver {
                    if args.len() == 1 {
                        let needle = self.reg_load(args[0])?.to_string();
                        Ok(VMValue::Bool(s.contains(&needle)))
                    } else {
                        Err(self.err_invalid("String.contains: requires 1 argument"))
                    }
                } else {
                    Err(self.err_invalid("String.contains: invalid receiver"))
                }
            }
            ("String", 301) => {
                // substring
                if let VMValue::String(s) = receiver {
                    let (start, end) = parse_substring_args(
                        self,
                        args,
                        ArgParsePolicy::LENIENT,
                        "String.substring: requires 1 or 2 arguments",
                    )?;
                    let mode = string_ops::index_mode_from_env();
                    let sub = string_ops::substring(s, start, end, mode);
                    Ok(VMValue::String(sub))
                } else {
                    Err(self.err_invalid("String.substring: invalid receiver"))
                }
            }

            // InstanceBox methods (slot 1..4)
            ("InstanceBox", 1) => {
                // getField(name)
                if let Some(out) = with_temp_receiver_dispatch(
                    self,
                    receiver,
                    TMP_RECV_INSTANCE_FIELD_OP,
                    TMP_OUT_INSTANCE_FIELD_OP,
                    |this, recv_tmp, out_tmp| {
                        super::boxes_object_fields::try_handle_object_fields(
                            this,
                            Some(out_tmp),
                            recv_tmp,
                            "getField",
                            args,
                        )
                    },
                )? {
                    return Ok(out);
                }
                Err(self.err_invalid("InstanceBox.getField: unsupported receiver"))
            }
            ("InstanceBox", 2) => {
                // setField(name, value)
                if let Some(out) = with_temp_receiver_dispatch(
                    self,
                    receiver,
                    TMP_RECV_INSTANCE_FIELD_OP,
                    TMP_OUT_INSTANCE_FIELD_OP,
                    |this, recv_tmp, out_tmp| {
                        super::boxes_object_fields::try_handle_object_fields(
                            this,
                            Some(out_tmp),
                            recv_tmp,
                            "setField",
                            args,
                        )
                    },
                )? {
                    return Ok(out);
                }
                Err(self.err_invalid("InstanceBox.setField: unsupported receiver"))
            }
            ("InstanceBox", 3) => {
                // has(name) -> bool
                let out = with_temp_receiver_dispatch(
                    self,
                    receiver,
                    TMP_RECV_INSTANCE_FIELD_OP,
                    TMP_OUT_INSTANCE_FIELD_OP,
                    |this, recv_tmp, out_tmp| {
                        super::boxes_object_fields::try_handle_object_fields(
                            this,
                            Some(out_tmp),
                            recv_tmp,
                            "getField",
                            args,
                        )
                    },
                )?;
                let exists = out.is_some_and(|v| !matches!(v, VMValue::Void));
                Ok(VMValue::Bool(exists))
            }
            ("InstanceBox", 4) => {
                // size()
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(inst) = bx
                        .as_any()
                        .downcast_ref::<crate::instance_v2::InstanceBox>()
                    {
                        let size = inst.get_fields().lock().unwrap().len() as i64;
                        return Ok(VMValue::Integer(size));
                    }
                }
                Err(self.err_invalid("InstanceBox.size: invalid receiver"))
            }

            // ArrayBox methods (slot 100+)
            ("ArrayBox", 100) => {
                // get
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(arr) = bx.as_any().downcast_ref::<crate::boxes::array::ArrayBox>() {
                        if let Some(a0) = args.get(0) {
                            let idx = self.load_as_box(*a0)?;
                            let ret = arr.get(idx);
                            return Ok(VMValue::from_nyash_box(ret));
                        }
                    }
                }
                Err(self.err_invalid("ArrayBox.get: invalid receiver or missing argument"))
            }
            ("ArrayBox", 101) => {
                // set
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(arr) = bx.as_any().downcast_ref::<crate::boxes::array::ArrayBox>() {
                        if args.len() >= 2 {
                            let idx = self.load_as_box(args[0])?;
                            let val = self.load_as_box(args[1])?;
                            let _ = arr.set(idx, val);
                            return Ok(VMValue::Void);
                        }
                    }
                }
                Err(self.err_invalid("ArrayBox.set: invalid receiver or missing arguments"))
            }
            ("ArrayBox", 102) => {
                // len/length
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(arr) = bx.as_any().downcast_ref::<crate::boxes::array::ArrayBox>() {
                        let ret = arr.length();
                        return Ok(VMValue::from_nyash_box(ret));
                    }
                }
                Err(self.err_invalid("ArrayBox.length: invalid receiver"))
            }
            ("ArrayBox", 103) => {
                // push
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(arr) = bx.as_any().downcast_ref::<crate::boxes::array::ArrayBox>() {
                        if let Some(a0) = args.get(0) {
                            let v = self.load_as_box(*a0)?;
                            let _ = arr.push(v);
                            return Ok(VMValue::Void);
                        }
                    }
                }
                Err(self.err_invalid("ArrayBox.push: invalid receiver or missing argument"))
            }

            // MapBox methods (slot 200+)
            ("MapBox", 200) | ("MapBox", 201) => {
                // size/len
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(map) = bx.as_any().downcast_ref::<crate::boxes::map_box::MapBox>() {
                        let ret = map.size();
                        return Ok(VMValue::from_nyash_box(ret));
                    }
                }
                Err(self.err_invalid("MapBox.size/len: invalid receiver"))
            }
            ("MapBox", 202) => {
                // has
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(map) = bx.as_any().downcast_ref::<crate::boxes::map_box::MapBox>() {
                        if let Some(a0) = args.get(0) {
                            let key = self.load_as_box(*a0)?;
                            let ret = map.has(key);
                            return Ok(VMValue::from_nyash_box(ret));
                        }
                    }
                }
                Err(self.err_invalid("MapBox.has: invalid receiver or missing argument"))
            }
            ("MapBox", 203) => {
                // get
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(map) = bx.as_any().downcast_ref::<crate::boxes::map_box::MapBox>() {
                        if let Some(a0) = args.get(0) {
                            let key = self.load_as_box(*a0)?;
                            let ret = map.get(key);
                            return Ok(VMValue::from_nyash_box(ret));
                        }
                    }
                }
                Err(self.err_invalid("MapBox.get: invalid receiver or missing argument"))
            }
            ("MapBox", 204) => {
                // set
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(map) = bx.as_any().downcast_ref::<crate::boxes::map_box::MapBox>() {
                        if args.len() >= 2 {
                            let key = self.load_as_box(args[0])?;
                            let value = self.load_as_box(args[1])?;
                            let ret = map.set(key, value);
                            return Ok(VMValue::from_nyash_box(ret));
                        }
                    }
                }
                Err(self.err_invalid("MapBox.set: invalid receiver or missing arguments"))
            }
            ("MapBox", 205) => {
                // delete/remove
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(map) = bx.as_any().downcast_ref::<crate::boxes::map_box::MapBox>() {
                        if let Some(a0) = args.get(0) {
                            let key = self.load_as_box(*a0)?;
                            let ret = map.delete(key);
                            return Ok(VMValue::from_nyash_box(ret));
                        }
                    }
                }
                Err(self.err_invalid("MapBox.delete/remove: invalid receiver or missing argument"))
            }
            ("MapBox", 206) => {
                // keys
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(map) = bx.as_any().downcast_ref::<crate::boxes::map_box::MapBox>() {
                        let ret = map.keys();
                        return Ok(VMValue::from_nyash_box(ret));
                    }
                }
                Err(self.err_invalid("MapBox.keys: invalid receiver"))
            }
            ("MapBox", 207) => {
                // values
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(map) = bx.as_any().downcast_ref::<crate::boxes::map_box::MapBox>() {
                        let ret = map.values();
                        return Ok(VMValue::from_nyash_box(ret));
                    }
                }
                Err(self.err_invalid("MapBox.values: invalid receiver"))
            }
            ("MapBox", 208) => {
                // clear
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(map) = bx.as_any().downcast_ref::<crate::boxes::map_box::MapBox>() {
                        let ret = map.clear();
                        return Ok(VMValue::from_nyash_box(ret));
                    }
                }
                Err(self.err_invalid("MapBox.clear: invalid receiver"))
            }

            // ConsoleBox methods (slot 400+)
            ("ConsoleBox", 400) => {
                // log/println
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(console) = bx
                        .as_any()
                        .downcast_ref::<crate::boxes::console_box::ConsoleBox>()
                    {
                        let message = if args.len() > 1 {
                            self.reg_load(args[1])?.to_string()
                        } else if args.len() > 0 {
                            self.reg_load(args[0])?.to_string()
                        } else {
                            return Err(self.err_invalid("ConsoleBox.log: requires 1 argument"));
                        };
                        console.log(&message);
                        return Ok(VMValue::Void);
                    }
                }
                Err(self.err_invalid("ConsoleBox.log: invalid receiver"))
            }
            ("ConsoleBox", 401) => {
                // warn
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(console) = bx
                        .as_any()
                        .downcast_ref::<crate::boxes::console_box::ConsoleBox>()
                    {
                        let message = if args.len() > 1 {
                            self.reg_load(args[1])?.to_string()
                        } else if args.len() > 0 {
                            self.reg_load(args[0])?.to_string()
                        } else {
                            return Err(self.err_invalid("ConsoleBox.warn: requires 1 argument"));
                        };
                        console.warn(&message);
                        return Ok(VMValue::Void);
                    }
                }
                Err(self.err_invalid("ConsoleBox.warn: invalid receiver"))
            }
            ("ConsoleBox", 402) => {
                // error
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(console) = bx
                        .as_any()
                        .downcast_ref::<crate::boxes::console_box::ConsoleBox>()
                    {
                        let message = if args.len() > 1 {
                            self.reg_load(args[1])?.to_string()
                        } else if args.len() > 0 {
                            self.reg_load(args[0])?.to_string()
                        } else {
                            return Err(self.err_invalid("ConsoleBox.error: requires 1 argument"));
                        };
                        console.error(&message);
                        return Ok(VMValue::Void);
                    }
                }
                Err(self.err_invalid("ConsoleBox.error: invalid receiver"))
            }
            ("ConsoleBox", 403) => {
                // clear
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(console) = bx
                        .as_any()
                        .downcast_ref::<crate::boxes::console_box::ConsoleBox>()
                    {
                        console.clear();
                        return Ok(VMValue::Void);
                    }
                }
                Err(self.err_invalid("ConsoleBox.clear: invalid receiver"))
            }

            // StringBox methods (slot 300+, overlaps with String primitive)
            ("StringBox", 308) => {
                // lastIndexOf
                if let VMValue::BoxRef(bx) = receiver {
                    let s_box = bx.to_string_box();
                    let s = s_box.value;
                    let needle = parse_last_index_of_args(
                        self,
                        args,
                        ArgParsePolicy::STRICT,
                        "StringBox.lastIndexOf: requires 1 argument",
                    )?;
                    let helper = crate::boxes::string_box::StringBox::new(s);
                    let result_box = helper.lastIndexOf(&needle);
                    return Ok(VMValue::from_nyash_box(result_box));
                }
                Err(self.err_invalid("StringBox.lastIndexOf: requires 1 argument"))
            }
            ("StringBox", 303) => {
                // indexOf/find
                if let VMValue::BoxRef(bx) = receiver {
                    let s_box = bx.to_string_box();
                    let s = s_box.value;
                    let helper = crate::boxes::string_box::StringBox::new(s);
                    let (needle, start) = parse_index_of_args(
                        self,
                        args,
                        ArgParsePolicy::STRICT,
                        "StringBox.indexOf: requires 1 or 2 arguments",
                    )?;
                    let result_box = match start {
                        Some(start) => helper.find_from(&needle, start),
                        None => helper.find(&needle),
                    };
                    return Ok(VMValue::from_nyash_box(result_box));
                }
                Err(self.err_invalid("StringBox.indexOf: requires 1 or 2 arguments"))
            }
            ("StringBox", 309) => {
                // contains
                if let VMValue::BoxRef(bx) = receiver {
                    if args.len() != 1 {
                        return Err(self.err_invalid("StringBox.contains: requires 1 argument"));
                    }
                    let needle = self.reg_load(args[0])?.to_string();
                    let s = bx.to_string_box().value;
                    return Ok(VMValue::Bool(s.contains(&needle)));
                }
                Err(self.err_invalid("StringBox.contains: invalid receiver"))
            }
            ("StringBox", 305) => {
                // trim
                if let VMValue::BoxRef(bx) = receiver {
                    if args.is_empty() {
                        let trimmed = bx.to_string_box().value.trim().to_string();
                        return Ok(VMValue::String(trimmed));
                    }
                    return Err(self.err_invalid("StringBox.trim: requires 0 arguments"));
                }
                Err(self.err_invalid("StringBox.trim: invalid receiver"))
            }

            // Plugin Box methods (slot >= 1000)
            (_, slot) if slot >= 1000 => {
                if let VMValue::BoxRef(bx) = receiver {
                    if let Some(_p) = bx
                        .as_any()
                        .downcast_ref::<crate::runtime::plugin_loader_v2::PluginBoxV2>()
                    {
                        let host = crate::runtime::plugin_loader_unified::get_global_plugin_host();
                        let _host = host.read().unwrap();
                        let _argv = self.load_args_as_boxes(args)?;
                        // Get method name from slot (reverse lookup would be needed in production)
                        // For now, fall back to old path
                        return Err(self.err_with_context(
                            "Plugin dispatch",
                            &format!("slot {} not yet implemented for plugin boxes", slot),
                        ));
                    }
                }
                Err(self.err_invalid(&format!("Plugin method slot {}: invalid receiver", slot)))
            }

            _ => Err(self.err_with_context(
                "dispatch_by_slot",
                &format!("Unknown type/slot combination: {} slot {}", type_name, slot),
            )),
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

        // Canonical alias map for slot dispatch.
        // Keep fallback surface minimal by normalizing aliases before lookup.
        let canonical_method = if matches!(type_name, "String" | "StringBox") && method == "find" {
            "indexOf"
        } else {
            method
        };

        // 2. Lookup type in TypeRegistry and get slot
        // Note: Try exact arity first, then try with args.len()-1 (in case receiver is duplicated in args)
        let slot = crate::runtime::type_registry::resolve_slot_by_name(
            type_name,
            canonical_method,
            args.len(),
        )
        .or_else(|| {
            // Fallback: try with one less argument (receiver might be in args)
            if args.len() > 0 {
                crate::runtime::type_registry::resolve_slot_by_name(
                    type_name,
                    canonical_method,
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
            // StringBox special methods (is_space, is_alpha)
            if box_ref.type_name() == "StringBox" {
                let s_box = box_ref.to_string_box();
                let _s = s_box.value;
                match method {
                    "is_space" => {
                        if let Some(arg_id) = args.get(0) {
                            let ch = self.reg_load(*arg_id)?.to_string();
                            let is_ws = ch == " " || ch == "\t" || ch == "\n" || ch == "\r";
                            return Ok(VMValue::Bool(is_ws));
                        } else {
                            return Err(self.err_invalid("is_space requires 1 argument"));
                        }
                    }
                    "is_alpha" => {
                        if let Some(arg_id) = args.get(0) {
                            let ch = self.reg_load(*arg_id)?.to_string();
                            let c = ch.chars().next().unwrap_or('\0');
                            let is_alpha =
                                ('A'..='Z').contains(&c) || ('a'..='z').contains(&c) || c == '_';
                            return Ok(VMValue::Bool(is_alpha));
                        } else {
                            return Err(self.err_invalid("is_alpha requires 1 argument"));
                        }
                    }
                    _ => {}
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

        // No slot found and no fallback matched
        Err(self.err_method_not_found(type_name, method))
    }
}
