use super::*;
use crate::mir::basic_block::{BasicBlock, BasicBlockId};
use crate::mir::MirFunction;
use std::fs::OpenOptions;
use std::io::Write;
use std::mem;

use super::utils::stepbudget;

mod block;
mod diagnostics;
mod phi;
mod trace;

pub(crate) use block::BlockOutcome;

impl MirInterpreter {
    fn build_block_table<'a>(func: &'a MirFunction) -> Vec<Option<&'a BasicBlock>> {
        let max_id = func
            .blocks
            .keys()
            .map(|bb| bb.as_u32() as usize)
            .max()
            .unwrap_or(0);
        let mut table = vec![None; max_id + 1];
        for (bb_id, block) in &func.blocks {
            table[bb_id.as_u32() as usize] = Some(block);
        }
        table
    }

    #[inline]
    fn trace_enabled(&self) -> bool {
        self.vm_trace_enabled
    }

    pub(super) fn exec_function_inner(
        &mut self,
        func: &MirFunction,
        arg_vals: Option<&[VMValue]>,
    ) -> Result<VMValue, VMError> {
        // Safety valve: cap nested exec_function_inner depth to avoid Rust stack overflow
        // on accidental infinite recursion in MIR (e.g., self-recursive call chains).
        const MAX_CALL_DEPTH: usize = 1024;
        let trace_stack =
            self.trace_enabled() || std::env::var("NYASH_VM_TRACE_LOG").ok().is_some();
        if self.joinir_debug_enabled {
            self.recent_steps.clear();
        }
        if trace_stack {
            self.call_stack.push(func.signature.name.clone());
        }
        self.call_depth = self.call_depth.saturating_add(1);
        if self.call_depth > MAX_CALL_DEPTH {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[vm-call-depth] exceeded {} in fn={} (depth={})",
                MAX_CALL_DEPTH, func.signature.name, self.call_depth
            ));
            if trace_stack {
                let start = self.call_stack.len().saturating_sub(16);
                ring0.log.debug(&format!(
                    "[vm-call-depth] stack (top {}):",
                    self.call_stack.len() - start
                ));
                for (idx, name) in self.call_stack[start..].iter().enumerate() {
                    ring0.log.debug(&format!("  {}: {}", start + idx, name));
                }
                self.call_stack.pop();
            }
            self.call_depth = self.call_depth.saturating_sub(1);
            return Err(VMError::InvalidInstruction(format!(
                "vm call stack depth exceeded (max_depth={}, fn={})",
                MAX_CALL_DEPTH, func.signature.name
            )));
        }
        // Phase 1: delegate cross-class reroute / narrow fallbacks to method_router
        if let Some(r) = super::method_router::pre_exec_reroute(self, func, arg_vals) {
            if trace_stack {
                self.call_stack.pop();
            }
            self.call_depth = self.call_depth.saturating_sub(1);
            return r;
        }
        let saved_regs = mem::take(&mut self.regs);
        let saved_fast_slots = mem::take(&mut self.reg_fast_slots);
        let saved_aliases = mem::take(&mut self.reg_copy_aliases);
        let saved_i64_cache = mem::take(&mut self.reg_i64_cache);
        let saved_bool_cache = mem::take(&mut self.reg_bool_cache);
        let saved_fn = self.cur_fn.clone();
        self.cur_fn = Some(func.signature.name.clone());
        if !self.vm_capture_last_inst_enabled {
            self.last_inst = None;
        }
        self.prepare_fast_regfile_slots(func.next_value_id);

        // Dev-only fail-fast: detect undefined ValueId used by Phi before VM executes.
        // This turns "late VM reg_load undefined" into an earlier, more actionable stop.
        self.preflight_fail_fast_phi_undefined_if_enabled(func)?;

        match arg_vals {
            Some(args) => {
                // Regular parameter binding: params and args are 1:1
                for (i, pid) in func.params.iter().enumerate() {
                    let v = args.get(i).cloned().unwrap_or(VMValue::Void);
                    self.write_reg(*pid, v);
                }
            }
            None => {
                // Seed all parameters with Void so SSA consumers observe defined values.
                for pid in &func.params {
                    self.write_reg(*pid, VMValue::Void);
                }
            }
        }

        let mut cur = func.entry_block;
        let mut last_pred: Option<BasicBlockId> = None;
        let block_table = Self::build_block_table(func);
        // Dev/runtime safety valve: cap the number of interpreter steps per function
        // to prevent infinite loops from hanging the process.
        let max_steps: u64 = std::env::var("HAKO_VM_MAX_STEPS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .or_else(|| {
                std::env::var("NYASH_VM_MAX_STEPS")
                    .ok()
                    .and_then(|v| v.parse::<u64>().ok())
            })
            .unwrap_or(1_000_000);
        let mut steps: u64 = 0;
        let trace_log_path: Option<String> = std::env::var("NYASH_VM_TRACE_LOG").ok().map(|v| {
            let trimmed = v.trim();
            if trimmed.is_empty() || trimmed == "1" {
                "__mir__.log".to_string()
            } else {
                v
            }
        });

        loop {
            steps += 1;
            // max_steps == 0 は「上限なし」を意味する（開発/診断専用）。
            if max_steps > 0 && steps > max_steps {
                let last_inst_str = self.last_inst.as_ref().map(|inst| format!("{:?}", inst));
                let target_bb = self.last_block.unwrap_or(cur);
                let span = func
                    .blocks
                    .get(&target_bb)
                    .and_then(|block| self.lookup_span_for_inst(block, self.last_inst_index));
                let (mir_dump_path, mir_dump_snip_path) =
                    if crate::config::env::joinir_dev::debug_enabled() {
                        stepbudget::prepare_stepbudget_dumps(func, cur, self.last_block)
                    } else {
                        (None, None)
                    };
                let trace_tail = stepbudget::format_trace_tail(&self.recent_steps);
                let loop_signature = stepbudget::loop_signature(&self.recent_steps);
                if trace_stack {
                    self.call_stack.pop();
                }
                self.call_depth = self.call_depth.saturating_sub(1);
                return Err(VMError::StepBudgetExceeded {
                    max_steps,
                    steps,
                    function: self.cur_fn.clone(),
                    current_block: cur,
                    last_block: self.last_block,
                    last_inst: last_inst_str,
                    last_inst_index: self.last_inst_index,
                    span,
                    source_file: func.metadata.source_file.clone(),
                    mir_dump_path,
                    mir_dump_snip_path,
                    trace_tail,
                    loop_signature,
                });
            }
            let block = block_table
                .get(cur.as_u32() as usize)
                .and_then(|entry| *entry)
                .ok_or_else(|| VMError::InvalidBasicBlock(format!("bb {:?} not found", cur)))?;

            if let Some(path) = trace_log_path.as_ref() {
                let _ = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .and_then(|mut f| {
                        writeln!(
                            f,
                            "[vm-trace-log] fn={} bb={:?} pred={:?} step={}",
                            self.cur_fn.as_deref().unwrap_or(""),
                            cur,
                            last_pred,
                            steps
                        )
                    });
            }

            if self.trace_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[vm-trace] enter bb={:?} pred={:?} fn={}",
                    cur,
                    last_pred,
                    self.cur_fn.as_deref().unwrap_or("")
                ));
            }

            self.apply_phi_nodes(block, last_pred)?;
            if let Err(e) = self.execute_block_instructions(block) {
                if self.trace_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[vm-trace] error in bb={:?}: {:?}\n  last_inst={:?}",
                        cur, e, self.last_inst
                    ));
                }
                // Optional concise error location print (env‑gated)
                if self.vm_error_loc_enabled {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[vm/error/loc] fn={} bb={:?} last_inst={:?}",
                        self.cur_fn.as_deref().unwrap_or("<unknown>"),
                        cur,
                        self.last_inst
                    ));
                }
                if trace_stack {
                    self.call_stack.pop();
                }
                self.call_depth = self.call_depth.saturating_sub(1);
                return Err(e);
            }

            let outcome = match self.handle_terminator(block) {
                Ok(outcome) => outcome,
                Err(e) => {
                    if trace_stack {
                        self.call_stack.pop();
                    }
                    self.call_depth = self.call_depth.saturating_sub(1);
                    return Err(e);
                }
            };

            match outcome {
                BlockOutcome::Return(result) => {
                    crate::runtime::leak_tracker::observe_temps(self.strong_temp_root_count());
                    crate::runtime::leak_tracker::observe_heap_fields(
                        self.strong_heap_field_root_count(),
                    );
                    self.cur_fn = saved_fn;
                    self.regs = saved_regs;
                    self.reg_fast_slots = saved_fast_slots;
                    self.reg_copy_aliases = saved_aliases;
                    self.reg_i64_cache = saved_i64_cache;
                    self.reg_bool_cache = saved_bool_cache;
                    if trace_stack {
                        self.call_stack.pop();
                    }
                    self.call_depth = self.call_depth.saturating_sub(1);
                    return Ok(result);
                }
                BlockOutcome::Next {
                    target,
                    predecessor,
                } => {
                    last_pred = Some(predecessor);
                    cur = target;
                }
            }
        }
    }
}
