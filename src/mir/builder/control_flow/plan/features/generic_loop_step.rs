//! GenericLoop step/cond feature (apply-only).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalizer::lower_loop_header_cond;
use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::skeletons::generic_loop::GenericLoopSkeleton;
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::MirBuilder;
use crate::mir::BasicBlockId;

pub(in crate::mir::builder) fn apply_generic_loop_condition(
    builder: &mut MirBuilder,
    skeleton: &mut GenericLoopSkeleton,
    condition: &ASTNode,
    loop_var: &str,
    error_prefix: &str,
) -> Result<(), String> {
    let phi_bindings = loop_carriers::build_loop_bindings(&[(loop_var, skeleton.loop_var_current)]);
    let cond_view = CondBlockView::from_expr(condition);

    // Phase 2b-2: Extract EdgeArgs from existing header branch (SSOT継承)
    let header_branch = skeleton
        .plan
        .frag
        .branches
        .iter()
        .find(|b| b.from == skeleton.plan.header_bb)
        .ok_or_else(|| format!("{error_prefix}: missing header branch"))?;
    let body_args = header_branch.then_args.clone();
    let after_args = header_branch.else_args.clone();

    // Phase 2b-2: Use short-circuit evaluation
    let header_result = lower_loop_header_cond(
        builder,
        &phi_bindings,
        &cond_view,
        skeleton.plan.header_bb,
        skeleton.plan.body_bb,
        skeleton.plan.after_bb,
        body_args,
        after_args,
        error_prefix,
    )?;

    // Merge block_effects (insert可: intermediate BB は新規追加)
    for (bb, effects) in header_result.block_effects {
        if let Some((_, existing)) = skeleton.plan.block_effects.iter_mut().find(|(b, _)| *b == bb) {
            existing.extend(effects);
        } else {
            skeleton.plan.block_effects.push((bb, effects));
        }
    }

    // Update cond_loop/cond_match (token として first_cond を使用)
    skeleton.plan.cond_loop = header_result.first_cond;
    skeleton.plan.cond_match = header_result.first_cond;

    // Replace branches: remove header branch, add short-circuit branches
    skeleton.plan.frag.branches.retain(|b| b.from != skeleton.plan.header_bb);
    skeleton.plan.frag.branches.extend(header_result.branches);

    Ok(())
}

pub(in crate::mir::builder) fn apply_generic_loop_step(
    builder: &mut MirBuilder,
    skeleton: &mut GenericLoopSkeleton,
    loop_increment: &ASTNode,
    loop_var: &str,
    error_prefix: &str,
) -> Result<(), String> {
    let phi_bindings = loop_carriers::build_loop_bindings(&[(loop_var, skeleton.loop_var_current)]);
    let step_effects = match loop_increment {
        ASTNode::Variable { .. } => {
            let (step_val, mut effects) =
                PlanNormalizer::lower_value_ast(loop_increment, builder, &phi_bindings)?;
            effects.push(CoreEffectPlan::Copy {
                dst: skeleton.loop_var_next,
                src: step_val,
            });
            effects
        }
        _ => {
            let (loop_inc_lhs, loop_inc_op, loop_inc_rhs, loop_inc_consts) =
                PlanNormalizer::lower_binop_ast(loop_increment, builder, &phi_bindings)?;
            let mut effects = loop_inc_consts;
            effects.push(CoreEffectPlan::BinOp {
                dst: skeleton.loop_var_next,
                lhs: loop_inc_lhs,
                op: loop_inc_op,
                rhs: loop_inc_rhs,
            });
            effects
        }
    };
    append_block_effects(
        &mut skeleton.plan.block_effects,
        skeleton.plan.step_bb,
        step_effects,
        error_prefix,
    )?;

    skeleton.plan.phis.push(loop_carriers::build_loop_phi_info(
        skeleton.plan.header_bb,
        skeleton.plan.preheader_bb,
        skeleton.plan.step_bb,
        skeleton.loop_var_current,
        skeleton.loop_var_init,
        skeleton.loop_var_next,
        format!("loop_var_{loop_var}"),
    ));

    Ok(())
}

fn append_block_effects(
    block_effects: &mut Vec<(BasicBlockId, Vec<CoreEffectPlan>)>,
    block: BasicBlockId,
    mut effects: Vec<CoreEffectPlan>,
    error_prefix: &str,
) -> Result<(), String> {
    for (bb, bb_effects) in block_effects.iter_mut() {
        if *bb == block {
            bb_effects.append(&mut effects);
            return Ok(());
        }
    }
    Err(format!(
        "{error_prefix}: missing block effects entry for {block:?}"
    ))
}
