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
    function: &crate::mir::verification::geometry::FunctionVerificationGeometry<'_>,
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
            Operand::NormalBool(value)
                if has_scalar_read_origin(function, *value, &crate::mir::MirType::Integer) =>
            {
                return Err("string-bool-domain");
            }
            Operand::NormalInteger(value)
                if results.contains_key(value)
                    || has_scalar_read_origin(function, *value, &crate::mir::MirType::Bool)
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

pub(super) fn check_text_read(
    function: &crate::mir::verification::geometry::FunctionVerificationGeometry<'_>,
    source: &TextSourceImage,
    results: &BTreeMap<ValueId, BasicBlockId>,
    id: BasicBlockId,
    instruction: &MirInstruction,
    dst: ValueId,
    kind: &crate::mir::MirType,
    operands: &[&Operand],
) -> Result<(), &'static str> {
    if operands.iter().any(|value| !value.is_text()) {
        return Err("string-text-operand-domain");
    }
    if function
        .metadata
        .value_types
        .get(&dst)
        .is_some_and(|ty| ty != kind)
    {
        return Err(result_type_refusal(kind));
    }
    if function.blocks.get(&id).and_then(|b| b.terminator.as_ref()) == Some(instruction) {
        return Err(if *kind == crate::mir::MirType::Integer {
            "string-length-terminator"
        } else {
            "string-read-bool-terminator"
        });
    }
    check_domains(function, source, results, id, instruction, operands)?;
    for value in operands.iter().filter_map(|operand| operand.value()) {
        check_available(function, id, instruction, value)?;
    }
    Ok(())
}

/// Ordinary Text reads cannot be moved above their actual definitions, even
/// within one block; generic block dominance alone does not prove that order.
pub(in crate::mir::verification::invoke) fn check_available<'a>(
    function: &'a crate::mir::verification::geometry::FunctionVerificationGeometry<'_>,
    mut id: BasicBlockId,
    mut at: &'a MirInstruction,
    mut value: ValueId,
) -> Result<(), &'static str> {
    let dom = function.dominators();
    let mut seen = std::collections::BTreeSet::new();
    loop {
        if !seen.insert(value) {
            return Err("string-borrowed-copy-cycle");
        }
        if function.params.contains(&value) {
            return Ok(());
        }
        let Some((block, index, row)) = function.definition(value) else {
            return Err("string-read-definition-missing");
        };
        if function.ambiguous(value) {
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
pub(in crate::mir::verification) fn has_scalar_read_origin(
    function: &crate::mir::verification::geometry::FunctionVerificationGeometry<'_>,
    value: ValueId,
    kind: &crate::mir::MirType,
) -> bool {
    fn walk(
        function: &crate::mir::verification::geometry::FunctionVerificationGeometry<'_>,
        value: ValueId,
        kind: &crate::mir::MirType,
        seen: &mut std::collections::BTreeSet<ValueId>,
    ) -> bool {
        if !seen.insert(value) {
            return false;
        }
        let Some((_, _, row)) = function.definition(value) else {
            return false;
        };
        // Ambiguous definitions have already failed SSA; never authorize a
        // scalar domain by choosing a convenient duplicate definition.
        if function.ambiguous(value) {
            return false;
        }
        match row {
            row if row
                .scalar_read_result_v1()
                .is_some_and(|(dst, actual)| dst == value && &actual == kind) =>
            {
                true
            }
            MirInstruction::Copy { src, .. } => walk(function, *src, kind, seen),
            MirInstruction::Phi { inputs, .. } => inputs
                .iter()
                .any(|(_, incoming)| walk(function, *incoming, kind, seen)),
            _ => false,
        }
    }
    walk(
        function,
        value,
        kind,
        &mut std::collections::BTreeSet::new(),
    )
}

fn result_type_refusal(kind: &crate::mir::MirType) -> &'static str {
    if *kind == crate::mir::MirType::Integer {
        "string-length-result-type"
    } else {
        "string-read-bool-result-type"
    }
}

pub(super) fn check_scalar_results(
    function: &crate::mir::verification::geometry::FunctionVerificationGeometry<'_>,
) -> Result<(), &'static str> {
    use crate::mir::MirType;
    // Do not walk origins repeatedly for a kind this function never reads.
    let present = [MirType::Integer, MirType::Bool].map(|kind| {
        function.blocks.values().any(|block| {
            block.all_instructions().any(|row| {
                row.scalar_read_result_v1()
                    .is_some_and(|(_, actual)| actual == kind)
            })
        })
    });
    if !present[0] && !present[1] {
        return Ok(());
    }
    for row in function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
    {
        for (enabled, kind) in present.into_iter().zip([MirType::Integer, MirType::Bool]) {
            if !enabled {
                continue;
            }
            let origin = |value| has_scalar_read_origin(function, value, &kind);
            if row.dst_value().is_some_and(|dst| {
                origin(dst)
                    && function
                        .metadata
                        .value_types
                        .get(&dst)
                        .is_some_and(|ty| *ty != kind)
            }) {
                return Err(result_type_refusal(&kind));
            }
            if let MirInstruction::Phi {
                dst,
                inputs,
                type_hint,
            } = row
            {
                if origin(*dst)
                    && (type_hint.as_ref().is_some_and(|ty| *ty != kind)
                        || inputs.iter().any(|(_, value)| {
                            matches!(intrinsic_scalar_type(function, *value,
                        &mut std::collections::BTreeSet::new()), Some(ty) if ty != kind)
                        }))
                {
                    return Err(result_type_refusal(&kind));
                }
            }
            if kind == MirType::Integer
                && matches!(row, MirInstruction::Branch { condition, .. } if origin(*condition))
            {
                return Err(result_type_refusal(&kind));
            }
            if matches!(row, MirInstruction::Return { value: Some(value) }
                if origin(*value) && function.signature.return_type != kind)
            {
                return Err(result_type_refusal(&kind));
            }
        }
        if !present[1] {
            continue;
        }
        // A normal Bool read must not become an integer merely by deleting metadata.
        // Unknown legacy values are not assigned a new type by this check.
        let bool_origin = |v| has_scalar_read_origin(function, v, &MirType::Bool);
        let incompatible = |v| {
            intrinsic_scalar_type(function, v, &mut std::collections::BTreeSet::new())
                .is_some_and(|ty| ty != MirType::Bool)
        };
        match row {
            MirInstruction::BinOp { op, lhs, rhs, .. }
                if bool_origin(*lhs) || bool_origin(*rhs) =>
            {
                if !matches!(op, crate::mir::BinaryOp::And | crate::mir::BinaryOp::Or)
                    || incompatible(*lhs)
                    || incompatible(*rhs)
                {
                    return Err(result_type_refusal(&MirType::Bool));
                }
            }
            MirInstruction::Compare { op, lhs, rhs, .. }
                if bool_origin(*lhs) || bool_origin(*rhs) =>
            {
                if !matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne)
                    || incompatible(*lhs)
                    || incompatible(*rhs)
                {
                    return Err(result_type_refusal(&MirType::Bool));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Independent primitive definitions expose contradictions without upgrading
/// unknown legacy values from cached type metadata. C checks complete PHI kinds.
fn intrinsic_scalar_type(
    function: &crate::mir::verification::geometry::FunctionVerificationGeometry<'_>,
    value: ValueId,
    seen: &mut std::collections::BTreeSet<ValueId>,
) -> Option<crate::mir::MirType> {
    use crate::mir::{ConstValue, MirType};
    if !seen.insert(value) {
        return None;
    }
    if function.ambiguous(value) {
        return None;
    }
    let definition = function.definition(value).map(|(_, _, row)| row);
    match definition {
        Some(
            MirInstruction::StringLength { .. }
            | MirInstruction::StringIndexOf { .. }
            | MirInstruction::Const {
                value: ConstValue::Integer(_),
                ..
            },
        ) => Some(MirType::Integer),
        Some(
            MirInstruction::Compare { .. }
            | MirInstruction::NullCompare { .. }
            | MirInstruction::StringCompare { .. }
            | MirInstruction::Const {
                value: ConstValue::Bool(_),
                ..
            },
        ) => Some(MirType::Bool),
        Some(MirInstruction::Const {
            value: ConstValue::Float(_),
            ..
        }) => Some(MirType::Float),
        Some(MirInstruction::InvokeNormalResult { invoke_block, .. }) => {
            let MirInstruction::Invoke { operation, .. } =
                function.blocks.get(invoke_block)?.terminator.as_ref()?
            else {
                return None;
            };
            match operation.normal_result_kind()? {
                crate::mir::instruction::InvokeNormalResultKind::I64 => Some(MirType::Integer),
                crate::mir::instruction::InvokeNormalResultKind::Bool => Some(MirType::Bool),
                crate::mir::instruction::InvokeNormalResultKind::OwnedString => {
                    Some(MirType::String)
                }
                _ => None,
            }
        }
        Some(MirInstruction::Copy { src, .. }) => intrinsic_scalar_type(function, *src, seen),
        _ => None,
    }
}
