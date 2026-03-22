pub(in crate::mir::builder) use super::helpers_layout::{
    create_phi_bindings, LoopBlocksStandard5,
};
use super::CoreEffectPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{BinaryOp, CompareOp, ConstValue, ValueId};
use std::collections::BTreeMap;

// ============================================================================
// Phase 286 P2.8: Normalizer Hygiene Helpers
// ============================================================================

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
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
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
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
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
}
