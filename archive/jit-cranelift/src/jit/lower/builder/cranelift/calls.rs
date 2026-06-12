use super::super::tls::tls_call_import_ret;
use super::super::{BinOpKind, CmpKind, IRBuilder, ParamKind};
use super::CraneliftBuilder;

impl IRBuilder for CraneliftBuilder {
    fn emit_host_call(&mut self, symbol: &str, _argc: usize, has_ret: bool) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        // Structured lower event for import call
        {
            let mut arg_types: Vec<&'static str> = Vec::new();
            for _ in 0.._argc {
                arg_types.push("I64");
            }
            crate::jit::events::emit_lower(
                serde_json::json!({
                    "id": symbol,
                    "decision": "allow",
                    "reason": "import_call",
                    "argc": _argc,
                    "arg_types": arg_types,
                    "ret": if has_ret { "I64" } else { "Void" }
                }),
                "hostcall",
                "<jit>",
            );
        }
        let call_conv = self.module.isa().default_call_conv();
        let mut sig = Signature::new(call_conv);
        // Collect up to _argc i64 values from stack (right-to-left) and pad with zeros to match arity
        let mut args: Vec<cranelift_codegen::ir::Value> = Vec::new();
        let take_n = _argc.min(self.value_stack.len());
        for _ in 0..take_n {
            if let Some(v) = self.value_stack.pop() {
                args.push(v);
            }
        }
        args.reverse();
        Self::with_fb(|fb| {
            while args.len() < _argc {
                args.push(fb.ins().iconst(types::I64, 0));
            }
        });
        for _ in 0.._argc {
            sig.params.push(AbiParam::new(types::I64));
        }

        let func_id = self
            .module
            .declare_function(symbol, cranelift_module::Linkage::Import, &sig)
            .expect("declare import failed");
        if let Some(v) = tls_call_import_ret(&mut self.module, func_id, &args, has_ret) {
            self.value_stack.push(v);
        }
    }
    fn emit_host_call_typed(
        &mut self,
        symbol: &str,
        params: &[ParamKind],
        has_ret: bool,
        ret_is_f64: bool,
    ) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        // Structured lower event for typed import call
        {
            let mut arg_types: Vec<&'static str> = Vec::new();
            for k in params {
                arg_types.push(match k {
                    ParamKind::I64 | ParamKind::B1 => "I64",
                    ParamKind::F64 => "F64",
                });
            }
            crate::jit::events::emit_lower(
                serde_json::json!({
                    "id": symbol,
                    "decision": "allow",
                    "reason": "import_call_typed",
                    "argc": params.len(),
                    "arg_types": arg_types,
                    "ret": if has_ret { if ret_is_f64 { "F64" } else { "I64" } } else { "Void" }
                }),
                "hostcall",
                "<jit>",
            );
        }
        let mut args: Vec<cranelift_codegen::ir::Value> = Vec::new();
        let take_n = params.len().min(self.value_stack.len());
        for _ in 0..take_n {
            if let Some(v) = self.value_stack.pop() {
                args.push(v);
            }
        }
        args.reverse();
        let call_conv = self.module.isa().default_call_conv();
        let mut sig = Signature::new(call_conv);
        let abi_param_for_kind = |k: &ParamKind| match k {
            ParamKind::I64 => AbiParam::new(types::I64),
            ParamKind::F64 => AbiParam::new(types::F64),
            ParamKind::B1 => AbiParam::new(types::I64),
        };
        for k in params {
            sig.params.push(abi_param_for_kind(k));
        }
        if has_ret {
            if ret_is_f64 {
                sig.returns.push(AbiParam::new(types::F64));
            } else {
                sig.returns.push(AbiParam::new(types::I64));
            }
        }
        let func_id = self
            .module
            .declare_function(symbol, cranelift_module::Linkage::Import, &sig)
            .expect("declare typed import failed");
        if let Some(v) = tls_call_import_ret(&mut self.module, func_id, &args, has_ret) {
            self.value_stack.push(v);
        }
    }
    fn emit_debug_i64_local(&mut self, tag: i64, slot: usize) {
        if std::env::var("NYASH_JIT_TRACE_LEN").ok().as_deref() != Some("1") {
            return;
        }
        use cranelift_codegen::ir::types;
        // Push tag and value
        let t = Self::with_fb(|fb| fb.ins().iconst(types::I64, tag));
        self.value_stack.push(t);
        self.load_local_i64(slot);
        // Use existing typed hostcall helper to pass two I64 args
        self.emit_host_call_typed(
            "nyash.jit.dbg_i64",
            &[ParamKind::I64, ParamKind::I64],
            true,
            false,
        );
        // Drop the returned value to keep stack balanced
        let _ = self.value_stack.pop();
    }
    fn emit_host_call_fixed3(&mut self, symbol: &str, has_ret: bool) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        let mut args: Vec<cranelift_codegen::ir::Value> = Vec::new();
        // Pop up to 3 values; pad with zeros to reach exactly 3
        let take_n = core::cmp::min(3, self.value_stack.len());
        for _ in 0..take_n {
            if let Some(v) = self.value_stack.pop() {
                args.push(v);
            }
        }
        args.reverse();
        Self::with_fb(|fb| {
            while args.len() < 3 {
                args.push(fb.ins().iconst(types::I64, 0));
            }
        });
        let call_conv = self.module.isa().default_call_conv();
        let mut sig = Signature::new(call_conv);
        for _ in 0..3 {
            sig.params.push(AbiParam::new(types::I64));
        }
        // Always declare with I64 return to keep signature stable across call sites
        sig.returns.push(AbiParam::new(types::I64));
        let func_id = self
            .module
            .declare_function(symbol, cranelift_module::Linkage::Import, &sig)
            .expect("declare import fixed3 failed");
        if let Some(v) = tls_call_import_ret(&mut self.module, func_id, &args, true) {
            if has_ret {
                self.value_stack.push(v);
            }
        }
    }
    fn emit_plugin_invoke(&mut self, type_id: u32, method_id: u32, argc: usize, has_ret: bool) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        // Pop argc values (right-to-left): receiver + up to 2 args
        let mut arg_vals: Vec<cranelift_codegen::ir::Value> = {
            let take_n = argc.min(self.value_stack.len());
            let mut tmp = Vec::new();
            for _ in 0..take_n {
                if let Some(v) = self.value_stack.pop() {
                    tmp.push(v);
                }
            }
            tmp.reverse();
            tmp
        };
        // Ensure receiver (a0) is a runtime handle via nyash.handle.of
        let a0_handle = {
            use crate::jit::r#extern::handles as h;
            let call_conv_h = self.module.isa().default_call_conv();
            let mut sig_h = Signature::new(call_conv_h);
            sig_h.params.push(AbiParam::new(types::I64));
            sig_h.returns.push(AbiParam::new(types::I64));
            let func_id_h = self
                .module
                .declare_function(h::SYM_HANDLE_OF, cranelift_module::Linkage::Import, &sig_h)
                .expect("declare handle.of failed");
            tls_call_import_ret(&mut self.module, func_id_h, &arg_vals[0..1], true)
                .expect("handle.of ret")
        };
        arg_vals[0] = a0_handle;
        // f64 shim allowed by env allowlist
        let use_f64 = if has_ret {
            if let Ok(list) = std::env::var("NYASH_JIT_PLUGIN_F64") {
                list.split(',').any(|e| { let mut it = e.split(':'); matches!((it.next(), it.next()), (Some(t), Some(m)) if t.parse::<u32>().ok()==Some(type_id) && m.parse::<u32>().ok()==Some(method_id)) })
            } else {
                false
            }
        } else {
            false
        };
        let call_conv = self.module.isa().default_call_conv();
        let mut sig = Signature::new(call_conv);
        for _ in 0..6 {
            sig.params.push(AbiParam::new(types::I64));
        }
        if has_ret {
            sig.returns
                .push(AbiParam::new(if use_f64 { types::F64 } else { types::I64 }));
        }
        let symbol = if use_f64 {
            "nyash_plugin_invoke3_f64"
        } else {
            "nyash_plugin_invoke3_i64"
        };
        let func_id = self
            .module
            .declare_function(symbol, cranelift_module::Linkage::Import, &sig)
            .expect("declare plugin shim failed");
        let ret_val = Self::with_fb(|fb| {
            if let Some(idx) = self.current_block_index {
                fb.switch_to_block(self.blocks[idx]);
            } else if let Some(b) = self.entry_block {
                fb.switch_to_block(b);
            }
            while arg_vals.len() < 3 {
                let z = fb.ins().iconst(types::I64, 0);
                arg_vals.push(z);
            }
            // handle.of on receiver (redundant-safe)
            let call_conv_h = self.module.isa().default_call_conv();
            let mut sig_h = Signature::new(call_conv_h);
            sig_h.params.push(AbiParam::new(types::I64));
            sig_h.returns.push(AbiParam::new(types::I64));
            let func_id_h = self
                .module
                .declare_function(
                    crate::jit::r#extern::handles::SYM_HANDLE_OF,
                    cranelift_module::Linkage::Import,
                    &sig_h,
                )
                .expect("declare handle.of failed");
            let fref_h = self.module.declare_func_in_func(func_id_h, fb.func);
            let call_h = fb.ins().call(fref_h, &[arg_vals[0]]);
            if let Some(rv) = fb.inst_results(call_h).get(0).copied() {
                arg_vals[0] = rv;
            }
            let fref = self.module.declare_func_in_func(func_id, fb.func);
            let c_type = fb.ins().iconst(types::I64, type_id as i64);
            let c_meth = fb.ins().iconst(types::I64, method_id as i64);
            let c_argc = fb.ins().iconst(types::I64, argc as i64);
            let call_inst = fb.ins().call(
                fref,
                &[
                    c_type,
                    c_meth,
                    c_argc,
                    arg_vals[0],
                    arg_vals[1],
                    arg_vals[2],
                ],
            );
            if has_ret {
                fb.inst_results(call_inst).get(0).copied()
            } else {
                None
            }
        });
        if let Some(v) = ret_val {
            self.value_stack.push(v);
        }
    }
    fn emit_plugin_invoke_by_name(&mut self, method: &str, argc: usize, has_ret: bool) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        // Collect call args
        let mut arg_vals: Vec<cranelift_codegen::ir::Value> = {
            let take_n = argc.min(self.value_stack.len());
            let mut tmp = Vec::new();
            for _ in 0..take_n {
                if let Some(v) = self.value_stack.pop() {
                    tmp.push(v);
                }
            }
            tmp.reverse();
            tmp
        };
        // Signature: nyash_plugin_invoke_name_*(argc, a0, a1, a2)
        let mut sig = Signature::new(self.module.isa().default_call_conv());
        sig.params.push(AbiParam::new(types::I64));
        sig.params.push(AbiParam::new(types::I64));
        sig.params.push(AbiParam::new(types::I64));
        sig.params.push(AbiParam::new(types::I64));
        sig.returns.push(AbiParam::new(types::I64));

        let sym = match method {
            "getattr" => "nyash_plugin_invoke_name_getattr_i64",
            _ => "nyash_plugin_invoke_name_call_i64",
        };
        let func_id = self
            .module
            .declare_function(sym, cranelift_module::Linkage::Import, &sig)
            .expect("declare name shim failed");
        let ret_val = Self::with_fb(|fb| {
            while arg_vals.len() < 3 {
                let z = fb.ins().iconst(types::I64, 0);
                arg_vals.push(z);
            }
            let fref = self.module.declare_func_in_func(func_id, fb.func);
            let cargc = fb.ins().iconst(types::I64, argc as i64);
            let call_inst = fb
                .ins()
                .call(fref, &[cargc, arg_vals[0], arg_vals[1], arg_vals[2]]);
            if has_ret {
                fb.inst_results(call_inst).get(0).copied()
            } else {
                None
            }
        });
        if let Some(v) = ret_val {
            self.value_stack.push(v);
        }
    }
    fn emit_string_handle_from_literal(&mut self, s: &str) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        // Pack up to 16 bytes into two u64 words (little-endian)
        let bytes = s.as_bytes();
        let mut lo: u64 = 0;
        let mut hi: u64 = 0;
        let take = core::cmp::min(16, bytes.len());
        for i in 0..take.min(8) {
            lo |= (bytes[i] as u64) << (8 * i as u32);
        }
        for i in 8..take {
            hi |= (bytes[i] as u64) << (8 * (i - 8) as u32);
        }
        // Call thunk: nyash.string.from_u64x2(lo, hi, len) -> handle(i64)
        let call_conv = self.module.isa().default_call_conv();
        let mut sig = Signature::new(call_conv);
        sig.params.push(AbiParam::new(types::I64)); // lo
        sig.params.push(AbiParam::new(types::I64)); // hi
        sig.params.push(AbiParam::new(types::I64)); // len
        sig.returns.push(AbiParam::new(types::I64));
        let func_id = self
            .module
            .declare_function(
                crate::jit::r#extern::collections::SYM_STRING_FROM_U64X2,
                cranelift_module::Linkage::Import,
                &sig,
            )
            .expect("declare string.from_u64x2");
        let v = Self::with_fb(|fb| {
            let lo_v = fb.ins().iconst(types::I64, lo as i64);
            let hi_v = fb.ins().iconst(types::I64, hi as i64);
            let len_v = fb.ins().iconst(types::I64, bytes.len() as i64);
            let fref = self.module.declare_func_in_func(func_id, fb.func);
            let call_inst = fb.ins().call(fref, &[lo_v, hi_v, len_v]);
            fb.inst_results(call_inst)
                .get(0)
                .copied()
                .expect("str.from_ptr ret")
        });
        self.value_stack.push(v);
        self.stats.0 += 1;
    }
}
