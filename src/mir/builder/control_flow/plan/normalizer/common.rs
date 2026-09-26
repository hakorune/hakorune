use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::{
    CoreCallSourceV1, CoreEffectPlan, LoopPlanExpressionPortV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::edge_args::JumpArgsLayout;
use crate::mir::EdgeArgs;
use crate::mir::{CompareOp, ConstValue, Effect, EffectMask, MirType, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn empty_args() -> EdgeArgs {
    EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: vec![],
    }
}

pub(in crate::mir::builder) fn negate_bool_cond(
    builder: &mut MirBuilder,
    cond_id: ValueId,
) -> (ValueId, Vec<CoreEffectPlan>) {
    let false_id = builder.alloc_typed(MirType::Bool);
    let neg_id = builder.alloc_typed(MirType::Bool);
    let effects = vec![
        CoreEffectPlan::Const {
            dst: false_id,
            value: ConstValue::Bool(false),
        },
        CoreEffectPlan::Compare {
            dst: neg_id,
            lhs: cond_id,
            op: CompareOp::Eq,
            rhs: false_id,
        },
    ];
    (neg_id, effects)
}

pub(in crate::mir::builder) fn lower_me_this_method_effect(
    builder: &MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    receiver: &ASTNode,
    source: CoreCallSourceV1,
    method: &str,
    args: Vec<ValueId>,
    arity: usize,
    dst: Option<ValueId>,
    missing_me_error: String,
    missing_this_error: String,
) -> Result<CoreEffectPlan, String> {
    let bound_me = phi_bindings.get("me").copied().or_else(|| {
        builder
            .function_state
            .variable_ctx
            .variable_map
            .get("me")
            .copied()
    });

    match receiver {
        ASTNode::Me { .. } => {
            if let Some(object_id) = bound_me {
                Ok(CoreEffectPlan::MethodCall {
                    source,
                    dst,
                    object: object_id,
                    method: method.to_string(),
                    args,
                    effects: EffectMask::PURE.add(Effect::Io),
                })
            } else if let Some(box_name) = builder.comp_ctx.current_static_box.as_deref() {
                Ok(CoreEffectPlan::GlobalCall {
                    source,
                    dst,
                    func: format!("{}.{}/{}", box_name, method, arity),
                    args,
                })
            } else {
                Err(missing_me_error)
            }
        }
        ASTNode::This { .. } => {
            if let Some(box_name) = builder.comp_ctx.current_static_box.as_deref() {
                Ok(CoreEffectPlan::GlobalCall {
                    source,
                    dst,
                    func: format!("{}.{}/{}", box_name, method, arity),
                    args,
                })
            } else if let Some(object_id) = bound_me {
                Ok(CoreEffectPlan::MethodCall {
                    source,
                    dst,
                    object: object_id,
                    method: method.to_string(),
                    args,
                    effects: EffectMask::PURE.add(Effect::Io),
                })
            } else {
                Err(missing_this_error)
            }
        }
        _ => Err("[normalizer] internal: expected me/this receiver".to_string()),
    }
}

/// Armed-lane declared-instance (`me.method`) consultation shared by the
/// statement- and value-position receivers.  `Some(plan)` carries the
/// canonical callable key and ledger-bound receiver; `None` keeps the
/// caller's existing receiver path.  Take errors propagate so an armed
/// locator never falls back to dynamic dispatch.
fn declared_instance_call_effect<P>(
    port: &P,
    input: &P::ExprInput<'_>,
    method: &str,
    arity: u32,
    dst: Option<ValueId>,
    args: Vec<ValueId>,
    source: CoreCallSourceV1,
    error_prefix: &str,
) -> Result<Option<CoreEffectPlan>, String>
where
    P: LoopPlanExpressionPortV1 + ?Sized,
{
    let Some(declared) = port
        .exact_source_declared_instance_call_v1(input, method, arity)
        .map_err(|error| format!("{error_prefix}: {error}"))?
    else {
        return Ok(None);
    };
    Ok(Some(CoreEffectPlan::DeclaredInstanceCall {
        dst,
        key: declared.key().clone(),
        receiver: declared.receiver(),
        args,
        source,
    }))
}

/// One `me`/`this` receiver call lane shared by the statement- and
/// value-position receivers: the armed declared-instance locator takes
/// precedence, then the ledger's exact receiver value, then the existing
/// bound/static fallback.  `receiver_input` is computed lazily so the
/// locator-backed lane never consumes the receiver source site twice.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::builder) fn me_this_method_call_effect<'input, P>(
    port: &P,
    receiver_ast: &ASTNode,
    receiver_input: impl FnOnce() -> Result<P::ExprInput<'input>, String>,
    builder: &MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    call_input: &P::ExprInput<'input>,
    method: &str,
    args: Vec<ValueId>,
    arity: usize,
    dst: Option<ValueId>,
    source: CoreCallSourceV1,
    error_prefix: &str,
    missing_me_error: String,
    missing_this_error: String,
) -> Result<CoreEffectPlan, String>
where
    P: LoopPlanExpressionPortV1 + ?Sized,
{
    if matches!(receiver_ast, ASTNode::Me { .. }) {
        if let Some(plan) = declared_instance_call_effect(
            port,
            call_input,
            method,
            arity as u32,
            dst,
            args.clone(),
            source.clone(),
            error_prefix,
        )? {
            return Ok(plan);
        }
    }
    if let Some(object_id) = port
        .exact_source_receiver_value(&receiver_input()?)
        .map_err(|error| format!("{error_prefix}: {error}"))?
    {
        return Ok(CoreEffectPlan::MethodCall {
            source,
            dst,
            object: object_id,
            method: method.to_owned(),
            args,
            effects: EffectMask::PURE.add(Effect::Io),
        });
    }
    lower_me_this_method_effect(
        builder,
        phi_bindings,
        receiver_ast,
        source,
        method,
        args,
        arity,
        dst,
        missing_me_error,
        missing_this_error,
    )
}
