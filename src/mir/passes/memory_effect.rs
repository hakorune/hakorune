/*!
 * Memory-effect layer owner seam.
 *
 * This pass owns the current memory-sensitive cleanup slices that were
 * previously buried inside the DCE lane. The landed cuts keep the
 * private-carrier load/store pruning behavior and add same-block
 * store-to-load forwarding, same-block redundant load elimination, and
 * immediate-successor overwritten-store pruning behind a dedicated
 * owner and stats surface so the optimizer can schedule it
 * independently.
 */

use crate::mir::optimizer_stats::OptimizationStats;
use crate::mir::passes::dce::local_fields::analyze_local_reads;
use crate::mir::passes::dce::memory::{
    analyze_private_carriers, collect_overwritten_private_stores,
    is_removable_effect_sensitive_memory_read_instruction,
};
use crate::mir::{MirFunction, MirModule, ValueId};
use std::collections::{HashMap, HashSet};

pub fn apply(module: &mut MirModule) -> OptimizationStats {
    let mut stats = OptimizationStats::new();
    for function in module.functions.values_mut() {
        stats.memory_effect_optimizations += eliminate_memory_effect_in_function(function);
    }
    stats
}

fn eliminate_memory_effect_in_function(function: &mut MirFunction) -> usize {
    let reachable_blocks = crate::mir::verification::utils::compute_reachable_blocks(function);
    let local_reads = analyze_local_reads(function, &reachable_blocks);
    let private_carriers = analyze_private_carriers(function, &reachable_blocks, &local_reads);
    let overwritten_private_stores =
        collect_overwritten_private_stores(function, &reachable_blocks, &private_carriers);
    let cross_block_overwritten_private_stores = collect_cross_block_overwritten_private_stores(
        function,
        &reachable_blocks,
        &private_carriers,
    );
    let forwarded_same_block_loads =
        forward_same_block_private_carrier_loads(function, &reachable_blocks, &private_carriers);

    let mut base_used_values: HashSet<ValueId> = HashSet::new();
    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }
        for (idx, instruction) in block.instructions.iter().enumerate() {
            if matches!(instruction, crate::mir::MirInstruction::KeepAlive { .. }) {
                continue;
            }
            if overwritten_private_stores.contains(&(*bid, idx)) {
                continue;
            }
            if is_removable_effect_sensitive_memory_read_instruction(instruction, &private_carriers)
            {
                continue;
            }
            if !instruction.effects().is_pure() {
                if let Some(dst) = instruction.dst_value() {
                    base_used_values.insert(dst);
                }
                for u in instruction.used_values() {
                    base_used_values.insert(u);
                }
            }
        }
    }

    seed_control_anchor_values(function, &reachable_blocks, &mut base_used_values);
    propagate_used_values(function, &reachable_blocks, &mut base_used_values);

    let mut used_values = base_used_values.clone();
    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }
        for instruction in &block.instructions {
            if let crate::mir::MirInstruction::KeepAlive { values } = instruction {
                if values.iter().any(|value| !base_used_values.contains(value)) {
                    used_values.extend(values.iter().copied());
                }
            }
        }
    }

    propagate_used_values(function, &reachable_blocks, &mut used_values);

    let mut eliminated = forwarded_same_block_loads;
    for (bbid, block) in &mut function.blocks {
        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let mut kept_insts = Vec::with_capacity(insts.len());
        let mut kept_spans = Vec::with_capacity(spans.len());
        for (idx, (inst, span)) in insts.into_iter().zip(spans.into_iter()).enumerate() {
            let mut keep = true;
            let removable_memory_read = reachable_blocks.contains(&bbid)
                && is_removable_effect_sensitive_memory_read_instruction(&inst, &private_carriers);
            if keep && removable_memory_read {
                if let Some(dst) = inst.dst_value() {
                    if !used_values.contains(&dst) {
                        eliminated += 1;
                        keep = false;
                    }
                }
            }
            let removable_overwritten_private_store = reachable_blocks.contains(&bbid)
                && overwritten_private_stores.contains(&(*bbid, idx));
            if keep && removable_overwritten_private_store {
                eliminated += 1;
                keep = false;
            }
            let removable_cross_block_overwritten_private_store = reachable_blocks.contains(&bbid)
                && cross_block_overwritten_private_stores.contains(&(*bbid, idx));
            if keep && removable_cross_block_overwritten_private_store {
                eliminated += 1;
                keep = false;
            }
            if keep {
                kept_insts.push(inst);
                kept_spans.push(span);
            }
        }
        block.instructions = kept_insts;
        block.instruction_spans = kept_spans;
    }

    eliminated
}

fn forward_same_block_private_carrier_loads(
    function: &mut MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    private_carriers: &crate::mir::passes::dce::memory::PrivateCarrierInfo,
) -> usize {
    let mut forwarded = 0usize;

    for (bid, block) in &mut function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }

        let mut available_value_by_root: HashMap<ValueId, ValueId> = HashMap::new();
        for instruction in &mut block.instructions {
            match instruction {
                crate::mir::MirInstruction::Store { value, ptr } => {
                    if let Some(root) = private_carriers.resolve_private_store_root(*ptr) {
                        available_value_by_root.insert(root, *value);
                    }
                }
                crate::mir::MirInstruction::Load { dst, ptr } => {
                    let Some(root) = private_carriers.resolve_private_store_root(*ptr) else {
                        continue;
                    };
                    if let Some(value) = available_value_by_root.get(&root).copied() {
                        let dst = *dst;
                        *instruction = crate::mir::MirInstruction::Copy { dst, src: value };
                        forwarded += 1;
                    } else {
                        available_value_by_root.insert(root, *dst);
                    }
                }
                _ => {}
            }
        }
    }

    forwarded
}

fn collect_cross_block_overwritten_private_stores(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    private_carriers: &crate::mir::passes::dce::memory::PrivateCarrierInfo,
) -> HashSet<(crate::mir::BasicBlockId, usize)> {
    let mut removable = HashSet::new();

    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }

        let Some(crate::mir::MirInstruction::Jump { target, edge_args }) =
            block.terminator.as_ref()
        else {
            continue;
        };
        if edge_args.is_some() {
            continue;
        }

        let Some(target_block) = function.blocks.get(target) else {
            continue;
        };
        if !reachable_blocks.contains(target) {
            continue;
        }

        let Some(crate::mir::MirInstruction::Store { ptr, .. }) = target_block.instructions.first()
        else {
            continue;
        };
        let Some(target_root) = private_carriers.resolve_private_store_root(*ptr) else {
            continue;
        };

        let mut candidate = None;
        for (idx, instruction) in block.instructions.iter().enumerate().rev() {
            match instruction {
                crate::mir::MirInstruction::Store { ptr, .. } => {
                    let Some(root) = private_carriers.resolve_private_store_root(*ptr) else {
                        break;
                    };
                    if root == target_root {
                        candidate = Some((*bid, idx));
                    }
                    break;
                }
                crate::mir::MirInstruction::Load { ptr, .. } => {
                    if private_carriers.resolve_private_store_root(*ptr) == Some(target_root) {
                        break;
                    }
                }
                crate::mir::MirInstruction::Copy { src, .. } => {
                    if private_carriers.resolve_private_store_root(*src) == Some(target_root) {
                        continue;
                    }
                }
                _ => {
                    if instruction.used_values().into_iter().any(|value| {
                        private_carriers.resolve_private_store_root(value) == Some(target_root)
                    }) {
                        break;
                    }
                }
            }
        }

        if let Some(entry) = candidate {
            removable.insert(entry);
        }
    }

    removable
}

fn seed_control_anchor_values(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    base_used_values: &mut HashSet<ValueId>,
) {
    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }
        if let Some(term) = &block.terminator {
            for u in term.used_values() {
                base_used_values.insert(u);
            }
        }
        for edge in block.out_edges() {
            if let Some(args) = edge.args {
                for u in args.values {
                    base_used_values.insert(u);
                }
            }
        }
    }
}

fn propagate_used_values(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    used_values: &mut HashSet<ValueId>,
) {
    let mut changed = true;
    while changed {
        changed = false;
        for (bid, block) in &function.blocks {
            if !reachable_blocks.contains(bid) {
                continue;
            }
            for instruction in &block.instructions {
                if let Some(dst) = instruction.dst_value() {
                    if used_values.contains(&dst) {
                        for u in instruction.used_values() {
                            if used_values.insert(u) {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
