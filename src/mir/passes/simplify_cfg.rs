/*!
 * SimplifyCFG - first structural simplification slice for the semantic bundle.
 *
 * This cut intentionally stays narrow:
 * - only merge `pred -> middle` when `pred` ends in `Jump { edge_args: None }`
 * - `middle` must be reachable, non-entry, have exactly one predecessor, and no PHIs
 * - loop-carried/self-edge cases stay out of scope
 */

use crate::mir::{BasicBlock, BasicBlockId, EffectMask, MirFunction, MirInstruction, MirModule};

pub fn simplify(module: &mut MirModule) -> usize {
    let mut simplified = 0usize;
    for function in module.functions.values_mut() {
        simplified += simplify_function(function);
    }
    simplified
}

fn simplify_function(function: &mut MirFunction) -> usize {
    let mut simplified = 0usize;

    loop {
        function.update_cfg();
        let Some((pred_id, middle_id)) = find_single_predecessor_jump_merge(function) else {
            break;
        };
        merge_single_predecessor_jump_block(function, pred_id, middle_id);
        simplified += 1;
    }

    simplified
}

fn find_single_predecessor_jump_merge(
    function: &MirFunction,
) -> Option<(BasicBlockId, BasicBlockId)> {
    let reachable_blocks = crate::mir::verification::utils::compute_reachable_blocks(function);

    for pred_id in function.block_ids() {
        if !reachable_blocks.contains(&pred_id) {
            continue;
        }

        let pred_block = function.blocks.get(&pred_id)?;
        let MirInstruction::Jump {
            target: middle_id,
            edge_args: None,
        } = pred_block.terminator.as_ref()?
        else {
            continue;
        };

        if *middle_id == pred_id || *middle_id == function.entry_block {
            continue;
        }

        let middle_block = function.blocks.get(middle_id)?;
        if !reachable_blocks.contains(middle_id) {
            continue;
        }
        if middle_block.terminator.is_none() {
            continue;
        }
        if middle_block.predecessors.len() != 1 || !middle_block.predecessors.contains(&pred_id) {
            continue;
        }
        if middle_block.phi_instructions().next().is_some() {
            continue;
        }
        if middle_block.successors.contains(&pred_id) {
            continue;
        }

        return Some((pred_id, *middle_id));
    }

    None
}

fn merge_single_predecessor_jump_block(
    function: &mut MirFunction,
    pred_id: BasicBlockId,
    middle_id: BasicBlockId,
) {
    let middle_block = function
        .blocks
        .remove(&middle_id)
        .expect("merge candidate middle block must exist");

    rewrite_phi_predecessor(function, middle_id, pred_id);

    let pred_block = function
        .blocks
        .get_mut(&pred_id)
        .expect("merge candidate predecessor block must exist");

    pred_block.instructions.extend(middle_block.instructions);
    pred_block
        .instruction_spans
        .extend(middle_block.instruction_spans);
    pred_block.terminator = middle_block.terminator;
    pred_block.terminator_span = middle_block.terminator_span;
    pred_block.return_env = middle_block.return_env;
    pred_block.return_env_layout = middle_block.return_env_layout;
    pred_block.successors = pred_block.successors_from_terminator();
    recompute_effects(pred_block);

    function.update_cfg();
}

fn rewrite_phi_predecessor(
    function: &mut MirFunction,
    old_predecessor: BasicBlockId,
    new_predecessor: BasicBlockId,
) {
    for block in function.blocks.values_mut() {
        for instruction in &mut block.instructions {
            let MirInstruction::Phi { inputs, .. } = instruction else {
                continue;
            };

            for (incoming_block, _) in inputs.iter_mut() {
                if *incoming_block == old_predecessor {
                    *incoming_block = new_predecessor;
                }
            }
        }
    }
}

fn recompute_effects(block: &mut BasicBlock) {
    let mut effects = EffectMask::PURE;
    for instruction in &block.instructions {
        effects = effects | instruction.effects();
    }
    if let Some(terminator) = &block.terminator {
        effects = effects | terminator.effects();
    }
    block.effects = effects;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::{
        join_ir::lowering::inline_boundary::JumpArgsLayout, ConstValue, EdgeArgs, EffectMask,
        FunctionSignature, MirModule, MirType, ValueId,
    };

    fn test_signature(name: &str, return_type: MirType) -> FunctionSignature {
        FunctionSignature {
            name: name.to_string(),
            params: vec![],
            return_type,
            effects: EffectMask::PURE,
        }
    }

    #[test]
    fn simplifies_linear_single_predecessor_jump_block() {
        let mut module = MirModule::new("simplify_cfg_linear".to_string());
        let mut function =
            MirFunction::new(test_signature("main", MirType::Integer), BasicBlockId(0));

        {
            let entry = function
                .blocks
                .get_mut(&BasicBlockId(0))
                .expect("entry block");
            entry.set_terminator(MirInstruction::Jump {
                target: BasicBlockId(1),
                edge_args: None,
            });
        }

        let mut middle = BasicBlock::new(BasicBlockId(1));
        middle.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(7),
        });
        middle.instruction_spans.push(Span::unknown());
        middle.set_terminator(MirInstruction::Return {
            value: Some(ValueId(1)),
        });
        function.add_block(middle);
        function
            .metadata
            .value_types
            .insert(ValueId(1), MirType::Integer);

        function.update_cfg();
        module.add_function(function);

        let simplified = simplify(&mut module);
        assert_eq!(simplified, 1);

        let function = module.functions.get("main").expect("main function");
        assert_eq!(function.blocks.len(), 1);
        let entry = function.blocks.get(&BasicBlockId(0)).expect("entry block");
        assert!(matches!(
            entry.instructions.as_slice(),
            [MirInstruction::Const {
                dst,
                value: ConstValue::Integer(7)
            }] if *dst == ValueId(1)
        ));
        assert!(matches!(
            entry.terminator,
            Some(MirInstruction::Return {
                value: Some(value)
            }) if value == ValueId(1)
        ));
    }

    #[test]
    fn simplifies_jump_block_and_rewrites_successor_phi_inputs() {
        let mut module = MirModule::new("simplify_cfg_phi_rewrite".to_string());
        let mut function =
            MirFunction::new(test_signature("main", MirType::Integer), BasicBlockId(0));

        {
            let entry = function
                .blocks
                .get_mut(&BasicBlockId(0))
                .expect("entry block");
            entry.instructions.push(MirInstruction::Const {
                dst: ValueId(1),
                value: ConstValue::Bool(true),
            });
            entry.instruction_spans.push(Span::unknown());
            entry.set_terminator(MirInstruction::Branch {
                condition: ValueId(1),
                then_bb: BasicBlockId(1),
                else_bb: BasicBlockId(3),
                then_edge_args: None,
                else_edge_args: None,
            });
        }

        let mut then_bridge = BasicBlock::new(BasicBlockId(1));
        then_bridge.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(2),
            edge_args: None,
        });
        function.add_block(then_bridge);

        let mut middle = BasicBlock::new(BasicBlockId(2));
        middle.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(11),
        });
        middle.instruction_spans.push(Span::unknown());
        middle.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(4),
            edge_args: None,
        });
        function.add_block(middle);

        let mut else_block = BasicBlock::new(BasicBlockId(3));
        else_block.instructions.push(MirInstruction::Const {
            dst: ValueId(3),
            value: ConstValue::Integer(22),
        });
        else_block.instruction_spans.push(Span::unknown());
        else_block.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(4),
            edge_args: None,
        });
        function.add_block(else_block);

        let mut merge = BasicBlock::new(BasicBlockId(4));
        merge.instructions.push(MirInstruction::Phi {
            dst: ValueId(4),
            inputs: vec![(BasicBlockId(2), ValueId(2)), (BasicBlockId(3), ValueId(3))],
            type_hint: Some(MirType::Integer),
        });
        merge.instruction_spans.push(Span::unknown());
        merge.set_terminator(MirInstruction::Return {
            value: Some(ValueId(4)),
        });
        function.add_block(merge);

        function
            .metadata
            .value_types
            .insert(ValueId(1), MirType::Bool);
        function
            .metadata
            .value_types
            .insert(ValueId(2), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Integer);

        function.update_cfg();
        module.add_function(function);

        let simplified = simplify(&mut module);
        assert_eq!(simplified, 1);

        let function = module.functions.get("main").expect("main function");
        assert!(!function.blocks.contains_key(&BasicBlockId(2)));

        let then_block = function.blocks.get(&BasicBlockId(1)).expect("then block");
        assert!(matches!(
            then_block.instructions.as_slice(),
            [MirInstruction::Const {
                dst,
                value: ConstValue::Integer(11)
            }] if *dst == ValueId(2)
        ));

        let merge = function.blocks.get(&BasicBlockId(4)).expect("merge block");
        let MirInstruction::Phi { inputs, .. } = &merge.instructions[0] else {
            panic!("expected phi");
        };
        assert_eq!(
            inputs,
            &vec![(BasicBlockId(1), ValueId(2)), (BasicBlockId(3), ValueId(3))]
        );
    }

    #[test]
    fn keeps_jump_block_when_predecessor_edge_args_are_present() {
        let mut module = MirModule::new("simplify_cfg_edge_args_guard".to_string());
        let mut function = MirFunction::new(test_signature("main", MirType::Void), BasicBlockId(0));

        {
            let entry = function
                .blocks
                .get_mut(&BasicBlockId(0))
                .expect("entry block");
            entry.set_jump_with_edge_args(
                BasicBlockId(1),
                Some(EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: vec![ValueId(9)],
                }),
            );
        }

        let mut middle = BasicBlock::new(BasicBlockId(1));
        middle.set_terminator(MirInstruction::Return { value: None });
        function.add_block(middle);
        function.update_cfg();
        module.add_function(function);

        let simplified = simplify(&mut module);
        assert_eq!(simplified, 0);

        let function = module.functions.get("main").expect("main function");
        assert_eq!(function.blocks.len(), 2);
    }
}
