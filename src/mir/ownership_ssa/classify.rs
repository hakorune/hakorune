use super::error::OwnershipSsaErrorV1;
use super::model::{MirOwnershipKindV1, OwnershipFunctionAbiV1};
use crate::mir::{MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

pub(super) fn classify(
    function: &MirFunction,
    abi: &OwnershipFunctionAbiV1,
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
                MirInstruction::CopyOwned { dst, .. } => {
                    insert_kind(&mut kinds, *dst, MirOwnershipKindV1::Owned)?;
                }
                MirInstruction::Phi { dst, inputs, .. } => {
                    phis.push((block.id, *dst, inputs.as_slice()));
                }
                _ => {
                    if let Some(dst) = instruction.dst_value() {
                        insert_kind(&mut kinds, dst, MirOwnershipKindV1::None)?;
                    }
                }
            }
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
