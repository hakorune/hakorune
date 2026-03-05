//! Phase 29bq P2.x: LoopCondReturnInBody Pipeline
//!
//! Minimal implementation for loop(cond) with nested return and no break/continue.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::features::carrier_merge::{
    lower_assignment_stmt, lower_local_init_stmt,
};
use crate::mir::builder::control_flow::plan::features::carriers;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::if_branch_lowering;
use crate::mir::builder::control_flow::plan::features::loop_carriers;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::loop_cond::return_in_body_facts::LoopCondReturnInBodyFacts;
use crate::mir::builder::control_flow::plan::loop_cond::return_in_body_recipe::{
    LoopCondReturnInBodyItem, LoopCondReturnInBodyRecipe,
};
use crate::mir::builder::control_flow::plan::normalizer::{
    helpers::LoopBlocksStandard5, loop_body_lowering, lower_loop_header_cond, PlanNormalizer,
};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{Effect, EffectMask, MirType};
use std::collections::BTreeMap;

const LOOP_COND_RETURN_IN_BODY_ERR: &str = "[normalizer] loop_cond_return_in_body";

pub(in crate::mir::builder) fn lower_loop_cond_return_in_body(
    builder: &mut MirBuilder,
    facts: LoopCondReturnInBodyFacts,
    _ctx: &LoopRouteContext,
) -> Result<LoweredRecipe, String> {
    let blocks = LoopBlocksStandard5::allocate(builder)?;
    let LoopBlocksStandard5 {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
    } = blocks;

    let carrier_sets = carriers::collect_from_body(&facts.recipe.body.body);
    let carrier_vars = carrier_sets.vars;

    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    let use_header_continue_target =
        planner_required && crate::config::env::joinir_dev::strict_enabled();

    let mut carrier_inits = BTreeMap::new();
    let mut carrier_phis = BTreeMap::new();
    let mut carrier_step_phis = BTreeMap::new();
    for var in &carrier_vars {
        let Some(&init_val) = builder.variable_ctx.variable_map.get(var) else {
            return Err(format!(
                "{LOOP_COND_RETURN_IN_BODY_ERR}: carrier {} missing init",
                var
            ));
        };
        let ty = builder
            .type_ctx
            .get_type(init_val)
            .cloned()
            .unwrap_or(MirType::Unknown);
        let phi_dst = builder.alloc_typed(ty.clone());
        let step_phi_dst = if use_header_continue_target {
            phi_dst
        } else {
            builder.alloc_typed(ty)
        };
        carrier_inits.insert(var.clone(), init_val);
        carrier_phis.insert(var.clone(), phi_dst);
        carrier_step_phis.insert(var.clone(), step_phi_dst);
    }
    if carrier_phis.is_empty() {
        return Err(format!("{LOOP_COND_RETURN_IN_BODY_ERR}: no loop carriers"));
    }

    let mut current_bindings = carrier_phis.clone();
    for (name, value_id) in &current_bindings {
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
    }

    // Phase 2b-3: Short-circuit evaluation for loop header condition
    let cond_view = CondBlockView::from_expr(&facts.condition);
    let header_result = lower_loop_header_cond(
        builder,
        &current_bindings,
        &cond_view,
        header_bb,
        body_bb,
        after_bb,
        empty_carriers_args(),
        empty_carriers_args(),
        LOOP_COND_RETURN_IN_BODY_ERR,
    )?;

    let wires = vec![
        edgecfg_stubs::build_loop_back_edge(body_bb, step_bb),
        edgecfg_stubs::build_loop_back_edge(step_bb, header_bb),
    ];

    let frag = Frag {
        entry: header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires,
        branches: header_result.branches,
    };

    let mut carrier_updates = BTreeMap::new();
    let mut body_plans = lower_return_in_body_block(
        builder,
        &mut current_bindings,
        &carrier_phis,
        &carrier_step_phis,
        &mut carrier_updates,
        &facts.recipe,
    )?;

    body_plans.push(CorePlan::Exit(parts::exit::build_continue_with_phi_args(
        builder,
        &carrier_step_phis,
        &current_bindings,
        LOOP_COND_RETURN_IN_BODY_ERR,
    )?));

    let mut phis = Vec::new();
    let mut final_values = Vec::new();
    for (var, header_phi_dst) in &carrier_phis {
        let init_val = match carrier_inits.get(var) {
            Some(value) => *value,
            None => continue,
        };
        if use_header_continue_target {
            phis.push(loop_carriers::build_preheader_only_phi_info(
                header_bb,
                preheader_bb,
                *header_phi_dst,
                init_val,
                format!("loop_cond_return_in_body_carrier_{}", var),
            ));
        } else {
            let Some(step_phi_dst) = carrier_step_phis.get(var).copied() else {
                continue;
            };
            phis.push(loop_carriers::build_step_join_phi_info(
                step_bb,
                step_phi_dst,
                format!("loop_cond_return_in_body_step_join_{}", var),
            ));
            phis.push(loop_carriers::build_loop_phi_info(
                header_bb,
                preheader_bb,
                step_bb,
                *header_phi_dst,
                init_val,
                step_phi_dst,
                format!("loop_cond_return_in_body_carrier_{}", var),
            ));
        }
        final_values.push((var.clone(), *header_phi_dst));
    }

    // Build block_effects: merge header_result.block_effects + static entries
    let mut block_effects: Vec<(crate::mir::BasicBlockId, Vec<CoreEffectPlan>)> =
        vec![(preheader_bb, vec![])];
    for (bb, effects) in header_result.block_effects {
        block_effects.push((bb, effects));
    }
    block_effects.push((body_bb, vec![]));
    block_effects.push((step_bb, vec![]));

    let continue_target = if use_header_continue_target {
        header_bb
    } else {
        step_bb
    };

    let (step_mode, has_explicit_step) = step_mode::inline_in_body_no_explicit_step();

    Ok(CorePlan::Loop(CoreLoopPlan {
        preheader_bb,
        preheader_is_fresh: false,
        header_bb,
        body_bb,
        step_bb,
        continue_target,
        after_bb,
        found_bb: after_bb,
        body: body_plans,
        cond_loop: header_result.first_cond,
        cond_match: header_result.first_cond,
        block_effects,
        phis,
        frag,
        final_values,
        step_mode,
        has_explicit_step,
    }))
}

fn get_stmt<'a>(body: &'a RecipeBody, stmt_ref: StmtRef) -> Result<&'a ASTNode, String> {
    body.get_ref(stmt_ref).ok_or_else(|| {
        format!(
            "{LOOP_COND_RETURN_IN_BODY_ERR}: missing stmt idx={}",
            stmt_ref.index()
        )
    })
}

fn lower_return_in_body_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    recipe: &LoopCondReturnInBodyRecipe,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut plans = Vec::new();
    for item in &recipe.items {
        let LoopCondReturnInBodyItem::Stmt(stmt_ref) = item;
        let stmt = get_stmt(&recipe.body, *stmt_ref)?;
        plans.extend(lower_stmt_ast(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            carrier_updates,
            stmt,
        )?);
    }
    Ok(plans)
}

fn lower_stmt_ast(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
) -> Result<Vec<LoweredRecipe>, String> {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            let effects = lower_assignment_stmt(
                builder,
                current_bindings,
                carrier_phis,
                carrier_updates,
                target,
                value,
                LOOP_COND_RETURN_IN_BODY_ERR,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            let effects = lower_local_init_stmt(
                builder,
                current_bindings,
                variables,
                initial_values,
                LOOP_COND_RETURN_IN_BODY_ERR,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::MethodCall { .. } => {
            let effects = loop_body_lowering::lower_method_call_stmt(
                builder,
                current_bindings,
                stmt,
                LOOP_COND_RETURN_IN_BODY_ERR,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::FunctionCall { .. } => {
            let effects = loop_body_lowering::lower_function_call_stmt(
                builder,
                current_bindings,
                stmt,
                LOOP_COND_RETURN_IN_BODY_ERR,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::Print { expression, .. } => {
            let (value_id, mut effects) =
                PlanNormalizer::lower_value_ast(expression, builder, current_bindings)?;
            effects.push(CoreEffectPlan::ExternCall {
                dst: None,
                iface_name: "env.console".to_string(),
                method_name: "log".to_string(),
                args: vec![value_id],
                effects: EffectMask::PURE.add(Effect::Io),
            });
            Ok(effects_to_plans(effects))
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => lower_if_with_join(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            carrier_updates,
            condition,
            then_body,
            else_body.as_ref(),
        ),
        ASTNode::Return { value, .. } => parts::entry::lower_return_with_effects(
            builder,
            value.as_deref(),
            current_bindings,
            LOOP_COND_RETURN_IN_BODY_ERR,
        ),
        _ => Err(format!(
            "{LOOP_COND_RETURN_IN_BODY_ERR}: unsupported stmt {:?}",
            stmt
        )),
    }
}

fn lower_if_with_join(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
) -> Result<Vec<LoweredRecipe>, String> {
    let plans = if_branch_lowering::lower_if_with_branch_lowerers_and_updates(
        builder,
        current_bindings,
        carrier_phis,
        carrier_updates,
        condition,
        then_body,
        else_body.map(|body| body.as_slice()),
        LOOP_COND_RETURN_IN_BODY_ERR,
        |builder, bindings, carrier_updates, stmt| {
            lower_stmt_ast(
                builder,
                bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                stmt,
            )
        },
    )?;

    {
        for join in plans.iter().flat_map(|plan| match plan {
            CorePlan::If(if_plan) => if_plan.joins.iter(),
            _ => [].iter(),
        }) {
            if carrier_phis.contains_key(&join.name) {
                carrier_updates.insert(join.name.clone(), join.dst);
            }
        }
    }
    Ok(plans)
}
