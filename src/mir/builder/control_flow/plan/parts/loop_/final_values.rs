use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn apply_loop_final_values_to_bindings(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    plan: &LoweredRecipe,
) {
    let CorePlan::Loop(loop_plan) = plan else {
        return;
    };
    for (name, value_id) in &loop_plan.final_values {
        builder
            .variable_ctx
            .variable_map
            .insert(name.clone(), *value_id);
        if current_bindings.contains_key(name) {
            current_bindings.insert(name.clone(), *value_id);
        }
    }
}
