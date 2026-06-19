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

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::ValueId;

    #[test]
    fn generic_loop_v1_carrier_orchestration_exposes_post_body_map_for_handoff() {
        let carrier_orchestration = GenericLoopV1CarrierOrchestration::new_for_tests(
            BTreeMap::from([("i".to_string(), ValueId::new(9))]),
            true,
        );

        assert_eq!(
            carrier_orchestration.post_body_map().get("i").copied(),
            Some(ValueId::new(9))
        );
    }

    #[test]
    fn generic_loop_v1_carrier_orchestration_allows_empty_post_body_map() {
        let carrier_orchestration =
            GenericLoopV1CarrierOrchestration::new_for_tests(BTreeMap::new(), false);

        assert!(carrier_orchestration.post_body_map().is_empty());
    }
}
