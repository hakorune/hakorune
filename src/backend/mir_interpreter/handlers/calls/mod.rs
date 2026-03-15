//! Call handling (split from handlers/calls.rs)
//! - Route by Callee kind
//! - Keep legacy path isolated for phased removal

use super::*;

mod externs;
mod global;
mod method;
// legacy by-name resolver has been removed (Phase 2 complete)

impl MirInterpreter {
    pub(crate) fn handle_call(
        &mut self,
        dst: Option<ValueId>,
        func: ValueId,
        callee: Option<&Callee>,
        args: &[ValueId],
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
        let call_result = if let Some(callee_type) = callee {
            self.execute_callee_call(callee_type, args)?
        } else {
            // Fast path: allow exact module function calls without legacy resolver.
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
