use super::{MirBuilder, ValueId};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::loop_api::LoopBuilderApi; // for current_block()

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum IfBranchKindV1 {
    Then,
    Else,
}

impl MirBuilder {
    pub(super) fn lower_if_form_with_condition_value_and_branch_lowerer<LowerBranch>(
        &mut self,
        condition_val: ValueId,
        condition_debug: Option<ASTNode>,
        has_explicit_else: bool,
        mut lower_branch: LowerBranch,
    ) -> Result<ValueId, String>
    where
        LowerBranch: FnMut(&mut MirBuilder, IfBranchKindV1) -> Result<ValueId, String>,
    {
        // Reserve a deterministic join id for debug region labeling
        let join_id = self.debug_next_join_id();
        // Pre-pin heuristic was deprecated; keep operands as-is for predictability.

        let cond_ast_for_debug = condition_debug;
        let condition_val = self.local_cond(condition_val);

        // Create blocks
        let then_block = self.next_block_id();
        let else_block = self.next_block_id();
        let merge_block = self.next_block_id();

        // Branch
        let pre_branch_bb = self.current_block()?;
        if crate::config::env::stageb_dev_verify_enabled()
            && crate::config::env::joinir_dev::strict_enabled()
            && crate::config::env::joinir_dev::planner_required_enabled()
        {
            if let Some(func) = self.function_state.current_function.as_ref() {
                let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
                if let Some(def_block) = def_blocks.get(&condition_val) {
                    if *def_block != pre_branch_bb {
                        let rhs_const = match &cond_ast_for_debug {
                            Some(ASTNode::BinaryOp { right, .. }) => match right.as_ref() {
                                ASTNode::Literal {
                                    value: LiteralValue::String(s),
                                    ..
                                } => Some(s.as_str()),
                                _ => None,
                            },
                            _ => None,
                        };
                        let mut msg = format!(
                            "[freeze:contract][if_form:cond_def_block_mismatch] fn={} pre_branch={:?} def_block={:?} cond={:?}",
                            func.signature.name,
                            pre_branch_bb,
                            def_block,
                            condition_val
                        );
                        if let Some(rhs_const) = rhs_const {
                            msg.push_str(&format!(" rhs_const=\"{}\"", rhs_const));
                        }
                        return Err(msg);
                    }
                }
            }
        }
        let mut condition_val = condition_val;
        crate::mir::builder::ssa::local::finalize_branch_cond(self, &mut condition_val)?;
        // Phase 268 P0: emit_conditional() deleted (replaced by emit_conditional_edgecfg() at line 206)

        // Snapshot variables before entering branches
        let pre_if_var_map = self.function_state.variable_ctx.variable_map.clone();

        let trace_if = crate::config::env::builder_if_trace();

        // then
        self.start_new_block(then_block)?;
        // Debug region: join then-branch
        self.debug_push_region(format!("join#{}", join_id) + "/then");
        // Scope enter for then-branch
        self.hint_scope_enter(0);
        // Materialize all variables at block entry via single-pred Phi (correctness-first)
        crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry(
            self,
            pre_branch_bb,
            &pre_if_var_map,
            "if_form/then",
        )?;
        if trace_if {
            for (name, &pre_v) in pre_if_var_map.iter() {
                if let Some(&phi_val) = self.function_state.variable_ctx.variable_map.get(name) {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[if-trace] then-entry phi var={} pre={:?} -> dst={:?}",
                        name, pre_v, phi_val
                    ));
                }
            }
        }
        let then_value_raw = lower_branch(self, IfBranchKindV1::Then)?;
        let then_exit_block = self.current_block()?;
        let then_reaches_merge = !self.is_current_block_terminated();
        let then_var_map_end = self.function_state.variable_ctx.variable_map.clone();
        if then_reaches_merge {
            // Scope leave for then-branch
            self.hint_scope_leave(0);
            // Phase 268 P0: emit_jump() deleted (handled by emit_conditional_edgecfg())
        }
        // Pop then-branch debug region
        self.debug_pop_region();

        // else
        self.start_new_block(else_block)?;
        // Debug region: join else-branch
        self.debug_push_region(format!("join#{}", join_id) + "/else");
        // Scope enter for else-branch
        self.hint_scope_enter(0);
        let (else_value_raw, else_var_map_end_opt) = if has_explicit_else {
            // Materialize all variables at block entry via single-pred Phi (correctness-first)
            crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry(
                self,
                pre_branch_bb,
                &pre_if_var_map,
                "if_form/else",
            )?;
            if trace_if {
                for (name, &pre_v) in pre_if_var_map.iter() {
                    if let Some(&phi_val) = self.function_state.variable_ctx.variable_map.get(name)
                    {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[if-trace] else-entry phi var={} pre={:?} -> dst={:?}",
                            name, pre_v, phi_val
                        ));
                    }
                }
            }
            let val = lower_branch(self, IfBranchKindV1::Else)?;
            (
                val,
                Some(self.function_state.variable_ctx.variable_map.clone()),
            )
        } else {
            // No else branch: materialize PHI nodes for the empty else block
            crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry(
                self,
                pre_branch_bb,
                &pre_if_var_map,
                "if_form/empty_else",
            )?;
            if trace_if {
                for (name, &pre_v) in pre_if_var_map.iter() {
                    if let Some(&phi_val) = self.function_state.variable_ctx.variable_map.get(name)
                    {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[if-trace] else-entry phi var={} pre={:?} -> dst={:?}",
                            name, pre_v, phi_val
                        ));
                    }
                }
            }
            let void_val = crate::mir::builder::emission::constant::emit_void(self)?;
            // Phase 25.1c/k: Pass PHI-renamed variable_map for empty else branch
            // This ensures merge_modified_vars uses correct ValueIds after PHI renaming
            (
                void_val,
                Some(self.function_state.variable_ctx.variable_map.clone()),
            )
        };
        let else_exit_block = self.current_block()?;
        let else_reaches_merge = !self.is_current_block_terminated();
        if else_reaches_merge {
            // Scope leave for else-branch
            self.hint_scope_leave(0);
            // Phase 268 P0: emit_jump() deleted (handled by emit_conditional_edgecfg())
        }
        // Pop else-branch debug region
        self.debug_pop_region();

        // Phase 268 P0: EdgeCFG Fragment ベース emit（emission 層経由）
        crate::mir::builder::emission::branch::emit_conditional_edgecfg(
            self,
            pre_branch_bb,
            condition_val,
            then_block,
            then_exit_block,
            then_reaches_merge,
            else_block,
            else_exit_block,
            else_reaches_merge,
            merge_block,
        )?;

        // merge: primary result via helper, then delta-based variable merges
        // Ensure PHIs are first in the block by suppressing entry pin copies here
        self.suppress_next_entry_pin_copy();
        self.start_new_block(merge_block)?;
        // Debug region: join merge
        self.debug_push_region(format!("join#{}", join_id) + "/join");
        self.push_if_merge(merge_block);

        // Phase 38: Pre-analysis hints removed (JoinIR AST lowering handles assignment detection)
        let assigned_then_pre: Option<String> = None;
        let assigned_else_pre: Option<String> = None;
        let pre_then_var_value: Option<ValueId> = None;

        let result_val = self.normalize_if_else_phi(
            then_block,
            else_block,
            if then_reaches_merge {
                Some(then_exit_block)
            } else {
                None
            },
            if else_reaches_merge {
                Some(else_exit_block)
            } else {
                None
            },
            then_value_raw,
            else_value_raw,
            &pre_if_var_map,
            &then_var_map_end,
            &else_var_map_end_opt,
            pre_then_var_value,
        )?;

        // Hint: join result variable(s)
        // 1) Primary: if both branches assign to the same variable name, emit a hint for that name
        if let (Some(tn), Some(en)) = (assigned_then_pre.as_deref(), assigned_else_pre.as_deref()) {
            if tn == en {
                self.hint_join_result(tn);
            }
        }
        // 2) Secondary: if both branches assign multiple variables, hint全件（制限なし）
        if let Some(ref else_map_end) = else_var_map_end_opt {
            for name in then_var_map_end.keys() {
                if Some(name.as_str()) == assigned_then_pre.as_deref() {
                    continue;
                }
                if else_map_end.contains_key(name) {
                    self.hint_join_result(name.as_str());
                }
            }
        }

        // Merge other modified variables (skip the primary assignment if any)
        let skip_name = assigned_then_pre.as_deref();
        self.merge_modified_vars(
            then_block,
            else_block,
            if then_reaches_merge {
                Some(then_exit_block)
            } else {
                None
            },
            if else_reaches_merge {
                Some(else_exit_block)
            } else {
                None
            },
            &pre_if_var_map,
            &then_var_map_end,
            &else_var_map_end_opt,
            skip_name,
        )?;

        self.pop_if_merge();
        // Pop merge debug region
        self.debug_pop_region();
        Ok(result_val)
    }
}
