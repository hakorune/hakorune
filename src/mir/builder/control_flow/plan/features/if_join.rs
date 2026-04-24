//! JoinFeature: apply CoreIfJoin phis at merge block.

use crate::mir::builder::control_flow::plan::CoreIfJoin;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ConstValue, MirInstruction, ValueId};
use std::collections::{HashMap, HashSet};

fn value_carries_pre(
    pre_val: ValueId,
    pre_root: Option<ValueId>,
    branch_val: ValueId,
    branch_root: Option<ValueId>,
) -> bool {
    branch_val == pre_val || (pre_root.is_some() && branch_root == pre_root)
}

fn should_log_carry_reset_to_init(
    pre_is_zero: bool,
    pre_val: ValueId,
    pre_root: Option<ValueId>,
    then_val: ValueId,
    then_is_zero: bool,
    then_root: Option<ValueId>,
    else_val: ValueId,
    else_is_zero: bool,
    else_root: Option<ValueId>,
) -> bool {
    if pre_is_zero {
        return false;
    }

    (then_is_zero && value_carries_pre(pre_val, pre_root, else_val, else_root))
        || (else_is_zero && value_carries_pre(pre_val, pre_root, then_val, then_root))
}

#[track_caller]
pub(in crate::mir::builder) fn apply_if_joins(
    builder: &mut MirBuilder,
    joins: &[CoreIfJoin],
    then_reaches_merge: bool,
    else_reaches_merge: bool,
    then_end_bb: Option<BasicBlockId>,
    else_end_bb: Option<BasicBlockId>,
) -> Result<(), String> {
    if joins.is_empty() {
        return Ok(());
    }

    let strict_planner_required_debug =
        crate::config::env::joinir_dev::strict_planner_required_debug_enabled();

    let mut def_blocks = None;
    let mut dominators = None;
    let mut fn_name = None;
    let mut merge_bb = None;
    let mut caller = None;
    let mut const_ints: Option<HashMap<ValueId, i64>> = None;
    let mut copy_srcs: Option<HashMap<ValueId, ValueId>> = None;

    if strict_planner_required_debug {
        let func =
            builder.scope_ctx.current_function.as_ref().ok_or_else(|| {
                "[if_join] No current function for join dominance check".to_string()
            })?;
        def_blocks = Some(crate::mir::verification::utils::compute_def_blocks(func));
        dominators = Some(crate::mir::verification::utils::compute_dominators(func));
        fn_name = Some(func.signature.name.clone());
        merge_bb = builder.current_block;
        let loc = std::panic::Location::caller();
        caller = Some(format!("{}:{}:{}", loc.file(), loc.line(), loc.column()));
    }

    let (release_def_blocks, release_dominators) = if strict_planner_required_debug {
        (None, None)
    } else if let Some(func) = builder.scope_ctx.current_function.as_ref() {
        (
            Some(crate::mir::verification::utils::compute_def_blocks(func)),
            Some(crate::mir::verification::utils::compute_dominators(func)),
        )
    } else {
        (None, None)
    };

    for join in joins {
        let mut then_in = join.then_val;
        let mut else_in = join.else_val;
        let mut then_reaches_merge_local = then_reaches_merge;
        let mut else_reaches_merge_local = else_reaches_merge;

        if let (Some(def_blocks), Some(dominators)) =
            (release_def_blocks.as_ref(), release_dominators.as_ref())
        {
            let fallback_incoming =
                |incoming: &mut ValueId,
                 branch_reaches_merge: &mut bool,
                 pred: Option<BasicBlockId>| {
                    let Some(pred) = pred else {
                        *branch_reaches_merge = false;
                        return;
                    };
                    let incoming_ok = def_blocks
                        .get(incoming)
                        .copied()
                        .map(|def_bb| dominators.dominates(def_bb, pred))
                        .unwrap_or(false);
                    if incoming_ok {
                        return;
                    }
                    if let Some(pre_val) = join.pre_val {
                        let pre_ok = def_blocks
                            .get(&pre_val)
                            .copied()
                            .map(|def_bb| dominators.dominates(def_bb, pred))
                            .unwrap_or(false);
                        if pre_ok {
                            *incoming = pre_val;
                            return;
                        }
                    }
                    *branch_reaches_merge = false;
                };

            if then_reaches_merge_local {
                fallback_incoming(&mut then_in, &mut then_reaches_merge_local, then_end_bb);
            }
            if else_reaches_merge_local {
                fallback_incoming(&mut else_in, &mut else_reaches_merge_local, else_end_bb);
            }
        }

        let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
        let mut then_pred = None;
        let mut else_pred = None;
        if then_reaches_merge_local {
            let pred = then_end_bb
                .ok_or_else(|| "[lowerer] Missing then end block for CorePlan::If".to_string())?;
            then_pred = Some(pred);
            inputs.push((pred, then_in));
        }
        if else_reaches_merge_local {
            let pred = else_end_bb
                .ok_or_else(|| "[lowerer] Missing else end block for CorePlan::If".to_string())?;
            else_pred = Some(pred);
            inputs.push((pred, else_in));
        }
        if inputs.is_empty() {
            continue;
        }
        let type_hint = builder.type_ctx.get_type(join.dst).cloned();

        if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
            let fn_name = builder
                .scope_ctx
                .current_function
                .as_ref()
                .map(|f| f.signature.name.as_str())
                .unwrap_or("<none>");
            let merge_bb = builder.current_block;
            let caller = std::panic::Location::caller();
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[if_join/emit_phi] fn={} merge_bb={:?} join={} dst=%{} then_pred={:?} then_in=%{} else_pred={:?} else_in=%{} caller={}:{}:{}",
                fn_name,
                merge_bb,
                join.name,
                join.dst.0,
                then_pred,
                then_in.0,
                else_pred,
                else_in.0,
                caller.file(),
                caller.line(),
                caller.column()
            ));
        }

        if strict_planner_required_debug {
            let def_blocks = def_blocks.as_ref().unwrap();
            let dominators = dominators.as_ref().unwrap();
            let fn_name = fn_name.as_ref().unwrap();
            let merge_bb = merge_bb;
            let caller = caller.as_ref().unwrap();

            let then_def_bb = then_pred.and_then(|_| def_blocks.get(&then_in).copied());
            let else_def_bb = else_pred.and_then(|_| def_blocks.get(&else_in).copied());

            {
                let func = builder.scope_ctx.current_function.as_ref().ok_or_else(|| {
                    "[if_join] No current function for join carry check".to_string()
                })?;
                if const_ints.is_none() || copy_srcs.is_none() {
                    let mut const_map = HashMap::new();
                    let mut copy_map = HashMap::new();
                    for bb in func.blocks.values() {
                        for inst in &bb.instructions {
                            match inst {
                                MirInstruction::Const { dst, value } => {
                                    if let ConstValue::Integer(n) = value {
                                        const_map.insert(*dst, *n);
                                    }
                                }
                                MirInstruction::Copy { dst, src } => {
                                    copy_map.insert(*dst, *src);
                                }
                                _ => {}
                            }
                        }
                    }
                    const_ints = Some(const_map);
                    copy_srcs = Some(copy_map);
                }
                let const_ints = const_ints.as_ref().unwrap();
                let copy_srcs = copy_srcs.as_ref().unwrap();

                let resolve_const_int = |start: ValueId| -> (Option<i64>, Option<ValueId>) {
                    let mut cur = start;
                    let mut visited: HashSet<ValueId> = HashSet::new();
                    for _ in 0..64 {
                        if !visited.insert(cur) {
                            break;
                        }
                        if let Some(n) = const_ints.get(&cur) {
                            return (Some(*n), Some(cur));
                        }
                        if let Some(next) = copy_srcs.get(&cur) {
                            cur = *next;
                            continue;
                        }
                        break;
                    }
                    (None, None)
                };

                // Only joins originating from join_payload (3-map diff) participate in
                // "carry reset" checks. Synthetic joins (pre_val=None) don't have a
                // meaningful pre-if binding to compare against.
                if let Some(pre_val) = join.pre_val {
                    let (pre_const, pre_root) = resolve_const_int(pre_val);
                    let (then_const, then_root) = resolve_const_int(then_in);
                    let (else_const, else_root) = resolve_const_int(else_in);

                    let pre_is_zero = pre_const == Some(0);
                    let then_is_zero = then_const == Some(0);
                    let else_is_zero = else_const == Some(0);

                    let should_log = should_log_carry_reset_to_init(
                        pre_is_zero,
                        pre_val,
                        pre_root,
                        then_in,
                        then_is_zero,
                        then_root,
                        else_in,
                        else_is_zero,
                        else_root,
                    );

                    if should_log && crate::config::env::joinir_dev::debug_enabled() {
                        let then_const_str = then_const
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "none".to_string());
                        let else_const_str = else_const
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "none".to_string());
                        let pre_const_str = pre_const
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "none".to_string());
                        let then_root_str = then_root
                            .map(|v| format!("%{}", v.0))
                            .unwrap_or_else(|| "none".to_string());
                        let else_root_str = else_root
                            .map(|v| format!("%{}", v.0))
                            .unwrap_or_else(|| "none".to_string());
                        let pre_root_str = pre_root
                            .map(|v| format!("%{}", v.0))
                            .unwrap_or_else(|| "none".to_string());
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[if_join/carry_reset_to_init] fn={} merge_bb={:?} join={} dst=%{} pre_in=%{} pre_const={} pre_root={} then_pred={:?} then_in=%{} then_const={} then_root={} else_pred={:?} else_in=%{} else_const={} else_root={} caller={}",
                            fn_name,
                            merge_bb,
                            join.name,
                            join.dst.0,
                            pre_val.0,
                            pre_const_str,
                            pre_root_str,
                            then_pred,
                            then_in.0,
                            then_const_str,
                            then_root_str,
                            else_pred,
                            else_in.0,
                            else_const_str,
                            else_root_str,
                            caller
                        ));
                    }
                }
            }

            let then_ok = then_pred
                .and_then(|pred| {
                    then_def_bb.and_then(|def_bb| Some(dominators.dominates(def_bb, pred)))
                })
                .unwrap_or(!then_reaches_merge_local);

            let else_ok = else_pred
                .and_then(|pred| {
                    else_def_bb.and_then(|def_bb| Some(dominators.dominates(def_bb, pred)))
                })
                .unwrap_or(!else_reaches_merge_local);

            if !then_ok || !else_ok {
                return Err(format!(
                    "[freeze:contract][if_join/phi_incoming_not_dominating] fn={} merge_bb={:?} join={} dst=%{} then_pred={:?} then_in=%{} then_def_bb={:?} else_pred={:?} else_in=%{} else_def_bb={:?} caller={}",
                    fn_name,
                    merge_bb,
                    join.name,
                    join.dst.0,
                    then_pred,
                    then_in.0,
                    then_def_bb,
                    else_pred,
                    else_in.0,
                    else_def_bb,
                    caller
                ));
            }
        }

        // Fail-fast: check for duplicate PHI dst across ALL blocks (strict/dev+planner_required only)
        if strict_planner_required_debug {
            if let Some(current_bb) = builder.current_block {
                if let Some(func) = &builder.scope_ctx.current_function {
                    // Check ALL blocks for existing PHI with same dst (SSA violation)
                    for (&bb_id, block) in &func.blocks {
                        for inst in &block.instructions {
                            if let MirInstruction::Phi { dst, .. } = inst {
                                if *dst == join.dst {
                                    return Err(format!(
                                        "[freeze:contract][if_join/duplicate_phi_dst] fn={} new_merge_bb={:?} existing_phi_bb={:?} dst=%{:?} join={}",
                                        func.signature.name, current_bb, bb_id, join.dst.0, join.name
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        builder.emit_instruction(MirInstruction::Phi {
            dst: join.dst,
            inputs,
            type_hint,
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::should_log_carry_reset_to_init;
    use crate::mir::ValueId;

    #[test]
    fn carry_reset_logs_when_one_branch_resets_and_other_carries_pre() {
        assert!(should_log_carry_reset_to_init(
            false,
            ValueId::new(10),
            Some(ValueId::new(1)),
            ValueId::new(11),
            true,
            Some(ValueId::new(2)),
            ValueId::new(12),
            false,
            Some(ValueId::new(1)),
        ));
    }

    #[test]
    fn carry_reset_does_not_log_when_both_branches_reset() {
        assert!(!should_log_carry_reset_to_init(
            false,
            ValueId::new(10),
            Some(ValueId::new(1)),
            ValueId::new(11),
            true,
            Some(ValueId::new(2)),
            ValueId::new(12),
            true,
            Some(ValueId::new(3)),
        ));
    }

    #[test]
    fn carry_reset_does_not_log_without_pre_carry_branch() {
        assert!(!should_log_carry_reset_to_init(
            false,
            ValueId::new(10),
            Some(ValueId::new(1)),
            ValueId::new(11),
            true,
            Some(ValueId::new(2)),
            ValueId::new(12),
            false,
            Some(ValueId::new(9)),
        ));
    }

    #[test]
    fn carry_reset_does_not_log_when_pre_is_zero() {
        assert!(!should_log_carry_reset_to_init(
            true,
            ValueId::new(10),
            Some(ValueId::new(1)),
            ValueId::new(11),
            true,
            Some(ValueId::new(2)),
            ValueId::new(12),
            false,
            Some(ValueId::new(1)),
        ));
    }
}
