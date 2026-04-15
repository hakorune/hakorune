//! GatherFactsStepBox (Phase 106)
//!
//! Responsibility: gather loop_break route analysis-only inputs
//! (legacy type labels: LoopBreakPrepFacts / LoopBreakPrepFactsBox).

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::condition_env::{ConditionBinding, ConditionEnv};
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_route_detection::function_scope_capture::CapturedEnv;
use crate::mir::ValueId;

use super::super::loop_break_prep_box::LoopBreakDebugLog;

pub(in crate::mir::builder) struct LoopBreakPrepFacts {
    pub loop_var_name: String,
    pub loop_var_id: ValueId,
    pub carrier_info: CarrierInfo,
    pub scope: LoopScopeShape,
    pub captured_env: CapturedEnv,
    pub join_value_space: JoinValueSpace,
    pub env: ConditionEnv,
    pub condition_bindings: Vec<ConditionBinding>,
    pub body_local_env: LoopBodyLocalEnv,
}

pub(crate) struct GatherFactsStepBox;

impl GatherFactsStepBox {
    pub(crate) fn gather(
        builder: &MirBuilder,
        condition: &ASTNode,
        body: &[ASTNode],
        fn_body: Option<&[ASTNode]>,
        ctx: &crate::mir::builder::control_flow::plan::route_prep_pipeline::RoutePrepContext,
        verbose: bool,
    ) -> Result<LoopBreakPrepFacts, String> {
        let log = LoopBreakDebugLog::new(verbose);
        use crate::mir::builder::control_flow::plan::condition_env_builder::ConditionEnvBuilder;
        use crate::mir::loop_route_detection::function_scope_capture::{
            analyze_captured_vars_v2, CapturedEnv,
        };

        let loop_var_name = ctx.loop_var_name.clone();
        let loop_var_id = ctx.loop_var_id;
        let mut carrier_info = ctx.carrier_info.clone();
        let scope = ctx.loop_scope.clone();

        log.log(
            "init",
            format!(
                "RoutePrepContext: loop_var='{}', loop_var_id={:?}, carriers={}",
                loop_var_name,
                loop_var_id,
                carrier_info.carriers.len()
            ),
        );

        log.log(
            "phase200c",
            format!(
                "fn_body is {}",
                if fn_body.is_some() { "SOME" } else { "NONE" }
            ),
        );
        let captured_env = if let Some(fn_body_ref) = fn_body {
            log.log(
                "phase200c",
                format!("fn_body has {} nodes", fn_body_ref.len()),
            );
            analyze_captured_vars_v2(fn_body_ref, condition, body, &scope)
        } else {
            log.log("phase200c", "fn_body is None, using empty CapturedEnv");
            CapturedEnv::new()
        };
        if verbose {
            log.log(
                "capture",
                format!(
                    "Phase 200-C: Captured {} variables",
                    captured_env.vars.len()
                ),
            );
            for var in &captured_env.vars {
                log.log(
                    "capture",
                    format!(
                        "  '{}': host_id={:?}, immutable={}",
                        var.name, var.host_id, var.is_immutable
                    ),
                );
            }
        }

        let mut captured_env = captured_env;
        let candidate_locals: std::collections::BTreeSet<String> =
            builder.variable_ctx.variable_map.keys().cloned().collect();

        if !candidate_locals.is_empty() {
            use crate::mir::loop_route_detection::pinned_local_analyzer::analyze_pinned_locals;

            match analyze_pinned_locals(body, &candidate_locals) {
                Ok(pinned_names) => {
                    if verbose && !pinned_names.is_empty() {
                        log.log(
                            "phase100_p1",
                            format!("Detected {} pinned locals", pinned_names.len()),
                        );
                    }

                    for pinned_name in pinned_names {
                        if let Some(&host_id) = builder.variable_ctx.variable_map.get(&pinned_name)
                        {
                            if verbose {
                                log.log(
                                    "phase100_p1",
                                    format!(
                                        "Wiring pinned local '{}' with host_id={:?}",
                                        pinned_name, host_id
                                    ),
                                );
                            }
                            captured_env.insert_pinned(pinned_name, host_id);
                        } else {
                            use crate::mir::join_ir::lowering::error_tags;
                            return Err(error_tags::freeze_with_hint(
                                "phase100/pinned/missing_host_id",
                                &format!(
                                    "Pinned local '{}' not found in variable_map",
                                    pinned_name
                                ),
                                "define the local before the loop (dominates loop entry)",
                            ));
                        }
                    }
                }
                Err(e) => return Err(format!("Pinned local analysis failed: {}", e)),
            }
        }

        use crate::mir::loop_route_detection::mutable_accumulator_analyzer::{
            AccumulatorKind, MutableAccumulatorAnalyzer, RhsExprKind,
        };

        let mutable_spec = MutableAccumulatorAnalyzer::analyze(body)?;

        if let Some(spec) = mutable_spec {
            if verbose {
                log.log(
                    "phase100_p2",
                    format!(
                        "Detected mutable accumulator: '{}' = '{}' + '{}'",
                        spec.target_name, spec.target_name, spec.rhs_var_or_lit
                    ),
                );
            }

            let target_id = builder
                .variable_ctx
                .variable_map
                .get(&spec.target_name)
                .ok_or_else(|| {
                    format!(
                        "[joinir/mutable-acc] Target '{}' not found in variable_map",
                        spec.target_name
                    )
                })?;

            let mut refined_kind = spec.kind;
            if spec.rhs_expr_kind == RhsExprKind::Var && refined_kind == AccumulatorKind::Int {
                use crate::mir::MirType;
                if let Some(target_type) = builder.type_ctx.value_types.get(target_id) {
                    match target_type {
                        MirType::Box(box_name) if box_name == "StringBox" => {
                            refined_kind = AccumulatorKind::String;
                            if verbose {
                                log.log(
                                    "phase100_p3",
                                    format!(
                                        "Refined accumulator kind: Int → String (target '{}' is StringBox)",
                                        spec.target_name
                                    ),
                                );
                            }
                        }
                        MirType::Integer => {
                            if verbose {
                                log.log(
                                    "phase100_p3",
                                    format!(
                                        "Confirmed accumulator kind: Int (target '{}' is Integer)",
                                        spec.target_name
                                    ),
                                );
                            }
                        }
                        _ => {
                            if verbose {
                                log.log(
                                    "phase100_p3",
                                    format!(
                                        "Accumulator kind: Int (default, target '{}' type unknown: {:?})",
                                        spec.target_name, target_type
                                    ),
                                );
                            }
                        }
                    }
                }
            }

            if refined_kind == AccumulatorKind::String {
                match spec.rhs_expr_kind {
                    RhsExprKind::Var => {
                        if verbose {
                            log.log(
                                "phase100_p3",
                                format!(
                                    "String accumulator '{}' = '{}' + '{}' (Variable RHS: OK)",
                                    spec.target_name, spec.target_name, spec.rhs_var_or_lit
                                ),
                            );
                        }
                    }
                    RhsExprKind::Literal => {
                        return Err(format!(
                            "[joinir/mutable-acc] String accumulator '{}' with Literal RHS not supported in Phase 100 P3 (will be P3.1)",
                            spec.target_name
                        ));
                    }
                }
            }

            match spec.rhs_expr_kind {
                RhsExprKind::Literal => {}
                RhsExprKind::Var => {
                    let rhs_name = &spec.rhs_var_or_lit;
                    let in_captured = captured_env.vars.iter().any(|v| &v.name == rhs_name);
                    let in_carrier = carrier_info.carriers.iter().any(|c| &c.name == rhs_name);

                    if in_carrier {
                        return Err(format!(
                            "[joinir/mutable-acc] RHS '{}' must be read-only (Condition/BodyLocal/Captured/Pinned), but found mutable Carrier",
                            rhs_name
                        ));
                    } else if !in_captured
                        && !builder.variable_ctx.variable_map.contains_key(rhs_name)
                    {
                        if verbose {
                            log.log(
                                "phase100_p2",
                                format!(
                                    "RHS '{}' not in captured/variable_map, assuming LoopBodyLocal (will validate later)",
                                    rhs_name
                                ),
                            );
                        }
                    }
                }
            }

            if spec.target_name == loop_var_name {
                if verbose {
                    log.log(
                        "phase100_p2",
                        format!(
                            "Skip promoting loop var '{}' to carrier (already loop_var)",
                            spec.target_name
                        ),
                    );
                }
            } else {
                if verbose {
                    log.log(
                        "phase100_p2",
                        format!(
                            "Promoting '{}' to mutable LoopState carrier",
                            spec.target_name
                        ),
                    );
                }

                use crate::mir::join_ir::lowering::carrier_info::CarrierVar;
                carrier_info
                    .carriers
                    .push(CarrierVar::new(spec.target_name.clone(), *target_id));
            }
        } else if verbose {
            log.log("phase100_p2", "No mutable accumulator pattern detected");
        }

        let mut join_value_space = JoinValueSpace::new();
        let (mut env, mut condition_bindings, _loop_var_join_id) =
            ConditionEnvBuilder::build_for_break_condition_v2(
                condition,
                &loop_var_name,
                &builder.variable_ctx.variable_map,
                loop_var_id,
                &mut join_value_space,
            )?;

        for var in &captured_env.vars {
            if let Some(&host_id) = builder.variable_ctx.variable_map.get(&var.name) {
                let join_id = join_value_space.alloc_param();
                env.insert(var.name.clone(), join_id);
                condition_bindings.push(ConditionBinding {
                    name: var.name.clone(),
                    host_value: host_id,
                    join_value: join_id,
                });
                log.log(
                    "capture",
                    format!(
                        "Phase 201: Added captured '{}': host={:?}, join={:?}",
                        var.name, host_id, join_id
                    ),
                );
            }
        }

        let body_local_env = LoopBodyLocalEnv::new();
        Ok(LoopBreakPrepFacts {
            loop_var_name,
            loop_var_id,
            carrier_info,
            scope,
            captured_env,
            join_value_space,
            env,
            condition_bindings,
            body_local_env,
        })
    }
}
