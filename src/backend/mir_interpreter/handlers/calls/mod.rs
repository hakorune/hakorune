//! Call handling (split from handlers/calls.rs)
//! - LegacyCallV0 is a terminal compatibility reject; no by-name lookup

use super::*;

mod externs;
mod global;
mod method;

impl MirInterpreter {
    pub(crate) fn reject_legacy_call(&self, callee: Option<&Callee>) -> Result<(), VMError> {
        let error = match callee {
            Some(Callee::Global(_)) => self.err_unsupported(
                "[vm-reference/legacy-call/global-stopped] canonical Global target required",
            ),
            Some(Callee::Method { .. }) => self.err_unsupported(
                "[vm-reference/legacy-call/method-stopped] canonical Method target required",
            ),
            Some(Callee::SameModuleInstance { .. }) => self.err_unsupported(
                "SameModuleInstance calls are unsupported in the VM reference lane",
            ),
            Some(Callee::Constructor { box_type }) => {
                self.err_unsupported(&format!("Constructor calls for {}", box_type))
            }
            Some(Callee::Closure { .. }) => self.err_unsupported("Closure creation in VM"),
            Some(Callee::Value(_)) => self.err_unsupported(
                "[vm-reference/legacy-call/value-stopped] canonical Value target required",
            ),
            Some(Callee::Extern(_)) => self.err_unsupported(
                "[vm-reference/legacy-call/extern-stopped] canonical Extern target required",
            ),
            None => self.err_with_context("call", "call-missing-callee: typed Callee required"),
        };
        Err(error)
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
            .reject_legacy_call(None)
            .expect_err("missing Callee must reject before by-name lookup");

        match err {
            VMError::InvalidInstruction(msg) => {
                assert_eq!(msg, "call: call-missing-callee: typed Callee required");
            }
            other => panic!("unexpected error kind: {:?}", other),
        }
    }
}
