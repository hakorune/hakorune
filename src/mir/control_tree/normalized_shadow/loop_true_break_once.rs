//! Phase 131/132-P4: loop(true) break-once Normalized lowering
//!
//! ## Responsibility
//!
//! - Lower `loop(true) { <assign>* ; break }` to Normalized JoinModule
//! - One-time execution loop (condition is always true, breaks immediately)
//! - PHI-free: all state passing via env arguments + continuations
//! - Phase 132-P4: Support minimal post-loop computation (one assign + return)
//!
//! ## Contract
//!
//! - Input: StepTree with loop(true) { body ; break } [; <post>] pattern
//! - Output: JoinModule with:
//!   - main(env) → TailCall(loop_step, env)
//!   - loop_step(env) → TailCall(loop_body, env)  // condition is always true
//!   - loop_body(env) → <assign statements update env> → TailCall(k_exit, env)
//!   - k_exit(env) → Ret(env[x]) or TailCall(post_k, env)  // Phase 132-P4
//!   - post_k(env) → <post assign> → Ret(env[x])  // Phase 132-P4
//!
//! ## Scope
//!
//! - Condition: Bool literal `true` only
//! - Body: Assign(int literal/var/add) + LocalDecl only (Phase 130 baseline)
//! - Exit: Break at end of body only (no continue, no return in body)
//! - Post-loop (Phase 131): Simple return only
//! - Post-loop (Phase 132-P4): One assignment + return (reuse Phase 130's lower_assign_stmt)
//! - Post-loop (Phase 133-P0): Multiple assignments + return (extend Phase 132-P4)
//!
//! ## Return Value Lowering SSOT (Phase 138+)
//!
//! - **SSOT Location**: `common/return_value_lowerer_box.rs`
//! - Function: `ReturnValueLowererBox::lower_to_value_id()`
//! - Responsibility: Lower return values (variable, literal, expr) to ValueId
//! - Supported patterns:
//!   - Variable: env lookup
//!   - Integer literal: Const generation
//!   - Add expr (Phase 137): x + 2 → BinOp(Add, env[x], Const(2))
//! - Fallback: Out-of-scope patterns return `Ok(None)` for legacy routing
//!
//! ### Usage
//!
//! - Phase 138 P0: loop_true_break_once.rs
//! - Phase 139 P0: post_if_post_k.rs (planned)
//!
//! ## Fail-Fast
//!
//! - Out of scope → Ok(None) (fallback to legacy)
//! - In scope but conversion failed → Err (with freeze_with_hint in strict mode)

use super::common::normalized_helpers::NormalizedHelperBox;
use super::common::return_value_lowerer_box::ReturnValueLowererBox;
use super::env_layout::EnvLayout;
use super::legacy::LegacyLowerer;
use super::loop_true_break_once_helpers as helpers;
use crate::mir::control_tree::step_tree::{StepNode, StepStmtKind, StepTree};
use crate::mir::join_ir::lowering::carrier_info::JoinFragmentMeta;
use crate::mir::join_ir::lowering::error_tags;
use crate::mir::join_ir::{JoinFuncId, JoinFunction, JoinInst, JoinModule};
use crate::mir::ValueId;

#[cfg(test)]
use crate::mir::join_ir::{ConstValue, MirLikeInst};
#[cfg(test)]
use crate::mir::join_ir_vm_bridge::join_func_name;
#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
mod tests;

/// Box-First: loop(true) break-once lowering to Normalized
pub struct LoopTrueBreakOnceBuilderBox;

impl LoopTrueBreakOnceBuilderBox {
    /// Try to lower loop(true) break-once pattern to Normalized JoinModule.
    ///
    /// Returns:
    /// - Ok(Some((module, meta))): Successfully lowered
    /// - Ok(None): Out of scope (fallback to legacy)
    /// - Err(msg): In scope but failed (internal error)
    pub fn lower(
        step_tree: &StepTree,
        env_layout: &EnvLayout,
    ) -> Result<Option<(JoinModule, JoinFragmentMeta)>, String> {
        crate::mir::control_tree::normalized_shadow::log_step_tree_gate_root(
            "loop_true_break_once",
        );
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "{} StepTree root: {:?}",
                crate::mir::control_tree::normalized_shadow::STEP_TREE_DEBUG_TAG,
                step_tree.root
            ));
        }

        // Extract loop(true) pattern from root
        let (prefix_nodes, loop_node, post_nodes) =
            match helpers::extract_loop_true_pattern(&step_tree.root) {
                Some(v) => v,
                None => {
                    if crate::config::env::joinir_dev::debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "{} extract_loop_true_pattern returned None",
                            crate::mir::control_tree::normalized_shadow::STEP_TREE_DEBUG_TAG
                        ));
                    }
                    return Ok(None); // Not a loop(true) pattern
                }
            };

        // Verify condition is Bool(true)
        let (cond_ast, body_node) = match loop_node {
            StepNode::Loop { cond_ast, body, .. } => (cond_ast, body.as_ref()),
            _ => return Ok(None),
        };

        if !NormalizedHelperBox::is_bool_true_literal(&cond_ast.0) {
            return Ok(None); // Condition is not Bool(true)
        }

        // Verify body ends with break
        let body_stmts = match body_node {
            StepNode::Block(stmts) => stmts,
            _ => return Ok(None), // Body is not a block
        };

        if !helpers::body_ends_with_break(body_stmts) {
            return Ok(None); // Body doesn't end with break
        }

        // Extract body statements (excluding final break)
        let body_prefix = &body_stmts[..body_stmts.len() - 1];

        // Verify no return/continue in body
        if helpers::has_unsupported_exits(body_prefix) {
            return Ok(None);
        }

        let env_fields = env_layout.env_fields();
        // Phase 143 fix: env params must be in Param region (100+) per JoinValueSpace contract.
        // All functions share the same params (env passing via continuation).
        let (main_params, mut next_value_id) =
            NormalizedHelperBox::alloc_env_params_param_region(&env_fields);

        // Function IDs (stable, dev-only).
        //
        // Contract: JoinIR→MIR bridge uses `JoinFuncId(2)` as the exit continuation (`k_exit`).
        let main_id = JoinFuncId::new(0);
        let loop_step_id = JoinFuncId::new(1);
        let k_exit_id = JoinFuncId::new(2);
        let loop_body_id = JoinFuncId::new(3);

        // main(env): <prefix> → TailCall(loop_step, env)
        // main_params allocated above in Param region. Clone for reuse.
        let mut env_main = NormalizedHelperBox::build_env_map(&env_fields, &main_params);
        let mut main_func =
            JoinFunction::new(main_id, "join_func_0".to_string(), main_params.clone());

        // Lower prefix (pre-loop) statements into main
        for n in prefix_nodes {
            match n {
                StepNode::Stmt { kind, .. } => match kind {
                    StepStmtKind::Assign {
                        ref target,
                        ref value_ast,
                    } => {
                        if LegacyLowerer::lower_assign_stmt(
                            target,
                            value_ast,
                            &mut main_func.body,
                            &mut next_value_id,
                            &mut env_main,
                        )
                        .is_err()
                        {
                            return Ok(None);
                        }
                    }
                    StepStmtKind::LocalDecl { .. } => {}
                    _ => {
                        return Ok(None);
                    }
                },
                _ => {
                    return Ok(None);
                }
            }
        }

        // main → loop_step tailcall
        let main_args =
            NormalizedHelperBox::collect_env_args(&env_fields, &env_main).map_err(|e| {
                error_tags::freeze_with_hint(
                    "phase131/loop_true/env_missing",
                    &e,
                    "ensure env layout and env map are built from the same SSOT field list",
                )
            })?;
        main_func.body.push(JoinInst::Call {
            func: loop_step_id,
            args: main_args,
            k_next: None,
            dst: None,
        });

        // loop_step(env): TailCall(loop_body, env)
        //
        // Contract: loop condition is Bool(true), so loop_step has no conditional branch.
        // This avoids introducing an unreachable "else" exit path that would require PHI.
        // Phase 143 fix: reuse Param region IDs for all functions
        let loop_step_params = main_params.clone();
        let env_loop_step = NormalizedHelperBox::build_env_map(&env_fields, &loop_step_params);
        let mut loop_step_func =
            JoinFunction::new(loop_step_id, "join_func_1".to_string(), loop_step_params);
        let loop_step_args = NormalizedHelperBox::collect_env_args(&env_fields, &env_loop_step)
            .map_err(|e| {
                error_tags::freeze_with_hint(
                    "phase131/loop_true/env_missing",
                    &e,
                    "ensure env layout and env map are built from the same SSOT field list",
                )
            })?;
        loop_step_func.body.push(JoinInst::Call {
            func: loop_body_id,
            args: loop_step_args,
            k_next: None,
            dst: None,
        });

        // loop_body(env): <assign statements> → TailCall(k_exit, env)
        // Phase 143 fix: reuse Param region IDs for all functions
        let loop_body_params = main_params.clone();
        let mut env_loop_body = NormalizedHelperBox::build_env_map(&env_fields, &loop_body_params);
        let env_loop_body_before = env_loop_body.clone();
        let mut loop_body_func =
            JoinFunction::new(loop_body_id, "join_func_3".to_string(), loop_body_params);

        // Lower body statements
        for n in body_prefix {
            match n {
                StepNode::Stmt { kind, .. } => match kind {
                    StepStmtKind::Assign {
                        ref target,
                        ref value_ast,
                    } => {
                        if LegacyLowerer::lower_assign_stmt(
                            target,
                            value_ast,
                            &mut loop_body_func.body,
                            &mut next_value_id,
                            &mut env_loop_body,
                        )
                        .is_err()
                        {
                            return Ok(None);
                        }
                    }
                    StepStmtKind::LocalDecl { .. } => {}
                    _ => {
                        return Ok(None);
                    }
                },
                _ => {
                    return Ok(None);
                }
            }
        }

        // loop_body → k_exit tailcall
        let loop_body_args = NormalizedHelperBox::collect_env_args(&env_fields, &env_loop_body)
            .map_err(|e| {
                error_tags::freeze_with_hint(
                    "phase131/loop_true/env_missing",
                    &e,
                    "ensure env layout and env map are built from the same SSOT field list",
                )
            })?;
        if crate::config::env::joinir_strict_enabled() {
            for n in body_prefix {
                let StepNode::Stmt { kind, .. } = n else {
                    continue;
                };
                let StepStmtKind::Assign { target, .. } = kind else {
                    continue;
                };
                let Some(target_name) = target.as_ref() else {
                    continue;
                };
                if !env_layout.writes.iter().any(|w| w == target_name) {
                    continue;
                }

                let before = env_loop_body_before.get(target_name).copied();
                let after = env_loop_body.get(target_name).copied();
                if let (Some(before), Some(after)) = (before, after) {
                    if before == after {
                        continue;
                    }

                    let idx = env_fields
                        .iter()
                        .position(|f| f == target_name)
                        .ok_or_else(|| {
                            error_tags::freeze_with_hint(
                                "phase131/loop_true/env_field_missing",
                                &format!("env_fields missing updated target '{target_name}'"),
                                "ensure EnvLayout.env_fields() is the SSOT used to build both env maps and call args",
                            )
                        })?;

                    let passed = loop_body_args.get(idx).copied().unwrap_or(ValueId(0));
                    if passed == before {
                        return Err(error_tags::freeze_with_hint(
                            "phase131/env_not_propagated",
                            &format!(
                                "loop_body updated '{target_name}' from {:?} to {:?}, but k_exit args still use the old ValueId {:?}",
                                before, after, passed
                            ),
                            "update env map before collecting k_exit args; use collect_env_args(env_fields, env) after assignments",
                        ));
                    }
                }
            }
        }
        loop_body_func.body.push(JoinInst::Call {
            func: k_exit_id,
            args: loop_body_args,
            k_next: None,
            dst: None,
        });

        // Phase 131/132-P4: ExitMeta SSOT (DirectValue)
        //
        // For Normalized shadow, the host variable_map reconnection must use the *final values*
        // produced by the loop (and post-loop if present), not the k_exit parameter placeholders.
        //
        // Contract: exit_values keys == env_layout.writes, values == final JoinIR-side ValueIds.
        // - Phase 131: Use env_loop_body values
        // - Phase 132-P4: Use env_post_k values (computed after post-loop lowering)
        use crate::mir::join_ir::lowering::carrier_info::ExitMeta;

        // Phase 132-P4/133-P0: Detect post-loop pattern
        // - post_nodes.is_empty() → Phase 131: k_exit → Ret(void)
        // - post_nodes == [Return(var)] → Phase 131: k_exit → Ret(env[var])
        // - post_nodes == [Assign, Return(var)] → Phase 132-P4: k_exit → TailCall(post_k)
        // - post_nodes == [Assign+, Return(var)] → Phase 133-P0: k_exit → TailCall(post_k)

        // DEBUG: Log post_nodes structure
        helpers::log_post_nodes_debug(post_nodes);

        // Phase 133-P0: Detect multi-assign + return pattern (generalize Phase 132-P4)
        let has_post_computation = if post_nodes.is_empty() {
            false
        } else if post_nodes.len() >= 2 {
            // Check if all nodes except the last are Assign statements
            let all_assigns = post_nodes[..post_nodes.len() - 1].iter().all(|n| {
                matches!(
                    n,
                    StepNode::Stmt {
                        kind: StepStmtKind::Assign { .. },
                        ..
                    }
                )
            });

            // Check if the last node is a Return statement
            let ends_with_return = matches!(
                post_nodes.last(),
                Some(StepNode::Stmt {
                    kind: StepStmtKind::Return { .. },
                    ..
                })
            );

            all_assigns && ends_with_return
        } else {
            false
        };

        helpers::log_post_computation_debug(has_post_computation);

        // k_exit(env): handle post-loop or return
        // Phase 143 fix: reuse Param region IDs for all functions
        // Phase 256 P1.7: Use canonical name from SSOT (legacy variant for normalized shadow)
        use crate::mir::join_ir::lowering::canonical_names as cn;
        let k_exit_params = main_params.clone();
        let env_k_exit = NormalizedHelperBox::build_env_map(&env_fields, &k_exit_params);
        let mut k_exit_func =
            JoinFunction::new(k_exit_id, cn::K_EXIT_LEGACY.to_string(), k_exit_params);

        if has_post_computation {
            // Phase 132-P4/133-P0: k_exit → TailCall(post_k, env)
            let post_k_id = JoinFuncId::new(4);
            let k_exit_args = NormalizedHelperBox::collect_env_args(&env_fields, &env_k_exit)
                .map_err(|e| {
                    error_tags::freeze_with_hint(
                        "phase131/loop_true/env_missing",
                        &e,
                        "ensure env layout and env map are built from the same SSOT field list",
                    )
                })?;
            k_exit_func.body.push(JoinInst::Call {
                func: post_k_id,
                args: k_exit_args,
                k_next: None,
                dst: None,
            });

            // post_k(env): <post assign>* → Ret(env[x])
            // Phase 143 fix: reuse Param region IDs for all functions
            let post_k_params = main_params.clone();
            let mut env_post_k = NormalizedHelperBox::build_env_map(&env_fields, &post_k_params);
            let mut post_k_func =
                JoinFunction::new(post_k_id, "join_func_4".to_string(), post_k_params);

            // Phase 133-P0: Lower multiple post-loop assignments
            // Split post_nodes into assigns and return (last element is return)
            let assign_nodes = &post_nodes[..post_nodes.len() - 1];
            let return_node = post_nodes.last().unwrap();

            // Lower all assignment statements
            for node in assign_nodes {
                let StepNode::Stmt {
                    kind:
                        StepStmtKind::Assign {
                            ref target,
                            ref value_ast,
                        },
                    ..
                } = node
                else {
                    return Ok(None);
                };
                if LegacyLowerer::lower_assign_stmt(
                    target,
                    value_ast,
                    &mut post_k_func.body,
                    &mut next_value_id,
                    &mut env_post_k,
                )
                .is_err()
                {
                    return Ok(None);
                }
            }

            // Lower post-loop return
            let StepNode::Stmt {
                kind: StepStmtKind::Return { ref value_ast },
                ..
            } = return_node
            else {
                return Ok(None);
            };

            // Lower post-loop return (Phase 136: support variable + integer literal)
            if value_ast.is_some() {
                // Return with value
                match ReturnValueLowererBox::lower_to_value_id(
                    value_ast,
                    &mut post_k_func.body,
                    &mut next_value_id,
                    &env_post_k,
                )? {
                    Some(vid) => {
                        post_k_func.body.push(JoinInst::Ret { value: Some(vid) });
                    }
                    None => {
                        // Out of scope (unsupported return value type)
                        return Ok(None);
                    }
                }
            } else {
                // Return void
                post_k_func.body.push(JoinInst::Ret { value: None });
            }

            // Phase 132-P4/133-P0: ExitMeta must use post_k's final env values
            let mut exit_values_for_meta: Vec<(String, ValueId)> = Vec::new();
            for var_name in &env_layout.writes {
                let final_vid = env_post_k.get(var_name).copied().ok_or_else(|| {
                    error_tags::freeze_with_hint(
                        "phase133/exit_meta/missing_final_value",
                        &format!("post_k env missing final value for write '{var_name}'"),
                        "ensure post-loop assignments update the env map before exit meta is computed",
                    )
                })?;
                exit_values_for_meta.push((var_name.clone(), final_vid));
            }

            // Build module with post_k
            let mut module = JoinModule::new();
            module.add_function(main_func);
            module.add_function(loop_step_func);
            module.add_function(loop_body_func);
            module.add_function(k_exit_func);
            module.add_function(post_k_func);
            module.entry = Some(main_id);

            // Phase 132-P4/133-P0 DEBUG: Verify all 5 functions are added
            if crate::config::env::joinir_dev_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[phase133/debug] JoinModule has {} functions (expected 5)",
                    module.functions.len()
                ));
                for (id, func) in &module.functions {
                    ring0.log.debug(&format!(
                        "[phase133/debug]   Function {}: {} ({} instructions)",
                        id.0,
                        func.name,
                        func.body.len()
                    ));
                }
            }

            let exit_meta = ExitMeta {
                exit_values: exit_values_for_meta,
            };
            let mut meta = JoinFragmentMeta::carrier_only(exit_meta);
            // Phase 256 P1.7: Use canonical name from SSOT (legacy variant for normalized shadow)
            meta.continuation_funcs
                .insert(cn::K_EXIT_LEGACY.to_string());

            return Ok(Some((module, meta)));
        }

        // Phase 131: Handle simple post-loop or no post-loop
        if post_nodes.is_empty() {
            // No post-loop: return void
            k_exit_func.body.push(JoinInst::Ret { value: None });
        } else if post_nodes.len() == 1 {
            // Single post statement: check if it's a return
            match &post_nodes[0] {
                StepNode::Stmt { kind, .. } => match kind {
                    StepStmtKind::Return { ref value_ast } => {
                        if value_ast.is_some() {
                            // Return with value (Phase 136: variable + integer literal)
                            match ReturnValueLowererBox::lower_to_value_id(
                                value_ast,
                                &mut k_exit_func.body,
                                &mut next_value_id,
                                &env_k_exit,
                            )? {
                                Some(vid) => {
                                    k_exit_func.body.push(JoinInst::Ret { value: Some(vid) });
                                }
                                None => {
                                    // Out of scope
                                    return Ok(None);
                                }
                            }
                        } else {
                            // Return void
                            k_exit_func.body.push(JoinInst::Ret { value: None });
                        }
                    }
                    _ => {
                        return Ok(None); // Unsupported post statement
                    }
                },
                _ => {
                    return Ok(None);
                }
            }
        } else {
            return Ok(None); // Unsupported post pattern
        }

        // Phase 131: ExitMeta uses loop_body's final env values
        let mut exit_values_for_meta: Vec<(String, ValueId)> = Vec::new();
        for var_name in &env_layout.writes {
            let final_vid = env_loop_body.get(var_name).copied().ok_or_else(|| {
                error_tags::freeze_with_hint(
                    "phase131/exit_meta/missing_final_value",
                    &format!("env missing final value for write '{var_name}'"),
                    "ensure loop body assignments update the env map before exit meta is computed",
                )
            })?;
            exit_values_for_meta.push((var_name.clone(), final_vid));
        }

        // Build module (Phase 131)
        let mut module = JoinModule::new();
        module.add_function(main_func);
        module.add_function(loop_step_func);
        module.add_function(loop_body_func);
        module.add_function(k_exit_func);
        module.entry = Some(main_id);
        // Phase 131 P1: Keep as Structured for execution via bridge
        // (Normalized is only for dev observation/verification)

        let exit_meta = ExitMeta {
            exit_values: exit_values_for_meta,
        };
        let mut meta = JoinFragmentMeta::carrier_only(exit_meta);
        // Phase 256 P1.7: Use canonical name from SSOT (legacy variant for normalized shadow)
        meta.continuation_funcs
            .insert(cn::K_EXIT_LEGACY.to_string());

        Ok(Some((module, meta)))
    }

    // helper methods moved to loop_true_break_once_helpers.rs
}
