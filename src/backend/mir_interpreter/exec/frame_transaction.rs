//! Closure-scoped ownership for one interpreter function frame.
//!
//! The transaction separates caller-frame restoration from the instruction
//! loop so every typed return path closes the same state. Shared heap/memory
//! state intentionally remains outside the snapshot.

use std::collections::VecDeque;
use std::mem;

use rustc_hash::FxHashMap;

use super::super::{MirInterpreter, StepTrace, VMError, VMValue};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

// Keep this below the ordinary Rust test/runtime thread stack limit. The VM
// must raise its stable resource error before host recursion can overflow.
const MAX_CALL_DEPTH: usize = 16;

struct RegisterFrameSnapshotV1 {
    regs: FxHashMap<ValueId, VMValue>,
    fast_slots: Vec<Option<VMValue>>,
    copy_aliases: FxHashMap<ValueId, ValueId>,
    i64_cache: Vec<Option<i64>>,
    bool_cache: Vec<Option<bool>>,
    current_function: Option<String>,
    ownership_ssa: Option<crate::mir::ownership_ssa::VerifiedOwnershipSsaV1>,
}

#[derive(Clone)]
struct DiagnosticFrameSnapshotV1 {
    last_block: Option<BasicBlockId>,
    last_inst: Option<MirInstruction>,
    last_inst_index: Option<usize>,
    recent_steps: VecDeque<StepTrace>,
}

#[derive(Debug)]
struct FunctionFrameRestoreErrorV1 {
    imbalances: Vec<&'static str>,
    function_name: String,
    saved_call_depth: usize,
    actual_call_depth: usize,
}

impl std::fmt::Display for FunctionFrameRestoreErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[vm/frame_transaction/restore_failed] function={} fields={} saved_call_depth={} actual_call_depth={}",
            self.function_name,
            self.imbalances.join(","),
            self.saved_call_depth,
            self.actual_call_depth
        )
    }
}

struct FunctionFrameTransactionV1<'interpreter> {
    interpreter: &'interpreter mut MirInterpreter,
    function_name: String,
    trace_stack: bool,
    saved_call_depth: usize,
    saved_call_stack: Vec<String>,
    saved_diagnostics: Option<DiagnosticFrameSnapshotV1>,
    register_frame: Option<RegisterFrameSnapshotV1>,
    closed: bool,
}

pub(super) fn with_function_frame(
    interpreter: &mut MirInterpreter,
    function: &MirFunction,
    arguments: Option<&[VMValue]>,
    operation: impl FnOnce(&mut MirInterpreter) -> Result<VMValue, VMError>,
) -> Result<VMValue, VMError> {
    FunctionFrameTransactionV1::open(interpreter, function).run(function, arguments, operation)
}

// SSA-RC-A1b is intentionally disconnected from production until SSA-I1.
#[allow(dead_code)]
pub(super) fn with_verified_ownership_function_frame(
    interpreter: &mut MirInterpreter,
    function: &MirFunction,
    arguments: Vec<VMValue>,
    expected_owner: crate::mir::ownership_ssa::OwnershipFunctionOwnerV1,
    witness: crate::mir::ownership_ssa::VerifiedOwnershipSsaV1,
    operation: impl FnOnce(&mut MirInterpreter) -> Result<VMValue, VMError>,
) -> Result<VMValue, VMError> {
    if witness.owner() != expected_owner {
        return Err(VMError::InvalidInstruction(
            "[freeze:contract][vm/ownership:foreign_function_owner]".to_string(),
        ));
    }
    FunctionFrameTransactionV1::open(interpreter, function)
        .run_verified(function, arguments, witness, operation)
}

impl<'interpreter> FunctionFrameTransactionV1<'interpreter> {
    fn open(interpreter: &'interpreter mut MirInterpreter, function: &MirFunction) -> Self {
        let trace_stack =
            interpreter.trace_enabled() || std::env::var("NYASH_VM_TRACE_LOG").ok().is_some();
        let saved_call_depth = interpreter.call_depth;
        let saved_call_stack = interpreter.call_stack.clone();
        let saved_diagnostics = DiagnosticFrameSnapshotV1 {
            last_block: interpreter.last_block,
            last_inst: interpreter.last_inst.clone(),
            last_inst_index: interpreter.last_inst_index,
            recent_steps: interpreter.recent_steps.clone(),
        };

        if interpreter.joinir_debug_enabled {
            interpreter.recent_steps.clear();
        }
        if trace_stack {
            interpreter.call_stack.push(function.signature.name.clone());
        }
        interpreter.call_depth = interpreter.call_depth.saturating_add(1);

        Self {
            interpreter,
            function_name: function.signature.name.clone(),
            trace_stack,
            saved_call_depth,
            saved_call_stack,
            saved_diagnostics: Some(saved_diagnostics),
            register_frame: None,
            closed: false,
        }
    }

    fn run(
        mut self,
        function: &MirFunction,
        arguments: Option<&[VMValue]>,
        operation: impl FnOnce(&mut MirInterpreter) -> Result<VMValue, VMError>,
    ) -> Result<VMValue, VMError> {
        let outcome = self.execute(function, arguments, operation);
        self.finish(outcome)
    }

    #[allow(dead_code)]
    fn run_verified(
        mut self,
        function: &MirFunction,
        arguments: Vec<VMValue>,
        witness: crate::mir::ownership_ssa::VerifiedOwnershipSsaV1,
        operation: impl FnOnce(&mut MirInterpreter) -> Result<VMValue, VMError>,
    ) -> Result<VMValue, VMError> {
        let outcome = self.execute_verified(function, arguments, witness, operation);
        self.finish(outcome)
    }

    fn execute(
        &mut self,
        function: &MirFunction,
        arguments: Option<&[VMValue]>,
        operation: impl FnOnce(&mut MirInterpreter) -> Result<VMValue, VMError>,
    ) -> Result<VMValue, VMError> {
        if self.interpreter.call_depth > MAX_CALL_DEPTH {
            self.log_call_depth_overflow();
            return Err(VMError::InvalidInstruction(format!(
                "vm call stack depth exceeded (max_depth={}, fn={})",
                MAX_CALL_DEPTH, self.function_name
            )));
        }

        if let Some(result) =
            super::super::method_router::pre_exec_reroute(self.interpreter, function, arguments)
        {
            return result;
        }

        self.interpreter
            .validate_function_entry_contracts(function, arguments)
            .and_then(|_| {
                crate::mir::verification::return_outcome::check_return_outcomes(function)
                    .map_err(|error| self.interpreter.err_invalid(error))
            })
            .and_then(|_| {
                crate::mir::type_contracts::local_slot::validate_local_slot_contracts(function)
                    .map_err(|error| self.interpreter.err_invalid(error))
            })?;

        self.install_register_frame(function);
        self.interpreter
            .preflight_fail_fast_phi_undefined_if_enabled(function)?;
        self.seed_parameters(function, arguments);
        operation(self.interpreter)
    }

    #[allow(dead_code)]
    fn execute_verified(
        &mut self,
        function: &MirFunction,
        arguments: Vec<VMValue>,
        witness: crate::mir::ownership_ssa::VerifiedOwnershipSsaV1,
        operation: impl FnOnce(&mut MirInterpreter) -> Result<VMValue, VMError>,
    ) -> Result<VMValue, VMError> {
        if self.interpreter.call_depth > MAX_CALL_DEPTH {
            self.log_call_depth_overflow();
            return Err(VMError::InvalidInstruction(format!(
                "vm call stack depth exceeded (max_depth={}, fn={})",
                MAX_CALL_DEPTH, self.function_name
            )));
        }
        self.interpreter
            .validate_function_entry_contracts(function, Some(&arguments))
            .and_then(|_| {
                crate::mir::verification::return_outcome::check_return_outcomes(function)
                    .map_err(|error| self.interpreter.err_invalid(error))
            })
            .and_then(|_| {
                crate::mir::type_contracts::local_slot::validate_local_slot_contracts(function)
                    .map_err(|error| self.interpreter.err_invalid(error))
            })?;

        self.install_register_frame(function);
        self.interpreter.active_ownership_ssa = Some(witness);
        self.interpreter
            .preflight_fail_fast_phi_undefined_if_enabled(function)?;
        self.seed_parameters_owned(function, arguments)?;
        operation(self.interpreter)
    }

    fn install_register_frame(&mut self, function: &MirFunction) {
        debug_assert!(self.register_frame.is_none());
        self.register_frame = Some(RegisterFrameSnapshotV1 {
            regs: mem::take(&mut self.interpreter.regs),
            fast_slots: mem::take(&mut self.interpreter.reg_fast_slots),
            copy_aliases: mem::take(&mut self.interpreter.reg_copy_aliases),
            i64_cache: mem::take(&mut self.interpreter.reg_i64_cache),
            bool_cache: mem::take(&mut self.interpreter.reg_bool_cache),
            current_function: self.interpreter.cur_fn.take(),
            ownership_ssa: self.interpreter.active_ownership_ssa.take(),
        });
        self.interpreter.cur_fn = Some(self.function_name.clone());
        if !self.interpreter.vm_capture_last_inst_enabled {
            self.interpreter.last_inst = None;
        }
        self.interpreter
            .prepare_fast_regfile_slots(function.next_value_id);
    }

    fn seed_parameters(&mut self, function: &MirFunction, arguments: Option<&[VMValue]>) {
        for (index, parameter) in function.params.iter().enumerate() {
            let value = arguments
                .and_then(|values| values.get(index))
                .cloned()
                .unwrap_or(VMValue::Void);
            self.interpreter.write_reg(*parameter, value);
        }
    }

    #[allow(dead_code)]
    fn seed_parameters_owned(
        &mut self,
        function: &MirFunction,
        arguments: Vec<VMValue>,
    ) -> Result<(), VMError> {
        if arguments.len() != function.params.len() {
            return Err(VMError::InvalidInstruction(format!(
                "verified ownership argument arity mismatch: expected={} actual={}",
                function.params.len(),
                arguments.len()
            )));
        }
        for (parameter, value) in function.params.iter().copied().zip(arguments) {
            self.interpreter.write_reg(parameter, value);
        }
        Ok(())
    }

    fn log_call_depth_overflow(&self) {
        // Diagnostic logging is optional; resource fail-fast must not depend
        // on runner-owned Ring0 setup or initialize global runtime state.
        let Some(ring0) = crate::runtime::ring0::GLOBAL_RING0.get() else {
            return;
        };
        ring0.log.debug(&format!(
            "[vm-call-depth] exceeded {} in fn={} (depth={})",
            MAX_CALL_DEPTH, self.function_name, self.interpreter.call_depth
        ));
        if self.trace_stack {
            let start = self.interpreter.call_stack.len().saturating_sub(16);
            ring0.log.debug(&format!(
                "[vm-call-depth] stack (top {}):",
                self.interpreter.call_stack.len() - start
            ));
            for (index, name) in self.interpreter.call_stack[start..].iter().enumerate() {
                ring0.log.debug(&format!("  {}: {}", start + index, name));
            }
        }
    }

    fn finish(mut self, outcome: Result<VMValue, VMError>) -> Result<VMValue, VMError> {
        let restore = self.restore();
        match (outcome, restore) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(primary), Ok(())) => Err(primary),
            (Ok(_), Err(restore)) => Err(VMError::FrameRestoreFailed {
                detail: restore.to_string(),
            }),
            (Err(primary), Err(restore)) => Err(VMError::DuringFrameRestore {
                primary: Box::new(primary),
                restore: restore.to_string(),
            }),
        }
    }

    fn restore(&mut self) -> Result<(), FunctionFrameRestoreErrorV1> {
        let validation = self.validate_before_restore();
        self.restore_unchecked();
        self.closed = true;
        validation
    }

    fn validate_before_restore(&self) -> Result<(), FunctionFrameRestoreErrorV1> {
        let mut imbalances = Vec::new();
        let expected_depth = self.saved_call_depth.saturating_add(1);
        if self.interpreter.call_depth != expected_depth {
            imbalances.push("call_depth");
        }
        let stack_matches = if self.trace_stack {
            self.interpreter.call_stack.len() == self.saved_call_stack.len() + 1
                && self
                    .interpreter
                    .call_stack
                    .starts_with(&self.saved_call_stack)
                && self.interpreter.call_stack.last() == Some(&self.function_name)
        } else {
            self.interpreter.call_stack == self.saved_call_stack
        };
        if !stack_matches {
            imbalances.push("call_stack");
        }
        if self.register_frame.is_some()
            && self.interpreter.cur_fn.as_deref() != Some(self.function_name.as_str())
        {
            imbalances.push("current_function");
        }

        if imbalances.is_empty() {
            Ok(())
        } else {
            Err(FunctionFrameRestoreErrorV1 {
                imbalances,
                function_name: self.function_name.clone(),
                saved_call_depth: self.saved_call_depth,
                actual_call_depth: self.interpreter.call_depth,
            })
        }
    }

    fn restore_unchecked(&mut self) {
        if let Some(snapshot) = self.register_frame.take() {
            self.interpreter.regs = snapshot.regs;
            self.interpreter.reg_fast_slots = snapshot.fast_slots;
            self.interpreter.reg_copy_aliases = snapshot.copy_aliases;
            self.interpreter.reg_i64_cache = snapshot.i64_cache;
            self.interpreter.reg_bool_cache = snapshot.bool_cache;
            self.interpreter.cur_fn = snapshot.current_function;
            self.interpreter.active_ownership_ssa = snapshot.ownership_ssa;
        }
        if let Some(snapshot) = self.saved_diagnostics.take() {
            self.interpreter.last_block = snapshot.last_block;
            self.interpreter.last_inst = snapshot.last_inst;
            self.interpreter.last_inst_index = snapshot.last_inst_index;
            self.interpreter.recent_steps = snapshot.recent_steps;
        }
        self.interpreter.call_depth = self.saved_call_depth;
        self.interpreter.call_stack = self.saved_call_stack.clone();
    }
}

impl Drop for FunctionFrameTransactionV1<'_> {
    fn drop(&mut self) {
        if self.closed {
            return;
        }
        self.restore_unchecked();
        if cfg!(debug_assertions) && !std::thread::panicking() {
            panic!("[vm/frame_transaction/dropped_without_close]");
        }
    }
}
