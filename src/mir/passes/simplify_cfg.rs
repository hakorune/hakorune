/*!
 * SimplifyCFG - first structural simplification slice for the semantic bundle.
 *
 * This cut intentionally stays narrow:
 * - merge `pred -> middle` only for a direct `Jump`
 * - `middle` must be reachable, non-entry, have exactly one predecessor, and
 *   carry only trivial single-input PHIs from that predecessor
 * - loop-carried/self-edge cases and branch edge-args stay out of scope
 */

use crate::mir::{
    definitions::call_unified::Callee, BasicBlock, BasicBlockId, EffectMask, MirFunction,
    MirInstruction, MirModule, ValueId,
};

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
            edge_args: _,
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
        if collect_trivial_phi_rewrites(middle_block, pred_id).is_none() {
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
    let mut middle_block = function
        .blocks
        .remove(&middle_id)
        .expect("merge candidate middle block must exist");
    let phi_rewrites = collect_trivial_phi_rewrites(&middle_block, pred_id)
        .expect("merge candidate middle block must have only trivial single-input PHIs");

    for (phi_dst, incoming_value) in &phi_rewrites {
        rewrite_value_uses_in_function(function, *phi_dst, *incoming_value);
        rewrite_value_uses_in_block(&mut middle_block, *phi_dst, *incoming_value);
    }
    if !phi_rewrites.is_empty() {
        middle_block.instructions.drain(0..phi_rewrites.len());
        middle_block.instruction_spans.drain(0..phi_rewrites.len());
    }

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

fn collect_trivial_phi_rewrites(
    middle_block: &BasicBlock,
    pred_id: BasicBlockId,
) -> Option<Vec<(ValueId, ValueId)>> {
    let mut rewrites = Vec::new();
    for instruction in middle_block.phi_instructions() {
        let MirInstruction::Phi { dst, inputs, .. } = instruction else {
            unreachable!("phi_instructions() must yield only PHI instructions");
        };
        let [(incoming_block, incoming_value)] = inputs.as_slice() else {
            return None;
        };
        if *incoming_block != pred_id {
            return None;
        }
        rewrites.push((*dst, *incoming_value));
    }
    Some(rewrites)
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

fn rewrite_value_uses_in_function(function: &mut MirFunction, from: ValueId, to: ValueId) {
    for block in function.blocks.values_mut() {
        rewrite_value_uses_in_block(block, from, to);
    }
}

fn rewrite_value_uses_in_block(block: &mut BasicBlock, from: ValueId, to: ValueId) {
    for instruction in &mut block.instructions {
        rewrite_value_uses_in_instruction(instruction, from, to);
    }
    if let Some(terminator) = &mut block.terminator {
        rewrite_value_uses_in_instruction(terminator, from, to);
    }
    if let Some(return_env) = &mut block.return_env {
        for value in return_env {
            rewrite_value_use(value, from, to);
        }
    }
}

fn rewrite_value_uses_in_instruction(instruction: &mut MirInstruction, from: ValueId, to: ValueId) {
    match instruction {
        MirInstruction::Const { .. } | MirInstruction::Catch { .. } | MirInstruction::Safepoint => {
        }
        MirInstruction::BinOp { lhs, rhs, .. } | MirInstruction::Compare { lhs, rhs, .. } => {
            rewrite_value_use(lhs, from, to);
            rewrite_value_use(rhs, from, to);
        }
        MirInstruction::UnaryOp { operand, .. }
        | MirInstruction::Load { ptr: operand, .. }
        | MirInstruction::FieldGet { base: operand, .. }
        | MirInstruction::VariantTag { value: operand, .. }
        | MirInstruction::VariantProject { value: operand, .. }
        | MirInstruction::TypeOp { value: operand, .. }
        | MirInstruction::Copy { src: operand, .. }
        | MirInstruction::Debug { value: operand, .. }
        | MirInstruction::Throw {
            exception: operand, ..
        }
        | MirInstruction::RefNew {
            box_val: operand, ..
        }
        | MirInstruction::WeakRef { value: operand, .. }
        | MirInstruction::Barrier { ptr: operand, .. }
        | MirInstruction::FutureNew { value: operand, .. }
        | MirInstruction::Await {
            future: operand, ..
        } => rewrite_value_use(operand, from, to),
        MirInstruction::Store { value, ptr } => {
            rewrite_value_use(value, from, to);
            rewrite_value_use(ptr, from, to);
        }
        MirInstruction::FieldSet { base, value, .. } => {
            rewrite_value_use(base, from, to);
            rewrite_value_use(value, from, to);
        }
        MirInstruction::VariantMake { payload, .. } => {
            if let Some(payload) = payload {
                rewrite_value_use(payload, from, to);
            }
        }
        MirInstruction::Call {
            func, callee, args, ..
        } => {
            if callee.is_none() {
                rewrite_value_use(func, from, to);
            }
            if let Some(callee) = callee {
                match callee {
                    Callee::Method { receiver, .. } => {
                        if let Some(receiver) = receiver {
                            rewrite_value_use(receiver, from, to);
                        }
                    }
                    Callee::Closure {
                        captures,
                        me_capture,
                        ..
                    } => {
                        for (_, capture) in captures {
                            rewrite_value_use(capture, from, to);
                        }
                        if let Some(me_capture) = me_capture {
                            rewrite_value_use(me_capture, from, to);
                        }
                    }
                    Callee::Value(value) => rewrite_value_use(value, from, to),
                    Callee::Global(_) | Callee::Constructor { .. } | Callee::Extern(_) => {}
                }
            }
            for arg in args {
                rewrite_value_use(arg, from, to);
            }
        }
        MirInstruction::NewClosure { captures, me, .. } => {
            for (_, capture) in captures {
                rewrite_value_use(capture, from, to);
            }
            if let Some(me) = me {
                rewrite_value_use(me, from, to);
            }
        }
        MirInstruction::Branch {
            condition,
            then_edge_args,
            else_edge_args,
            ..
        } => {
            rewrite_value_use(condition, from, to);
            rewrite_edge_args_values(then_edge_args, from, to);
            rewrite_edge_args_values(else_edge_args, from, to);
        }
        MirInstruction::Jump { edge_args, .. } => {
            rewrite_edge_args_values(edge_args, from, to);
        }
        MirInstruction::Return { value } => {
            if let Some(value) = value {
                rewrite_value_use(value, from, to);
            }
        }
        MirInstruction::Phi { inputs, .. } => {
            for (_, incoming_value) in inputs {
                rewrite_value_use(incoming_value, from, to);
            }
        }
        MirInstruction::NewBox { args, .. } => {
            for arg in args {
                rewrite_value_use(arg, from, to);
            }
        }
        MirInstruction::KeepAlive { values } | MirInstruction::ReleaseStrong { values } => {
            for value in values {
                rewrite_value_use(value, from, to);
            }
        }
        MirInstruction::FutureSet { future, value } => {
            rewrite_value_use(future, from, to);
            rewrite_value_use(value, from, to);
        }
        MirInstruction::Select {
            cond,
            then_val,
            else_val,
            ..
        } => {
            rewrite_value_use(cond, from, to);
            rewrite_value_use(then_val, from, to);
            rewrite_value_use(else_val, from, to);
        }
    }
}

fn rewrite_edge_args_values(
    edge_args: &mut Option<crate::mir::EdgeArgs>,
    from: ValueId,
    to: ValueId,
) {
    if let Some(edge_args) = edge_args {
        for value in &mut edge_args.values {
            rewrite_value_use(value, from, to);
        }
    }
}

fn rewrite_value_use(value: &mut ValueId, from: ValueId, to: ValueId) {
    if *value == from {
        *value = to;
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
    fn simplifies_jump_block_when_predecessor_edge_args_are_dead_for_middle() {
        let mut module = MirModule::new("simplify_cfg_edge_args_merge".to_string());
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
        assert_eq!(simplified, 1);

        let function = module.functions.get("main").expect("main function");
        assert_eq!(function.blocks.len(), 1);
        let entry = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert!(matches!(
            entry.terminator,
            Some(MirInstruction::Return { value: None })
        ));
    }

    #[test]
    fn keeps_jump_block_when_middle_has_phi_inputs_for_edge_args() {
        let mut module = MirModule::new("simplify_cfg_edge_args_phi_merge".to_string());
        let mut function =
            MirFunction::new(test_signature("main", MirType::Integer), BasicBlockId(0));

        {
            let entry = function
                .blocks
                .get_mut(&BasicBlockId(0))
                .expect("entry block");
            entry.instructions.push(MirInstruction::Const {
                dst: ValueId(1),
                value: ConstValue::Integer(7),
            });
            entry.instruction_spans.push(Span::unknown());
            entry.set_jump_with_edge_args(
                BasicBlockId(1),
                Some(EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: vec![ValueId(1)],
                }),
            );
        }

        let mut middle = BasicBlock::new(BasicBlockId(1));
        middle.instructions.push(MirInstruction::Phi {
            dst: ValueId(2),
            inputs: vec![(BasicBlockId(0), ValueId(1))],
            type_hint: Some(MirType::Integer),
        });
        middle.instruction_spans.push(Span::unknown());
        middle.set_terminator(MirInstruction::Return {
            value: Some(ValueId(2)),
        });
        function.add_block(middle);
        function
            .metadata
            .value_types
            .insert(ValueId(1), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(2), MirType::Integer);
        function.update_cfg();
        module.add_function(function);

        let simplified = simplify(&mut module);
        assert_eq!(simplified, 1);

        let function = module.functions.get("main").expect("main function");
        assert_eq!(function.blocks.len(), 1);

        let entry = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert!(!entry
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Phi { .. })));
        assert!(matches!(
            entry.terminator,
            Some(MirInstruction::Return {
                value: Some(value)
            }) if value == ValueId(1)
        ));
    }

    #[test]
    fn simplifies_jump_block_and_rewrites_successor_phi_values_for_trivial_middle_phi() {
        let mut module = MirModule::new("simplify_cfg_phi_value_rewrite".to_string());
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

        let mut pred = BasicBlock::new(BasicBlockId(1));
        pred.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(11),
        });
        pred.instruction_spans.push(Span::unknown());
        pred.set_jump_with_edge_args(
            BasicBlockId(2),
            Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(2)],
            }),
        );
        function.add_block(pred);

        let mut middle = BasicBlock::new(BasicBlockId(2));
        middle.instructions.push(MirInstruction::Phi {
            dst: ValueId(3),
            inputs: vec![(BasicBlockId(1), ValueId(2))],
            type_hint: Some(MirType::Integer),
        });
        middle.instruction_spans.push(Span::unknown());
        middle.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(4),
            edge_args: None,
        });
        function.add_block(middle);

        let mut else_block = BasicBlock::new(BasicBlockId(3));
        else_block.instructions.push(MirInstruction::Const {
            dst: ValueId(4),
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
            dst: ValueId(5),
            inputs: vec![(BasicBlockId(2), ValueId(3)), (BasicBlockId(3), ValueId(4))],
            type_hint: Some(MirType::Integer),
        });
        merge.instruction_spans.push(Span::unknown());
        merge.set_terminator(MirInstruction::Return {
            value: Some(ValueId(5)),
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
        function
            .metadata
            .value_types
            .insert(ValueId(5), MirType::Integer);
        function.update_cfg();
        module.add_function(function);

        let simplified = simplify(&mut module);
        assert_eq!(simplified, 1);

        let function = module.functions.get("main").expect("main function");
        assert!(!function.blocks.contains_key(&BasicBlockId(2)));

        let pred = function.blocks.get(&BasicBlockId(1)).expect("pred block");
        assert!(matches!(
            pred.terminator,
            Some(MirInstruction::Jump {
                target,
                edge_args: None
            }) if target == BasicBlockId(4)
        ));

        let merge = function.blocks.get(&BasicBlockId(4)).expect("merge block");
        let MirInstruction::Phi { inputs, .. } = &merge.instructions[0] else {
            panic!("expected phi");
        };
        assert_eq!(
            inputs,
            &vec![(BasicBlockId(1), ValueId(2)), (BasicBlockId(3), ValueId(4))]
        );
    }
}
