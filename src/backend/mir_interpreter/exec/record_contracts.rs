use super::super::{MirInterpreter, VMError, VMValue};
use super::exact_numeric_value_checker::validate_exact_numeric_runtime_value;
use crate::mir::type_contracts::record_value::RECORD_CONTRACT_REFRESH_BYPASS_TAG;
use crate::mir::{MirFunction, ValueId};

impl MirInterpreter {
    pub(super) fn execute_record_field_contract_check(
        &mut self,
        function: &MirFunction,
        contract_id: &str,
        field_index: usize,
        value_id: ValueId,
    ) -> Result<(), VMError> {
        let contract = function
            .metadata
            .record_value_contracts
            .iter()
            .find(|contract| contract.contract_id == contract_id)
            .ok_or_else(|| {
                self.err_invalid(format!(
                    "{} function={} contract={}",
                    RECORD_CONTRACT_REFRESH_BYPASS_TAG, function.signature.name, contract_id
                ))
            })?;
        let field = contract
            .fields
            .iter()
            .find(|field| field.field_index == field_index && field.value_id == value_id)
            .ok_or_else(|| {
                self.err_invalid(format!(
                    "[type/record_contract_source_drift] function={} contract={} field_index={}",
                    function.signature.name, contract_id, field_index
                ))
            })?;
        let value = self.reg_load(value_id)?;
        validate_exact_numeric_runtime_value(&value, &field.declared_type_name).map_err(|reason| {
            self.err_invalid(format!(
                "[type/record_contract_field_runtime_mismatch] function={} record={} field={} declared_type={} reason={}",
                function.signature.name,
                contract.diagnostic_record_name,
                field.diagnostic_field_name,
                field.declared_type_name,
                reason
            ))
        })
    }

    pub(super) fn execute_record_value_publish(
        &mut self,
        function: &MirFunction,
        contract_id: &str,
        dst: ValueId,
    ) -> Result<(), VMError> {
        let contract = function
            .metadata
            .record_value_contracts
            .iter()
            .find(|contract| contract.contract_id == contract_id && contract.dst_value_id == dst)
            .ok_or_else(|| {
                self.err_invalid(format!(
                    "[type/record_contract_stale_carrier] function={} contract={}",
                    function.signature.name, contract_id
                ))
            })?;
        let _ = contract;
        self.write_reg(dst, VMValue::Void);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::mir_interpreter::MirInterpreter;
    use crate::mir::function::RecordValueBoundaryKind;
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirModule, MirType,
    };

    #[test]
    fn direct_vm_entry_without_refreshed_carrier_fails() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.main/0".to_string(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let value = function.next_value_id();
        let dst = function.next_value_id();
        let block = function.get_block_mut(function.entry_block).unwrap();
        block.add_instruction(MirInstruction::Const {
            dst: value,
            value: ConstValue::Integer(1),
        });
        block.add_instruction(MirInstruction::RecordFieldContractCheck {
            contract_id: "record-value:1".to_string(),
            schema_fingerprint: "unrefreshed".to_string(),
            field_index: 0,
            value,
        });
        block.add_instruction(MirInstruction::RecordValuePublish {
            dst,
            contract_id: "record-value:1".to_string(),
            boundary: RecordValueBoundaryKind::Construct,
            diagnostic_record_name: "Point".to_string(),
            schema_fingerprint: "unrefreshed".to_string(),
            base: None,
            fields: vec![value],
        });
        block.add_instruction(MirInstruction::Return { value: Some(value) });
        let mut module = MirModule::new("record-refresh-bypass".to_string());
        module.add_function(function);
        let error = MirInterpreter::new()
            .execute_function_with_args(&module, "Main.main/0", &[])
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("[type/record_contract_refresh_bypass]"),
            "{error}"
        );
    }
}
