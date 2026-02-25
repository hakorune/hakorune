//! Dead Code Elimination (pure instruction DCE)
//!
//! Extracted from the monolithic optimizer to enable modular pass composition.

use crate::mir::{MirFunction, MirModule, ValueId};
use crate::runtime::get_global_ring0;
use std::collections::HashSet;

/// Eliminate dead code (unused results of pure instructions) across the module.
/// Returns the number of eliminated instructions.
pub fn eliminate_dead_code(module: &mut MirModule) -> usize {
    let mut eliminated_total = 0usize;
    for (_func_name, func) in &mut module.functions {
        eliminated_total += eliminate_dead_code_in_function(func);
    }
    eliminated_total
}

fn eliminate_dead_code_in_function(function: &mut MirFunction) -> usize {
    // Collect values that must be kept (used results + effects)
    let mut used_values: HashSet<ValueId> = HashSet::new();

    // Mark values used by side-effecting instructions and terminators
    for (_bid, block) in &function.blocks {
        for instruction in &block.instructions {
            let has_dst = instruction.dst_value().is_some();
            if !instruction.effects().is_pure() || !has_dst {
                if let Some(dst) = instruction.dst_value() {
                    used_values.insert(dst);
                }
                for u in instruction.used_values() {
                    used_values.insert(u);
                }
            }
        }
        if let Some(term) = &block.terminator {
            for u in term.used_values() {
                used_values.insert(u);
            }
        }
        for edge in block.out_edges() {
            if let Some(args) = edge.args {
                for u in args.values {
                    used_values.insert(u);
                }
            }
        }
    }

    // Backward propagation: if a value is used, mark its operands as used
    let mut changed = true;
    while changed {
        changed = false;
        for (_bid, block) in &function.blocks {
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
            if inst.effects().is_pure() {
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
    if eliminated > 0 {
        function.update_cfg();
    }
    eliminated
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
    use crate::mir::{BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirInstruction, MirType};

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

            copy_emitter::emit_copy_into_detached_block(bb0, v2, v1, CopyEmitReason::TestDceEdgeArgCopy)
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
}
