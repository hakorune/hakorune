use crate::ast::ASTNode;
use crate::mir::EdgeArgs;
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
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
    method: &str,
    args: Vec<ValueId>,
    arity: usize,
    dst: Option<ValueId>,
    missing_me_error: String,
    missing_this_error: String,
) -> Result<CoreEffectPlan, String> {
    let bound_me = phi_bindings
        .get("me")
        .copied()
        .or_else(|| builder.variable_ctx.variable_map.get("me").copied());

    match receiver {
        ASTNode::Me { .. } => {
            if let Some(object_id) = bound_me {
                Ok(CoreEffectPlan::MethodCall {
                    dst,
                    object: object_id,
                    method: method.to_string(),
                    args,
                    effects: EffectMask::PURE.add(Effect::Io),
                })
            } else if let Some(box_name) = builder.comp_ctx.current_static_box.as_deref() {
                Ok(CoreEffectPlan::GlobalCall {
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
                    dst,
                    func: format!("{}.{}/{}", box_name, method, arity),
                    args,
                })
            } else if let Some(object_id) = bound_me {
                Ok(CoreEffectPlan::MethodCall {
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
