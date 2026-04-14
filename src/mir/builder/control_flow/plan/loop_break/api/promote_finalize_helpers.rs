use crate::mir::builder::MirBuilder;

use super::super::super::loop_break_prep_box::LoopBreakPrepInputs;

pub(super) fn finalize_promoted_inputs(
    builder: &mut MirBuilder,
    inputs: &mut LoopBreakPrepInputs,
    promoted_pairs: Vec<(String, String)>,
    debug: bool,
) -> Result<(), String> {
    assign_promoted_carrier_join_ids(inputs, promoted_pairs)?;
    validate_promoted_break_condition(builder, inputs, debug);
    Ok(())
}

fn assign_promoted_carrier_join_ids(
    inputs: &mut LoopBreakPrepInputs,
    promoted_pairs: Vec<(String, String)>,
) -> Result<(), String> {
    for carrier in &mut inputs.carrier_info.carriers {
        let carrier_join_id = inputs.join_value_space.alloc_param();
        carrier.join_id = Some(carrier_join_id);
    }

    for (promoted_var, promoted_carrier_name) in promoted_pairs {
        let join_id = inputs
            .carrier_info
            .find_carrier(&promoted_carrier_name)
            .and_then(|c| c.join_id)
            .ok_or_else(|| {
                format!(
                    "[phase229] promoted carrier '{}' has no join_id",
                    promoted_carrier_name
                )
            })?;
        inputs.env.insert(promoted_var, join_id);
    }

    Ok(())
}

fn validate_promoted_break_condition(
    builder: &mut MirBuilder,
    inputs: &LoopBreakPrepInputs,
    debug: bool,
) {
    use crate::mir::join_ir::lowering::expr_lowerer::{
        ExprContext, ExprLowerer, ExprLoweringError,
    };
    use crate::mir::join_ir::lowering::scope_manager::LoopBreakScopeManager;

    let scope_manager = LoopBreakScopeManager {
        condition_env: &inputs.env,
        loop_body_local_env: Some(&inputs.body_local_env),
        captured_env: Some(&inputs.captured_env),
        carrier_info: &inputs.carrier_info,
    };

    match ExprLowerer::new(&scope_manager, ExprContext::Condition, builder)
        .with_debug(debug)
        .lower(&inputs.break_condition_node)
    {
        Ok(_) => {}
        Err(ExprLoweringError::UnsupportedNode(_)) => {}
        Err(_) => {}
    }
}
