//! Borrowed string corridor sinking pilot.
//!
//! First real transform for the string corridor lane:
//! `substring(...).length()` is rewritten into a single direct extern call so
//! the corridor can stay borrowed without forcing publication/materialization.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::mir::{
    refresh_function_string_corridor_candidates, refresh_function_string_corridor_facts,
    BasicBlockId, Callee, EffectMask, MirFunction, MirInstruction, MirModule,
    StringCorridorOp, ValueId,
};

pub const SUBSTRING_LEN_EXTERN: &str = "nyash.string.substring_len_hii";

pub fn sink_borrowed_string_corridors(module: &mut MirModule) -> usize {
    let mut rewritten = 0usize;
    for (_name, function) in &mut module.functions {
        rewritten += sink_borrowed_string_corridors_in_function(function);
    }
    rewritten
}

fn sink_borrowed_string_corridors_in_function(function: &mut MirFunction) -> usize {
    refresh_function_string_corridor_facts(function);

    let def_map = build_def_map(function);
    let use_counts = build_use_counts(function);
    let plans_by_block = collect_plans(function, &def_map, &use_counts);
    apply_plans(function, plans_by_block)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SubstringLenPlan {
    inner_idx: usize,
    inner_dst: ValueId,
    outer_idx: usize,
    outer_dst: ValueId,
    source: ValueId,
    start: ValueId,
    end: ValueId,
    effects: EffectMask,
}

fn build_def_map(function: &MirFunction) -> HashMap<ValueId, (BasicBlockId, usize)> {
    let mut defs: HashMap<ValueId, (BasicBlockId, usize)> = HashMap::new();
    for (bbid, block) in &function.blocks {
        for (idx, inst) in block.instructions.iter().enumerate() {
            if let Some(dst) = inst.dst_value() {
                defs.insert(dst, (*bbid, idx));
            }
        }
    }
    defs
}

fn build_use_counts(function: &MirFunction) -> HashMap<ValueId, usize> {
    let mut uses: HashMap<ValueId, usize> = HashMap::new();
    for block in function.blocks.values() {
        for inst in &block.instructions {
            for vid in inst.used_values() {
                *uses.entry(vid).or_insert(0) += 1;
            }
        }
        if let Some(term) = &block.terminator {
            for vid in term.used_values() {
                *uses.entry(vid).or_insert(0) += 1;
            }
        }
        for edge in block.out_edges() {
            if let Some(args) = edge.args {
                for vid in args.values {
                    *uses.entry(vid).or_insert(0) += 1;
                }
            }
        }
    }
    uses
}

fn collect_plans(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> BTreeMap<BasicBlockId, Vec<SubstringLenPlan>> {
    let mut plans_by_block: BTreeMap<BasicBlockId, Vec<SubstringLenPlan>> = BTreeMap::new();

    for (bbid, block) in &function.blocks {
        let mut plans: Vec<SubstringLenPlan> = Vec::new();

        for (outer_idx, inst) in block.instructions.iter().enumerate() {
            let Some((outer_dst, receiver, effects)) = match_len_call(inst) else {
                continue;
            };

            let Some(outer_fact) = function.metadata.string_corridor_facts.get(&outer_dst) else {
                continue;
            };
            if outer_fact.op != StringCorridorOp::StrLen {
                continue;
            }

            if use_counts.get(&receiver).copied().unwrap_or(0) != 1 {
                continue;
            }

            let Some((inner_bbid, inner_idx)) = def_map.get(&receiver).copied() else {
                continue;
            };
            if inner_bbid != *bbid || inner_idx >= outer_idx {
                continue;
            }

            let Some(inner_fact) = function.metadata.string_corridor_facts.get(&receiver) else {
                continue;
            };
            if inner_fact.op != StringCorridorOp::StrSlice {
                continue;
            }

            let Some((source, start, end)) = extract_substring_args(&block.instructions[inner_idx]) else {
                continue;
            };

            plans.push(SubstringLenPlan {
                inner_idx,
                inner_dst: receiver,
                outer_idx,
                outer_dst,
                source,
                start,
                end,
                effects,
            });
        }

        if !plans.is_empty() {
            plans_by_block.insert(*bbid, plans);
        }
    }

    plans_by_block
}

fn match_len_call(inst: &MirInstruction) -> Option<(ValueId, ValueId, EffectMask)> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee:
                Some(Callee::Method {
                    method, receiver: Some(receiver), ..
                }),
            args,
            effects,
            ..
        } if args.is_empty() && matches!(method.as_str(), "length" | "len") => {
            Some((*dst, *receiver, *effects))
        }
        _ => None,
    }
}

fn extract_substring_args(inst: &MirInstruction) -> Option<(ValueId, ValueId, ValueId)> {
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    method, receiver: Some(source), ..
                }),
            args,
            ..
        } if args.len() == 2 && matches!(method.as_str(), "substring" | "slice") => {
            Some((*source, args[0], args[1]))
        }
        MirInstruction::Call {
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if args.len() == 3 && name == "nyash.string.substring_hii" => {
            Some((args[0], args[1], args[2]))
        }
        _ => None,
    }
}

fn apply_plans(
    function: &mut MirFunction,
    plans_by_block: BTreeMap<BasicBlockId, Vec<SubstringLenPlan>>,
) -> usize {
    let mut rewritten = 0usize;

    for (bbid, plans) in plans_by_block {
        if plans.is_empty() {
            continue;
        }
        let Some(block) = function.blocks.get_mut(&bbid) else {
            continue;
        };

        let mut remove_indices: BTreeSet<usize> = BTreeSet::new();
        let mut replacements: BTreeMap<usize, MirInstruction> = BTreeMap::new();
        let mut removed_values: Vec<ValueId> = Vec::new();

        for plan in plans {
            remove_indices.insert(plan.inner_idx);
            removed_values.push(plan.inner_dst);
            replacements.insert(
                plan.outer_idx,
                MirInstruction::Call {
                    dst: Some(plan.outer_dst),
                    func: ValueId::INVALID,
                    callee: Some(Callee::Extern(SUBSTRING_LEN_EXTERN.to_string())),
                    args: vec![plan.source, plan.start, plan.end],
                    effects: plan.effects,
                },
            );
            function.metadata.optimization_hints.push(format!(
                "string_corridor_sink:substring_len_hii:%{}",
                plan.outer_dst.0
            ));
            rewritten += 1;
        }

        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let mut new_insts = Vec::with_capacity(insts.len());
        let mut new_spans = Vec::with_capacity(spans.len());

        for (idx, (inst, span)) in insts.into_iter().zip(spans.into_iter()).enumerate() {
            if remove_indices.contains(&idx) {
                continue;
            }
            if let Some(rewritten_inst) = replacements.get(&idx) {
                new_insts.push(rewritten_inst.clone());
                new_spans.push(span);
            } else {
                new_insts.push(inst);
                new_spans.push(span);
            }
        }

        block.instructions = new_insts;
        block.instruction_spans = new_spans;

        for vid in removed_values {
            function.metadata.value_types.remove(&vid);
            function.metadata.value_origin_callers.remove(&vid);
        }
    }

    if rewritten > 0 {
        function.update_cfg();
        refresh_function_string_corridor_facts(function);
        refresh_function_string_corridor_candidates(function);
    }
    rewritten
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirModule, MirType};

    fn method_call(
        dst: ValueId,
        receiver: ValueId,
        box_name: &str,
        method: &str,
        args: Vec<ValueId>,
        ty: MirType,
    ) -> MirInstruction {
        let _ = ty;
        MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }

    #[test]
    fn rewrites_single_use_substring_length_chain_to_direct_extern() {
        let mut module = MirModule::new("substring_len".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string()), MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(3),
            ValueId(0),
            "StringBox",
            "substring",
            vec![ValueId(1), ValueId(2)],
            MirType::Box("StringBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(4),
            ValueId(3),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(4)),
        });

        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Box("StringBox".to_string()));
        function.metadata.value_types.insert(ValueId(4), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 1);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert_eq!(block.instructions.len(), block.instruction_spans.len());
        assert_eq!(block.instructions.len(), 1);
        match &block.instructions[0] {
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Extern(name)),
                args,
                ..
            } => {
                assert_eq!(*dst, Some(ValueId(4)));
                assert_eq!(name, SUBSTRING_LEN_EXTERN);
                assert_eq!(args, &vec![ValueId(0), ValueId(1), ValueId(2)]);
            }
            other => panic!("expected direct extern rewrite, got {other:?}"),
        }
    }

    #[test]
    fn keeps_substring_when_result_has_multiple_uses() {
        let mut module = MirModule::new("substring_len_multiuse".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string()), MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(3),
            ValueId(0),
            "StringBox",
            "substring",
            vec![ValueId(1), ValueId(2)],
            MirType::Box("StringBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(4),
            ValueId(3),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(5),
            ValueId(3),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(4)),
        });

        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Box("StringBox".to_string()));
        function.metadata.value_types.insert(ValueId(4), MirType::Integer);
        function.metadata.value_types.insert(ValueId(5), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 0);
    }
}
