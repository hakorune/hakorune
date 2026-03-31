use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{BinaryOp, ConstValue, ValueId};

pub(super) fn debug_log_block_effects_binop_lit3(builder: &MirBuilder, effects: &[CoreEffectPlan]) {
    if !joinir_dev::strict_planner_required_debug_enabled() {
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
        "[loop/block_effects:binop_lit3] fn={} bb={:?} effects_len={} const_int3_dsts=[{}] add_binops=[dst=%{} lhs=%{} rhs=%{}]",
        fn_name,
        builder.current_block,
        effects.len(),
        const_int3_dsts,
        dst.0,
        lhs.0,
        rhs.0
    ));
}

