use super::error::OwnershipSsaErrorV1;
use super::model::{MirOwnershipKindV1, OwnershipFunctionAbiV1};
use crate::mir::{MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

pub(super) fn classify(
    function: &MirFunction,
    abi: &OwnershipFunctionAbiV1,
    managed_string_checked_in_module: bool,
) -> Result<BTreeMap<ValueId, MirOwnershipKindV1>, OwnershipSsaErrorV1> {
    if function.params.len() != abi.parameter_kinds().len() {
        return Err(OwnershipSsaErrorV1::ParameterArity {
            expected: function.params.len(),
            actual: abi.parameter_kinds().len(),
        });
    }

    let mut kinds = BTreeMap::new();
    for (value, kind) in function
        .params
        .iter()
        .copied()
        .zip(abi.parameter_kinds().iter().copied())
    {
        insert_kind(&mut kinds, value, kind)?;
    }

    let mut phis = Vec::new();
    for block in sorted_blocks(function) {
        for instruction in &block.instructions {
            match instruction {
                MirInstruction::InvokeNormalResult { invoke_block, dst } => {
                    let origin = function
                        .blocks
                        .get(invoke_block)
                        .and_then(|b| b.terminator.as_ref());
                    match origin {
                        Some(MirInstruction::Invoke {
                            operation: crate::mir::instruction::InvokeOperation::NewBox { .. },
                            normal_landing,
                            ..
                        }) => {
                            if *normal_landing != block.id {
                                return Err(OwnershipSsaErrorV1::InvalidAcquisition {
                                    block: block.id,
                                });
                            }
                            insert_kind(&mut kinds, *dst, MirOwnershipKindV1::Owned)?;
                        }
                        Some(MirInstruction::Invoke {
                            operation: crate::mir::instruction::InvokeOperation::Shared(operation),
                            normal_landing,
                            ..
                        }) => {
                            use crate::mir::instruction::SharedInvokeOperation;
                            if *normal_landing != block.id {
                                return Err(OwnershipSsaErrorV1::InvalidAcquisition {
                                    block: block.id,
                                });
                            }
                            let kind = match operation {
                                SharedInvokeOperation::New { .. }
                                | SharedInvokeOperation::Clone { .. } => MirOwnershipKindV1::Owned,
                                SharedInvokeOperation::Borrow { .. } => {
                                    MirOwnershipKindV1::Borrowed
                                }
                                SharedInvokeOperation::Release { .. } => {
                                    return Err(OwnershipSsaErrorV1::InvalidAcquisition {
                                        block: block.id,
                                    })
                                }
                            };
                            insert_kind(&mut kinds, *dst, kind)?;
                        }
                        Some(MirInstruction::Invoke {
                            operation,
                            normal_landing,
                            ..
                        }) if managed_string_checked_in_module
                            && abi.owner().is_source()
                            && operation.normal_result_kind()
                                == Some(
                                    crate::mir::instruction::InvokeNormalResultKind::OwnedString,
                                ) =>
                        {
                            if *normal_landing != block.id {
                                return Err(OwnershipSsaErrorV1::InvalidAcquisition {
                                    block: block.id,
                                });
                            }
                            // None is outside the OBJECT ownership domain, not
                            // an unowned String. The same final module must pass
                            // its independent affine String lifetime verifier.
                            insert_kind(&mut kinds, *dst, MirOwnershipKindV1::None)?;
                        }
                        Some(MirInstruction::Invoke { operation, .. })
                            if matches!(
                                operation.normal_result_kind(),
                                Some(
                                    crate::mir::instruction::InvokeNormalResultKind::I64
                                        | crate::mir::instruction::InvokeNormalResultKind::Bool
                                )
                            ) =>
                        {
                            insert_kind(&mut kinds, *dst, MirOwnershipKindV1::None)?;
                        }
                        _ => {
                            return Err(OwnershipSsaErrorV1::InvalidAcquisition { block: block.id })
                        }
                    }
                }
                MirInstruction::Copy { dst, src } if abi.aliases().get(dst) == Some(src) => {
                    insert_kind(&mut kinds, *dst, MirOwnershipKindV1::Owned)?;
                }
                MirInstruction::CopyOwned { dst, .. } => {
                    insert_kind(&mut kinds, *dst, MirOwnershipKindV1::Owned)?;
                }
                MirInstruction::Phi { dst, inputs, .. } => {
                    phis.push((block.id, *dst, inputs.as_slice()));
                }
                _ => {
                    if let Some(dst) = instruction.dst_value() {
                        if abi.owner().is_source()
                            && !matches!(
                                instruction,
                                MirInstruction::Const {
                                    value: crate::mir::ConstValue::Integer(_)
                                        | crate::mir::ConstValue::Bool(_)
                                        | crate::mir::ConstValue::Void
                                        | crate::mir::ConstValue::Null,
                                    ..
                                } | MirInstruction::BinOp { .. }
                                    | MirInstruction::UnaryOp { .. }
                                    | MirInstruction::Compare { .. }
                                    | MirInstruction::NullCompare { .. }
                                    | MirInstruction::Copy { .. }
                                    | MirInstruction::FaultFrameEnter { .. }
                                    | MirInstruction::ObjectFieldGet { .. }
                            )
                        {
                            return Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction {
                                block: block.id,
                            });
                        }
                        insert_kind(&mut kinds, dst, MirOwnershipKindV1::None)?;
                    }
                }
            }
        }
    }

    for (alias, source) in abi.aliases() {
        let copies = function.blocks.values().flat_map(|b| &b.instructions)
            .filter(|instruction| matches!(instruction, MirInstruction::Copy { dst, src } if dst == alias && src == source)).count();
        if copies != 1
            || kinds.get(source) != Some(&MirOwnershipKindV1::Owned)
            || abi.aliases().contains_key(source)
        {
            return Err(OwnershipSsaErrorV1::InvalidSourceAlias { value: *alias });
        }
    }
    for value in abi.initially_published() {
        if kinds.get(value) != Some(&MirOwnershipKindV1::Owned) || abi.aliases().contains_key(value)
        {
            return Err(OwnershipSsaErrorV1::InvalidSourceAlias { value: *value });
        }
    }

    for _ in 0..=phis.len() {
        let mut changed = false;
        for (_, dst, inputs) in &phis {
            if kinds.contains_key(dst) {
                continue;
            }
            let known = inputs
                .iter()
                .filter_map(|(_, value)| kinds.get(value).copied())
                .collect::<Vec<_>>();
            if let Some(first) = known.first().copied() {
                if known.iter().any(|kind| *kind != first) {
                    return Err(OwnershipSsaErrorV1::PhiKindMismatch {
                        block: phis.iter().find(|(_, d, _)| d == dst).unwrap().0,
                        dst: *dst,
                    });
                }
                kinds.insert(*dst, first);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    for (block, dst, inputs) in phis {
        let Some(kind) = kinds.get(&dst).copied() else {
            return Err(OwnershipSsaErrorV1::UnknownValueKind { value: dst });
        };
        for (_, value) in inputs {
            let Some(input_kind) = kinds.get(value).copied() else {
                return Err(OwnershipSsaErrorV1::UnknownValueKind { value: *value });
            };
            if input_kind != kind {
                return Err(OwnershipSsaErrorV1::PhiKindMismatch { block, dst });
            }
        }
        if kind == MirOwnershipKindV1::Borrowed {
            return Err(OwnershipSsaErrorV1::BorrowedPhiForbidden { block, dst });
        }
    }
    Ok(kinds)
}

/// A raw field view is not a Home. Prove its complete nonescaping read closure
/// from public definitions and one readonly block interval, without type hints
/// or generator loans. The caller still checks the owned root's current state.
pub(super) fn local_field_borrow(
    function: &crate::mir::verification::geometry::FunctionVerificationGeometry<'_>,
    module: &crate::mir::MirModule,
    block: crate::mir::BasicBlockId,
    value: ValueId,
    consuming: hakorune_mir_defs::CanonicalFieldRefV1,
) -> Result<
    (
        ValueId,
        hakorune_mir_defs::CanonicalObjectIdV1,
        Vec<ValueId>,
    ),
    OwnershipSsaErrorV1,
> {
    let invalid = || OwnershipSsaErrorV1::InvalidObjectLifecycle { block, value };
    module
        .validate_object_definition_membership()
        .map_err(|_| invalid())?;
    let mut pending = vec![value];
    let mut views = Vec::new();
    let mut root = None;
    while let Some(view) = pending.pop() {
        if views.contains(&view) {
            continue;
        }
        if function.params.contains(&view) {
            return Err(invalid());
        }
        let (origin, index, producer) = function.definition(view).ok_or_else(invalid)?;
        if function.ambiguous(view)
            || origin != block
            || index >= function.blocks[&block].instructions.len()
        {
            return Err(invalid());
        }
        let MirInstruction::ObjectFieldGet { base, field, .. } = producer else {
            return Err(invalid());
        };
        let child = nonnullable_field_child(module, *field).ok_or_else(invalid)?;
        if view == value && consuming.object() != child {
            return Err(invalid());
        }
        if function.ambiguous(*base) {
            return Err(invalid());
        }
        let parent = function.definition(*base);
        if let Some((
            parent_block,
            parent_index,
            MirInstruction::ObjectFieldGet {
                field: parent_field,
                ..
            },
        )) = parent
        {
            if parent_block != block
                || parent_index >= index
                || nonnullable_field_child(module, *parent_field) != Some(field.object())
            {
                return Err(invalid());
            }
            pending.push(*base);
        } else {
            let candidate = (*base, field.object());
            if root.is_some_and(|root| root != candidate) {
                return Err(invalid());
            }
            root = Some(candidate);
        }
        let mut last = index;
        for (use_block, rows) in &function.blocks {
            for (use_index, row) in rows.all_instructions().enumerate() {
                if !row.used_values().contains(&view) {
                    continue;
                }
                if *use_block != block
                    || use_index <= index
                    || use_index >= function.blocks[&block].instructions.len()
                {
                    return Err(invalid());
                }
                let MirInstruction::ObjectFieldGet { dst, base, field } = row else {
                    return Err(invalid());
                };
                if *base != view || field.object() != child {
                    return Err(invalid());
                }
                if !integer_field(module, *field) {
                    nonnullable_field_child(module, *field).ok_or_else(invalid)?;
                    pending.push(*dst);
                }
                last = last.max(use_index);
            }
        }
        for row in &function.blocks[&block].instructions[index + 1..last + 1] {
            let readonly = match row {
                MirInstruction::Const {
                    value:
                        crate::mir::ConstValue::Integer(_)
                        | crate::mir::ConstValue::Bool(_)
                        | crate::mir::ConstValue::Null
                        | crate::mir::ConstValue::Void,
                    ..
                }
                | MirInstruction::BinOp { .. }
                | MirInstruction::UnaryOp { .. }
                | MirInstruction::Compare { .. }
                | MirInstruction::NullCompare { .. }
                | MirInstruction::Copy { .. } => true,
                MirInstruction::ObjectFieldGet { field, .. } => {
                    integer_field(module, *field)
                        || nonnullable_field_child(module, *field).is_some()
                }
                _ => false,
            };
            if !readonly {
                return Err(invalid());
            }
        }
        views.push(view);
    }
    let (root, object) = root.ok_or_else(invalid)?;
    Ok((root, object, views))
}

fn nonnullable_field_child(
    module: &crate::mir::MirModule,
    field: hakorune_mir_defs::CanonicalFieldRefV1,
) -> Option<hakorune_mir_defs::CanonicalObjectIdV1> {
    let definition = module.canonical_field_definition(field)?;
    if definition.is_weak {
        return None;
    }
    let ty = crate::mir::declared_type_storage::declared_object_field_type_v1(
        definition.declared_type_name.as_deref()?,
    )?;
    if ty.explicitly_nullable() {
        return None;
    }
    let child = module.canonical_object_field_child_v1(field)?;
    (module.canonical_object_definition(child)?.diagnostic_name() == ty.class_name())
        .then_some(child)
}

fn integer_field(
    module: &crate::mir::MirModule,
    field: hakorune_mir_defs::CanonicalFieldRefV1,
) -> bool {
    module
        .canonical_field_definition(field)
        .is_some_and(|field| {
            !field.is_weak
                && field
                    .declared_type_name
                    .as_deref()
                    .is_some_and(crate::mir::numeric_substrate::is_numeric_integer_type_name)
        })
}

fn insert_kind(
    kinds: &mut BTreeMap<ValueId, MirOwnershipKindV1>,
    value: ValueId,
    kind: MirOwnershipKindV1,
) -> Result<(), OwnershipSsaErrorV1> {
    if let Some(first) = kinds.insert(value, kind) {
        if first != kind {
            return Err(OwnershipSsaErrorV1::KindConflict {
                value,
                first,
                second: kind,
            });
        }
    }
    Ok(())
}

fn sorted_blocks(function: &MirFunction) -> Vec<&crate::mir::BasicBlock> {
    let mut blocks = function.blocks.values().collect::<Vec<_>>();
    blocks.sort_by_key(|block| block.id.0);
    blocks
}
