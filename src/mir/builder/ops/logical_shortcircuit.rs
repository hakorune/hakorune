//! Logical short-circuit evaluation (&&, ||) operations.
//!
//! **Purpose**: Lower logical && and || operators to control-flow with proper short-circuit semantics.
//!
//! **Responsibilities**:
//! - Evaluate LHS first, branch on its truthiness
//! - Skip RHS evaluation if result is determined (short-circuit)
//! - Evaluate RHS conditionally and branch on its truthiness
//! - Merge 3 paths (skip, rhs_true, rhs_false) with PHI construction
//! - Variable map snapshotting and multi-predecessor merge
//! - Scope tracking (hint_scope_enter/leave)
//! - Slot pinning for operands to ensure safe reuse across blocks
//!
//! **Control Flow Structure** (Phase 29bq+ Option 2: 1-join 構造 / 3-predecessor merge):
//! ```text
//! entry
//!   ├─ LHS eval
//!   └─ branch on LHS
//!       ├─ skip_block (short-circuit: AND→false, OR→true)
//!       │   └─ jump → merge
//!       └─ eval_rhs_block (evaluate RHS)
//!           ├─ branch on RHS
//!           ├─ rhs_true_block → merge
//!           └─ rhs_false_block → merge
//! merge (3 predecessors: skip, rhs_true, rhs_false)
//!   └─ PHI for result + variable merge
//! ```
//!
//! **AND semantics**: LHS==true → eval RHS, LHS==false → skip (result=false)
//! **OR semantics**: LHS==true → skip (result=true), LHS==false → eval RHS

use super::super::{MirBuilder, ValueId};
use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::loop_api::LoopBuilderApi; // for current_block()
use crate::mir::MirType;
use std::collections::BTreeMap;

/// Lower logical && / || with proper short-circuit semantics.
///
/// **Result**: Bool value (PHI of 3 paths)
/// **RHS evaluation**: Only if needed (short-circuit)
///
/// **Phase 29bq+ Option 2: 1-join 構造（3-predecessor merge）**
pub(in crate::mir::builder) fn build_logical_shortcircuit(
    builder: &mut MirBuilder,
    left: ASTNode,
    operator: BinaryOperator,
    right: ASTNode,
) -> Result<ValueId, String> {
    let is_and = matches!(operator, BinaryOperator::And);

    // Evaluate LHS only once and pin to a slot so it can be reused safely across blocks
    let lhs_val0 = builder.build_expression(left)?;
    let lhs_val = builder.pin_to_slot(lhs_val0, "@sc_lhs")?;

    // Prepare blocks: eval_rhs_block (evaluates RHS), skip_block (skips RHS), merge_block
    let eval_rhs_block = builder.next_block_id();
    let skip_block = builder.next_block_id();
    let rhs_true_block = builder.next_block_id();
    let rhs_false_block = builder.next_block_id();
    let merge_block = builder.next_block_id();

    // Branch on LHS truthiness
    // AND: true → eval RHS, false → skip (result=false)
    //  OR: true → skip (result=true), false → eval RHS
    let mut lhs_cond = builder.local_cond(lhs_val);
    crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut lhs_cond)?;
    let (then_target, else_target) = if is_and {
        (eval_rhs_block, skip_block)
    } else {
        (skip_block, eval_rhs_block)
    };
    crate::mir::builder::emission::branch::emit_conditional(
        builder,
        lhs_cond,
        then_target,
        else_target,
    )?;
    let pre_branch_bb = builder.current_block()?;

    // Snapshot variables before entering branches
    let pre_if_var_map = builder.variable_ctx.variable_map.clone();

    // ---- SKIP branch (short-circuit path) ----
    builder.start_new_block(skip_block)?;
    builder.hint_scope_enter(0);
    crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry(
        builder,
        pre_branch_bb,
        &pre_if_var_map,
        "shortcircuit/skip",
    )?;
    // AND: skip → false, OR: skip → true
    let skip_value = crate::mir::builder::emission::constant::emit_bool(builder, !is_and)?;
    builder.hint_scope_leave(0);
    crate::mir::builder::emission::branch::emit_jump(builder, merge_block)?;
    let skip_exit = builder.current_block()?;
    let skip_var_map = builder.variable_ctx.variable_map.clone();

    // ---- EVAL RHS branch ----
    builder.start_new_block(eval_rhs_block)?;
    builder.hint_scope_enter(0);
    crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry(
        builder,
        pre_branch_bb,
        &pre_if_var_map,
        "shortcircuit/eval_rhs",
    )?;
    // Evaluate RHS and branch on its truthiness
    let rhs_val = builder.build_expression(right)?;
    let mut rhs_cond = builder.local_cond(rhs_val);
    crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut rhs_cond)?;
    crate::mir::builder::emission::branch::emit_conditional(
        builder,
        rhs_cond,
        rhs_true_block,
        rhs_false_block,
    )?;
    // Capture var_map after RHS evaluation (shared by rhs_true/rhs_false)
    let rhs_eval_var_map = builder.variable_ctx.variable_map.clone();

    // ---- RHS TRUE path → merge ----
    builder.start_new_block(rhs_true_block)?;
    builder.variable_ctx.variable_map = rhs_eval_var_map.clone();
    let rhs_true_value = crate::mir::builder::emission::constant::emit_bool(builder, true)?;
    builder.hint_scope_leave(0);
    crate::mir::builder::emission::branch::emit_jump(builder, merge_block)?;
    let rhs_true_exit = builder.current_block()?;
    let rhs_true_var_map = builder.variable_ctx.variable_map.clone();

    // ---- RHS FALSE path → merge ----
    builder.start_new_block(rhs_false_block)?;
    builder.variable_ctx.variable_map = rhs_eval_var_map.clone();
    let rhs_false_value = crate::mir::builder::emission::constant::emit_bool(builder, false)?;
    builder.hint_scope_leave(0);
    crate::mir::builder::emission::branch::emit_jump(builder, merge_block)?;
    let rhs_false_exit = builder.current_block()?;
    let rhs_false_var_map = builder.variable_ctx.variable_map.clone();

    // ---- MERGE (3-predecessor) ----
    builder.suppress_next_entry_pin_copy();
    builder.start_new_block(merge_block)?;
    builder.push_if_merge(merge_block);

    // Result PHI: 3 inputs (skip, rhs_true, rhs_false)
    let result_inputs: Vec<(crate::mir::BasicBlockId, ValueId)> = vec![
        (skip_exit, skip_value),
        (rhs_true_exit, rhs_true_value),
        (rhs_false_exit, rhs_false_value),
    ];
    if let Some(func) = builder.scope_ctx.current_function.as_mut() {
        func.update_cfg();
    }
    let result_val = builder.insert_phi(result_inputs)?;
    builder
        .type_ctx
        .value_types
        .insert(result_val, MirType::Bool);

    // Variable merge using merge_modified_vars_multi (3-predecessor)
    let mut exits: BTreeMap<crate::mir::BasicBlockId, BTreeMap<String, ValueId>> = BTreeMap::new();
    exits.insert(skip_exit, skip_var_map);
    exits.insert(rhs_true_exit, rhs_true_var_map);
    exits.insert(rhs_false_exit, rhs_false_var_map);
    builder.merge_modified_vars_multi(exits, &pre_if_var_map, None)?;

    builder.pop_if_merge();
    Ok(result_val)
}
