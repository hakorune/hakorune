//! String concat chain canonicalization for MIR.
//!
//! Rewrites one-level string `BinOp::Add` chains:
//! - `(a + b) + c`
//! - `a + (b + c)`
//!
//! into:
//! `Call Extern("nyash.string.concat3_hhh", [a, b, c])`
//!
//! Contract:
//! - Only rewrites when the inner `Add` value is single-use by the outer `Add`.
//! - Removes the folded inner instruction in the same pass (span-aligned).

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::mir::{
    BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, MirFunction, MirInstruction, MirModule,
    MirType, ValueId,
};

const CONCAT3_EXTERN: &str = "nyash.string.concat3_hhh";
const CONCAT_HH_EXTERN: &str = "nyash.string.concat_hh";

#[derive(Debug, Clone)]
struct Concat3Plan {
    outer_idx: usize,
    inner_idx: usize,
    inner_dst: ValueId,
    outer_dst: ValueId,
    args: [ValueId; 3],
}

/// Canonicalize string concat chains to `concat3_hhh`.
///
/// Returns number of outer instructions rewritten.
pub fn canonicalize_string_concat3(module: &mut MirModule) -> usize {
    let mut rewritten = 0usize;
    for (_name, func) in &mut module.functions {
        rewritten += canonicalize_in_function(func);
    }
    rewritten
}

fn canonicalize_in_function(function: &mut MirFunction) -> usize {
    let stringish = infer_stringish_values(function);
    let def_map = build_def_map(function);
    let use_counts = build_use_counts(function);

    let mut plans_by_block: BTreeMap<BasicBlockId, Vec<Concat3Plan>> = BTreeMap::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for bbid in block_ids {
        let Some(block) = function.blocks.get(&bbid) else {
            continue;
        };
        let mut claimed: BTreeSet<usize> = BTreeSet::new();

        for outer_idx in 0..block.instructions.len() {
            if claimed.contains(&outer_idx) {
                continue;
            }

            let (outer_dst, outer_lhs, outer_rhs) = match &block.instructions[outer_idx] {
                MirInstruction::BinOp {
                    dst,
                    op: BinaryOp::Add,
                    lhs,
                    rhs,
                } => (*dst, *lhs, *rhs),
                _ => continue,
            };

            if !is_string_add_candidate(outer_dst, outer_lhs, outer_rhs, &stringish) {
                continue;
            }

            let picked = pick_inner_concat_candidate(
                block,
                bbid,
                outer_idx,
                outer_lhs,
                outer_rhs,
                &stringish,
                &def_map,
                &use_counts,
                &claimed,
            );

            let Some((inner_idx, inner_dst, args)) = picked else {
                continue;
            };

            claimed.insert(inner_idx);
            claimed.insert(outer_idx);
            plans_by_block.entry(bbid).or_default().push(Concat3Plan {
                outer_idx,
                inner_idx,
                inner_dst,
                outer_dst,
                args,
            });
        }
    }

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
                    callee: Some(Callee::Extern(CONCAT3_EXTERN.to_string())),
                    args: plan.args.to_vec(),
                    effects: EffectMask::PURE,
                },
            );
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
        }
    }

    rewritten
}

fn pick_inner_concat_candidate(
    block: &crate::mir::BasicBlock,
    bbid: BasicBlockId,
    outer_idx: usize,
    outer_lhs: ValueId,
    outer_rhs: ValueId,
    stringish: &HashSet<ValueId>,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
    claimed: &BTreeSet<usize>,
) -> Option<(usize, ValueId, [ValueId; 3])> {
    // Left-assoc first: (a + b) + c
    if let Some(candidate) = inspect_inner_add(
        block, bbid, outer_idx, outer_lhs, stringish, def_map, use_counts, claimed,
    ) {
        let (inner_idx, inner_dst, il, ir) = candidate;
        return Some((inner_idx, inner_dst, [il, ir, outer_rhs]));
    }

    // Right-assoc: a + (b + c)
    if let Some(candidate) = inspect_inner_add(
        block, bbid, outer_idx, outer_rhs, stringish, def_map, use_counts, claimed,
    ) {
        let (inner_idx, inner_dst, il, ir) = candidate;
        return Some((inner_idx, inner_dst, [outer_lhs, il, ir]));
    }

    None
}

fn inspect_inner_add(
    block: &crate::mir::BasicBlock,
    bbid: BasicBlockId,
    outer_idx: usize,
    inner_vid: ValueId,
    stringish: &HashSet<ValueId>,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
    claimed: &BTreeSet<usize>,
) -> Option<(usize, ValueId, ValueId, ValueId)> {
    let (def_bbid, inner_idx) = *def_map.get(&inner_vid)?;
    if def_bbid != bbid || inner_idx >= outer_idx || claimed.contains(&inner_idx) {
        return None;
    }

    let (inner_dst, il, ir) = match &block.instructions[inner_idx] {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } => (*dst, *lhs, *rhs),
        _ => return None,
    };

    if inner_dst != inner_vid {
        return None;
    }
    if use_counts.get(&inner_dst).copied().unwrap_or(0) != 1 {
        return None;
    }
    if !is_string_add_candidate(inner_dst, il, ir, stringish) {
        return None;
    }

    Some((inner_idx, inner_dst, il, ir))
}

fn is_string_add_candidate(
    dst: ValueId,
    lhs: ValueId,
    rhs: ValueId,
    stringish: &HashSet<ValueId>,
) -> bool {
    stringish.contains(&dst) || stringish.contains(&lhs) || stringish.contains(&rhs)
}

fn is_stringish_type(ty: &MirType) -> bool {
    match ty {
        MirType::String => true,
        MirType::Box(name) if name == "StringBox" => true,
        _ => false,
    }
}

fn is_string_concat_symbol(name: &str) -> bool {
    name == CONCAT_HH_EXTERN || name == CONCAT3_EXTERN
}

fn infer_stringish_values(function: &MirFunction) -> HashSet<ValueId> {
    let mut out: HashSet<ValueId> = HashSet::new();

    for (vid, ty) in &function.metadata.value_types {
        if is_stringish_type(ty) {
            out.insert(*vid);
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let inserted = match inst {
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::String(_),
                    } => out.insert(*dst),
                    MirInstruction::Copy { dst, src } if out.contains(src) => out.insert(*dst),
                    MirInstruction::BinOp {
                        dst,
                        op: BinaryOp::Add,
                        lhs,
                        rhs,
                    } if out.contains(lhs) || out.contains(rhs) => out.insert(*dst),
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Extern(name) | Callee::Global(name)),
                        ..
                    } if is_string_concat_symbol(name) => out.insert(*dst),
                    _ => false,
                };
                if inserted {
                    changed = true;
                }
            }
        }
    }

    out
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

#[cfg(test)]
mod tests {
    use super::canonicalize_string_concat3;
    use crate::ast::Span;
    use crate::mir::{
        BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirModule, MirType, ValueId,
    };

    fn build_concat3_chain_module(right_assoc: bool) -> MirModule {
        let mut module = MirModule::new("concat3".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));
        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");

        block.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::String("ha".to_string()),
        });
        block.instruction_spans.push(Span::unknown());

        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::String("ko".to_string()),
        });
        block.instruction_spans.push(Span::unknown());

        block.instructions.push(MirInstruction::Const {
            dst: ValueId(3),
            value: ConstValue::String("run".to_string()),
        });
        block.instruction_spans.push(Span::unknown());

        if right_assoc {
            // %4 = %2 + %3
            block.instructions.push(MirInstruction::BinOp {
                dst: ValueId(4),
                op: BinaryOp::Add,
                lhs: ValueId(2),
                rhs: ValueId(3),
            });
            block.instruction_spans.push(Span::unknown());
            // %5 = %1 + %4
            block.instructions.push(MirInstruction::BinOp {
                dst: ValueId(5),
                op: BinaryOp::Add,
                lhs: ValueId(1),
                rhs: ValueId(4),
            });
            block.instruction_spans.push(Span::unknown());
        } else {
            // %4 = %1 + %2
            block.instructions.push(MirInstruction::BinOp {
                dst: ValueId(4),
                op: BinaryOp::Add,
                lhs: ValueId(1),
                rhs: ValueId(2),
            });
            block.instruction_spans.push(Span::unknown());
            // %5 = %4 + %3
            block.instructions.push(MirInstruction::BinOp {
                dst: ValueId(5),
                op: BinaryOp::Add,
                lhs: ValueId(4),
                rhs: ValueId(3),
            });
            block.instruction_spans.push(Span::unknown());
        }

        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(5)),
        });

        func.metadata
            .value_types
            .insert(ValueId(1), MirType::String);
        func.metadata
            .value_types
            .insert(ValueId(2), MirType::String);
        func.metadata
            .value_types
            .insert(ValueId(3), MirType::String);
        func.metadata
            .value_types
            .insert(ValueId(4), MirType::String);
        func.metadata
            .value_types
            .insert(ValueId(5), MirType::String);

        module.add_function(func);
        module
    }

    #[test]
    fn rewrites_left_assoc_chain_to_concat3() {
        let mut module = build_concat3_chain_module(false);
        let rewritten = canonicalize_string_concat3(&mut module);
        assert_eq!(rewritten, 1);

        let block = &module
            .get_function("main")
            .expect("main exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry exists");
        assert_eq!(block.instructions.len(), block.instruction_spans.len());

        let mut saw_concat3 = false;
        let mut saw_add = false;
        for inst in &block.instructions {
            match inst {
                MirInstruction::Call {
                    dst,
                    callee: Some(Callee::Extern(name)),
                    args,
                    ..
                } if *dst == Some(ValueId(5)) && name == "nyash.string.concat3_hhh" => {
                    saw_concat3 = true;
                    assert_eq!(args, &vec![ValueId(1), ValueId(2), ValueId(3)]);
                }
                MirInstruction::BinOp {
                    op: BinaryOp::Add, ..
                } => saw_add = true,
                _ => {}
            }
        }
        assert!(saw_concat3);
        assert!(!saw_add, "inner/outer Add should both be eliminated");
    }

    #[test]
    fn rewrites_right_assoc_chain_to_concat3() {
        let mut module = build_concat3_chain_module(true);
        let rewritten = canonicalize_string_concat3(&mut module);
        assert_eq!(rewritten, 1);

        let block = &module
            .get_function("main")
            .expect("main exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry exists");
        assert_eq!(block.instructions.len(), block.instruction_spans.len());

        let mut saw_concat3 = false;
        let mut saw_add = false;
        for inst in &block.instructions {
            match inst {
                MirInstruction::Call {
                    dst,
                    callee: Some(Callee::Extern(name)),
                    args,
                    ..
                } if *dst == Some(ValueId(5)) && name == "nyash.string.concat3_hhh" => {
                    saw_concat3 = true;
                    assert_eq!(args, &vec![ValueId(1), ValueId(2), ValueId(3)]);
                }
                MirInstruction::BinOp {
                    op: BinaryOp::Add, ..
                } => saw_add = true,
                _ => {}
            }
        }
        assert!(saw_concat3);
        assert!(!saw_add, "inner/outer Add should both be eliminated");
    }

    #[test]
    fn keeps_non_chain_add_unchanged() {
        let mut module = MirModule::new("concat3_non_chain".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId(0));
        let block = func
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry block exists");

        block.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::String("ha".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::String("ko".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(3)),
        });

        module.add_function(func);

        let rewritten = canonicalize_string_concat3(&mut module);
        assert_eq!(rewritten, 0);
    }
}
