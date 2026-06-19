//! PHI input materialization SSOT.
//!
//! A PHI input is an edge value: the incoming value must be valid at the
//! predecessor block attached to that input. If a pure value was defined on a
//! sibling path, this helper rematerializes an equivalent value in the
//! predecessor before PHI insertion.

use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction, ValueId};
use hakorune_mir_defs::CalleeBoxKind;
use std::collections::{HashMap, HashSet};

struct PhiInputMaterializationAnalysis {
    def_blocks: HashMap<ValueId, BasicBlockId>,
    dominators: crate::mir::verification::utils::DominatorTree,
}

impl PhiInputMaterializationAnalysis {
    fn new(func: &mut MirFunction) -> Self {
        func.update_cfg();
        Self {
            def_blocks: crate::mir::verification::utils::compute_def_blocks(func),
            dominators: crate::mir::verification::utils::compute_dominators(func),
        }
    }
}

fn find_def_inst(
    func: &MirFunction,
    value: ValueId,
) -> Option<(BasicBlockId, Option<MirInstruction>)> {
    if func.params.iter().any(|param| *param == value) {
        return Some((func.entry_block, None));
    }

    for (bb, block) in &func.blocks {
        for inst in &block.instructions {
            if inst.dst_value() == Some(value) {
                return Some((*bb, Some(inst.clone())));
            }
        }
        if let Some(term) = &block.terminator {
            if term.dst_value() == Some(value) {
                return Some((*bb, Some(term.clone())));
            }
        }
    }

    None
}

fn rematerialize_for_pred(
    func: &mut MirFunction,
    analysis: &PhiInputMaterializationAnalysis,
    pred: BasicBlockId,
    value: ValueId,
    context: &str,
    edge_kind: &str,
    visited: &mut HashSet<ValueId>,
) -> Result<ValueId, String> {
    let dominates_pred = analysis
        .def_blocks
        .get(&value)
        .copied()
        .map(|def_bb| analysis.dominators.dominates(def_bb, pred))
        .unwrap_or(false);

    if !visited.insert(value) {
        return Err(format!(
            "[freeze:contract][ssa/phi_input/remat_cycle] fn={} pred={:?} context={} edge={} value=%{}",
            func.signature.name, pred, context, edge_kind, value.0
        ));
    }

    let Some((def_bb, def_inst)) = find_def_inst(func, value) else {
        return Err(format!(
            "[freeze:contract][ssa/phi_input/without_def] fn={} pred={:?} context={} edge={} value=%{}",
            func.signature.name, pred, context, edge_kind, value.0
        ));
    };

    let Some(def_inst) = def_inst else {
        if dominates_pred {
            visited.remove(&value);
            return Ok(value);
        }
        return Err(format!(
            "[freeze:contract][ssa/phi_input/non_dominating_param] fn={} pred={:?} context={} edge={} value=%{} def_block={:?}",
            func.signature.name, pred, context, edge_kind, value.0, def_bb
        ));
    };

    if def_bb == pred {
        visited.remove(&value);
        return Ok(value);
    }

    let remat = match def_inst {
        MirInstruction::Const {
            value: const_value, ..
        } => {
            let dst = func.next_value_id();
            MirInstruction::Const {
                dst,
                value: const_value,
            }
        }
        MirInstruction::Copy { src, .. } => {
            let src =
                rematerialize_for_pred(func, analysis, pred, src, context, edge_kind, visited)?;
            let dst = func.next_value_id();
            MirInstruction::Copy { dst, src }
        }
        MirInstruction::BinOp { op, lhs, rhs, .. } => {
            let lhs =
                rematerialize_for_pred(func, analysis, pred, lhs, context, edge_kind, visited)?;
            let rhs =
                rematerialize_for_pred(func, analysis, pred, rhs, context, edge_kind, visited)?;
            let dst = func.next_value_id();
            MirInstruction::BinOp { dst, op, lhs, rhs }
        }
        MirInstruction::Compare { op, lhs, rhs, .. } => {
            let lhs =
                rematerialize_for_pred(func, analysis, pred, lhs, context, edge_kind, visited)?;
            let rhs =
                rematerialize_for_pred(func, analysis, pred, rhs, context, edge_kind, visited)?;
            let dst = func.next_value_id();
            MirInstruction::Compare { dst, op, lhs, rhs }
        }
        MirInstruction::UnaryOp { op, operand, .. } => {
            let operand =
                rematerialize_for_pred(func, analysis, pred, operand, context, edge_kind, visited)?;
            let dst = func.next_value_id();
            MirInstruction::UnaryOp { dst, op, operand }
        }
        MirInstruction::Select {
            cond,
            then_val,
            else_val,
            ..
        } => {
            let cond =
                rematerialize_for_pred(func, analysis, pred, cond, context, edge_kind, visited)?;
            let then_val = rematerialize_for_pred(
                func, analysis, pred, then_val, context, edge_kind, visited,
            )?;
            let else_val = rematerialize_for_pred(
                func, analysis, pred, else_val, context, edge_kind, visited,
            )?;
            let dst = func.next_value_id();
            MirInstruction::Select {
                dst,
                cond,
                then_val,
                else_val,
            }
        }
        MirInstruction::Call {
            dst: Some(_),
            func: call_func,
            callee,
            args,
            effects,
        } if is_rematerializable_string_method_call(&callee) => {
            let args = args
                .into_iter()
                .map(|arg| {
                    rematerialize_for_pred(func, analysis, pred, arg, context, edge_kind, visited)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let callee = rematerialize_callee_for_pred(
                func, analysis, pred, callee, context, edge_kind, visited,
            )?;
            let dst = func.next_value_id();
            MirInstruction::Call {
                dst: Some(dst),
                func: call_func,
                callee,
                args,
                effects,
            }
        }
        other => {
            if dominates_pred {
                visited.remove(&value);
                return Ok(value);
            }
            return Err(format!(
                "[freeze:contract][ssa/phi_input/non_rematerializable] fn={} pred={:?} context={} edge={} value=%{} def_block={:?} def_kind={:?}",
                func.signature.name, pred, context, edge_kind, value.0, def_bb, other
            ));
        }
    };

    let dst = remat
        .dst_value()
        .ok_or_else(|| "[ssa/phi_input] rematerialized instruction missing dst".to_string())?;
    let fn_name = func.signature.name.clone();
    let block = func.get_block_mut(pred).ok_or_else(|| {
        format!(
            "[freeze:contract][ssa/phi_input/missing_pred_block] fn={} pred={:?} context={} edge={} value=%{}",
            fn_name, pred, context, edge_kind, value.0
        )
    })?;
    block.add_instruction_before_terminator(remat);
    visited.remove(&value);
    Ok(dst)
}

fn is_rematerializable_string_method_call(callee: &Option<Callee>) -> bool {
    matches!(
        callee,
        Some(Callee::Method {
            box_name,
            method,
            box_kind: CalleeBoxKind::RuntimeData,
            ..
        }) if matches!(box_name.as_str(), "RuntimeDataBox" | "StringBox")
            && method == "substring"
    )
}

fn rematerialize_callee_for_pred(
    func: &mut MirFunction,
    analysis: &PhiInputMaterializationAnalysis,
    pred: BasicBlockId,
    callee: Option<Callee>,
    context: &str,
    edge_kind: &str,
    visited: &mut HashSet<ValueId>,
) -> Result<Option<Callee>, String> {
    match callee {
        Some(Callee::Method {
            box_name,
            method,
            receiver,
            certainty,
            box_kind,
        }) => {
            let receiver = receiver
                .map(|value| {
                    rematerialize_for_pred(func, analysis, pred, value, context, edge_kind, visited)
                })
                .transpose()?;
            Ok(Some(Callee::Method {
                box_name,
                method,
                receiver,
                certainty,
                box_kind,
            }))
        }
        other => Ok(other),
    }
}

pub(in crate::mir::builder) fn for_pred(
    func: &mut MirFunction,
    pred: BasicBlockId,
    value: ValueId,
    context: &str,
    edge_kind: &str,
) -> Result<ValueId, String> {
    let analysis = PhiInputMaterializationAnalysis::new(func);
    rematerialize_for_pred(
        func,
        &analysis,
        pred,
        value,
        context,
        edge_kind,
        &mut HashSet::new(),
    )
}

pub(in crate::mir::builder) fn materialize_all_phi_inputs(
    func: &mut MirFunction,
    context: &str,
) -> Result<usize, String> {
    let mut changed = prune_unused_phi_instructions(func);
    changed += complete_missing_self_carried_phi_inputs(func);
    let mut work = Vec::new();
    for (block_id, block) in &func.blocks {
        for (inst_idx, inst) in block.instructions.iter().enumerate() {
            if let MirInstruction::Phi { inputs, .. } = inst {
                for (input_idx, (pred, incoming)) in inputs.iter().enumerate() {
                    work.push((*block_id, inst_idx, input_idx, *pred, *incoming));
                }
            }
        }
    }

    let analysis = PhiInputMaterializationAnalysis::new(func);
    for (block_id, inst_idx, input_idx, pred, incoming) in work {
        let materialized = rematerialize_for_pred(
            func,
            &analysis,
            pred,
            incoming,
            context,
            "phi",
            &mut HashSet::new(),
        )?;
        if materialized == incoming {
            continue;
        }
        let fn_name = func.signature.name.clone();
        let block = func.get_block_mut(block_id).ok_or_else(|| {
            format!(
                "[freeze:contract][ssa/phi_input/missing_phi_block] fn={} block={:?} context={}",
                fn_name, block_id, context
            )
        })?;
        let Some(MirInstruction::Phi { inputs, .. }) = block.instructions.get_mut(inst_idx) else {
            return Err(format!(
                "[freeze:contract][ssa/phi_input/missing_phi_inst] fn={} block={:?} inst_idx={} context={}",
                fn_name, block_id, inst_idx, context
            ));
        };
        let Some((_, slot)) = inputs.get_mut(input_idx) else {
            return Err(format!(
                "[freeze:contract][ssa/phi_input/missing_phi_input] fn={} block={:?} inst_idx={} input_idx={} context={}",
                fn_name, block_id, inst_idx, input_idx, context
            ));
        };
        *slot = materialized;
        changed += 1;
    }

    Ok(changed)
}

fn prune_unused_phi_instructions(func: &mut MirFunction) -> usize {
    let mut used = HashSet::new();
    for block in func.blocks.values() {
        for inst in block.all_instructions() {
            for value in inst.used_values() {
                used.insert(value);
            }
        }
    }

    let mut changed = 0usize;
    for block in func.blocks.values_mut() {
        let mut remove_indices = Vec::new();
        for (idx, inst) in block.instructions.iter().enumerate() {
            let MirInstruction::Phi { dst, .. } = inst else {
                continue;
            };
            if !used.contains(dst) {
                remove_indices.push(idx);
            }
        }

        for idx in remove_indices.into_iter().rev() {
            block.instructions.remove(idx);
            if idx < block.instruction_spans.len() {
                block.instruction_spans.remove(idx);
            }
            changed += 1;
        }
    }
    changed
}

fn complete_missing_self_carried_phi_inputs(func: &mut MirFunction) -> usize {
    func.update_cfg();
    let preds = crate::mir::verification::utils::compute_predecessors(func);
    let reachable = crate::mir::verification::utils::compute_reachable_blocks(func);
    let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
    let dominators = crate::mir::verification::utils::compute_dominators(func);

    let mut additions = Vec::new();
    for (block_id, block) in &func.blocks {
        if !reachable.contains(block_id) {
            continue;
        }
        let Some(expected_preds) = preds.get(block_id) else {
            continue;
        };

        for (inst_idx, inst) in block.instructions.iter().enumerate() {
            let MirInstruction::Phi { dst, inputs, .. } = inst else {
                continue;
            };
            let input_preds: HashSet<BasicBlockId> = inputs.iter().map(|(pred, _)| *pred).collect();

            for pred in expected_preds {
                if !reachable.contains(pred) || input_preds.contains(pred) {
                    continue;
                }
                // A missing input can be completed as "unchanged on this edge"
                // only when the PHI definition block dominates that predecessor.
                // This covers loop-invariant / unchanged-carrier backedges while
                // avoiding fabricated values for unrelated merge predecessors.
                if dominators.dominates(*block_id, *pred) {
                    additions.push((*block_id, inst_idx, *pred, *dst));
                    continue;
                }

                let mut dominating_inputs = inputs
                    .iter()
                    .filter_map(|(_, incoming)| {
                        let def_bb = def_blocks.get(incoming).copied()?;
                        dominators.dominates(def_bb, *pred).then_some(*incoming)
                    })
                    .collect::<Vec<_>>();
                dominating_inputs.sort_by_key(|value| value.0);
                dominating_inputs.dedup();
                if dominating_inputs.len() == 1 {
                    additions.push((*block_id, inst_idx, *pred, dominating_inputs[0]));
                }
            }
        }
    }

    let changed = additions.len();
    for (block_id, inst_idx, pred, dst) in additions {
        let Some(block) = func.get_block_mut(block_id) else {
            continue;
        };
        let Some(MirInstruction::Phi { inputs, .. }) = block.instructions.get_mut(inst_idx) else {
            continue;
        };
        if !inputs
            .iter()
            .any(|(existing_pred, _)| *existing_pred == pred)
        {
            inputs.push((pred, dst));
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType};
    use hakorune_mir_defs::TypeCertainty;

    fn test_signature(name: &str) -> FunctionSignature {
        FunctionSignature {
            name: name.to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: crate::mir::EffectMask::PURE,
        }
    }

    #[test]
    fn completes_self_carried_phi_input_for_dominated_backedge() {
        let mut func = MirFunction::new(test_signature("self_carried"), BasicBlockId::new(0));
        func.add_block(BasicBlock::new(BasicBlockId::new(1)));
        func.add_block(BasicBlock::new(BasicBlockId::new(2)));

        let seed = func.next_value_id();
        let phi = func.next_value_id();
        func.get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: seed,
                value: ConstValue::Integer(0),
            });
        func.get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(1),
                edge_args: None,
            });
        func.get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .add_instruction(MirInstruction::Phi {
                dst: phi,
                inputs: vec![(BasicBlockId::new(0), seed)],
                type_hint: None,
            });
        func.get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(2),
                edge_args: None,
            });
        let body_use = func.next_value_id();
        func.get_block_mut(BasicBlockId::new(2))
            .unwrap()
            .add_instruction(MirInstruction::Copy {
                dst: body_use,
                src: phi,
            });
        func.get_block_mut(BasicBlockId::new(2))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(1),
                edge_args: None,
            });

        let changed = materialize_all_phi_inputs(&mut func, "test").unwrap();
        assert_eq!(changed, 1);

        let header = func.get_block(BasicBlockId::new(1)).unwrap();
        let MirInstruction::Phi { inputs, .. } = &header.instructions[0] else {
            panic!("expected phi");
        };
        assert!(inputs.contains(&(BasicBlockId::new(2), phi)));
    }

    #[test]
    fn does_not_complete_missing_input_for_undominated_merge_pred() {
        let mut func = MirFunction::new(test_signature("merge"), BasicBlockId::new(0));
        func.add_block(BasicBlock::new(BasicBlockId::new(1)));
        func.add_block(BasicBlock::new(BasicBlockId::new(2)));
        func.add_block(BasicBlock::new(BasicBlockId::new(3)));

        let cond = func.next_value_id();
        let branch_local = func.next_value_id();
        let phi = func.next_value_id();
        func.get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: cond,
                value: ConstValue::Integer(0),
            });
        func.get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Branch {
                condition: cond,
                then_bb: BasicBlockId::new(1),
                else_bb: BasicBlockId::new(2),
                then_edge_args: None,
                else_edge_args: None,
            });
        func.get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: branch_local,
                value: ConstValue::Integer(1),
            });
        func.get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(3),
                edge_args: None,
            });
        func.get_block_mut(BasicBlockId::new(2))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(3),
                edge_args: None,
            });
        func.get_block_mut(BasicBlockId::new(3))
            .unwrap()
            .add_instruction(MirInstruction::Phi {
                dst: phi,
                inputs: vec![(BasicBlockId::new(1), branch_local)],
                type_hint: None,
            });

        let changed = complete_missing_self_carried_phi_inputs(&mut func);
        assert_eq!(changed, 0);

        let merge = func.get_block(BasicBlockId::new(3)).unwrap();
        let MirInstruction::Phi { inputs, .. } = &merge.instructions[0] else {
            panic!("expected phi");
        };
        assert_eq!(inputs.len(), 1);
    }

    #[test]
    fn completes_missing_input_with_single_dominating_existing_incoming() {
        let mut func = MirFunction::new(test_signature("unchanged_edge"), BasicBlockId::new(0));
        func.add_block(BasicBlock::new(BasicBlockId::new(1)));
        func.add_block(BasicBlock::new(BasicBlockId::new(2)));
        func.add_block(BasicBlock::new(BasicBlockId::new(3)));

        let seed = func.next_value_id();
        let phi = func.next_value_id();
        func.get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: seed,
                value: ConstValue::Integer(0),
            });
        func.get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Branch {
                condition: seed,
                then_bb: BasicBlockId::new(1),
                else_bb: BasicBlockId::new(2),
                then_edge_args: None,
                else_edge_args: None,
            });
        func.get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(3),
                edge_args: None,
            });
        func.get_block_mut(BasicBlockId::new(2))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(3),
                edge_args: None,
            });
        func.get_block_mut(BasicBlockId::new(3))
            .unwrap()
            .add_instruction(MirInstruction::Phi {
                dst: phi,
                inputs: vec![(BasicBlockId::new(1), seed)],
                type_hint: None,
            });

        let changed = complete_missing_self_carried_phi_inputs(&mut func);
        assert_eq!(changed, 1);

        let merge = func.get_block(BasicBlockId::new(3)).unwrap();
        let MirInstruction::Phi { inputs, .. } = &merge.instructions[0] else {
            panic!("expected phi");
        };
        assert!(inputs.contains(&(BasicBlockId::new(2), seed)));
    }

    #[test]
    fn prunes_unused_phi_before_missing_input_validation() {
        let mut func = MirFunction::new(test_signature("unused_phi"), BasicBlockId::new(0));
        func.add_block(BasicBlock::new(BasicBlockId::new(1)));
        func.add_block(BasicBlock::new(BasicBlockId::new(2)));
        func.add_block(BasicBlock::new(BasicBlockId::new(3)));

        let cond = func.next_value_id();
        let branch_local = func.next_value_id();
        let phi = func.next_value_id();
        func.get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: cond,
                value: ConstValue::Integer(0),
            });
        func.get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Branch {
                condition: cond,
                then_bb: BasicBlockId::new(1),
                else_bb: BasicBlockId::new(2),
                then_edge_args: None,
                else_edge_args: None,
            });
        func.get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: branch_local,
                value: ConstValue::Integer(1),
            });
        func.get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(3),
                edge_args: None,
            });
        func.get_block_mut(BasicBlockId::new(2))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(3),
                edge_args: None,
            });
        func.get_block_mut(BasicBlockId::new(3))
            .unwrap()
            .add_instruction(MirInstruction::Phi {
                dst: phi,
                inputs: vec![(BasicBlockId::new(1), branch_local)],
                type_hint: None,
            });

        let changed = materialize_all_phi_inputs(&mut func, "test").unwrap();
        assert_eq!(changed, 1);
        assert!(func
            .get_block(BasicBlockId::new(3))
            .unwrap()
            .instructions
            .is_empty());
    }

    #[test]
    fn rematerializes_runtime_data_substring_for_phi_pred() {
        let mut func = MirFunction::new(test_signature("substring_phi"), BasicBlockId::new(0));
        func.add_block(BasicBlock::new(BasicBlockId::new(1)));
        func.add_block(BasicBlock::new(BasicBlockId::new(2)));

        let text = func.next_value_id();
        let start = func.next_value_id();
        let end = func.next_value_id();
        let substring = func.next_value_id();

        {
            let entry = func.get_block_mut(BasicBlockId::new(0)).unwrap();
            entry.add_instruction(MirInstruction::Const {
                dst: text,
                value: ConstValue::String("abcdef".to_string()),
            });
            entry.add_instruction(MirInstruction::Const {
                dst: start,
                value: ConstValue::Integer(1),
            });
            entry.add_instruction(MirInstruction::Const {
                dst: end,
                value: ConstValue::Integer(3),
            });
            entry.set_terminator(MirInstruction::Branch {
                condition: start,
                then_bb: BasicBlockId::new(1),
                else_bb: BasicBlockId::new(2),
                then_edge_args: None,
                else_edge_args: None,
            });
        }

        func.get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .add_instruction(MirInstruction::Call {
                dst: Some(substring),
                func: ValueId::new(u32::MAX),
                callee: Some(Callee::Method {
                    box_name: "RuntimeDataBox".to_string(),
                    method: "substring".to_string(),
                    receiver: Some(text),
                    certainty: TypeCertainty::Union,
                    box_kind: CalleeBoxKind::RuntimeData,
                }),
                args: vec![start, end],
                effects: EffectMask::READ,
            });

        let materialized =
            for_pred(&mut func, BasicBlockId::new(2), substring, "test", "phi").unwrap();
        assert_ne!(materialized, substring);

        let pred = func.get_block(BasicBlockId::new(2)).unwrap();
        assert!(matches!(
            pred.instructions.last(),
            Some(MirInstruction::Call {
                dst: Some(dst),
                callee: Some(Callee::Method { method, .. }),
                ..
            }) if *dst == materialized && method == "substring"
        ));
    }
}
