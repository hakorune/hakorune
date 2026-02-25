//! Phase 29bq P2.x: LoopCondContinueWithReturn Pipeline
//!
//! Minimal implementation for continue-only loops with nested return.
//! Reuses exit_if_map (return PHI) + continue-if handling (continue PHI).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::plan::features::carrier_merge::{
    lower_assignment_stmt, lower_local_init_stmt,
};
use crate::mir::builder::control_flow::plan::features::carriers;
use crate::mir::builder::control_flow::plan::features::coreloop_frame::{
    build_coreloop_frame, build_header_step_phis,
};
use crate::mir::builder::control_flow::plan::features::if_branch_lowering;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_facts::LoopCondContinueWithReturnFacts;
use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_recipe::ContinueWithReturnItem;
use crate::mir::builder::control_flow::plan::normalizer::{
    loop_body_lowering, lower_loop_header_cond, PlanNormalizer,
};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::steps::{
    build_standard5_internal_wires, collect_carrier_inits, empty_carriers_args,
};
use crate::mir::builder::control_flow::plan::steps::{effects_to_plans, lower_stmt_block};
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::{Effect, EffectMask};
use std::collections::{BTreeMap, BTreeSet};

const LOOP_COND_CONTINUE_WITH_RETURN_ERR: &str = "[normalizer] loop_cond_continue_with_return";

/// Helper for compact map/set trace logging.
/// Format: `[plan/trace] tag: len=N`
#[inline]
fn trace_collection_len(tag: &str, len: usize) {
    if crate::config::env::is_joinir_debug() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[plan/trace] {}: len={}", tag, len));
    }
}

pub(in crate::mir::builder) fn lower_loop_cond_continue_with_return(
    builder: &mut MirBuilder,
    facts: LoopCondContinueWithReturnFacts,
    _ctx: &LoopPatternContext,
) -> Result<LoweredRecipe, String> {
    // Pass 1: Collect carrier variables (recipe-specific, SSOT entry)
    let carrier_sets = carriers::collect_from_recipe_continue_with_return(&facts.recipe);
    let carrier_vars: BTreeSet<String> = carrier_sets.vars.into_iter().collect();

    // Pass 2: Build carrier_inits (step: lookup from variable_map)
    let carrier_inits = collect_carrier_inits(
        builder,
        carrier_vars.iter().cloned(),
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?;

    // Phase 11: StepBb mode only (HeaderBb legacy removed)
    lower_loop_cond_continue_with_return_stepbb(builder, facts, &carrier_vars, &carrier_inits)
}

/// StepBb mode implementation using CoreLoop Skeleton template.
///
/// This is the default mode for loop_cond_continue_with_return.
/// Uses the coreloop_skeleton template for block allocation and PHI management.
fn lower_loop_cond_continue_with_return_stepbb(
    builder: &mut MirBuilder,
    facts: LoopCondContinueWithReturnFacts,
    carrier_vars: &BTreeSet<String>,
    carrier_inits: &BTreeMap<String, crate::mir::ValueId>,
) -> Result<LoweredRecipe, String> {
    // Use template for block allocation + PHI dst allocation
    let frame = build_coreloop_frame(
        builder,
        carrier_vars,
        carrier_inits,
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?;

    // Extract block IDs from frame
    let preheader_bb = frame.preheader_bb;
    let header_bb = frame.header_bb;
    let body_bb = frame.body_bb;
    let step_bb = frame.step_bb;
    let after_bb = frame.after_bb;
    let continue_target = frame.continue_target; // Always step_bb in StepBb mode

    // Check for empty carriers (fail-fast)
    if frame.carrier_header_phis.is_empty() && !carrier_vars.is_empty() {
        return Err(format!(
            "{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: no loop carriers"
        ));
    }

    // Set up current_bindings with header PHI destinations
    let mut current_bindings = frame.carrier_header_phis.clone();
    trace_collection_len("init_current_bindings", current_bindings.len());
    for (name, value_id) in &current_bindings {
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
    }

    // Phase 2b-1: Short-circuit evaluation for loop header condition
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
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?;

    // Build Frag: internal wires + short-circuit branches
    let internal_wires = build_standard5_internal_wires(&frame, empty_carriers_args());
    let frag = Frag {
        entry: header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires: internal_wires,
        branches: header_result.branches,
    };

    // Lower loop body (using frame's PHI maps)
    let mut carrier_updates = BTreeMap::new();
    let body_view = BodyView::Recipe(&facts.recipe.body);
    let mut body_plans = lower_continue_with_return_block(
        builder,
        &mut current_bindings,
        &frame.carrier_header_phis,
        &frame.carrier_step_phis,
        &mut carrier_updates,
        &body_view,
        &facts.recipe.items,
    )?;

    // Continue with PHI args (debug trace)
    trace_collection_len("carrier_vars", carrier_vars.len());
    trace_collection_len("carrier_inits", carrier_inits.len());
    trace_collection_len("current_bindings", current_bindings.len());
    trace_collection_len("carrier_step_phis", frame.carrier_step_phis.len());

    // Add final continue exit using template helper
    body_plans.push(CorePlan::Exit(parts::exit::build_continue_with_phi_args(
        builder,
        &frame.carrier_step_phis,
        &current_bindings,
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?));

    // Generate PHIs using template (StepBb mode: step + header PHIs)
    let phis = build_header_step_phis(&frame, "loop_cond_continue_with_return")?;

    // Build final_values from header PHIs
    let final_values: Vec<(String, crate::mir::ValueId)> = frame
        .carrier_header_phis
        .iter()
        .map(|(var, phi_dst)| (var.clone(), *phi_dst))
        .collect();

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
        phis,
        frag,
        final_values,
        step_mode,
        has_explicit_step,
    }))
}

fn get_body_stmt<'a>(body: &BodyView<'a>, stmt_ref: StmtRef) -> Result<&'a ASTNode, String> {
    body.get_stmt(stmt_ref).ok_or_else(|| {
        format!(
            "{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: missing stmt idx={}",
            stmt_ref.index()
        )
    })
}

fn lower_continue_with_return_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    body: &BodyView<'_>,
    items: &[ContinueWithReturnItem],
) -> Result<Vec<LoweredRecipe>, String> {
    let mut plans = Vec::new();
    for item in items {
        plans.extend(lower_continue_with_return_item(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            carrier_updates,
            body,
            item,
        )?);
    }
    Ok(plans)
}

fn lower_continue_with_return_item(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    body: &BodyView<'_>,
    item: &ContinueWithReturnItem,
) -> Result<Vec<LoweredRecipe>, String> {
    match item {
        ContinueWithReturnItem::Stmt(stmt_ref) => {
            let stmt = get_body_stmt(body, *stmt_ref)?;
            lower_stmt_ast(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                stmt,
            )
        }
        ContinueWithReturnItem::ContinueIf {
            if_stmt,
            prelude_span,
            prelude_items,
        } => {
            let stmt = get_body_stmt(body, *if_stmt)?;
            let ASTNode::If {
                condition,
                then_body,
                else_body: _,
                ..
            } = stmt
            else {
                return Err(format!(
                    "{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: continue_if is not if"
                ));
            };
            let (prelude_start, prelude_end) = prelude_span.indices();
            if prelude_end > then_body.len() || prelude_start > prelude_end {
                return Err(format!(
                    "{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: continue_if prelude span out of range"
                ));
            }
            let prelude_body = &then_body[prelude_start..prelude_end];
            lower_continue_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                condition,
                prelude_body,
                prelude_items,
            )
        }
        ContinueWithReturnItem::HeteroReturnIf { if_stmt } => {
            let stmt = get_body_stmt(body, *if_stmt)?;
            let ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return Err(format!(
                    "{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: hetero_return_if is not if"
                ));
            };
            let then_assignment = then_body.first().ok_or_else(|| {
                format!("{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: hetero_return_if empty then_body")
            })?;
            let Some(else_body) = else_body.as_ref() else {
                return Err(format!(
                    "{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: hetero_return_if missing else_body"
                ));
            };
            lower_hetero_return_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                condition,
                then_assignment,
                else_body,
            )
        }
        ContinueWithReturnItem::IfAny(stmt_ref) => {
            let stmt = get_body_stmt(body, *stmt_ref)?;
            let ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return Err(format!(
                    "{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: if_any is not if"
                ));
            };
            if let Some(plans) = parts::entry::lower_conditional_update_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                condition,
                then_body,
                else_body.as_ref(),
                LOOP_COND_CONTINUE_WITH_RETURN_ERR,
            )? {
                return Ok(plans);
            }

            let saved_bindings = current_bindings.clone();
            let then_plans = lower_stmt_block(then_body, |stmt| {
                lower_stmt_ast(
                    builder,
                    current_bindings,
                    carrier_phis,
                    carrier_step_phis,
                    carrier_updates,
                    stmt,
                )
            })?;
            *current_bindings = saved_bindings.clone();
            let else_plans = match else_body {
                Some(body) => Some(lower_stmt_block(body, |stmt| {
                    lower_stmt_ast(
                        builder,
                        current_bindings,
                        carrier_phis,
                        carrier_step_phis,
                        carrier_updates,
                        stmt,
                    )
                })?),
                None => None,
            };
            *current_bindings = saved_bindings;

            let cond_view = CondBlockView::from_expr(condition);
            let mut then_plans_once = Some(then_plans);
            let mut else_plans_once = else_plans;
            let has_else = else_plans_once.is_some();
            let mut lower_else =
                |_builder: &mut MirBuilder,
                 _bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
                    Ok(else_plans_once.take().ok_or_else(|| {
	                    format!(
	                        "{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: internal error: else_plans consumed twice"
	                    )
	                })?)
                };
            let lower_else: Option<
                &mut dyn FnMut(
                    &mut MirBuilder,
                    &mut BTreeMap<String, crate::mir::ValueId>,
                ) -> Result<Vec<LoweredRecipe>, String>,
            > = if has_else {
                Some(&mut lower_else)
            } else {
                None
            };

            let should_update_binding =
                |name: &str, bindings: &BTreeMap<String, crate::mir::ValueId>| {
                    carrier_phis.contains_key(name) || bindings.contains_key(name)
                };
            parts::entry::lower_if_join_with_branch_lowerers(
                builder,
                current_bindings,
                &cond_view,
                LOOP_COND_CONTINUE_WITH_RETURN_ERR,
                &mut |_builder, _bindings| {
                    Ok(then_plans_once.take().ok_or_else(|| {
                        format!(
                            "{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: internal error: then_plans consumed twice"
                        )
                    })?)
                },
                lower_else,
                &should_update_binding,
            )
        }
    }
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
                LOOP_COND_CONTINUE_WITH_RETURN_ERR,
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
                LOOP_COND_CONTINUE_WITH_RETURN_ERR,
            )?;

            // デバッグログ追加（dev/debugガード下で1行だけ）
            // 変数名を列挙して current_bindings に存在する数を数える（型安全）
            if crate::config::env::is_joinir_debug() {
                let mut found = 0usize;
                for name in variables {
                    if current_bindings.contains_key(name) {
                        found += 1;
                    }
                }
                trace_collection_len("local_init_bindings_found", found);
            }

            Ok(effects_to_plans(effects))
        }
        ASTNode::MethodCall { .. } => {
            let effects = loop_body_lowering::lower_method_call_stmt(
                builder,
                current_bindings,
                stmt,
                LOOP_COND_CONTINUE_WITH_RETURN_ERR,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::FunctionCall { .. } => {
            let effects = loop_body_lowering::lower_function_call_stmt(
                builder,
                current_bindings,
                stmt,
                LOOP_COND_CONTINUE_WITH_RETURN_ERR,
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
        } => {
            if let Some(plans) = parts::entry::lower_conditional_update_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                condition,
                then_body,
                else_body.as_ref(),
                LOOP_COND_CONTINUE_WITH_RETURN_ERR,
            )? {
                return Ok(plans);
            }

            // Fallback to general if lowering (handles nested if-else-if chains).
            // NOTE: branch-lowerers are required to preserve branch maps for CoreIfJoin.
            if_branch_lowering::lower_if_with_branch_lowerers_and_updates(
                builder,
                current_bindings,
                carrier_phis,
                carrier_updates,
                condition,
                then_body,
                else_body.as_deref(),
                LOOP_COND_CONTINUE_WITH_RETURN_ERR,
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
            )
        }
        ASTNode::Return { value, .. } => parts::entry::lower_return_with_effects(
            builder,
            value.as_deref(),
            current_bindings,
            LOOP_COND_CONTINUE_WITH_RETURN_ERR,
        ),
        _ => {
            // For other statement types, let the normalizer handle it
            let (_value_id, effects) =
                PlanNormalizer::lower_value_ast(stmt, builder, current_bindings)?;
            Ok(effects_to_plans(effects))
        }
    }
}

fn lower_hetero_return_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_assignment: &ASTNode,
    else_chain: &[ASTNode],
) -> Result<Vec<LoweredRecipe>, String> {
    // For heterogeneous return-if with else_chain:
    // Pattern: if cond { then_assignment (e.g., in_str = 1) } else { else_chain (nested if-else) }
    //
    // Solution: Use CoreIfJoin for merge at join point (SSA-correct)
    //
    // Step 1: Collect carrier variable updates from then_assignment
    use crate::mir::builder::control_flow::plan::features::conditional_update_join::collect_conditional_update_branch;

    let pre_if_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    let then_body = std::slice::from_ref(then_assignment);
    let then_branch = collect_conditional_update_branch(
        builder,
        current_bindings,
        then_body,
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?;

    // Step 2: Lower then body
    let mut then_plans = Vec::new();
    then_plans.extend(effects_to_plans(then_branch.effects));
    then_plans.extend(lower_stmt_ast(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        then_assignment,
    )?);
    let then_map = builder.variable_ctx.variable_map.clone();

    // Step 3: Lower else body
    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();

    let mut else_plans = Vec::new();
    for stmt in else_chain {
        else_plans.extend(lower_stmt_ast(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            carrier_updates,
            stmt,
        )?);
    }
    let else_map = builder.variable_ctx.variable_map.clone();

    // Step 4: Restore state and create joins
    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings;

    if crate::config::env::is_joinir_debug() {
        let mut then_changed = Vec::new();
        let mut else_changed = Vec::new();
        for (name, pre_val) in &pre_if_map {
            let then_val = then_map.get(name).copied().unwrap_or(*pre_val);
            if then_val != *pre_val {
                then_changed.push(name.as_str());
            }
            let else_val = else_map.get(name).copied().unwrap_or(*pre_val);
            if else_val != *pre_val {
                else_changed.push(name.as_str());
            }
        }
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace] hetero_return_if maps then_changed_count={} else_changed_count={}",
            then_changed.len(),
            else_changed.len(),
        ));
    }

    // Step 5: Create joins for carrier variables (updates may occur in either branch).
    // Use carrier_phis keys to avoid dropping else-branch carrier updates.
    let carrier_vars_for_join: Vec<&String> = carrier_phis.keys().collect();

    // Step 6: Build condition value + if plan (SSOT: parts::dispatch)
    let cond_view = CondBlockView::from_expr(condition);
    let should_update_binding = |name: &str, bindings: &BTreeMap<String, crate::mir::ValueId>| {
        carrier_phis.contains_key(name) || bindings.contains_key(name)
    };
    let plans = parts::entry::lower_value_cond_if_with_filtered_joins(
        builder,
        current_bindings,
        &cond_view,
        &pre_if_map,
        &then_map,
        &else_map,
        carrier_vars_for_join.into_iter(),
        then_plans,
        else_plans,
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
        &should_update_binding,
        |name, dst| {
            carrier_updates.insert(name.to_owned(), dst);
        },
    )?;

    if crate::config::env::is_joinir_debug() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace] hetero_return_if: CoreIfJoin merge + if-else"
        ));
    }

    Ok(plans)
}

fn lower_continue_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    prelude_body: &[ASTNode],
    prelude_items: &[ContinueWithReturnItem],
) -> Result<Vec<LoweredRecipe>, String> {
    // Debug: check bindings BEFORE lowering prelude
    trace_collection_len(
        "continue_if_before_prelude:current_bindings",
        current_bindings.len(),
    );

    let mut then_plans = Vec::new();
    let prelude_view = BodyView::Slice(prelude_body);

    // Save bindings before lowering prelude (branch-local context)
    let saved_bindings = current_bindings.clone();
    let saved_map = builder.variable_ctx.variable_map.clone();
    then_plans.extend(lower_continue_with_return_block(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        &prelude_view,
        prelude_items,
    )?);
    then_plans.push(CorePlan::Exit(parts::exit::build_continue_with_phi_args(
        builder,
        carrier_step_phis,
        current_bindings,
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
    )?));

    // Restore bindings after building the continue exit (prevent branch-local changes from leaking).
    *current_bindings = saved_bindings;
    builder.variable_ctx.variable_map = saved_map;

    // Debug: check bindings before lowering continue-if condition
    if crate::config::env::is_joinir_debug() {}

    let cond_view = CondBlockView::from_expr(condition);
    let mut then_plans_once = Some(then_plans);
    parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        current_bindings,
        &cond_view,
        LOOP_COND_CONTINUE_WITH_RETURN_ERR,
        &mut |_builder, _bindings| {
            Ok(then_plans_once.take().ok_or_else(|| {
                format!("{LOOP_COND_CONTINUE_WITH_RETURN_ERR}: internal error: then_plans consumed twice")
            })?)
        },
        None,
        &|_name, _bindings| false,
    )
}
