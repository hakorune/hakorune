//! Independent affine String state; source facts only lend exact aliases,
//! carrier transfers and operations. Normal/Fault order remains checked here.
use super::source::{Origin, TextSourceImage};
use super::*;
#[path = "invoke_string_read.rs"]
pub(super) mod read;

#[path = "invoke_string_state.rs"]
mod state;
use state::{Carrier, Live, Residence};

pub(super) fn check(
    function: &super::super::FunctionVerificationGeometry<'_>,
    source: Option<&TextSourceImage>,
) -> Result<(), (BasicBlockId, &'static str, Option<String>)> {
    let mut site = function.entry_block;
    let mut outstanding = None;
    let result = (|| {
        let empty = TextSourceImage::default();
        let source = source.unwrap_or(&empty);
        read::check_scalar_results(function)?;
        // Lifecycle dependencies are not actual arguments. Only an owned Text
        // Invoke can carry them, even when no other String operation is selected.
        for (block_id, row) in function
            .blocks
            .iter()
            .flat_map(|(id, block)| block.all_instructions().map(move |row| (*id, row)))
        {
            site = block_id;
            match row {
                MirInstruction::Invoke {
                    operation: InvokeOperation::String(Op::EndIfOwned { .. }),
                    ..
                } if !source.permits_operation(block_id, row) => return Err("string-owned-escape"),
                MirInstruction::Call(call) if !call.text_argument_cleanup.is_empty() => {
                    return Err("string-owned-escape")
                }
                MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, result },
                    ..
                } if !call.text_argument_cleanup.is_empty()
                    && *result != crate::mir::instruction::InvokeCallResultKind::OwnedString =>
                {
                    return Err("string-owned-escape")
                }
                _ => (),
            }
        }
        let selected = |instruction: &MirInstruction| {
            matches!(
                instruction,
                MirInstruction::StringResidenceInstall { .. }
                    | MirInstruction::StringLength { .. }
                    | MirInstruction::StringIndexOf { .. }
                    | MirInstruction::StringCompare { .. }
            ) || matches!(
            instruction, MirInstruction::Invoke { operation, .. }
                if matches!(operation, InvokeOperation::FieldSet { mode, .. } if mode.has_owned_text_candidate_v1())
                    || matches!(operation, InvokeOperation::String(_)
                    | InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::FieldHas { .. }
                        | crate::mir::instruction::MapInvokeOperation::FieldGetIndexed { .. }
                        | crate::mir::instruction::MapInvokeOperation::FieldInstallIndexed { .. }
                        | crate::mir::instruction::MapInvokeOperation::FieldDeleteIndexed { .. })
                    | InvokeOperation::ArrayElementWrite { value: crate::mir::instruction::ArrayElementWriteValueV1::Text(_), .. })
                    || operation.normal_result_kind() == Some(InvokeNormalResultKind::OwnedString))
        };
        if source.slots.is_empty()
            && !function
                .blocks
                .values()
                .any(|block| block.all_instructions().any(selected))
        {
            return Ok(());
        }
        let mut results = BTreeMap::<ValueId, BasicBlockId>::new();
        let mut producers = BTreeMap::<BasicBlockId, ValueId>::new();
        for block in function.blocks.values() {
            site = block.id;
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
            site = block.id;
            for instruction in block.all_instructions() {
                if let MirInstruction::Invoke { operation, .. } = instruction {
                    if let InvokeOperation::ArrayElementWrite {
                        kind,
                        producer,
                        index,
                        value,
                        ..
                    } = operation
                    {
                        if !value.valid_for(*kind, *producer, *index) {
                            return Err(domain_refusal(
                                "string-text-operand-domain",
                                &site,
                                operation,
                            ));
                        }
                    }
                    if matches!(operation, InvokeOperation::FieldSet { mode, value, .. }
                    if mode.has_owned_text_candidate_v1() && !results.contains_key(value))
                    {
                        return Err("string-owned-result-missing");
                    }
                    if let InvokeOperation::String(operation) = operation {
                        if !operation.valid_operand_domains() {
                            return Err(domain_refusal(
                                if matches!(operation, Op::Add { .. }) {
                                    "string-add-left-domain"
                                } else {
                                    "string-text-operand-domain"
                                },
                                &site,
                                operation,
                            ));
                        }
                        if let Op::Substring { start, end, .. } = operation {
                            for operand in [start, end] {
                                let Some(value) = operand.value() else {
                                    return Err(domain_refusal(
                                        "string-integer-domain",
                                        &site,
                                        operation,
                                    ));
                                };
                                if results.contains_key(&value)
                                    || matches!(operand, Operand::NormalInteger(_)
                                if check_borrowed_operand(function, value).is_ok())
                                {
                                    return Err(domain_refusal(
                                        "string-integer-domain",
                                        &site,
                                        operation,
                                    ));
                                }
                                if matches!(operand, Operand::Dynamic(_))
                                    && !is_tagged_dynamic_formal(function, value)
                                {
                                    return Err(domain_refusal(
                                        "string-dynamic-source-domain",
                                        &site,
                                        operation,
                                    ));
                                }
                            }
                        }
                    }
                    read::check_domains(
                        function,
                        source,
                        &results,
                        block.id,
                        instruction,
                        &text_operands(operation),
                    )?;
                    if matches!(operation, InvokeOperation::String(Op::End { value }) if !results.contains_key(value))
                    {
                        return Err("string-end-result-missing");
                    }
                }
                if let MirInstruction::StringResidenceInstall {
                    value,
                    replaces,
                    returning,
                } = instruction
                {
                    if (*returning && replaces.is_some())
                        || replaces.is_some_and(|previous| {
                            previous == *value
                                || (!results.contains_key(&previous)
                                    && !(source.permits_operation(block.id, instruction)
                                        && matches!(
                                            source.origin(previous),
                                            Some(Origin::Slot(_))
                                        )))
                        })
                    {
                        return Err("string-residence-replacement-result-missing");
                    }
                    if !results.contains_key(value) {
                        return Err("string-residence-install-result-missing");
                    }
                    if block
                        .terminator
                        .as_ref()
                        .is_some_and(|term| std::ptr::eq(term, instruction))
                    {
                        return Err("string-residence-install-terminator");
                    }
                }
                if let Some((dst, kind, operands)) = instruction.text_read_operands_v1() {
                    read::check_text_read(
                        function,
                        source,
                        &results,
                        block.id,
                        instruction,
                        dst,
                        &kind,
                        &operands,
                    )?;
                }
                // A pair cannot be copied, boxed, PHI-merged, returned, stored,
                // passed to arbitrary calls or interpreted as one scalar word.
                let uses = instruction.used_values();
                for value in uses.iter().copied() {
                    if source.is_alias(value) {
                        let actual_uses = uses.iter().filter(|used| **used == value).count();
                        let permitted = if source.permits_alias(block.id, instruction)
                            || matches!(instruction, MirInstruction::KeepAlive { .. })
                        {
                            actual_uses
                        } else if source.permits_operation(block.id, instruction) {
                            match instruction {
                                MirInstruction::StringResidenceInstall {
                                    value: installed,
                                    replaces: Some(old),
                                    ..
                                } if *old == value
                                    && *installed != value
                                    && matches!(source.origin(value), Some(Origin::Slot(_))) =>
                                {
                                    1
                                }
                                row if row.text_read_operands_v1().is_some() =>
                                    row.text_read_operands_v1().expect("matched Text read").2.iter().filter(|operand|
                                        matches!(operand, Operand::TextOperand(used) if *used == value)
                                            && matches!(source.origin(value), Some(Origin::Slot(_)))).count(),
                                MirInstruction::Invoke { operation, .. } => {
                                    let text = text_operands(operation).iter().filter(|operand|
                                        matches!(operand, Operand::TextOperand(used) if *used == value)
                                            && matches!(source.origin(value), Some(Origin::Slot(_)))).count();
                                    let end = usize::from(
                                        matches!(operation,
                                        InvokeOperation::String(Op::EndIfOwned { value: ended }) if *ended == value)
                                            && matches!(
                                                source.origin(value),
                                                Some(Origin::Slot(_))
                                            ),
                                    );
                                    text + end
                                }
                                _ => 0,
                            }
                        } else {
                            0
                        };
                        // used_values preserves occurrences. A Text borrow in
                        // one operand cannot authorize an integer/base/frame
                        // operand that happens to reuse the same ValueId.
                        if permitted != actual_uses {
                            return Err("string-owned-escape");
                        }
                    }
                    if !results.contains_key(&value) {
                        continue;
                    }
                    let allowed = matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::String(operation), .. }
                    if matches!(operation, Op::End { value: end } if *end == value)
                        || operation.operands().iter().any(|operand|
                            matches!(operand, Operand::OwnedString(owned) if *owned == value)));
                    let allowed = allowed
                        || instruction.text_read_operands_v1().is_some_and(|(_, _, operands)| operands.iter()
                            .any(|operand| matches!(operand, Operand::OwnedString(used) if *used == value)));
                    let allowed = allowed
                        || matches!(instruction, MirInstruction::Invoke { operation, .. }
                    if text_operands(operation).iter().any(|operand| matches!(operand, Operand::OwnedString(owned) if *owned == value)));
                    // A typed physical Return may transfer only an installed Home;
                    // the path check below requires that exact live pair on Normal.
                    let allowed = allowed
                        || matches!(instruction, MirInstruction::Invoke {
                    operation: InvokeOperation::FieldSet { mode, value: stored, .. }, .. }
                    if mode.has_owned_text_candidate_v1() && *stored == value);
                    let allowed = allowed
                        || matches!(instruction, MirInstruction::Return { value: Some(returned) }
                    if *returned == value && function.signature.return_type == crate::mir::MirType::String);
                    // KeepAlive is optimizer metadata, checked for SSA availability
                    // by the shared verifier; it performs no runtime pair access.
                    let allowed = allowed
                        || matches!(instruction, MirInstruction::KeepAlive { .. })
                        || matches!(instruction, MirInstruction::StringResidenceInstall { value: installed, replaces, .. } if *installed == value || *replaces == Some(value));
                    let allowed = allowed
                        || matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::Call { call,
                        result: crate::mir::instruction::InvokeCallResultKind::OwnedString }, .. }
                        if call.text_argument_cleanup.contains(&value));
                    let allowed = allowed || source.permits_alias(block.id, instruction);
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
        let mut seen = BTreeMap::<(BasicBlockId, bool), Vec<Live>>::new();
        let mut pending = vec![(function.entry_block, Live::default(), false)];
        while let Some((id, mut live, faulted)) = pending.pop() {
            site = id;
            let previous = seen.entry((id, faulted)).or_default();
            if previous.contains(&live) {
                continue;
            }
            if previous
                .iter()
                .any(|old| !old.compatible_join(&live, source))
            {
                return Err("string-lifetime-join");
            }
            previous.push(live.clone());
            let block = function.blocks.get(&id).ok_or("string-block-missing")?;
            for row in &block.instructions {
                if let Some((_, _, operands)) = row.text_read_operands_v1() {
                    live.check_text_reads(&operands, faulted, source)?;
                }
                if let MirInstruction::StringResidenceInstall {
                    value,
                    replaces,
                    returning,
                } = row
                {
                    if *returning {
                        if faulted
                            || live.hold.is_some()
                            || live.replacement.is_some()
                            || replaces.is_some()
                            || !live.temporary.is_empty()
                            || !live.has_home(*value)
                        {
                            return Err("string-residence-install-not-newest-temporary");
                        }
                        live.hold = Some(*value);
                        continue;
                    }
                    if live.hold.is_some()
                        || live.call_pending.is_some()
                        || faulted
                        || live.replacement.is_some()
                        || !results.contains_key(value)
                        || live.has_home(*value)
                        || live.temporary.pop() != Some(*value)
                    {
                        return Err("string-residence-install-not-newest-temporary");
                    }
                    if let Some(previous) = replaces {
                        if *previous == *value
                            || !(live.has_home(*previous)
                                || (source.permits_operation(id, row)
                                    && live.descriptor_live(*previous, source)))
                        {
                            return Err("string-residence-replacement-not-live-home");
                        }
                        live.replacement = Some((*value, *previous));
                    }
                    live.homes.push(Residence::Home(*value));
                }
            }
            if matches!(
                block.terminator,
                Some(MirInstruction::StringResidenceInstall { .. })
            ) {
                return Err("string-residence-install-terminator");
            }
            let term = block
                .terminator
                .as_ref()
                .ok_or("string-terminator-missing")?;
            if live.replacement.is_some()
                && matches!(term, MirInstruction::Invoke { operation, .. }
            if !matches!(operation, InvokeOperation::String(Op::End { .. } | Op::EndIfOwned { .. })))
            {
                return Err("string-effect-before-replacement-end");
            }
            let displaced_replacement = live.replacement;
            if let MirInstruction::Invoke { operation, .. } = term {
                live.check_text_reads(&text_operands(operation), faulted, source)?;
                if let InvokeOperation::String(Op::EndIfOwned { value }) = operation {
                    if !source.permits_operation(id, term)
                        || !live.end_carrier(*value, faulted, source)
                    {
                        return Err("string-end-order-or-not-live");
                    }
                }
                if let InvokeOperation::String(Op::End { value }) = operation {
                    let before =
                        crate::config::env::builder_debug_enabled().then(|| format!("{live:?}"));
                    if !live.end(*value, faulted, source.retained_for(id, term), source) {
                        return Err(domain_refusal(
                            "string-end-order-or-not-live",
                            &id,
                            &(operation, faulted, before),
                        ));
                    }
                }
            }
            if let MirInstruction::Invoke {
                operation: InvokeOperation::Call { call, .. },
                ..
            } = term
            {
                if live.replacement.is_some() || live.hold.is_some() || live.call_pending.is_some()
                {
                    return Err("string-effect-before-replacement-end");
                }
                if call.text_argument_cleanup.len() > live.temporary.len()
                    || !call
                        .text_argument_cleanup
                        .iter()
                        .rev()
                        .eq(live.temporary[live
                            .temporary
                            .len()
                            .saturating_sub(call.text_argument_cleanup.len())..]
                            .iter())
                {
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
                    if function.signature.return_type != crate::mir::MirType::String
                        || faulted
                        || live.replacement.is_some()
                        || !live.temporary.is_empty()
                        || live
                            .homes
                            .iter()
                            .copied()
                            .filter(|home| live.active(*home))
                            .collect::<Vec<_>>()
                            != [Residence::Home(*value)]
                        || live.hold.is_some_and(|held| held != *value)
                    {
                        return Err("string-owned-escape");
                    }
                    live.homes.retain(|home| *home != Residence::Home(*value));
                    live.hold = None;
                }
            }
            let targets = block.successors_from_terminator();
            if targets.is_empty() && !live.is_empty() {
                outstanding = Some(format!("outstanding={live:?}"));
                let preds: Vec<_> = function
                    .blocks
                    .iter()
                    .filter(|(_, other)| other.successors_from_terminator().contains(&id))
                    .map(|(pred, other)| (pred, &other.instructions, &other.terminator))
                    .collect();
                return Err(domain_refusal(
                    "string-missing-end",
                    &id,
                    &(&block.instructions, term, preds),
                ));
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
                    // A carrier replaced on Normal passes its slot to the new Home
                    // (older than Homes acquired later); a Fault of that End keeps
                    // acquisition order.
                    if let (
                        true,
                        false,
                        Some((new, old)),
                        InvokeOperation::String(Op::EndIfOwned { value }),
                    ) = (
                        target == *normal_landing,
                        faulted,
                        displaced_replacement,
                        operation,
                    ) {
                        if *value == old {
                            if let Some(Origin::Slot(slot)) = source.origin(old) {
                                if let Some(index) = next
                                    .homes
                                    .iter()
                                    .position(|home| *home == Residence::Carrier(slot))
                                {
                                    if next.homes.last() == Some(&Residence::Home(new)) {
                                        next.homes.pop();
                                        next.homes[index] = Residence::Home(new);
                                    }
                                }
                            }
                        }
                    }
                    // Source corroboration remains mandatory in the module checker.
                    // Initialize transfers only on Normal. Text Replace takes
                    // the candidate at entry and disposes it internally on a
                    // precommit Fault; a committed slot retains it on old-End Fault.
                    if let InvokeOperation::FieldSet { mode, value, .. } = operation {
                        if mode.has_owned_text_candidate_v1() {
                            if faulted
                                || live.replacement.is_some()
                                || live.hold.is_some()
                                || live.call_pending.is_some()
                                || live.temporary.last() != Some(value)
                            {
                                return Err("string-borrow-not-live");
                            }
                            if target == *normal_landing
                                || mode.consumes_text_candidate_on_fault_v1()
                            {
                                next.temporary.pop();
                            }
                        }
                    }
                    if target == *normal_landing
                        && operation.normal_result_kind()
                            == Some(InvokeNormalResultKind::OwnedString)
                    {
                        let result = *producers.get(&id).ok_or("string-result-missing")?;
                        if next.contains(&result) {
                            return Err("string-result-duplicate");
                        }
                        let suffix = match operation {
                            InvokeOperation::Call { call, .. } => call.text_argument_cleanup.len(),
                            _ => 0,
                        };
                        next.temporary.insert(next.temporary.len() - suffix, result);
                        if suffix != 0 {
                            next.call_pending = Some(result);
                        }
                    }
                }
                if !next.edge(id, target, source) {
                    return Err("string-lifetime-join");
                }
                pending.push((target, next, next_faulted));
            }
        }
        if function.blocks.values().any(|block| {
            block
                .instructions
                .iter()
                .any(|row| matches!(row, MirInstruction::StringResidenceInstall { .. }))
                && !seen.keys().any(|(id, _)| *id == block.id)
        }) {
            return Err("string-residence-install-unreachable");
        }
        Ok(())
    })();
    result.map_err(|reason| (site, reason, outstanding))
}

/// Default-off: name the operation behind a static string-domain refusal.
fn domain_refusal(
    reason: &'static str,
    block: &BasicBlockId,
    operation: &impl std::fmt::Debug,
) -> &'static str {
    if crate::config::env::builder_debug_enabled() {
        eprintln!("[string-domain] {reason} block={block:?} operation={operation:?}");
    }
    reason
}
