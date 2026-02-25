//! Phase 129-B: if-as-last lowering with join_k (dev-only)

use super::env_layout::EnvLayout;
use super::common::normalized_helpers::NormalizedHelperBox;
use super::legacy::LegacyLowerer;
use crate::mir::control_tree::step_tree::{StepNode, StepStmtKind, StepTree};
use crate::mir::join_ir::lowering::carrier_info::JoinFragmentMeta;
use crate::mir::join_ir::lowering::error_tags;
use crate::mir::join_ir::{ConstValue, JoinFunction, JoinFuncId, JoinInst, JoinModule, MirLikeInst};

pub struct IfAsLastJoinKLowererBox;

impl IfAsLastJoinKLowererBox {
    /// Phase 129-B: If-as-last shape detection (no post-if)
    pub fn expects_join_k_as_last(step_tree: &StepTree) -> bool {
        match &step_tree.root {
            StepNode::If { .. } => true,
            StepNode::Block(nodes) => matches!(nodes.last(), Some(StepNode::If { .. })),
            _ => false,
        }
    }

    /// Lower if-only StepTree to Normalized JoinModule using join_k tailcalls.
    ///
    /// Scope: "if-as-last" only (no post-if). If it doesn't match, return Ok(None).
    pub fn lower(
        step_tree: &StepTree,
        env_layout: &EnvLayout,
    ) -> Result<Option<(JoinModule, JoinFragmentMeta)>, String> {
        let (prefix_nodes, if_node) = match &step_tree.root {
            StepNode::If { .. } => (&[][..], &step_tree.root),
            StepNode::Block(nodes) => {
                let last = nodes.last();
                if matches!(last, Some(StepNode::If { .. })) {
                    (&nodes[..nodes.len() - 1], nodes.last().unwrap())
                } else {
                    return Ok(None);
                }
            }
            _ => return Ok(None),
        };

        let if_node = match if_node {
            StepNode::If { .. } => if_node,
            _ => return Ok(None),
        };

        let env_fields = env_layout.env_fields();
        // Phase 143 fix: env params must be in Param region (100+) per JoinValueSpace contract.
        // All functions share the same params (env passing via continuation).
        let (main_params, mut next_value_id) = NormalizedHelperBox::alloc_env_params_param_region(&env_fields);

        // IDs (stable, dev-only)
        let main_id = JoinFuncId::new(0);
        let k_then_id = JoinFuncId::new(1);
        let k_else_id = JoinFuncId::new(2);
        let join_k_id = JoinFuncId::new(3);

        // main(env)
        // main_params allocated above in Param region. Clone for reuse.
        let mut env_main = NormalizedHelperBox::build_env_map(&env_fields, &main_params);
        let mut main_func = JoinFunction::new(main_id, "main".to_string(), main_params.clone());

        // Lower prefix (pre-if) statements into main
        for n in prefix_nodes {
            match n {
                StepNode::Stmt { kind, .. } => match kind {
                    StepStmtKind::Assign { target, value_ast } => {
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

        // Extract return variable and branch bodies.
        fn split_branch_for_as_last(
            branch: &StepNode,
        ) -> Result<(&[StepNode], &crate::mir::control_tree::step_tree::AstNodeHandle), String> {
            use crate::mir::control_tree::step_tree::StepNode;
            use crate::mir::control_tree::step_tree::StepStmtKind;
            use crate::mir::join_ir::lowering::error_tags;

            match branch {
                StepNode::Stmt { kind, .. } => match kind {
                    StepStmtKind::Return { value_ast } => {
                        let ast_handle = value_ast.as_ref().ok_or_else(|| {
                            error_tags::freeze_with_hint(
                                "phase129/join_k/branch_return_void",
                                "branch return must return a variable (not void)",
                                "use `return x` in both then/else branches",
                            )
                        })?;
                        Ok((&[][..], ast_handle))
                    }
                    _ => Err(error_tags::freeze_with_hint(
                        "phase129/join_k/branch_not_return",
                        "branch must end with return",
                        "use `return x` in both then/else branches",
                    )),
                },
                StepNode::Block(nodes) => {
                    let last = nodes.last().ok_or_else(|| {
                        error_tags::freeze_with_hint(
                            "phase129/join_k/branch_empty",
                            "branch is empty",
                            "add `return x` as the last statement of the branch",
                        )
                    })?;
                    match last {
                        StepNode::Stmt { kind, .. } => match kind {
                            StepStmtKind::Return { value_ast } => {
                                let ast_handle = value_ast.as_ref().ok_or_else(|| {
                                    error_tags::freeze_with_hint(
                                        "phase129/join_k/branch_return_void",
                                        "branch return must return a variable (not void)",
                                        "use `return x` in both then/else branches",
                                    )
                                })?;
                                Ok((&nodes[..nodes.len() - 1], ast_handle))
                            }
                            _ => Err(error_tags::freeze_with_hint(
                                "phase129/join_k/branch_not_return",
                                "branch must end with return",
                                "add `return x` as the last statement of the branch",
                            )),
                        },
                        _ => Err(error_tags::freeze_with_hint(
                            "phase129/join_k/branch_last_not_stmt",
                            "branch last node must be a statement return",
                            "ensure the branch ends with `return x`",
                        )),
                    }
                }
                _ => Err(error_tags::freeze_with_hint(
                    "phase129/join_k/branch_node_unsupported",
                    "unsupported branch node",
                    "Phase 129-B supports only Block/Return branches",
                )),
            }
        }

        fn extract_return_var_name(ast_handle: &crate::mir::control_tree::step_tree::AstNodeHandle) -> Result<String, String> {
            use crate::ast::ASTNode;
            use crate::mir::join_ir::lowering::error_tags;
            match ast_handle.0.as_ref() {
                ASTNode::Variable { name, .. } => Ok(name.clone()),
                _ => Err(error_tags::freeze_with_hint(
                    "phase129/join_k/return_expr_unsupported",
                    "branch return expression must be a variable",
                    "use `return x` (variable) in both then/else branches",
                )),
            }
        }

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
            None => return Ok(None),
        };

        let (then_prefix, then_ret_ast) = match split_branch_for_as_last(then_branch) {
            Ok(v) => v,
            Err(_msg) => return Ok(None),
        };
        let (else_prefix, else_ret_ast) = match split_branch_for_as_last(else_branch) {
            Ok(v) => v,
            Err(_msg) => return Ok(None),
        };

        let then_ret_var = match extract_return_var_name(then_ret_ast) {
            Ok(v) => v,
            Err(_msg) => return Ok(None),
        };
        let else_ret_var = match extract_return_var_name(else_ret_ast) {
            Ok(v) => v,
            Err(_msg) => return Ok(None),
        };

        if then_ret_var != else_ret_var {
            return Ok(None);
        }

        let ret_var = then_ret_var;
        if !env_layout.writes.iter().any(|w| w == &ret_var) {
            return Ok(None);
        }

        // join_k(env_phi): return env_phi[ret_var]
        // Phase 143 fix: reuse Param region IDs for all functions
        let join_k_params = main_params.clone();
        let env_join_k = NormalizedHelperBox::build_env_map(&env_fields, &join_k_params);
        let ret_vid = env_join_k.get(&ret_var).copied().ok_or_else(|| {
            error_tags::freeze_with_hint(
                "phase129/join_k/ret_vid_missing",
                "return variable not found in join_k env",
                "ensure env layout includes the return variable in writes",
            )
        })?;
        let mut join_k_func = JoinFunction::new(join_k_id, "join_k".to_string(), join_k_params);
        join_k_func.body.push(JoinInst::Ret { value: Some(ret_vid) });

        // k_then(env_in): <prefix> ; tailcall join_k(env_out)
        // Phase 143 fix: reuse Param region IDs for all functions
        let then_params = main_params.clone();
        let mut env_then = NormalizedHelperBox::build_env_map(&env_fields, &then_params);
        let mut then_func = JoinFunction::new(k_then_id, "k_then".to_string(), then_params);
        for n in then_prefix {
            match n {
                StepNode::Stmt { kind, .. } => match kind {
                    StepStmtKind::Assign { target, value_ast } => {
                        if LegacyLowerer::lower_assign_stmt(
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
                        return Ok(None);
                    }
                },
                _ => {
                    return Ok(None);
                }
            }
        }
        let then_args = NormalizedHelperBox::collect_env_args(&env_fields, &env_then)
            .map_err(|e| error_tags::freeze_with_hint(
                "phase129/join_k/env_missing",
                &e,
                "ensure env layout and env map are built from the same SSOT field list",
            ))?;
        then_func.body.push(JoinInst::Call {
            func: join_k_id,
            args: then_args,
            k_next: None,
            dst: None,
        });

        // k_else(env_in): <prefix> ; tailcall join_k(env_out)
        // Phase 143 fix: reuse Param region IDs for all functions
        let else_params = main_params.clone();
        let mut env_else = NormalizedHelperBox::build_env_map(&env_fields, &else_params);
        let mut else_func = JoinFunction::new(k_else_id, "k_else".to_string(), else_params);
        for n in else_prefix {
            match n {
                StepNode::Stmt { kind, .. } => match kind {
                    StepStmtKind::Assign { target, value_ast } => {
                        if LegacyLowerer::lower_assign_stmt(
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
                        return Ok(None);
                    }
                },
                _ => {
                    return Ok(None);
                }
            }
        }
        let else_args = NormalizedHelperBox::collect_env_args(&env_fields, &env_else)
            .map_err(|e| error_tags::freeze_with_hint(
                "phase129/join_k/env_missing",
                &e,
                "ensure env layout and env map are built from the same SSOT field list",
            ))?;
        else_func.body.push(JoinInst::Call {
            func: join_k_id,
            args: else_args,
            k_next: None,
            dst: None,
        });

        // main: cond compare + conditional jump to k_then, else to k_else
        let (lhs_var, op, rhs_literal) = LegacyLowerer::parse_minimal_compare(&cond_ast.0)?;
        let lhs_vid = env_main.get(&lhs_var).copied().ok_or_else(|| {
            error_tags::freeze_with_hint(
                "phase129/join_k/cond_lhs_missing",
                &format!("condition lhs var '{lhs_var}' not found in env"),
                "ensure the if condition uses a variable from writes or captured inputs",
            )
        })?;
        let rhs_vid = NormalizedHelperBox::alloc_value_id(&mut next_value_id);
        main_func.body.push(JoinInst::Compute(MirLikeInst::Const {
            dst: rhs_vid,
            value: ConstValue::Integer(rhs_literal),
        }));
        let cond_vid = NormalizedHelperBox::alloc_value_id(&mut next_value_id);
        main_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cond_vid,
            op,
            lhs: lhs_vid,
            rhs: rhs_vid,
        }));

        let main_args = NormalizedHelperBox::collect_env_args(&env_fields, &env_main)
            .map_err(|e| error_tags::freeze_with_hint(
                "phase129/join_k/env_missing",
                &e,
                "ensure env layout and env map are built from the same SSOT field list",
            ))?;
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
        module.entry = Some(main_id);
        module.mark_normalized();

        Ok(Some((module, JoinFragmentMeta::empty())))
    }
}
