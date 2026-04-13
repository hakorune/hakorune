//! PHI Merge Helper - Unified PHI insertion logic
//!
//! Phase 25.1q: PhiMergeHelper統一化
//! - phi.rs の2箇所のPHI insertion重複（L120-137 vs L155-174）を統一
//! - Conservative戦略によるvariable merging の一元化
//!
//! Box-First理論: PHI insertion の境界を明確にし、差し替え可能な箱として提供

use super::{BasicBlockId, MirBuilder, ValueId};
use std::collections::{BTreeMap, BTreeSet, HashSet}; // Phase 25.1: 決定性確保, Phase 58: インライン化

/// PHI Merge Helper - 統一PHI挿入ロジック（Conservative戦略）
///
/// # Purpose
/// - 複数のブランチ出口からマージブロックへのPHI挿入を統一処理
/// - Conservative戦略: 全変数に対してPHIを生成（correctness-first）
///
/// # Usage
/// ```ignore
/// let mut helper = PhiMergeHelper::new(&mut builder, then_exit, else_exit);
/// helper.merge_variable("x".to_string(), then_v, else_v)?;
/// ```
pub struct PhiMergeHelper<'a> {
    builder: &'a mut MirBuilder,
    then_exit: Option<BasicBlockId>,
    else_exit: Option<BasicBlockId>,
}

impl<'a> PhiMergeHelper<'a> {
    /// Create a new PhiMergeHelper
    ///
    /// # Arguments
    /// * `builder` - MirBuilder instance
    /// * `then_exit` - Then-branch exit block (None if unreachable)
    /// * `else_exit` - Else-branch exit block (None if unreachable)
    pub fn new(
        builder: &'a mut MirBuilder,
        then_exit: Option<BasicBlockId>,
        else_exit: Option<BasicBlockId>,
    ) -> Self {
        Self {
            builder,
            then_exit,
            else_exit,
        }
    }

    /// Merge a single variable using Conservative PHI strategy
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `then_v` - Value from then-branch
    /// * `else_v` - Value from else-branch
    ///
    /// # Returns
    /// Ok(()) on success, Err(String) on failure
    ///
    /// # Conservative Strategy
    /// - 0 predecessors: skip (unreachable)
    /// - 1 predecessor: direct insert (no PHI needed)
    /// - 2+ predecessors: insert PHI node
    pub fn merge_variable(
        &mut self,
        name: String,
        then_v: ValueId,
        else_v: ValueId,
    ) -> Result<(), String> {
        let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
        if let Some(tp) = self.then_exit {
            inputs.push((tp, then_v));
        }
        if let Some(ep) = self.else_exit {
            inputs.push((ep, else_v));
        }

        match inputs.len() {
            0 => {
                // Both branches unreachable - skip
                Ok(())
            }
            1 => {
                // Single predecessor - direct insert (no PHI)
                let (_pred, v) = inputs[0];
                self.builder.variable_ctx.variable_map.insert(name, v);
                Ok(())
            }
            _ => {
                // Multiple predecessors - insert PHI
                if let Some(func) = self.builder.scope_ctx.current_function.as_mut() {
                    func.update_cfg();
                }
                if let (Some(func), Some(cur_bb)) = (
                    &self.builder.scope_ctx.current_function,
                    self.builder.current_block,
                ) {
                    crate::mir::phi_core::common::debug_verify_phi_inputs(func, cur_bb, &inputs);
                }
                let merged = self.builder.insert_phi(inputs)?;
                self.builder.variable_ctx.variable_map.insert(name, merged);
                Ok(())
            }
        }
    }

    /// Merge a variable with explicit destination ValueId (for primary result)
    ///
    /// # Arguments
    /// * `dst` - Destination ValueId for PHI result
    /// * `then_v` - Value from then-branch
    /// * `else_v` - Value from else-branch
    ///
    /// # Returns
    /// Ok(()) on success, Err(String) on failure
    #[allow(dead_code)] // Reserved: explicit dst PHI merge for future use
    pub fn merge_with_dst(
        &mut self,
        dst: ValueId,
        then_v: ValueId,
        else_v: ValueId,
    ) -> Result<(), String> {
        let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
        if let Some(tp) = self.then_exit {
            inputs.push((tp, then_v));
        }
        if let Some(ep) = self.else_exit {
            inputs.push((ep, else_v));
        }

        match inputs.len() {
            0 | 1 => {
                // Should not happen for explicit dst merge
                Ok(())
            }
            _ => {
                // Insert PHI with explicit dst
                if let Some(func) = self.builder.scope_ctx.current_function.as_mut() {
                    func.update_cfg();
                }
                if let (Some(func), Some(cur_bb)) = (
                    &self.builder.scope_ctx.current_function,
                    self.builder.current_block,
                ) {
                    crate::mir::phi_core::common::debug_verify_phi_inputs(func, cur_bb, &inputs);
                }
                self.builder.insert_phi_with_dst(dst, inputs)?;
                Ok(())
            }
        }
    }

    /// Merge all variables from both branches (Conservative strategy)
    ///
    /// # Arguments
    /// * `pre_if_snapshot` - Variable map before if statement
    /// * `then_map_end` - Variable map at end of then-branch
    /// * `else_map_end_opt` - Variable map at end of else-branch (None for empty else)
    /// * `skip_var` - Optional variable name to skip (already merged elsewhere)
    ///
    /// # Returns
    /// Ok(changed_vars) - Set of variables that were changed, for pin handling
    ///
    /// # Phase 57-58 改善
    ///
    /// - Phase 57: 戻り値を `()` から `HashSet<String>` に変更
    /// - Phase 58: ConservativeMerge::analyze をインライン化
    ///
    /// # Phase 58 改善
    ///
    /// ConservativeMerge::analyze をインライン化。
    /// conservative.rs の struct は削除され、ロジックのみここに残る。
    ///
    /// ## Conservative ∘ Elimination = Minimal SSA
    ///
    /// - Conservative (this): correctness-first, generate all PHIs
    /// - Elimination (future): efficiency optimization, remove unused PHIs
    pub fn merge_all_vars(
        &mut self,
        pre_if_snapshot: &BTreeMap<String, ValueId>, // Phase 25.1: BTreeMap化
        then_map_end: &BTreeMap<String, ValueId>,    // Phase 25.1: BTreeMap化
        else_map_end_opt: &Option<BTreeMap<String, ValueId>>, // Phase 25.1: BTreeMap化
        skip_var: Option<&str>,
    ) -> Result<HashSet<String>, String> {
        let (def_blocks, dominators) =
            if let Some(func) = self.builder.scope_ctx.current_function.as_ref() {
                (
                    Some(crate::mir::verification::utils::compute_def_blocks(func)),
                    Some(crate::mir::verification::utils::compute_dominators(func)),
                )
            } else {
                (None, None)
            };

        // ========================================
        // Phase 58: ConservativeMerge::analyze インライン化
        // ========================================
        // 旧: crate::mir::phi_core::conservative::ConservativeMerge::analyze(...)
        // 新: 以下のロジックを直接ここに記述

        // 1. all_vars: 全ブランチに存在する変数のユニオン（Conservative戦略）
        let mut all_vars = HashSet::new();
        all_vars.extend(pre_if_snapshot.keys().cloned());
        all_vars.extend(then_map_end.keys().cloned());
        if let Some(ref else_map) = else_map_end_opt {
            all_vars.extend(else_map.keys().cloned());
        }

        // 2. changed_vars: 実際に変更された変数のセット
        //    決定的順序のためBTreeSet使用
        let mut names: BTreeSet<&str> = BTreeSet::new();
        for k in then_map_end.keys() {
            names.insert(k.as_str());
        }
        if let Some(emap) = else_map_end_opt.as_ref() {
            for k in emap.keys() {
                names.insert(k.as_str());
            }
        }
        let mut changed_vars = HashSet::new();
        // アルファベット順で決定的にイテレート
        for &name in &names {
            let pre = pre_if_snapshot.get(name);
            let t = then_map_end.get(name);
            let e = else_map_end_opt.as_ref().and_then(|m| m.get(name));
            if (t.is_some() && Some(*t.unwrap()) != pre.copied())
                || (e.is_some() && Some(*e.unwrap()) != pre.copied())
            {
                changed_vars.insert(name.to_string());
            }
        }
        // ========================================

        let trace_conservative = crate::config::env::builder_conservative_phi_trace();

        for name in &all_vars {
            if skip_var.map(|s| s == name.as_str()).unwrap_or(false) {
                if trace_conservative {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[Conservative PHI] Skipping {}: matches skip_var",
                        name
                    ));
                }
                continue;
            }

            let pre_val_opt = pre_if_snapshot.get(name.as_str()).copied();
            let raw_then_v_opt = then_map_end.get(name.as_str()).copied().or(pre_val_opt);
            let raw_else_v_opt = else_map_end_opt
                .as_ref()
                .and_then(|m| m.get(name.as_str()).copied())
                .or(pre_val_opt);

            let sanitize_for_pred =
                |candidate: Option<ValueId>, pred: Option<BasicBlockId>| -> Option<ValueId> {
                    let (Some(candidate), Some(pred), Some(def_blocks), Some(dominators)) =
                        (candidate, pred, def_blocks.as_ref(), dominators.as_ref())
                    else {
                        return candidate;
                    };

                    let candidate_ok = def_blocks
                        .get(&candidate)
                        .copied()
                        .map(|def_bb| dominators.dominates(def_bb, pred))
                        .unwrap_or(false);
                    if candidate_ok {
                        return Some(candidate);
                    }

                    let Some(pre_val) = pre_val_opt else {
                        return None;
                    };
                    let pre_ok = def_blocks
                        .get(&pre_val)
                        .copied()
                        .map(|def_bb| dominators.dominates(def_bb, pred))
                        .unwrap_or(false);
                    if pre_ok {
                        Some(pre_val)
                    } else {
                        None
                    }
                };

            let then_v_opt = sanitize_for_pred(raw_then_v_opt, self.then_exit);
            let else_v_opt = sanitize_for_pred(raw_else_v_opt, self.else_exit);

            let (then_v, else_v) = match (then_v_opt, else_v_opt) {
                (Some(tv), Some(ev)) => {
                    if trace_conservative {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[Conservative PHI] Generating PHI for {}: then={:?} else={:?}",
                            name, tv, ev
                        ));
                    }
                    (tv, ev)
                }
                (Some(_), None) | (None, Some(_)) => {
                    if trace_conservative {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[Conservative PHI] Skipping {}: branch-local without dominating outer value",
                            name
                        ));
                    }
                    continue;
                }
                (None, None) => {
                    if trace_conservative {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[Conservative PHI] Skipping {}: undefined everywhere",
                            name
                        ));
                    }
                    continue;
                }
            };

            self.merge_variable(name.clone(), then_v, else_v)?;
        }

        // Phase 57: 変更された変数セットを返す（phi.rsでの冗長呼び出し削除用）
        Ok(changed_vars)
    }
}
