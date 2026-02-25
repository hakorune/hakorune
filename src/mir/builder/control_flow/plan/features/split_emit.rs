//! SplitScan emit helpers (delimiter match + emit effects).

use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::{BinaryOp, CompareOp, Effect, EffectMask, ValueId};

pub(in crate::mir::builder) fn build_match_body(
    s_host: ValueId,
    sep_host: ValueId,
    i_current: ValueId,
    sep_len: ValueId,
    i_plus_sep: ValueId,
    chunk: ValueId,
    cond_match: ValueId,
) -> Vec<LoweredRecipe> {
    vec![
        CorePlan::Effect(CoreEffectPlan::BinOp {
            dst: i_plus_sep,
            lhs: i_current,
            op: BinaryOp::Add,
            rhs: sep_len,
        }),
        CorePlan::Effect(CoreEffectPlan::MethodCall {
            dst: Some(chunk),
            object: s_host,
            method: "substring".to_string(),
            args: vec![i_current, i_plus_sep],
            effects: EffectMask::PURE.add(Effect::Io),
        }),
        CorePlan::Effect(CoreEffectPlan::Compare {
            dst: cond_match,
            lhs: chunk,
            op: CompareOp::Eq,
            rhs: sep_host,
        }),
    ]
}

pub(in crate::mir::builder) fn build_then_effects(
    s_host: ValueId,
    result_host: ValueId,
    start_current: ValueId,
    i_current: ValueId,
    sep_len: ValueId,
    segment: ValueId,
    start_next_then: ValueId,
) -> Vec<CoreEffectPlan> {
    vec![
        CoreEffectPlan::MethodCall {
            dst: Some(segment),
            object: s_host,
            method: "substring".to_string(),
            args: vec![start_current, i_current],
            effects: EffectMask::PURE.add(Effect::Io),
        },
        CoreEffectPlan::MethodCall {
            dst: None,
            object: result_host,
            method: "push".to_string(),
            args: vec![segment],
            effects: EffectMask::MUT,
        },
        CoreEffectPlan::BinOp {
            dst: start_next_then,
            lhs: i_current,
            op: BinaryOp::Add,
            rhs: sep_len,
        },
    ]
}
