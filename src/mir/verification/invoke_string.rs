//! Independent physical String lifetimes. No source admission or type inference.
//! Owned pairs may be lent only at source-proved Invoke argument slots; End reverses actual Normal
//! acquisition order, including every later Fault prefix.
use crate::mir::instruction::{
    InvokeNormalResultKind, InvokeOperation, StringInvokeOperand as Operand, StringInvokeOperation as Op,
};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

/// Structural role/lifetime checks are also used before the final source
/// handoff. Module verification separately demands every exact Call occurrence.
pub(super) fn check_source_uses(
    function: &MirFunction,
    original: impl Fn(BasicBlockId, usize, usize, ValueId) -> bool,
) -> Result<(), &'static str> {
    let owned: std::collections::BTreeSet<_> = function.blocks.values().flat_map(|block| block.all_instructions())
        .filter_map(|row| match row {
            MirInstruction::InvokeNormalResult { dst, invoke_block }
                if function.blocks.get(invoke_block).and_then(|block| block.terminator.as_ref()).is_some_and(|row|
                    matches!(row, MirInstruction::Invoke { operation, .. } if operation.normal_result_kind() == Some(InvokeNormalResultKind::OwnedString))) => Some(*dst),
            _ => None,
        }).collect();
    for (block, body) in &function.blocks {
        for (index, row) in body.all_instructions().enumerate() {
            let call = match row {
                MirInstruction::Call(call) | MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } => call,
                _ => continue,
            };
            for (ordinal, value) in call.args.iter().enumerate() {
                if owned.contains(value) && !original(*block, index, ordinal, *value) {
                    return Err("string-original-borrow-slot-missing");
                }
            }
        }
    }
    Ok(())
}

fn is_call_argument(instruction: &MirInstruction, value: ValueId) -> bool {
    matches!(instruction, MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. }
        if call.args.contains(&value) && call.callee.receiver() != Some(value)
            && matches!(call.callee, crate::mir::Callee::Global(_) | crate::mir::Callee::SameModuleInstance { .. }))
}

fn text_operands(operation: &InvokeOperation) -> Vec<&Operand> {
    match operation {
        InvokeOperation::String(operation) => operation.operands(),
        InvokeOperation::ArrayElementWrite { value: crate::mir::instruction::ArrayElementWriteValueV1::Text(value), .. } => vec![value],
        InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::FieldHas { key, .. }
            | crate::mir::instruction::MapInvokeOperation::FieldGetIndexed { key, .. }) => vec![key],
        _ => Vec::new(),
    }
}

pub(super) fn check(function: &MirFunction) -> Result<(), &'static str> {
    #[derive(Clone, Default, PartialEq, Eq)]
    struct Live { temporary: Vec<ValueId>, homes: Vec<ValueId>, replacement: Option<(ValueId, ValueId)>, hold: Option<ValueId>, call_pending: Option<ValueId> }
    impl Live {
        fn contains(&self, value: &ValueId) -> bool { self.temporary.contains(value) || self.homes.contains(value) }
        fn is_empty(&self) -> bool { self.temporary.is_empty() && self.homes.is_empty() && self.replacement.is_none() && self.hold.is_none() && self.call_pending.is_none() }
        fn end(&mut self, value: ValueId, faulted: bool) -> bool {
            if let Some(top) = self.temporary.last() {
                if *top != value { return false; }
                self.temporary.pop();
                if self.call_pending.is_some_and(|held| self.temporary.last() == Some(&held)) {
                    self.call_pending = None;
                }
                return true;
            }
            if let Some((new, old)) = self.replacement {
                if !faulted {
                    if value != old || self.homes.last() != Some(&new) { return false; }
                    let Some(index) = self.homes.iter().position(|home| *home == old) else { return false };
                    self.homes.remove(index);
                    self.replacement = None;
                    return true;
                }
                if value == new { self.replacement = None; }
            }
            let index = self.homes.iter().rposition(|home| faulted || Some(*home) != self.hold);
            let Some(index) = index else { return false };
            if self.homes[index] != value { return false; }
            self.homes.remove(index);
            if self.hold == Some(value) { self.hold = None; }
            true
        }
    }
    // Lifecycle dependencies are not actual arguments. Only an owned Text
    // Invoke can carry them, even when no other String operation is selected.
    for row in function.blocks.values().flat_map(|block| block.all_instructions()) {
        match row {
            MirInstruction::Call(call) if !call.text_argument_cleanup.is_empty() => return Err("string-owned-escape"),
            MirInstruction::Invoke { operation: InvokeOperation::Call { call, result }, .. }
                if !call.text_argument_cleanup.is_empty() && *result != crate::mir::instruction::InvokeCallResultKind::OwnedString =>
                    return Err("string-owned-escape"),
            _ => (),
        }
    }
    let selected = |instruction: &MirInstruction| {
        matches!(instruction, MirInstruction::StringResidenceInstall { .. }) || matches!(
            instruction, MirInstruction::Invoke { operation, .. }
                if matches!(operation, InvokeOperation::String(_)
                    | InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::FieldHas { .. }
                        | crate::mir::instruction::MapInvokeOperation::FieldGetIndexed { .. })
                    | InvokeOperation::ArrayElementWrite { value: crate::mir::instruction::ArrayElementWriteValueV1::Text(_), .. })
                    || operation.normal_result_kind() == Some(InvokeNormalResultKind::OwnedString))
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
                        operation,
                        ..
                    }) if operation.normal_result_kind() == Some(InvokeNormalResultKind::OwnedString)
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
                operation,
                ..
            } = instruction
            {
                if let InvokeOperation::ArrayElementWrite { kind, producer, index, value, .. } = operation {
                    if !value.valid_for(*kind, *producer, *index) { return Err("string-text-operand-domain"); }
                }
                if let InvokeOperation::String(operation) = operation {
                    if !operation.valid_operand_domains() {
                        return Err(if matches!(operation, Op::Add { .. }) { "string-add-left-domain" } else { "string-text-operand-domain" });
                    }
                    if matches!(operation, Op::Substring { start, end, .. } if results.contains_key(start) || results.contains_key(end)
                        || check_borrowed_operand(function, *start).is_ok() || check_borrowed_operand(function, *end).is_ok()) {
                        return Err("string-integer-domain");
                    }
                }
                for operand in text_operands(operation) {
                    match operand {
                        Operand::OwnedString(value) if !results.contains_key(value) => {
                            return Err("string-owned-result-missing");
                        }
                        Operand::BorrowedText(value) => check_borrowed_operand(function, *value)?,
                        Operand::NormalInteger(value) if results.contains_key(value) || check_borrowed_operand(function, *value).is_ok() => {
                            return Err("string-integer-domain");
                        }
                        _ => {}
                    }
                }
                if matches!(operation, InvokeOperation::String(Op::End { value }) if !results.contains_key(value)) {
                    return Err("string-end-result-missing");
                }
            }
            if let MirInstruction::StringResidenceInstall { value, replaces, returning } = instruction {
                if (*returning && replaces.is_some()) || replaces.is_some_and(|previous| previous == *value || !results.contains_key(&previous)) {
                    return Err("string-residence-replacement-result-missing");
                }
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
                let allowed = allowed || matches!(instruction, MirInstruction::Invoke { operation, .. }
                    if text_operands(operation).iter().any(|operand| matches!(operand, Operand::OwnedString(owned) if *owned == value)));
                // A typed physical Return may transfer only an installed Home;
                // the path check below requires that exact live pair on Normal.
                let allowed = allowed || matches!(instruction, MirInstruction::Return { value: Some(returned) }
                    if *returned == value && function.signature.return_type == crate::mir::MirType::String);
                // KeepAlive is optimizer metadata, checked for SSA availability
                // by the shared verifier; it performs no runtime pair access.
                let allowed = allowed || matches!(instruction, MirInstruction::KeepAlive { .. })
                    || matches!(instruction, MirInstruction::StringResidenceInstall { value: installed, replaces, .. } if *installed == value || *replaces == Some(value));
                let allowed = allowed || matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::Call { call,
                        result: crate::mir::instruction::InvokeCallResultKind::OwnedString }, .. }
                        if call.text_argument_cleanup.contains(&value));
                if !allowed && !is_call_argument(instruction, value) {
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
            if let MirInstruction::StringResidenceInstall { value, replaces, returning } = row {
                if *returning {
                    if faulted || live.hold.is_some() || live.replacement.is_some() || replaces.is_some()
                        || !live.temporary.is_empty() || !live.homes.contains(value) {
                        return Err("string-residence-install-not-newest-temporary");
                    }
                    live.hold = Some(*value);
                    continue;
                }
                if live.hold.is_some() || live.call_pending.is_some() || faulted || live.replacement.is_some() || !results.contains_key(value) || live.homes.contains(value)
                    || live.temporary.pop() != Some(*value) { return Err("string-residence-install-not-newest-temporary"); }
                if let Some(previous) = replaces {
                    if *previous == *value || !live.homes.contains(previous) { return Err("string-residence-replacement-not-live-home"); }
                    live.replacement = Some((*value, *previous));
                }
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
        if live.replacement.is_some() && matches!(term, MirInstruction::Invoke { operation, .. }
            if !matches!(operation, InvokeOperation::String(Op::End { .. }))) {
            return Err("string-effect-before-replacement-end");
        }
        if let MirInstruction::Invoke {
            operation,
            ..
        } = term
        {
            if !text_operands(operation).is_empty() && (faulted || live.replacement.is_some() || live.hold.is_some() || live.call_pending.is_some()) && !matches!(operation, InvokeOperation::String(Op::End { .. })) {
                return Err("string-effect-after-fault");
            }
            for operand in text_operands(operation) {
                if matches!(operand, Operand::OwnedString(value) if !live.contains(value)) {
                    return Err("string-borrow-not-live");
                }
            }
            if let InvokeOperation::String(Op::End { value }) = operation {
                if !live.end(*value, faulted) {
                    return Err("string-end-order-or-not-live");
                }
            }
        }
        if let MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } = term {
            if live.replacement.is_some() || live.hold.is_some() || live.call_pending.is_some() { return Err("string-effect-before-replacement-end"); }
            if call.text_argument_cleanup.len() > live.temporary.len()
                || !call.text_argument_cleanup.iter().rev().eq(live.temporary[live.temporary.len().saturating_sub(call.text_argument_cleanup.len())..].iter()) {
                return Err("string-end-order-or-not-live");
            }
            for value in &call.args {
                if results.contains_key(value) && (faulted || !live.contains(value)) {
                    return Err("string-call-borrow-not-live");
                }
            }
        }
        if let MirInstruction::Return { value: Some(value) } = term {
            if results.contains_key(value) {
                if function.signature.return_type != crate::mir::MirType::String || faulted
                    || live.replacement.is_some() || !live.temporary.is_empty()
                    || live.homes.as_slice() != [*value] || live.hold.is_some_and(|held| held != *value) {
                    return Err("string-owned-escape");
                }
                live.homes.pop();
                live.hold = None;
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
                    && operation.normal_result_kind() == Some(InvokeNormalResultKind::OwnedString)
                {
                    let result = *producers.get(&id).ok_or("string-result-missing")?;
                    let suffix = match operation { InvokeOperation::Call { call, .. } => call.text_argument_cleanup.len(), _ => 0 };
                    next.temporary.insert(next.temporary.len() - suffix, result);
                    if suffix != 0 { next.call_pending = Some(result); }
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

/// Physical evidence only: source/domain/use closure is checked independently.
/// A readonly descriptor must retain its exact marked formal through Copies.
fn check_borrowed_operand(function: &MirFunction, mut value: ValueId) -> Result<(), &'static str> {
    use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
    let mut seen = std::collections::BTreeSet::new();
    loop {
        if !seen.insert(value) { return Err("string-borrowed-copy-cycle"); }
        let mut definitions = function.blocks.values().flat_map(|block| block.all_instructions())
            .filter(|row| row.dst_value() == Some(value));
        let mut params = function.params.iter().enumerate().filter(|(_, original)| **original == value);
        if let Some((index, _)) = params.next() {
            if params.next().is_some() || definitions.next().is_some()
                || function.metadata.physical_param_carriers.as_deref().and_then(|rows| rows.get(index)) != Some(&Carrier::BorrowedTaggedValue)
                || function.metadata.declared_param_decls.get(index).is_none_or(|row| row.implicit_receiver)
                || !matches!(function.signature.params.get(index), Some(crate::mir::MirType::Integer | crate::mir::MirType::Unknown)) {
                return Err("string-borrowed-formal");
            }
            return Ok(());
        }
        let Some(MirInstruction::Copy { src, .. }) = definitions.next() else { return Err("string-borrowed-origin"); };
        if definitions.next().is_some() { return Err("string-borrowed-copy-duplicate"); }
        value = *src;
    }
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
                    Op::Materialize { value: literal("猫\0") },
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
        let mut good = graph();
        good.blocks.get_mut(&BasicBlockId(2)).unwrap().instructions.push(
            MirInstruction::KeepAlive { values: vec![ValueId(3)] });
        let verified = super::super::check_function(&good);
        assert!(verified.is_ok(), "{verified:?}");
        assert_eq!(check(&good), Ok(()));
        // A function-level role check lends without transferring ownership.
        // The mandatory module source gate separately checks each occurrence.
        let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method("Values", "sink", 1);
        let call = crate::mir::definitions::MirCall::global(None, key.canonical_global_target_v1().unwrap(), vec![ValueId(3)]);
        let mut borrowed = good.clone();
        let call_index = borrowed.blocks[&BasicBlockId(2)].instructions.len();
        let Some(MirInstruction::Invoke { operation, .. }) = borrowed.blocks.get_mut(&BasicBlockId(2)).unwrap().terminator.as_mut() else { panic!("original Invoke") };
        *operation = InvokeOperation::Call { call: call.clone(), result: crate::mir::instruction::InvokeCallResultKind::Unit };
        assert_eq!(check(&borrowed), Ok(()));
        assert_eq!(check_source_uses(&borrowed, |_, _, _, _| false), Err("string-original-borrow-slot-missing"));
        let exact = |block, index, ordinal, value| block == BasicBlockId(2) && index == call_index && ordinal == 0 && value == ValueId(3);
        assert_eq!(check_source_uses(&borrowed, exact), Ok(()));
        let mut duplicated = borrowed.clone();
        let Some(MirInstruction::Invoke { operation: InvokeOperation::Call { call: duplicated_call, .. }, .. }) = duplicated.blocks.get_mut(&BasicBlockId(2)).unwrap().terminator.as_mut() else { panic!("Call") };
        duplicated_call.args.push(ValueId(3));
        assert_eq!(check_source_uses(&duplicated, exact), Err("string-original-borrow-slot-missing"));
        let mut plain = borrowed.clone();
        plain.blocks.get_mut(&BasicBlockId(2)).unwrap().terminator = Some(MirInstruction::Call(call));
        assert_eq!(check(&plain), Err("string-owned-escape"));
        let mut after_end = borrowed.clone();
        after_end.blocks.get_mut(&BasicBlockId(4)).unwrap().terminator = borrowed.blocks[&BasicBlockId(2)].terminator.clone();
        assert_eq!(check(&after_end), Err("string-call-borrow-not-live"));
        // A source-backed lifecycle handoff preserves the pair, while moving
        // its cleanup to the Home domain behind remaining expression children.
        // A Text Call holds its result below its own argument suffix. The
        // Normal and Call-Fault graphs retain distinct acquisition obligations.
        let mut text_call = good.clone();
        let mut returning_call = crate::mir::definitions::MirCall::global(None,
            key.canonical_global_target_v1().unwrap(), vec![ValueId(2)]);
        returning_call.text_argument_cleanup = vec![ValueId(2)];
        let Some(MirInstruction::Invoke { operation, .. }) = text_call.blocks.get_mut(&BasicBlockId(1)).unwrap().terminator.as_mut() else { panic!("Call") };
        *operation = InvokeOperation::Call { call: returning_call.clone(), result: crate::mir::instruction::InvokeCallResultKind::OwnedString };
        // End argument first, then use and End the newly available result.
        text_call.blocks.get_mut(&BasicBlockId(2)).unwrap().terminator = Some(end(2, 1002, 23));
        text_call.blocks.get_mut(&BasicBlockId(3)).unwrap().terminator = Some(end(3, 1003, 90));
        text_call.blocks.get_mut(&BasicBlockId(4)).unwrap().terminator =
            Some(MirInstruction::Return { value: Some(ValueId(1)) });
        let mut fault = crate::mir::BasicBlock::new(BasicBlockId(23));
        fault.set_terminator(end(3, 90, 91));
        text_call.add_block(fault);
        assert_eq!(check(&text_call), Ok(()));
        for suffix in [vec![ValueId(1)], vec![ValueId(2), ValueId(2)]] {
            let mut wrong = text_call.clone();
            let Some(MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. }) = wrong.blocks.get_mut(&BasicBlockId(1)).unwrap().terminator.as_mut() else { unreachable!() };
            call.text_argument_cleanup = suffix;
            assert!(check(&wrong).is_err());
        }
        let mut early_read = text_call.clone();
        early_read.blocks.get_mut(&BasicBlockId(2)).unwrap().terminator = good.blocks[&BasicBlockId(2)].terminator.clone();
        assert!(check(&early_read).is_err(), "held result cannot be borrowed before argument End");
        let mut installed = good.clone();
        installed.blocks.get_mut(&BasicBlockId(2)).unwrap().instructions.push(
            MirInstruction::StringResidenceInstall { value: ValueId(3), replaces: None, returning: false });
        for (id, value) in [(3, 2), (4, 3), (20, 2), (21, 3), (22, 3)] {
            let Some(MirInstruction::Invoke { operation: InvokeOperation::String(Op::End { value: target }), .. }) =
                &mut installed.blocks.get_mut(&BasicBlockId(id)).unwrap().terminator else { panic!("End") };
            *target = ValueId(value);
        }
        assert_eq!(check(&installed), Ok(()));
        assert!(super::super::check_function(&installed).is_ok());
        // Text Return transfers the same installed Home only after sibling
        // cleanup; the original Fault suffix still Ends that residence.
        let mut returned = installed.clone();
        returned.signature.return_type = crate::mir::MirType::String;
        returned.blocks.get_mut(&BasicBlockId(4)).unwrap().terminator =
            Some(MirInstruction::Return { value: Some(ValueId(3)) });
        returned.blocks.remove(&BasicBlockId(1004));
        returned.blocks.remove(&BasicBlockId(5));
        assert_eq!(check(&returned), Ok(()));
        for (block, value) in [(4, 2), (20, 3)] {
            let mut wrong = returned.clone();
            wrong.blocks.get_mut(&BasicBlockId(block)).unwrap().terminator =
                Some(MirInstruction::Return { value: Some(ValueId(value)) });
            assert_eq!(check(&wrong), Err("string-owned-escape"));
        }
        let mut temporary_return = good.clone();
        temporary_return.signature.return_type = crate::mir::MirType::String;
        temporary_return.blocks.get_mut(&BasicBlockId(2)).unwrap().terminator =
            Some(MirInstruction::Return { value: Some(ValueId(3)) });
        assert_eq!(check(&temporary_return), Err("string-owned-escape"));
        let mut both_homes = good.clone();
        for (id, value) in [(1, 2), (2, 3)] {
            both_homes.blocks.get_mut(&BasicBlockId(id)).unwrap().instructions.push(
                MirInstruction::StringResidenceInstall { value: ValueId(value), replaces: None, returning: false });
        }
        assert_eq!(check(&both_homes), Ok(()), "Home cleanup is reverse installation order");
        // Hold the newer Home while ending the older one. If that End faults,
        // the held Home is still the next original-rank unwind obligation.
        let mut held_return = both_homes.clone();
        held_return.signature.return_type = MirType::String;
        held_return.blocks.get_mut(&BasicBlockId(2)).unwrap().terminator =
            Some(MirInstruction::Jump { target: BasicBlockId(3), edge_args: None });
        let hold = MirInstruction::StringResidenceInstall { value: ValueId(3), replaces: None, returning: true };
        held_return.blocks.get_mut(&BasicBlockId(3)).unwrap().instructions.push(hold.clone());
        held_return.blocks.get_mut(&BasicBlockId(3)).unwrap().terminator = Some(end(2, 1003, 20));
        held_return.blocks.get_mut(&BasicBlockId(4)).unwrap().terminator =
            Some(MirInstruction::Return { value: Some(ValueId(3)) });
        held_return.blocks.get_mut(&BasicBlockId(20)).unwrap().terminator = Some(end(3, 90, 91));
        assert_eq!(check(&held_return), Ok(()));
        for fault in [false, true] {
            let mut wrong = held_return.clone();
            if fault {
                wrong.blocks.get_mut(&BasicBlockId(20)).unwrap().terminator = Some(end(2, 90, 91));
            } else {
                wrong.blocks.get_mut(&BasicBlockId(3)).unwrap().instructions.push(hold.clone());
            }
            assert!(check(&wrong).is_err(), "duplicate hold and skipped Fault End must refuse");
        }

        for (id, value) in [(2, 2), (2, 1), (2, 3), (10, 2)] {
            let mut bad = installed.clone();
            let rows = &mut bad.blocks.get_mut(&BasicBlockId(id)).unwrap().instructions;
            let event = MirInstruction::StringResidenceInstall { value: ValueId(value), replaces: None, returning: false };
            if id == 2 && value == 2 { rows.insert(1, event); } else { rows.push(event); }
            assert_eq!(check(&bad), Err(if value == 1 { "string-residence-install-result-missing" }
                else { "string-residence-install-not-newest-temporary" }));
        }
        let mut early_home_end = good.clone();
        early_home_end.blocks.get_mut(&BasicBlockId(2)).unwrap().instructions.push(
            MirInstruction::StringResidenceInstall { value: ValueId(3), replaces: None, returning: false });
        assert_eq!(check(&early_home_end), Err("string-end-order-or-not-live"));
        let mut marker_only = MirFunction::new(good.signature.clone(), BasicBlockId(0));
        let mut block = BasicBlock::new(BasicBlockId(0));
        block.add_instruction(MirInstruction::StringResidenceInstall { value: ValueId(99), replaces: None, returning: false });
        block.set_terminator(MirInstruction::Return { value: None });
        marker_only.add_block(block);
        assert_eq!(check(&marker_only), Err("string-residence-install-result-missing"));
        for value in [3, 99] {
            let mut disconnected = good.clone();
            let mut block = BasicBlock::new(BasicBlockId(99));
            block.add_instruction(MirInstruction::StringResidenceInstall { value: ValueId(value), replaces: None, returning: false });
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
