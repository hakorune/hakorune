//! Shared lowering helpers for loop body statements (generic_loop_v0/v1 + loop_true_break_continue).

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::builder::calls::extern_calls;
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::control_flow::plan::normalizer::common::lower_me_this_method_effect;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::MirBuilder;
use crate::mir::{BinaryOp, CompareOp, ConstValue, EffectMask, MirType, ValueId};
use std::borrow::Cow;
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn lower_assignment_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    target: &ASTNode,
    value: &ASTNode,
    error_prefix: &str,
) -> Result<(Option<(String, ValueId)>, Vec<CoreEffectPlan>), String> {
    match target {
        ASTNode::Variable { name, .. } => {
            let (value_id, effects) = PlanNormalizer::lower_value_ast(value, builder, phi_bindings)?;
            Ok((Some((name.clone(), value_id)), effects))
        }
        ASTNode::FieldAccess { object, field, .. } => {
            let (object_id, mut effects) =
                PlanNormalizer::lower_value_ast(object, builder, phi_bindings)?;
            let (value_id, mut value_effects) =
                PlanNormalizer::lower_value_ast(value, builder, phi_bindings)?;
            effects.append(&mut value_effects);

            let field_lit = ASTNode::Literal {
                value: LiteralValue::String(field.clone()),
                span: Span::unknown(),
            };
            let (field_name_id, mut field_effects) =
                PlanNormalizer::lower_value_ast(&field_lit, builder, phi_bindings)?;
            effects.append(&mut field_effects);

            effects.push(CoreEffectPlan::MethodCall {
                dst: None,
                object: object_id,
                method: "setField".to_string(),
                args: vec![field_name_id, value_id],
                effects: EffectMask::WRITE,
            });
            Ok((None, effects))
        }
        ASTNode::Index { target, index, .. } => {
            let (target_id, mut effects) =
                PlanNormalizer::lower_value_ast(target, builder, phi_bindings)?;
            let (index_id, mut index_effects) =
                PlanNormalizer::lower_value_ast(index, builder, phi_bindings)?;
            effects.append(&mut index_effects);
            let (value_id, mut value_effects) =
                PlanNormalizer::lower_value_ast(value, builder, phi_bindings)?;
            effects.append(&mut value_effects);

            effects.push(CoreEffectPlan::MethodCall {
                dst: None,
                object: target_id,
                method: "set".to_string(),
                args: vec![index_id, value_id],
                effects: EffectMask::MUT,
            });
            Ok((None, effects))
        }
        _ => Err(format!("{error_prefix}: unsupported assignment target")),
    }
}

pub(in crate::mir::builder) fn lower_assignment_value(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    target: &ASTNode,
    value: &ASTNode,
    error_prefix: &str,
) -> Result<(String, ValueId, Vec<CoreEffectPlan>), String> {
    let (binding, effects) =
        lower_assignment_stmt(builder, phi_bindings, target, value, error_prefix)?;
    let Some((name, value_id)) = binding else {
        return Err(format!("{error_prefix}: non-variable assignment"));
    };
    Ok((name, value_id, effects))
}

pub(in crate::mir::builder) fn local_init_node_or_null<'a>(
    init: Option<&'a Box<ASTNode>>,
) -> Cow<'a, ASTNode> {
    match init {
        Some(init) => Cow::Borrowed(init.as_ref()),
        None => Cow::Owned(ASTNode::Literal {
            value: LiteralValue::Null,
            span: Span::unknown(),
        }),
    }
}

pub(in crate::mir::builder) fn lower_local_init_values(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    variables: &[String],
    initial_values: &[Option<Box<ASTNode>>],
    error_prefix: &str,
) -> Result<(Vec<(String, ValueId)>, Vec<CoreEffectPlan>), String> {
    if variables.len() != initial_values.len() {
        return Err(format!("{error_prefix}: local init arity mismatch"));
    }
    let mut effects = Vec::new();
    let mut inits = Vec::with_capacity(variables.len());
    for (name, init) in variables.iter().zip(initial_values.iter()) {
        let init_node = local_init_node_or_null(init.as_ref());
        let (value_id, mut init_effects) =
            PlanNormalizer::lower_value_ast(init_node.as_ref(), builder, phi_bindings)?;
        effects.append(&mut init_effects);
        inits.push((name.to_string(), value_id));
    }
    Ok((inits, effects))
}

pub(in crate::mir::builder) fn lower_method_call_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<CoreEffectPlan>, String> {
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = stmt
    else {
        return Err(format!("{error_prefix}: expected method call"));
    };

    let mut arg_ids = Vec::new();
    let mut effects = Vec::new();
    for arg in arguments {
        let (arg_id, mut arg_effects) =
            PlanNormalizer::lower_value_ast(arg, builder, phi_bindings)?;
        arg_ids.push(arg_id);
        effects.append(&mut arg_effects);
    }
    debug_log_callstmt_binop_lit3(builder, &effects, "method");

    match object.as_ref() {
        ASTNode::Variable { name, .. } if name == "env" => {
            let Some((iface_name, method_name, effects_mask, _returns_value)) =
                extern_calls::get_env_method_spec("env", method)
            else {
                return Err(format!("{error_prefix}: env method not supported: {}", method));
            };
            effects.push(CoreEffectPlan::ExternCall {
                dst: None,
                iface_name,
                method_name,
                args: arg_ids,
                effects: effects_mask,
            });
        }
        ASTNode::Variable { name, .. } => {
            let object_id = if let Some(&phi_dst) = phi_bindings.get(name) {
                phi_dst
            } else if let Some(&value_id) = builder.variable_ctx.variable_map.get(name) {
                value_id
            } else if builder.comp_ctx.user_defined_boxes.contains_key(name) {
                let func = format!("{}.{}/{}", name, method, arguments.len());
                effects.push(CoreEffectPlan::GlobalCall {
                    dst: None,
                    func,
                    args: arg_ids,
                });
                return Ok(effects);
            } else {
                return Err(format!(
                    "{error_prefix}: method call object {} not found",
                    name
                ));
            };
            effects.push(CoreEffectPlan::MethodCall {
                dst: None,
                object: object_id,
                method: method.clone(),
                args: arg_ids,
                effects: EffectMask::PURE.add(crate::mir::Effect::Io),
            });
        }
        ASTNode::Me { .. } | ASTNode::This { .. } => {
            let effect = lower_me_this_method_effect(
                builder,
                phi_bindings,
                object.as_ref(),
                method,
                arg_ids,
                arguments.len(),
                None,
                format!("{error_prefix}: me.method without bound receiver"),
                format!("{error_prefix}: this.method without static box"),
            )?;
            effects.push(effect);
        }
        _ => {
            let (object_id, mut obj_effects) =
                PlanNormalizer::lower_value_ast(object, builder, phi_bindings)?;
            effects.append(&mut obj_effects);
            effects.push(CoreEffectPlan::MethodCall {
                dst: None,
                object: object_id,
                method: method.clone(),
                args: arg_ids,
                effects: EffectMask::PURE.add(crate::mir::Effect::Io),
            });
        }
    }

    Ok(effects)
}

fn debug_log_callstmt_binop_lit3(
    builder: &MirBuilder,
    effects: &[CoreEffectPlan],
    kind: &'static str,
) {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        return;
    }

    let mut int3_dsts: Vec<ValueId> = Vec::new();
    let mut add_binop: Option<(ValueId, ValueId, ValueId)> = None;
    for effect in effects {
        match effect {
            CoreEffectPlan::Const { dst, value } => {
                if matches!(value, ConstValue::Integer(3)) {
                    int3_dsts.push(*dst);
                }
            }
            CoreEffectPlan::BinOp { dst, lhs, op, rhs } => {
                if *op == BinaryOp::Add && add_binop.is_none() {
                    add_binop = Some((*dst, *lhs, *rhs));
                }
            }
            _ => {}
        }
    }

    if int3_dsts.is_empty() || add_binop.is_none() {
        return;
    }

    let fn_name = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str())
        .unwrap_or("<none>");
    let const_int3_dsts = int3_dsts
        .iter()
        .map(|v| format!("%{}", v.0))
        .collect::<Vec<_>>()
        .join(",");
    let (dst, lhs, rhs) = add_binop.unwrap();
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[callstmt/effects:binop_lit3] fn={} bb={:?} effects_len={} const_int3_dsts=[{}] add_binops=[dst=%{} lhs=%{} rhs=%{}] kind={}",
        fn_name,
        builder.current_block,
        effects.len(),
        const_int3_dsts,
        dst.0,
        lhs.0,
        rhs.0,
        kind
    ));
}

pub(in crate::mir::builder) fn lower_function_call_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<CoreEffectPlan>, String> {
    let ASTNode::FunctionCall { name, arguments, .. } = stmt else {
        return Err(format!("{error_prefix}: expected function call"));
    };

    let mut arg_ids = Vec::new();
    let mut effects = Vec::new();
    for arg in arguments {
        let (arg_id, mut arg_effects) =
            PlanNormalizer::lower_value_ast(arg, builder, phi_bindings)?;
        arg_ids.push(arg_id);
        effects.append(&mut arg_effects);
    }
    debug_log_callstmt_binop_lit3(builder, &effects, "function");
    effects.push(CoreEffectPlan::GlobalCall {
        dst: None,
        func: name.clone(),
        args: arg_ids,
    });
    Ok(effects)
}

pub(in crate::mir::builder) fn lower_bool_expr(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    ast: &ASTNode,
    error_prefix: &str,
) -> Result<(ValueId, Vec<CoreEffectPlan>), String> {
    match ast {
        ASTNode::MethodCall { .. }
        | ASTNode::Variable { .. }
        | ASTNode::Literal {
            value: LiteralValue::Bool(_),
            ..
        } => {
            let (value_id, effects) = PlanNormalizer::lower_value_ast(ast, builder, phi_bindings)?;
            debug_log_bool_expr_binop_lit3(builder, &effects, "simple");
            Ok((value_id, effects))
        }
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand,
            ..
        } => {
            let (inner, mut effects) = lower_bool_expr(builder, phi_bindings, operand, error_prefix)?;
            let false_id = builder.alloc_typed(MirType::Bool);
            effects.push(CoreEffectPlan::Const {
                dst: false_id,
                value: ConstValue::Bool(false),
            });
            let dst = builder.alloc_typed(MirType::Bool);
            effects.push(CoreEffectPlan::Compare {
                dst,
                lhs: inner,
                op: CompareOp::Eq,
                rhs: false_id,
            });
            debug_log_bool_expr_binop_lit3(builder, &effects, "not");
            Ok((dst, effects))
        }
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => match operator {
            BinaryOperator::And | BinaryOperator::Or => {
                let (lhs, mut lhs_effects) =
                    lower_bool_expr(builder, phi_bindings, left, error_prefix)?;
                let (rhs, mut rhs_effects) =
                    lower_bool_expr(builder, phi_bindings, right, error_prefix)?;
                let dst = builder.alloc_typed(MirType::Bool);
                lhs_effects.append(&mut rhs_effects);
                lhs_effects.push(CoreEffectPlan::BinOp {
                    dst,
                    lhs,
                    op: match operator {
                        BinaryOperator::And => BinaryOp::And,
                        BinaryOperator::Or => BinaryOp::Or,
                        _ => unreachable!(),
                    },
                    rhs,
                });
                debug_log_bool_expr_binop_lit3(builder, &lhs_effects, "and_or");
                Ok((dst, lhs_effects))
            }
            BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual => {
                let (lhs, op, rhs, mut consts) =
                    PlanNormalizer::lower_compare_ast(ast, builder, phi_bindings)?;
                let dst = builder.alloc_typed(MirType::Bool);
                consts.push(CoreEffectPlan::Compare { dst, lhs, op, rhs });
                debug_log_bool_expr_binop_lit3(builder, &consts, "compare");
                Ok((dst, consts))
            }
            _ => Err(format!("{error_prefix}: unsupported bool op")),
        },
        _ => Err(format!("{error_prefix}: unsupported bool expr")),
    }
}

fn debug_log_bool_expr_binop_lit3(
    builder: &MirBuilder,
    effects: &[CoreEffectPlan],
    kind: &'static str,
) {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        return;
    }

    let mut int3_dsts: Vec<ValueId> = Vec::new();
    let mut add_binop: Option<(ValueId, ValueId, ValueId)> = None;
    for effect in effects {
        match effect {
            CoreEffectPlan::Const { dst, value } => {
                if matches!(value, ConstValue::Integer(3)) {
                    int3_dsts.push(*dst);
                }
            }
            CoreEffectPlan::BinOp { dst, lhs, op, rhs } => {
                if *op == BinaryOp::Add && add_binop.is_none() {
                    add_binop = Some((*dst, *lhs, *rhs));
                }
            }
            _ => {}
        }
    }

    if int3_dsts.is_empty() || add_binop.is_none() {
        return;
    }

    let fn_name = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str())
        .unwrap_or("<none>");
    let const_int3_dsts = int3_dsts
        .iter()
        .map(|v| format!("%{}", v.0))
        .collect::<Vec<_>>()
        .join(",");
    let (dst, lhs, rhs) = add_binop.unwrap();
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[bool_expr/effects:binop_lit3] fn={} bb={:?} effects_len={} const_int3_dsts=[{}] add_binops=[dst=%{} lhs=%{} rhs=%{}] kind={}",
        fn_name,
        builder.current_block,
        effects.len(),
        const_int3_dsts,
        dst.0,
        lhs.0,
        rhs.0,
        kind
    ));
}
