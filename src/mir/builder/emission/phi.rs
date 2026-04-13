//! PHI helpers for branch-entry materialization.
//!
//! This module keeps branch-entry PHI wiring in one place so if/short-circuit
//! lowering paths do not duplicate variable-map synchronization logic.

use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};

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
    let sanitized_map = if let Some(func) = builder.scope_ctx.current_function.as_ref() {
        let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
        let dominators = crate::mir::verification::utils::compute_dominators(func);
        pre_if_var_map
            .iter()
            .filter_map(|(name, &pre_v)| {
                let dominates = def_blocks
                    .get(&pre_v)
                    .copied()
                    .map(|def_bb| dominators.dominates(def_bb, pre_branch_bb))
                    .unwrap_or(false);
                dominates.then_some((name.clone(), pre_v))
            })
            .collect::<std::collections::BTreeMap<_, _>>()
    } else {
        pre_if_var_map.clone()
    };

    builder.variable_ctx.variable_map = sanitized_map.clone();
    for (name, &pre_v) in sanitized_map.iter() {
        let phi_val = builder.insert_phi_single(pre_branch_bb, pre_v)?;
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), phi_val);
    }
    Ok(())
}
