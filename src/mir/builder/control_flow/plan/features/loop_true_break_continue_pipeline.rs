use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::features::carriers::collect_outer_from_body;
use crate::mir::builder::control_flow::plan::features::exit_if_map;
use crate::mir::builder::control_flow::plan::features::loop_true_break_continue_phi_materializer::LoopTrueBreakContinuePhiMaterializer;
use crate::mir::builder::control_flow::plan::features::loop_true_break_continue_verifier::verify_loop_true_break_continue_phi_closure;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1::lower_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::features::step_mode;
use crate::mir::builder::control_flow::plan::loop_cond::true_break_continue::LoopTrueBreakContinueFacts;
use crate::mir::builder::control_flow::plan::loop_true_break_continue::recipe::{
    ElseItem, LoopTrueItem,
};
use crate::mir::builder::control_flow::plan::normalizer::loop_body_lowering;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::skeletons::loop_true::alloc_loop_true_skeleton;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePhiInfo, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::effect::Effect;
use crate::mir::policies::BodyLoweringPolicy;
use crate::mir::EffectMask;
use std::collections::BTreeMap;

// Feature-only: keep logic in skeletons/features to avoid mixed rule drift.
const LOOP_TRUE_ERR: &str = "[normalizer] loop_true_break_continue";

pub(in crate::mir::builder) fn lower_loop_true_break_continue(
    builder: &mut MirBuilder,
    facts: LoopTrueBreakContinueFacts,
    _ctx: &LoopRouteContext,
) -> Result<LoweredRecipe, String> {
    lower_loop_true_break_continue_inner(builder, facts)
}

pub(in crate::mir::builder) fn lower_loop_true_break_continue_inner(
    builder: &mut MirBuilder,
    facts: LoopTrueBreakContinueFacts,
) -> Result<LoweredRecipe, String> {
    let skeleton = alloc_loop_true_skeleton(builder)?;
    let preheader_bb = skeleton.preheader_bb;
    let header_bb = skeleton.header_bb;
    let body_bb = skeleton.body_bb;
    let step_bb = skeleton.step_bb;
    let after_bb = skeleton.after_bb;
    let cond_loop = skeleton.cond_loop;
    let block_effects = skeleton.block_effects;
    let frag = skeleton.frag;

    let carrier_vars = collect_outer_from_body(builder, &facts.recipe.body.body).vars;
    let phi_materializer =
        LoopTrueBreakContinuePhiMaterializer::prepare(builder, &carrier_vars, LOOP_TRUE_ERR)?;
    let carrier_phis = phi_materializer.carrier_phis().clone();
    let carrier_step_phis = phi_materializer.carrier_step_phis().clone();
    let mut current_bindings = phi_materializer.phi_bindings();

    let recipe = &facts.recipe;

    let mut body_break_phi_dsts: Option<BTreeMap<String, crate::mir::ValueId>> = None;
    let mut body_after_phis: Vec<CorePhiInfo> = Vec::new();

    let mut body_plans: Vec<LoweredRecipe> = match facts.body_lowering_policy {
        BodyLoweringPolicy::ExitAllowed { .. } => {
            let Some(body_exit_allowed) = facts.body_exit_allowed.as_ref() else {
                return Err(format!(
                    "[freeze:contract][loop_true_break_continue] body_lowering_policy=ExitAllowed but body_exit_allowed=None: ctx={LOOP_TRUE_ERR}"
                ));
            };
            let body = &recipe.body.body;
            let has_break = body.iter().any(body_has_break_stmt);
            let break_phi_dsts = if has_break {
                let (out, after_phis) = phi_materializer.plan_break_after_phis(
                    builder,
                    &carrier_vars,
                    header_bb,
                    after_bb,
                );
                body_after_phis = after_phis;
                out
            } else {
                BTreeMap::new()
            };
            if !break_phi_dsts.is_empty() {
                body_break_phi_dsts = Some(break_phi_dsts.clone());
            }

            let verified = parts::entry::verify_exit_allowed_block_with_pre(
                &body_exit_allowed.arena,
                &body_exit_allowed.block,
                LOOP_TRUE_ERR,
                Some(&current_bindings),
            )?;
            parts::entry::lower_exit_allowed_block_verified(
                builder,
                &mut current_bindings,
                &carrier_step_phis,
                &break_phi_dsts,
                verified,
                LOOP_TRUE_ERR,
            )?
        }
        BodyLoweringPolicy::RecipeOnly => Vec::new(),
    };

    if body_plans.is_empty() {
        for item in &recipe.items {
            let mut plans = match item {
                LoopTrueItem::Stmt(r) => {
                    let Some(stmt) = recipe.body.get_ref(*r) else {
                        return Err(format!("{LOOP_TRUE_ERR}: Stmt recipe idx out of range"));
                    };
                    lower_simple_stmt(builder, &mut current_bindings, stmt)?
                }
                LoopTrueItem::ProgramGeneralBlock(r) => {
                    let Some(stmt) = recipe.body.get_ref(*r) else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: ProgramGeneralBlock recipe idx out of range"
                        ));
                    };
                    let ASTNode::Program { statements, .. } = stmt else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: ProgramGeneralBlock recipe expects Program node"
                        ));
                    };
                    lower_general_if_body_block(
                        builder,
                        &mut current_bindings,
                        &carrier_phis,
                        &carrier_step_phis,
                        &statements,
                    )?
                }
                LoopTrueItem::ExitIf(r) => {
                    let Some(stmt) = recipe.body.get_ref(*r) else {
                        return Err(format!("{LOOP_TRUE_ERR}: ExitIf recipe idx out of range"));
                    };
                    let ASTNode::If {
                        condition,
                        then_body,
                        else_body,
                        ..
                    } = stmt
                    else {
                        return Err(format!("{LOOP_TRUE_ERR}: ExitIf recipe expects If node"));
                    };
                    match body_break_phi_dsts.as_ref() {
                        Some(break_phi_dsts) => {
                            exit_if_map::lower_if_exit_stmt_with_break_phi_args(
                                builder,
                                &current_bindings,
                                &carrier_step_phis,
                                break_phi_dsts,
                                &condition,
                                &then_body,
                                else_body.as_ref(),
                                LOOP_TRUE_ERR,
                            )?
                        }
                        None => exit_if_map::lower_if_exit_stmt(
                            builder,
                            &current_bindings,
                            &carrier_step_phis,
                            &condition,
                            &then_body,
                            else_body.as_ref(),
                            LOOP_TRUE_ERR,
                        )?,
                    }
                }
                LoopTrueItem::IfTailExitPair(p) => {
                    let Some(if_stmt) = recipe.body.get(p.a) else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: IfTailExitPair if idx out of range"
                        ));
                    };
                    let ASTNode::If {
                        condition,
                        then_body,
                        else_body: None,
                        ..
                    } = if_stmt
                    else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: IfTailExitPair recipe expects If node"
                        ));
                    };
                    let Some(exit_stmt) = recipe.body.get(p.b) else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: IfTailExitPair exit idx out of range"
                        ));
                    };
                    if !matches!(exit_stmt, ASTNode::Break { .. } | ASTNode::Continue { .. }) {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: IfTailExitPair recipe expects Break/Continue"
                        ));
                    }
                    let else_body = vec![exit_stmt.clone()];
                    match body_break_phi_dsts.as_ref() {
                        Some(break_phi_dsts) => {
                            exit_if_map::lower_if_exit_stmt_with_break_phi_args(
                                builder,
                                &current_bindings,
                                &carrier_step_phis,
                                break_phi_dsts,
                                &condition,
                                &then_body,
                                Some(&else_body),
                                LOOP_TRUE_ERR,
                            )?
                        }
                        None => exit_if_map::lower_if_exit_stmt(
                            builder,
                            &current_bindings,
                            &carrier_step_phis,
                            &condition,
                            &then_body,
                            Some(&else_body),
                            LOOP_TRUE_ERR,
                        )?,
                    }
                }
                LoopTrueItem::NestedLoopDepth1(r) => {
                    let Some(stmt) = recipe.body.get_ref(*r) else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: NestedLoopDepth1 recipe idx out of range"
                        ));
                    };
                    let ASTNode::Loop {
                        condition, body, ..
                    } = stmt
                    else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: NestedLoopDepth1 recipe expects Loop node"
                        ));
                    };
                    let plan =
                        lower_nested_loop_depth1_any(builder, &condition, &body, LOOP_TRUE_ERR)?;
                    vec![plan]
                }
                LoopTrueItem::GeneralIf(r) => {
                    let Some(stmt) = recipe.body.get_ref(*r) else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: GeneralIf recipe idx out of range"
                        ));
                    };
                    let ASTNode::If {
                        condition,
                        then_body,
                        else_body,
                        ..
                    } = stmt
                    else {
                        return Err(format!("{LOOP_TRUE_ERR}: GeneralIf recipe expects If node"));
                    };
                    lower_general_if_stmt(
                        builder,
                        &mut current_bindings,
                        &carrier_phis,
                        &carrier_step_phis,
                        stmt,
                        &condition,
                        &then_body,
                        else_body.as_ref(),
                    )?
                }
                LoopTrueItem::GeneralIfElseExit {
                    if_ref,
                    else_recipe,
                } => {
                    let Some(stmt) = recipe.body.get_ref(*if_ref) else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: GeneralIfElseExit recipe idx out of range"
                        ));
                    };
                    let ASTNode::If {
                        condition,
                        then_body,
                        ..
                    } = stmt
                    else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: GeneralIfElseExit recipe expects If node"
                        ));
                    };

                    let cond_view = cond_block_view::CondBlockView::from_expr(&condition);

                    let mut lower_then =
                        |builder: &mut MirBuilder,
                         bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
                            lower_general_if_body_block(
                                builder,
                                bindings,
                                &carrier_phis,
                                &carrier_step_phis,
                                &then_body,
                            )
                        };

                    let mut lower_else =
                        |builder: &mut MirBuilder,
                         bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
                            let mut else_plans = Vec::new();
                            for item in &else_recipe.items {
                                match item {
                                    ElseItem::ExitIf(stmt_ref) => {
                                        let Some(else_stmt) = else_recipe.else_body.get_ref(*stmt_ref)
                                        else {
                                            return Err(format!(
                                                "{LOOP_TRUE_ERR}: ElseItem::ExitIf idx out of range"
                                            ));
                                        };
                                        let ASTNode::If {
                                            condition: else_cond,
                                            then_body: else_then,
                                            else_body: else_else,
                                            ..
                                        } = else_stmt
                                        else {
                                            return Err(format!(
                                                "{LOOP_TRUE_ERR}: ElseItem::ExitIf expects If node"
                                            ));
                                        };

                                        let exit_plans = match body_break_phi_dsts.as_ref() {
                                            Some(break_phi_dsts) => exit_if_map::lower_if_exit_stmt_with_break_phi_args(
                                                builder,
                                                &*bindings,
                                                &carrier_step_phis,
                                                break_phi_dsts,
                                                else_cond,
                                                else_then,
                                                else_else.as_ref(),
                                                LOOP_TRUE_ERR,
                                            )?,
                                            None => exit_if_map::lower_if_exit_stmt(
                                                builder,
                                                &*bindings,
                                                &carrier_step_phis,
                                                else_cond,
                                                else_then,
                                                else_else.as_ref(),
                                                LOOP_TRUE_ERR,
                                            )?,
                                        };
                                        else_plans.extend(exit_plans);
                                    }
                                    ElseItem::PreludeStmt(stmt_ref) => {
                                        let Some(else_stmt) = else_recipe.else_body.get_ref(*stmt_ref)
                                        else {
                                            return Err(format!(
                                                "{LOOP_TRUE_ERR}: ElseItem::PreludeStmt idx out of range"
                                            ));
                                        };
                                        let stmt_plans =
                                            lower_simple_stmt(builder, bindings, else_stmt)?;
                                        else_plans.extend(stmt_plans);
                                    }
                                }
                            }
                            Ok(else_plans)
                        };

                    let should_update_binding =
                        |name: &str, bindings: &BTreeMap<String, crate::mir::ValueId>| {
                            carrier_phis.contains_key(name) || bindings.contains_key(name)
                        };

                    parts::entry::lower_if_join_with_branch_lowerers(
                        builder,
                        &mut current_bindings,
                        &cond_view,
                        LOOP_TRUE_ERR,
                        &mut lower_then,
                        Some(&mut lower_else),
                        &should_update_binding,
                    )?
                }
                LoopTrueItem::TailReturn(r) => {
                    let Some(stmt) = recipe.body.get_ref(*r) else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: TailReturn recipe idx out of range"
                        ));
                    };
                    let ASTNode::Return { value, .. } = stmt else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: TailReturn recipe expects Return node"
                        ));
                    };
                    let Some(value) = value.as_ref() else {
                        return Err(format!(
                            "{LOOP_TRUE_ERR}: TailReturn requires return(value)"
                        ));
                    };
                    parts::entry::lower_return_with_effects(
                        builder,
                        Some(value),
                        &current_bindings,
                        LOOP_TRUE_ERR,
                    )?
                }
            };
            body_plans.append(&mut plans);
        }
    }

    // Normal fallthrough should also supply per-carrier phi args.
    // This avoids "single next_val" assumptions when continue edges update carriers.
    let requires_fallthrough_continue = !matches!(recipe.items.last(), Some(LoopTrueItem::TailReturn(_)))
        && !matches!(body_plans.last(), Some(CorePlan::Exit(_)));
    if requires_fallthrough_continue {
        let exit = parts::exit::build_continue_with_phi_args(
            builder,
            &carrier_step_phis,
            &current_bindings,
            LOOP_TRUE_ERR,
        )?;
        body_plans.push(CorePlan::Exit(exit));
    }

    let body_after_phi_count = body_after_phis.len();
    let phi_closure = phi_materializer.close(
        preheader_bb,
        header_bb,
        step_bb,
        body_break_phi_dsts.as_ref(),
        body_after_phis,
        LOOP_TRUE_ERR,
    )?;
    verify_loop_true_break_continue_phi_closure(
        &phi_closure,
        &body_plans,
        body_break_phi_dsts.as_ref(),
        body_after_phi_count,
        carrier_phis.len(),
        requires_fallthrough_continue,
        LOOP_TRUE_ERR,
    )?;

    let (step_mode, has_explicit_step) = step_mode::inline_in_body_no_explicit_step();

    Ok(CorePlan::Loop(CoreLoopPlan {
        preheader_bb,
        preheader_is_fresh: false,
        header_bb,
        body_bb,
        step_bb,
        continue_target: step_bb,
        after_bb,
        found_bb: after_bb,
        body: body_plans,
        cond_loop,
        cond_match: cond_loop,
        block_effects,
        phis: phi_closure.phis().to_vec(),
        frag,
        final_values: phi_closure.final_values().to_vec(),
        step_mode,
        has_explicit_step,
    }))
}

fn body_has_break_stmt(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Break { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(body_has_break_stmt)
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(body_has_break_stmt))
        }
        ASTNode::Loop { body, .. } => body.iter().any(body_has_break_stmt),
        ASTNode::Program { statements, .. } => statements.iter().any(body_has_break_stmt),
        _ => false,
    }
}

/// Lower simple statements only (Assignment/Local/MethodCall/FunctionCall/Print/Program).
/// Recipe guarantees no If/Loop nodes are passed here.
fn lower_simple_stmt(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
) -> Result<Vec<LoweredRecipe>, String> {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            let (name, value_id, effects) = loop_body_lowering::lower_assignment_value(
                builder,
                current_bindings,
                target,
                value,
                LOOP_TRUE_ERR,
            )?;
            current_bindings.insert(name.clone(), value_id);
            builder.variable_ctx.variable_map.insert(name, value_id);
            Ok(effects_to_plans(effects))
        }
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            let (inits, effects) = loop_body_lowering::lower_local_init_values(
                builder,
                current_bindings,
                variables,
                initial_values,
                LOOP_TRUE_ERR,
            )?;
            for (name, value_id) in inits {
                current_bindings.insert(name.clone(), value_id);
                builder.variable_ctx.variable_map.insert(name, value_id);
            }
            Ok(effects_to_plans(effects))
        }
        ASTNode::MethodCall { .. } => {
            let effects =
                loop_body_lowering::lower_method_call_stmt(builder, current_bindings, stmt, LOOP_TRUE_ERR)?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::FunctionCall { .. } => {
            let effects =
                loop_body_lowering::lower_function_call_stmt(builder, current_bindings, stmt, LOOP_TRUE_ERR)?;
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
        ASTNode::Program { statements, .. } => {
            lower_simple_block(builder, current_bindings, statements)
        }
        _ => Err(format!(
            "{LOOP_TRUE_ERR}: simple stmt expects Assignment/Local/MethodCall/FunctionCall/Print/Program, got {:?}",
            stmt
        )),
    }
}

/// Lower a block of simple statements.
fn lower_simple_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    body: &[ASTNode],
) -> Result<Vec<LoweredRecipe>, String> {
    let mut out = Vec::new();
    for stmt in body {
        let mut plans = lower_simple_stmt(builder, current_bindings, stmt)?;
        out.append(&mut plans);
    }
    Ok(out)
}

fn lower_general_if_body_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    body: &[ASTNode],
) -> Result<Vec<LoweredRecipe>, String> {
    let mut out = Vec::new();
    for stmt in body {
        match stmt {
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let mut plans = lower_general_if_stmt(
                    builder,
                    current_bindings,
                    carrier_phis,
                    carrier_step_phis,
                    stmt,
                    condition,
                    then_body,
                    else_body.as_ref(),
                )?;
                out.append(&mut plans);
            }
            _ => {
                let mut plans = lower_simple_stmt(builder, current_bindings, stmt)?;
                out.append(&mut plans);
            }
        }
    }
    Ok(out)
}

/// Lower a general if statement (carrier update only, no exit).
/// Recipe guarantees the if body contains only "general-if body" statements (simple + nested general-if).
fn lower_general_if_stmt(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
) -> Result<Vec<LoweredRecipe>, String> {
    // M28: Under planner_required, use RecipeBlock-based lowering for stmt-only if(no-exit)
    if crate::config::env::joinir_dev::planner_required_enabled()
        && try_build_stmt_only_block_recipe(then_body).is_some()
        && else_body.map_or(true, |eb| try_build_stmt_only_block_recipe(eb).is_some())
    {
        if let Some(recipe) = try_build_no_exit_block_recipe(std::slice::from_ref(stmt), true) {
            // Capture carrier_phis for should_update_binding policy
            let carrier_phis_ref = carrier_phis;
            let should_update_binding =
                |name: &str, bindings: &BTreeMap<String, crate::mir::ValueId>| {
                    carrier_phis_ref.contains_key(name) || bindings.contains_key(name)
                };

            let verified = parts::entry::verify_no_exit_block_with_pre(
                &recipe.arena,
                &recipe.block,
                LOOP_TRUE_ERR,
                Some(&current_bindings),
            )?;
            return parts::entry::lower_no_exit_block_with_stmt_lowerer_verified(
                builder,
                current_bindings,
                carrier_step_phis,
                None, // break_phi_dsts
                verified,
                LOOP_TRUE_ERR,
                || {
                    // make_lower_stmt: returns a closure for lowering each statement
                    |builder: &mut MirBuilder,
                     bindings: &mut BTreeMap<String, crate::mir::ValueId>,
                     _carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
                     _break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
                     stmt: &ASTNode,
                     _error_prefix: &str| {
                        lower_simple_stmt(builder, bindings, stmt)
                    }
                },
                should_update_binding,
            );
        }
    }

    // Fallback: existing conditional_update path
    let lower_block = |builder: &mut MirBuilder,
                       bindings: &mut BTreeMap<String, crate::mir::ValueId>,
                       carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
                       carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
                       body: &[ASTNode]| {
        lower_general_if_body_block(builder, bindings, carrier_phis, carrier_step_phis, body)
    };
    if let Some(plans) = crate::mir::builder::control_flow::plan::features::conditional_update_join::try_lower_general_if_recipe_authority(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        condition,
        then_body,
        else_body,
        LOOP_TRUE_ERR,
        lower_block,
    )? {
        return Ok(plans);
    }
    Err(format!("{LOOP_TRUE_ERR}: general if rejected"))
}
