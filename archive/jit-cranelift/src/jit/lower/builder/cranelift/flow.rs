use super::super::tls::clif_tls;
use super::super::{BinOpKind, CmpKind, IRBuilder, ParamKind};
use super::CraneliftBuilder;

impl IRBuilder for CraneliftBuilder {
    fn prepare_blocks(&mut self, count: usize) {
        // Allow being called before begin_function; stash desired count
        let mut need_tls = false;
        clif_tls::FB.with(|cell| {
            need_tls = cell.borrow().is_none();
        });
        if need_tls {
            self.pending_blocks = self.pending_blocks.max(count);
            return;
        }
        Self::with_fb(|fb| {
            if count == 0 {
                return;
            }
            if self.blocks.len() < count {
                for _ in 0..(count - self.blocks.len()) {
                    self.blocks.push(fb.create_block());
                }
            }
        });
    }
    fn switch_to_block(&mut self, index: usize) {
        if index >= self.blocks.len() {
            return;
        }
        // Avoid redundant switch_to_block calls that can trip FunctionBuilder state
        if self.current_block_index == Some(index) {
            return;
        }
        Self::with_fb(|fb| {
            // If switching away from a non-terminated block, inject jump to keep CFG sane
            if let Some(cur) = self.current_block_index {
                if self.cur_needs_term && cur != index {
                    fb.ins().jump(self.blocks[index], &[]);
                    self.cur_needs_term = false;
                }
            }
            fb.switch_to_block(self.blocks[index]);
            self.current_block_index = Some(index);
            // New current block now requires a terminator before any further switch
            self.cur_needs_term = true;
        });
    }
    fn seal_block(&mut self, _index: usize) { /* final sealing handled in end_function */
    }
    fn br_if_top_is_true(&mut self, then_index: usize, else_index: usize) {
        use cranelift_codegen::ir::condcodes::IntCC;
        Self::with_fb(|fb| {
            if then_index >= self.blocks.len() || else_index >= self.blocks.len() {
                return;
            }
            let cond_val = if let Some(v) = self.value_stack.pop() {
                v
            } else {
                fb.ins().iconst(cranelift_codegen::ir::types::I64, 0)
            };
            let b1 = fb.ins().icmp_imm(IntCC::NotEqual, cond_val, 0);
            fb.ins().brif(
                b1,
                self.blocks[then_index],
                &[],
                self.blocks[else_index],
                &[],
            );
        });
        self.cur_needs_term = false;
        self.stats.3 += 1;
    }
    fn jump_to(&mut self, target_index: usize) {
        Self::with_fb(|fb| {
            if target_index < self.blocks.len() {
                fb.ins().jump(self.blocks[target_index], &[]);
            }
        });
        self.stats.3 += 1;
    }
    fn ensure_block_params_i64(&mut self, index: usize, count: usize) {
        self.block_param_counts.insert(index, count);
    }
    fn push_block_param_i64_at(&mut self, pos: usize) {
        let v = Self::with_fb(|fb| {
            let b = if let Some(i) = self.current_block_index {
                self.blocks[i]
            } else {
                self.entry_block.unwrap()
            };
            let params = fb.func.dfg.block_params(b).to_vec();
            params
                .get(pos)
                .copied()
                .unwrap_or_else(|| fb.ins().iconst(cranelift_codegen::ir::types::I64, 0))
        });
        self.value_stack.push(v);
    }
    fn br_if_with_args(
        &mut self,
        then_index: usize,
        else_index: usize,
        then_n: usize,
        else_n: usize,
    ) {
        use cranelift_codegen::ir::{condcodes::IntCC, types};
        if then_index >= self.blocks.len() || else_index >= self.blocks.len() {
            return;
        }
        let mut else_args: Vec<cranelift_codegen::ir::Value> = Vec::new();
        for _ in 0..else_n {
            if let Some(v) = self.value_stack.pop() {
                else_args.push(v);
            }
        }
        else_args.reverse();
        let mut then_args: Vec<cranelift_codegen::ir::Value> = Vec::new();
        for _ in 0..then_n {
            if let Some(v) = self.value_stack.pop() {
                then_args.push(v);
            }
        }
        then_args.reverse();
        Self::with_fb(|fb| {
            let then_has_inst = self.materialize_succ_params(fb, then_index);
            let else_has_inst = self.materialize_succ_params(fb, else_index);
            let cond_b1 = if let Some(v) = self.value_stack.pop() {
                let ty = fb.func.dfg.value_type(v);
                if ty == types::I64 {
                    let out = fb.ins().icmp_imm(IntCC::NotEqual, v, 0);
                    crate::jit::rt::b1_norm_inc(1);
                    out
                } else {
                    v
                }
            } else {
                let zero = fb.ins().iconst(types::I64, 0);
                let out = fb.ins().icmp_imm(IntCC::NotEqual, zero, 0);
                crate::jit::rt::b1_norm_inc(1);
                out
            };
            let targs = if then_has_inst { Vec::new() } else { then_args };
            let eargs = if else_has_inst { Vec::new() } else { else_args };
            fb.ins().brif(
                cond_b1,
                self.blocks[then_index],
                &targs,
                self.blocks[else_index],
                &eargs,
            );
        });
        self.cur_needs_term = false;
        self.stats.3 += 1;
    }
    fn jump_with_args(&mut self, target_index: usize, n: usize) {
        let mut args: Vec<cranelift_codegen::ir::Value> = Vec::new();
        for _ in 0..n {
            if let Some(v) = self.value_stack.pop() {
                args.push(v);
            }
        }
        args.reverse();
        Self::with_fb(|fb| {
            let has_inst = self.materialize_succ_params(fb, target_index);
            if has_inst {
                args.clear();
            }
            fb.ins().jump(self.blocks[target_index], &args);
        });
        self.cur_needs_term = false;
        self.stats.3 += 1;
    }
    fn hint_ret_bool(&mut self, is_b1: bool) {
        self.ret_hint_is_b1 = is_b1;
    }
    fn ensure_local_i64(&mut self, index: usize) {
        use cranelift_codegen::ir::{StackSlotData, StackSlotKind};
        if self.local_slots.contains_key(&index) {
            return;
        }
        Self::with_fb(|fb| {
            let slot =
                fb.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8));
            self.local_slots.insert(index, slot);
        });
    }
    fn store_local_i64(&mut self, index: usize) {
        use cranelift_codegen::ir::{condcodes::IntCC, types};
        if let Some(mut v) = self.value_stack.pop() {
            if !self.local_slots.contains_key(&index) {
                self.ensure_local_i64(index);
            }
            let slot = self.local_slots.get(&index).copied();
            Self::with_fb(|fb| {
                let ty = fb.func.dfg.value_type(v);
                if ty != types::I64 {
                    if ty == types::F64 {
                        v = fb.ins().fcvt_to_sint(types::I64, v);
                    } else {
                        let one = fb.ins().iconst(types::I64, 1);
                        let zero = fb.ins().iconst(types::I64, 0);
                        let b1 = fb.ins().icmp_imm(IntCC::NotEqual, v, 0);
                        v = fb.ins().select(b1, one, zero);
                    }
                }
                if let Some(slot) = slot {
                    fb.ins().stack_store(v, slot, 0);
                }
                if std::env::var("NYASH_JIT_TRACE_LOCAL").ok().as_deref() == Some("1") {
                    eprintln!(
                        "[JIT-LOCAL] store idx={} (tracked_slots={})",
                        index,
                        self.local_slots.len()
                    );
                }
            });
            if std::env::var("NYASH_JIT_TRACE_LOCAL").ok().as_deref() == Some("1") {
                // Also emit value via dbg hook: tag = 1000 + index
                let tag = Self::with_fb(|fb| fb.ins().iconst(types::I64, (1000 + index as i64)));
                self.value_stack.push(tag);
                self.value_stack.push(v);
                self.emit_host_call_typed(
                    "nyash.jit.dbg_i64",
                    &[ParamKind::I64, ParamKind::I64],
                    true,
                    false,
                );
                let _ = self.value_stack.pop();
            }
        }
    }
    fn load_local_i64(&mut self, index: usize) {
        use cranelift_codegen::ir::types;
        if !self.local_slots.contains_key(&index) {
            self.ensure_local_i64(index);
        }
        if let Some(&slot) = self.local_slots.get(&index) {
            let v = Self::with_fb(|fb| fb.ins().stack_load(types::I64, slot, 0));
            if std::env::var("NYASH_JIT_TRACE_LOCAL").ok().as_deref() == Some("1") {
                eprintln!(
                    "[JIT-LOCAL] load idx={} (tracked_slots={})",
                    index,
                    self.local_slots.len()
                );
            }
            self.value_stack.push(v);
            self.stats.0 += 1;
            if std::env::var("NYASH_JIT_TRACE_LOCAL").ok().as_deref() == Some("1") {
                // tag = 2000 + index
                let tag = Self::with_fb(|fb| fb.ins().iconst(types::I64, (2000 + index as i64)));
                self.value_stack.push(tag);
                self.value_stack.push(v);
                self.emit_host_call_typed(
                    "nyash.jit.dbg_i64",
                    &[ParamKind::I64, ParamKind::I64],
                    true,
                    false,
                );
                let _ = self.value_stack.pop();
            }
        }
    }
}
