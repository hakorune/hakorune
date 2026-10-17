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
#[cfg(test)]
use crate::ast::ASTNode;
use crate::ast::BinaryOperator;
#[cfg(test)]
use crate::mir::builder::recursive_child_lowering::drive_raw_legacy_expression_v1;
use crate::mir::loop_api::LoopBuilderApi; // for current_block()
use crate::mir::MirType;
use std::collections::BTreeMap;

/// Physical observations from this sole CFG writer. This is not source
/// authority: a consumer must pair it with its exact original logical node.
#[derive(Debug, Clone)]
pub(in crate::mir) struct CompletedLogicalShortCircuitV1 {
    operator: BinaryOperator,
    lhs: ValueId,
    pinned_lhs: ValueId,
    rhs: ValueId,
    result: ValueId,
    lhs_condition: ValueId,
    rhs_condition: ValueId,
    lhs_branch: crate::mir::BasicBlockId,
    rhs_branch: crate::mir::BasicBlockId,
    rhs_entry: crate::mir::BasicBlockId,
    merge: crate::mir::BasicBlockId,
    paths: [(crate::mir::BasicBlockId, ValueId); 3],
    bindings: Vec<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>,
}
impl CompletedLogicalShortCircuitV1 {
    pub(in crate::mir) fn operator(&self) -> &BinaryOperator { &self.operator }
    pub(in crate::mir) fn pinned_lhs(&self) -> ValueId { self.pinned_lhs }
    pub(in crate::mir) fn operands(&self) -> (ValueId, ValueId) { (self.lhs, self.rhs) }
    pub(in crate::mir) fn value(&self) -> ValueId { self.result }
    pub(in crate::mir) fn conditions(&self) -> (ValueId, ValueId) { (self.lhs_condition, self.rhs_condition) }
    pub(in crate::mir) fn branches(&self) -> (crate::mir::BasicBlockId, crate::mir::BasicBlockId) { (self.lhs_branch, self.rhs_branch) }
    pub(in crate::mir) fn rhs_entry(&self) -> crate::mir::BasicBlockId { self.rhs_entry }
    pub(in crate::mir) fn merge(&self) -> crate::mir::BasicBlockId { self.merge }
    pub(in crate::mir) fn paths(&self) -> &[(crate::mir::BasicBlockId, ValueId); 3] { &self.paths }
    pub(in crate::mir) fn bindings(&self) -> &[(crate::mir::BasicBlockId, crate::mir::MirInstruction)] { &self.bindings }
}

/// Delimit only this writer's actual append. Child expression instructions
/// belong to their own source packets and are never swept into this record.
fn append_cursor(builder: &MirBuilder) -> Result<(crate::mir::BasicBlockId, usize), String> {
    let id = builder.current_block()?;
    let block = builder.function_state.current_function.as_ref().and_then(|function| function.blocks.get(&id))
        .ok_or_else(|| "[freeze:contract][logical-shortcircuit/missing-block]".to_owned())?;
    if block.terminator.is_some() { return Err("[freeze:contract][logical-shortcircuit/closed-append]".into()); }
    Ok((id, block.instructions.len()))
}
fn retain_append(
    builder: &MirBuilder, cursor: (crate::mir::BasicBlockId, usize),
    bindings: &mut Vec<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>,
) -> Result<(), String> {
    if builder.current_block()? != cursor.0 { return Err("[freeze:contract][logical-shortcircuit/append-block-drift]".into()); }
    let block = builder.function_state.current_function.as_ref().and_then(|function| function.blocks.get(&cursor.0))
        .ok_or_else(|| "[freeze:contract][logical-shortcircuit/missing-block]".to_owned())?;
    let suffix = block.instructions.get(cursor.1..)
        .ok_or_else(|| "[freeze:contract][logical-shortcircuit/append-truncated]".to_owned())?;
    bindings.extend(suffix.iter().chain(block.terminator.iter()).cloned().map(|row| (cursor.0, row)));
    Ok(())
}

/// Lower logical && / || with proper short-circuit semantics.
///
/// **Result**: Bool value (PHI of 3 paths)
/// **RHS evaluation**: Only if needed (short-circuit)
///
/// **Phase 29bq+ Option 2: 1-join 構造（3-predecessor merge）**
#[cfg(test)]
pub(in crate::mir::builder) fn build_logical_shortcircuit_pre_sc0_i0_reference_v1(
    builder: &mut MirBuilder,
    left: ASTNode,
    operator: BinaryOperator,
    right: ASTNode,
) -> Result<ValueId, String> {
    let lhs_val0 = drive_raw_legacy_expression_v1(builder, left)?;
    build_logical_shortcircuit_after_lhs_v1(builder, operator, lhs_val0, move |builder| {
        drive_raw_legacy_expression_v1(builder, right)
    })
}

/// Existing short-circuit CFG/PHI owner after the LHS has been lowered.
///
/// `lower_rhs` is invoked only after entering the eval-RHS block. Associated
/// input adapters may therefore defer both RHS location demand and lowering
/// without acquiring any CFG, PHI, result, or type authority.
pub(in crate::mir::builder) fn build_logical_shortcircuit_after_lhs_v1<LowerRhs>(
    builder: &mut MirBuilder,
    operator: BinaryOperator,
    lhs_val0: ValueId,
    lower_rhs: LowerRhs,
) -> Result<ValueId, String>
where
    LowerRhs: FnOnce(&mut MirBuilder) -> Result<ValueId, String>,
{
    build_logical_shortcircuit_after_lhs_recorded_v1(builder, operator, lhs_val0, lower_rhs)
        .map(|completed| completed.value())
}

pub(in crate::mir::builder) fn build_logical_shortcircuit_after_lhs_recorded_v1<LowerRhs>(
    builder: &mut MirBuilder, operator: BinaryOperator, lhs_val0: ValueId, lower_rhs: LowerRhs,
) -> Result<CompletedLogicalShortCircuitV1, String>
where LowerRhs: FnOnce(&mut MirBuilder) -> Result<ValueId, String>,
{
    if !matches!(operator, BinaryOperator::And | BinaryOperator::Or) {
        return Err("[freeze:contract][logical-shortcircuit/operator]".into());
    }
    let mut bindings = Vec::new();
    let lhs_cursor = append_cursor(builder)?;
    let is_and = matches!(operator, BinaryOperator::And);

    // Evaluate LHS only once and pin to a slot so it can be reused safely across blocks
    let lhs_val = builder.pin_to_slot(lhs_val0, "@sc_lhs")?;
    let lhs_pin_slot = builder.function_state.pin_slot_names.get(&lhs_val).cloned();

    // Prepare blocks: eval_rhs_block (evaluates RHS), skip_block (skips RHS), merge_block
    let eval_rhs_block = builder.next_block_id();
    let skip_block = builder.next_block_id();
    let rhs_true_block = builder.next_block_id();
    let rhs_false_block = builder.next_block_id();
    let merge_block = builder.next_block_id();

    // Branch on LHS truthiness
    // AND: true → eval RHS, false → skip (result=false)
    //  OR: true → skip (result=true), false → eval RHS
    let mut lhs_cond = super::super::ssa::local::try_cond(builder, lhs_val)?;
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
    retain_append(builder, lhs_cursor, &mut bindings)?;

    // The LHS pin is an expression-local branch condition, not an outer local.
    // Keep the SSA value for the already-emitted branch, but do not let the
    // synthetic slot participate in branch-entry materialization or PHI merge.
    if let Some(slot) = lhs_pin_slot.as_ref() {
        builder
            .function_state
            .variable_ctx
            .variable_map
            .remove(slot);
    }

    // Snapshot variables before entering branches
    let pre_if_var_map = builder.function_state.variable_ctx.variable_map.clone();

    // ---- SKIP branch (short-circuit path) ----
    builder.start_new_block(skip_block)?;
    let skip_cursor = append_cursor(builder)?;
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
    retain_append(builder, skip_cursor, &mut bindings)?;
    let skip_var_map = builder.function_state.variable_ctx.variable_map.clone();

    // ---- EVAL RHS branch ----
    builder.start_new_block(eval_rhs_block)?;
    let rhs_entry_cursor = append_cursor(builder)?;
    builder.hint_scope_enter(0);
    crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry(
        builder,
        pre_branch_bb,
        &pre_if_var_map,
        "shortcircuit/eval_rhs",
    )?;
    // Evaluate RHS and branch on its truthiness
    retain_append(builder, rhs_entry_cursor, &mut bindings)?;
    let rhs_val = lower_rhs(builder)?;
    let rhs_cursor = append_cursor(builder)?;
    let mut rhs_cond = super::super::ssa::local::try_cond(builder, rhs_val)?;
    crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut rhs_cond)?;
    crate::mir::builder::emission::branch::emit_conditional(
        builder,
        rhs_cond,
        rhs_true_block,
        rhs_false_block,
    )?;
    retain_append(builder, rhs_cursor, &mut bindings)?;
    let rhs_branch = builder.current_block()?;
    // Capture var_map after RHS evaluation (shared by rhs_true/rhs_false)
    let rhs_eval_var_map = builder.function_state.variable_ctx.variable_map.clone();

    // ---- RHS TRUE path → merge ----
    builder.start_new_block(rhs_true_block)?;
    let true_cursor = append_cursor(builder)?;
    builder.function_state.variable_ctx.variable_map = rhs_eval_var_map.clone();
    let rhs_true_value = crate::mir::builder::emission::constant::emit_bool(builder, true)?;
    builder.hint_scope_leave(0);
    crate::mir::builder::emission::branch::emit_jump(builder, merge_block)?;
    let rhs_true_exit = builder.current_block()?;
    retain_append(builder, true_cursor, &mut bindings)?;
    let rhs_true_var_map = builder.function_state.variable_ctx.variable_map.clone();

    // ---- RHS FALSE path → merge ----
    builder.start_new_block(rhs_false_block)?;
    let false_cursor = append_cursor(builder)?;
    builder.function_state.variable_ctx.variable_map = rhs_eval_var_map.clone();
    let rhs_false_value = crate::mir::builder::emission::constant::emit_bool(builder, false)?;
    builder.hint_scope_leave(0);
    crate::mir::builder::emission::branch::emit_jump(builder, merge_block)?;
    let rhs_false_exit = builder.current_block()?;
    retain_append(builder, false_cursor, &mut bindings)?;
    let rhs_false_var_map = builder.function_state.variable_ctx.variable_map.clone();

    // ---- MERGE (3-predecessor) ----
    builder.suppress_next_entry_pin_copy();
    builder.start_new_block(merge_block)?;
    let merge_cursor = append_cursor(builder)?;
    builder.push_if_merge(merge_block);

    // PHI construction may append edge Copies to these exact three source
    // exits. Retain only that writer's append window, including no child work.
    let predecessor_cursors = [skip_exit, rhs_true_exit, rhs_false_exit].map(|id| {
        let block = &builder.function_state.current_function.as_ref().expect("active function").blocks[&id];
        (id, block.instructions.len(), block.terminator.clone())
    });
    // Result PHI: 3 inputs (skip, rhs_true, rhs_false)
    let result_inputs: Vec<(crate::mir::BasicBlockId, ValueId)> = vec![
        (skip_exit, skip_value),
        (rhs_true_exit, rhs_true_value),
        (rhs_false_exit, rhs_false_value),
    ];
    if let Some(func) = builder.function_state.current_function.as_mut() {
        func.update_cfg();
    }
    let result_val = builder.insert_phi(result_inputs)?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result_val, MirType::Bool);

    // Variable merge using merge_modified_vars_multi (3-predecessor)
    let mut exits: BTreeMap<crate::mir::BasicBlockId, BTreeMap<String, ValueId>> = BTreeMap::new();
    exits.insert(skip_exit, skip_var_map);
    exits.insert(rhs_true_exit, rhs_true_var_map);
    exits.insert(rhs_false_exit, rhs_false_var_map);
    builder.merge_modified_vars_multi(exits, &pre_if_var_map, None)?;
    if let Some(slot) = lhs_pin_slot.as_ref() {
        builder
            .function_state
            .variable_ctx
            .variable_map
            .remove(slot);
    }

    builder.pop_if_merge();
    for (id, start, terminal) in predecessor_cursors {
        let block = &builder.function_state.current_function.as_ref()
            .ok_or("[freeze:contract][logical-shortcircuit/merge-function]")?.blocks[&id];
        if block.terminator != terminal {
            return Err("[freeze:contract][logical-shortcircuit/merge-exit-drift]".into());
        }
        let appended = block.instructions.get(start..)
            .ok_or("[freeze:contract][logical-shortcircuit/merge-exit-truncated]")?;
        let terminal_index = bindings.iter().position(|(block_id, row)| *block_id == id && Some(row) == terminal.as_ref())
            .ok_or("[freeze:contract][logical-shortcircuit/merge-exit-record]")?;
        bindings.splice(terminal_index..terminal_index, appended.iter().cloned().map(|row| (id, row)));
    }
    retain_append(builder, merge_cursor, &mut bindings)?;
    Ok(CompletedLogicalShortCircuitV1 {
        operator, lhs: lhs_val0, pinned_lhs: lhs_val, rhs: rhs_val, result: result_val,
        lhs_condition: lhs_cond, rhs_condition: rhs_cond,
        lhs_branch: pre_branch_bb, rhs_branch, rhs_entry: eval_rhs_block, merge: merge_block,
        paths: [(skip_exit, skip_value), (rhs_true_exit, rhs_true_value), (rhs_false_exit, rhs_false_value)],
        bindings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::builder::MirBuilder;

    fn bool_lit(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn shortcircuit_lhs_pin_does_not_escape_variable_map() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("shortcircuit_lhs_pin_lifetime".to_string());

        build_logical_shortcircuit_pre_sc0_i0_reference_v1(
            &mut builder,
            bool_lit(false),
            BinaryOperator::And,
            bool_lit(true),
        )
        .expect("short-circuit lower");

        assert!(
            !builder
                .function_state
                .variable_ctx
                .variable_map
                .keys()
                .any(|name| name.contains("@sc_lhs")),
            "short-circuit LHS pin must remain expression-local"
        );
    }
}
