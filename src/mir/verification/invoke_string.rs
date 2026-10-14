//! Independent physical String lifetimes. No source admission or type inference.
//! Owned pairs never escape the expression lane, and End reverses actual Normal
//! acquisition order, including every later Fault prefix.
use crate::mir::instruction::{
    InvokeOperation, StringInvokeOperand as Operand, StringInvokeOperation as Op,
};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

pub(super) fn check(function: &MirFunction) -> Result<(), &'static str> {
    let selected = |instruction: &MirInstruction| {
        matches!(
            instruction,
            MirInstruction::Invoke {
                operation: InvokeOperation::String(_),
                ..
            }
        )
    };
    if !function
        .blocks
        .values()
        .any(|block| block.all_instructions().any(selected))
    {
        return Ok(());
    }
    let mut results = BTreeMap::<ValueId, BasicBlockId>::new();
    let mut producers = BTreeMap::<BasicBlockId, ValueId>::new();
    for block in function.blocks.values() {
        for instruction in block.all_instructions() {
            if let MirInstruction::InvokeNormalResult { dst, invoke_block } = instruction {
                if matches!(
                    function
                        .blocks
                        .get(invoke_block)
                        .and_then(|b| b.terminator.as_ref()),
                    Some(MirInstruction::Invoke {
                        operation: InvokeOperation::String(Op::Add { .. }),
                        ..
                    })
                ) {
                    if results.insert(*dst, *invoke_block).is_some()
                        || producers.insert(*invoke_block, *dst).is_some()
                    {
                        return Err("string-result-duplicate");
                    }
                }
            }
        }
    }
    for block in function.blocks.values() {
        for instruction in block.all_instructions() {
            if let MirInstruction::Invoke {
                operation: InvokeOperation::String(operation),
                ..
            } = instruction
            {
                if matches!(operation, Op::Add { left, .. } if !left.is_text()) {
                    return Err("string-add-left-domain");
                }
                for operand in operation.operands() {
                    match operand {
                        Operand::OwnedString(value) if !results.contains_key(value) => {
                            return Err("string-owned-result-missing");
                        }
                        Operand::NormalInteger(value) if results.contains_key(value) => {
                            return Err("string-integer-domain");
                        }
                        _ => {}
                    }
                }
                if matches!(operation, Op::End { value } if !results.contains_key(value)) {
                    return Err("string-end-result-missing");
                }
            }
            // A pair cannot be copied, boxed, PHI-merged, returned, stored,
            // passed to arbitrary calls or interpreted as one scalar word.
            for value in instruction.used_values() {
                if !results.contains_key(&value) {
                    continue;
                }
                let allowed = matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::String(operation), .. }
                    if matches!(operation, Op::End { value: end } if *end == value)
                        || operation.operands().iter().any(|operand|
                            matches!(operand, Operand::OwnedString(owned) if *owned == value)));
                if !allowed {
                    return Err("string-owned-escape");
                }
            }
        }
        if block
            .return_env
            .as_ref()
            .is_some_and(|env| env.iter().any(|v| results.contains_key(v)))
        {
            return Err("string-owned-escape");
        }
    }
    let mut seen = BTreeMap::<(BasicBlockId, bool), Vec<ValueId>>::new();
    let mut pending = vec![(function.entry_block, Vec::<ValueId>::new(), false)];
    while let Some((id, mut live, faulted)) = pending.pop() {
        if let Some(previous) = seen.get(&(id, faulted)) {
            if previous != &live {
                return Err("string-lifetime-join");
            }
            continue;
        }
        seen.insert((id, faulted), live.clone());
        let block = function.blocks.get(&id).ok_or("string-block-missing")?;
        let term = block
            .terminator
            .as_ref()
            .ok_or("string-terminator-missing")?;
        if let MirInstruction::Invoke {
            operation: InvokeOperation::String(operation),
            ..
        } = term
        {
            if faulted && !matches!(operation, Op::End { .. }) {
                return Err("string-effect-after-fault");
            }
            for operand in operation.operands() {
                if matches!(operand, Operand::OwnedString(value) if !live.contains(value)) {
                    return Err("string-borrow-not-live");
                }
            }
            if let Op::End { value } = operation {
                if live.pop() != Some(*value) {
                    return Err("string-end-order-or-not-live");
                }
            }
        }
        let targets = block.successors_from_terminator();
        if targets.is_empty() && !live.is_empty() {
            return Err("string-missing-end");
        }
        for target in targets.into_iter().rev() {
            let mut next = live.clone();
            let mut next_faulted = faulted;
            if let MirInstruction::Invoke {
                operation,
                normal_landing,
                fault_landing,
                ..
            } = term
            {
                next_faulted |= target == *fault_landing;
                if target == *normal_landing
                    && matches!(operation, InvokeOperation::String(Op::Add { .. }))
                {
                    next.push(*producers.get(&id).ok_or("string-result-missing")?);
                }
            }
            pending.push((target, next, next_faulted));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::instruction::FaultFrameMode;
    use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType};
    fn invoke(operation: Op, normal: u32, fault: u32) -> MirInstruction {
        MirInstruction::Invoke {
            operation: InvokeOperation::String(operation),
            fault_frame: ValueId(0),
            normal_landing: BasicBlockId(normal),
            fault_landing: BasicBlockId(fault),
        }
    }
    fn end(value: u32, normal: u32, fault: u32) -> MirInstruction {
        invoke(
            Op::End {
                value: ValueId(value),
            },
            normal,
            fault,
        )
    }
    fn graph() -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "checked_string".into(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::CONTROL,
            },
            BasicBlockId(0),
        );
        let literal = |s: &str| Operand::StaticText { utf8: s.into() };
        let mut rows = vec![
            (
                0,
                invoke(
                    Op::Add {
                        left: literal("猫\0"),
                        right: literal(""),
                    },
                    1,
                    90,
                ),
            ),
            (
                1,
                invoke(
                    Op::Add {
                        left: Operand::OwnedString(ValueId(2)),
                        right: Operand::NormalInteger(ValueId(1)),
                    },
                    2,
                    10,
                ),
            ),
            (
                2,
                invoke(
                    Op::Print {
                        value: Operand::OwnedString(ValueId(3)),
                    },
                    3,
                    20,
                ),
            ),
            (3, end(3, 4, 21)),
            (4, end(2, 5, 90)),
            (
                5,
                MirInstruction::Return {
                    value: Some(ValueId(1)),
                },
            ),
            (10, end(2, 90, 91)),
            (20, end(3, 21, 22)),
            (21, end(2, 90, 91)),
            (22, end(2, 90, 91)),
        ];
        for id in [90, 91] {
            rows.push((
                id,
                MirInstruction::ReturnFault {
                    fault_frame: ValueId(0),
                },
            ));
        }
        // Every Invoke Normal landing is exclusive, including Unit cleanup.
        // Fault cleanup may converge only after that owned landing.
        let mut trampolines = Vec::new();
        for (id, term) in &mut rows {
            if let MirInstruction::Invoke {
                operation,
                normal_landing,
                ..
            } = term
            {
                if !matches!(operation, InvokeOperation::String(Op::Add { .. })) {
                    let target = *normal_landing;
                    *normal_landing = BasicBlockId(1000 + *id);
                    trampolines.push((
                        1000 + *id,
                        MirInstruction::Jump {
                            target,
                            edge_args: None,
                        },
                    ));
                }
            }
        }
        rows.extend(trampolines);
        for (id, term) in rows {
            let mut block = BasicBlock::new(BasicBlockId(id));
            if id == 0 {
                block.add_instruction(MirInstruction::FaultFrameEnter {
                    dst: ValueId(0),
                    mode: FaultFrameMode::RootOwned,
                });
                block.add_instruction(MirInstruction::Const {
                    dst: ValueId(1),
                    value: ConstValue::Integer(7),
                });
            }
            if id == 1 || id == 2 {
                block.add_instruction(MirInstruction::InvokeNormalResult {
                    invoke_block: BasicBlockId(id - 1),
                    dst: ValueId(id + 1),
                });
            }
            block.set_terminator(term);
            function.add_block(block);
        }
        function
    }
    #[test]
    fn checked_string_normal_fault_prefixes_are_affine_and_do_not_escape() {
        let good = graph();
        let verified = super::super::check_function(&good);
        assert!(verified.is_ok(), "{verified:?}");
        assert_eq!(check(&good), Ok(()));
        let cases: Vec<(u32, MirInstruction, &str)> = vec![
            (3, end(2, 4, 21), "string-end-order-or-not-live"),
            (4, end(3, 5, 90), "string-end-order-or-not-live"),
            (
                10,
                MirInstruction::ReturnFault {
                    fault_frame: ValueId(0),
                },
                "string-missing-end",
            ),
            (
                20,
                MirInstruction::ReturnFault {
                    fault_frame: ValueId(0),
                },
                "string-missing-end",
            ),
            (10, end(3, 90, 91), "string-end-order-or-not-live"),
            (
                10,
                invoke(
                    Op::Print {
                        value: Operand::OwnedString(ValueId(2)),
                    },
                    90,
                    91,
                ),
                "string-effect-after-fault",
            ),
            (
                2,
                invoke(
                    Op::Print {
                        value: Operand::NormalInteger(ValueId(3)),
                    },
                    3,
                    20,
                ),
                "string-integer-domain",
            ),
            (
                2,
                MirInstruction::Return {
                    value: Some(ValueId(3)),
                },
                "string-owned-escape",
            ),
        ];
        for (id, term, reason) in cases {
            let mut bad = good.clone();
            bad.blocks.get_mut(&BasicBlockId(id)).unwrap().terminator = Some(term);
            assert_eq!(check(&bad), Err(reason), "mutated block {id}");
        }
        let mut escaped = good.clone();
        escaped
            .blocks
            .get_mut(&BasicBlockId(2))
            .unwrap()
            .instructions
            .push(MirInstruction::Copy {
                dst: ValueId(9),
                src: ValueId(3),
            });
        assert_eq!(check(&escaped), Err("string-owned-escape"));
        let mut fault_projection = good.clone();
        fault_projection
            .blocks
            .get_mut(&BasicBlockId(10))
            .unwrap()
            .instructions
            .push(MirInstruction::InvokeNormalResult {
                invoke_block: BasicBlockId(1),
                dst: ValueId(9),
            });
        assert!(super::super::check_function(&fault_projection).is_err());
        // One operation's operands survive SSA rewriting in original order;
        // static bytes are not a fabricated ValueId or a C string.
        let mut operation = Op::Add {
            left: Operand::OwnedString(ValueId(2)),
            right: Operand::NormalInteger(ValueId(1)),
        };
        operation.rewrite_values(|value| value.0 += 10);
        assert_eq!(operation.used_values(), vec![ValueId(12), ValueId(11)]);
        assert_eq!(
            operation.normal_result_kind(),
            Some(crate::mir::instruction::InvokeNormalResultKind::OwnedString)
        );
    }
}
