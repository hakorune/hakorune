use crate::ast::ASTNode;
use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::edgecfg::api::{BranchStub, EdgeStub, ExitKind, Frag};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
use crate::mir::builder::control_flow::plan::normalizer::{
    helpers::LoopBlocksStandard5, loop_body_lowering, PlanNormalizer,
};
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreExitPlan, CoreIfPlan, CoreLoopPlan, CorePhiInfo, CorePlan,
    LoopTrueBreakContinueFacts,
};
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{ConstValue, MirType};
use std::collections::BTreeMap;

const LOOP_TRUE_ERR: &str = "[normalizer] loop_true_break_continue";

impl PlanNormalizer {
    pub(in crate::mir::builder) fn normalize_loop_true_break_continue(
        builder: &mut MirBuilder,
        facts: LoopTrueBreakContinueFacts,
        _ctx: &LoopPatternContext,
    ) -> Result<CorePlan, String> {
        let blocks = LoopBlocksStandard5::allocate(builder)?;
        let LoopBlocksStandard5 {
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
        } = blocks;

        let cond_loop = builder.alloc_typed(MirType::Bool);
        let header_effects = vec![CoreEffectPlan::Const {
            dst: cond_loop,
            value: ConstValue::Bool(true),
        }];

        let block_effects = vec![
            (preheader_bb, vec![]),
            (header_bb, header_effects),
            (body_bb, vec![]),
            (step_bb, vec![]),
        ];

        let empty_args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        };

        let branches = vec![BranchStub {
            from: header_bb,
            cond: cond_loop,
            then_target: body_bb,
            then_args: empty_args.clone(),
            else_target: after_bb,
            else_args: empty_args.clone(),
        }];

        let wires = vec![
            EdgeStub {
                from: body_bb,
                kind: ExitKind::Normal,
                target: Some(step_bb),
                args: empty_args.clone(),
            },
            EdgeStub {
                from: step_bb,
                kind: ExitKind::Normal,
                target: Some(header_bb),
                args: empty_args.clone(),
            },
        ];

        let frag = Frag {
            entry: header_bb,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires,
            branches,
        };

        let carrier_vars = collect_carrier_vars(builder, &facts.body);
        let mut carrier_inits = BTreeMap::new();
        let mut carrier_phis = BTreeMap::new();
        for var in &carrier_vars {
            let Some(&init_val) = builder.variable_ctx.variable_map.get(var) else {
                continue;
            };
            let ty = builder
                .type_ctx
                .get_type(init_val)
                .cloned()
                .unwrap_or(MirType::Unknown);
            let phi_dst = builder.alloc_typed(ty);
            carrier_inits.insert(var.clone(), init_val);
            carrier_phis.insert(var.clone(), phi_dst);
        }

        if carrier_phis.is_empty() {
            return Err(format!("{LOOP_TRUE_ERR}: no loop carriers"));
        }

        let phi_bindings = carrier_phis.clone();
        let mut carrier_updates = BTreeMap::new();

        let mut body_plans = Vec::new();
        let mut idx = 0usize;
        while idx < facts.body.len() {
            let stmt = &facts.body[idx];
            let next = facts.body.get(idx + 1);

            // Common parser-loop idiom:
            //   if (cond) { ... continue } break
            //   if (cond) { ... break } continue
            //
            // We treat the trailing exit as the else-exit for the preceding if.
            // This avoids emitting a top-level Break/Continue in the body block,
            // which would conflict with EdgeCFG's 1-block=1-terminator contract.
            if is_if_tail_exit_pair(stmt, next) {
                let ASTNode::If {
                    condition,
                    then_body,
                    else_body: None,
                    ..
                } = stmt
                else {
                    unreachable!();
                };
                let paired_exit = match next.unwrap() {
                    ASTNode::Break { .. } => CoreExitPlan::Break(1),
                    ASTNode::Continue { .. } => CoreExitPlan::Continue(1),
                    _ => unreachable!(),
                };
                let mut plans = lower_if_exit_stmt(
                    builder,
                    &phi_bindings,
                    &carrier_phis,
                    &mut carrier_updates,
                    condition,
                    then_body,
                    None,
                    Some(paired_exit),
                )?;
                body_plans.append(&mut plans);
                idx += 2;
                continue;
            }

            let mut plans = lower_loop_true_stmt(
                builder,
                &phi_bindings,
                &carrier_phis,
                &mut carrier_updates,
                stmt,
            )?;
            body_plans.append(&mut plans);
            idx += 1;
        }

        let mut phis = Vec::new();
        let mut final_values = Vec::new();
        for (var, phi_dst) in &carrier_phis {
            let init_val = match carrier_inits.get(var) {
                Some(value) => *value,
                None => continue,
            };
            let next_val = carrier_updates.get(var).copied().unwrap_or(init_val);
            phis.push(CorePhiInfo {
                block: header_bb,
                dst: *phi_dst,
                inputs: vec![(preheader_bb, init_val), (step_bb, next_val)],
                tag: format!("loop_true_carrier_{}", var),
            });
            final_values.push((var.clone(), *phi_dst));
        }

        Ok(CorePlan::Loop(CoreLoopPlan {
            preheader_bb,
            header_bb,
            body_bb,
            step_bb,
            after_bb,
            found_bb: after_bb,
            body: body_plans,
            cond_loop,
            cond_match: cond_loop,
            block_effects,
            phis,
            frag,
            final_values,
        }))
    }
}

fn lower_loop_true_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
) -> Result<Vec<CorePlan>, String> {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            let effects = lower_assignment_stmt(
                builder,
                phi_bindings,
                carrier_phis,
                carrier_updates,
                target,
                value,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            let effects =
                lower_local_init_stmt(builder, phi_bindings, variables, initial_values)?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::MethodCall { .. } => {
            let effects =
                loop_body_lowering::lower_method_call_stmt(builder, phi_bindings, stmt, LOOP_TRUE_ERR)?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::FunctionCall { .. } => {
            let effects =
                loop_body_lowering::lower_function_call_stmt(builder, phi_bindings, stmt, LOOP_TRUE_ERR)?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => lower_if_exit_stmt(
            builder,
            phi_bindings,
            carrier_phis,
            carrier_updates,
            condition,
            then_body,
            else_body.as_ref(),
            None,
        ),
        ASTNode::Break { .. } | ASTNode::Continue { .. } => {
            Err(format!("{LOOP_TRUE_ERR}: exit must be inside if"))
        }
        _ => Err(format!("{LOOP_TRUE_ERR}: unsupported stmt")),
    }
}

fn lower_if_exit_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    else_exit_override: Option<CoreExitPlan>,
) -> Result<Vec<CorePlan>, String> {
    let (cond_id, cond_effects) =
        loop_body_lowering::lower_bool_expr(builder, phi_bindings, condition, LOOP_TRUE_ERR)?;
    let mut plans = effects_to_plans(cond_effects);

    let then_plans = lower_exit_block(
        builder,
        phi_bindings,
        carrier_phis,
        carrier_updates,
        then_body,
    )?;
    let else_plans = match else_body {
        Some(body) => Some(lower_exit_block(
            builder,
            phi_bindings,
            carrier_phis,
            carrier_updates,
            body,
        )?),
        None => else_exit_override.map(|exit| vec![CorePlan::Exit(exit)]),
    };

    plans.push(CorePlan::If(CoreIfPlan {
        condition: cond_id,
        then_plans,
        else_plans,
    }));
    Ok(plans)
}

fn is_if_tail_exit_pair(stmt: &ASTNode, next: Option<&ASTNode>) -> bool {
    let ASTNode::If {
        then_body,
        else_body: None,
        ..
    } = stmt
    else {
        return false;
    };
    let Some(next) = next else {
        return false;
    };
    let Some(last) = then_body.last() else {
        return false;
    };
    matches!(
        (last, next),
        (ASTNode::Continue { .. }, ASTNode::Break { .. })
            | (ASTNode::Break { .. }, ASTNode::Continue { .. })
    )
}

fn lower_exit_block(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    body: &[ASTNode],
) -> Result<Vec<CorePlan>, String> {
    if body.is_empty() {
        return Err(format!("{LOOP_TRUE_ERR}: empty exit block"));
    }

    let mut out = Vec::new();
    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                if is_last {
                    return Err(format!("{LOOP_TRUE_ERR}: exit missing at tail"));
                }
                let effects = lower_assignment_stmt(
                    builder,
                    phi_bindings,
                    carrier_phis,
                    carrier_updates,
                    target,
                    value,
                )?;
                out.extend(effects_to_plans(effects));
            }
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                if is_last {
                    return Err(format!("{LOOP_TRUE_ERR}: exit missing at tail"));
                }
                let effects =
                    lower_local_init_stmt(builder, phi_bindings, variables, initial_values)?;
                out.extend(effects_to_plans(effects));
            }
            ASTNode::MethodCall { .. } => {
                if is_last {
                    return Err(format!("{LOOP_TRUE_ERR}: exit missing at tail"));
                }
                let effects = loop_body_lowering::lower_method_call_stmt(
                    builder,
                    phi_bindings,
                    stmt,
                    LOOP_TRUE_ERR,
                )?;
                out.extend(effects_to_plans(effects));
            }
            ASTNode::FunctionCall { .. } => {
                if is_last {
                    return Err(format!("{LOOP_TRUE_ERR}: exit missing at tail"));
                }
                let effects = loop_body_lowering::lower_function_call_stmt(
                    builder,
                    phi_bindings,
                    stmt,
                    LOOP_TRUE_ERR,
                )?;
                out.extend(effects_to_plans(effects));
            }
            ASTNode::Break { .. } => {
                if !is_last {
                    return Err(format!("{LOOP_TRUE_ERR}: break must be at tail"));
                }
                out.push(CorePlan::Exit(CoreExitPlan::Break(1)));
            }
            ASTNode::Continue { .. } => {
                if !is_last {
                    return Err(format!("{LOOP_TRUE_ERR}: continue must be at tail"));
                }
                out.push(CorePlan::Exit(CoreExitPlan::Continue(1)));
            }
            _ => {
                return Err(format!(
                    "{LOOP_TRUE_ERR}: unsupported stmt in exit block"
                ));
            }
        }
    }
    Ok(out)
}

fn lower_assignment_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    target: &ASTNode,
    value: &ASTNode,
) -> Result<Vec<CoreEffectPlan>, String> {
    let (name, value_id, effects) = loop_body_lowering::lower_assignment_value(
        builder,
        phi_bindings,
        target,
        value,
        LOOP_TRUE_ERR,
    )?;
    if carrier_phis.contains_key(&name) {
        carrier_updates.insert(name.clone(), value_id);
    }
    builder
        .variable_ctx
        .variable_map
        .insert(name, value_id);
    Ok(effects)
}

fn lower_local_init_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    variables: &[String],
    initial_values: &[Option<Box<ASTNode>>],
) -> Result<Vec<CoreEffectPlan>, String> {
    let (inits, effects) = loop_body_lowering::lower_local_init_values(
        builder,
        phi_bindings,
        variables,
        initial_values,
        LOOP_TRUE_ERR,
    )?;
    for (name, value_id) in inits {
        builder
            .variable_ctx
            .variable_map
            .insert(name, value_id);
    }
    Ok(effects)
}

fn effects_to_plans(effects: Vec<CoreEffectPlan>) -> Vec<CorePlan> {
    effects.into_iter().map(CorePlan::Effect).collect()
}

fn collect_carrier_vars(
    builder: &MirBuilder,
    body: &[ASTNode],
) -> Vec<String> {
    let mut carriers = BTreeMap::<String, ()>::new();
    for stmt in body {
        let ASTNode::Assignment { target, .. } = stmt else {
            continue;
        };
        let ASTNode::Variable { name, .. } = target.as_ref() else {
            continue;
        };
        if builder.variable_ctx.variable_map.contains_key(name) {
            carriers.insert(name.clone(), ());
        }
    }
    carriers.keys().cloned().collect()
}
