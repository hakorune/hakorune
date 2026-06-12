use std::collections::{BTreeMap, BTreeSet};

use crate::mir::escape_barrier::{classify_escape_uses, EscapeBarrier};
use crate::mir::function::MirFunction;
use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

use super::{push_region_error, MemOpSite};

pub(super) fn check_memop_escape(
    function: &MirFunction,
    sites: &[MemOpSite],
    errors: &mut Vec<VerificationError>,
) {
    let mut produced: BTreeMap<ValueId, (FastMemRegionId, MemOpKind)> = BTreeMap::new();
    for site in sites {
        if !memop_escape_tracked_kind(site.kind) {
            continue;
        }
        if let Some(dst) = site.dst {
            produced.insert(dst, (site.region, site.kind));
        }
    }
    extend_single_input_phi_aliases(function, &mut produced);
    if produced.is_empty() {
        return;
    }

    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, sp) in block.all_spanned_instructions_enumerated() {
            let is_memop = matches!(sp.inst, MirInstruction::MemOp { .. });
            if is_memop {
                continue;
            }
            for escape_use in fastmem_escape_uses(sp.inst) {
                let Some((region, kind)) = produced.get(&escape_use.value).copied() else {
                    continue;
                };
                if allowed_fastmem_escape_use(sp.inst, kind, escape_use.value) {
                    continue;
                }
                push_region_error(
                    function,
                    Some(block_id),
                    Some(instruction_index),
                    Some(region.0),
                    function
                        .metadata
                        .fastmem_regions
                        .iter()
                        .find(|metadata| metadata.id == region)
                        .map(|metadata| metadata.contract.clone()),
                    &format!(
                        "memop-value-escapes kind={} barrier={}",
                        kind.display_name(),
                        escape_use.barrier
                    ),
                    errors,
                );
            }
        }
    }
}

fn extend_single_input_phi_aliases(
    function: &MirFunction,
    produced: &mut BTreeMap<ValueId, (FastMemRegionId, MemOpKind)>,
) {
    let mut changed = true;
    while changed {
        changed = false;
        for block_id in function.block_ids() {
            let Some(block) = function.blocks.get(&block_id) else {
                continue;
            };
            for sp in block.all_spanned_instructions() {
                match sp.inst {
                    MirInstruction::Phi { dst, inputs, .. } => {
                        if produced.contains_key(dst) || inputs.len() != 1 {
                            if produced.contains_key(dst) || inputs.is_empty() {
                                continue;
                            }
                            let Some(origin) = phi_merge_origin(produced, inputs.as_slice()) else {
                                continue;
                            };
                            produced.insert(*dst, origin);
                            changed = true;
                            continue;
                        }
                        let Some((_, input)) = inputs.first() else {
                            continue;
                        };
                        let Some(origin) = produced.get(input).copied() else {
                            continue;
                        };
                        produced.insert(*dst, origin);
                        changed = true;
                    }
                    MirInstruction::Copy { dst, src } => {
                        if produced.contains_key(dst) {
                            continue;
                        }
                        let Some(origin @ (_, kind)) = produced.get(src).copied() else {
                            continue;
                        };
                        if memop_escape_tracked_kind(kind) {
                            produced.insert(*dst, origin);
                            changed = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn phi_merge_origin(
    produced: &BTreeMap<ValueId, (FastMemRegionId, MemOpKind)>,
    inputs: &[(BasicBlockId, ValueId)],
) -> Option<(FastMemRegionId, MemOpKind)> {
    let mut origin: Option<(FastMemRegionId, MemOpKind)> = None;
    for (_, input) in inputs {
        let current = produced.get(input).copied()?;
        if !phi_merge_aliasable_memop_kind(current.1) {
            return None;
        }
        match origin {
            None => origin = Some(current),
            Some(existing) if existing == current => {}
            Some(_) => return None,
        }
    }
    origin
}

fn phi_merge_aliasable_memop_kind(kind: MemOpKind) -> bool {
    matches!(
        kind,
        MemOpKind::TableIndex | MemOpKind::AtomicRemoteHeadDrain
    )
}

#[derive(Debug, Clone, Copy)]
struct FastMemEscapeUse {
    value: ValueId,
    barrier: FastMemEscapeBarrier,
}

#[derive(Debug, Clone, Copy)]
enum FastMemEscapeBarrier {
    Shared(EscapeBarrier),
    OrdinaryUse,
}

impl std::fmt::Display for FastMemEscapeBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shared(barrier) => write!(f, "{}", barrier),
            Self::OrdinaryUse => f.write_str("ordinary_use"),
        }
    }
}

fn fastmem_escape_uses(inst: &MirInstruction) -> Vec<FastMemEscapeUse> {
    let shared = classify_escape_uses(inst);
    let mut uses: Vec<FastMemEscapeUse> = shared
        .iter()
        .map(|use_site| FastMemEscapeUse {
            value: use_site.value,
            barrier: FastMemEscapeBarrier::Shared(use_site.barrier),
        })
        .collect();
    let shared_values: BTreeSet<ValueId> = shared.iter().map(|use_site| use_site.value).collect();
    for value in inst.used_values() {
        if shared_values.contains(&value) {
            continue;
        }
        uses.push(FastMemEscapeUse {
            value,
            barrier: FastMemEscapeBarrier::OrdinaryUse,
        });
    }
    uses
}

fn allowed_fastmem_escape_use(inst: &MirInstruction, kind: MemOpKind, value: ValueId) -> bool {
    matches!(
        (kind, inst),
        (
            MemOpKind::OwnerEq,
            MirInstruction::Branch { condition, .. }
        ) if *condition == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::OwnerEq,
            MirInstruction::Copy { src, .. }
        ) if *src == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::TableIndex,
            MirInstruction::FieldGet { base, .. }
        ) if *base == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::TableIndex,
            MirInstruction::FieldSet { base, .. }
        ) if *base == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::TableIndex,
            MirInstruction::Copy { src, .. }
        ) if *src == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::TableIndex,
            MirInstruction::Call {
                callee: Some(crate::mir::Callee::Extern(name) | crate::mir::Callee::Global(name)),
                args,
                ..
            }
        ) if name.starts_with("mem.") && args.iter().any(|arg| *arg == value)
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::OwnerEq,
            MirInstruction::Call {
                callee: Some(crate::mir::Callee::Extern(name) | crate::mir::Callee::Global(name)),
                args,
                ..
            }
        ) if name.starts_with("mem.") && args.iter().any(|arg| *arg == value)
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::TableIndex,
            MirInstruction::Phi { inputs, .. }
        ) if inputs.iter().any(|(_, input)| *input == value)
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AddrOf,
            MirInstruction::BinOp { lhs, rhs, .. }
        ) if *lhs == value || *rhs == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AddrOf,
            MirInstruction::Compare { lhs, rhs, .. }
        ) if *lhs == value || *rhs == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AddrOf,
            MirInstruction::Copy { src, .. }
        ) if *src == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AtomicRemoteHeadDrain,
            MirInstruction::FieldGet { base, .. }
        ) if *base == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AtomicRemoteHeadDrain,
            MirInstruction::FieldSet { base, .. }
        ) if *base == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AtomicRemoteHeadDrain,
            MirInstruction::Copy { src, .. }
        ) if *src == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AtomicRemoteHeadDrain,
            MirInstruction::Call {
                callee: Some(crate::mir::Callee::Extern(name) | crate::mir::Callee::Global(name)),
                args,
                ..
            }
        ) if name.starts_with("mem.") && args.iter().any(|arg| *arg == value)
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AtomicRemoteHeadDrain,
            MirInstruction::Phi { inputs, .. }
        ) if inputs.iter().any(|(_, input)| *input == value)
    )
}

fn memop_escape_tracked_kind(kind: MemOpKind) -> bool {
    matches!(
        kind,
        MemOpKind::AddrOf
            | MemOpKind::TableIndex
            | MemOpKind::AtomicRemoteHeadDrain
            | MemOpKind::OwnerEq
    )
}
