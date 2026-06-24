//! Call handling (split from handlers/calls.rs)
//! - Route by Callee kind
//! - By-name fallback remains for compatibility when Callee is absent

use super::*;
use crate::boxes::array::ArrayBox;

mod externs;
mod global;
mod method;

impl MirInterpreter {
    pub(crate) fn handle_call(
        &mut self,
        dst: Option<ValueId>,
        func: ValueId,
        callee: Option<&Callee>,
        args: &[ValueId],
        block: Option<BasicBlockId>,
        instruction_index: Option<usize>,
    ) -> Result<(), VMError> {
        if std::env::var("HAKO_CABI_TRACE").ok().as_deref() == Some("1") {
            match callee {
                Some(Callee::Global(n)) => {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[hb:path] call Callee::Global {} argc={}",
                        n,
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
                    "[hb:path] call Legacy func_id={:?} argc={}",
                    func,
                    args.len()
                )),
            }
        }
        // SSOT fast-path: route hostbridge.extern_invoke to extern dispatcher regardless of resolution form
        if let Some(Callee::Global(func_name)) = callee {
            if func_name == "hostbridge.extern_invoke"
                || func_name.starts_with("hostbridge.extern_invoke/")
            {
                let v = self.execute_extern_function("hostbridge.extern_invoke", args)?;
                self.write_result(dst, v);
                return Ok(());
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
            if let Some(Callee::Method { method, receiver: Some(recv_id), .. }) = callee {
                if method == "get" || method == "set" {
                    if self.try_direct_array_i64_fastpath(dst, *recv_id, method, args, blk, inst_idx)? {
                        return Ok(());
                    }
                }
            }
        }
        let call_result = if let Some(callee_type) = callee {
            self.execute_callee_call(callee_type, args)?
        } else {
            // Fast path: allow exact module function calls when Callee is absent.
            let name_val = self.reg_load(func)?;
            if let VMValue::String(ref s) = name_val {
                if let Some(f) = self.functions.get(s).cloned() {
                    let mut argv: Vec<VMValue> = Vec::with_capacity(args.len());
                    for a in args {
                        argv.push(self.reg_load(*a)?);
                    }
                    self.exec_function_inner(&f, Some(&argv))?
                } else {
                    return Err(self.err_with_context("call", &format!(
                        "unknown function '{}' (by-name calls unsupported). attach Callee in builder or define the function",
                        s
                    )));
                }
            } else {
                return Err(self.err_with_context(
                    "call",
                    "by-name calls unsupported without Callee attachment",
                ));
            }
        };
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
            let m = p.block() == block
                && p.instruction_index() == instruction_index
                && p.receiver_value() == recv_id
                && p.array_kind() == "DirectArrayI64";
            if !m {
                    p.block(), block, p.instruction_index(), instruction_index, p.receiver_value(), recv_id, p.array_kind());
            }
            m
        });
        let plan = match plan {
            Some(p) => p,
            None => return Ok(false),
        };

        // (b) Resolve receiver — must be ArrayBox with InlineI64 storage
        let recv_val = self.reg_load(recv_id)?;
        let bx = match &recv_val {
            VMValue::BoxRef(bx) => bx,
            _ => return Err(self.err_invalid(
                "[direct_array_i64] receiver not a box (plan/runtime mismatch)",
            )),
        };
        let arr = bx.as_any().downcast_ref::<ArrayBox>().ok_or_else(|| {
            self.err_invalid("[direct_array_i64] receiver is not ArrayBox")
        })?;
        if !arr.uses_inline_i64_slots() {
            return Err(self.err_invalid(
                "[direct_array_i64] plan present but storage is not InlineI64",
            ));
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
                    return Err(self.err_invalid(format!(
                        "[direct_array_i64] store failed idx={}",
                        idx
                    )));
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
            Callee::Global(func_name) => self.execute_global_function(func_name, args),
            Callee::Method {
                box_name,
                method,
                receiver,
                ..
            } => self.execute_method_callee(box_name, method, receiver, args),
            Callee::Constructor { box_type } => {
                Err(self.err_unsupported(&format!("Constructor calls for {}", box_type)))
            }
            Callee::Closure { .. } => Err(self.err_unsupported("Closure creation in VM")),
            Callee::Value(func_val_id) => {
                let _ = self.reg_load(*func_val_id)?;
                Err(self.err_unsupported("First-class function calls in VM"))
            }
            Callee::Extern(extern_name) => self.execute_extern_function(extern_name, args),
        }
    }
}
