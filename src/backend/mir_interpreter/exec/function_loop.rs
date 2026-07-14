//! Instruction-loop body for an already installed interpreter function frame.

use std::fs::OpenOptions;
use std::io::Write;

use super::super::{MirInterpreter, VMError, VMValue};
use super::BlockOutcome;
use crate::mir::{BasicBlock, BasicBlockId, MirFunction};

use super::super::utils::stepbudget;

impl MirInterpreter {
    fn build_block_table(function: &MirFunction) -> Vec<Option<&BasicBlock>> {
        let max_id = function
            .blocks
            .keys()
            .map(|block| block.as_u32() as usize)
            .max()
            .unwrap_or(0);
        let mut table = vec![None; max_id + 1];
        for (block_id, block) in &function.blocks {
            table[block_id.as_u32() as usize] = Some(block);
        }
        table
    }

    pub(super) fn execute_installed_function(
        &mut self,
        function: &MirFunction,
    ) -> Result<VMValue, VMError> {
        let mut current = function.entry_block;
        let mut last_predecessor: Option<BasicBlockId> = None;
        let block_table = Self::build_block_table(function);
        let max_steps = configured_max_steps();
        let mut steps = 0u64;
        let trace_log_path = trace_log_path();

        loop {
            steps += 1;
            if max_steps > 0 && steps > max_steps {
                return Err(self.step_budget_error(function, current, max_steps, steps));
            }

            let block = block_table
                .get(current.as_u32() as usize)
                .and_then(|entry| *entry)
                .ok_or_else(|| VMError::InvalidBasicBlock(format!("bb {:?} not found", current)))?;

            self.append_trace_log(trace_log_path.as_deref(), current, last_predecessor, steps);
            if self.trace_enabled() {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[vm-trace] enter bb={:?} pred={:?} fn={}",
                    current,
                    last_predecessor,
                    self.cur_fn.as_deref().unwrap_or("")
                ));
            }

            self.apply_phi_nodes(block, last_predecessor)?;
            if let Err(error) = self.execute_block_instructions(function, block) {
                self.log_instruction_error(current, &error);
                return Err(error);
            }

            match self.handle_terminator(block)? {
                BlockOutcome::Return(result) => {
                    let contract_result = self
                        .validate_function_return_contract(function, &result)
                        .and_then(|_| self.validate_typed_array_return(function, &result));
                    crate::runtime::leak_tracker::observe_temps(self.strong_temp_root_count());
                    crate::runtime::leak_tracker::observe_heap_fields(
                        self.strong_heap_field_root_count(),
                    );
                    return contract_result.map(|_| result);
                }
                BlockOutcome::Next {
                    target,
                    predecessor,
                } => {
                    last_predecessor = Some(predecessor);
                    current = target;
                }
            }
        }
    }

    fn step_budget_error(
        &self,
        function: &MirFunction,
        current: BasicBlockId,
        max_steps: u64,
        steps: u64,
    ) -> VMError {
        let last_inst = self.last_inst.as_ref().map(|inst| format!("{:?}", inst));
        let target_block = self.last_block.unwrap_or(current);
        let span = function
            .blocks
            .get(&target_block)
            .and_then(|block| self.lookup_span_for_inst(block, self.last_inst_index));
        let (mir_dump_path, mir_dump_snip_path) = if crate::config::env::joinir_dev::debug_enabled()
        {
            stepbudget::prepare_stepbudget_dumps(function, current, self.last_block)
        } else {
            (None, None)
        };

        VMError::StepBudgetExceeded {
            max_steps,
            steps,
            function: self.cur_fn.clone(),
            current_block: current,
            last_block: self.last_block,
            last_inst,
            last_inst_index: self.last_inst_index,
            span,
            source_file: function.metadata.source_file.clone(),
            mir_dump_path,
            mir_dump_snip_path,
            trace_tail: stepbudget::format_trace_tail(&self.recent_steps),
            loop_signature: stepbudget::loop_signature(&self.recent_steps),
        }
    }

    fn append_trace_log(
        &self,
        path: Option<&str>,
        current: BasicBlockId,
        predecessor: Option<BasicBlockId>,
        steps: u64,
    ) {
        let Some(path) = path else {
            return;
        };
        let _ = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .and_then(|mut file| {
                writeln!(
                    file,
                    "[vm-trace-log] fn={} bb={:?} pred={:?} step={}",
                    self.cur_fn.as_deref().unwrap_or(""),
                    current,
                    predecessor,
                    steps
                )
            });
    }

    fn log_instruction_error(&self, current: BasicBlockId, error: &VMError) {
        if self.trace_enabled() {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[vm-trace] error in bb={:?}: {:?}\n  last_inst={:?}",
                current, error, self.last_inst
            ));
        }
        if self.vm_error_loc_enabled {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[vm/error/loc] fn={} bb={:?} last_inst={:?}",
                self.cur_fn.as_deref().unwrap_or("<unknown>"),
                current,
                self.last_inst
            ));
        }
    }
}

fn configured_max_steps() -> u64 {
    std::env::var("HAKO_VM_MAX_STEPS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .or_else(|| {
            std::env::var("NYASH_VM_MAX_STEPS")
                .ok()
                .and_then(|value| value.parse::<u64>().ok())
        })
        .unwrap_or(1_000_000)
}

fn trace_log_path() -> Option<String> {
    std::env::var("NYASH_VM_TRACE_LOG").ok().map(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() || trimmed == "1" {
            "__mir__.log".to_string()
        } else {
            value
        }
    })
}
