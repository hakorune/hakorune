//! PHI Insertion - Thin wrapper for builder context
//!
//! Purpose: Eliminate PHI insertion boilerplate across Pattern6/7/8
//!
//! Architecture:
//! - SSOT: insert_phi_at_head_spanned() in src/mir/ssot/cf_common.rs
//! - This module: Builder context extraction (current_function, span) + fail-fast
//!
//! Refactoring Context:
//! - Before: Pattern6/7/8 each have ~15 lines of boilerplate (if let Some(ref mut func) ...)
//! - After: Single function call with error propagation

use crate::mir::builder::MirBuilder;
use crate::mir::ssot::cf_common::insert_phi_at_head_spanned;
use crate::mir::{BasicBlockId, ValueId};

/// Insert PHI node at block header (builder wrapper)
///
/// # Arguments
/// - `builder`: MirBuilder (for current_function, span extraction)
/// - `block`: Target block for PHI insertion
/// - `phi_dst`: Destination ValueId for PHI result
/// - `phi_inputs`: Vec of (predecessor_block, value) pairs
///
/// # Errors
/// Returns Err if current_function is None (fail-fast)
///
/// # Example
/// ```ignore
/// // Pattern6/7/8 header PHI insertion
/// insert_loop_phi(
///     builder,
///     header_bb,
///     i_current,
///     vec![(preheader_bb, i_init_val), (step_bb, i_next)],
///     "pattern7",
/// )?;
/// ```
pub(in crate::mir::builder) fn insert_loop_phi(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    phi_dst: ValueId,
    phi_inputs: Vec<(BasicBlockId, ValueId)>,
    context: &str,
) -> Result<(), String> {
    let func = builder
        .scope_ctx
        .current_function
        .as_mut()
        .ok_or_else(|| format!("[{}] insert_loop_phi: No current function", context))?;

    let span = builder.metadata_ctx.current_span();

    insert_phi_at_head_spanned(func, block, phi_dst, phi_inputs, span)?;

    Ok(())
}

/// Materialize all variables from pre_if_var_map as single-pred PHIs at block entry.
///
/// # Purpose
/// Branch 分岐後の各ブロック入口で、pre_if_var_map の全変数を1入力PHIで定義。
/// variable_ctx.variable_map を更新し、後続コードが PHI 定義済み変数を参照可能にする。
///
/// # SSOT（入口 PHI 定型の責務）
/// - **この関数が入口 PHI 定型の SSOT**（直書き禁止）
/// - 適用先: `if_form.rs`（then/else/empty_else 入口）, `ops.rs`（short-circuit skip/eval_rhs）
/// - PHI 挿入: `MirBuilder::insert_phi_single` 経由
///
/// # Arguments
/// - `builder`: MirBuilder
/// - `pre_branch_bb`: 分岐元ブロック（PHI の predecessor）
/// - `pre_if_var_map`: 分岐前の変数→ValueId マップ
/// - `_context`: デバッグ用コンテキスト文字列（将来のトレース用）
pub(in crate::mir::builder) fn materialize_vars_single_pred_at_entry(
    builder: &mut MirBuilder,
    pre_branch_bb: BasicBlockId,
    pre_if_var_map: &std::collections::BTreeMap<String, ValueId>,
    _context: &str,
) -> Result<(), String> {
    builder.variable_ctx.variable_map = pre_if_var_map.clone();
    for (name, &pre_v) in pre_if_var_map.iter() {
        let phi_val = builder.insert_phi_single(pre_branch_bb, pre_v)?;
        builder.variable_ctx.variable_map.insert(name.clone(), phi_val);
    }
    Ok(())
}
