//! Phase 123-128 legacy lowering path (kept for compatibility)

use crate::mir::control_tree::normalized_shadow::env_layout::EnvLayout;
use crate::mir::control_tree::normalized_shadow::support::expr_lowering;
use crate::mir::control_tree::step_tree::StepTree;
use crate::mir::control_tree::step_tree_contract_box::StepTreeContract;
use crate::mir::join_ir::lowering::carrier_info::JoinFragmentMeta;
use crate::mir::join_ir::{JoinFuncId, JoinFunction, JoinModule};
use crate::mir::ValueId;
use std::collections::BTreeMap;

pub struct LegacyLowerer;

impl LegacyLowerer {
    /// Lower if-only StepTree to Normalized JoinModule (Phase 123-128 legacy path)
    ///
    /// Returns Ok(None) when unsupported patterns are encountered.
    pub fn lower_if_only_to_normalized(
        step_tree: &StepTree,
        env_layout: &EnvLayout,
    ) -> Result<Option<(JoinModule, JoinFragmentMeta)>, String> {
        let main_func_id = JoinFuncId::new(0);

        // Phase 125 P2: writes 用の ValueId 生成
        let mut next_value_id = 1;
        let writes_params: Vec<ValueId> = env_layout
            .writes
            .iter()
            .map(|_| {
                let vid = ValueId(next_value_id);
                next_value_id += 1;
                vid
            })
            .collect();

        // Phase 129-B: inputs 用の ValueId 生成（writes + inputs が env params のSSOT）
        let inputs_params: Vec<ValueId> = env_layout
            .inputs
            .iter()
            .map(|_| {
                let vid = ValueId(next_value_id);
                next_value_id += 1;
                vid
            })
            .collect();

        // Phase 125 P2: env マッピング（writes + inputs）
        let mut env: BTreeMap<String, ValueId> = BTreeMap::new();
        for (name, vid) in env_layout.writes.iter().zip(writes_params.iter()) {
            env.insert(name.clone(), *vid);
        }
        for (name, vid) in env_layout.inputs.iter().zip(inputs_params.iter()) {
            env.insert(name.clone(), *vid);
        }

        // Phase 129-B: 関数パラメータは writes + inputs（env params SSOT）
        let mut env_params = writes_params;
        env_params.extend(inputs_params);

        // main 関数生成
        let mut main_func = JoinFunction::new(main_func_id, "main".to_string(), env_params.clone());

        // Phase 123-128: Return node lowering
        // If Phase 123-128 patterns are not supported, return Ok(None)
        match Self::lower_return_from_tree(
            &step_tree.root,
            &mut main_func.body,
            &mut next_value_id,
            &mut env,
            &step_tree.contract,
        ) {
            Ok(()) => {
                // Success - continue
            }
            Err(msg)
                if msg.starts_with("[phase123/")
                    || msg.starts_with("[phase124/")
                    || msg.starts_with("[phase125/")
                    || msg.starts_with("[phase128/") =>
            {
                // Phase 123-128 limitation - out of scope
                return Ok(None);
            }
            Err(msg) => {
                // Real error - propagate
                return Err(msg);
            }
        }

        // JoinModule 構築
        let mut module = JoinModule::new();
        module.add_function(main_func);
        module.entry = Some(main_func_id);
        module.mark_normalized();

        // JoinFragmentMeta 生成（最小）
        let meta = JoinFragmentMeta::empty();

        Ok(Some((module, meta)))
    }

    /// Phase 123-128 P1-P3: Lower node from StepTree
    fn lower_return_from_tree(
        node: &crate::mir::control_tree::step_tree::StepNode,
        body: &mut Vec<crate::mir::join_ir::JoinInst>,
        next_value_id: &mut u32,
        env: &mut BTreeMap<String, ValueId>,
        contract: &StepTreeContract,
    ) -> Result<(), String> {
        use crate::mir::control_tree::step_tree::{StepNode, StepStmtKind};
        use crate::mir::join_ir::JoinInst;

        match node {
            StepNode::Block(nodes) => {
                // Process nodes in order
                for n in nodes {
                    match n {
                        StepNode::Stmt {
                            kind: StepStmtKind::Assign { target, value_ast },
                            ..
                        } => {
                            // Phase 128: Process assign statement
                            expr_lowering::lower_assign_stmt(
                                target,
                                value_ast,
                                body,
                                next_value_id,
                                env,
                            )?;
                            // Continue to next node
                        }
                        StepNode::Stmt {
                            kind: StepStmtKind::Return { value_ast },
                            ..
                        } => {
                            return Self::lower_return_value(
                                value_ast,
                                body,
                                next_value_id,
                                env,
                                contract,
                            );
                        }
                        StepNode::If { .. } => {
                            // Phase 123 P3: Lower If node
                            return Self::lower_if_node(n, body, next_value_id, env, contract);
                        }
                        _ => {
                            // Other nodes not yet supported
                        }
                    }
                }
                // No return found - default to void
                body.push(JoinInst::Ret { value: None });
                Ok(())
            }
            StepNode::Stmt {
                kind: StepStmtKind::Return { value_ast },
                ..
            } => Self::lower_return_value(value_ast, body, next_value_id, env, contract),
            StepNode::If { .. } => {
                // Phase 123 P3: Lower If node
                Self::lower_if_node(node, body, next_value_id, env, contract)
            }
            _ => {
                // No return in tree - default to void
                body.push(JoinInst::Ret { value: None });
                Ok(())
            }
        }
    }

    /// Phase 123-128 P3: Lower If node with minimal compare
    fn lower_if_node(
        node: &crate::mir::control_tree::step_tree::StepNode,
        body: &mut Vec<crate::mir::join_ir::JoinInst>,
        next_value_id: &mut u32,
        env: &mut BTreeMap<String, ValueId>,
        contract: &StepTreeContract,
    ) -> Result<(), String> {
        use crate::mir::control_tree::step_tree::StepNode;
        use crate::mir::join_ir::{ConstValue, JoinInst, MirLikeInst};

        if let StepNode::If {
            cond_ast,
            then_branch,
            else_branch,
            ..
        } = node
        {
            let ast = &cond_ast.0;

            // Phase 123 P3: Parse minimal binary comparison only
            let (_lhs_var, op, rhs_literal) = expr_lowering::parse_minimal_compare(ast)?;

            // Generate Compare instruction
            // 1. Load/create lhs variable (for now, assume it's a parameter)
            // For Phase 123 minimal: we'll just create a load instruction placeholder
            // This is a simplification - real implementation would need variable resolution
            let lhs_vid = ValueId(*next_value_id);
            *next_value_id += 1;

            // For now, emit a const for the variable (placeholder)
            // Real implementation in Phase 124 will use reads facts
            body.push(JoinInst::Compute(MirLikeInst::Const {
                dst: lhs_vid,
                value: ConstValue::Integer(0), // Placeholder
            }));

            // 2. Create constant for rhs literal
            let rhs_vid = ValueId(*next_value_id);
            *next_value_id += 1;
            body.push(JoinInst::Compute(MirLikeInst::Const {
                dst: rhs_vid,
                value: ConstValue::Integer(rhs_literal),
            }));

            // 3. Generate Compare instruction
            let cond_vid = ValueId(*next_value_id);
            *next_value_id += 1;
            body.push(JoinInst::Compute(MirLikeInst::Compare {
                dst: cond_vid,
                op,
                lhs: lhs_vid,
                rhs: rhs_vid,
            }));

            // Phase 123 P3: Verify then/else branches contain only Return(Integer literal)
            Self::verify_branch_is_return_literal(then_branch)?;
            if let Some(else_br) = else_branch {
                Self::verify_branch_is_return_literal(else_br)?;
            }

            // For Phase 123-124, we generate a simplified structure:
            // The actual branching logic will be added in future phases
            // For now, just emit the then branch return
            Self::lower_return_from_tree(then_branch, body, next_value_id, env, contract)?;

            Ok(())
        } else {
            Err("[phase123/if/internal] Expected If node".to_string())
        }
    }

    /// Verify branch contains only Return(Integer literal)
    fn verify_branch_is_return_literal(
        branch: &crate::mir::control_tree::step_tree::StepNode,
    ) -> Result<(), String> {
        use crate::ast::{ASTNode, LiteralValue};
        use crate::mir::control_tree::step_tree::{StepNode, StepStmtKind};

        match branch {
            StepNode::Stmt {
                kind: StepStmtKind::Return { value_ast },
                ..
            } => {
                if let Some(ast_handle) = value_ast {
                    let ast = &ast_handle.0;
                    if let ASTNode::Literal {
                        value: LiteralValue::Integer(_),
                        ..
                    } = &**ast
                    {
                        Ok(())
                    } else {
                        Err(format!(
                            "[phase123/if/branch_return_not_int_literal] Phase 123 only supports Return(Integer literal) in then/else branches. Hint: Return integer literal only or wait for Phase 124"
                        ))
                    }
                } else {
                    Err(format!(
                        "[phase123/if/branch_return_void] Phase 123 requires Return(Integer literal) in branches, not void return. Hint: Return integer literal or wait for Phase 124"
                    ))
                }
            }
            StepNode::Block(nodes) => {
                // Check first node only
                if nodes.is_empty() {
                    return Err(format!(
                        "[phase123/if/branch_empty] Phase 123 requires Return(Integer literal) in branches. Hint: Add return statement"
                    ));
                }
                Self::verify_branch_is_return_literal(&nodes[0])
            }
            _ => Err(format!(
                "[phase123/if/branch_not_return] Phase 123 only supports Return(Integer literal) in then/else branches. Hint: Use return statement with integer literal"
            )),
        }
    }

    /// Phase 123-125 P1-P2-P3-P4: Lower return value
    fn lower_return_value(
        value_ast: &Option<crate::mir::control_tree::step_tree::AstNodeHandle>,
        body: &mut Vec<crate::mir::join_ir::JoinInst>,
        next_value_id: &mut u32,
        env: &BTreeMap<String, ValueId>,
        contract: &StepTreeContract,
    ) -> Result<(), String> {
        use crate::ast::{ASTNode, LiteralValue};
        use crate::mir::join_ir::{ConstValue, JoinInst, MirLikeInst};

        match value_ast {
            None => {
                body.push(JoinInst::Ret { value: None });
                Ok(())
            }
            Some(ast_handle) => {
                let ast = &ast_handle.0;
                match &**ast {
                    ASTNode::Literal { value, .. } => match value {
                        LiteralValue::Integer(i) => {
                            // Phase 123 P1: Integer literal → Const + Ret(Some(vid))
                            let const_vid = ValueId(*next_value_id);
                            *next_value_id += 1;

                            // Generate Const instruction (wrapped in Compute)
                            body.push(JoinInst::Compute(MirLikeInst::Const {
                                dst: const_vid,
                                value: ConstValue::Integer(*i),
                            }));

                            // Generate Ret instruction
                            body.push(JoinInst::Ret {
                                value: Some(const_vid),
                            });

                            Ok(())
                        }
                        _ => {
                            // Phase 123: Other literals not supported
                            Err(format!(
                                "[phase123/return/literal_unsupported] Phase 123 only supports integer literals. Hint: Use integer literal or wait for Phase 124"
                            ))
                        }
                    },
                    ASTNode::Variable { name, .. } => {
                        // Phase 124-125 P3-P4: Variable return support (dev-only)
                        // Check if variable is in env (writes + inputs)
                        if let Some(&vid) = env.get(name) {
                            // Phase 125 P4: Variable found in env (writes or inputs) - return it
                            body.push(JoinInst::Ret { value: Some(vid) });
                            Ok(())
                        } else {
                            // Phase 125 P4: Variable not in env - Fail-Fast with hint
                            // Check if variable is in reads (potential input)
                            let in_reads = contract.reads.contains(name);
                            let in_writes = contract.writes.contains(name);

                            let hint = if in_reads && !in_writes {
                                // Variable is read but not available as input
                                format!(
                                    "Variable '{}' is read but not available from outer scope. \
                                     Hint: Pass as function parameter, add to pinned capture, or define before if",
                                    name
                                )
                            } else if in_writes {
                                // Variable is written but not captured in env
                                format!(
                                    "Variable '{}' is written but not captured in env. \
                                     Hint: Ensure writes are captured in env layout (Phase 125)",
                                    name
                                )
                            } else {
                                // Not in reads or writes
                                format!(
                                    "Variable '{}' not found in env. \
                                     Hint: Add to reads/writes contract or define before return",
                                    name
                                )
                            };

                            Err(format!(
                                "[phase125/return/variable_missing] Variable '{}' not found in env. {}",
                                name, hint
                            ))
                        }
                    }
                    _ => {
                        // Phase 123: Other expressions not supported
                        Err(format!(
                            "[phase123/return/expr_unsupported] Phase 123 only supports integer literals or variables. Hint: Use simple return or wait for future phases"
                        ))
                    }
                }
            }
        }
    }
}
