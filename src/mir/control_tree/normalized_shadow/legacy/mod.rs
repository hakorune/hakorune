//! Phase 123-128 legacy lowering path (kept for compatibility)

use crate::mir::control_tree::normalized_shadow::env_layout::EnvLayout;
use crate::mir::control_tree::step_tree::StepTree;
use crate::mir::control_tree::step_tree_contract_box::StepTreeContract;
use crate::mir::join_ir::lowering::carrier_info::JoinFragmentMeta;
use crate::mir::join_ir::{CompareOp, JoinFuncId, JoinFunction, JoinModule};
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
    pub fn lower_return_from_tree(
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
                            Self::lower_assign_stmt(target, value_ast, body, next_value_id, env)?;
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

    /// Phase 128-130: Lower assign statement (int literal, variable, add)
    pub fn lower_assign_stmt(
        target: &Option<String>,
        value_ast: &Option<crate::mir::control_tree::step_tree::AstNodeHandle>,
        body: &mut Vec<crate::mir::join_ir::JoinInst>,
        next_value_id: &mut u32,
        env: &mut BTreeMap<String, ValueId>,
    ) -> Result<(), String> {
        use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
        use crate::mir::join_ir::lowering::error_tags;
        use crate::mir::join_ir::{BinOpKind, ConstValue, JoinInst, MirLikeInst};

        // Check target
        let target_name = target.as_ref().ok_or_else(|| {
            "[phase128/assign/target] Assign target must be a variable".to_string()
        })?;

        // Check value_ast
        let value_ast = value_ast
            .as_ref()
            .ok_or_else(|| "[phase128/assign/value] Assign value AST is missing".to_string())?;

        // Parse value - Phase 128: int literal, Phase 130 P1: variable, Phase 130 P2: add
        match value_ast.0.as_ref() {
            // Phase 128: Integer literal
            ASTNode::Literal {
                value: LiteralValue::Integer(i),
                ..
            } => {
                // Generate Const instruction
                let dst_vid = ValueId(*next_value_id);
                *next_value_id += 1;

                body.push(JoinInst::Compute(MirLikeInst::Const {
                    dst: dst_vid,
                    value: ConstValue::Integer(*i),
                }));

                // Update env
                env.insert(target_name.clone(), dst_vid);

                Ok(())
            }
            // Phase 130 P1: Variable (x = y)
            ASTNode::Variable { name, .. } => {
                // RHS must resolve from env (writes or inputs)
                let src_vid = env.get(name).copied().ok_or_else(|| {
                    error_tags::freeze_with_hint(
                        "phase130/assign/var/rhs_missing",
                        &format!("RHS variable '{name}' not found in env"),
                        "ensure the variable is defined before assignment (in writes or inputs)",
                    )
                })?;

                // Phase 130 P1: Variable assignment in Normalized IR
                // In the continuation-passing style, we just update the env map.
                // The actual SSA phi/merge happens at function boundaries via env args.
                // No instruction emission needed here.
                env.insert(target_name.clone(), src_vid);

                Ok(())
            }
            // Phase 130 P2: BinaryOp (x = x + <int literal> only)
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                // Phase 130 P2 contract: only Add with specific shape
                if !matches!(operator, BinaryOperator::Add) {
                    return Err(error_tags::freeze_with_hint(
                        "phase130/assign/add/unsupported_op",
                        &format!("Phase 130 only supports Add operator, got {:?}", operator),
                        "use x = x + <literal> pattern or wait for future phases",
                    ));
                }

                // Left must be variable
                let lhs_var = match &**left {
                    ASTNode::Variable { name, .. } => name.clone(),
                    _ => {
                        return Err(error_tags::freeze_with_hint(
                            "phase130/assign/add/lhs_not_var",
                            "Phase 130 Add: LHS must be a variable",
                            "use pattern x = x + <literal>",
                        ));
                    }
                };

                // Right must be integer literal
                let rhs_int = match &**right {
                    ASTNode::Literal {
                        value: LiteralValue::Integer(i),
                        ..
                    } => *i,
                    _ => {
                        return Err(error_tags::freeze_with_hint(
                            "phase130/assign/add/rhs_not_int_literal",
                            "Phase 130 Add: RHS must be integer literal",
                            "use pattern x = x + <literal>",
                        ));
                    }
                };

                // Phase 130 P2 contract: dst must equal lhs (x = x + 3, not y = x + 3)
                if target_name != &lhs_var {
                    return Err(error_tags::freeze_with_hint(
                        "phase130/assign/add/dst_neq_lhs",
                        &format!(
                            "Phase 130 Add: dst '{}' must equal lhs '{}' (x = x + 3 pattern)",
                            target_name, lhs_var
                        ),
                        "use pattern x = x + <literal> where dst == lhs",
                    ));
                }

                // Load lhs from env
                let lhs_vid = env.get(&lhs_var).copied().ok_or_else(|| {
                    error_tags::freeze_with_hint(
                        "phase130/assign/add/lhs_missing",
                        &format!("Add LHS variable '{}' not found in env", lhs_var),
                        "ensure the variable is defined before the add operation",
                    )
                })?;

                // Create constant for RHS
                let rhs_vid = ValueId(*next_value_id);
                *next_value_id += 1;
                body.push(JoinInst::Compute(MirLikeInst::Const {
                    dst: rhs_vid,
                    value: ConstValue::Integer(rhs_int),
                }));

                // Generate BinOp Add
                let result_vid = ValueId(*next_value_id);
                *next_value_id += 1;
                body.push(JoinInst::Compute(MirLikeInst::BinOp {
                    dst: result_vid,
                    op: BinOpKind::Add,
                    lhs: lhs_vid,
                    rhs: rhs_vid,
                }));

                // Update env
                env.insert(target_name.clone(), result_vid);

                Ok(())
            }
            _ => {
                // Out of scope
                Err(format!(
                    "[phase130/assign/unsupported] Phase 130 supports: int literal, variable, or x = x + <int literal>  Hint: Use supported pattern or wait for future phases"
                ))
            }
        }
    }

    /// Phase 123-128 P3: Lower If node with minimal compare
    pub fn lower_if_node(
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
            let (_lhs_var, op, rhs_literal) = Self::parse_minimal_compare(ast)?;

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

    /// Parse minimal binary comparison: Variable op Integer
    ///
    /// Returns: (variable_name, compare_op, integer_value)
    pub fn parse_minimal_compare(
        ast: &crate::ast::ASTNode,
    ) -> Result<(String, CompareOp, i64), String> {
        use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

        match ast {
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                // Phase 123: Only support Variable on left, Integer literal on right
                let var_name = match &**left {
                    ASTNode::Variable { name, .. } => name.clone(),
                    _ => {
                        return Err(format!(
                            "[phase123/if/compare_lhs_unsupported] Phase 123 only supports Variable on left side of comparison. Hint: Use simple variable comparison or wait for Phase 124"
                        ));
                    }
                };

                let int_value = match &**right {
                    ASTNode::Literal {
                        value: LiteralValue::Integer(i),
                        ..
                    } => *i,
                    _ => {
                        return Err(format!(
                            "[phase123/if/compare_rhs_unsupported] Phase 123 only supports Integer literal on right side of comparison. Hint: Use integer literal or wait for Phase 124"
                        ));
                    }
                };

                let compare_op = match operator {
                    BinaryOperator::Equal => CompareOp::Eq,
                    BinaryOperator::NotEqual => CompareOp::Ne,
                    BinaryOperator::Less => CompareOp::Lt,
                    BinaryOperator::LessEqual => CompareOp::Le,
                    BinaryOperator::Greater => CompareOp::Gt,
                    BinaryOperator::GreaterEqual => CompareOp::Ge,
                    _ => {
                        return Err(format!(
                            "[phase123/if/compare_op_unsupported] Phase 123 only supports comparison operators (==, !=, <, <=, >, >=). Hint: Use comparison operator or wait for Phase 124"
                        ));
                    }
                };

                Ok((var_name, compare_op, int_value))
            }
            _ => Err(format!(
                "[phase123/if/cond_unsupported] Phase 123 only supports binary comparisons. Hint: Use simple comparison (var == literal) or wait for Phase 124"
            )),
        }
    }

    /// Verify branch contains only Return(Integer literal)
    pub fn verify_branch_is_return_literal(
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
    pub fn lower_return_value(
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
