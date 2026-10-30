//! PHI Merge Helper - Unified PHI insertion logic
//!
//! Phase 25.1q: PhiMergeHelper統一化
//! - phi.rs の2箇所のPHI insertion重複（L120-137 vs L155-174）を統一
//! - Conservative戦略によるvariable merging の一元化
//!
//! Box-First理論: PHI insertion の境界を明確にし、差し替え可能な箱として提供

use super::{BasicBlockId, MirBuilder, ValueId};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Entry parameters or dominating values may cross a complete join without a new Phi.
/// This is physical correspondence only; it issues no type or source authority.
pub(super) fn reusable_join_values(
    function: &crate::mir::MirFunction,
    merge: Option<BasicBlockId>,
    exits: &[BasicBlockId],
    defs: &HashMap<ValueId, BasicBlockId>,
    dominance: &crate::mir::verification::utils::DominatorTree,
) -> BTreeSet<ValueId> {
    let Some(merge) = merge else { return BTreeSet::new() };
    let expected: BTreeSet<_> = exits.iter().copied().collect();
    let actual: Vec<_> = function.blocks.iter().flat_map(|(pred, body)|
        body.out_edges().into_iter().filter_map(move |edge|
            (edge.target == merge).then_some(*pred))).collect();
    if exits.len() < 2 || expected.len() != exits.len() || actual.len() != exits.len()
        || actual.into_iter().collect::<BTreeSet<_>>() != expected
    { return BTreeSet::new(); }
    let mut counts: HashMap<ValueId, usize> = HashMap::new();
    for param in &function.params { *counts.entry(*param).or_default() += 1; }
    for body in function.blocks.values() {
        for row in body.all_instructions() {
            if let Some(dst) = row.dst_value() { *counts.entry(dst).or_default() += 1; }
        }
    }
    // A unique parameter is available on every execution of this function,
    // including a nested If whose enclosing entry edge is still being built.
    let reachable = dominance.is_reachable(merge)
        && expected.iter().all(|pred| dominance.is_reachable(*pred));
    defs.iter().filter_map(|(value, block)|
        (counts.get(value) == Some(&1) && *block != merge
            && (function.params.contains(value) || (reachable && dominance.is_reachable(*block)
                && dominance.dominates(*block, merge)))).then_some(*value)).collect()
}

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
                self.builder
                    .function_state
                    .variable_ctx
                    .variable_map
                    .insert(name, v);
                Ok(())
            }
            _ => {
                // Multiple predecessors - insert PHI
                if let Some(func) = self.builder.function_state.current_function.as_mut() {
                    func.update_cfg();
                }
                if let (Some(func), Some(cur_bb)) = (
                    &self.builder.function_state.current_function,
                    self.builder.function_state.current_block,
                ) {
                    crate::mir::phi_core::common::debug_verify_phi_inputs(func, cur_bb, &inputs);
                }
                let merged = self.builder.insert_phi(inputs)?;
                self.builder
                    .function_state
                    .variable_ctx
                    .variable_map
                    .insert(name, merged);
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
    /// Pins use this same merge owner; no second merge or change-set receipt
    /// is needed after all branch variable maps have been joined.
    ///
    /// The same reaching value can be retained before PHI allocation only
    /// for a complete reachable join with one dominating definition. Other
    /// inputs keep the existing PHI completion and validation path.
    pub fn merge_all_vars(
        &mut self,
        pre_if_snapshot: &BTreeMap<String, ValueId>, // Phase 25.1: BTreeMap化
        then_map_end: &BTreeMap<String, ValueId>,    // Phase 25.1: BTreeMap化
        else_map_end_opt: &Option<BTreeMap<String, ValueId>>, // Phase 25.1: BTreeMap化
        skip_var: Option<&str>,
    ) -> Result<(), String> {
        let merge = self.builder.function_state.current_block;
        let (def_blocks, dominators, reusable_values) =
            if let Some(func) = self.builder.function_state.current_function.as_mut() {
                func.update_cfg();
                let defs = crate::mir::verification::utils::compute_def_blocks(func);
                let dominance = crate::mir::verification::utils::compute_dominators(func);
                let exits: Vec<_> = self.then_exit.into_iter().chain(self.else_exit).collect();
                let reusable = reusable_join_values(func, merge, &exits, &defs, &dominance);
                (Some(defs), Some(dominance), reusable)
            } else {
                (None, None, BTreeSet::new())
            };

        // ========================================
        // ========================================
        // 新: 以下のロジックを直接ここに記述

        // 1. all_vars: 全ブランチに存在する変数のユニオン（Conservative戦略）
        let mut all_vars = BTreeSet::new();
        all_vars.extend(pre_if_snapshot.keys().cloned());
        all_vars.extend(then_map_end.keys().cloned());
        if let Some(ref else_map) = else_map_end_opt {
            all_vars.extend(else_map.keys().cloned());
        }

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

            // Preserve the original branch reads, not values repaired by the
            // sanitizer. No Phi token, type fact or source receipt is issued.
            if then_v == else_v && raw_then_v_opt == Some(then_v)
                && raw_else_v_opt == Some(else_v) && reusable_values.contains(&then_v)
            {
                self.builder.function_state.variable_ctx.variable_map.insert(name.clone(), then_v);
                continue;
            }
            self.merge_variable(name.clone(), then_v, else_v)?;
        }
        Ok(())
    }
}
