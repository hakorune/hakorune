//! Phase 273 P3: Plan Lowering - Non-loop plan lowering (Seq/If/BranchN)
//!
//! # Responsibilities
//!
//! - Lower Seq plans (sequential execution)
//! - Lower If plans (conditional branching with joins)
//! - Lower BranchN plans (multi-way branching → If chain)
//!
//! # Design
//!
//! - Recursive calls to lower_with_stack() for nested plans
//! - Join emission for If merge points
//! - BranchN is rewritten to nested If chain

use super::LoopFrame;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::{
    CoreBranchNPlan, CoreEffectPlan, CoreIfPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

impl super::PlanLowerer {
    fn lower_plan_list(
        builder: &mut MirBuilder,
        plans: &[LoweredRecipe],
        ctx: &LoopRouteContext,
        loop_stack: &mut Vec<LoopFrame>,
        list_ctx: &'static str,
    ) -> Result<Option<ValueId>, String> {
        let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
            && crate::config::env::joinir_dev::planner_required_enabled();

        let mut result = None;
        for (idx, plan) in plans.iter().enumerate() {
            if strict_planner_required {
                // Seq-level forward-ref check: if an operand is undefined "so far" but is defined later in the
                // same plan list, fail-fast close to the ordering bug.
                if let CorePlan::Effect(CoreEffectPlan::BinOp { dst, lhs, op, rhs }) = plan {
                    if let Some(func) = builder.scope_ctx.current_function.as_ref() {
                        let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
                        if !def_blocks.contains_key(lhs) {
                            let origin_span = builder
                                .metadata_ctx
                                .value_span(*lhs)
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "unknown".to_string());
                            if let Some((def_idx, def_kind)) =
                                find_forward_def_in_seq(&plans[(idx + 1)..], *lhs)
                            {
                                return Err(format!(
                                    "[freeze:contract][plan_lowering/seq_forward_ref] fn={} bb={:?} ctx={} use_idx={} use=%{} operand=lhs use_by=CoreEffectPlan::BinOp dst=%{} op={:?} def_idx={} def_kind={} use_origin_span={}",
                                    func.signature.name,
                                    builder.current_block,
                                    list_ctx,
                                    idx,
                                    lhs.0,
                                    dst.0,
                                    op,
                                    idx + 1 + def_idx,
                                    def_kind,
                                    origin_span
                                ));
                            }
                            let debug_suffix = if crate::config::env::joinir_dev::debug_enabled() {
                                let (const_dsts, const_origin_spans, has_use_const, add_binops) =
                                    collect_seq_debug(&builder.metadata_ctx, plans, *lhs);
                                format!(
                                    " list_const_int3_dsts=[{}] list_const_int3_origin_spans=[{}] list_has_use_const={} list_add_binops=[{}]",
                                    const_dsts,
                                    const_origin_spans,
                                    if has_use_const { "yes" } else { "no" },
                                    add_binops
                                )
                            } else {
                                String::new()
                            };
                            return Err(format!(
                                "[freeze:contract][plan_lowering/seq_undefined_operand] fn={} bb={:?} ctx={} use_idx={} use=%{} operand=lhs use_by=CoreEffectPlan::BinOp dst=%{} op={:?} use_origin_span={}{}",
                                func.signature.name,
                                builder.current_block,
                                list_ctx,
                                idx,
                                lhs.0,
                                dst.0,
                                op,
                                origin_span,
                                debug_suffix
                            ));
                        }
                        if !def_blocks.contains_key(rhs) {
                            let origin_span = builder
                                .metadata_ctx
                                .value_span(*rhs)
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "unknown".to_string());
                            if let Some((def_idx, def_kind)) =
                                find_forward_def_in_seq(&plans[(idx + 1)..], *rhs)
                            {
                                return Err(format!(
                                    "[freeze:contract][plan_lowering/seq_forward_ref] fn={} bb={:?} ctx={} use_idx={} use=%{} operand=rhs use_by=CoreEffectPlan::BinOp dst=%{} op={:?} def_idx={} def_kind={} use_origin_span={}",
                                    func.signature.name,
                                    builder.current_block,
                                    list_ctx,
                                    idx,
                                    rhs.0,
                                    dst.0,
                                    op,
                                    idx + 1 + def_idx,
                                    def_kind,
                                    origin_span
                                ));
                            }
                            let debug_suffix = if crate::config::env::joinir_dev::debug_enabled() {
                                let (const_dsts, const_origin_spans, has_use_const, add_binops) =
                                    collect_seq_debug(&builder.metadata_ctx, plans, *rhs);
                                format!(
                                    " list_const_int3_dsts=[{}] list_const_int3_origin_spans=[{}] list_has_use_const={} list_add_binops=[{}]",
                                    const_dsts,
                                    const_origin_spans,
                                    if has_use_const { "yes" } else { "no" },
                                    add_binops
                                )
                            } else {
                                String::new()
                            };
                            return Err(format!(
                                "[freeze:contract][plan_lowering/seq_undefined_operand] fn={} bb={:?} ctx={} use_idx={} use=%{} operand=rhs use_by=CoreEffectPlan::BinOp dst=%{} op={:?} use_origin_span={}{}",
                                func.signature.name,
                                builder.current_block,
                                list_ctx,
                                idx,
                                rhs.0,
                                dst.0,
                                op,
                                origin_span,
                                debug_suffix
                            ));
                        }
                    }
                }
            }

            result = Self::lower_with_stack(builder, plan.clone(), ctx, loop_stack)?;
            if builder.is_current_block_terminated() {
                break;
            }
        }
        Ok(result)
    }

    /// Seq: process plans in order
    pub(super) fn lower_seq(
        builder: &mut MirBuilder,
        plans: Vec<LoweredRecipe>,
        ctx: &LoopRouteContext,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        Self::lower_plan_list(builder, &plans, ctx, loop_stack, "seq")
    }

    /// If: emit Branch and lower then/else plans (standalone)
    pub(super) fn lower_if(
        builder: &mut MirBuilder,
        if_plan: CoreIfPlan,
        ctx: &LoopRouteContext,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::builder::emission::branch::{emit_conditional, emit_jump};

        let CoreIfPlan {
            condition,
            then_plans,
            else_plans,
            joins,
        } = if_plan;

        let _pre_branch_bb = builder
            .current_block
            .ok_or_else(|| "[lowerer] No current block for CorePlan::If".to_string())?;

        let then_bb = builder.next_block_id();
        let else_bb = builder.next_block_id();
        let merge_bb = builder.next_block_id();

        builder.ensure_block_exists(then_bb)?;
        builder.ensure_block_exists(else_bb)?;
        builder.ensure_block_exists(merge_bb)?;

        let mut condition_val = condition;
        crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut condition_val)?;
        emit_conditional(builder, condition_val, then_bb, else_bb)?;

        // then
        builder.start_new_block(then_bb)?;
        Self::lower_plan_list(builder, &then_plans, ctx, loop_stack, "if_then")?;
        let then_reaches_merge = !builder.is_current_block_terminated();
        let then_end_bb = builder.current_block;
        if then_reaches_merge {
            emit_jump(builder, merge_bb)?;
        }

        // else
        builder.start_new_block(else_bb)?;
        if let Some(else_plans) = else_plans.as_ref() {
            Self::lower_plan_list(builder, else_plans, ctx, loop_stack, "if_else")?;
        }
        let else_reaches_merge = !builder.is_current_block_terminated();
        let else_end_bb = builder.current_block;
        if else_reaches_merge {
            emit_jump(builder, merge_bb)?;
        }

        // merge (may be unreachable if both branches terminate)
        builder.start_new_block(merge_bb)?;
        super::super::features::if_join::apply_if_joins(
            builder,
            &joins,
            then_reaches_merge,
            else_reaches_merge,
            then_end_bb,
            else_end_bb,
        )?;
        Ok(None)
    }

    /// BranchN: rewrite into nested If chain and reuse lower_if.
    pub(super) fn lower_branchn(
        builder: &mut MirBuilder,
        branch_plan: CoreBranchNPlan,
        ctx: &LoopRouteContext,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        let if_chain = super::super::branchn::branchn_to_if_chain(branch_plan)?;
        Self::lower_with_stack(builder, if_chain, ctx, loop_stack)
    }
}

fn effect_defined_value(effect: &CoreEffectPlan) -> Option<(ValueId, &'static str)> {
    match effect {
        CoreEffectPlan::MethodCall { dst: Some(v), .. } => Some((*v, "MethodCall")),
        CoreEffectPlan::MethodCall { dst: None, .. } => None,
        CoreEffectPlan::GlobalCall { dst: Some(v), .. } => Some((*v, "GlobalCall")),
        CoreEffectPlan::GlobalCall { dst: None, .. } => None,
        CoreEffectPlan::ValueCall { dst: Some(v), .. } => Some((*v, "ValueCall")),
        CoreEffectPlan::ValueCall { dst: None, .. } => None,
        CoreEffectPlan::ExternCall { dst: Some(v), .. } => Some((*v, "ExternCall")),
        CoreEffectPlan::ExternCall { dst: None, .. } => None,
        CoreEffectPlan::NewBox { dst, .. } => Some((*dst, "NewBox")),
        CoreEffectPlan::BinOp { dst, .. } => Some((*dst, "BinOp")),
        CoreEffectPlan::Compare { dst, .. } => Some((*dst, "Compare")),
        CoreEffectPlan::Select { dst, .. } => Some((*dst, "Select")),
        CoreEffectPlan::Const { dst, .. } => Some((*dst, "Const")),
        CoreEffectPlan::Copy { dst, .. } => Some((*dst, "Copy")),
        CoreEffectPlan::ExitIf { .. } | CoreEffectPlan::IfEffect { .. } => None,
    }
}

fn find_forward_def_in_seq(
    plans: &[LoweredRecipe],
    target: ValueId,
) -> Option<(usize, &'static str)> {
    for (idx, plan) in plans.iter().enumerate() {
        if let CorePlan::Effect(effect) = plan {
            if let Some((def_value, def_kind)) = effect_defined_value(effect) {
                if def_value == target {
                    return Some((idx, def_kind));
                }
            }
        }
    }
    None
}

fn collect_seq_debug(
    metadata_ctx: &crate::mir::builder::metadata_context::MetadataContext<
        crate::ast::Span,
        crate::mir::region::RegionId,
    >,
    plans: &[LoweredRecipe],
    use_value: ValueId,
) -> (String, String, bool, String) {
    let mut const_dsts = Vec::new();
    let mut const_origin_spans = Vec::new();
    let mut has_use_const = false;
    let mut add_binops = Vec::new();
    for plan in plans {
        if let CorePlan::Effect(effect) = plan {
            match effect {
                CoreEffectPlan::Const { dst, value } => {
                    if matches!(value, crate::mir::ConstValue::Integer(3)) {
                        const_dsts.push(*dst);
                        if const_origin_spans.len() < 3 {
                            let origin = metadata_ctx
                                .value_span(*dst)
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "unknown".to_string());
                            const_origin_spans.push(origin);
                        }
                        if *dst == use_value {
                            has_use_const = true;
                        }
                    }
                }
                CoreEffectPlan::BinOp { dst, lhs, op, rhs } => {
                    if *op == crate::mir::BinaryOp::Add && add_binops.len() < 2 {
                        add_binops.push(format!("dst=%{} lhs=%{} rhs=%{}", dst.0, lhs.0, rhs.0));
                    }
                }
                _ => {}
            }
        }
    }
    let const_str = const_dsts
        .iter()
        .map(|v| format!("%{}", v.0))
        .collect::<Vec<_>>()
        .join(",");
    let const_origin_str = const_origin_spans.join(";");
    let add_str = add_binops.join(";");
    (const_str, const_origin_str, has_use_const, add_str)
}
