//! Located-source LoopBreak physical adapter.
//!
//! The source candidate and source-bound Recipe are already issued before this
//! entry.  This adapter only pairs the exact root carrier with the existing
//! associated-source Parts dispatcher, which in turn reuses `lower_loop_v0_core`.
//! It never constructs `LoopRouteContext` or reclassifies a route.

use std::collections::BTreeMap;

use crate::mir::builder::control_flow::plan::parts::{
    lower_callable_loop_source_parts_block, CallableLoopSourcePartsBlockV1,
    PartsAssociatedBlockModeV1,
};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::normal_callable_loop_source_facts::SourceLoopBreakPhysicalInputV1;
use crate::mir::builder::normal_callable_loop_source_port::CallableLoopSourceStmtInputV1;
use crate::mir::builder::MirBuilder;

const LOOP_BREAK_SOURCE_ERR: &str = "[freeze:contract][callable-loop/loop-break/source-physical]";

pub(in crate::mir::builder) fn lower_loop_break_source(
    builder: &mut MirBuilder,
    input: &SourceLoopBreakPhysicalInputV1<'_, '_>,
) -> Result<LoweredRecipe, String> {
    input.validate_for_source_port()?;
    let recipe = input.source_recipe()?;
    let port = *input.source_port();
    let root_stmt = CallableLoopSourceStmtInputV1::Located {
        node: input.parent_node(),
        source: input.parent_source().clone(),
    };
    let root =
        CallableLoopSourcePartsBlockV1::singleton(&recipe.arena, &recipe.root, root_stmt, &port)
            .map_err(|error| format!("{LOOP_BREAK_SOURCE_ERR}/root-seal: {error:?}"))?;
    let mut current_bindings = builder.function_state.variable_ctx.variable_map.clone();
    let plans = lower_callable_loop_source_parts_block(
        port,
        &root,
        PartsAssociatedBlockModeV1::NoExit,
        builder,
        &mut current_bindings,
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &mut BTreeMap::new(),
        LOOP_BREAK_SOURCE_ERR,
    )?;
    let [plan] = plans.as_slice() else {
        return Err(format!(
            "{LOOP_BREAK_SOURCE_ERR}/root-plan-cardinality={}",
            plans.len()
        ));
    };
    Ok(plan.clone())
}
