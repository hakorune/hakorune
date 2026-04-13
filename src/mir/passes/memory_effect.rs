/*!
 * Memory-effect layer owner seam.
 *
 * This pass owns the current memory-sensitive cleanup slices that were
 * previously buried inside the DCE lane. The first cut keeps the landed
 * private-carrier load/store pruning behavior but gives it a dedicated
 * owner and stats surface so the optimizer can schedule it independently.
 */

use crate::mir::optimizer_stats::OptimizationStats;
use crate::mir::passes::dce::local_fields::analyze_local_reads;
use crate::mir::passes::dce::memory::{
    analyze_private_carriers, collect_overwritten_private_stores,
    is_removable_effect_sensitive_memory_read_instruction,
};
use crate::mir::{MirFunction, MirModule, ValueId};
use std::collections::HashSet;

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

    let mut eliminated = 0usize;
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
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirModule, MirType, ValueId,
    };

    #[test]
    fn memory_effect_prunes_dead_load_from_private_carrier_root() {
        let mut module = MirModule::new("memory_effect_test".to_string());

        let sig = FunctionSignature {
            name: "test/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId(0));

        let v_box = ValueId(1);
        let v_ptr = ValueId(2);
        let v_loaded = ValueId(3);

        {
            let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
            bb0.instructions.push(MirInstruction::NewBox {
                dst: v_box,
                box_type: "Point".to_string(),
                args: vec![],
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::RefNew {
                dst: v_ptr,
                box_val: v_box,
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::Load {
                dst: v_loaded,
                ptr: v_ptr,
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.set_terminator(MirInstruction::Return { value: None });
        }

        module.add_function(func);

        let stats = apply(&mut module);
        assert_eq!(stats.memory_effect_optimizations, 1);

        let func = module.get_function("test/0").unwrap();
        let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
        assert!(bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::RefNew { dst, .. } if *dst == v_ptr)));
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Load { dst, .. } if *dst == v_loaded)));
    }

    #[test]
    fn memory_effect_prunes_overwritten_store_on_private_carrier_root() {
        let mut module = MirModule::new("memory_effect_test".to_string());

        let sig = FunctionSignature {
            name: "test/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId(0));

        let v_box = ValueId(1);
        let v_ptr = ValueId(2);
        let v_value1 = ValueId(3);
        let v_value2 = ValueId(4);

        {
            let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
            bb0.instructions.push(MirInstruction::NewBox {
                dst: v_box,
                box_type: "Point".to_string(),
                args: vec![],
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::RefNew {
                dst: v_ptr,
                box_val: v_box,
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::Const {
                dst: v_value1,
                value: ConstValue::Integer(7),
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::Const {
                dst: v_value2,
                value: ConstValue::Integer(9),
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::Store {
                value: v_value1,
                ptr: v_ptr,
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::Store {
                value: v_value2,
                ptr: v_ptr,
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.set_terminator(MirInstruction::Return { value: None });
        }

        module.add_function(func);

        let stats = apply(&mut module);
        assert_eq!(stats.memory_effect_optimizations, 1);

        let func = module.get_function("test/0").unwrap();
        let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
        assert!(bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Store { value, .. } if *value == v_value2)));
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Store { value, .. } if *value == v_value1)));
    }
}
