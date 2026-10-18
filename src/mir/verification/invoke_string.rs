//! Independent physical String lifetimes. No source admission or type inference.
//! Owned pairs never escape the expression lane, and End reverses actual Normal
//! acquisition order, including every later Fault prefix.
use crate::mir::instruction::{
    InvokeOperation, StringInvokeOperand as Operand, StringInvokeOperation as Op,
};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

pub(super) fn check(function: &MirFunction) -> Result<(), &'static str> {
    #[derive(Clone, Default, PartialEq, Eq)]
    struct Live { temporary: Vec<ValueId>, homes: Vec<ValueId> }
    impl Live {
        fn contains(&self, value: &ValueId) -> bool { self.temporary.contains(value) || self.homes.contains(value) }
        fn is_empty(&self) -> bool { self.temporary.is_empty() && self.homes.is_empty() }
        fn end(&mut self) -> Option<ValueId> { if self.temporary.is_empty() { self.homes.pop() } else { self.temporary.pop() } }
    }
    let selected = |instruction: &MirInstruction| {
        matches!(instruction, MirInstruction::StringResidenceInstall { .. }) || matches!(
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
                        operation: InvokeOperation::String(operation),
                        ..
                    }) if operation.owns_normal_result()
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
                if !operation.valid_operand_domains() {
                    return Err(if matches!(operation, Op::Add { .. }) { "string-add-left-domain" } else { "string-text-operand-domain" });
                }
                if matches!(operation, Op::Substring { start, end, .. } if results.contains_key(start) || results.contains_key(end)) {
                    return Err("string-integer-domain");
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
            if let MirInstruction::StringResidenceInstall { value } = instruction {
                if !results.contains_key(value) { return Err("string-residence-install-result-missing"); }
                if block.terminator.as_ref().is_some_and(|term| std::ptr::eq(term, instruction)) {
                    return Err("string-residence-install-terminator");
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
                let allowed = allowed || matches!(instruction, MirInstruction::StringResidenceInstall { value: installed } if *installed == value);
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
    let mut seen = BTreeMap::<(BasicBlockId, bool), Live>::new();
    let mut pending = vec![(function.entry_block, Live::default(), false)];
    while let Some((id, mut live, faulted)) = pending.pop() {
        if let Some(previous) = seen.get(&(id, faulted)) {
            if previous != &live {
                return Err("string-lifetime-join");
            }
            continue;
        }
        seen.insert((id, faulted), live.clone());
        let block = function.blocks.get(&id).ok_or("string-block-missing")?;
        for row in &block.instructions {
            if let MirInstruction::StringResidenceInstall { value } = row {
                if faulted || !results.contains_key(value) || live.homes.contains(value)
                    || live.temporary.pop() != Some(*value) { return Err("string-residence-install-not-newest-temporary"); }
                live.homes.push(*value);
            }
        }
        if matches!(block.terminator, Some(MirInstruction::StringResidenceInstall { .. })) {
            return Err("string-residence-install-terminator");
        }
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
                if live.end() != Some(*value) {
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
                    && matches!(operation, InvokeOperation::String(operation) if operation.owns_normal_result())
                {
                    next.temporary.push(*producers.get(&id).ok_or("string-result-missing")?);
                }
            }
            pending.push((target, next, next_faulted));
        }
    }
    if function.blocks.values().any(|block| block.instructions.iter().any(|row|
        matches!(row, MirInstruction::StringResidenceInstall { .. }))
        && !seen.keys().any(|(id, _)| *id == block.id)) {
        return Err("string-residence-install-unreachable");
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
                if !matches!(operation, InvokeOperation::String(operation) if operation.owns_normal_result()) {
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
        // A source-backed lifecycle handoff preserves the pair, while moving
        // its cleanup to the Home domain behind remaining expression children.
        let mut installed = good.clone();
        installed.blocks.get_mut(&BasicBlockId(2)).unwrap().instructions.push(
            MirInstruction::StringResidenceInstall { value: ValueId(3) });
        for (id, value) in [(3, 2), (4, 3), (20, 2), (21, 3), (22, 3)] {
            let Some(MirInstruction::Invoke { operation: InvokeOperation::String(Op::End { value: target }), .. }) =
                &mut installed.blocks.get_mut(&BasicBlockId(id)).unwrap().terminator else { panic!("End") };
            *target = ValueId(value);
        }
        assert_eq!(check(&installed), Ok(()));
        assert!(super::super::check_function(&installed).is_ok());
        let mut both_homes = good.clone();
        for (id, value) in [(1, 2), (2, 3)] {
            both_homes.blocks.get_mut(&BasicBlockId(id)).unwrap().instructions.push(
                MirInstruction::StringResidenceInstall { value: ValueId(value) });
        }
        assert_eq!(check(&both_homes), Ok(()), "Home cleanup is reverse installation order");
        for (id, value) in [(2, 2), (2, 1), (2, 3), (10, 2)] {
            let mut bad = installed.clone();
            let rows = &mut bad.blocks.get_mut(&BasicBlockId(id)).unwrap().instructions;
            let event = MirInstruction::StringResidenceInstall { value: ValueId(value) };
            if id == 2 && value == 2 { rows.insert(1, event); } else { rows.push(event); }
            assert_eq!(check(&bad), Err(if value == 1 { "string-residence-install-result-missing" }
                else { "string-residence-install-not-newest-temporary" }));
        }
        let mut early_home_end = good.clone();
        early_home_end.blocks.get_mut(&BasicBlockId(2)).unwrap().instructions.push(
            MirInstruction::StringResidenceInstall { value: ValueId(3) });
        assert_eq!(check(&early_home_end), Err("string-end-order-or-not-live"));
        let mut marker_only = MirFunction::new(good.signature.clone(), BasicBlockId(0));
        let mut block = BasicBlock::new(BasicBlockId(0));
        block.add_instruction(MirInstruction::StringResidenceInstall { value: ValueId(99) });
        block.set_terminator(MirInstruction::Return { value: None });
        marker_only.add_block(block);
        assert_eq!(check(&marker_only), Err("string-residence-install-result-missing"));
        for value in [3, 99] {
            let mut disconnected = good.clone();
            let mut block = BasicBlock::new(BasicBlockId(99));
            block.add_instruction(MirInstruction::StringResidenceInstall { value: ValueId(value) });
            block.set_terminator(MirInstruction::Return { value: None });
            disconnected.add_block(block);
            assert_eq!(check(&disconnected), Err(if value == 3 { "string-residence-install-unreachable" }
                else { "string-residence-install-result-missing" }));
        }
        let mut slice = good.clone();
        if let Some(MirInstruction::Invoke { operation, .. }) = &mut slice.blocks.get_mut(&BasicBlockId(1)).unwrap().terminator {
            *operation = InvokeOperation::String(Op::Substring { value: Operand::OwnedString(ValueId(2)), start: ValueId(1), end: ValueId(1) });
        }
        assert!(super::super::check_function(&slice).is_ok());
        assert_eq!(check(&slice), Ok(()), "substring has the same Normal acquisition and both Fault prefixes");
        let mut bad_slice = slice.clone();
        if let Some(MirInstruction::Invoke { operation: InvokeOperation::String(Op::Substring { start, .. }), .. }) =
            &mut bad_slice.blocks.get_mut(&BasicBlockId(1)).unwrap().terminator { *start = ValueId(2); }
        assert_eq!(check(&bad_slice), Err("string-integer-domain"));
        for operation in [Op::Length { value: Operand::OwnedString(ValueId(3)) },
            Op::IndexOf { value: Operand::OwnedString(ValueId(3)), needle: Operand::StaticText { utf8: "界".into() } }] {
            let mut read = good.clone();
            if let Some(MirInstruction::Invoke { operation: slot, .. }) = &mut read.blocks.get_mut(&BasicBlockId(2)).unwrap().terminator {
                *slot = InvokeOperation::String(operation);
            }
            read.blocks.get_mut(&BasicBlockId(1002)).unwrap().instructions.push(MirInstruction::InvokeNormalResult {
                invoke_block: BasicBlockId(2), dst: ValueId(10),
            });
            assert!(super::super::check_function(&read).is_ok());
            assert_eq!(check(&read), Ok(()), "scalar reads borrow without acquisition or End");
        }
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
