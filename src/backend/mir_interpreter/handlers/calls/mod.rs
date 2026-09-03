//! Call handling (split from handlers/calls.rs)
//! - Route by Callee kind
//! - Missing Callee is a terminal compatibility reject; no by-name lookup

use super::*;

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
        _block: Option<BasicBlockId>,
        _instruction_index: Option<usize>,
    ) -> Result<(), VMError> {
        if matches!(callee, Some(Callee::Value(_))) {
            return Err(self.err_unsupported(
                "[vm-reference/legacy-call/value-stopped] canonical Value target required",
            ));
        }
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
        if matches!(callee, Some(Callee::Method { .. })) {
            return Err(self.err_unsupported(
                "[vm-reference/legacy-call/method-stopped] canonical Method target required",
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
                Some(Callee::Method { .. }) => {
                    unreachable!("legacy Method calls are rejected before trace dispatch")
                }
                None => crate::runtime::get_global_ring0().log.debug(&format!(
                    "[hb:path] call missing-callee argc={}",
                    args.len()
                )),
            }
        }
        let Some(callee_type) = callee else {
            return Err(self.err_with_context("call", "call-missing-callee: typed Callee required"));
        };
        let call_result = self.execute_callee_call(callee_type, args)?;
        self.write_result(dst, call_result);
        Ok(())
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
            Callee::Method { .. } => Err(self.err_unsupported(
                "[vm-reference/legacy-call/method-stopped] canonical Method target required",
            )),
            Callee::SameModuleInstance { .. } => Err(self.err_unsupported(
                "SameModuleInstance calls are unsupported in the VM reference lane",
            )),
            Callee::Constructor { box_type } => {
                Err(self.err_unsupported(&format!("Constructor calls for {}", box_type)))
            }
            Callee::Closure { .. } => Err(self.err_unsupported("Closure creation in VM")),
            Callee::Value(_) => Err(self.err_unsupported(
                "[vm-reference/legacy-call/value-stopped] canonical Value target required",
            )),
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
    use hakorune_mir_defs::{CalleeBoxKind, CanonicalGlobalTargetV1, TypeCertainty};

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
    fn legacy_value_call_rejects_before_dynamic_dispatch() {
        let mut interp = MirInterpreter::new();
        let instruction = MirInstruction::LegacyCallV0 {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Value(ValueId::new(42))),
            args: Vec::new(),
            effects: EffectMask::PURE,
        };

        let error = interp
            .execute_instruction(&instruction)
            .expect_err("legacy Value must stop before register load or dynamic dispatch");
        assert!(
            error
                .to_string()
                .contains("[vm-reference/legacy-call/value-stopped]"),
            "{error}"
        );
    }

    #[test]
    fn legacy_method_call_rejects_before_method_dispatch() {
        let mut interp = MirInterpreter::new();
        let instruction = MirInstruction::LegacyCallV0 {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_owned(),
                method: "get".to_owned(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(1), ValueId::new(2)],
            effects: EffectMask::PURE,
        };

        let error = interp
            .execute_instruction(&instruction)
            .expect_err("legacy Method must stop before method dispatch");
        assert!(
            error
                .to_string()
                .contains("[vm-reference/legacy-call/method-stopped]"),
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
