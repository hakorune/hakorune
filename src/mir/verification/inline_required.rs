use crate::mir::function::MirFunction;
use crate::mir::inline_plan::{required_inline_plan_violations, InlineRequest};
use crate::mir::verification_types::VerificationError;

pub(super) fn check_required_inline_plans(
    function: &MirFunction,
) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();
    for plan in &function.metadata.inline_plans {
        if plan.request != InlineRequest::Required {
            continue;
        }
        for violation in required_inline_plan_violations(function, plan) {
            errors.push(VerificationError::InlinePlanViolation {
                function: function.signature.name.clone(),
                tag: violation.tag.to_string(),
                block: violation.block,
                instruction_index: violation.instruction_index,
                reason: violation.reason,
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::RuneAttr;
    use crate::mir::{
        BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirType, ValueId,
    };

    fn function_with_runes(runes: Vec<RuneAttr>, instructions: Vec<MirInstruction>) -> MirFunction {
        let signature = FunctionSignature {
            name: "Main.fast/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function.metadata.runes = runes;
        let block = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block");
        for instruction in instructions {
            block.add_instruction(instruction);
        }
        crate::mir::inline_plan::refresh_function_inline_plans(&mut function);
        function
    }

    fn rune(name: &str, arg: &str) -> RuneAttr {
        RuneAttr {
            name: name.to_string(),
            args: vec![arg.to_string()],
        }
    }

    fn required_contract_runes() -> Vec<RuneAttr> {
        vec![
            rune("Lowering", "inline_required"),
            rune("Contract", "no_alloc"),
            rune("Contract", "no_safepoint"),
        ]
    }

    fn error_text(function: &MirFunction) -> String {
        check_required_inline_plans(function)
            .expect_err("expected required inline violation")
            .into_iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn required_inline_verifies_leaf_with_required_contracts() {
        let function = function_with_runes(
            required_contract_runes(),
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

        assert!(check_required_inline_plans(&function).is_ok());
        assert_eq!(function.metadata.inline_plans.len(), 1);
        assert!(function.metadata.inline_plans[0].verified);
    }

    #[test]
    fn required_inline_rejects_missing_contracts() {
        let function = function_with_runes(
            vec![rune("Lowering", "inline_required")],
            vec![MirInstruction::Return { value: None }],
        );

        let text = error_text(&function);
        assert!(text.contains("[inline-plan/missing-contract]"));
        assert!(!function.metadata.inline_plans[0].verified);
    }

    #[test]
    fn mir_verifier_runs_required_inline_check() {
        let function = function_with_runes(
            vec![rune("Lowering", "inline_required")],
            vec![MirInstruction::Return { value: None }],
        );

        let errors = crate::mir::verification::MirVerifier::new()
            .verify_function(&function)
            .expect_err("MirVerifier should run required inline checks");

        assert!(errors.iter().any(|error| matches!(
            error,
            VerificationError::InlinePlanViolation { tag, .. }
                if tag == "missing-contract"
        )));
    }

    #[test]
    fn required_inline_rejects_nested_call() {
        let function = function_with_runes(
            required_contract_runes(),
            vec![MirInstruction::Call {
                dst: None,
                func: ValueId::new(1),
                callee: Some(Callee::Global("Main.helper/0".to_string())),
                args: vec![],
                effects: EffectMask::PURE,
            }],
        );

        let text = error_text(&function);
        assert!(text.contains("[inline-plan/unsupported-call]"));
        assert!(!function.metadata.inline_plans[0].verified);
    }
}
