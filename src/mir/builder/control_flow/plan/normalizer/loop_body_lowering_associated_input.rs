//! Associated-source statement primitives for loop-body lowering.
//!
//! These helpers preserve the existing raw statement semantics while routing
//! every expression child through one borrowed `LoopPlanExpressionPortV1`.

use super::{loop_body_lowering, PlanNormalizer};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::builder::calls::extern_calls;
use crate::mir::builder::control_flow::plan::normalizer::common::lower_me_this_method_effect;
use crate::mir::builder::control_flow::plan::{
    CoreCallSourceV1, CoreEffectPlan, CoreExitPlan, CorePlan, LoopPlanExpressionPortV1,
    LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::{CompareOp, ConstValue, EffectMask, MirType, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn lower_assignment_inputs<'input, P>(
    port: &P,
    target: P::ExprInput<'input>,
    value: P::ExprInput<'input>,
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<(Option<(String, ValueId)>, Vec<CoreEffectPlan>), String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    match port.expr_syntax(&target) {
        ASTNode::Variable { name, .. } => {
            let (value_id, mut effects) =
                PlanNormalizer::lower_value_input(port, value, builder, phi_bindings)?;
            let (value_id, contract_effect) =
                loop_body_lowering::local_contract_reassignment_effect(builder, name, value_id)?;
            effects.extend(contract_effect);
            Ok((Some((name.clone(), value_id)), effects))
        }
        ASTNode::FieldAccess { field, .. } => {
            let object = port
                .child_expr(&target, ExprChildRoleV1::Receiver)
                .map_err(|error| error.render())?;
            let (object_id, mut effects) =
                PlanNormalizer::lower_value_input(port, object, builder, phi_bindings)?;
            let (value_id, mut value_effects) =
                PlanNormalizer::lower_value_input(port, value, builder, phi_bindings)?;
            effects.append(&mut value_effects);
            let declared_type =
                PlanNormalizer::declared_field_type_for_base(builder, object_id, field);
            effects.push(CoreEffectPlan::FieldSet {
                base: object_id,
                field: field.clone(),
                value: value_id,
                declared_type,
            });
            Ok((None, effects))
        }
        ASTNode::Index { .. } => {
            let indexed = port
                .child_expr(&target, ExprChildRoleV1::IndexTarget)
                .map_err(|error| error.render())?;
            let index = port
                .child_expr(&target, ExprChildRoleV1::IndexSubscript)
                .map_err(|error| error.render())?;
            let (target_id, mut effects) =
                PlanNormalizer::lower_value_input(port, indexed, builder, phi_bindings)?;
            let (index_id, mut index_effects) =
                PlanNormalizer::lower_value_input(port, index, builder, phi_bindings)?;
            effects.append(&mut index_effects);
            let (value_id, mut value_effects) =
                PlanNormalizer::lower_value_input(port, value, builder, phi_bindings)?;
            effects.append(&mut value_effects);
            effects.push(CoreEffectPlan::MethodCall {
                source: CoreCallSourceV1::Unlocated,
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

pub(in crate::mir::builder) fn lower_local_initializer_inputs<'input, P>(
    port: &P,
    variables: &[String],
    inputs: Vec<Option<P::ExprInput<'input>>>,
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<(Vec<(String, ValueId)>, Vec<CoreEffectPlan>), String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    if variables.len() != inputs.len() {
        return Err(format!("{error_prefix}: local init arity mismatch"));
    }
    let mut effects = Vec::new();
    let mut inits = Vec::with_capacity(variables.len());
    for (name, input) in variables.iter().zip(inputs) {
        let (value_id, mut init_effects) = match input {
            Some(input) => PlanNormalizer::lower_value_input(port, input, builder, phi_bindings)?,
            None => {
                let null = ASTNode::Literal {
                    value: LiteralValue::Null,
                    span: Span::unknown(),
                };
                PlanNormalizer::lower_value_ast(&null, builder, phi_bindings)?
            }
        };
        effects.append(&mut init_effects);
        inits.push((name.clone(), value_id));
    }
    Ok((inits, effects))
}

pub(in crate::mir::builder) fn lower_local_statement_input<'input, P>(
    port: &P,
    statement: P::StmtInput<'input>,
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<(Vec<(String, ValueId)>, Vec<CoreEffectPlan>), String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = port.stmt_syntax(&statement)
    else {
        return Err(format!("{error_prefix}: expected local statement"));
    };
    let mut inputs = Vec::with_capacity(initial_values.len());
    for (index, initial) in initial_values.iter().enumerate() {
        inputs.push(if initial.is_some() {
            Some(
                port.child_expr_from_stmt(
                    &statement,
                    ExprChildRoleV1::LocalInitializer(index as u32),
                )
                .map_err(|error| error.render())?,
            )
        } else {
            None
        });
    }
    lower_local_initializer_inputs(port, variables, inputs, builder, phi_bindings, error_prefix)
}

pub(in crate::mir::builder) fn lower_method_call_statement_input<'input, P>(
    port: &P,
    input: P::ExprInput<'input>,
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<Vec<CoreEffectPlan>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let ASTNode::MethodCall {
        method, arguments, ..
    } = port.expr_syntax(&input)
    else {
        return Err(format!("{error_prefix}: expected method call"));
    };
    let source = port.call_source(&input).map_err(|error| error.render())?;
    let mut arg_ids = Vec::with_capacity(arguments.len());
    let mut effects = Vec::new();
    for index in 0..arguments.len() {
        let argument = port
            .child_expr(&input, ExprChildRoleV1::CallArgument(index as u32))
            .map_err(|error| error.render())?;
        let (arg_id, mut arg_effects) =
            PlanNormalizer::lower_value_input(port, argument, builder, phi_bindings)?;
        arg_ids.push(arg_id);
        effects.append(&mut arg_effects);
    }
    loop_body_lowering::debug_log_callstmt_binop_lit3(builder, &effects, "method");

    let receiver = port
        .child_expr(&input, ExprChildRoleV1::Receiver)
        .map_err(|error| error.render())?;
    match port.expr_syntax(&receiver) {
        ASTNode::Variable { name, .. } if name == "env" => {
            let Some((iface_name, method_name, effects_mask, _)) =
                extern_calls::get_env_method_spec("env", method)
            else {
                return Err(format!(
                    "{error_prefix}: env method not supported: {method}"
                ));
            };
            effects.push(CoreEffectPlan::ExternCall {
                source,
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
                effects.push(CoreEffectPlan::GlobalCall {
                    source,
                    dst: None,
                    func: format!("{}.{}/{}", name, method, arguments.len()),
                    args: arg_ids,
                });
                return Ok(effects);
            } else {
                return Err(format!(
                    "{error_prefix}: method call object {name} not found"
                ));
            };
            effects.push(CoreEffectPlan::MethodCall {
                source,
                dst: None,
                object: object_id,
                method: method.clone(),
                args: arg_ids,
                effects: EffectMask::PURE.add(crate::mir::Effect::Io),
            });
        }
        ASTNode::Me { .. } | ASTNode::This { .. } => effects.push(lower_me_this_method_effect(
            builder,
            phi_bindings,
            port.expr_syntax(&receiver),
            source,
            method,
            arg_ids,
            arguments.len(),
            None,
            format!("{error_prefix}: me.method without bound receiver"),
            format!("{error_prefix}: this.method without static box"),
        )?),
        _ => {
            let (object_id, mut object_effects) =
                PlanNormalizer::lower_value_input(port, receiver, builder, phi_bindings)?;
            effects.append(&mut object_effects);
            effects.push(CoreEffectPlan::MethodCall {
                source,
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

pub(in crate::mir::builder) fn lower_function_call_statement_input<'input, P>(
    port: &P,
    input: P::ExprInput<'input>,
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<Vec<CoreEffectPlan>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let ASTNode::FunctionCall {
        name, arguments, ..
    } = port.expr_syntax(&input)
    else {
        return Err(format!("{error_prefix}: expected function call"));
    };
    let source = port.call_source(&input).map_err(|error| error.render())?;
    let extern_target = if name == "externcall" {
        if arguments.is_empty() {
            return Err(format!(
                "{error_prefix}: externcall requires a target string literal"
            ));
        }
        Some(
            port.child_expr(&input, ExprChildRoleV1::CallArgument(0))
                .map_err(|error| error.render())?,
        )
    } else {
        None
    };
    let start = usize::from(extern_target.is_some());
    let mut arg_ids = Vec::with_capacity(arguments.len().saturating_sub(start));
    let mut effects = Vec::new();
    for index in start..arguments.len() {
        let argument = port
            .child_expr(&input, ExprChildRoleV1::CallArgument(index as u32))
            .map_err(|error| error.render())?;
        let (arg_id, mut arg_effects) =
            PlanNormalizer::lower_value_input(port, argument, builder, phi_bindings)?;
        arg_ids.push(arg_id);
        effects.append(&mut arg_effects);
    }
    loop_body_lowering::debug_log_callstmt_binop_lit3(builder, &effects, "function");

    if let Some(target) = extern_target {
        let extern_name = crate::mir::builder::calls::special_handlers::extract_string_literal(
            port.expr_syntax(&target),
        )
        .ok_or_else(|| format!("{error_prefix}: externcall target must be a string literal"))?;
        let (iface_name, method_name) = extern_calls::split_explicit_extern_name(&extern_name);
        effects.push(CoreEffectPlan::ExternCall {
            source,
            dst: None,
            iface_name,
            method_name,
            args: arg_ids,
            effects: EffectMask::IO,
        });
    } else {
        effects.push(CoreEffectPlan::GlobalCall {
            source,
            dst: None,
            func: name.clone(),
            args: arg_ids,
        });
    }
    Ok(effects)
}

pub(in crate::mir::builder) fn lower_return_statement_input<'input, P>(
    port: &P,
    statement: P::StmtInput<'input>,
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let ASTNode::Return { value, .. } = port.stmt_syntax(&statement) else {
        return Err(format!("{error_prefix}: expected return statement"));
    };
    let mut plans = Vec::new();
    let value_id = if value.is_some() {
        let input = port
            .child_expr_from_stmt(&statement, ExprChildRoleV1::ReturnValue)
            .map_err(|error| error.render())?;
        let (value_id, effects) =
            PlanNormalizer::lower_value_input(port, input, builder, current_bindings)?;
        plans.extend(crate::mir::builder::control_flow::plan::steps::effects_to_plans(effects));
        Some(value_id)
    } else {
        None
    };
    plans.push(CorePlan::Exit(CoreExitPlan::Return(value_id)));
    Ok(plans)
}

pub(in crate::mir::builder) fn lower_bool_expression_input<'input, P>(
    port: &P,
    input: P::ExprInput<'input>,
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    error_prefix: &str,
) -> Result<(ValueId, Vec<CoreEffectPlan>), String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    match port.expr_syntax(&input) {
        ASTNode::MethodCall { .. }
        | ASTNode::Variable { .. }
        | ASTNode::Literal {
            value: LiteralValue::Bool(_),
            ..
        } => {
            let result = PlanNormalizer::lower_value_input(port, input, builder, phi_bindings)?;
            loop_body_lowering::debug_log_bool_expr_binop_lit3(builder, &result.1, "simple");
            Ok(result)
        }
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            ..
        } => {
            let operand = port
                .child_expr(&input, ExprChildRoleV1::UnaryOperand)
                .map_err(|error| error.render())?;
            let (inner, mut effects) =
                lower_bool_expression_input(port, operand, builder, phi_bindings, error_prefix)?;
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
            loop_body_lowering::debug_log_bool_expr_binop_lit3(builder, &effects, "not");
            Ok((dst, effects))
        }
        ASTNode::BinaryOp { operator, .. } => match operator {
            BinaryOperator::And | BinaryOperator::Or => {
                let left = port
                    .child_expr(&input, ExprChildRoleV1::BinaryLeft)
                    .map_err(|error| error.render())?;
                let right = port
                    .child_expr(&input, ExprChildRoleV1::BinaryRight)
                    .map_err(|error| error.render())?;
                let (lhs, mut effects) =
                    lower_bool_expression_input(port, left, builder, phi_bindings, error_prefix)?;
                let (rhs, mut rhs_effects) =
                    lower_bool_expression_input(port, right, builder, phi_bindings, error_prefix)?;
                let constant = builder.alloc_typed(MirType::Bool);
                effects.push(CoreEffectPlan::Const {
                    dst: constant,
                    value: ConstValue::Bool(matches!(operator, BinaryOperator::Or)),
                });
                effects.append(&mut rhs_effects);
                let dst = builder.alloc_typed(MirType::Bool);
                effects.push(CoreEffectPlan::Select {
                    dst,
                    cond: lhs,
                    then_val: if matches!(operator, BinaryOperator::Or) {
                        constant
                    } else {
                        rhs
                    },
                    else_val: if matches!(operator, BinaryOperator::Or) {
                        rhs
                    } else {
                        constant
                    },
                });
                loop_body_lowering::debug_log_bool_expr_binop_lit3(builder, &effects, "and_or");
                Ok((dst, effects))
            }
            BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual => {
                let (lhs, op, rhs, mut effects) =
                    PlanNormalizer::lower_compare_input(port, input, builder, phi_bindings)?;
                let dst = builder.alloc_typed(MirType::Bool);
                effects.push(CoreEffectPlan::Compare { dst, lhs, op, rhs });
                loop_body_lowering::debug_log_bool_expr_binop_lit3(builder, &effects, "compare");
                Ok((dst, effects))
            }
            _ => Err(format!("{error_prefix}: unsupported bool op")),
        },
        _ => Err(format!("{error_prefix}: unsupported bool expr")),
    }
}
