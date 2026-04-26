//! Phase 143 P0/P1: loop(true) + if + break/continue Normalized lowering
//!
//! ## Responsibility
//!
//! - Lower `loop(true) { if(cond_pure) break/continue }` to Normalized JoinModule
//! - Conditional exit control flow (Branch instructions)
//! - PHI-free: all state passing via env arguments + continuations
//! - Pure condition expressions only (Phase 143 scope)
//!
//! ## Scope
//!
//! - Loop condition: `true` literal only
//! - Loop body: Single `if` statement only
//! - If branch: `break` (P0) or `continue` (P1)
//! - Condition expression: Pure only (variables, literals, arith, compare)
//! - Else branch: Not supported (P2 feature)
//!
//! ## Return Behavior
//!
//! - Ok(Some((module, meta))): Successfully lowered
//! - Ok(None): Out of scope (try the next route)
//! - Err(msg): In scope but failed (internal error, strict mode freeze)

use super::common::expr_lowerer_box::NormalizedExprLowererBox;
use super::common::expr_lowering_contract::ExprLoweringScope;
use super::common::loop_if_exit_contract::{LoopIfExitShape, LoopIfExitThen, OutOfScopeReason};
use super::common::normalized_helpers::NormalizedHelperBox;
use super::env_layout::EnvLayout;
use crate::mir::control_tree::step_tree::{StepNode, StepStmtKind, StepTree};
use crate::mir::join_ir::lowering::canonical_names as cn;
use crate::mir::join_ir::lowering::carrier_info::JoinFragmentMeta;
use crate::mir::join_ir::{JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst, UnaryOp};
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Box-First: loop(true) + if + break/continue lowering to Normalized
pub struct LoopTrueIfBreakContinueBuilderBox;

impl LoopTrueIfBreakContinueBuilderBox {
    /// Try to lower loop(true) + if + break/continue pattern to Normalized JoinModule.
    ///
    /// Phase 143 P0/P1: Supports break (P0) and continue (P1)
    pub fn lower(
        step_tree: &StepTree,
        env_layout: &EnvLayout,
    ) -> Result<Option<(JoinModule, JoinFragmentMeta)>, String> {
        // DEBUG: Log attempt
        if crate::config::env::joinir_dev_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[phase143/debug] Attempting loop_true_if_break/continue pattern (P0/P1)"
            ));
        }

        // Shape: loop(true) { if(cond) break/continue }
        // Step 1: Extract shape match
        let (shape, cond_ast) = match Self::extract_pattern_shape(&step_tree.root) {
            Ok(Some((s, cond))) => {
                if crate::config::env::joinir_dev_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[phase143/debug] Route shape matched: loop(true) if break/continue"
                    ));
                }
                (s, cond)
            }
            Ok(None) => {
                if crate::config::env::joinir_dev_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[phase143/debug] Route shape not matched, out of scope"
                    ));
                }
                return Ok(None); // Out of scope
            }
            Err(reason) => {
                if crate::config::env::joinir_dev_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[phase143/debug] Route shape out-of-scope: {:?}",
                        reason
                    ));
                }
                return Ok(None);
            }
        };

        // Validate that shape is P2-compatible (supports break/continue with optional else)
        if let Err(reason) = shape.validate_for_p2() {
            if crate::config::env::joinir_dev_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[phase143/debug] P2 validation failed: {:?}",
                    reason
                ));
            }
            return Ok(None);
        }

        // Step 3 (P0): Validate condition lowering with PureOnly scope
        // This step checks if the condition can be lowered as a pure expression.
        // We don't emit the actual JoinModule yet (that's Steps 4-6),
        // but we verify early that the condition is in scope.

        let env_fields = env_layout.env_fields();
        let mut env_check: BTreeMap<String, ValueId> = BTreeMap::new();

        // Build a dummy env for validation (temp ValueIds)
        let mut temp_vid = 1u32;
        for field in &env_fields {
            env_check.insert(field.clone(), ValueId(temp_vid));
            temp_vid += 1;
        }

        // Try to lower the condition with PureOnly scope
        let mut dummy_body: Vec<JoinInst> = Vec::new();
        let mut temp_next_vid = temp_vid;

        match NormalizedExprLowererBox::lower_expr_with_scope(
            ExprLoweringScope::PureOnly,
            &cond_ast,
            &env_check,
            &mut dummy_body,
            &mut temp_next_vid,
        ) {
            Ok(Some(_vid)) => {
                if crate::config::env::joinir_dev_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0
                        .log
                        .debug(&format!("[phase143/debug] Condition is pure and lowerable"));
                }
            }
            Ok(None) => {
                if crate::config::env::joinir_dev_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[phase143/debug] Condition is not pure (impure/out-of-scope for this route)"
                    ));
                }
                return Ok(None); // Condition not pure; out of scope for this route
            }
            Err(e) => {
                return Err(format!("phase143/condition_lowering: {}", e));
            }
        }

        // Step 4 (P0): Build JoinModule with 6 functions and Branch instructions
        // === Phase 4: Allocate IDs and build env ===
        let env_fields = env_layout.env_fields();
        // Phase 143 fix: env params must be in Param region (100+) per JoinValueSpace contract.
        // All functions share the same params (env passing via continuation).
        let (main_params, mut next_value_id) =
            NormalizedHelperBox::alloc_env_params_param_region(&env_fields);

        // Allocate 6 JoinFuncIds
        let main_id = JoinFuncId::new(0);
        let loop_step_id = JoinFuncId::new(1);
        let loop_cond_check_id = JoinFuncId::new(2);
        let k_then_id = JoinFuncId::new(3);
        let k_else_id = JoinFuncId::new(4);
        let k_exit_id = JoinFuncId::new(5);

        // === main(env) ===
        // main_params allocated above in Param region. Clone for reuse.
        let env_main = NormalizedHelperBox::build_env_map(&env_fields, &main_params);
        let main_func = {
            let mut f = JoinFunction::new(main_id, cn::MAIN.to_string(), main_params.clone());
            // main: Call(loop_step, env)
            let loop_step_args = NormalizedHelperBox::collect_env_args(&env_fields, &env_main)?;
            f.body.push(JoinInst::Call {
                func: loop_step_id,
                args: loop_step_args,
                k_next: None,
                dst: None,
            });
            f
        };

        // === loop_step(env) ===
        // Phase 143 fix: reuse Param region IDs for all functions (consistent env passing)
        let loop_step_params = main_params.clone();
        let env_loop_step = NormalizedHelperBox::build_env_map(&env_fields, &loop_step_params);
        let loop_step_func = {
            let mut f =
                JoinFunction::new(loop_step_id, cn::LOOP_STEP.to_string(), loop_step_params);
            // loop_step: Call(loop_cond_check, env)
            let loop_cond_check_args =
                NormalizedHelperBox::collect_env_args(&env_fields, &env_loop_step)?;
            f.body.push(JoinInst::Call {
                func: loop_cond_check_id,
                args: loop_cond_check_args,
                k_next: None,
                dst: None,
            });
            f
        };

        // === loop_cond_check(env): Lower condition and emit Jump ===
        // Phase 143 fix: reuse Param region IDs
        let loop_cond_check_params = main_params.clone();
        let env_loop_cond_check =
            NormalizedHelperBox::build_env_map(&env_fields, &loop_cond_check_params);
        let loop_cond_check_func = {
            let mut f = JoinFunction::new(
                loop_cond_check_id,
                "loop_cond_check".to_string(),
                loop_cond_check_params,
            );

            // Reuse the validated condition lowering from Step 3
            let cond_vid = match NormalizedExprLowererBox::lower_expr_with_scope(
                ExprLoweringScope::PureOnly,
                &cond_ast,
                &env_loop_cond_check,
                &mut f.body,
                &mut next_value_id,
            ) {
                Ok(Some(vid)) => vid,
                _ => {
                    return Err(
                        "phase143/branch_emission: condition lowering failed unexpectedly"
                            .to_string(),
                    );
                }
            };

            // P2: Emit conditional Jump/Call based on then/else actions (4-way match)
            let k_exit_args =
                NormalizedHelperBox::collect_env_args(&env_fields, &env_loop_cond_check)?;
            let loop_step_args =
                NormalizedHelperBox::collect_env_args(&env_fields, &env_loop_cond_check)?;

            match (shape.then, shape.else_) {
                // P0: Break-only (no else)
                (LoopIfExitThen::Break, None) => {
                    // If cond_vid is true: jump to k_exit (BREAK)
                    f.body.push(JoinInst::Jump {
                        cont: k_exit_id.as_cont(),
                        args: k_exit_args,
                        cond: Some(cond_vid),
                    });
                    // If cond_vid is false: call loop_step() to iterate again
                    f.body.push(JoinInst::Call {
                        func: loop_step_id,
                        args: loop_step_args,
                        k_next: None,
                        dst: None,
                    });
                }

                // P1: Continue-only (no else)
                (LoopIfExitThen::Continue, None) => {
                    // Unconditional continue (condition ignored)
                    let _ = cond_vid;
                    f.body.push(JoinInst::Call {
                        func: loop_step_id,
                        args: loop_step_args,
                        k_next: None,
                        dst: None,
                    });
                }

                // P2: Break-Continue (then=break, else=continue)
                (LoopIfExitThen::Break, Some(LoopIfExitThen::Continue)) => {
                    // If cond true: jump to k_exit (break)
                    f.body.push(JoinInst::Jump {
                        cont: k_exit_id.as_cont(),
                        args: k_exit_args,
                        cond: Some(cond_vid),
                    });
                    // If cond false: call loop_step (continue)
                    f.body.push(JoinInst::Call {
                        func: loop_step_id,
                        args: loop_step_args,
                        k_next: None,
                        dst: None,
                    });
                }

                // P2: Continue-Break (then=continue, else=break)
                (LoopIfExitThen::Continue, Some(LoopIfExitThen::Break)) => {
                    // If cond true: call loop_step (continue)
                    // If cond false: jump to k_exit (break)
                    // Strategy: Invert condition
                    let inverted_cond = NormalizedHelperBox::alloc_value_id(&mut next_value_id);
                    f.body.push(JoinInst::Compute(MirLikeInst::UnaryOp {
                        dst: inverted_cond,
                        op: UnaryOp::Not,
                        operand: cond_vid,
                    }));
                    f.body.push(JoinInst::Jump {
                        cont: k_exit_id.as_cont(),
                        args: k_exit_args,
                        cond: Some(inverted_cond),
                    });
                    f.body.push(JoinInst::Call {
                        func: loop_step_id,
                        args: loop_step_args,
                        k_next: None,
                        dst: None,
                    });
                }

                // P2: Break-Break (both branches break)
                (LoopIfExitThen::Break, Some(LoopIfExitThen::Break)) => {
                    // Unconditional jump to k_exit
                    let _ = cond_vid;
                    f.body.push(JoinInst::Jump {
                        cont: k_exit_id.as_cont(),
                        args: k_exit_args,
                        cond: None, // Unconditional
                    });
                }

                // P2: Continue-Continue (both branches continue)
                (LoopIfExitThen::Continue, Some(LoopIfExitThen::Continue)) => {
                    // Unconditional call to loop_step
                    let _ = cond_vid;
                    f.body.push(JoinInst::Call {
                        func: loop_step_id,
                        args: loop_step_args,
                        k_next: None,
                        dst: None,
                    });
                }
            }

            f
        };

        // === k_then(env): Reserved (P2+) ===
        //
        // Phase 143 P1 does not emit conditional Jump for Continue (see loop_cond_check note),
        // so k_then is currently unused. It is kept as a placeholder for P2+ extensions
        // (e.g., else-branch support).
        // Phase 143 fix: reuse Param region IDs
        let k_then_params = main_params.clone();
        let k_then_func = { JoinFunction::new(k_then_id, "k_then".to_string(), k_then_params) };

        // === k_else(env): Not used in P0 (direct Call from loop_cond_check fallthrough) ===
        // Kept for structural clarity in case future extensions need it
        // Phase 143 fix: reuse Param region IDs
        let k_else_params = main_params.clone();
        let _env_k_else = NormalizedHelperBox::build_env_map(&env_fields, &k_else_params);
        let k_else_func = {
            let f = JoinFunction::new(k_else_id, "k_else".to_string(), k_else_params);
            // Empty placeholder: P0 doesn't use this function
            f
        };

        // === k_exit(env): Return with exit values (Phase 143 P0 Steps 5-6) ===
        // Phase 143 fix: reuse Param region IDs
        let k_exit_params = main_params.clone();
        let env_k_exit = NormalizedHelperBox::build_env_map(&env_fields, &k_exit_params);
        let k_exit_func = {
            let mut f = JoinFunction::new(k_exit_id, cn::K_EXIT.to_string(), k_exit_params.clone());

            // Phase 143 P0: Build exit values from env_layout.writes
            // Each write variable is returned as an exit value
            let mut exit_values: Vec<(String, ValueId)> = Vec::new();
            for write_var in &env_layout.writes {
                if let Some(vid) = env_k_exit.get(write_var) {
                    exit_values.push((write_var.clone(), *vid));
                }
            }

            if crate::config::env::joinir_dev_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[phase143/debug] k_exit: {} exit values",
                    exit_values.len()
                ));
            }

            // For now, return with the first exit value if any exist, else None
            // This is a simplified P0 implementation
            let return_value = if !exit_values.is_empty() {
                Some(exit_values[0].1)
            } else {
                None
            };

            f.body.push(JoinInst::Ret {
                value: return_value,
            });

            f
        };

        // === Build JoinModule ===
        let mut module = JoinModule::new();
        module.add_function(main_func);
        module.add_function(loop_step_func);
        module.add_function(loop_cond_check_func);
        module.add_function(k_then_func);
        module.add_function(k_else_func);
        module.add_function(k_exit_func);

        module.entry = Some(main_id);
        module.mark_normalized();

        if crate::config::env::joinir_dev_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[phase143/debug] JoinModule complete: 6 functions with conditional Jump/Return"
            ));
        }

        // === Build ExitMeta for carrier info ===
        // Step 6: ExitMeta construction with exit values
        let mut exit_meta_values: Vec<(String, ValueId)> = Vec::new();
        for write_var in &env_layout.writes {
            if let Some(vid) = env_k_exit.get(write_var) {
                exit_meta_values.push((write_var.clone(), *vid));
            }
        }

        let meta = JoinFragmentMeta::carrier_only(
            crate::mir::join_ir::lowering::carrier_info::ExitMeta::multiple(exit_meta_values),
        );

        if crate::config::env::joinir_dev_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[phase143/debug] Phase 143 P0: JoinModule and ExitMeta complete"
            ));
        }

        // ✅ Return complete JoinModule + ExitMeta (Phase 143 P0 full implementation!)
        Ok(Some((module, meta)))
    }

    /// Extract loop(true) + single if + break/continue pattern
    ///
    /// Returns Ok(Some((shape, cond_ast))) if pattern matches, Ok(None) if clearly out of scope,
    /// and Err for specific out-of-scope reasons.
    /// Shape: StepNode::Loop with body containing single If with break/continue
    /// Also returns the If condition AST for lowering.
    fn extract_pattern_shape(
        root: &StepNode,
    ) -> Result<Option<(LoopIfExitShape, crate::ast::ASTNode)>, OutOfScopeReason> {
        match root {
            StepNode::Loop { cond_ast, body, .. } => {
                // Condition must be Bool(true) literal
                if !NormalizedHelperBox::is_bool_true_literal(&cond_ast.0) {
                    return Err(OutOfScopeReason::NotLoopTrue);
                }

                // Body must be single if statement (possibly wrapped in Block)
                let if_node = match body.as_ref() {
                    StepNode::If { .. } => Some(body.as_ref()),
                    StepNode::Block(stmts) if stmts.len() == 1 => match &stmts[0] {
                        StepNode::If { .. } => Some(&stmts[0]),
                        _ => None,
                    },
                    _ => None,
                };

                let if_node = if_node.ok_or(OutOfScopeReason::BodyNotSingleIf)?;

                // If statement structure: if(cond_pure) { break/continue }
                if let StepNode::If {
                    cond_ast: if_cond,
                    then_branch,
                    else_branch,
                    ..
                } = if_node
                {
                    // Extract then action (P0/P1/P2: Break OR Continue)
                    let then_action = Self::extract_exit_action(then_branch)?;

                    // P2: Extract else action if present
                    let else_action = if let Some(else_node) = else_branch {
                        Some(Self::extract_exit_action(else_node)?)
                    } else {
                        None
                    };

                    // Build contract shape
                    let shape = LoopIfExitShape {
                        has_else: else_branch.is_some(),
                        then: then_action,
                        else_: else_action,
                        cond_scope: ExprLoweringScope::PureOnly,
                    };

                    Ok(Some((shape, (*if_cond.0).clone())))
                } else {
                    Err(OutOfScopeReason::BodyNotSingleIf)
                }
            }
            _ => Err(OutOfScopeReason::NotLoopTrue),
        }
    }

    /// Extract exit action from branch node
    ///
    /// Returns Ok(LoopIfExitThen::Break) for break statements
    /// Returns Ok(LoopIfExitThen::Continue) for continue statements
    /// Returns Err for unsupported branches
    fn extract_exit_action(branch: &StepNode) -> Result<LoopIfExitThen, OutOfScopeReason> {
        match branch {
            StepNode::Stmt {
                kind: StepStmtKind::Break,
                ..
            } => Ok(LoopIfExitThen::Break),
            StepNode::Stmt {
                kind: StepStmtKind::Continue,
                ..
            } => Ok(LoopIfExitThen::Continue),
            StepNode::Block(stmts) if stmts.len() == 1 => match &stmts[0] {
                StepNode::Stmt {
                    kind: StepStmtKind::Break,
                    ..
                } => Ok(LoopIfExitThen::Break),
                StepNode::Stmt {
                    kind: StepStmtKind::Continue,
                    ..
                } => Ok(LoopIfExitThen::Continue),
                _ => Err(OutOfScopeReason::ThenNotExit(
                    "Not break/continue".to_string(),
                )),
            },
            _ => Err(OutOfScopeReason::ThenNotExit(
                "Complex branch not supported".to_string(),
            )),
        }
    }
}

// Unit tests are in: normalized_shadow/tests/phase143_loop_if_exit_contract.rs
// (Refactored in Phase 143 R0 to separate concerns)
