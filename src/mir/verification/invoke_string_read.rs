//! Read-only Text validation shared by ordinary rows and Invoke effects.
use super::*;

impl Live {
    pub(super) fn check_text_reads(
        &self,
        operands: &[&Operand],
        faulted: bool,
        source: &TextSourceImage,
    ) -> Result<(), &'static str> {
        if !operands.is_empty()
            && (faulted
                || self.replacement.is_some()
                || self.hold.is_some()
                || self.call_pending.is_some())
        {
            return Err("string-effect-after-fault");
        }
        for operand in operands {
            if matches!(operand, Operand::TextOperand(value) if !self.descriptor_live(*value, source))
                || matches!(operand, Operand::OwnedString(value) if !self.contains(value))
            {
                return Err("string-borrow-not-live");
            }
        }
        Ok(())
    }
}

pub(super) fn check_domains(
    function: &MirFunction,
    source: &TextSourceImage,
    results: &BTreeMap<ValueId, BasicBlockId>,
    id: BasicBlockId,
    instruction: &MirInstruction,
    operands: &[&Operand],
) -> Result<(), &'static str> {
    for operand in operands {
        match operand {
            Operand::TextOperand(value)
                if !source.permits_operation(id, instruction)
                    || !matches!(source.origin(*value), Some(Origin::Slot(_))) =>
            {
                if crate::config::env::builder_debug_enabled() {
                    eprintln!(
                        "[string-domain] text-operand value={value:?} permits={} origin_slot={}",
                        source.permits_operation(id, instruction),
                        matches!(source.origin(*value), Some(Origin::Slot(_)))
                    );
                }
                return Err(domain_refusal(
                    "string-text-operand-domain",
                    &id,
                    instruction,
                ));
            }
            Operand::OwnedString(value) if !results.contains_key(value) => {
                return Err("string-owned-result-missing");
            }
            Operand::BorrowedText(value) => {
                check_borrowed_operand(function, *value)?;
                check_available(function, id, instruction, *value)?
            }
            Operand::NormalBool(value) if has_length_origin(function, *value) => {
                return Err("string-bool-domain");
            }
            Operand::NormalInteger(value)
                if results.contains_key(value)
                    || check_borrowed_operand(function, *value).is_ok() =>
            {
                return Err(domain_refusal("string-integer-domain", &id, instruction));
            }
            Operand::Dynamic(value) if !is_tagged_dynamic_formal(function, *value) => {
                return Err(domain_refusal(
                    "string-dynamic-source-domain",
                    &id,
                    instruction,
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

pub(super) fn check_length(
    function: &MirFunction,
    source: &TextSourceImage,
    results: &BTreeMap<ValueId, BasicBlockId>,
    id: BasicBlockId,
    instruction: &MirInstruction,
    dst: ValueId,
    value: &Operand,
) -> Result<(), &'static str> {
    if !value.is_text() {
        return Err("string-text-operand-domain");
    }
    if function
        .metadata
        .value_types
        .get(&dst)
        .is_some_and(|ty| *ty != crate::mir::MirType::Integer)
    {
        return Err("string-length-result-type");
    }
    if function.blocks.get(&id).and_then(|b| b.terminator.as_ref()) == Some(instruction) {
        return Err("string-length-terminator");
    }
    check_domains(function, source, results, id, instruction, &[value])?;
    if let Some(value) = value.value() {
        check_available(function, id, instruction, value)?;
    }
    Ok(())
}

/// Ordinary Text reads cannot be moved above their actual definitions, even
/// within one block; generic block dominance alone does not prove that order.
fn check_available<'a>(
    function: &'a MirFunction,
    mut id: BasicBlockId,
    mut at: &'a MirInstruction,
    mut value: ValueId,
) -> Result<(), &'static str> {
    let dom = crate::mir::verification::utils::compute_dominators(function);
    let mut seen = std::collections::BTreeSet::new();
    loop {
        if !seen.insert(value) {
            return Err("string-borrowed-copy-cycle");
        }
        if function.params.contains(&value) {
            return Ok(());
        }
        let mut definitions = function
            .blocks
            .iter()
            .flat_map(|(block, body)| {
                body.all_instructions()
                    .enumerate()
                    .map(move |(index, row)| (*block, index, row))
            })
            .filter(|(_, _, row)| row.dst_value() == Some(value));
        let Some((block, index, row)) = definitions.next() else {
            return Err("string-read-definition-missing");
        };
        if definitions.next().is_some() {
            return Err("string-borrowed-copy-duplicate");
        }
        let use_index = function.blocks[&id]
            .all_instructions()
            .position(|row| std::ptr::eq(row, at))
            .ok_or("string-read-site-missing")?;
        if (block == id && index >= use_index) || (block != id && !dom.dominates(block, id)) {
            return Err("string-read-before-definition");
        }
        if let MirInstruction::Copy { src, .. } = row {
            value = *src;
            id = block;
            at = row;
        } else {
            return Ok(());
        }
    }
}

/// Derive the scalar result from actual definitions, never an absent type hint.
fn has_length_origin(function: &MirFunction, value: ValueId) -> bool {
    fn walk(
        function: &MirFunction,
        value: ValueId,
        seen: &mut std::collections::BTreeSet<ValueId>,
    ) -> bool {
        if !seen.insert(value) {
            return false;
        }
        function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .any(|row| match row {
                MirInstruction::StringLength { dst, .. } => *dst == value,
                MirInstruction::Copy { dst, src } if *dst == value => walk(function, *src, seen),
                MirInstruction::Phi { dst, inputs, .. } if *dst == value => inputs
                    .iter()
                    .any(|(_, incoming)| walk(function, *incoming, seen)),
                _ => false,
            })
    }
    walk(function, value, &mut std::collections::BTreeSet::new())
}

pub(super) fn check_i64_results(function: &MirFunction) -> Result<(), &'static str> {
    use crate::mir::MirType;
    if !function.blocks.values().any(|block| {
        block
            .all_instructions()
            .any(|row| matches!(row, MirInstruction::StringLength { .. }))
    }) {
        return Ok(());
    }

    for row in function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
    {
        if row.dst_value().is_some_and(|dst| {
            has_length_origin(function, dst)
                && function
                    .metadata
                    .value_types
                    .get(&dst)
                    .is_some_and(|ty| *ty != MirType::Integer)
        }) {
            return Err("string-length-result-type");
        }
        if matches!(row, MirInstruction::Branch { condition, .. } if has_length_origin(function, *condition))
        {
            return Err("string-length-result-type");
        }
        if matches!(row, MirInstruction::Return { value: Some(value) }
            if has_length_origin(function, *value) && function.signature.return_type != MirType::Integer)
        {
            return Err("string-length-result-type");
        }
    }
    Ok(())
}
