//! Dead Code Elimination (pure instruction DCE)
//!
//! Extracted from the monolithic optimizer to enable modular pass composition.

use crate::mir::phi_query::collect_passthrough_phi_parents;
use crate::mir::{classify_escape_uses, resolve_value_origin_from_parent_map, ParentMap};
use crate::mir::{MirFunction, MirModule, ValueId};
use crate::runtime::get_global_ring0;
use std::collections::HashSet;

/// Eliminate dead code (unused results of pure instructions) across the module
/// and prune unreachable blocks as structural CFG cleanup.
///
/// This pass also removes pure no-dst calls plus dead field reads and writes on
/// definitely non-escaping local boxes when they are otherwise unused.
///
/// Returns the number of eliminated instructions.
pub fn eliminate_dead_code(module: &mut MirModule) -> usize {
    let mut eliminated_total = 0usize;
    for (_func_name, func) in &mut module.functions {
        eliminated_total += eliminate_dead_code_in_function(func);
    }
    eliminated_total
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

fn is_removable_no_dst_pure_instruction(inst: &crate::mir::MirInstruction) -> bool {
    matches!(
        inst,
        crate::mir::MirInstruction::Safepoint | crate::mir::MirInstruction::Call { dst: None, .. }
    )
}

#[derive(Default)]
struct LocalReadInfo {
    local_boxes: HashSet<ValueId>,
    escaping: HashSet<ValueId>,
    field_read_roots: HashSet<ValueId>,
    alias_parents: ParentMap,
}

impl LocalReadInfo {
    fn is_non_escaping_local(&self, value: ValueId) -> bool {
        let root = resolve_value_origin_from_parent_map(value, &self.alias_parents);
        self.local_boxes.contains(&root) && !self.escaping.contains(&root)
    }

    fn is_unobserved_local(&self, value: ValueId) -> bool {
        let root = resolve_value_origin_from_parent_map(value, &self.alias_parents);
        self.local_boxes.contains(&root)
            && !self.escaping.contains(&root)
            && !self.field_read_roots.contains(&root)
    }
}

fn analyze_local_reads(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
) -> LocalReadInfo {
    let mut info = LocalReadInfo::default();

    // First collect local-box roots and direct alias parents.
    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }

        for instruction in &block.instructions {
            if let crate::mir::MirInstruction::NewBox { dst, .. } = instruction {
                info.local_boxes.insert(*dst);
            }
            if let crate::mir::MirInstruction::Copy { dst, src } = instruction {
                info.alias_parents.insert(*dst, *src);
            }
        }
    }

    info.alias_parents
        .extend(collect_passthrough_phi_parents(function));

    // Then classify escape-bearing uses against the completed alias graph.
    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }

        for instruction in &block.instructions {
            for use_site in classify_escape_uses(instruction) {
                let root =
                    resolve_value_origin_from_parent_map(use_site.value, &info.alias_parents);
                if info.local_boxes.contains(&root) {
                    info.escaping.insert(root);
                }
            }
            if let crate::mir::MirInstruction::FieldGet { base, .. } = instruction {
                let root = resolve_value_origin_from_parent_map(*base, &info.alias_parents);
                if info.local_boxes.contains(&root) {
                    info.field_read_roots.insert(root);
                }
            }
        }

        if let Some(term) = &block.terminator {
            for use_site in classify_escape_uses(term) {
                let root =
                    resolve_value_origin_from_parent_map(use_site.value, &info.alias_parents);
                if info.local_boxes.contains(&root) {
                    info.escaping.insert(root);
                }
            }
        }
    }

    info
}

fn is_removable_effect_sensitive_read_instruction(
    inst: &crate::mir::MirInstruction,
    local_reads: &LocalReadInfo,
) -> bool {
    matches!(
        inst,
        crate::mir::MirInstruction::FieldGet { base, .. } if local_reads.is_non_escaping_local(*base)
    )
}

fn is_removable_effect_sensitive_write_instruction(
    inst: &crate::mir::MirInstruction,
    local_reads: &LocalReadInfo,
) -> bool {
    matches!(
        inst,
        crate::mir::MirInstruction::FieldSet { base, .. } if local_reads.is_unobserved_local(*base)
    )
}

fn eliminate_dead_code_in_function(function: &mut MirFunction) -> usize {
    let reachable_blocks = crate::mir::verification::utils::compute_reachable_blocks(function);
    let local_reads = analyze_local_reads(function, &reachable_blocks);

    // Collect values that must be kept for reasons other than KeepAlive.
    let mut base_used_values: HashSet<ValueId> = HashSet::new();

    // Mark values used by side-effecting instructions and control-flow terminators.
    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }
        for instruction in &block.instructions {
            if matches!(instruction, crate::mir::MirInstruction::KeepAlive { .. }) {
                continue;
            }
            if is_removable_effect_sensitive_read_instruction(instruction, &local_reads) {
                continue;
            }
            if is_removable_effect_sensitive_write_instruction(instruction, &local_reads) {
                continue;
            }
            let anchors_liveness = matches!(
                instruction,
                crate::mir::MirInstruction::Branch { .. }
                    | crate::mir::MirInstruction::Jump { .. }
                    | crate::mir::MirInstruction::Return { .. }
            );
            if !instruction.effects().is_pure() || anchors_liveness {
                if let Some(dst) = instruction.dst_value() {
                    base_used_values.insert(dst);
                }
                for u in instruction.used_values() {
                    base_used_values.insert(u);
                }
            }
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

    // First propagate liveness without KeepAlive so we can tell which KeepAlive
    // instructions are redundant for other reachable reasons.
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

    // Backward propagation: if a value is used, mark its operands as used
    propagate_used_values(function, &reachable_blocks, &mut used_values);

    // Remove unused pure instructions
    let mut eliminated = 0usize;
    let dce_trace = std::env::var("NYASH_DCE_TRACE").ok().as_deref() == Some("1");
    for (bbid, block) in &mut function.blocks {
        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let mut kept_insts = Vec::with_capacity(insts.len());
        let mut kept_spans = Vec::with_capacity(spans.len());
        for (inst, span) in insts.into_iter().zip(spans.into_iter()) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
    use crate::mir::{
        BasicBlock, BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature,
        MirInstruction, MirType,
    };

    #[test]
    fn test_dce_keeps_edge_args_values() {
        let mut module = MirModule::new("dce_test".to_string());

        let sig = FunctionSignature {
            name: "test/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId(0));

        let v1 = ValueId(1);
        let v2 = ValueId(2);
        let v_dead = ValueId(3);
        let bb1 = BasicBlockId(1);

        {
            let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
            bb0.instructions.push(MirInstruction::Const {
                dst: v1,
                value: ConstValue::Integer(123),
            });
            bb0.instruction_spans.push(Span::unknown());

            copy_emitter::emit_copy_into_detached_block(
                bb0,
                v2,
                v1,
                CopyEmitReason::TestDceEdgeArgCopy,
            )
            .unwrap();

            // This pure instruction should be eliminated.
            bb0.instructions.push(MirInstruction::Const {
                dst: v_dead,
                value: ConstValue::Integer(999),
            });
            bb0.instruction_spans.push(Span::unknown());

            // SSOT: edge args are semantic use-sites (ExitLine, continuation args).
            bb0.set_jump_with_edge_args(
                bb1,
                Some(crate::mir::EdgeArgs {
                    layout: crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout::CarriersOnly,
                    values: vec![v2],
                }),
            );
        }

        let mut exit_block = crate::mir::BasicBlock::new(bb1);
        exit_block.set_terminator(MirInstruction::Return { value: None });
        func.add_block(exit_block);

        module.add_function(func);

        let eliminated = eliminate_dead_code(&mut module);
        assert_eq!(eliminated, 1);

        let func = module.get_function("test/0").unwrap();
        let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();

        // Contract: spans must stay aligned with instructions.
        assert_eq!(bb0.instructions.len(), bb0.instruction_spans.len());

        // Contract: values that appear only in edge args must be kept (and their deps).
        assert!(bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Copy { dst, .. } if *dst == v2)));
        assert!(bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v1)));

        // The unused const must be eliminated.
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_dead)));
    }

    #[test]
    fn test_dce_prunes_unreachable_pure_block() {
        let mut module = MirModule::new("dce_test".to_string());

        let sig = FunctionSignature {
            name: "test/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId(0));

        let v_entry = ValueId(1);
        let v_dead_copy = ValueId(2);
        let reachable_exit = BasicBlockId(1);
        let unreachable_bb = BasicBlockId(2);

        {
            let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
            bb0.instructions.push(MirInstruction::Const {
                dst: v_entry,
                value: ConstValue::Integer(123),
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.set_jump_with_edge_args(reachable_exit, None);
        }

        let mut bb1 = BasicBlock::new(reachable_exit);
        bb1.set_terminator(MirInstruction::Return { value: None });
        func.add_block(bb1);

        let mut dead_block = BasicBlock::new(unreachable_bb);
        copy_emitter::emit_copy_into_detached_block(
            &mut dead_block,
            v_dead_copy,
            v_entry,
            CopyEmitReason::TestDceEdgeArgCopy,
        )
        .unwrap();
        dead_block.set_terminator(MirInstruction::Return { value: None });
        func.add_block(dead_block);

        module.add_function(func);

        let eliminated = eliminate_dead_code(&mut module);
        assert_eq!(eliminated, 2);

        let func = module.get_function("test/0").unwrap();
        let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();

        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_entry)));
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Copy { dst, .. } if *dst == v_dead_copy)));
        assert!(!func.blocks.contains_key(&unreachable_bb));
    }

    #[test]
    fn test_dce_prunes_unreachable_effectful_block() {
        let mut module = MirModule::new("dce_test".to_string());

        let sig = FunctionSignature {
            name: "test/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId(0));

        let v_entry = ValueId(1);
        let v_dead_ptr = ValueId(2);
        let reachable_exit = BasicBlockId(1);
        let unreachable_bb = BasicBlockId(2);

        {
            let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
            bb0.instructions.push(MirInstruction::Const {
                dst: v_entry,
                value: ConstValue::Integer(123),
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.set_jump_with_edge_args(reachable_exit, None);
        }

        let mut bb1 = BasicBlock::new(reachable_exit);
        bb1.set_terminator(MirInstruction::Return { value: None });
        func.add_block(bb1);

        let mut dead_block = BasicBlock::new(unreachable_bb);
        dead_block.instructions.push(MirInstruction::Const {
            dst: v_dead_ptr,
            value: ConstValue::Integer(999),
        });
        dead_block.instruction_spans.push(Span::unknown());
        dead_block.instructions.push(MirInstruction::Store {
            value: v_entry,
            ptr: v_dead_ptr,
        });
        dead_block.instruction_spans.push(Span::unknown());
        dead_block.set_terminator(MirInstruction::Return { value: None });
        func.add_block(dead_block);

        module.add_function(func);

        let eliminated = eliminate_dead_code(&mut module);
        assert_eq!(eliminated, 2);

        let func = module.get_function("test/0").unwrap();
        let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();

        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_entry)));
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v_dead_ptr)));
        assert!(!func.blocks.contains_key(&unreachable_bb));
    }

    #[test]
    fn test_dce_prunes_redundant_keepalive_when_return_already_keeps_value_live() {
        let mut module = MirModule::new("dce_test".to_string());

        let sig = FunctionSignature {
            name: "test/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId(0));

        let v1 = ValueId(1);

        {
            let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
            bb0.instructions.push(MirInstruction::Const {
                dst: v1,
                value: ConstValue::Integer(123),
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions
                .push(MirInstruction::KeepAlive { values: vec![v1] });
            bb0.instruction_spans.push(Span::unknown());
            bb0.set_terminator(MirInstruction::Return { value: Some(v1) });
        }

        module.add_function(func);

        let eliminated = eliminate_dead_code(&mut module);
        assert_eq!(eliminated, 1);

        let func = module.get_function("test/0").unwrap();
        let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
        assert!(bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v1)));
        assert!(!bb0.instructions.iter().any(
            |inst| matches!(inst, MirInstruction::KeepAlive { values } if values == &vec![v1])
        ));
    }

    #[test]
    fn test_dce_keeps_nonredundant_keepalive_when_it_is_the_only_live_reason() {
        let mut module = MirModule::new("dce_test".to_string());

        let sig = FunctionSignature {
            name: "test/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId(0));

        let v1 = ValueId(1);

        {
            let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
            bb0.instructions.push(MirInstruction::Const {
                dst: v1,
                value: ConstValue::Integer(123),
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions
                .push(MirInstruction::KeepAlive { values: vec![v1] });
            bb0.instruction_spans.push(Span::unknown());
            bb0.set_terminator(MirInstruction::Return { value: None });
        }

        module.add_function(func);

        let eliminated = eliminate_dead_code(&mut module);
        assert_eq!(eliminated, 0);

        let func = module.get_function("test/0").unwrap();
        let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
        assert!(bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v1)));
        assert!(bb0.instructions.iter().any(
            |inst| matches!(inst, MirInstruction::KeepAlive { values } if values == &vec![v1])
        ));
    }

    #[test]
    fn test_dce_prunes_safepoint_noop_when_it_has_no_other_effects() {
        let mut module = MirModule::new("dce_test".to_string());

        let sig = FunctionSignature {
            name: "test/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId(0));

        let v1 = ValueId(1);

        {
            let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
            bb0.instructions.push(MirInstruction::Const {
                dst: v1,
                value: ConstValue::Integer(123),
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::Safepoint);
            bb0.instruction_spans.push(Span::unknown());
            bb0.set_terminator(MirInstruction::Return { value: Some(v1) });
        }

        module.add_function(func);

        let eliminated = eliminate_dead_code(&mut module);
        assert_eq!(eliminated, 1);

        let func = module.get_function("test/0").unwrap();
        let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
        assert!(bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v1)));
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Safepoint)));
    }

    #[test]
    fn test_dce_prunes_dead_field_get_from_non_escaping_local_box() {
        let mut module = MirModule::new("dce_test".to_string());

        let sig = FunctionSignature {
            name: "test/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId(0));

        let v_box = ValueId(1);
        let v_copy = ValueId(2);
        let v_field = ValueId(3);

        {
            let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
            bb0.instructions.push(MirInstruction::NewBox {
                dst: v_box,
                box_type: "Point".to_string(),
                args: vec![],
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::Copy {
                dst: v_copy,
                src: v_box,
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::FieldGet {
                dst: v_field,
                base: v_copy,
                field: "child".to_string(),
                declared_type: Some(MirType::Integer),
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.set_terminator(MirInstruction::Return { value: None });
        }

        module.add_function(func);

        let eliminated = eliminate_dead_code(&mut module);
        assert_eq!(eliminated, 2);

        let func = module.get_function("test/0").unwrap();
        let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
        assert!(bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::NewBox { dst, .. } if *dst == v_box)));
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Copy { dst, .. } if *dst == v_copy)));
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::FieldGet { dst, .. } if *dst == v_field)));
    }

    #[test]
    fn test_dce_prunes_dead_field_set_on_non_escaping_local_box() {
        let mut module = MirModule::new("dce_test".to_string());

        let sig = FunctionSignature {
            name: "test/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId(0));

        let v_box = ValueId(1);
        let v_copy = ValueId(2);
        let v_value = ValueId(3);

        {
            let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
            bb0.instructions.push(MirInstruction::NewBox {
                dst: v_box,
                box_type: "Point".to_string(),
                args: vec![],
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::Copy {
                dst: v_copy,
                src: v_box,
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::Const {
                dst: v_value,
                value: ConstValue::Integer(77),
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::FieldSet {
                base: v_copy,
                field: "child".to_string(),
                value: v_value,
                declared_type: Some(MirType::Integer),
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.set_terminator(MirInstruction::Return {
                value: Some(v_value),
            });
        }

        module.add_function(func);

        let eliminated = eliminate_dead_code(&mut module);
        assert_eq!(eliminated, 2);

        let func = module.get_function("test/0").unwrap();
        let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
        assert!(bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::NewBox { dst, .. } if *dst == v_box)));
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Copy { dst, .. } if *dst == v_copy)));
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::FieldSet { .. })));
    }

    #[test]
    fn test_dce_prunes_pure_no_dst_call_and_its_dead_operand_chain() {
        let mut module = MirModule::new("dce_test".to_string());

        let sig = FunctionSignature {
            name: "test/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(sig, BasicBlockId(0));

        let v1 = ValueId(1);

        {
            let bb0 = func.blocks.get_mut(&BasicBlockId(0)).unwrap();
            bb0.instructions.push(MirInstruction::Const {
                dst: v1,
                value: ConstValue::Integer(123),
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.instructions.push(MirInstruction::Call {
                dst: None,
                func: ValueId(999),
                callee: Some(Callee::Global("noop".to_string())),
                args: vec![v1],
                effects: EffectMask::PURE,
            });
            bb0.instruction_spans.push(Span::unknown());
            bb0.set_terminator(MirInstruction::Return { value: None });
        }

        module.add_function(func);

        let eliminated = eliminate_dead_code(&mut module);
        assert_eq!(eliminated, 2);

        let func = module.get_function("test/0").unwrap();
        let bb0 = func.blocks.get(&BasicBlockId(0)).unwrap();
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Const { dst, .. } if *dst == v1)));
        assert!(!bb0
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Call { dst: None, .. })));
    }
}
