//! Phase 29bq P2.x: LoopCondReturnInBody Pipeline
//!
//! Minimal implementation for loop(cond) with nested return and no break/continue.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::edgecfg::api::Frag;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::loop_cond_return_in_body::LoopCondReturnInBodyFacts;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::carrier_merge::{
    lower_assignment_stmt, lower_local_init_stmt,
};
use crate::mir::builder::control_flow::plan::features::carriers;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::if_branch_lowering;
use crate::mir::builder::control_flow::plan::features::loop_cond_return_in_body_cleanup::{
    apply_fallthrough_continue_exit, body_exits_all_paths,
};
use crate::mir::builder::control_flow::plan::features::loop_cond_return_in_body_phi_materializer::LoopCondReturnInBodyPhiMaterializer;
use crate::mir::builder::control_flow::plan::features::loop_cond_return_in_body_verifier::verify_loop_cond_return_in_body_phi_closure;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::normalizer::cond_lowering_loop_header::lower_loop_header_cond;
use crate::mir::builder::control_flow::plan::normalizer::{
    helpers::LoopBlocksStandard5, loop_body_lowering, PlanNormalizer,
};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::control_flow::recipes::loop_cond_return_in_body::{
    LoopCondReturnInBodyItem, LoopCondReturnInBodyRecipe,
};
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::builder::MirBuilder;
use crate::mir::{Effect, EffectMask};
use std::collections::{BTreeMap, BTreeSet};

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
    let mut carrier_vars = carrier_sets.vars;
    if carrier_vars.is_empty() {
        carrier_vars = collect_carrier_vars_from_condition(builder, &facts.condition);
    }
    if carrier_vars.is_empty() {
        return Err(format!("{LOOP_COND_RETURN_IN_BODY_ERR}: no loop carriers"));
    }

    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    let use_header_continue_target =
        planner_required && crate::config::env::joinir_dev::strict_enabled();

    let mut phi_materializer = LoopCondReturnInBodyPhiMaterializer::prepare(
        builder,
        &carrier_vars,
        use_header_continue_target,
        header_bb,
        step_bb,
        LOOP_COND_RETURN_IN_BODY_ERR,
    )?;

    // Phase 2b-3: Short-circuit evaluation for loop header condition
    let cond_view = CondBlockView::from_expr(&facts.condition);
    let header_result = lower_loop_header_cond(
        builder,
        phi_materializer.current_bindings(),
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

    let carrier_phis = phi_materializer.carrier_phis().clone();
    let carrier_step_phis = phi_materializer.carrier_step_phis().clone();
    let mut carrier_updates = BTreeMap::new();
    let mut body_plans = lower_return_in_body_block(
        builder,
        phi_materializer.current_bindings_mut(),
        &carrier_phis,
        &carrier_step_phis,
        &mut carrier_updates,
        &facts.recipe,
    )?;

    let body_exits_all_paths = body_exits_all_paths(&body_plans);
    let continue_target = if use_header_continue_target {
        header_bb
    } else {
        step_bb
    };
    let phi_closure = phi_materializer.close(
        header_bb,
        preheader_bb,
        step_bb,
        use_header_continue_target,
        body_exits_all_paths,
        LOOP_COND_RETURN_IN_BODY_ERR,
    )?;
    verify_loop_cond_return_in_body_phi_closure(
        &phi_closure,
        continue_target,
        use_header_continue_target,
        body_exits_all_paths,
        carrier_vars.len(),
        LOOP_COND_RETURN_IN_BODY_ERR,
    )?;
    apply_fallthrough_continue_exit(&mut body_plans, &phi_closure);

    // Build block_effects: merge header_result.block_effects + static entries
    let mut block_effects: Vec<(crate::mir::BasicBlockId, Vec<CoreEffectPlan>)> =
        vec![(preheader_bb, vec![])];
    for (bb, effects) in header_result.block_effects {
        block_effects.push((bb, effects));
    }
    block_effects.push((body_bb, vec![]));
    block_effects.push((step_bb, vec![]));

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
        phis: phi_closure.phis().to_vec(),
        frag,
        final_values: phi_closure.final_values().to_vec(),
        step_mode,
        has_explicit_step,
    }))
}

fn collect_carrier_vars_from_condition(builder: &MirBuilder, condition: &ASTNode) -> Vec<String> {
    let mut vars = BTreeSet::<String>::new();
    collect_vars_from_expr(condition, &mut vars);

    let mut carriers = BTreeMap::<String, ()>::new();
    for name in vars {
        if builder.variable_ctx.variable_map.contains_key(&name) {
            carriers.insert(name, ());
        }
    }
    carriers.keys().cloned().collect()
}

fn collect_vars_from_expr(ast: &ASTNode, vars: &mut BTreeSet<String>) {
    match ast {
        ASTNode::Variable { name, .. } => {
            vars.insert(name.clone());
        }
        ASTNode::Literal { .. } => {}
        ASTNode::UnaryOp { operand, .. } => collect_vars_from_expr(operand, vars),
        ASTNode::BinaryOp { left, right, .. } => {
            collect_vars_from_expr(left, vars);
            collect_vars_from_expr(right, vars);
        }
        ASTNode::GroupedAssignmentExpr { rhs, .. } => collect_vars_from_expr(rhs, vars),
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            collect_vars_from_expr(object, vars);
            for arg in arguments {
                collect_vars_from_expr(arg, vars);
            }
        }
        ASTNode::FunctionCall { arguments, .. } => {
            for arg in arguments {
                collect_vars_from_expr(arg, vars);
            }
        }
        ASTNode::Call {
            callee, arguments, ..
        } => {
            collect_vars_from_expr(callee, vars);
            for arg in arguments {
                collect_vars_from_expr(arg, vars);
            }
        }
        ASTNode::Index { target, index, .. } => {
            collect_vars_from_expr(target, vars);
            collect_vars_from_expr(index, vars);
        }
        ASTNode::FieldAccess { object, .. } => collect_vars_from_expr(object, vars),
        ASTNode::ArrayLiteral { elements, .. } => {
            for el in elements {
                collect_vars_from_expr(el, vars);
            }
        }
        ASTNode::MapLiteral { entries, .. } => {
            for (_, value) in entries {
                collect_vars_from_expr(value, vars);
            }
        }
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => {
            for stmt in prelude_stmts {
                collect_vars_from_stmt(stmt, vars);
            }
            collect_vars_from_expr(tail_expr, vars);
        }
        _ => {}
    }
}

fn collect_vars_from_stmt(stmt: &ASTNode, vars: &mut BTreeSet<String>) {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            collect_vars_from_expr(target, vars);
            collect_vars_from_expr(value, vars);
        }
        ASTNode::Local { initial_values, .. } => {
            for init in initial_values.iter().flatten() {
                collect_vars_from_expr(init, vars);
            }
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            collect_vars_from_expr(condition, vars);
            for child in then_body {
                collect_vars_from_stmt(child, vars);
            }
            if let Some(else_body) = else_body {
                for child in else_body {
                    collect_vars_from_stmt(child, vars);
                }
            }
        }
        ASTNode::Loop {
            condition, body, ..
        }
        | ASTNode::While {
            condition, body, ..
        } => {
            collect_vars_from_expr(condition, vars);
            for child in body {
                collect_vars_from_stmt(child, vars);
            }
        }
        ASTNode::ForRange {
            start, end, body, ..
        } => {
            collect_vars_from_expr(start, vars);
            collect_vars_from_expr(end, vars);
            for child in body {
                collect_vars_from_stmt(child, vars);
            }
        }
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            collect_vars_from_expr(object, vars);
            for arg in arguments {
                collect_vars_from_expr(arg, vars);
            }
        }
        ASTNode::FunctionCall { arguments, .. } => {
            for arg in arguments {
                collect_vars_from_expr(arg, vars);
            }
        }
        ASTNode::Call {
            callee, arguments, ..
        } => {
            collect_vars_from_expr(callee, vars);
            for arg in arguments {
                collect_vars_from_expr(arg, vars);
            }
        }
        ASTNode::Print { expression, .. } => collect_vars_from_expr(expression, vars),
        ASTNode::Return { value, .. } => {
            if let Some(value) = value {
                collect_vars_from_expr(value, vars);
            }
        }
        ASTNode::Program { statements, .. }
        | ASTNode::ScopeBox {
            body: statements, ..
        } => {
            for child in statements {
                collect_vars_from_stmt(child, vars);
            }
        }
        _ => {}
    }
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
