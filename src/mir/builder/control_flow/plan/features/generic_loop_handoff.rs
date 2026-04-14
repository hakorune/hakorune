//! GenericLoop condition/step handoff helpers (apply-only).

use crate::mir::builder::control_flow::plan::features::{
    generic_loop_body::GenericLoopV1CarrierOrchestration, generic_loop_step,
};
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::{
    GenericLoopV0Facts, GenericLoopV1Facts,
};
use crate::mir::builder::control_flow::plan::skeletons::generic_loop::GenericLoopSkeleton;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::collections::BTreeMap;

const GENERIC_LOOP_ERR: &str = "[normalizer] generic loop v0";

pub(in crate::mir::builder) fn apply_generic_loop_v0_condition_step_handoff(
    builder: &mut MirBuilder,
    facts: &GenericLoopV0Facts,
    skeleton: &mut GenericLoopSkeleton,
    pre_body_map: BTreeMap<String, ValueId>,
    post_body_map: BTreeMap<String, ValueId>,
) -> Result<(), String> {
    builder.variable_ctx.variable_map = pre_body_map;
    generic_loop_step::apply_generic_loop_condition(
        builder,
        skeleton,
        &facts.condition,
        &facts.loop_var,
        GENERIC_LOOP_ERR,
    )?;
    builder.variable_ctx.variable_map = post_body_map;
    generic_loop_step::apply_generic_loop_step(
        builder,
        skeleton,
        &facts.loop_increment,
        &facts.loop_var,
        GENERIC_LOOP_ERR,
    )?;
    Ok(())
}

pub(in crate::mir::builder) fn apply_generic_loop_v1_condition_step_handoff(
    builder: &mut MirBuilder,
    facts: &GenericLoopV1Facts,
    skeleton: &mut GenericLoopSkeleton,
    pre_body_map: BTreeMap<String, ValueId>,
    carrier_orchestration: &GenericLoopV1CarrierOrchestration,
) -> Result<(), String> {
    builder.variable_ctx.variable_map = pre_body_map;
    generic_loop_step::apply_generic_loop_condition(
        builder,
        skeleton,
        &facts.condition,
        &facts.loop_var,
        GENERIC_LOOP_ERR,
    )?;

    builder.variable_ctx.variable_map = carrier_orchestration.post_body_map().clone();
    let loop_var_step_src =
        carrier_orchestration.loop_var_step_src(&facts.loop_var, skeleton.loop_var_current);
    builder
        .variable_ctx
        .variable_map
        .insert(facts.loop_var.clone(), loop_var_step_src);
    if carrier_orchestration.body_has_continue_edge() {
        generic_loop_step::apply_generic_loop_step(
            builder,
            skeleton,
            &facts.loop_increment,
            &facts.loop_var,
            GENERIC_LOOP_ERR,
        )?;
        crate::mir::builder::control_flow::joinir::trace::trace().varmap(
            "generic_loop_v1_post_step",
            &builder.variable_ctx.variable_map,
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::ValueId;

    #[test]
    fn generic_loop_v1_condition_step_handoff_uses_post_body_loop_var() {
        let carrier_orchestration = GenericLoopV1CarrierOrchestration::new_for_tests(
            BTreeMap::from([("i".to_string(), ValueId::new(9))]),
            true,
        );

        assert_eq!(
            carrier_orchestration.loop_var_step_src("i", ValueId::new(3)),
            ValueId::new(9)
        );
        assert!(carrier_orchestration.body_has_continue_edge());
    }

    #[test]
    fn generic_loop_v1_condition_step_handoff_falls_back_without_continue_edge() {
        let carrier_orchestration =
            GenericLoopV1CarrierOrchestration::new_for_tests(BTreeMap::new(), false);

        assert_eq!(
            carrier_orchestration.loop_var_step_src("i", ValueId::new(3)),
            ValueId::new(3)
        );
        assert!(!carrier_orchestration.body_has_continue_edge());
    }
}
