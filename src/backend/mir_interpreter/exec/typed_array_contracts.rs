use super::super::{MirInterpreter, VMError, VMValue};
use crate::mir::function::{TypedArrayContractBoundary, TypedArrayElementContract};
use crate::mir::{MirFunction, ValueId};

const NON_ARRAY_TAG: &str = "[type/typed_array_contract_non_array_value]";
const STATE_CONFLICT_TAG: &str = "[type/typed_array_contract_state_conflict]";
const EXISTING_MISMATCH_TAG: &str = "[type/typed_array_contract_existing_element_mismatch]";

impl MirInterpreter {
    pub(super) fn execute_typed_array_claim(
        &self,
        function: &MirFunction,
        contract_id: &str,
        value: &VMValue,
    ) -> Result<(), VMError> {
        let contract = function
            .metadata
            .typed_array_element_contracts
            .iter()
            .find(|contract| contract.contract_id == contract_id)
            .ok_or_else(|| {
                self.err_invalid(format!(
                    "[type/typed_array_contract_carrier_missing] contract={contract_id}"
                ))
            })?;
        self.claim_typed_array_value(contract, value)
    }

    pub(super) fn execute_typed_array_claim_instruction(
        &self,
        function: &MirFunction,
        contract_id: &str,
        array: ValueId,
    ) -> Result<(), VMError> {
        let value = self.reg_load(array)?;
        self.execute_typed_array_claim(function, contract_id, &value)
    }

    pub(super) fn validate_typed_array_parameters(
        &self,
        function: &MirFunction,
        arguments: &[VMValue],
    ) -> Result<(), VMError> {
        for contract in function
            .metadata
            .typed_array_element_contracts
            .iter()
            .filter(|contract| contract.boundary == TypedArrayContractBoundary::ParameterEntry)
        {
            let crate::mir::function::TypedArrayContractSourceIdentity::Parameter { formal_index } =
                contract.source_identity
            else {
                return Err(
                    self.err_invalid("[type/typed_array_contract_source_drift] parameter_identity")
                );
            };
            let value = arguments.get(formal_index).ok_or_else(|| {
                self.err_invalid("[type/parameter_arity_mismatch] typed_array_parameter")
            })?;
            self.claim_typed_array_value(contract, value)?;
        }
        Ok(())
    }

    pub(super) fn validate_typed_array_return(
        &self,
        function: &MirFunction,
        value: &VMValue,
    ) -> Result<(), VMError> {
        for contract in function
            .metadata
            .typed_array_element_contracts
            .iter()
            .filter(|contract| contract.boundary == TypedArrayContractBoundary::ReturnExit)
        {
            self.claim_typed_array_value(contract, value)?;
        }
        Ok(())
    }

    fn claim_typed_array_value(
        &self,
        contract: &TypedArrayElementContract,
        value: &VMValue,
    ) -> Result<(), VMError> {
        let VMValue::BoxRef(boxed) = value else {
            return Err(self.err_invalid(format!(
                "{} contract={}",
                NON_ARRAY_TAG, contract.contract_id
            )));
        };
        let Some(array) = boxed
            .as_any()
            .downcast_ref::<crate::boxes::array::ArrayBox>()
        else {
            return Err(self.err_invalid(format!(
                "{} contract={}",
                NON_ARRAY_TAG, contract.contract_id
            )));
        };
        array
            .claim_element_contract(contract.element_spec)
            .map_err(|error| match error {
                crate::boxes::array::runtime_contract::TypedArrayRuntimeContractError::StateConflict => self
                    .err_invalid(format!(
                        "{} contract={}",
                        STATE_CONFLICT_TAG, contract.contract_id
                    )),
                crate::boxes::array::runtime_contract::TypedArrayRuntimeContractError::ExistingElementMismatch {
                    index,
                    reason,
                } => self.err_invalid(format!(
                    "{} contract={} index={} reason={}",
                    EXISTING_MISMATCH_TAG, contract.contract_id, index, reason
                )),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::mir_interpreter::MirInterpreter;
    use crate::mir::function::{
        TypedArrayBoundaryValue, TypedArrayContractSource, TypedArrayContractSourceIdentity,
    };
    use crate::mir::{
        ArrayElementWriteKind, ArrayWriteProducerKind, ArrayWriteSiteId, BasicBlockId, BindingId,
        ConstValue, EffectMask, FunctionSignature, LocalSlotId, MirInstruction, MirModule, MirType,
    };
    use crate::typed_array_contract_spec::{ArrayElementContractSpec, ExactArrayElementType};
    use std::sync::Arc;

    fn module_with_typed_push(value: i64) -> MirModule {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.typed_push/0".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::WRITE,
            },
            BasicBlockId::new(0),
        );
        let array = function.next_value_id();
        let item = function.next_value_id();
        let block = function.get_block_mut(function.entry_block).unwrap();
        block.add_instruction(MirInstruction::NewBox {
            dst: array,
            box_type: "ArrayBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::ArrayStateContractClaim {
            contract_id: "typed-array:test".to_string(),
            array,
        });
        block.add_instruction(MirInstruction::Const {
            dst: item,
            value: ConstValue::Integer(value),
        });
        block.add_instruction(MirInstruction::ArrayElementWrite {
            site_id: ArrayWriteSiteId::new(0),
            dst: None,
            kind: ArrayElementWriteKind::Push,
            producer: ArrayWriteProducerKind::MethodCall,
            receiver: array,
            index: None,
            value: item,
        });
        block.add_instruction(MirInstruction::Return { value: None });
        function
            .metadata
            .typed_array_contract_sources
            .push(TypedArrayContractSource {
                contract_id: "typed-array:test".to_string(),
                boundary: TypedArrayContractBoundary::LocalInit,
                source_identity: TypedArrayContractSourceIdentity::LocalSlot(LocalSlotId::from(
                    BindingId::new(0),
                )),
                boundary_value: TypedArrayBoundaryValue::Value(array),
                element_spec: ArrayElementContractSpec {
                    element: ExactArrayElementType::U8,
                },
            });
        let mut module = MirModule::new("typed-array-runtime".to_string());
        module.add_function(function);
        crate::mir::refresh_and_validate_for_boundary(
            &mut module,
            crate::mir::ContractRefreshBoundary::VmExecution,
        )
        .unwrap();
        module
    }

    #[test]
    fn claim_and_write_share_one_vm_runtime_owner() {
        MirInterpreter::new()
            .execute_function_with_args(&module_with_typed_push(255), "Main.typed_push/0", &[])
            .unwrap();

        let error = MirInterpreter::new()
            .execute_function_with_args(&module_with_typed_push(256), "Main.typed_push/0", &[])
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("[type/typed_array_contract_element_runtime_mismatch]"),
            "{error}"
        );
    }

    #[test]
    fn parameter_and_return_boundaries_adopt_the_selected_runtime_state() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.boundaries/1".to_string(),
                params: vec![MirType::Unknown],
                return_type: MirType::Unknown,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.metadata.declared_param_decls = vec![crate::mir::function::MirParamDecl {
            name: "bytes".to_string(),
            declared_type_name: Some("Array<u8>".to_string()),
            implicit_receiver: false,
        }];
        function.metadata.declared_return_type_name = Some("Array<u8>".to_string());
        let mut module = MirModule::new("typed-array-boundaries".to_string());
        module.add_function(function);
        crate::mir::refresh_and_validate_for_boundary(
            &mut module,
            crate::mir::ContractRefreshBoundary::VmExecution,
        )
        .unwrap();
        let function = module.get_function("Main.boundaries/1").unwrap();
        let array: Arc<dyn crate::box_trait::NyashBox> =
            Arc::new(crate::boxes::array::ArrayBox::new_with_elements(vec![
                Box::new(crate::box_trait::IntegerBox::new(7)),
            ]));
        let value = VMValue::BoxRef(array);
        let interpreter = MirInterpreter::new();
        interpreter
            .validate_typed_array_parameters(function, std::slice::from_ref(&value))
            .unwrap();
        interpreter
            .validate_typed_array_return(function, &value)
            .unwrap();

        let invalid: Arc<dyn crate::box_trait::NyashBox> =
            Arc::new(crate::boxes::array::ArrayBox::new_with_elements(vec![
                Box::new(crate::box_trait::IntegerBox::new(256)),
            ]));
        let error = interpreter
            .validate_typed_array_parameters(function, &[VMValue::BoxRef(invalid)])
            .unwrap_err()
            .to_string();
        assert!(error.contains(EXISTING_MISMATCH_TAG), "{error}");
    }
}
