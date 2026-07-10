use super::super::{MirInterpreter, VMError};
use super::exact_numeric_value_checker::validate_exact_numeric_runtime_value;
use crate::mir::type_contracts::local_slot::local_slot_contract;
use crate::mir::{LocalSlotId, MirFunction, ValueId};

impl MirInterpreter {
    pub(super) fn execute_local_contract_write(
        &mut self,
        function: &MirFunction,
        dst: ValueId,
        src: ValueId,
        local_slot_id: LocalSlotId,
    ) -> Result<(), VMError> {
        let contract = local_slot_contract(function, local_slot_id).ok_or_else(|| {
            self.err_invalid(format!(
                "[type/local_contract_carrier_missing] function={} slot={:?}",
                function.signature.name, local_slot_id
            ))
        })?;
        let value = self.reg_load(src)?;
        validate_exact_numeric_runtime_value(&value, &contract.declared_type_name).map_err(
            |reason| {
                self.err_invalid(format!(
                    "[type/local_contract_violation] function={} local={} declared_type={} reason={}",
                    function.signature.name,
                    contract.diagnostic_source_name,
                    contract.declared_type_name,
                    reason
                ))
            },
        )?;
        self.write_reg(dst, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::mir_interpreter::MirInterpreter;
    use crate::mir::function::{LocalContractWriteKind, LocalSlotContract};
    use crate::mir::{
        BasicBlockId, BindingId, ConstValue, EffectMask, FunctionSignature, MirInstruction,
        MirModule, MirType,
    };

    fn module_with_local(value: ConstValue) -> MirModule {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.local/0".to_string(),
                params: Vec::new(),
                return_type: MirType::Unknown,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let src = function.next_value_id();
        let dst = function.next_value_id();
        let slot = LocalSlotId::from(BindingId::new(0));
        let block = function.get_block_mut(function.entry_block).unwrap();
        block.add_instruction(MirInstruction::Const { dst: src, value });
        block.add_instruction(MirInstruction::LocalContractWrite {
            dst,
            src,
            local_slot_id: slot,
            write_kind: LocalContractWriteKind::Init,
        });
        block.add_instruction(MirInstruction::Return { value: Some(dst) });
        function
            .metadata
            .local_slot_contracts
            .push(LocalSlotContract {
                contract_id: "local-slot:0".to_string(),
                local_slot_id: slot,
                diagnostic_source_name: "x".to_string(),
                declared_type_name: "u8".to_string(),
                runtime_check_required: true,
                proof_elision_allowed: false,
                backend_capability_required: "local_slot_exact_numeric".to_string(),
            });
        let mut module = MirModule::new("local-contract-runtime".to_string());
        module.add_function(function);
        module
    }

    #[test]
    fn local_write_checks_before_destination_publication() {
        let valid = module_with_local(ConstValue::Integer(255));
        let value = MirInterpreter::new()
            .execute_function_with_args(&valid, "Main.local/0", &[])
            .unwrap();
        assert!(matches!(value, crate::backend::VMValue::Integer(255)));

        for invalid in [
            ConstValue::Integer(256),
            ConstValue::String("bad".to_string()),
        ] {
            let error = MirInterpreter::new()
                .execute_function_with_args(&module_with_local(invalid), "Main.local/0", &[])
                .unwrap_err()
                .to_string();
            assert!(error.contains("[type/local_contract_violation]"), "{error}");
        }
    }
}
