use crate::ast::Span;
use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, ValueId};

fn next_block_id(f: &MirFunction) -> BasicBlockId {
    let mut mx = 0u32;
    for k in f.blocks.keys() {
        if k.0 >= mx {
            mx = k.0 + 1;
        }
    }
    BasicBlockId::new(mx)
}

/// Create a fresh basic block and insert it into the function.
pub(super) fn new_block(f: &mut MirFunction) -> BasicBlockId {
    let id = next_block_id(f);
    f.add_block(BasicBlock::new(id));
    id
}

/// Merge two incoming values either by inserting Copy on predecessor edges
/// (no_phi mode) or by adding a Phi at the merge block head.
pub(super) fn merge_values(
    f: &mut MirFunction,
    no_phi: bool,
    merge_bb: BasicBlockId,
    pred_a: BasicBlockId,
    val_a: ValueId,
    pred_b: BasicBlockId,
    val_b: ValueId,
) -> Result<ValueId, String> {
    if val_a == val_b {
        return Ok(val_a);
    }
    let dst = f.next_value_id();
    if no_phi {
        if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
            let def_blocks = crate::mir::verification::utils::compute_def_blocks(f);
            let dominators = crate::mir::verification::utils::compute_dominators(f);
            let fn_name = f.signature.name.as_str();
            let check = |pred: BasicBlockId, src: ValueId| -> Result<(), String> {
                let def_block = def_blocks.get(&src).copied();
                let dominates = def_block
                    .map(|db| dominators.dominates(db, pred))
                    .unwrap_or(false);
                if !dominates {
                    let def_block_label = def_block
                        .map(|b| format!("{:?}", b))
                        .unwrap_or_else(|| "None".to_string());
                    return Err(format!(
                        "[freeze:contract][json_v0_bridge/non_dominating_copy] fn={} bb={:?} src=%{} def_block={} op=merge_edge_copy",
                        fn_name,
                        pred,
                        src.0,
                        def_block_label
                    ));
                }
                Ok(())
            };
            check(pred_a, val_a)?;
            check(pred_b, val_b)?;
        }
        // Insert edge-copy BEFORE terminator (critical fix for ValueId undefined errors)
        copy_emitter::emit_copy_before_terminator(
            f,
            pred_a,
            dst,
            val_a,
            CopyEmitReason::JsonV0BridgeMergeEdgeCopy,
        )?;
        copy_emitter::emit_copy_before_terminator(
            f,
            pred_b,
            dst,
            val_b,
            CopyEmitReason::JsonV0BridgeMergeEdgeCopy,
        )?;
    } else {
        crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
            f,
            merge_bb,
            dst,
            vec![(pred_a, val_a), (pred_b, val_b)],
            Span::unknown(),
        )?;
    }
    Ok(dst)
}

/// Merge then/else variable maps into `out_vars` using either PHI at merge
/// or edge-copies in no-phi mode.
pub(super) fn merge_var_maps(
    f: &mut MirFunction,
    no_phi: bool,
    merge_bb: BasicBlockId,
    then_end: BasicBlockId,
    else_end: BasicBlockId,
    then_vars: std::collections::BTreeMap<String, ValueId>,
    else_vars: std::collections::BTreeMap<String, ValueId>,
    base_vars: std::collections::BTreeMap<String, ValueId>,
    out_vars: &mut std::collections::BTreeMap<String, ValueId>,
) -> Result<(), String> {
    use std::collections::BTreeSet;
    let mut names: BTreeSet<String> = base_vars.keys().cloned().collect();
    for k in then_vars.keys() {
        names.insert(k.clone());
    }
    for k in else_vars.keys() {
        names.insert(k.clone());
    }
    for name in names {
        let tv = then_vars.get(&name).copied();
        let ev = else_vars.get(&name).copied();
        let exists_base = base_vars.contains_key(&name);
        match (tv, ev, exists_base) {
            (Some(tval), Some(eval), _) => {
                let merged = merge_values(f, no_phi, merge_bb, then_end, tval, else_end, eval)?;
                out_vars.insert(name, merged);
            }
            (Some(tval), None, true) => {
                if let Some(&bval) = base_vars.get(&name) {
                    let merged = merge_values(f, no_phi, merge_bb, then_end, tval, else_end, bval)?;
                    out_vars.insert(name, merged);
                }
            }
            (None, Some(eval), true) => {
                if let Some(&bval) = base_vars.get(&name) {
                    let merged = merge_values(f, no_phi, merge_bb, then_end, bval, else_end, eval)?;
                    out_vars.insert(name, merged);
                }
            }
            _ => {}
        }
    }
    Ok(())
}
