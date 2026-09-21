//! Located-source composite LoopBreak physical adapter.
//!
//! The composite input has already co-sealed the resolver Recipe, source
//! port, and ordered selected handoffs.  This function only reuses the
//! existing associated-source Parts/LoopV0 lowering spine.

use std::collections::BTreeMap;

use crate::mir::builder::control_flow::plan::parts::{
    lower_callable_loop_source_parts_block, CallableLoopSourcePartsBlockV1,
    PartsAssociatedBlockModeV1,
};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::normal_callable_loop_source_facts::SourceLoopBreakCompositePhysicalInputV1;
use crate::mir::builder::normal_callable_loop_source_port::CallableLoopSourceStmtInputV1;
use crate::mir::builder::MirBuilder;

const LOOP_BREAK_COMPOSITE_SOURCE_ERR: &str =
    "[freeze:contract][callable-loop/composite/source-physical]";

pub(in crate::mir::builder) fn lower_loop_break_composite_source(
    builder: &mut MirBuilder,
    input: &SourceLoopBreakCompositePhysicalInputV1<'_, '_>,
) -> Result<LoweredRecipe, String> {
    input.validate_for_source_port()?;
    let port = *input.source_port();
    let root_stmt = CallableLoopSourceStmtInputV1::Located {
        node: input.parent_node(),
        source: input.parent_source().clone(),
    };
    let root = CallableLoopSourcePartsBlockV1::singleton(
        &input.recipe().arena,
        &input.recipe().root,
        root_stmt,
        &port,
    )
    .map_err(|error| format!("{LOOP_BREAK_COMPOSITE_SOURCE_ERR}/root-seal: {error:?}"))?;
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
        LOOP_BREAK_COMPOSITE_SOURCE_ERR,
    )?;
    let [plan] = plans.as_slice() else {
        return Err(format!(
            "{LOOP_BREAK_COMPOSITE_SOURCE_ERR}/root-plan-cardinality={}",
            plans.len()
        ));
    };
    Ok(plan.clone())
}
