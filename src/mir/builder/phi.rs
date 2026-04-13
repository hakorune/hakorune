use super::MirBuilder;
use crate::ast::ASTNode;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet}; // Phase 25.1: 決定性確保

// Phase 84-5: if_phi.rs deleted, type inference now handled by GenericTypeResolver/PhiTypeResolver

impl MirBuilder {
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
        // 📦 Phase 25.1q: Use PhiMergeHelper for unified PHI insertion
        // 📦 Phase 57: ConservativeMerge 冗長呼び出し削除
        //    - 以前: ここで ConservativeMerge::analyze を呼び、merge_all_vars 内でも呼んでいた（2回）
        //    - 現在: merge_all_vars が changed_vars を返すので、1回で済む
        use std::collections::HashSet;

        // Use PhiMergeHelper for unified variable merging
        let mut helper =
            super::phi_merge::PhiMergeHelper::new(self, then_exit_block_opt, else_exit_block_opt);
        let changed_set: HashSet<String> =
            helper.merge_all_vars(pre_if_snapshot, then_map_end, else_map_end_opt, skip_var)?;

        // Ensure pinned synthetic slots ("__pin$...") have a block-local definition at the merge,
        // even if their values did not change across branches. This avoids undefined uses when
        // subsequent blocks re-use pinned values without modifications.
        for (pin_name, pre_val) in pre_if_snapshot.iter() {
            if !pin_name.starts_with("__pin$") {
                continue;
            }
            if skip_var.map(|s| s == pin_name.as_str()).unwrap_or(false) {
                continue;
            }
            if changed_set.contains(pin_name) {
                continue;
            }
            let then_v = then_map_end
                .get(pin_name.as_str())
                .copied()
                .unwrap_or(*pre_val);
            let else_v = else_map_end_opt
                .as_ref()
                .and_then(|m| m.get(pin_name.as_str()).copied())
                .unwrap_or(*pre_val);
            let mut inputs: Vec<(super::BasicBlockId, super::ValueId)> = Vec::new();
            if let Some(tp) = then_exit_block_opt {
                inputs.push((tp, then_v));
            }
            if let Some(ep) = else_exit_block_opt {
                inputs.push((ep, else_v));
            }
            match inputs.len() {
                0 => {}
                1 => {
                    let (_pred, v) = inputs[0];
                    self.variable_ctx.variable_map.insert(pin_name.clone(), v);
                }
                _ => {
                    if let Some(func) = self.scope_ctx.current_function.as_mut() {
                        func.update_cfg();
                    }
                    // Debug verification (when function and current_block available)
                    if let (Some(func), Some(cur_bb)) =
                        (&self.scope_ctx.current_function, self.current_block)
                    {
                        crate::mir::phi_core::common::debug_verify_phi_inputs(
                            func, cur_bb, &inputs,
                        );
                    }
                    let merged = self.next_value_id();
                    // SSOT: PHI insertion via phi_lifecycle
                    if let Some(cur_bb) = self.current_block {
                        crate::mir::builder::emission::phi_lifecycle::define_phi_final(
                            self,
                            cur_bb,
                            merged,
                            inputs,
                            "phi:merge_modified_vars",
                        )?;
                    } else {
                        // Legacy path: no current block, use emit_instruction
                        self.emit_instruction(MirInstruction::Phi {
                            dst: merged,
                            inputs,
                            type_hint: None, // Phase 63-6: Legacy path, no type hint
                        })?;
                    }
                    self.variable_ctx
                        .variable_map
                        .insert(pin_name.clone(), merged);
                }
            }
        }
        Ok(())
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
        let (def_blocks, dominators) = if let Some(func) = self.scope_ctx.current_function.as_ref()
        {
            (
                Some(crate::mir::verification::utils::compute_def_blocks(func)),
                Some(crate::mir::verification::utils::compute_dominators(func)),
            )
        } else {
            (None, None)
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
                inputs.push((*exit_bb, val));
            }
            if missing_outer_value {
                // Branch-local values that are absent from the pre-merge snapshot must not
                // escape via a synthetic merge PHI. Outer scope has no stable binding for them.
                continue;
            }
            match inputs.len() {
                0 => {}
                1 => {
                    self.variable_ctx.variable_map.insert(var, inputs[0].1);
                }
                _ => {
                    if let Some(func) = self.scope_ctx.current_function.as_mut() {
                        func.update_cfg();
                    }
                    if let (Some(func), Some(cur_bb)) =
                        (&self.scope_ctx.current_function, self.current_block)
                    {
                        crate::mir::phi_core::common::debug_verify_phi_inputs(
                            func, cur_bb, &inputs,
                        );
                    }
                    let merged = self.insert_phi(inputs)?;
                    self.variable_ctx.variable_map.insert(var, merged);
                }
            }
        }
        Ok(())
    }

    /// Normalize Phi creation for if/else constructs.
    /// This handles variable reassignment patterns and ensures a single exit value.
    pub(super) fn normalize_if_else_phi(
        &mut self,
        _then_block: BasicBlockId,
        _else_block: BasicBlockId,
        then_exit_block_opt: Option<BasicBlockId>,
        else_exit_block_opt: Option<BasicBlockId>,
        then_value_raw: ValueId,
        else_value_raw: ValueId,
        pre_if_var_map: &BTreeMap<String, ValueId>, // Phase 25.1: BTreeMap化
        _then_ast_for_analysis: &ASTNode,
        _else_ast_for_analysis: &Option<ASTNode>,
        then_var_map_end: &BTreeMap<String, ValueId>, // Phase 25.1: BTreeMap化
        else_var_map_end_opt: &Option<BTreeMap<String, ValueId>>, // Phase 25.1: BTreeMap化
        pre_then_var_value: Option<ValueId>,
    ) -> Result<ValueId, String> {
        // If only the then-branch assigns a variable (e.g., `if c { x = ... }`) and the else
        // does not assign the same variable, bind that variable to a Phi of (then_value, pre_if_value).
        // Phase 38: Pre-analysis removed (JoinIR AST lowering handles assignment detection)
        let assigned_var_then: Option<String> = None;
        let assigned_var_else: Option<String> = None;
        let result_val = self.next_value_id();

        // フェーズM: no_phi_mode分岐削除（常にPHI命令を使用）

        if let Some(var_name) = assigned_var_then.clone() {
            let else_assigns_same = assigned_var_else
                .as_ref()
                .map(|s| s == &var_name)
                .unwrap_or(false);
            // Resolve branch-end values for the assigned variable
            let then_value_for_var = then_var_map_end
                .get(&var_name)
                .copied()
                .unwrap_or(then_value_raw);
            // Check if else branch actually modified the variable (even if not as last expression)
            let else_modified_var = else_var_map_end_opt
                .as_ref()
                .and_then(|m| m.get(&var_name).copied());
            let else_value_for_var = if else_assigns_same {
                else_var_map_end_opt
                    .as_ref()
                    .and_then(|m| m.get(&var_name).copied())
                    .unwrap_or(else_value_raw)
            } else if let Some(else_modified) = else_modified_var {
                // Else modifies the variable (even if not as the last expression)
                else_modified
            } else {
                // Else doesn't modify the variable: use pre-if value if available
                pre_then_var_value.unwrap_or(else_value_raw)
            };
            // Build inputs from reachable predecessors only
            let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
            if let Some(tp) = then_exit_block_opt {
                inputs.push((tp, then_value_for_var));
            }
            if let Some(ep) = else_exit_block_opt {
                inputs.push((ep, else_value_for_var));
            }
            match inputs.len() {
                0 => {}
                1 => {
                    // Direct bind (no PHI needed)
                    self.variable_ctx.variable_map = pre_if_var_map.clone();
                    self.variable_ctx.variable_map.insert(var_name, inputs[0].1);
                    return Ok(inputs[0].1);
                }
                _ => {
                    if let Some(func) = self.scope_ctx.current_function.as_mut() {
                        func.update_cfg();
                    }
                    if let (Some(func), Some(cur_bb)) =
                        (&self.scope_ctx.current_function, self.current_block)
                    {
                        crate::mir::phi_core::common::debug_verify_phi_inputs(
                            func, cur_bb, &inputs,
                        );
                    }
                    self.insert_phi_with_dst(result_val, inputs)?;
                }
            }
            self.variable_ctx.variable_map = pre_if_var_map.clone();
            self.variable_ctx.variable_map.insert(var_name, result_val);
        } else {
            // No variable assignment pattern detected – just emit Phi for expression result
            let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();
            if let Some(tp) = then_exit_block_opt {
                inputs.push((tp, then_value_raw));
            }
            if let Some(ep) = else_exit_block_opt {
                inputs.push((ep, else_value_raw));
            }
            match inputs.len() {
                0 => {
                    /* leave result_val as fresh, but unused; synthesize void */
                    let v = crate::mir::builder::emission::constant::emit_void(self)?;
                    return Ok(v);
                }
                1 => {
                    return Ok(inputs[0].1);
                }
                _ => {
                    if let Some(func) = self.scope_ctx.current_function.as_mut() {
                        func.update_cfg();
                    }
                    if let (Some(func), Some(cur_bb)) =
                        (&self.scope_ctx.current_function, self.current_block)
                    {
                        crate::mir::phi_core::common::debug_verify_phi_inputs(
                            func, cur_bb, &inputs,
                        );
                    }
                    self.insert_phi_with_dst(result_val, inputs)?;
                }
            }
            // Merge variable map conservatively to pre-if snapshot (no new bindings)
            self.variable_ctx.variable_map = pre_if_var_map.clone();
        }

        Ok(result_val)
    }
}
