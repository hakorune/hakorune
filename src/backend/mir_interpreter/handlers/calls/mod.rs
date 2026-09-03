//! Call handling (split from handlers/calls.rs)
//! - Route by Callee kind
//! - Missing Callee is a terminal compatibility reject; no by-name lookup

use super::*;
use crate::boxes::array::ArrayBox;

mod externs;
mod global;
mod method;

impl MirInterpreter {
    pub(crate) fn handle_call(
        &mut self,
        dst: Option<ValueId>,
        _func: ValueId,
        callee: Option<&Callee>,
        args: &[ValueId],
        block: Option<BasicBlockId>,
        instruction_index: Option<usize>,
    ) -> Result<(), VMError> {
        if matches!(callee, Some(Callee::Global(_))) {
            return Err(self.err_unsupported(
                "[vm-reference/legacy-call/global-stopped] canonical Global target required",
            ));
        }
        if matches!(callee, Some(Callee::Extern(_))) {
            return Err(self.err_unsupported(
                "[vm-reference/legacy-call/extern-stopped] canonical Extern target required",
            ));
        }
        if std::env::var("HAKO_CABI_TRACE").ok().as_deref() == Some("1") {
            match callee {
                Some(Callee::Global(n)) => {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[hb:path] call Callee::Global {} argc={}",
                        n.display_name(),
                        args.len()
                    ));
                }
                Some(Callee::Method {
                    box_name, method, ..
                }) => crate::runtime::get_global_ring0().log.debug(&format!(
                    "[hb:path] call Callee::Method {}.{} argc={}",
                    box_name,
                    method,
                    args.len()
                )),
                Some(Callee::SameModuleInstance { key, receiver }) => {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[hb:path] call Callee::SameModuleInstance {}.{} / {} recv=%{} argc={}",
                        key.owner(),
                        key.name(),
                        key.arity(),
                        receiver.as_u32(),
                        args.len()
                    ));
                }
                Some(Callee::Constructor { box_type }) => {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[hb:path] call Callee::Constructor {} argc={}",
                        box_type,
                        args.len()
                    ))
                }
                Some(Callee::Closure { .. }) => {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[hb:path] call Callee::Closure argc={}",
                        args.len()
                    ));
                }
                Some(Callee::Value(_)) => {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug(&format!("[hb:path] call Callee::Value argc={}", args.len()));
                }
                Some(Callee::Extern(n)) => {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[hb:path] call Callee::Extern {} argc={}",
                        n,
                        args.len()
                    ));
                }
                None => crate::runtime::get_global_ring0().log.debug(&format!(
                    "[hb:path] call missing-callee argc={}",
                    args.len()
                )),
            }
        }
        if let Some(Callee::Method {
            box_name,
            method,
            receiver,
            ..
        }) = callee
        {
            if receiver.is_none() && box_name == "hostbridge" && method == "extern_invoke" {
                let v = self.execute_extern_function("hostbridge.extern_invoke", args)?;
                self.write_result(dst, v);
                return Ok(());
            }
        }
        // F1: DirectArrayI64 fast-path — before generic dispatch
        let blk = block.or(self.last_block);
        let inst_idx = instruction_index.or(self.last_inst_index);
        if let (Some(blk), Some(inst_idx)) = (blk, inst_idx) {
            if let Some(Callee::Method {
                method,
                receiver: Some(recv_id),
                ..
            }) = callee
            {
                if method == "get" || method == "set" {
                    if self
                        .try_direct_array_i64_fastpath(dst, *recv_id, method, args, blk, inst_idx)?
                    {
                        return Ok(());
                    }
                }
            }
        }
        let Some(callee_type) = callee else {
            return Err(self.err_with_context("call", "call-missing-callee: typed Callee required"));
        };
        let call_result = self.execute_callee_call(callee_type, args)?;
        self.write_result(dst, call_result);
        Ok(())
    }

    /// F1: DirectArrayI64 fast-path consumer.
    /// Returns Ok(true) if handled, Ok(false) if no plan (fallthrough), Err on mismatch.
    fn try_direct_array_i64_fastpath(
        &mut self,
        dst: Option<ValueId>,
        recv_id: ValueId,
        _method: &str,
        _args: &[ValueId],
        block: BasicBlockId,
        instruction_index: usize,
    ) -> Result<bool, VMError> {
        // (a) Plan lookup — mirror numeric_contracts pattern
        let func_name = match &self.cur_fn {
            Some(n) => n.clone(),
            None => return Ok(false),
        };
        let func = match self.functions.get(&func_name) {
            Some(f) => f,
            None => return Ok(false),
        };
        let plan = func.metadata.direct_array_access_plans.iter().find(|p| {
            p.block() == block
                && p.instruction_index() == instruction_index
                && p.receiver_value() == recv_id
                && p.array_kind() == "DirectArrayI64"
        });
        let plan = match plan {
            Some(p) => p,
            None => return Ok(false),
        };

        // (b) Resolve receiver — must be ArrayBox with InlineI64 storage
        let recv_val = self.reg_load(recv_id)?;
        let bx = match &recv_val {
            VMValue::BoxRef(bx) => bx,
            _ => return Ok(false),
        };
        let Some(arr) = bx.as_any().downcast_ref::<ArrayBox>() else {
            return Ok(false);
        };
        if !arr.uses_inline_i64_slots() {
            return Ok(false);
        }

        // (c) Resolve index
        let idx_val = self.reg_load(plan.index_value())?;
        let idx = match &idx_val {
            VMValue::Integer(v) => *v,
            _ => return Err(self.err_invalid("[direct_array_i64] index not i64")),
        };

        // (d) Execute
        use crate::mir::direct_array_access_plan::DirectArrayAccessOp;
        match plan.op() {
            DirectArrayAccessOp::Load => {
                let v = arr.slot_load_i64_raw(idx).ok_or_else(|| {
                    self.err_invalid(format!("[direct_array_i64] load OOB idx={}", idx))
                })?;
                if let Some(d) = plan.result_value().or(dst) {
                    self.write_reg(d, VMValue::Integer(v));
                }
                self.emit_direct_array_trace("load", block, instruction_index, idx, v);
            }
            DirectArrayAccessOp::Store => {
                let value_vid = plan.value_value().ok_or_else(|| {
                    self.err_invalid("[direct_array_i64] store plan missing value_value")
                })?;
                let val_reg = self.reg_load(value_vid)?;
                let val = match &val_reg {
                    VMValue::Integer(v) => *v,
                    _ => return Err(self.err_invalid("[direct_array_i64] store value not i64")),
                };
                if !arr.slot_store_i64_raw(idx, val) {
                    return Err(
                        self.err_invalid(format!("[direct_array_i64] store failed idx={}", idx))
                    );
                }
                if let Some(d) = plan.result_value().or(dst) {
                    self.write_reg(d, VMValue::Void);
                }
                self.emit_direct_array_trace("store", block, instruction_index, idx, val);
            }
        }
        Ok(true)
    }

    fn emit_direct_array_trace(
        &self,
        op: &str,
        block: BasicBlockId,
        inst: usize,
        idx: i64,
        val: i64,
    ) {
        eprintln!(
            "[vm-trace][direct_array_i64] op={} bb={:?} inst={} idx={} val={} (method dispatch AVOIDED)",
            op, block, inst, idx, val
        );
    }

    pub(super) fn execute_callee_call(
        &mut self,
        callee: &Callee,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        match callee {
            Callee::Global(_) => Err(self.err_unsupported(
                "[vm-reference/legacy-call/global-stopped] canonical Global target required",
            )),
            Callee::Method {
                box_name,
                method,
                receiver,
                ..
            } => self.execute_method_callee(box_name, method, receiver, args),
            Callee::SameModuleInstance { .. } => Err(self.err_unsupported(
                "SameModuleInstance calls are unsupported in the VM reference lane",
            )),
            Callee::Constructor { box_type } => {
                Err(self.err_unsupported(&format!("Constructor calls for {}", box_type)))
            }
            Callee::Closure { .. } => Err(self.err_unsupported("Closure creation in VM")),
            Callee::Value(func_val_id) => {
                let _ = self.reg_load(*func_val_id)?;
                Err(self.err_unsupported("First-class function calls in VM"))
            }
            Callee::Extern(_) => Err(self.err_unsupported(
                "[vm-reference/legacy-call/extern-stopped] canonical Extern target required",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::vm_types::VMError;
    use crate::mir::definitions::MirCall;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};
    use hakorune_mir_defs::CanonicalGlobalTargetV1;

    fn void_function(name: &str) -> MirFunction {
        let entry = BasicBlockId::new(0);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: name.to_owned(),
                params: Vec::new(),
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            entry,
        );
        function
            .get_block_mut(entry)
            .expect("function entry exists")
            .add_instruction(MirInstruction::Return { value: None });
        function
    }

    #[test]
    fn canonical_print_call_executes_through_instruction_dispatch() {
        let mut interp = MirInterpreter::new();
        let arg = ValueId::new(1);
        interp.regs.insert(arg, VMValue::Integer(42));
        let instruction = MirInstruction::Call(MirCall::new(
            None,
            Callee::Global(CanonicalGlobalTargetV1::builtin_print()),
            vec![arg],
        ));

        interp
            .execute_instruction(&instruction)
            .expect("canonical Print must be handled by the instruction reader");
    }

    #[test]
    fn canonical_print_call_rejects_wrong_arity_before_dispatch() {
        let mut interp = MirInterpreter::new();
        let instruction = MirInstruction::Call(MirCall::new(
            None,
            Callee::Global(CanonicalGlobalTargetV1::builtin_print()),
            Vec::new(),
        ));

        let error = interp
            .execute_instruction(&instruction)
            .expect_err("Print/0 must fail before provider dispatch");
        assert!(error.to_string().contains("expects 1 arg"), "{error}");
    }

    #[test]
    fn canonical_missing_same_module_global_rejects_before_legacy_dispatch() {
        let mut interp = MirInterpreter::new();
        let target = CanonicalGlobalTargetV1::new_free_function("print".into(), 0)
            .expect("test free-function target must be valid");
        let instruction =
            MirInstruction::Call(MirCall::new(None, Callee::Global(target), Vec::new()));

        let error = interp
            .execute_instruction(&instruction)
            .expect_err("missing same-module target must fail closed");
        assert!(
            error
                .to_string()
                .contains("vm-reference/global-target/unsupported"),
            "{error}"
        );
    }

    #[test]
    fn canonical_free_function_call_executes_through_instruction_dispatch() {
        let mut interp = MirInterpreter::new();
        interp
            .functions
            .insert("free/0".to_owned(), void_function("free/0"));
        let target = CanonicalGlobalTargetV1::new_free_function("free".into(), 0)
            .expect("valid free-function target");
        let instruction =
            MirInstruction::call(None, Callee::Global(target), Vec::new(), EffectMask::PURE);

        interp
            .execute_instruction(&instruction)
            .expect("canonical FreeFunction must use the exact table key");
    }

    #[test]
    fn canonical_static_method_call_executes_through_instruction_dispatch() {
        let mut interp = MirInterpreter::new();
        interp
            .functions
            .insert("Helper.run/0".to_owned(), void_function("Helper.run/0"));
        let target =
            CanonicalGlobalTargetV1::new_static_box_method("Helper".into(), "run".into(), 0)
                .expect("valid static-method target");
        let instruction =
            MirInstruction::call(None, Callee::Global(target), Vec::new(), EffectMask::PURE);

        interp
            .execute_instruction(&instruction)
            .expect("canonical StaticBoxMethod must use the exact table key");
    }

    #[test]
    fn legacy_global_call_rejects_before_legacy_dispatch() {
        let mut interp = MirInterpreter::new();
        let instruction = MirInstruction::LegacyCallV0 {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global(
                CanonicalGlobalTargetV1::new_free_function("free".into(), 0)
                    .expect("valid free-function target"),
            )),
            args: Vec::new(),
            effects: EffectMask::PURE,
        };

        let error = interp
            .execute_instruction(&instruction)
            .expect_err("legacy Global must stop before the old dispatch");
        assert!(
            error
                .to_string()
                .contains("[vm-reference/legacy-call/global-stopped]"),
            "{error}"
        );
    }

    #[test]
    fn missing_callee_rejects_before_legacy_register_lookup() {
        let mut interp = MirInterpreter::new();
        let func = ValueId::new(7);
        interp
            .regs
            .insert(func, VMValue::String("Main.hidden/0".to_string()));

        let err = interp
            .handle_call(None, func, None, &[], None, None)
            .expect_err("missing Callee must reject before by-name lookup");

        match err {
            VMError::InvalidInstruction(msg) => {
                assert_eq!(msg, "call: call-missing-callee: typed Callee required");
            }
            other => panic!("unexpected error kind: {:?}", other),
        }
    }
}
