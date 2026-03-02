use super::CoreEffectPlan;
use crate::mir::builder::calls::extern_calls;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, BinaryOp, CompareOp, ConstValue, Effect, EffectMask, MirType, ValueId};
use std::collections::{BTreeMap, HashSet};

// ============================================================================
// Phase 286 P2.8: Normalizer Hygiene Helpers
// ============================================================================

/// Standard 5-block layout for simple loops (Pattern1/4/8/9)
///
/// CFG: preheader → header → body → step → header (back-edge)
///                      ↓
///                   after
#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct LoopBlocksStandard5 {
    pub(in crate::mir::builder) preheader_bb: BasicBlockId,
    pub(in crate::mir::builder) header_bb: BasicBlockId,
    pub(in crate::mir::builder) body_bb: BasicBlockId,
    pub(in crate::mir::builder) step_bb: BasicBlockId,
    pub(in crate::mir::builder) after_bb: BasicBlockId,
}

impl LoopBlocksStandard5 {
    /// Allocate 5 blocks for a standard loop
    pub(in crate::mir::builder) fn allocate(builder: &mut MirBuilder) -> Result<Self, String> {
        let preheader_bb = builder
            .current_block
            .ok_or_else(|| "[normalizer] No current block for loop entry".to_string())?;
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let step_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();
        Ok(Self {
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
        })
    }
}

/// Extended 8-block layout for if-phi loops (Pattern3)
///
/// CFG: preheader → header → body → then/else → merge → step → header
///                      ↓
///                   after
#[derive(Debug, Clone, Copy)]
pub(super) struct LoopBlocksWithIfPhi {
    pub(super) preheader_bb: BasicBlockId,
    pub(super) header_bb: BasicBlockId,
    pub(super) body_bb: BasicBlockId,
    pub(super) then_bb: BasicBlockId,
    pub(super) else_bb: BasicBlockId,
    pub(super) merge_bb: BasicBlockId,
    pub(super) step_bb: BasicBlockId,
    pub(super) after_bb: BasicBlockId,
}

impl LoopBlocksWithIfPhi {
    /// Allocate 8 blocks for an if-phi loop
    pub(super) fn allocate(builder: &mut MirBuilder) -> Result<Self, String> {
        let preheader_bb = builder
            .current_block
            .ok_or_else(|| "[normalizer] No current block for loop entry".to_string())?;
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let then_bb = builder.next_block_id();
        let else_bb = builder.next_block_id();
        let merge_bb = builder.next_block_id();
        let step_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();
        Ok(Self {
            preheader_bb,
            header_bb,
            body_bb,
            then_bb,
            else_bb,
            merge_bb,
            step_bb,
            after_bb,
        })
    }
}

/// Create phi_bindings map from variable name-ValueId pairs
///
/// phi_bindings are used to override variable_map lookups during AST lowering,
/// ensuring loop variables reference PHI destinations instead of initial values.
pub(in crate::mir::builder) fn create_phi_bindings(
    bindings: &[(&str, ValueId)],
) -> BTreeMap<String, ValueId> {
    bindings
        .iter()
        .map(|(name, id)| (name.to_string(), *id))
        .collect()
}

impl super::PlanNormalizer {
    /// Helper: Lower Compare AST to (lhs ValueId, op, rhs ValueId, const_effects)
    pub(in crate::mir::builder) fn lower_compare_ast(
        ast: &crate::ast::ASTNode,
        builder: &mut MirBuilder,
        phi_bindings: &BTreeMap<String, ValueId>,
    ) -> Result<(ValueId, CompareOp, ValueId, Vec<CoreEffectPlan>), String> {
        // Ensure current_span points at the comparison expression (not the last lowered operand).
        builder.metadata_ctx.set_current_span(ast.span());
        use crate::ast::{ASTNode, BinaryOperator};

        match ast {
            ASTNode::BinaryOp { operator, left, right, .. } => {
                let op = match operator {
                    BinaryOperator::Less => CompareOp::Lt,
                    BinaryOperator::LessEqual => CompareOp::Le,
                    BinaryOperator::Greater => CompareOp::Gt,
                    BinaryOperator::GreaterEqual => CompareOp::Ge,
                    BinaryOperator::Equal => CompareOp::Eq,
                    BinaryOperator::NotEqual => CompareOp::Ne,
                    _ => {
                        return Err(format!(
                            "[normalizer] Unsupported compare operator: {:?}",
                            operator
                        ))
                    }
                };

                let (lhs, mut lhs_consts) = Self::lower_value_ast(left, builder, phi_bindings)?;
                let (rhs, rhs_consts) = Self::lower_value_ast(right, builder, phi_bindings)?;

                lhs_consts.extend(rhs_consts);

                // Restore the comparison span (callers may emit Compare immediately after this).
                builder.metadata_ctx.set_current_span(ast.span());
                Ok((lhs, op, rhs, lhs_consts))
            }
            _ => Err(format!(
                "[normalizer] Expected BinaryOp for compare, got: {:?}",
                ast
            )),
        }
    }

    /// Helper: Lower BinOp AST to (lhs ValueId, op, rhs ValueId, const_effects)
    #[track_caller]
    pub(in crate::mir::builder) fn lower_binop_ast(
        ast: &crate::ast::ASTNode,
        builder: &mut MirBuilder,
        phi_bindings: &BTreeMap<String, ValueId>,
    ) -> Result<(ValueId, BinaryOp, ValueId, Vec<CoreEffectPlan>), String> {
        use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

        match ast {
            ASTNode::BinaryOp { operator, left, right, .. } => {
                let op = match operator {
                    BinaryOperator::Add => BinaryOp::Add,
                    BinaryOperator::Subtract => BinaryOp::Sub,
                    BinaryOperator::Multiply => BinaryOp::Mul,
                    BinaryOperator::Divide => BinaryOp::Div,
                    BinaryOperator::Modulo => BinaryOp::Mod,
                    _ => {
                        return Err(format!(
                            "[normalizer] Unsupported binary operator: {:?}",
                            operator
                        ))
                    }
                };

                let (lhs, mut lhs_consts) = Self::lower_value_ast(left, builder, phi_bindings)?;
                let (rhs, rhs_consts) = Self::lower_value_ast(right, builder, phi_bindings)?;

                lhs_consts.extend(rhs_consts);

                if crate::config::env::joinir_dev::strict_planner_required_debug_enabled()
                    && op == BinaryOp::Add
                {
                    let lit3_lhs = matches!(
                        left.as_ref(),
                        ASTNode::Literal {
                            value: LiteralValue::Integer(3),
                            ..
                        }
                    );
                    let lit3_rhs = matches!(
                        right.as_ref(),
                        ASTNode::Literal {
                            value: LiteralValue::Integer(3),
                            ..
                        }
                    );
                    if lit3_lhs || lit3_rhs {
                        let fn_name = builder
                            .scope_ctx
                            .current_function
                            .as_ref()
                            .map(|f| f.signature.name.as_str())
                            .unwrap_or("<none>");
                        let consts_len = lhs_consts.len();
                        let rhs_const_def = if lit3_rhs {
                            lhs_consts.iter().any(|effect| {
                                matches!(
                                    effect,
                                    CoreEffectPlan::Const {
                                        dst,
                                        value: ConstValue::Integer(3)
                                    } if *dst == rhs
                                )
                            })
                        } else {
                            false
                        };
                        let lhs_const_def = if lit3_lhs {
                            lhs_consts.iter().any(|effect| {
                                matches!(
                                    effect,
                                    CoreEffectPlan::Const {
                                        dst,
                                        value: ConstValue::Integer(3)
                                    } if *dst == lhs
                                )
                            })
                        } else {
                            false
                        };
                        let side = match (lit3_lhs, lit3_rhs) {
                            (true, true) => "both",
                            (true, false) => "lhs",
                            (false, true) => "rhs",
                            (false, false) => "none",
                        };
                        let caller = std::panic::Location::caller();
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[binop/lower:lit_int3] fn={} bb={:?} lhs=%{} rhs=%{} side={} consts_len={} rhs_const_def={} lhs_const_def={} caller={}",
                            fn_name,
                            builder.current_block,
                            lhs.0,
                            rhs.0,
                            side,
                            consts_len,
                            if rhs_const_def { "yes" } else { "no" },
                            if lhs_const_def { "yes" } else { "no" },
                            caller
                        ));
                    }
                }

                Ok((lhs, op, rhs, lhs_consts))
            }
            _ => Err(format!("[normalizer] Expected BinOp, got: {:?}", ast)),
        }
    }

    /// Helper: Lower value AST to (ValueId, const_effects)
    /// Returns the ValueId and any Const instructions needed to define literals
    ///
    /// phi_bindings: PHI dst for loop variables (takes precedence over variable_map)
    #[track_caller]
    pub(in crate::mir::builder) fn lower_value_ast(
        ast: &crate::ast::ASTNode,
        builder: &mut MirBuilder,
        phi_bindings: &BTreeMap<String, ValueId>,
    ) -> Result<(ValueId, Vec<CoreEffectPlan>), String> {
        // Keep instruction spans meaningful in the plan-based lowering path too.
        // This mirrors `MirBuilder::build_expression_impl`.
        builder.metadata_ctx.set_current_span(ast.span());

        use crate::ast::{ASTNode, LiteralValue, UnaryOperator};
        use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
        use crate::mir::{BinaryOp, ConstValue, MirType};

        match ast {
            ASTNode::Variable { name, .. } => {
                if let Some(&phi_dst) = phi_bindings.get(name) {
                    return Ok((phi_dst, vec![]));
                }
                if let Some(&value_id) = builder.variable_ctx.variable_map.get(name) {
                    Ok((value_id, vec![]))
                } else {
                    Err(format!("[normalizer] Variable {} not found", name))
                }
            }
            ASTNode::Literal { value, span, .. } => {
                let value_id = builder.next_value_id();
                // Diagnostics-only: record literal origin spans only when strict/dev or debug is enabled.
                // This avoids keeping extra state in normal/release compiles.
                if crate::config::env::joinir_dev::debug_enabled()
                    || (crate::config::env::joinir_dev::strict_enabled()
                        && crate::config::env::joinir_dev::planner_required_enabled())
                {
                    builder.metadata_ctx.record_value_span(value_id, *span);
                }
                let (const_value, value_type) = match value {
                    LiteralValue::Integer(n) => (ConstValue::Integer(*n), MirType::Integer),
                    LiteralValue::String(s) => (ConstValue::String(s.clone()), MirType::String),
                    LiteralValue::Bool(b) => (ConstValue::Bool(*b), MirType::Bool),
                    LiteralValue::Null => (ConstValue::Null, MirType::Unknown),
                    LiteralValue::Void => (ConstValue::Void, MirType::Void),
                    _ => {
                        return Err(format!(
                            "[normalizer] Unsupported literal type: {:?}",
                            value
                        ))
                    }
                };

                builder.type_ctx.set_type(value_id, value_type);

                if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
                    let caller = std::panic::Location::caller();
                    let ring0 = crate::runtime::get_global_ring0();
                    let fn_name = builder
                        .scope_ctx
                        .current_function
                        .as_ref()
                        .map(|f| f.signature.name.as_str())
                        .unwrap_or("<none>");
                    let next_value_id = builder
                        .scope_ctx
                        .current_function
                        .as_ref()
                        .map(|f| f.next_value_id)
                        .unwrap_or(0);
                    let file = builder
                        .metadata_ctx
                        .current_source_file()
                        .unwrap_or_else(|| "unknown".to_string());
                    ring0.log.debug(&format!(
                        "[lit/lower:alloc] fn={} bb={:?} v=%{} lit={:?} span={} file={} next={} emit=skipped:plan_effect caller={}",
                        fn_name,
                        builder.current_block,
                        value_id.0,
                        value,
                        span.location_string(),
                        file,
                        next_value_id,
                        caller
                    ));
                }

                let const_effect = CoreEffectPlan::Const {
                    dst: value_id,
                    value: const_value,
                };

                Ok((value_id, vec![const_effect]))
            }
            ASTNode::UnaryOp {
                operator,
                operand,
                ..
            } => match operator {
                UnaryOperator::Minus => {
                    let (rhs, mut effects) = Self::lower_value_ast(operand, builder, phi_bindings)?;
                    let rhs_ty = builder
                        .type_ctx
                        .get_type(rhs)
                        .cloned()
                        .unwrap_or(MirType::Integer);
                    let (zero_val, zero_ty) = match rhs_ty {
                        MirType::Float => (ConstValue::Float(0.0), MirType::Float),
                        _ => (ConstValue::Integer(0), MirType::Integer),
                    };
                    let zero_id = builder.alloc_typed(zero_ty);
                    effects.push(CoreEffectPlan::Const {
                        dst: zero_id,
                        value: zero_val,
                    });
                    let dst = builder.alloc_typed(rhs_ty);
                    effects.push(CoreEffectPlan::BinOp {
                        dst,
                        lhs: zero_id,
                        op: BinaryOp::Sub,
                        rhs,
                    });
                    Ok((dst, effects))
                }
                UnaryOperator::BitNot => {
                    let (rhs, mut effects) = Self::lower_value_ast(operand, builder, phi_bindings)?;
                    let mask_id = builder.alloc_typed(MirType::Integer);
                    effects.push(CoreEffectPlan::Const {
                        dst: mask_id,
                        value: ConstValue::Integer(-1),
                    });
                    let dst = builder.alloc_typed(MirType::Integer);
                    effects.push(CoreEffectPlan::BinOp {
                        dst,
                        lhs: rhs,
                        op: BinaryOp::BitXor,
                        rhs: mask_id,
                    });
                    Ok((dst, effects))
                }
                UnaryOperator::Not => Err("[normalizer] Unary 'not' is condition-only".to_string()),
                UnaryOperator::Weak => Err("[normalizer] Unary 'weak' is not supported yet".to_string()),
            },
            ASTNode::MethodCall { object, method, arguments, .. } => {
                let mut arg_ids = Vec::new();
                let mut arg_effects = Vec::new();
                for arg in arguments {
                    let (arg_id, mut effects) = Self::lower_value_ast(arg, builder, phi_bindings)?;
                    arg_ids.push(arg_id);
                    arg_effects.append(&mut effects);
                }

                let result_id = builder.next_value_id();
                builder.type_ctx.set_type(result_id, MirType::Integer);

                match object.as_ref() {
                    ASTNode::Variable { name, .. } if name == "env" => {
                        let Some((iface_name, method_name, effects, returns_value)) =
                            extern_calls::get_env_method_spec("env", method)
                        else {
                            return Err(format!(
                                "[normalizer] env method not supported: {}",
                                method
                            ));
                        };
                        if !returns_value {
                            return Err(format!(
                                "[normalizer] env method used as value: {}",
                                method
                            ));
                        }
                        arg_effects.push(CoreEffectPlan::ExternCall {
                            dst: Some(result_id),
                            iface_name,
                            method_name,
                            args: arg_ids,
                            effects,
                        });
                    }
                    ASTNode::Variable { name, .. } => {
                        if let Some(&phi_dst) = phi_bindings.get(name) {
                            arg_effects.push(CoreEffectPlan::MethodCall {
                                dst: Some(result_id),
                                object: phi_dst,
                                method: method.clone(),
                                args: arg_ids,
                                effects: EffectMask::PURE.add(Effect::Io),
                            });
                        } else if let Some(&value_id) =
                            builder.variable_ctx.variable_map.get(name)
                        {
                            arg_effects.push(CoreEffectPlan::MethodCall {
                                dst: Some(result_id),
                                object: value_id,
                                method: method.clone(),
                                args: arg_ids,
                                effects: EffectMask::PURE.add(Effect::Io),
                            });
                        } else if builder.comp_ctx.user_defined_boxes.contains_key(name) {
                            let func = format!("{}.{}/{}", name, method, arguments.len());
                            arg_effects.push(CoreEffectPlan::GlobalCall {
                                dst: Some(result_id),
                                func,
                                args: arg_ids,
                            });
                        } else {
                            return Err(format!(
                                "[normalizer] Method call object {} not found",
                                name
                            ));
                        }
                    }
                    ASTNode::Literal {
                        value: LiteralValue::String(_),
                        ..
                    } => {
                        let (object_id, mut object_effects) =
                            Self::lower_value_ast(object, builder, phi_bindings)?;
                        arg_effects.append(&mut object_effects);
                        arg_effects.push(CoreEffectPlan::MethodCall {
                            dst: Some(result_id),
                            object: object_id,
                            method: method.clone(),
                            args: arg_ids,
                            effects: EffectMask::PURE.add(Effect::Io),
                        });
                    }
                    ASTNode::Me { .. } => {
                        if let Some(&object_id) = phi_bindings
                            .get("me")
                            .or_else(|| builder.variable_ctx.variable_map.get("me"))
                        {
                            arg_effects.push(CoreEffectPlan::MethodCall {
                                dst: Some(result_id),
                                object: object_id,
                                method: method.clone(),
                                args: arg_ids,
                                effects: EffectMask::PURE.add(Effect::Io),
                            });
                        } else if let Some(box_name) = builder.comp_ctx.current_static_box.clone() {
                            // Static-box helper contexts may not materialize `me`.
                            let func = format!("{}.{}/{}", box_name, method, arguments.len());
                            arg_effects.push(CoreEffectPlan::GlobalCall {
                                dst: Some(result_id),
                                func,
                                args: arg_ids,
                            });
                        } else {
                            return Err(
                                "[normalizer] me.method() without bound receiver".to_string()
                            );
                        }
                    }
                    ASTNode::This { .. } => {
                        if let Some(box_name) = builder.comp_ctx.current_static_box.clone() {
                            let func = format!("{}.{}/{}", box_name, method, arguments.len());
                            arg_effects.push(CoreEffectPlan::GlobalCall {
                                dst: Some(result_id),
                                func,
                                args: arg_ids,
                            });
                        } else if let Some(&object_id) = phi_bindings
                            .get("me")
                            .or_else(|| builder.variable_ctx.variable_map.get("me"))
                        {
                            arg_effects.push(CoreEffectPlan::MethodCall {
                                dst: Some(result_id),
                                object: object_id,
                                method: method.clone(),
                                args: arg_ids,
                                effects: EffectMask::PURE.add(Effect::Io),
                            });
                        } else {
                            return Err(
                                "[normalizer] this.method() without current_static_box".to_string()
                            );
                        }
                    }
                    _ => {
                        let (object_id, mut object_effects) =
                            Self::lower_value_ast(object, builder, phi_bindings)?;
                        arg_effects.append(&mut object_effects);
                        arg_effects.push(CoreEffectPlan::MethodCall {
                            dst: Some(result_id),
                            object: object_id,
                            method: method.clone(),
                            args: arg_ids,
                            effects: EffectMask::PURE.add(Effect::Io),
                        });
                    }
                }

                Ok((result_id, arg_effects))
            }
            ASTNode::Call {
                callee,
                arguments,
                ..
            } => {
                let (callee_id, mut effects) =
                    Self::lower_value_ast(callee, builder, phi_bindings)?;
                let mut arg_ids = Vec::new();
                for arg in arguments {
                    let (arg_id, mut arg_effects) =
                        Self::lower_value_ast(arg, builder, phi_bindings)?;
                    arg_ids.push(arg_id);
                    effects.append(&mut arg_effects);
                }

                let result_id = builder.next_value_id();
                builder.type_ctx.set_type(result_id, MirType::Unknown);
                effects.push(CoreEffectPlan::ValueCall {
                    dst: Some(result_id),
                    callee: callee_id,
                    args: arg_ids,
                });

                Ok((result_id, effects))
            }
            ASTNode::New { class, arguments, .. } => {
                let mut arg_ids = Vec::new();
                let mut effects = Vec::new();
                for arg in arguments {
                    let (arg_id, mut arg_effects) =
                        Self::lower_value_ast(arg, builder, phi_bindings)?;
                    arg_ids.push(arg_id);
                    effects.append(&mut arg_effects);
                }

                let result_id = builder.next_value_id();
                Self::record_newbox_metadata(builder, result_id, class);
                effects.push(CoreEffectPlan::NewBox {
                    dst: result_id,
                    box_type: class.clone(),
                    args: arg_ids,
                });

                Ok((result_id, effects))
            }
            ASTNode::BinaryOp { .. } => {
                let (lhs, op, rhs, mut consts) =
                    Self::lower_binop_ast(ast, builder, phi_bindings)?;
                let result_id = builder.alloc_typed(MirType::Integer);

                if crate::config::env::joinir_dev::strict_enabled()
                    && crate::config::env::joinir_dev::planner_required_enabled()
                {
                    let mut defined_values = HashSet::new();
                    for effect in &consts {
                        let def = match effect {
                            CoreEffectPlan::MethodCall { dst: Some(v), .. } => Some(*v),
                            CoreEffectPlan::GlobalCall { dst: Some(v), .. } => Some(*v),
                            CoreEffectPlan::ValueCall { dst: Some(v), .. } => Some(*v),
                            CoreEffectPlan::ExternCall { dst: Some(v), .. } => Some(*v),
                            CoreEffectPlan::NewBox { dst, .. } => Some(*dst),
                            CoreEffectPlan::BinOp { dst, .. } => Some(*dst),
                            CoreEffectPlan::Compare { dst, .. } => Some(*dst),
                            CoreEffectPlan::Select { dst, .. } => Some(*dst),
                            CoreEffectPlan::Const { dst, .. } => Some(*dst),
                            CoreEffectPlan::Copy { dst, .. } => Some(*dst),
                            _ => None,
                        };
                        if let Some(def) = def {
                            defined_values.insert(def);
                        }
                    }

                    let is_defined = |value_id: ValueId| -> bool {
                        phi_bindings.values().any(|v| *v == value_id)
                            || builder
                                .variable_ctx
                                .variable_map
                                .values()
                                .any(|v| *v == value_id)
                            || defined_values.contains(&value_id)
                    };
                    let fn_name = builder
                        .scope_ctx
                        .current_function
                        .as_ref()
                        .map(|f| f.signature.name.as_str())
                        .unwrap_or("<none>");
                    if !is_defined(lhs) {
                        return Err(format!(
                            "[freeze:contract][normalizer/binop_operand_missing_def] fn={} bb={:?} dst=%{} op={:?} operand=lhs v=%{}",
                            fn_name,
                            builder.current_block,
                            result_id.0,
                            op,
                            lhs.0
                        ));
                    }
                    if !is_defined(rhs) {
                        return Err(format!(
                            "[freeze:contract][normalizer/binop_operand_missing_def] fn={} bb={:?} dst=%{} op={:?} operand=rhs v=%{}",
                            fn_name,
                            builder.current_block,
                            result_id.0,
                            op,
                            rhs.0
                        ));
                    }
                }

                consts.push(CoreEffectPlan::BinOp {
                    dst: result_id,
                    lhs,
                    op,
                    rhs,
                });
                Ok((result_id, consts))
            }
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => {
                // Phase B2-7: BlockExpr in value-required contexts (planner-required normalizer).
                //
                // v1 safety contract: forbid non-local exits in prelude (recursive check).
                for stmt in prelude_stmts {
                    if stmt.contains_non_local_exit() {
                        return Err(
                            "[freeze:contract][blockexpr] exit stmt is forbidden in BlockExpr prelude"
                                .to_string(),
                        );
                    }
                }

                let (bindings, mut effects) = super::cond_lowering_prelude::lower_cond_prelude_stmts(
                    builder,
                    phi_bindings,
                    prelude_stmts,
                    "[normalizer][blockexpr]",
                )?;

                let (tail_id, mut tail_effects) =
                    Self::lower_value_ast(tail_expr.as_ref(), builder, &bindings)?;
                effects.append(&mut tail_effects);
                Ok((tail_id, effects))
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let Some(else_body) = else_body else {
                    return Err("[normalizer] value-if requires else branch".to_string());
                };
                if then_body.len() != 1 || else_body.len() != 1 {
                    return Err("[normalizer] value-if requires single expr in each branch".to_string());
                }
                let then_expr = &then_body[0];
                let else_expr = &else_body[0];
                if !is_pure_value_expr(condition)
                    || !is_pure_value_expr(then_expr)
                    || !is_pure_value_expr(else_expr)
                {
                    return Err("[normalizer] value-if requires pure expressions".to_string());
                }
                let cond_view = CondBlockView::from_expr(condition);
                let (cond_id, mut effects) = super::cond_lowering_entry::lower_bool_expr_value_id(
                    builder,
                    phi_bindings,
                    &cond_view,
                    "[normalizer] value-if",
                )?;
                let (then_id, mut then_effects) =
                    Self::lower_value_ast(then_expr, builder, phi_bindings)?;
                let (else_id, mut else_effects) =
                    Self::lower_value_ast(else_expr, builder, phi_bindings)?;
                effects.append(&mut then_effects);
                effects.append(&mut else_effects);
                let ty = builder
                    .type_ctx
                    .get_type(then_id)
                    .cloned()
                    .unwrap_or(MirType::Unknown);
                let result_id = builder.alloc_typed(ty);
                effects.push(CoreEffectPlan::Select {
                    dst: result_id,
                    cond: cond_id,
                    then_val: then_id,
                    else_val: else_id,
                });
                Ok((result_id, effects))
            }
            _ => Err(format!("[normalizer] Unsupported value AST: {:?}", ast)),
        }
    }

    fn record_newbox_metadata(builder: &mut MirBuilder, value_id: ValueId, class: &str) {
        let class_name = class.to_string();
        builder
            .type_ctx
            .value_types
            .insert(value_id, MirType::Box(class_name.clone()));
        builder
            .type_ctx
            .value_origin_newbox
            .insert(value_id, class_name.clone());
        builder
            .comp_ctx
            .type_registry
            .record_newbox(value_id, class_name.clone());
        builder
            .comp_ctx
            .type_registry
            .record_type(value_id, MirType::Box(class_name));
    }
}

fn is_pure_value_expr(ast: &crate::ast::ASTNode) -> bool {
    use crate::ast::{ASTNode, BinaryOperator};

    fn is_known_pure_method_call_for_value_if(object: &ASTNode, method: &str) -> bool {
        if matches!(
            (object, method),
            // Stage-B/JsonFrag normalizer uses ternary value-if with this helper.
            // It is deterministic and side-effect free for the current semantics.
            (ASTNode::Variable { name, .. }, "int_to_str") if name == "StringHelpers"
        ) {
            return true;
        }

        // Selfhost FuncLowering uses ternary value-if with String slice helpers.
        // These methods are pure reads and safe for Select-based lowering.
        if matches!(method, "substring" | "length" | "contains") {
            return matches!(
                object,
                ASTNode::Variable { .. }
                    | ASTNode::FieldAccess { .. }
                    | ASTNode::ThisField { .. }
                    | ASTNode::MeField { .. }
            );
        }

        false
    }

    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => {
            is_known_pure_method_call_for_value_if(object, method)
                && arguments.iter().all(is_pure_value_expr)
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            let Some(else_body) = else_body else {
                return false;
            };
            if then_body.len() != 1 || else_body.len() != 1 {
                return false;
            }
            is_pure_value_expr(condition)
                && is_pure_value_expr(&then_body[0])
                && is_pure_value_expr(&else_body[0])
        }
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => prelude_stmts.is_empty() && is_pure_value_expr(tail_expr),
        ASTNode::UnaryOp { operand, .. } => is_pure_value_expr(operand),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            matches!(
                operator,
                BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo
                    | BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual
            ) && is_pure_value_expr(left)
                && is_pure_value_expr(right)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_pure_value_expr;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn int_lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn value_if_allows_pure_string_substring() {
        let cond = ASTNode::BinaryOp {
            operator: BinaryOperator::GreaterEqual,
            left: Box::new(var("dot")),
            right: Box::new(int_lit(0)),
            span: Span::unknown(),
        };
        let then_expr = ASTNode::MethodCall {
            object: Box::new(var("last_val")),
            method: "substring".to_string(),
            arguments: vec![int_lit(0), var("dot")],
            span: Span::unknown(),
        };
        let else_expr = var("last_val");
        let value_if = ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![then_expr],
            else_body: Some(vec![else_expr]),
            span: Span::unknown(),
        };
        assert!(is_pure_value_expr(&value_if));
    }

    #[test]
    fn value_if_allows_empty_blockexpr_wrapped_branches() {
        let cond = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(var("a")),
            right: Box::new(var("b")),
            span: Span::unknown(),
        };
        let then_expr = ASTNode::BlockExpr {
            prelude_stmts: vec![],
            tail_expr: Box::new(int_lit(10)),
            span: Span::unknown(),
        };
        let else_expr = ASTNode::BlockExpr {
            prelude_stmts: vec![],
            tail_expr: Box::new(int_lit(20)),
            span: Span::unknown(),
        };
        let value_if = ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![then_expr],
            else_body: Some(vec![else_expr]),
            span: Span::unknown(),
        };
        assert!(is_pure_value_expr(&value_if));
    }

    #[test]
    fn value_if_rejects_blockexpr_with_prelude_side_effect() {
        let cond = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(var("a")),
            right: Box::new(var("b")),
            span: Span::unknown(),
        };
        let then_expr = ASTNode::BlockExpr {
            prelude_stmts: vec![ASTNode::Local {
                variables: vec!["tmp".to_string()],
                initial_values: vec![Some(Box::new(int_lit(1)))],
                span: Span::unknown(),
            }],
            tail_expr: Box::new(int_lit(10)),
            span: Span::unknown(),
        };
        let else_expr = int_lit(20);
        let value_if = ASTNode::If {
            condition: Box::new(cond),
            then_body: vec![then_expr],
            else_body: Some(vec![else_expr]),
            span: Span::unknown(),
        };
        assert!(!is_pure_value_expr(&value_if));
    }
}
