//! Phase 129-C: Post-if continuation lowering with post_k
//!
//! ## Responsibility
//!
//! - Lower `Seq([If, Post...])` patterns to join_k + post_k continuation
//! - join_k merges environments from then/else branches
//! - post_k executes post-if statements and returns
//! - PHI-free: all merging done via env arguments
//!
//! ## Contract
//!
//! - Input: StepTree with if-only pattern + post-if statements
//! - Output: JoinModule with:
//!   - main: condition check → TailCall(k_then/k_else, env)
//!   - k_then: then statements → TailCall(join_k, env_then)
//!   - k_else: else statements → TailCall(join_k, env_else)
//!   - join_k: merge → TailCall(post_k, merged_env)
//!   - post_k: post-if statements → Ret
//!
//! ## Scope
//!
//! - Post-if: Return value lowering uses `ReturnValueLowererBox` (Phase 138 SSOT)
//! - If body: Assign(int literal) only (Phase 128 baseline)
//! - Condition: minimal compare only (Phase 123 baseline)
//!
//! ## Fail-Fast
//!
//! - Out of scope → Ok(None) (try the next route)
//! - In scope but conversion failed → Err (with freeze_with_hint in strict mode)

use std::collections::BTreeMap;

use super::common::expr_lowerer_box::NormalizedExprLowererBox;
use super::common::expr_lowering_contract::ExprLoweringScope;
use super::common::normalized_helpers::NormalizedHelperBox;
use super::common::return_value_lowerer_box::ReturnValueLowererBox;
use super::env_layout::EnvLayout;
use super::support::expr_lowering;
use crate::mir::control_tree::step_tree::{StepNode, StepStmtKind, StepTree};
use crate::mir::join_ir::lowering::carrier_info::JoinFragmentMeta;
use crate::mir::join_ir::lowering::error_tags;
use crate::mir::join_ir::{
    ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst,
};
use crate::mir::ValueId;

/// Box-First: Post-if continuation lowering with post_k
pub struct PostIfPostKBuilderBox;

impl PostIfPostKBuilderBox {
    /// Try to lower if-with-post pattern to Normalized JoinModule using post_k.
    ///
    /// Returns:
    /// - Ok(Some((module, meta))): Successfully lowered
    /// - Ok(None): Out of scope (try the next route)
    /// - Err(msg): In scope but failed (internal error)
    pub fn lower(
        step_tree: &StepTree,
        env_layout: &EnvLayout,
    ) -> Result<Option<(JoinModule, JoinFragmentMeta)>, String> {
        // Extract if + post pattern
        let (prefix_nodes, if_node, post_nodes) = match Self::extract_if_with_post(&step_tree.root)
        {
            Some(v) => v,
            None => return Ok(None), // Not an if-with-post pattern
        };

        let env_fields = env_layout.env_fields();
        // Phase 143 fix: env params must be in Param region (100+) per JoinValueSpace contract.
        // All functions share the same params (env passing via continuation).
        let (main_params, mut next_value_id) =
            NormalizedHelperBox::alloc_env_params_param_region(&env_fields);

        // IDs (stable, dev-only)
        let main_id = JoinFuncId::new(0);
        let k_then_id = JoinFuncId::new(1);
        let k_else_id = JoinFuncId::new(2);
        let join_k_id = JoinFuncId::new(3);
        let post_k_id = JoinFuncId::new(4);

        // main(env)
        // main_params allocated above in Param region. Clone for reuse.
        let mut env_main = NormalizedHelperBox::build_env_map(&env_fields, &main_params);
        let mut main_func = JoinFunction::new(main_id, "main".to_string(), main_params.clone());

        // Lower prefix (pre-if) statements into main
        for n in prefix_nodes {
            match n {
                StepNode::Stmt { kind, .. } => match kind {
                    StepStmtKind::Assign { target, value_ast } => {
                        if expr_lowering::lower_assign_stmt(
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

        // Extract if condition and branches
        let (cond_ast, then_branch, else_branch) = match if_node {
            StepNode::If {
                cond_ast,
                then_branch,
                else_branch,
                ..
            } => (cond_ast, then_branch.as_ref(), else_branch.as_deref()),
            _ => unreachable!(),
        };

        let else_branch = match else_branch {
            Some(b) => b,
            None => return Ok(None), // Phase 129-C requires explicit else
        };

        // Extract branch statements (without return, since post-if handles return)
        let then_stmts = Self::extract_branch_stmts(then_branch)?;
        let else_stmts = Self::extract_branch_stmts(else_branch)?;

        // k_then(env_in): <then_stmts> ; tailcall join_k(env_out)
        // Phase 143 fix: reuse Param region IDs for all functions
        let then_params = main_params.clone();
        let mut env_then = NormalizedHelperBox::build_env_map(&env_fields, &then_params);
        let mut then_func = JoinFunction::new(k_then_id, "k_then".to_string(), then_params);

        for stmt in then_stmts {
            match stmt {
                StepStmtKind::Assign { target, value_ast } => {
                    if expr_lowering::lower_assign_stmt(
                        target,
                        value_ast,
                        &mut then_func.body,
                        &mut next_value_id,
                        &mut env_then,
                    )
                    .is_err()
                    {
                        return Ok(None);
                    }
                }
                StepStmtKind::LocalDecl { .. } => {}
                _ => {
                    return Ok(None); // Unsupported statement
                }
            }
        }

        let then_args =
            NormalizedHelperBox::collect_env_args(&env_fields, &env_then).map_err(|e| {
                error_tags::freeze_with_hint(
                    "phase129/post_k/env_missing",
                    &e,
                    "ensure env layout and env map are built from the same SSOT field list",
                )
            })?;
        then_func.body.push(JoinInst::Call {
            func: join_k_id,
            args: then_args,
            k_next: None,
            dst: None,
        });

        // k_else(env_in): <else_stmts> ; tailcall join_k(env_out)
        // Phase 143 fix: reuse Param region IDs for all functions
        let else_params = main_params.clone();
        let mut env_else = NormalizedHelperBox::build_env_map(&env_fields, &else_params);
        let mut else_func = JoinFunction::new(k_else_id, "k_else".to_string(), else_params);

        for stmt in else_stmts {
            match stmt {
                StepStmtKind::Assign { target, value_ast } => {
                    if expr_lowering::lower_assign_stmt(
                        target,
                        value_ast,
                        &mut else_func.body,
                        &mut next_value_id,
                        &mut env_else,
                    )
                    .is_err()
                    {
                        return Ok(None);
                    }
                }
                StepStmtKind::LocalDecl { .. } => {}
                _ => {
                    return Ok(None); // Unsupported statement
                }
            }
        }

        let else_args =
            NormalizedHelperBox::collect_env_args(&env_fields, &env_else).map_err(|e| {
                error_tags::freeze_with_hint(
                    "phase129/post_k/env_missing",
                    &e,
                    "ensure env layout and env map are built from the same SSOT field list",
                )
            })?;
        else_func.body.push(JoinInst::Call {
            func: join_k_id,
            args: else_args,
            k_next: None,
            dst: None,
        });

        // join_k(env_phi): tailcall post_k(env_phi)
        // Phase 143 fix: reuse Param region IDs for all functions
        let join_k_params = main_params.clone();
        let env_join_k = NormalizedHelperBox::build_env_map(&env_fields, &join_k_params);
        let mut join_k_func = JoinFunction::new(join_k_id, "join_k".to_string(), join_k_params);

        let join_k_args =
            NormalizedHelperBox::collect_env_args(&env_fields, &env_join_k).map_err(|e| {
                error_tags::freeze_with_hint(
                    "phase129/post_k/env_missing",
                    &e,
                    "ensure env layout and env map are built from the same SSOT field list",
                )
            })?;
        join_k_func.body.push(JoinInst::Call {
            func: post_k_id,
            args: join_k_args,
            k_next: None,
            dst: None,
        });

        // post_k(env): <post_stmts> ; Ret
        // Phase 143 fix: reuse Param region IDs for all functions
        let post_k_params = main_params.clone();
        let env_post_k = NormalizedHelperBox::build_env_map(&env_fields, &post_k_params);
        let mut post_k_func = JoinFunction::new(post_k_id, "post_k".to_string(), post_k_params);

        // Lower post-if statements
        for n in post_nodes {
            match n {
                StepNode::Stmt { kind, .. } => match kind {
                    StepStmtKind::Return { value_ast } => {
                        if let Some(_ast_handle) = value_ast {
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
                                    return Ok(None);
                                }
                            }
                        } else {
                            post_k_func.body.push(JoinInst::Ret { value: None });
                        }
                    }
                    _ => {
                        return Ok(None); // Unsupported post-if statement
                    }
                },
                _ => {
                    return Ok(None);
                }
            }
        }

        // If no return was emitted, add void return
        if !post_k_func
            .body
            .iter()
            .any(|inst| matches!(inst, JoinInst::Ret { .. }))
        {
            post_k_func.body.push(JoinInst::Ret { value: None });
        }

        // Phase 146 P0: Use lower_expr_with_scope() SSOT (legacy fallback)
        let cond_vid = match NormalizedExprLowererBox::lower_expr_with_scope(
            ExprLoweringScope::PureOnly,
            &cond_ast.0,
            &env_main,
            &mut main_func.body,
            &mut next_value_id,
        ) {
            Ok(Some(vid)) => vid,
            Ok(None) => {
                // Fall back to the Phase 129 baseline minimal compare route.
                Self::lower_condition_baseline(
                    &cond_ast.0,
                    &env_main,
                    &mut main_func.body,
                    &mut next_value_id,
                )?
            }
            Err(e) => return Err(format!("phase146/p0/cond_lowering: {}", e)),
        };

        let main_args =
            NormalizedHelperBox::collect_env_args(&env_fields, &env_main).map_err(|e| {
                error_tags::freeze_with_hint(
                    "phase129/post_k/env_missing",
                    &e,
                    "ensure env layout and env map are built from the same SSOT field list",
                )
            })?;
        main_func.body.push(JoinInst::Jump {
            cont: k_then_id.as_cont(),
            args: main_args.clone(),
            cond: Some(cond_vid),
        });
        main_func.body.push(JoinInst::Jump {
            cont: k_else_id.as_cont(),
            args: main_args,
            cond: None,
        });

        // Build module
        let mut module = JoinModule::new();
        module.add_function(main_func);
        module.add_function(then_func);
        module.add_function(else_func);
        module.add_function(join_k_func);
        module.add_function(post_k_func);
        module.entry = Some(main_id);
        module.mark_normalized();

        Ok(Some((module, JoinFragmentMeta::empty())))
    }

    /// Extract if-with-post pattern: (prefix, if_node, post)
    ///
    /// Returns None if not an if-with-post pattern.
    fn extract_if_with_post(node: &StepNode) -> Option<(&[StepNode], &StepNode, &[StepNode])> {
        match node {
            StepNode::Block(nodes) => {
                // Find the last If node
                let if_pos = nodes
                    .iter()
                    .position(|n| matches!(n, StepNode::If { .. }))?;

                // Must have post-if statements
                if if_pos == nodes.len() - 1 {
                    return None; // No post-if (this is handled by if_as_last_join_k)
                }

                let if_node = &nodes[if_pos];
                let prefix = &nodes[..if_pos];
                let post = &nodes[if_pos + 1..];

                Some((prefix, if_node, post))
            }
            _ => None,
        }
    }

    /// Extract statements from a branch (excluding return)
    ///
    /// Phase 129-C: branches should not end with return (post-if handles return)
    fn extract_branch_stmts(branch: &StepNode) -> Result<Vec<&StepStmtKind>, String> {
        match branch {
            StepNode::Block(nodes) => {
                let mut stmts = Vec::new();
                for n in nodes {
                    match n {
                        StepNode::Stmt { kind, .. } => {
                            // Skip return statements (handled in post_k)
                            if !matches!(kind, StepStmtKind::Return { .. }) {
                                stmts.push(kind);
                            }
                        }
                        _ => {
                            // Unsupported node in branch
                            return Ok(Vec::new()); // Signal out-of-scope
                        }
                    }
                }
                Ok(stmts)
            }
            StepNode::Stmt { kind, .. } => {
                // Single statement branch
                if matches!(kind, StepStmtKind::Return { .. }) {
                    Ok(Vec::new()) // Empty if only return
                } else {
                    Ok(vec![kind])
                }
            }
            _ => Ok(Vec::new()), // Unsupported
        }
    }

    /// Phase 146 P0: Baseline condition lowering for out-of-scope ANF cases.
    ///
    /// When ANF routing is unavailable (e.g., PureOnly scope, HAKO_ANF_DEV=0),
    /// fall back to Phase 129 baseline minimal compare lowering.
    fn lower_condition_baseline(
        cond_ast: &crate::ast::ASTNode,
        env: &BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<ValueId, String> {
        let (lhs_var, op, rhs_literal) = expr_lowering::parse_minimal_compare(cond_ast)?;
        let lhs_vid = env.get(&lhs_var).copied().ok_or_else(|| {
            error_tags::freeze_with_hint(
                "phase146/p0/cond_lhs_missing",
                &format!("condition lhs '{lhs_var}' not in env"),
                "ensure variable exists in writes or inputs",
            )
        })?;

        let rhs_vid = NormalizedHelperBox::alloc_value_id(next_value_id);
        body.push(JoinInst::Compute(MirLikeInst::Const {
            dst: rhs_vid,
            value: ConstValue::Integer(rhs_literal),
        }));

        let cond_vid = NormalizedHelperBox::alloc_value_id(next_value_id);
        body.push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cond_vid,
            op,
            lhs: lhs_vid,
            rhs: rhs_vid,
        }));

        Ok(cond_vid)
    }
}
