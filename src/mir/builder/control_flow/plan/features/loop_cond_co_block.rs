//! Block-level lowering for continue-only pattern.

use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::recipes::loop_cond_continue_only::ContinueOnlyStmtRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::loop_cond_co_stmt::lower_continue_only_stmt;

/// Lower a block of continue-only statements to CorePlans.
pub(super) fn lower_continue_only_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    body: &BodyView<'_>,
    items: &[ContinueOnlyStmtRecipe],
) -> Result<Vec<LoweredRecipe>, String> {
    let mut plans = Vec::new();
    for stmt in items {
        let mut stmt_plans = lower_continue_only_stmt(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            carrier_updates,
            body,
            stmt,
        )?;
        plans.append(&mut stmt_plans);
    }
    Ok(plans)
}
