use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, CarrierInit, CarrierRole};
use crate::mir::join_ir::lowering::common::body_local_derived_emitter::{
    BodyLocalDerivedEmitter, BodyLocalDerivedRecipe,
};
use crate::mir::join_ir::lowering::common::dual_value_rewriter::{
    try_derive_conditiononly_is_from_bodylocal_pos, try_derive_looplocal_from_bodylocal_pos,
};
use crate::mir::join_ir::lowering::common::conditional_step_emitter::emit_conditional_step_update;
use crate::mir::join_ir::lowering::condition_to_joinir::ConditionEnv;
use crate::mir::join_ir::lowering::debug_output_box::DebugOutputBox;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr;
use crate::mir::join_ir::lowering::update_env::UpdateEnv;
use crate::mir::join_ir::lowering::{carrier_update_emitter};
use crate::mir::join_ir::JoinInst;
use crate::mir::loop_canonicalizer::UpdateKind;
use crate::mir::loop_canonicalizer::LoopSkeleton;
use crate::mir::ValueId;
use std::collections::BTreeMap;

pub(crate) struct CarrierUpdateResult {
    pub updated_carrier_values: Vec<ValueId>,
    pub loop_var_next_override: Option<ValueId>,
}

pub(crate) fn emit_carrier_updates(
    env: &ConditionEnv,
    carrier_info: &CarrierInfo,
    carrier_updates: &BTreeMap<String, UpdateExpr>,
    mut body_local_env: Option<&mut LoopBodyLocalEnv>,
    body_local_derived_recipe: Option<&BodyLocalDerivedRecipe>,
    skeleton: Option<&LoopSkeleton>,
    carrier_param_ids: &[ValueId],
    alloc_local_fn: &mut dyn FnMut() -> ValueId,
    carrier_update_block: &mut Vec<JoinInst>,
    dev_log: &DebugOutputBox,
) -> Result<CarrierUpdateResult, String> {
    let mut loop_var_next_override: Option<ValueId> = None;

    if let (Some(recipe), Some(body_env)) = (body_local_derived_recipe, body_local_env.as_deref_mut()) {
        let emission = BodyLocalDerivedEmitter::emit(
            recipe,
            alloc_local_fn,
            env,
            body_env,
            carrier_update_block,
        )?;
        loop_var_next_override = Some(emission.loop_counter_next);
        dev_log.log_if_enabled(|| {
            format!(
                "[phase94/body_local_derived] enabled: name='{}', loop_counter='{}', loop_counter_next={:?}",
                recipe.name, recipe.loop_counter_name, emission.loop_counter_next
            )
        });
    }

    let body_env_ref = body_local_env.as_deref();
    debug_assert_eq!(carrier_param_ids.len(), carrier_info.carriers.len());

    let mut updated_carrier_values: Vec<ValueId> = Vec::new();

    for (idx, carrier) in carrier_info.carriers.iter().enumerate() {
        let carrier_name = &carrier.name;

        if carrier.init == CarrierInit::LoopLocalZero {
            if let Some(src_val) =
                try_derive_looplocal_from_bodylocal_pos(carrier_name, body_env_ref)
            {
                updated_carrier_values.push(src_val);
                continue;
            }
        }

        if carrier.role == CarrierRole::ConditionOnly {
            if let Some(cmp) = try_derive_conditiononly_is_from_bodylocal_pos(
                carrier_name,
                body_env_ref,
                alloc_local_fn,
                carrier_update_block,
            ) {
                updated_carrier_values.push(cmp);
                continue;
            }

            let current_value = env.get(carrier_name).ok_or_else(|| {
                format!("ConditionOnly carrier '{}' not found in env", carrier_name)
            })?;
            updated_carrier_values.push(current_value);
            dev_log.log_if_enabled(|| {
                format!(
                    "[carrier_update] Phase 227: ConditionOnly '{}' passthrough: {:?}",
                    carrier_name, current_value
                )
            });
            continue;
        }

        if carrier.init == CarrierInit::FromHost && !carrier_updates.contains_key(carrier_name) {
            let current_value = env
                .get(carrier_name)
                .ok_or_else(|| format!("FromHost carrier '{}' not found in env", carrier_name))?;
            updated_carrier_values.push(current_value);
            dev_log.log_if_enabled(|| {
                format!(
                    "[carrier_update] Phase 247-EX: FromHost '{}' passthrough: {:?}",
                    carrier_name, current_value
                )
            });
            continue;
        }

        if let Some(skel) = skeleton {
            if let Some(carrier_slot) = skel.carriers.iter().find(|c| c.name == *carrier_name) {
                if let UpdateKind::ConditionalStep {
                    cond,
                    then_delta,
                    else_delta,
                } = &carrier_slot.update_kind
                {
                    dev_log.log_if_enabled(|| {
                        format!(
                            "Phase 92 P2-1: ConditionalStep detected for carrier '{}': then={}, else={}",
                            carrier_name, then_delta, else_delta
                        )
                    });

                    let carrier_param = carrier.join_id.ok_or_else(|| {
                        format!(
                            "[loop_break/conditional_step] Carrier '{}' join_id not set (header PHI not generated?)",
                            carrier_name
                        )
                    })?;

                    let updated_value = emit_conditional_step_update(
                        carrier_name,
                        carrier_param,
                        &*cond,
                        *then_delta,
                        *else_delta,
                        alloc_local_fn,
                        env,
                        body_env_ref,
                        carrier_update_block,
                    )
                    .map_err(|e| format!("[loop_break/conditional_step] {}", e))?;
                    updated_carrier_values.push(updated_value);
                    dev_log.log_if_enabled(|| {
                        format!(
                            "Phase 92 P2-1: ConditionalStep carrier '{}' updated -> {:?}",
                            carrier_name, updated_value
                        )
                    });
                    continue;
                }
            }
        }

        let update_expr = carrier_updates.get(carrier_name).ok_or_else(|| {
            format!(
                "No update expression found for carrier '{}' in carrier_updates map",
                carrier_name
            )
        })?;

        let updated_value = if let Some(body_env) = body_env_ref {
            let update_env = UpdateEnv::new(env, body_env, &carrier_info.promoted_body_locals);
            carrier_update_emitter::emit_carrier_update_with_env(
                carrier,
                update_expr,
                alloc_local_fn,
                &update_env,
                carrier_update_block,
            )?
        } else {
            carrier_update_emitter::emit_carrier_update(
                carrier,
                update_expr,
                alloc_local_fn,
                env,
                carrier_update_block,
            )?
        };

        updated_carrier_values.push(updated_value);

        dev_log.log_if_enabled(|| {
            format!(
                "Phase 176-3: Carrier '{}' update: {:?} -> {:?}",
                carrier_name, carrier_param_ids[idx], updated_value
            )
        });
    }

    Ok(CarrierUpdateResult {
        updated_carrier_values,
        loop_var_next_override,
    })
}
