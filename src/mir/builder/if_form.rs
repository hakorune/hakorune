use super::{MirBuilder, ValueId};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::control_form::IfShape;
use crate::mir::loop_api::LoopBuilderApi; // for current_block()
use crate::mir::phi_core::phi_builder_box::PhiBuilderOps;
use crate::mir::BasicBlockId;

/// Phase 61-4-F: MirBuilder 用 PhiBuilderOps 実装
///
/// ループ外 if の JoinIR 経路で emit_toplevel_phis() を呼ぶためのラッパー。
struct ToplevelOps<'a>(&'a mut MirBuilder);

impl<'a> PhiBuilderOps for ToplevelOps<'a> {
    fn new_value(&mut self) -> ValueId {
        self.0.next_value_id()
    }

    fn emit_phi(
        &mut self,
        block: BasicBlockId,
        dst: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String> {
        // SSOT: PHI insertion via phi_lifecycle
        crate::mir::builder::emission::phi_lifecycle::define_phi_final(
            self.0,
            block,
            dst,
            inputs,
            "if_form:insert_merge_phi",
        )
    }

    fn update_var(&mut self, name: String, value: ValueId) {
        self.0.variable_ctx.variable_map.insert(name, value);
    }

    fn get_block_predecessors(&self, block: BasicBlockId) -> Vec<BasicBlockId> {
        if let Some(ref func) = self.0.scope_ctx.current_function {
            func.blocks
                .get(&block)
                .map(|bb| bb.predecessors.iter().copied().collect())
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    fn emit_void(&mut self) -> Result<ValueId, String> {
        crate::mir::builder::emission::constant::emit_void(self.0)
    }

    fn set_current_block(&mut self, block: BasicBlockId) -> Result<(), String> {
        // IMPORTANT: do not just assign `current_block`.
        // We must ensure the block exists and clear per-block caches (LocalSSA/scheduler) just like
        // the main lowering paths do, otherwise later emission can silently miss instructions.
        self.0.start_new_block(block)
    }

    fn block_exists(&self, block: BasicBlockId) -> bool {
        if let Some(ref func) = self.0.scope_ctx.current_function {
            func.blocks.contains_key(&block)
        } else {
            false
        }
    }
}

impl MirBuilder {
    /// Lower an if/else using a structured IfForm (header→then/else→merge).
    /// PHI-off: edge-copy only on predecessors; PHI-on: Phi at merge.
    pub(super) fn lower_if_form(
        &mut self,
        condition: ASTNode,
        then_branch: ASTNode,
        else_branch: Option<ASTNode>,
    ) -> Result<ValueId, String> {
        // Reserve a deterministic join id for debug region labeling
        let join_id = self.debug_next_join_id();
        // Pre-pin heuristic was deprecated; keep operands as-is for predictability.

        let cond_ast_for_debug = condition.clone();
        let condition_val = self.build_expression(condition)?;
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
            if let Some(func) = self.scope_ctx.current_function.as_ref() {
                let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
                if let Some(def_block) = def_blocks.get(&condition_val) {
                    if *def_block != pre_branch_bb {
                        let rhs_const = match &cond_ast_for_debug {
                            ASTNode::BinaryOp { right, .. } => match right.as_ref() {
                                ASTNode::Literal { value: LiteralValue::String(s), .. } => Some(s.as_str()),
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
        let pre_if_var_map = self.variable_ctx.variable_map.clone();

        let trace_if = crate::config::env::builder_if_trace();

        // then
        self.start_new_block(then_block)?;
        // Debug region: join then-branch
        self.debug_push_region(format!("join#{}", join_id) + "/then");
        // Scope enter for then-branch
        self.hint_scope_enter(0);
        let then_ast_for_analysis = then_branch.clone();
        // Materialize all variables at block entry via single-pred Phi (correctness-first)
        crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry(
            self, pre_branch_bb, &pre_if_var_map, "if_form/then"
        )?;
        if trace_if {
            for (name, &pre_v) in pre_if_var_map.iter() {
                if let Some(&phi_val) = self.variable_ctx.variable_map.get(name) {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[if-trace] then-entry phi var={} pre={:?} -> dst={:?}",
                        name, pre_v, phi_val
                    ));
                }
            }
        }
        let then_value_raw = self.build_expression(then_branch)?;
        let then_exit_block = self.current_block()?;
        let then_reaches_merge = !self.is_current_block_terminated();
        let then_var_map_end = self.variable_ctx.variable_map.clone();
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
        let (else_value_raw, else_ast_for_analysis, else_var_map_end_opt) =
            if let Some(else_ast) = else_branch {
                // Materialize all variables at block entry via single-pred Phi (correctness-first)
                crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry(
                    self, pre_branch_bb, &pre_if_var_map, "if_form/else"
                )?;
                if trace_if {
                    for (name, &pre_v) in pre_if_var_map.iter() {
                        if let Some(&phi_val) = self.variable_ctx.variable_map.get(name) {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug(&format!(
                                "[if-trace] else-entry phi var={} pre={:?} -> dst={:?}",
                                name, pre_v, phi_val
                            ));
                        }
                    }
                }
                let val = self.build_expression(else_ast.clone())?;
                (
                    val,
                    Some(else_ast),
                    Some(self.variable_ctx.variable_map.clone()),
                )
            } else {
                // No else branch: materialize PHI nodes for the empty else block
                crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry(
                    self, pre_branch_bb, &pre_if_var_map, "if_form/empty_else"
                )?;
                if trace_if {
                    for (name, &pre_v) in pre_if_var_map.iter() {
                        if let Some(&phi_val) = self.variable_ctx.variable_map.get(name) {
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
                (void_val, None, Some(self.variable_ctx.variable_map.clone()))
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

        // Phase 61-4: JoinIR 経路試行（ループ外 If）
        //
        // - 実際の有効化判定は config::env 側のポリシー関数に集約する。
        let joinir_enabled = crate::config::env::joinir_if_select_enabled();
        let joinir_toplevel = crate::config::env::joinir_if_toplevel_enabled();
        let joinir_dryrun = crate::config::env::joinir_if_toplevel_dryrun_enabled();
        let mut joinir_success = false;

        // 関数名ガードチェック
        let func_name = self
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.signature.name.as_str())
            .unwrap_or("");
        let is_target = crate::mir::join_ir::lowering::is_joinir_if_toplevel_target(func_name);

        // Phase 80: Core ON 時は代表関数で JoinIR を本線として試行
        let core_mainline =
            crate::mir::join_ir::lowering::should_try_joinir_mainline(func_name, false);
        let strict_mode =
            crate::mir::join_ir::lowering::should_panic_on_joinir_failure(func_name, false);

        // Core ON + 本線対象の場合は環境変数に関わらず試行
        let should_try_joinir =
            core_mainline || (joinir_enabled && is_target && (joinir_toplevel || joinir_dryrun));

        if should_try_joinir {
            if let Some(ref func) = self.scope_ctx.current_function {
                let context =
                    crate::mir::join_ir::lowering::if_phi_context::IfPhiContext::pure_if();

                match crate::mir::join_ir::lowering::try_lower_if_to_joinir(
                    func,
                    pre_branch_bb,
                    false,
                    Some(&context),
                ) {
                    Some(join_inst) => {
                        if joinir_dryrun || joinir_toplevel {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug(&format!(
                                "[Phase 61-4] ✅ Toplevel If lowered via JoinIR ({}): {:?}",
                                func_name, join_inst
                            ));
                        }

                        // PhiSpec 計算
                        let phi_spec = crate::mir::join_ir::lowering::if_phi_spec::compute_phi_spec_from_joinir(&context, &join_inst);

                        if joinir_dryrun {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug(&format!(
                                "[Phase 61-4] 🔍 dry-run: JoinIR PhiSpec header={}, exit={}",
                                phi_spec.header_count(),
                                phi_spec.exit_count()
                            ));
                        }

                        // Phase 61-4-F: 本番経路 - emit_toplevel_phis でPHI生成
                        if joinir_toplevel {
                            // IfShape 構築
                            let if_shape = IfShape {
                                cond_block: pre_branch_bb,
                                then_block,
                                else_block: Some(else_block),
                                merge_block,
                            };

                            // PHI 生成
                            let phi_count = {
                                let mut ops = ToplevelOps(self);
                                crate::mir::if_in_loop_phi::IfInLoopPhiEmitter::emit_toplevel_phis(
                                    &phi_spec,
                                    &pre_if_var_map,
                                    &then_var_map_end,
                                    else_var_map_end_opt.as_ref(),
                                    &mut ops,
                                    &if_shape,
                                )?
                            };

                            if joinir_dryrun {
                                let ring0 = crate::runtime::get_global_ring0();
                                ring0.log.debug(&format!(
                                    "[Phase 61-4] ✅ Production path: {} PHIs generated via JoinIR",
                                    phi_count
                                ));
                            }

                            joinir_success = true;
                        }
                    }
                    None => {
                        if joinir_dryrun {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug(&format!(
                                "[Phase 61-4] ⏭️ JoinIR shape not matched for {}, using fallback",
                                func_name
                            ));
                        }
                        // Phase 80/81: Strict mode では本線対象関数の失敗でパニック
                        if strict_mode {
                            panic!(
                                "[joinir/if] strict mode: shape not matched for {} (if_form.rs)",
                                func_name
                            );
                        }
                    }
                }
            }
        }

        // Phase 61-4: JoinIR成功時はスキップ、失敗時は既存経路
        let result_val = if joinir_success {
            // JoinIR 経路（未実装 - フォールバック）
            crate::mir::builder::emission::constant::emit_void(self)?
        } else {
            self.normalize_if_else_phi(
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
                &then_ast_for_analysis,
                &else_ast_for_analysis,
                &then_var_map_end,
                &else_var_map_end_opt,
                pre_then_var_value,
            )?
        };

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
        if !joinir_success {
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
        }

        self.pop_if_merge();
        // Pop merge debug region
        self.debug_pop_region();
        Ok(result_val)
    }
}
