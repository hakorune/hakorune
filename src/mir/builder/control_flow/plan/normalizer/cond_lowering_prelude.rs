//! Condition prelude statement processing.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalizer::{loop_body_lowering, PlanNormalizer};
use crate::mir::builder::control_flow::plan::normalizer::lower_cond_value;
use crate::mir::builder::control_flow::plan::policies::cond_prelude_vocab::{classify_cond_prelude_stmt, CondPreludeStmtKind};
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{CompareOp, ConstValue, Effect, EffectMask, MirType, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder::control_flow::plan::normalizer) fn lower_cond_prelude_stmts(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    prelude_stmts: &[ASTNode],
    error_prefix: &str,
) -> Result<(BTreeMap<String, ValueId>, Vec<CoreEffectPlan>), String> {
    if prelude_stmts.is_empty() {
        return Ok((phi_bindings.clone(), Vec::new()));
    }

    let mut bindings = phi_bindings.clone();
    let mut effects = Vec::new();
    for stmt in prelude_stmts {
        if stmt.contains_non_local_exit_outside_loops() {
            return Err("[freeze:contract][cond_prelude] exit stmt is forbidden in condition prelude".to_string());
        }

        let Some(kind) = classify_cond_prelude_stmt(stmt) else {
            return Err(format!("[freeze:contract][cond_prelude] {error_prefix}: unsupported stmt: {}", stmt.node_type()));
        };

        match kind {
            CondPreludeStmtKind::Assignment => {
                let ASTNode::Assignment { target, value, .. } = stmt else { unreachable!() };
                let (name, value_id, mut stmt_effects) = loop_body_lowering::lower_assignment_value(builder, &bindings, target, value, error_prefix)?;
                bindings.insert(name.clone(), value_id);
                builder.variable_ctx.variable_map.insert(name, value_id);
                effects.append(&mut stmt_effects);
            }
            CondPreludeStmtKind::If => {
                let ASTNode::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } = stmt
                else {
                    unreachable!()
                };

                let cond_view = CondBlockView::from_expr(condition);
                let (cond_id, mut cond_effects) =
                    lower_cond_value(builder, &bindings, &cond_view, error_prefix)?;
                effects.append(&mut cond_effects);

                let (then_bindings, then_effects) =
                    lower_cond_prelude_stmts(builder, &bindings, then_body, error_prefix)?;
                let has_else = else_body.is_some();
                let (else_bindings, else_effects) = if let Some(else_body) = else_body.as_ref() {
                    lower_cond_prelude_stmts(builder, &bindings, else_body, error_prefix)?
                } else {
                    (bindings.clone(), Vec::new())
                };

                let mut if_cond = cond_id;
                let mut if_then_effects = then_effects;
                let mut if_else_effects = if has_else {
                    Some(else_effects)
                } else {
                    None
                };

                if let Some(else_effects) = if_else_effects.take() {
                    if if_then_effects.is_empty() && else_effects.is_empty() {
                        if_else_effects = None;
                    } else if if_then_effects.is_empty() {
                        let (neg_cond, mut neg_effects) = build_negated_bool_value(builder, cond_id);
                        effects.append(&mut neg_effects);
                        if_cond = neg_cond;
                        if_then_effects = else_effects;
                        if_else_effects = None;
                    } else if else_effects.is_empty() {
                        if_else_effects = None;
                    } else {
                        if_else_effects = Some(else_effects);
                    }
                }

                if !if_then_effects.is_empty() {
                    effects.push(CoreEffectPlan::IfEffect {
                        cond: if_cond,
                        then_effects: if_then_effects,
                        else_effects: if_else_effects,
                    });
                }

                // Merge only pre-existing bindings; branch-local declarations do not
                // escape the lexical prelude boundary.
                let merge_keys: Vec<String> = bindings.keys().cloned().collect();
                for name in merge_keys {
                    let Some(base_id) = bindings.get(&name).copied() else {
                        continue;
                    };
                    let then_id = then_bindings.get(&name).copied().unwrap_or(base_id);
                    let else_id = else_bindings.get(&name).copied().unwrap_or(base_id);
                    if then_id == else_id {
                        if then_id != base_id {
                            bindings.insert(name.clone(), then_id);
                            builder.variable_ctx.variable_map.insert(name, then_id);
                        }
                        continue;
                    }

                    let merged_ty = match (
                        builder.type_ctx.get_type(then_id).cloned(),
                        builder.type_ctx.get_type(else_id).cloned(),
                    ) {
                        (Some(then_ty), Some(else_ty)) if then_ty == else_ty => then_ty,
                        (Some(then_ty), None) => then_ty,
                        (None, Some(else_ty)) => else_ty,
                        _ => MirType::Unknown,
                    };
                    let merged_id = builder.alloc_typed(merged_ty);
                    effects.push(CoreEffectPlan::Select {
                        dst: merged_id,
                        cond: cond_id,
                        then_val: then_id,
                        else_val: else_id,
                    });
                    bindings.insert(name.clone(), merged_id);
                    builder.variable_ctx.variable_map.insert(name, merged_id);
                }
            }
            CondPreludeStmtKind::Local => {
                let ASTNode::Local { variables, initial_values, .. } = stmt else { unreachable!() };
                let (inits, mut stmt_effects) = loop_body_lowering::lower_local_init_values(builder, &bindings, variables, initial_values, error_prefix)?;
                for (name, value_id) in inits {
                    bindings.insert(name.clone(), value_id);
                    builder.variable_ctx.variable_map.insert(name, value_id);
                }
                effects.append(&mut stmt_effects);
            }
            CondPreludeStmtKind::MethodCall => {
                let mut stmt_effects = loop_body_lowering::lower_method_call_stmt(builder, &bindings, stmt, error_prefix)?;
                effects.append(&mut stmt_effects);
            }
            CondPreludeStmtKind::FunctionCall => {
                let mut stmt_effects = loop_body_lowering::lower_function_call_stmt(builder, &bindings, stmt, error_prefix)?;
                effects.append(&mut stmt_effects);
            }
            CondPreludeStmtKind::Print => {
                let ASTNode::Print { expression, .. } = stmt else { unreachable!() };
                let (value_id, mut stmt_effects) = PlanNormalizer::lower_value_ast(expression, builder, &bindings)?;
                stmt_effects.push(CoreEffectPlan::ExternCall {
                    dst: None,
                    iface_name: "env.console".to_string(),
                    method_name: "log".to_string(),
                    args: vec![value_id],
                    effects: EffectMask::PURE.add(Effect::Io),
                });
                effects.append(&mut stmt_effects);
            }
        }
    }

    Ok((bindings, effects))
}

fn build_negated_bool_value(
    builder: &mut MirBuilder,
    cond_id: ValueId,
) -> (ValueId, Vec<CoreEffectPlan>) {
    let false_id = builder.alloc_typed(MirType::Bool);
    let negated_id = builder.alloc_typed(MirType::Bool);
    let effects = vec![
        CoreEffectPlan::Const {
            dst: false_id,
            value: ConstValue::Bool(false),
        },
        CoreEffectPlan::Compare {
            dst: negated_id,
            lhs: cond_id,
            op: CompareOp::Eq,
            rhs: false_id,
        },
    ];
    (negated_id, effects)
}
