//! Shared lowering helpers for loop body statements (generic_loop_v0/v1 + loop_true_break_continue).

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::calls::extern_calls;
use crate::mir::builder::control_flow::plan::normalizer::loop_body_lowering_associated_input;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::{
    CoreCallSourceV1, CoreEffectPlan, RawLoopPlanExpressionPortV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{BinaryOp, ConstValue, EffectMask, ValueId};
use std::borrow::Cow;
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn lower_assignment_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    target: &ASTNode,
    value: &ASTNode,
    error_prefix: &str,
) -> Result<(Option<(String, ValueId)>, Vec<CoreEffectPlan>), String> {
    let port = RawLoopPlanExpressionPortV1::new();
    loop_body_lowering_associated_input::lower_assignment_inputs(
        &port,
        port.expr(target),
        port.expr(value),
        builder,
        phi_bindings,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn local_contract_reassignment_effect(
    builder: &mut MirBuilder,
    name: &str,
    src: ValueId,
) -> Result<(ValueId, Option<CoreEffectPlan>), String> {
    let Some(binding_id) = builder.binding_ctx.lookup(name) else {
        return Ok((src, None));
    };
    let local_slot_id = crate::mir::LocalSlotId::from(binding_id);
    let has_contract = builder
        .scope_ctx
        .current_function
        .as_ref()
        .and_then(|function| {
            crate::mir::type_contracts::local_slot::local_slot_contract(function, local_slot_id)
        })
        .is_some();
    if !has_contract {
        return Ok((src, None));
    }
    let dst = builder.next_value_id();
    let effect = CoreEffectPlan::LocalContractWrite {
        dst,
        src,
        local_slot_id,
        write_kind: crate::mir::function::LocalContractWriteKind::Reassign,
    };
    Ok((dst, Some(effect)))
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
    let port = RawLoopPlanExpressionPortV1::new();
    let inputs = initial_values
        .iter()
        .map(|input| input.as_deref().map(|input| port.expr(input)))
        .collect();
    loop_body_lowering_associated_input::lower_local_initializer_inputs(
        &port,
        variables,
        inputs,
        builder,
        phi_bindings,
        error_prefix,
    )
}

fn lower_explicit_extern_call_args(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    arguments: &[ASTNode],
    error_prefix: &str,
) -> Result<(String, Vec<ValueId>, Vec<CoreEffectPlan>), String> {
    if arguments.is_empty() {
        return Err(format!(
            "{error_prefix}: externcall requires a target string literal"
        ));
    }

    let extern_name =
        crate::mir::builder::calls::special_handlers::extract_string_literal(&arguments[0])
            .ok_or_else(|| format!("{error_prefix}: externcall target must be a string literal"))?;

    let mut arg_ids = Vec::new();
    let mut effects = Vec::new();
    for arg in &arguments[1..] {
        let (arg_id, mut arg_effects) =
            PlanNormalizer::lower_value_ast(arg, builder, phi_bindings)?;
        arg_ids.push(arg_id);
        effects.append(&mut arg_effects);
    }

    Ok((extern_name, arg_ids, effects))
}

pub(in crate::mir::builder) fn lower_explicit_extern_call_value(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    arguments: &[ASTNode],
    error_prefix: &str,
) -> Result<(ValueId, Vec<CoreEffectPlan>), String> {
    let (extern_name, arg_ids, mut effects) =
        lower_explicit_extern_call_args(builder, phi_bindings, arguments, error_prefix)?;
    let result_id = builder.next_value_id();
    builder.type_ctx.set_type(
        result_id,
        extern_calls::explicit_extern_return_type(&extern_name),
    );
    let (iface_name, method_name) = extern_calls::split_explicit_extern_name(&extern_name);
    effects.push(CoreEffectPlan::ExternCall {
        source: CoreCallSourceV1::Unlocated,
        dst: Some(result_id),
        iface_name,
        method_name,
        args: arg_ids,
        effects: EffectMask::IO,
    });
    Ok((result_id, effects))
}

pub(in crate::mir::builder) fn lower_method_call_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<CoreEffectPlan>, String> {
    let port = RawLoopPlanExpressionPortV1::new();
    loop_body_lowering_associated_input::lower_method_call_statement_input(
        &port,
        port.expr(stmt),
        builder,
        phi_bindings,
        error_prefix,
    )
}

pub(super) fn debug_log_callstmt_binop_lit3(
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
    let port = RawLoopPlanExpressionPortV1::new();
    loop_body_lowering_associated_input::lower_function_call_statement_input(
        &port,
        port.expr(stmt),
        builder,
        phi_bindings,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn lower_bool_expr(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    ast: &ASTNode,
    error_prefix: &str,
) -> Result<(ValueId, Vec<CoreEffectPlan>), String> {
    let port = RawLoopPlanExpressionPortV1::new();
    loop_body_lowering_associated_input::lower_bool_expression_input(
        &port,
        port.expr(ast),
        builder,
        phi_bindings,
        error_prefix,
    )
}

pub(super) fn debug_log_bool_expr_binop_lit3(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::MirType;

    fn span() -> Span {
        Span::unknown()
    }

    fn lit_str(value: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(value.to_string()),
            span: span(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    #[test]
    fn explicit_externcall_value_lowers_to_extern_effect() {
        let mut builder = MirBuilder::new();
        let mut bindings = BTreeMap::new();
        bindings.insert("head".to_string(), ValueId::new(10));

        let (dst, effects) = lower_explicit_extern_call_value(
            &mut builder,
            &bindings,
            &[
                lit_str("hako_atomic_ptr_load_ordered"),
                var("head"),
                lit_int(1),
            ],
            "test externcall value",
        )
        .expect("externcall value must lower");

        assert_eq!(
            builder.type_ctx.get_type(dst).cloned(),
            Some(MirType::Integer)
        );
        assert!(
            effects.iter().any(|effect| matches!(
                effect,
                CoreEffectPlan::ExternCall {
                    dst: Some(call_dst),
                    iface_name,
                    method_name,
                    args,
                    effects,
                    source: _,
                } if *call_dst == dst
                    && iface_name.is_empty()
                    && method_name == "hako_atomic_ptr_load_ordered"
                    && args.first() == Some(&ValueId::new(10))
                    && *effects == EffectMask::IO
            )),
            "explicit externcall value must become CoreEffectPlan::ExternCall: {:?}",
            effects
        );
    }

    #[test]
    fn explicit_externcall_statement_lowers_to_extern_effect() {
        let mut builder = MirBuilder::new();
        let mut bindings = BTreeMap::new();
        bindings.insert("head".to_string(), ValueId::new(10));
        bindings.insert("old".to_string(), ValueId::new(11));
        let stmt = ASTNode::FunctionCall {
            name: "externcall".to_string(),
            arguments: vec![
                lit_str("hako_atomic_ptr_store_ordered"),
                var("head"),
                var("old"),
                lit_int(2),
            ],
            span: span(),
        };

        let effects =
            lower_function_call_stmt(&mut builder, &bindings, &stmt, "test externcall stmt")
                .expect("externcall statement must lower");

        assert!(
            effects.iter().any(|effect| matches!(
                effect,
                CoreEffectPlan::ExternCall {
                    dst: None,
                    iface_name,
                    method_name,
                    args,
                    effects,
                    source: _,
                } if iface_name.is_empty()
                    && method_name == "hako_atomic_ptr_store_ordered"
                    && args.first() == Some(&ValueId::new(10))
                    && args.get(1) == Some(&ValueId::new(11))
                    && *effects == EffectMask::IO
            )),
            "explicit externcall statement must become CoreEffectPlan::ExternCall: {:?}",
            effects
        );
    }
}
