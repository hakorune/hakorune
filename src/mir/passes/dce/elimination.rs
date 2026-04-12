use super::local_fields::{
    analyze_local_reads, collect_overwritten_local_field_sets,
    is_removable_effect_sensitive_read_instruction,
    is_removable_effect_sensitive_write_instruction,
};
use super::memory::{
    analyze_private_carriers, collect_overwritten_private_stores,
    is_removable_effect_sensitive_memory_read_instruction,
};
use super::{is_removable_no_dst_pure_instruction, propagate_used_values};
use crate::mir::{MirFunction, ValueId};
use crate::runtime::get_global_ring0;
use std::collections::HashSet;

fn seed_control_anchor_values(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    base_used_values: &mut HashSet<ValueId>,
) {
    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }
        // Branch/Jump/Return are routed into `block.terminator` by BasicBlock and
        // should not rely on legacy instruction-list seeding here.
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

pub(super) fn eliminate_dead_code_in_function(function: &mut MirFunction) -> usize {
    let reachable_blocks = crate::mir::verification::utils::compute_reachable_blocks(function);
    let local_reads = analyze_local_reads(function, &reachable_blocks);
    let private_carriers = analyze_private_carriers(function, &reachable_blocks, &local_reads);
    let overwritten_local_writes =
        collect_overwritten_local_field_sets(function, &reachable_blocks, &local_reads);
    let overwritten_private_stores =
        collect_overwritten_private_stores(function, &reachable_blocks, &private_carriers);

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
            if is_removable_effect_sensitive_read_instruction(instruction, &local_reads) {
                continue;
            }
            if is_removable_effect_sensitive_memory_read_instruction(instruction, &private_carriers)
            {
                continue;
            }
            if is_removable_effect_sensitive_write_instruction(instruction, &local_reads) {
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

    let mut eliminated = 0usize;
    let dce_trace = std::env::var("NYASH_DCE_TRACE").ok().as_deref() == Some("1");
    for (bbid, block) in &mut function.blocks {
        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let mut kept_insts = Vec::with_capacity(insts.len());
        let mut kept_spans = Vec::with_capacity(spans.len());
        for (idx, (inst, span)) in insts.into_iter().zip(spans.into_iter()).enumerate() {
            let mut keep = true;
            let removable_local_read = reachable_blocks.contains(&bbid)
                && is_removable_effect_sensitive_read_instruction(&inst, &local_reads);
            if keep && removable_local_read {
                if let Some(dst) = inst.dst_value() {
                    if !used_values.contains(&dst) {
                        if dce_trace {
                            get_global_ring0().log.debug(&format!(
                                "[dce] Eliminating removable local read in bb{}: {:?}",
                                bbid.0, inst
                            ));
                        }
                        eliminated += 1;
                        keep = false;
                    }
                }
            }
            let removable_memory_read = reachable_blocks.contains(&bbid)
                && is_removable_effect_sensitive_memory_read_instruction(&inst, &private_carriers);
            if keep && removable_memory_read {
                if let Some(dst) = inst.dst_value() {
                    if !used_values.contains(&dst) {
                        if dce_trace {
                            get_global_ring0().log.debug(&format!(
                                "[dce] Eliminating removable private-carrier read in bb{}: {:?}",
                                bbid.0, inst
                            ));
                        }
                        eliminated += 1;
                        keep = false;
                    }
                }
            }
            let removable_local_write = reachable_blocks.contains(&bbid)
                && is_removable_effect_sensitive_write_instruction(&inst, &local_reads);
            if keep && removable_local_write {
                if dce_trace {
                    get_global_ring0().log.debug(&format!(
                        "[dce] Eliminating removable local write in bb{}: {:?}",
                        bbid.0, inst
                    ));
                }
                eliminated += 1;
                keep = false;
            }
            let removable_overwritten_local_write = reachable_blocks.contains(&bbid)
                && overwritten_local_writes.contains(&(*bbid, idx));
            if keep && removable_overwritten_local_write {
                if dce_trace {
                    get_global_ring0().log.debug(&format!(
                        "[dce] Eliminating overwritten local write in bb{}: {:?}",
                        bbid.0, inst
                    ));
                }
                eliminated += 1;
                keep = false;
            }
            let removable_overwritten_private_store = reachable_blocks.contains(&bbid)
                && overwritten_private_stores.contains(&(*bbid, idx));
            if keep && removable_overwritten_private_store {
                if dce_trace {
                    get_global_ring0().log.debug(&format!(
                        "[dce] Eliminating overwritten private-carrier store in bb{}: {:?}",
                        bbid.0, inst
                    ));
                }
                eliminated += 1;
                keep = false;
            }
            if keep && inst.effects().is_pure() {
                if reachable_blocks.contains(&bbid) {
                    if let crate::mir::MirInstruction::KeepAlive { values } = &inst {
                        if values.iter().all(|value| base_used_values.contains(value)) {
                            if dce_trace {
                                get_global_ring0().log.debug(&format!(
                                    "[dce] Eliminating redundant KeepAlive in bb{}: {:?}",
                                    bbid.0, inst
                                ));
                            }
                            eliminated += 1;
                            keep = false;
                        }
                    }
                }
                if keep && inst.dst_value().is_none() && is_removable_no_dst_pure_instruction(&inst)
                {
                    if dce_trace {
                        get_global_ring0().log.debug(&format!(
                            "[dce] Eliminating removable no-dst pure instruction in bb{}: {:?}",
                            bbid.0, inst
                        ));
                    }
                    eliminated += 1;
                    keep = false;
                }
                if let Some(dst) = inst.dst_value() {
                    if !used_values.contains(&dst) {
                        if dce_trace {
                            get_global_ring0().log.debug(&format!(
                                "[dce] Eliminating unused pure instruction in bb{}: %{} = {:?}",
                                bbid.0, dst.0, inst
                            ));
                        }
                        eliminated += 1;
                        keep = false;
                    }
                }
            }
            if keep {
                kept_insts.push(inst);
                kept_spans.push(span);
            }
        }
        block.instructions = kept_insts;
        block.instruction_spans = kept_spans;
    }

    let pruned_blocks = function.prune_unreachable_blocks();
    if dce_trace && pruned_blocks > 0 {
        get_global_ring0().log.debug(&format!(
            "[dce] Pruned {} unreachable block(s) after liveness elimination",
            pruned_blocks
        ));
    }

    eliminated
}
