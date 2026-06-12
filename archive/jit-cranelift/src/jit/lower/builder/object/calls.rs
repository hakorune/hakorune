use super::super::{IRBuilder, ParamKind};
use super::{ObjectBuilder, ValueTag};

impl IRBuilder for ObjectBuilder {
    fn emit_host_call(&mut self, symbol: &str, argc: usize, has_ret: bool) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        use cranelift_frontend::FunctionBuilder;
        // Structured lower event for import call (AOT builder)
        {
            let mut arg_types: Vec<&'static str> = Vec::new();
            for _ in 0..argc {
                arg_types.push("I64");
            }
            crate::jit::events::emit_lower(
                serde_json::json!({
                    "id": symbol,
                    "decision": "allow",
                    "reason": "import_call",
                    "argc": argc,
                    "arg_types": arg_types,
                    "ret": if has_ret { "I64" } else { "Void" }
                }),
                "hostcall",
                "<aot>",
            );
        }
        let mut fb = FunctionBuilder::new(&mut self.ctx.func, &mut self.fbc);
        if let Some(idx) = self.current_block_index {
            fb.switch_to_block(self.blocks[idx]);
        } else if let Some(b) = self.entry_block {
            fb.switch_to_block(b);
        }
        let mut sig = Signature::new(self.module.isa().default_call_conv());
        for _ in 0..argc {
            sig.params.push(AbiParam::new(types::I64));
        }
        if has_ret {
            sig.returns.push(AbiParam::new(types::I64));
        }
        let func_id = self
            .module
            .declare_function(symbol, cranelift_module::Linkage::Import, &sig)
            .expect("declare hostcall");
        let mut args: Vec<cranelift_codegen::ir::Value> = Vec::with_capacity(argc);
        for _ in 0..argc {
            if let Some(v) = self.value_stack.pop() {
                args.push(v);
            } else {
                args.push(fb.ins().iconst(types::I64, 0));
            }
        }
        args.reverse();
        // Ensure i64 for all
        for a in args.iter_mut() {
            if fb.func.dfg.value_type(*a) != types::I64 {
                *a = fb.ins().fcvt_to_sint(types::I64, *a);
            }
        }
        let fref = self.module.declare_func_in_func(func_id, fb.func);
        let call_inst = fb.ins().call(fref, &args);
        if has_ret {
            if let Some(v) = fb.inst_results(call_inst).get(0).copied() {
                self.value_stack.push(v);
            }
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
        use cranelift_frontend::FunctionBuilder;
        // Structured lower event for typed import call (AOT builder)
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
                "<aot>",
            );
        }
        let mut fb = FunctionBuilder::new(&mut self.ctx.func, &mut self.fbc);
        if let Some(idx) = self.current_block_index {
            fb.switch_to_block(self.blocks[idx]);
        } else if let Some(b) = self.entry_block {
            fb.switch_to_block(b);
        }
        let mut sig = Signature::new(self.module.isa().default_call_conv());
        for &k in params {
            match k {
                ParamKind::I64 => sig.params.push(AbiParam::new(types::I64)),
                ParamKind::F64 => sig.params.push(AbiParam::new(types::F64)),
                ParamKind::B1 => sig.params.push(AbiParam::new(types::I64)),
            }
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
            .expect("declare hostcall typed");
        // Gather args from stack (reverse)
        let mut args: Vec<cranelift_codegen::ir::Value> = Vec::with_capacity(params.len());
        for &k in params.iter().rev() {
            let mut v = if let Some(v) = self.value_stack.pop() {
                v
            } else {
                match k {
                    ParamKind::I64 | ParamKind::B1 => fb.ins().iconst(types::I64, 0),
                    ParamKind::F64 => fb.ins().f64const(0.0),
                }
            };
            // Coerce
            v = match k {
                ParamKind::I64 | ParamKind::B1 => {
                    if fb.func.dfg.value_type(v) != types::I64 {
                        fb.ins().fcvt_to_sint(types::I64, v)
                    } else {
                        v
                    }
                }
                ParamKind::F64 => {
                    if fb.func.dfg.value_type(v) != types::F64 {
                        fb.ins().fcvt_from_sint(types::F64, v)
                    } else {
                        v
                    }
                }
            };
            args.push(v);
        }
        args.reverse();
        let fref = self.module.declare_func_in_func(func_id, fb.func);
        let call_inst = fb.ins().call(fref, &args);
        if has_ret {
            if let Some(mut v) = fb.inst_results(call_inst).get(0).copied() {
                if ret_is_f64 && fb.func.dfg.value_type(v) != types::F64 {
                    v = fb.ins().fcvt_from_sint(types::F64, v);
                }
                if !ret_is_f64 && fb.func.dfg.value_type(v) != types::I64 {
                    v = fb.ins().fcvt_to_sint(types::I64, v);
                }
                self.value_stack.push(v);
            }
        }
    }

    fn emit_host_call_fixed3(&mut self, symbol: &str, has_ret: bool) {
        self.emit_host_call(symbol, 3, has_ret);
    }

    fn emit_string_handle_from_literal(&mut self, s: &str) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        use cranelift_frontend::FunctionBuilder;
        // Pack up to 16 bytes of the literal into two u64 words
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
        let mut fb = FunctionBuilder::new(&mut self.ctx.func, &mut self.fbc);
        if let Some(idx) = self.current_block_index {
            fb.switch_to_block(self.blocks[idx]);
        } else if let Some(b) = self.entry_block {
            fb.switch_to_block(b);
        }
        // Declare import: nyash.string.from_u64x2(lo, hi, len) -> i64
        let mut sig = Signature::new(self.module.isa().default_call_conv());
        sig.params.push(AbiParam::new(types::I64));
        sig.params.push(AbiParam::new(types::I64));
        sig.params.push(AbiParam::new(types::I64));
        sig.returns.push(AbiParam::new(types::I64));
        let func_id = self
            .module
            .declare_function(
                crate::jit::r#extern::collections::SYM_STRING_FROM_U64X2,
                cranelift_module::Linkage::Import,
                &sig,
            )
            .expect("declare string.from_u64x2");
        let lo_v = fb.ins().iconst(types::I64, lo as i64);
        let hi_v = fb.ins().iconst(types::I64, hi as i64);
        let len_v = fb.ins().iconst(types::I64, bytes.len() as i64);
        let fref = self.module.declare_func_in_func(func_id, fb.func);
        let call_inst = fb.ins().call(fref, &[lo_v, hi_v, len_v]);
        if let Some(v) = fb.inst_results(call_inst).get(0).copied() {
            self.value_stack.push(v);
            self.value_tags.push(ValueTag::Handle);
        }
    }

    fn br_if_with_args(
        &mut self,
        then_index: usize,
        else_index: usize,
        then_n: usize,
        else_n: usize,
    ) {
        use cranelift_codegen::ir::{condcodes::IntCC, types};
        use cranelift_frontend::FunctionBuilder;
        if then_index >= self.blocks.len() || else_index >= self.blocks.len() {
            return;
        }
        let mut fb = FunctionBuilder::new(&mut self.ctx.func, &mut self.fbc);
        if let Some(idx) = self.current_block_index {
            fb.switch_to_block(self.blocks[idx]);
        } else if let Some(b) = self.entry_block {
            fb.switch_to_block(b);
        }
        // Pop else args, then then args (stack topに近い方から)
        let mut else_args: Vec<cranelift_codegen::ir::Value> = Vec::new();
        for _ in 0..else_n {
            if let Some(v) = self.value_stack.pop() {
                else_args.push(v);
                let _ = self.value_tags.pop();
            } else {
                else_args.push(fb.ins().iconst(types::I64, 0));
            }
        }
        else_args.reverse();
        let mut then_args: Vec<cranelift_codegen::ir::Value> = Vec::new();
        for _ in 0..then_n {
            if let Some(v) = self.value_stack.pop() {
                then_args.push(v);
                let _ = self.value_tags.pop();
            } else {
                then_args.push(fb.ins().iconst(types::I64, 0));
            }
        }
        then_args.reverse();
        // Cond
        let cond_val = if let Some(v) = self.value_stack.pop() {
            v
        } else {
            fb.ins().iconst(types::I64, 0)
        };
        let b1 = if fb.func.dfg.value_type(cond_val) == types::I64 {
            fb.ins().icmp_imm(IntCC::NotEqual, cond_val, 0)
        } else {
            fb.ins().icmp_imm(IntCC::NotEqual, cond_val, 0)
        };
        // Coerce args to i64
        for a in then_args.iter_mut() {
            if fb.func.dfg.value_type(*a) != types::I64 {
                *a = fb.ins().fcvt_to_sint(types::I64, *a);
            }
        }
        for a in else_args.iter_mut() {
            if fb.func.dfg.value_type(*a) != types::I64 {
                *a = fb.ins().fcvt_to_sint(types::I64, *a);
            }
        }
        fb.ins().brif(
            b1,
            self.blocks[then_index],
            &then_args,
            self.blocks[else_index],
            &else_args,
        );
        self.stats.3 += 1;
    }

    fn jump_with_args(&mut self, target_index: usize, n: usize) {
        use cranelift_codegen::ir::types;
        use cranelift_frontend::FunctionBuilder;
        if target_index >= self.blocks.len() {
            return;
        }
        let mut fb = FunctionBuilder::new(&mut self.ctx.func, &mut self.fbc);
        if let Some(idx) = self.current_block_index {
            fb.switch_to_block(self.blocks[idx]);
        } else if let Some(b) = self.entry_block {
            fb.switch_to_block(b);
        }
        let mut args: Vec<cranelift_codegen::ir::Value> = Vec::new();
        for _ in 0..n {
            if let Some(v) = self.value_stack.pop() {
                args.push(v);
                let _ = self.value_tags.pop();
            } else {
                args.push(fb.ins().iconst(types::I64, 0));
            }
        }
        args.reverse();
        for a in args.iter_mut() {
            if fb.func.dfg.value_type(*a) != types::I64 {
                *a = fb.ins().fcvt_to_sint(types::I64, *a);
            }
        }
        fb.ins().jump(self.blocks[target_index], &args);
        self.stats.3 += 1;
    }

    fn emit_plugin_invoke(&mut self, _type_id: u32, _method_id: u32, argc: usize, has_ret: bool) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        use cranelift_frontend::FunctionBuilder;
        // We import NyRT tagged invoke entry (by-id). Signature:
        // nyash_plugin_invoke3_tagged_i64(type_id, method_id, argc, a0, a1, tag1, a2, tag2, a3, tag3, a4, tag4) -> i64
        let mut fb = FunctionBuilder::new(&mut self.ctx.func, &mut self.fbc);
        if let Some(idx) = self.current_block_index {
            fb.switch_to_block(self.blocks[idx]);
        } else if let Some(b) = self.entry_block {
            fb.switch_to_block(b);
        }

        // Pop args in reverse: last pushed is top. Collect up to 4 (excluding recv)
        let mut arg_vals: Vec<cranelift_codegen::ir::Value> = Vec::new();
        let mut arg_tags: Vec<ValueTag> = Vec::new();
        for _ in 0..argc.saturating_sub(1) {
            // exclude receiver (first param)
            if let Some(v) = self.value_stack.pop() {
                arg_vals.push(v);
                arg_tags.push(self.value_tags.pop().unwrap_or(ValueTag::Unknown));
            }
        }
        // Receiver
        let recv = if let Some(v) = self.value_stack.pop() {
            let _ = self.value_tags.pop();
            v
        } else {
            fb.ins().iconst(types::I64, 0)
        };
        arg_vals.reverse();
        arg_tags.reverse();
        let mut tag_i64 = |t: ValueTag| -> i64 {
            match t {
                ValueTag::Handle => 8,
                ValueTag::F64 => 5,
                ValueTag::I64 => 3,
                ValueTag::Unknown => 3,
            }
        };

        // Build signature and declare import
        let mut sig = Signature::new(self.module.isa().default_call_conv());
        for _ in 0..12 {
            sig.params.push(AbiParam::new(types::I64));
        }
        sig.returns.push(AbiParam::new(types::I64));
        let func_id = self
            .module
            .declare_function(
                "nyash_plugin_invoke3_tagged_i64",
                cranelift_module::Linkage::Import,
                &sig,
            )
            .expect("declare plugin invoke tagged");
        let fref = self.module.declare_func_in_func(func_id, fb.func);

        // Prepare args array
        let mut args: Vec<cranelift_codegen::ir::Value> = Vec::with_capacity(12);
        let to_i64 = |fb: &mut FunctionBuilder, v: cranelift_codegen::ir::Value| {
            if fb.func.dfg.value_type(v) != types::I64 {
                fb.ins().fcvt_to_sint(types::I64, v)
            } else {
                v
            }
        };

        let t_i64 = |_fb: &mut FunctionBuilder, x: i64| -> cranelift_codegen::ir::Value {
            _fb.ins().iconst(types::I64, x)
        };

        // Pass through type_id/method_id from lowering (method_id must match plugin vtable)
        args.push(t_i64(&mut fb, _type_id as i64)); // type_id (runtime may override with real_type_id)
        args.push(t_i64(&mut fb, _method_id as i64)); // method_id
        args.push(t_i64(&mut fb, argc as i64 - 1)); // argc excluding recv
        args.push(to_i64(&mut fb, recv)); // a0 (recv)

        // a1/tag1, a2/tag2, a3/tag3, a4/tag4
        for i in 0..4 {
            if let Some(v) = arg_vals.get(i).copied() {
                args.push(to_i64(&mut fb, v));
                let tg = tag_i64(*arg_tags.get(i).unwrap_or(&ValueTag::Unknown));
                args.push(t_i64(&mut fb, tg));
            } else {
                args.push(t_i64(&mut fb, 0));
                args.push(t_i64(&mut fb, 3));
            }
        }

        let call_inst = fb.ins().call(fref, &args);
        if has_ret {
            if let Some(v) = fb.inst_results(call_inst).get(0).copied() {
                self.value_stack.push(v);
                self.value_tags.push(ValueTag::I64);
            }
        }
    }

    fn emit_plugin_invoke_by_name(&mut self, _method: &str, argc: usize, has_ret: bool) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        use cranelift_frontend::FunctionBuilder;
        // Use nyash.plugin.invoke_by_name_i64(recv_h, method_cstr, argc, a1, a2)
        // Limit: supports up to 2 args beyond receiver.
        let mut fb = FunctionBuilder::new(&mut self.ctx.func, &mut self.fbc);
        if let Some(idx) = self.current_block_index {
            fb.switch_to_block(self.blocks[idx]);
        } else if let Some(b) = self.entry_block {
            fb.switch_to_block(b);
        }

        // Pop args and recv
        let mut arg_vals: Vec<cranelift_codegen::ir::Value> = Vec::new();
        for _ in 0..argc.saturating_sub(1) {
            if let Some(v) = self.value_stack.pop() {
                arg_vals.push(v);
                let _ = self.value_tags.pop();
            }
        }
        let recv = if let Some(v) = self.value_stack.pop() {
            let _ = self.value_tags.pop();
            v
        } else {
            fb.ins().iconst(types::I64, 0)
        };
        arg_vals.reverse();

        let mut sig = Signature::new(self.module.isa().default_call_conv());
        for _ in 0..5 {
            sig.params.push(AbiParam::new(types::I64));
        }
        sig.returns.push(AbiParam::new(types::I64));
        let func_id = self
            .module
            .declare_function(
                "nyash.plugin.invoke_by_name_i64",
                cranelift_module::Linkage::Import,
                &sig,
            )
            .expect("declare plugin invoke by-name");
        let fref = self.module.declare_func_in_func(func_id, fb.func);

        let to_i64 = |fb: &mut FunctionBuilder, v: cranelift_codegen::ir::Value| {
            if fb.func.dfg.value_type(v) != types::I64 {
                fb.ins().fcvt_to_sint(types::I64, v)
            } else {
                v
            }
        };
        let zero = fb.ins().iconst(types::I64, 0);
        let mut args: Vec<cranelift_codegen::ir::Value> = Vec::with_capacity(5);
        args.push(to_i64(&mut fb, recv));
        // method ptr not supported in object builder (no easy CStr symbol payload); pass 0 to let runtime reject if mistakenly used.
        args.push(zero);
        args.push(fb.ins().iconst(types::I64, (argc as i64).saturating_sub(1)));
        args.push(
            arg_vals
                .get(0)
                .copied()
                .map(|v| to_i64(&mut fb, v))
                .unwrap_or(zero),
        );
        args.push(
            arg_vals
                .get(1)
                .copied()
                .map(|v| to_i64(&mut fb, v))
                .unwrap_or(zero),
        );

        let call_inst = fb.ins().call(fref, &args);
        if has_ret {
            if let Some(v) = fb.inst_results(call_inst).get(0).copied() {
                self.value_stack.push(v);
                self.value_tags.push(ValueTag::I64);
            }
        }
    }
}
