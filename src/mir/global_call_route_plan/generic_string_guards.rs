use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{BasicBlockId, CompareOp, ConstValue, MirFunction, MirInstruction, ValueId};

#[derive(Debug, Clone, Copy)]
struct NonVoidStringGuard {
    predecessor: BasicBlockId,
    successor: BasicBlockId,
    value_root: ValueId,
}

pub(super) fn generic_pure_string_non_void_guard_phi_values(
    function: &MirFunction,
) -> BTreeSet<ValueId> {
    let aliases = generic_pure_string_copy_aliases(function);
    let void_roots = generic_pure_string_void_const_roots(function, &aliases);
    let compare_guards =
        generic_pure_string_non_void_compare_guards(function, &aliases, &void_roots);
    let branch_guards =
        generic_pure_string_non_void_branch_guards(function, &aliases, &compare_guards);
    let mut guarded_values = BTreeSet::new();

    for guard in branch_guards {
        let Some(block) = function.blocks.get(&guard.successor) else {
            continue;
        };
        for instruction in &block.instructions {
            let MirInstruction::Phi { dst, inputs, .. } = instruction else {
                continue;
            };
            if inputs.iter().any(|(pred, value)| {
                *pred == guard.predecessor
                    && generic_pure_string_resolve_alias(&aliases, *value) == guard.value_root
            }) {
                guarded_values.insert(*dst);
            }
        }
    }

    guarded_values
}

fn generic_pure_string_copy_aliases(function: &MirFunction) -> BTreeMap<ValueId, ValueId> {
    let mut aliases = BTreeMap::new();
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            if let MirInstruction::Copy { dst, src } = instruction {
                aliases.insert(*dst, *src);
            }
        }
    }
    aliases
}

fn generic_pure_string_void_const_roots(
    function: &MirFunction,
    aliases: &BTreeMap<ValueId, ValueId>,
) -> BTreeSet<ValueId> {
    let mut void_roots = BTreeSet::new();
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            if let MirInstruction::Const {
                dst,
                value: ConstValue::Null | ConstValue::Void,
            } = instruction
            {
                void_roots.insert(generic_pure_string_resolve_alias(aliases, *dst));
            }
        }
    }
    void_roots
}

fn generic_pure_string_non_void_compare_guards(
    function: &MirFunction,
    aliases: &BTreeMap<ValueId, ValueId>,
    void_roots: &BTreeSet<ValueId>,
) -> BTreeMap<ValueId, (CompareOp, ValueId)> {
    let mut guards = BTreeMap::new();
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            let MirInstruction::Compare { dst, op, lhs, rhs } = instruction else {
                continue;
            };
            if !matches!(op, CompareOp::Eq | CompareOp::Ne) {
                continue;
            }
            let lhs_root = generic_pure_string_resolve_alias(aliases, *lhs);
            let rhs_root = generic_pure_string_resolve_alias(aliases, *rhs);
            let value_root = if void_roots.contains(&lhs_root) && !void_roots.contains(&rhs_root) {
                rhs_root
            } else if void_roots.contains(&rhs_root) && !void_roots.contains(&lhs_root) {
                lhs_root
            } else {
                continue;
            };
            guards.insert(
                generic_pure_string_resolve_alias(aliases, *dst),
                (*op, value_root),
            );
        }
    }
    guards
}

fn generic_pure_string_non_void_branch_guards(
    function: &MirFunction,
    aliases: &BTreeMap<ValueId, ValueId>,
    compare_guards: &BTreeMap<ValueId, (CompareOp, ValueId)>,
) -> Vec<NonVoidStringGuard> {
    let mut guards = Vec::new();
    for (block_id, block) in &function.blocks {
        let Some(MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            ..
        }) = &block.terminator
        else {
            continue;
        };
        let condition_root = generic_pure_string_resolve_alias(aliases, *condition);
        let Some((op, value_root)) = compare_guards.get(&condition_root).copied() else {
            continue;
        };
        let successor = match op {
            CompareOp::Ne => *then_bb,
            CompareOp::Eq => *else_bb,
            _ => continue,
        };
        guards.push(NonVoidStringGuard {
            predecessor: *block_id,
            successor,
            value_root,
        });
    }
    guards
}

fn generic_pure_string_resolve_alias(
    aliases: &BTreeMap<ValueId, ValueId>,
    value: ValueId,
) -> ValueId {
    let mut current = value;
    for _ in 0..=aliases.len() {
        let Some(next) = aliases.get(&current).copied() else {
            break;
        };
        if next == current {
            break;
        }
        current = next;
    }
    current
}
