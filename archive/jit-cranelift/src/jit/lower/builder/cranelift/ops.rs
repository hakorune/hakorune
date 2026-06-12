use super::super::{BinOpKind, CmpKind, IRBuilder, ParamKind};
use super::CraneliftBuilder;

impl IRBuilder for CraneliftBuilder {
    fn emit_const_i64(&mut self, val: i64) {
        use cranelift_codegen::ir::types;
        let v = Self::with_fb(|fb| fb.ins().iconst(types::I64, val));
        self.value_stack.push(v);
        self.stats.0 += 1;
    }
    fn emit_const_f64(&mut self, val: f64) {
        use cranelift_codegen::ir::types;
        let v = Self::with_fb(|fb| fb.ins().f64const(val));
        self.value_stack.push(v);
        self.stats.0 += 1;
    }
    fn emit_binop(&mut self, op: BinOpKind) {
        use cranelift_codegen::ir::types;
        if self.value_stack.len() < 2 {
            return;
        }
        let mut rhs = self.value_stack.pop().unwrap();
        let mut lhs = self.value_stack.pop().unwrap();
        let res = Self::with_fb(|fb| {
            let lty = fb.func.dfg.value_type(lhs);
            let rty = fb.func.dfg.value_type(rhs);
            let native_f64 = crate::jit::config::current().native_f64;
            let use_f64 = native_f64 && (lty == types::F64 || rty == types::F64);
            if use_f64 {
                if lty != types::F64 {
                    lhs = fb.ins().fcvt_from_sint(types::F64, lhs);
                }
                if rty != types::F64 {
                    rhs = fb.ins().fcvt_from_sint(types::F64, rhs);
                }
                match op {
                    BinOpKind::Add => fb.ins().fadd(lhs, rhs),
                    BinOpKind::Sub => fb.ins().fsub(lhs, rhs),
                    BinOpKind::Mul => fb.ins().fmul(lhs, rhs),
                    BinOpKind::Div => fb.ins().fdiv(lhs, rhs),
                    // Cranelift does not have a native fmod; approximate by integer remainder on truncated values
                    BinOpKind::Mod => {
                        let li = fb
                            .ins()
                            .fcvt_to_sint(cranelift_codegen::ir::types::I64, lhs);
                        let ri = fb
                            .ins()
                            .fcvt_to_sint(cranelift_codegen::ir::types::I64, rhs);
                        fb.ins().srem(li, ri)
                    }
                }
            } else {
                match op {
                    BinOpKind::Add => fb.ins().iadd(lhs, rhs),
                    BinOpKind::Sub => fb.ins().isub(lhs, rhs),
                    BinOpKind::Mul => fb.ins().imul(lhs, rhs),
                    BinOpKind::Div => fb.ins().sdiv(lhs, rhs),
                    BinOpKind::Mod => fb.ins().srem(lhs, rhs),
                }
            }
        });
        self.value_stack.push(res);
        self.stats.1 += 1;
    }
    fn emit_compare(&mut self, op: CmpKind) {
        use cranelift_codegen::ir::{
            condcodes::{FloatCC, IntCC},
            types,
        };
        if self.value_stack.len() < 2 {
            return;
        }
        let mut rhs = self.value_stack.pop().unwrap();
        let mut lhs = self.value_stack.pop().unwrap();
        Self::with_fb(|fb| {
            let lty = fb.func.dfg.value_type(lhs);
            let rty = fb.func.dfg.value_type(rhs);
            let native_f64 = crate::jit::config::current().native_f64;
            let use_f64 = native_f64 && (lty == types::F64 || rty == types::F64);
            let b1 = if use_f64 {
                if lty != types::F64 {
                    lhs = fb.ins().fcvt_from_sint(types::F64, lhs);
                }
                if rty != types::F64 {
                    rhs = fb.ins().fcvt_from_sint(types::F64, rhs);
                }
                let cc = match op {
                    CmpKind::Eq => FloatCC::Equal,
                    CmpKind::Ne => FloatCC::NotEqual,
                    CmpKind::Lt => FloatCC::LessThan,
                    CmpKind::Le => FloatCC::LessThanOrEqual,
                    CmpKind::Gt => FloatCC::GreaterThan,
                    CmpKind::Ge => FloatCC::GreaterThanOrEqual,
                };
                fb.ins().fcmp(cc, lhs, rhs)
            } else {
                let cc = match op {
                    CmpKind::Eq => IntCC::Equal,
                    CmpKind::Ne => IntCC::NotEqual,
                    CmpKind::Lt => IntCC::SignedLessThan,
                    CmpKind::Le => IntCC::SignedLessThanOrEqual,
                    CmpKind::Gt => IntCC::SignedGreaterThan,
                    CmpKind::Ge => IntCC::SignedGreaterThanOrEqual,
                };
                fb.ins().icmp(cc, lhs, rhs)
            };
            self.value_stack.push(b1);
            self.stats.2 += 1;
        });
    }
    fn emit_select_i64(&mut self) {
        use cranelift_codegen::ir::{condcodes::IntCC, types};
        if self.value_stack.len() < 3 {
            return;
        }
        let mut else_v = self.value_stack.pop().unwrap();
        let mut then_v = self.value_stack.pop().unwrap();
        let mut cond_v = self.value_stack.pop().unwrap();
        let sel = Self::with_fb(|fb| {
            let cty = fb.func.dfg.value_type(cond_v);
            if cty == types::I64 {
                cond_v = fb.ins().icmp_imm(IntCC::NotEqual, cond_v, 0);
                crate::jit::rt::b1_norm_inc(1);
            }
            let tty = fb.func.dfg.value_type(then_v);
            if tty != types::I64 {
                then_v = fb.ins().fcvt_to_sint(types::I64, then_v);
            }
            let ety = fb.func.dfg.value_type(else_v);
            if ety != types::I64 {
                else_v = fb.ins().fcvt_to_sint(types::I64, else_v);
            }
            if std::env::var("NYASH_JIT_TRACE_SEL").ok().as_deref() == Some("1") {
                use cranelift_codegen::ir::{AbiParam, Signature};
                let mut sig = Signature::new(self.module.isa().default_call_conv());
                sig.params.push(AbiParam::new(types::I64));
                sig.params.push(AbiParam::new(types::I64));
                sig.returns.push(AbiParam::new(types::I64));
                let fid = self
                    .module
                    .declare_function("nyash.jit.dbg_i64", cranelift_module::Linkage::Import, &sig)
                    .expect("declare dbg_i64");
                let fref = self.module.declare_func_in_func(fid, fb.func);
                let t_cond = fb.ins().iconst(types::I64, 100);
                let one = fb.ins().iconst(types::I64, 1);
                let zero = fb.ins().iconst(types::I64, 0);
                let ci = fb.ins().select(cond_v, one, zero);
                let _ = fb.ins().call(fref, &[t_cond, ci]);
                let t_then = fb.ins().iconst(types::I64, 101);
                let _ = fb.ins().call(fref, &[t_then, then_v]);
                let t_else = fb.ins().iconst(types::I64, 102);
                let _ = fb.ins().call(fref, &[t_else, else_v]);
            }
            fb.ins().select(cond_v, then_v, else_v)
        });
        self.value_stack.push(sel);
    }
    fn emit_jump(&mut self) {
        self.stats.3 += 1;
    }
    fn emit_branch(&mut self) {
        self.stats.3 += 1;
    }
    fn emit_return(&mut self) {
        use cranelift_codegen::ir::types;
        self.stats.4 += 1;
        Self::with_fb(|fb| {
            if fb.func.signature.returns.is_empty() {
                fb.ins().return_(&[]);
                return;
            }
            let mut v = if let Some(x) = self.value_stack.pop() {
                x
            } else {
                fb.ins().iconst(types::I64, 0)
            };
            let v_ty = fb.func.dfg.value_type(v);
            if v_ty != types::I64 {
                v = if v_ty == types::F64 {
                    fb.ins().fcvt_to_sint(types::I64, v)
                } else {
                    let one = fb.ins().iconst(types::I64, 1);
                    let zero = fb.ins().iconst(types::I64, 0);
                    fb.ins().select(v, one, zero)
                }
            }
            if std::env::var("NYASH_JIT_TRACE_RET").ok().as_deref() == Some("1")
                || std::env::var("NYASH_JIT_FORCE_RET_DBG").ok().as_deref() == Some("1")
            {
                use cranelift_codegen::ir::{AbiParam, Signature};
                let mut sig = Signature::new(self.module.isa().default_call_conv());
                sig.params.push(AbiParam::new(types::I64));
                sig.params.push(AbiParam::new(types::I64));
                sig.returns.push(AbiParam::new(types::I64));
                let fid = self
                    .module
                    .declare_function("nyash.jit.dbg_i64", cranelift_module::Linkage::Import, &sig)
                    .expect("declare dbg_i64");
                let fref = self.module.declare_func_in_func(fid, fb.func);
                let tag = fb.ins().iconst(types::I64, 201);
                let _ = fb.ins().call(fref, &[tag, v]);
            }
            // Persist return value in a dedicated stack slot to avoid SSA arg mishaps on ret block
            if self.ret_slot.is_none() {
                use cranelift_codegen::ir::StackSlotData;
                let ss = fb.create_sized_stack_slot(StackSlotData::new(
                    cranelift_codegen::ir::StackSlotKind::ExplicitSlot,
                    8,
                ));
                self.ret_slot = Some(ss);
            }
            if let Some(ss) = self.ret_slot {
                fb.ins().stack_store(v, ss, 0);
            }
            // Unconditional debug of return value just before ret block jump (feed result back to v)
            {
                use cranelift_codegen::ir::{AbiParam, Signature};
                let mut sig = Signature::new(self.module.isa().default_call_conv());
                sig.params.push(AbiParam::new(types::I64));
                sig.params.push(AbiParam::new(types::I64));
                sig.returns.push(AbiParam::new(types::I64));
                let fid = self
                    .module
                    .declare_function("nyash.jit.dbg_i64", cranelift_module::Linkage::Import, &sig)
                    .expect("declare dbg_i64");
                let fref = self.module.declare_func_in_func(fid, fb.func);
                let tag = fb.ins().iconst(types::I64, 211);
                let call_inst = fb.ins().call(fref, &[tag, v]);
                if let Some(rv) = fb.inst_results(call_inst).get(0).copied() {
                    v = rv;
                }
            }
            if let Some(rb) = self.ret_block {
                fb.ins().jump(rb, &[v]);
            }
        });
        self.cur_needs_term = false;
    }
}
