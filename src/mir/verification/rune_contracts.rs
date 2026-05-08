use crate::mir::contracts::backend_core_ops::instruction_tag;
use crate::mir::effect_capability_plan::EffectRequirement;
use crate::mir::function::MirFunction;
use crate::mir::verification_types::VerificationError;
use crate::mir::{Effect, MirInstruction};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct RuneContractSet {
    no_alloc: bool,
    no_safepoint: bool,
}

impl RuneContractSet {
    fn from_effect_plans(function: &MirFunction) -> Self {
        let mut set = Self::default();
        for plan in &function.metadata.effect_plans {
            for requirement in &plan.requires {
                match requirement {
                    EffectRequirement::NoAlloc => set.no_alloc = true,
                    EffectRequirement::NoSafepoint => set.no_safepoint = true,
                }
            }
        }
        set
    }

    fn has_live_checks(self) -> bool {
        self.no_alloc || self.no_safepoint
    }
}

pub(super) fn check_rune_contracts(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    let contracts = RuneContractSet::from_effect_plans(function);
    if !contracts.has_live_checks() {
        return Ok(());
    }

    let mut errors = Vec::new();
    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, sp) in block.all_spanned_instructions_enumerated() {
            if contracts.no_alloc && sp.inst.effects().contains(Effect::Alloc) {
                errors.push(contract_error(
                    function,
                    block_id,
                    instruction_index,
                    "no_alloc",
                    sp.inst,
                    "instruction may allocate",
                ));
            }
            if contracts.no_safepoint && matches!(sp.inst, MirInstruction::Safepoint) {
                errors.push(contract_error(
                    function,
                    block_id,
                    instruction_index,
                    "no_safepoint",
                    sp.inst,
                    "explicit safepoint is forbidden",
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn contract_error(
    function: &MirFunction,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
    contract: &'static str,
    instruction: &MirInstruction,
    reason: &'static str,
) -> VerificationError {
    let instruction_tag = instruction_tag(instruction);
    VerificationError::RuneContractViolation {
        block,
        instruction_index,
        contract: contract.to_string(),
        instruction: instruction_tag.to_string(),
        reason: format!(
            "[freeze:contract][rune/{contract}] fn={} bb={} inst={} op={} reason={reason}",
            function.signature.name, block, instruction_index, instruction_tag
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::RuneAttr;
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirType, ValueId,
    };

    fn test_function_with_contracts(
        contracts: &[&str],
        instructions: Vec<MirInstruction>,
    ) -> MirFunction {
        let signature = FunctionSignature {
            name: "Test.contract/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function.metadata.runes = contracts
            .iter()
            .map(|name| RuneAttr {
                name: "Contract".to_string(),
                args: vec![(*name).to_string()],
            })
            .collect();

        let block = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block");
        for instruction in instructions {
            block.add_instruction(instruction);
        }
        crate::mir::effect_capability_plan::refresh_function_effect_capability_plans(&mut function);
        function
    }

    fn error_text(function: &MirFunction) -> String {
        check_rune_contracts(function)
            .expect_err("expected contract violation")
            .into_iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn rune_contract_verifier_consumes_effect_plan_metadata() {
        let mut function = test_function_with_contracts(
            &["no_alloc"],
            vec![MirInstruction::NewBox {
                dst: ValueId::new(1),
                box_type: "Box".to_string(),
                args: vec![],
            }],
        );
        function.metadata.runes.clear();

        let text = error_text(&function);
        assert!(text.contains("[freeze:contract][rune/no_alloc]"));
        assert_eq!(function.metadata.effect_plans.len(), 1);
    }

    #[test]
    fn no_alloc_contract_accepts_non_allocating_body() {
        let function = test_function_with_contracts(
            &["no_alloc"],
            vec![
                MirInstruction::Const {
                    dst: ValueId::new(1),
                    value: ConstValue::Integer(7),
                },
                MirInstruction::Return {
                    value: Some(ValueId::new(1)),
                },
            ],
        );

        assert!(check_rune_contracts(&function).is_ok());
    }

    #[test]
    fn no_alloc_contract_rejects_newbox() {
        let function = test_function_with_contracts(
            &["no_alloc"],
            vec![MirInstruction::NewBox {
                dst: ValueId::new(1),
                box_type: "Box".to_string(),
                args: vec![],
            }],
        );

        let text = error_text(&function);
        assert!(text.contains("[freeze:contract][rune/no_alloc]"));
        assert!(text.contains("op=NewBox"));
    }

    #[test]
    fn mir_verifier_runs_rune_contract_check() {
        let function = test_function_with_contracts(
            &["no_alloc"],
            vec![
                MirInstruction::NewBox {
                    dst: ValueId::new(1),
                    box_type: "Box".to_string(),
                    args: vec![],
                },
                MirInstruction::Return { value: None },
            ],
        );

        let errors = crate::mir::verification::MirVerifier::new()
            .verify_function(&function)
            .expect_err("MirVerifier should run rune contract checks");

        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::RuneContractViolation { contract, .. }
                if contract == "no_alloc"
        )));
    }

    #[test]
    fn no_alloc_contract_rejects_alloc_effect_call() {
        let function = test_function_with_contracts(
            &["no_alloc"],
            vec![MirInstruction::Call {
                dst: Some(ValueId::new(1)),
                func: ValueId::new(99),
                callee: None,
                args: vec![],
                effects: EffectMask::PURE.add(Effect::Alloc),
            }],
        );

        let text = error_text(&function);
        assert!(text.contains("[freeze:contract][rune/no_alloc]"));
        assert!(text.contains("op=Call"));
    }

    #[test]
    fn no_safepoint_contract_rejects_explicit_safepoint() {
        let function =
            test_function_with_contracts(&["no_safepoint"], vec![MirInstruction::Safepoint]);

        let text = error_text(&function);
        assert!(text.contains("[freeze:contract][rune/no_safepoint]"));
        assert!(text.contains("op=Safepoint"));
    }

    #[test]
    fn non_live_contract_metadata_stays_noop() {
        let function = test_function_with_contracts(
            &["pure"],
            vec![MirInstruction::NewBox {
                dst: ValueId::new(1),
                box_type: "Box".to_string(),
                args: vec![],
            }],
        );

        assert!(check_rune_contracts(&function).is_ok());
    }
}
