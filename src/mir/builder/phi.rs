use super::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::{BTreeMap, BTreeSet}; // Phase 25.1: 決定性確保

// Phase 84-5: if_phi.rs deleted; finalization owns return-type strategy.

impl MirBuilder {
    pub(crate) fn define_current_block_phi_final(
        &mut self,
        dst: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
        tag: &str,
    ) -> Result<(), String> {
        let Some(cur_bb) = self.function_state.current_block else {
            return Err(format!(
                "[freeze:contract][phi_lifecycle/no_current_block] tag={tag}"
            ));
        };
        crate::mir::builder::emission::phi_lifecycle::define_phi_final(
            self, cur_bb, dst, inputs, tag,
        )
    }

    pub(crate) fn define_current_block_phi_final_with_type_hint(
        &mut self,
        dst: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
        type_hint: Option<crate::mir::MirType>,
        tag: &str,
    ) -> Result<(), String> {
        let Some(cur_bb) = self.function_state.current_block else {
            return Err(format!(
                "[freeze:contract][phi_lifecycle/no_current_block] tag={tag}"
            ));
        };
        crate::mir::builder::emission::phi_lifecycle::define_phi_final_with_type_hint(
            self, cur_bb, dst, inputs, type_hint, tag,
        )
    }

    /// Merge all variables modified in then/else relative to pre_if_snapshot.
    /// In PHI-off mode inserts edge copies from branch exits to merge. In PHI-on mode emits Phi.
    /// `skip_var` allows skipping a variable already merged elsewhere (e.g., bound to an expression result).
    pub(super) fn merge_modified_vars(
        &mut self,
        _then_block: super::BasicBlockId,
        _else_block: super::BasicBlockId,
        then_exit_block_opt: Option<super::BasicBlockId>,
        else_exit_block_opt: Option<super::BasicBlockId>,
        pre_if_snapshot: &BTreeMap<String, super::ValueId>, // Phase 25.1: BTreeMap化
        then_map_end: &BTreeMap<String, super::ValueId>,    // Phase 25.1: BTreeMap化
        else_map_end_opt: &Option<BTreeMap<String, super::ValueId>>, // Phase 25.1: BTreeMap化
        skip_var: Option<&str>,
    ) -> Result<(), String> {
        let mut helper =
            super::phi_merge::PhiMergeHelper::new(self, then_exit_block_opt, else_exit_block_opt);
        helper.merge_all_vars(pre_if_snapshot, then_map_end, else_map_end_opt, skip_var)
    }

    /// Merge variables from N predecessors into current merge block.
    /// Phase 29bq+ Option 2: short-circuit 用の N-predecessor 変数マージ
    ///
    /// # SSOT
    /// - 用途: short-circuit 専用（build_logical_shortcircuit から呼び出し）
    /// - 設計: `docs/development/current/main/design/short-circuit-joins-ssot.md`
    /// - PHI 挿入: `src/mir/utils/phi_helpers.rs`（insert_phi）経由
    ///
    /// exits: BTreeMap<BasicBlockId, BTreeMap<String, ValueId>> - 決定的順序を保証
    pub(super) fn merge_modified_vars_multi(
        &mut self,
        exits: BTreeMap<BasicBlockId, BTreeMap<String, ValueId>>,
        pre_if_snapshot: &BTreeMap<String, ValueId>,
        skip_var: Option<&str>,
    ) -> Result<(), String> {
        let (def_blocks, dominators, reusable_values) =
            if let Some(func) = self.function_state.current_function.as_mut() {
                func.update_cfg();
                let defs = crate::mir::verification::utils::compute_def_blocks(func);
                let dominance = crate::mir::verification::utils::compute_dominators(func);
                let predecessors: Vec<_> = exits.keys().copied().collect();
                let reusable = super::phi_merge::reusable_join_values(
                    func, self.function_state.current_block, &predecessors, &defs, &dominance);
                (Some(defs), Some(dominance), reusable)
            } else {
                (None, None, BTreeSet::new())
            };

        // 全変数を収集（決定的順序）
        let all_vars: BTreeSet<String> = exits
            .values()
            .flat_map(|m| m.keys().cloned())
            .chain(pre_if_snapshot.keys().cloned())
            .collect();

        for var in all_vars {
            if skip_var.map(|s| s == var.as_str()).unwrap_or(false) {
                continue;
            }
            // 各 exit から1本ずつ inputs を構築（pred数を減らさない）
            let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
            let pre_val = pre_if_snapshot.get(&var).copied();
            let mut missing_outer_value = false;
            let mut original_inputs = true;
            for (exit_bb, var_map) in &exits {
                let candidate = var_map.get(&var).copied().or(pre_val);
                let val = match (candidate, def_blocks.as_ref(), dominators.as_ref()) {
                    (Some(candidate), Some(def_blocks), Some(dominators)) => {
                        let candidate_ok = def_blocks
                            .get(&candidate)
                            .copied()
                            .map(|def_bb| dominators.dominates(def_bb, *exit_bb))
                            .unwrap_or(false);
                        if candidate_ok {
                            candidate
                        } else if let Some(pre_v) = pre_val {
                            let pre_ok = def_blocks
                                .get(&pre_v)
                                .copied()
                                .map(|def_bb| dominators.dominates(def_bb, *exit_bb))
                                .unwrap_or(false);
                            if pre_ok {
                                pre_v
                            } else {
                                missing_outer_value = true;
                                break;
                            }
                        } else {
                            missing_outer_value = true;
                            break;
                        }
                    }
                    (Some(candidate), _, _) => candidate,
                    (None, _, _) => {
                        missing_outer_value = true;
                        break;
                    }
                };
                original_inputs &= candidate == Some(val);
                inputs.push((*exit_bb, val));
            }
            if missing_outer_value {
                // Branch-local values that are absent from the pre-merge snapshot must not
                // escape via a synthetic merge PHI. Outer scope has no stable binding for them.
                continue;
            }
            if original_inputs && inputs.first().is_some_and(|(_, value)|
                reusable_values.contains(value) && inputs.iter().all(|(_, other)| other == value))
            {
                self.function_state.variable_ctx.variable_map.insert(var, inputs[0].1);
                continue;
            }
            match inputs.len() {
                0 => {}
                1 => {
                    self.function_state
                        .variable_ctx
                        .variable_map
                        .insert(var, inputs[0].1);
                }
                _ => {
                    if let Some(func) = self.function_state.current_function.as_mut() {
                        func.update_cfg();
                    }
                    if let (Some(func), Some(cur_bb)) = (
                        &self.function_state.current_function,
                        self.function_state.current_block,
                    ) {
                        crate::mir::phi_core::common::debug_verify_phi_inputs(
                            func, cur_bb, &inputs,
                        );
                    }
                    let merged = self.insert_phi(inputs)?;
                    self.function_state
                        .variable_ctx
                        .variable_map
                        .insert(var, merged);
                }
            }
        }
        Ok(())
    }

    /// Publish the value of an if expression from its normal exits.
    pub(super) fn merge_if_result_v1(
        &mut self,
        arms: [(Option<BasicBlockId>, ValueId); 2],
    ) -> Result<ValueId, String> {
        let result = self.next_value_id();
        let inputs: Vec<_> = arms.into_iter()
            .filter_map(|(exit, value)| exit.map(|block| (block, value)))
            .collect();
        match inputs.as_slice() {
            [] => crate::mir::builder::emission::constant::emit_void(self),
            [(_, value)] => Ok(*value),
            _ => {
                if let Some(function) = self.function_state.current_function.as_mut() {
                    function.update_cfg();
                }
                if let (Some(function), Some(block)) = (
                    &self.function_state.current_function, self.function_state.current_block,
                ) {
                    crate::mir::phi_core::common::debug_verify_phi_inputs(function, block, &inputs);
                }
                self.insert_phi_with_dst(result, inputs)?;
                Ok(result)
            }
        }
    }
}
