use crate::mir::type_contracts::return_exit::{
    validate_return_exit_contract, RETURN_CONTRACT_FALLTHROUGH_TAG,
};
use crate::mir::{MirFunction, MirInstruction};

/// Verifies outcome existence for active non-void return contracts.
///
/// This does not type-check return operands and does not reinterpret cleanup;
/// the VM exit owner validates the final runtime value after cleanup CFG.
pub(crate) fn check_return_outcomes(function: &MirFunction) -> Result<(), String> {
    validate_return_exit_contract(function)?;
    if function.metadata.return_exit_contract.is_none() {
        return Ok(());
    }

    let reachable = crate::mir::verification::utils::compute_reachable_blocks(function);
    let mut reachable_blocks: Vec<_> = reachable.into_iter().collect();
    reachable_blocks.sort();
    for block_id in reachable_blocks {
        let Some(block) = function.blocks.get(&block_id) else {
            return Err(format!(
                "{} function={} block={} reason=missing-reachable-block",
                RETURN_CONTRACT_FALLTHROUGH_TAG, function.signature.name, block_id
            ));
        };
        match block.terminator.as_ref() {
            Some(MirInstruction::Return { value: Some(_) })
            | Some(MirInstruction::Jump { .. })
            | Some(MirInstruction::Branch { .. })
            | Some(MirInstruction::CheckedCallOut { .. })
            | Some(MirInstruction::CheckedCallOutFault { .. })
            | Some(MirInstruction::PinnedTextResidenceEnter { .. })
            | Some(MirInstruction::PinnedTextResidenceTrap { .. }) => {}
            Some(MirInstruction::Return { value: None }) => {
                return Err(format!(
                    "{} function={} block={} reason=return-without-value",
                    RETURN_CONTRACT_FALLTHROUGH_TAG, function.signature.name, block_id
                ));
            }
            Some(other) => {
                return Err(format!(
                    "{} function={} block={} reason=unsupported-terminator terminator={:?}",
                    RETURN_CONTRACT_FALLTHROUGH_TAG, function.signature.name, block_id, other
                ));
            }
            None => {
                return Err(format!(
                    "{} function={} block={} reason=reachable-fallthrough",
                    RETURN_CONTRACT_FALLTHROUGH_TAG, function.signature.name, block_id
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::type_contracts::return_exit::refresh_function_return_exit_contract;
    use crate::mir::{
        BasicBlock, BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirType, ValueId,
    };

    fn contracted_function() -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.value/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.metadata.declared_return_type_name = Some("i64".to_string());
        refresh_function_return_exit_contract(&mut function);
        function
    }

    #[test]
    fn accepts_reachable_value_return_and_ignores_unreachable_fallthrough() {
        let mut function = contracted_function();
        let entry = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(1),
        });
        entry.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        function.add_block(BasicBlock::new(BasicBlockId::new(9)));
        assert!(check_return_outcomes(&function).is_ok());
    }

    #[test]
    fn rejects_reachable_void_return_and_fallthrough() {
        let mut function = contracted_function();
        function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Return { value: None });
        assert!(check_return_outcomes(&function)
            .unwrap_err()
            .contains("return-without-value"));

        function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .terminator = None;
        assert!(check_return_outcomes(&function)
            .unwrap_err()
            .contains("reachable-fallthrough"));
    }
}
