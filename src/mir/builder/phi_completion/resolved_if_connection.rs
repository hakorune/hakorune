//! Canonical resolved-If-only CFG-ready preparation handoff.
//!
//! This sidecar intentionally consumes the route's sealed witness instead of
//! accepting raw expected predecessor rows. It has no instruction, origin, or
//! type-publication authority; `phi_lifecycle` remains the shared final commit
//! owner.

use super::{render_preparation_error, CfgReadyPhiRowsV1, PhiDraftV1, PreparedPhiCompletionV1};
use crate::mir::builder::resolved_lowering::if_cfg_ready_bridge::VerifiedResolvedIfCfgReadyJoinRowsV1;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

pub(in crate::mir::builder) fn prepare_for_resolved_if(
    builder: &MirBuilder,
    rows: &VerifiedResolvedIfCfgReadyJoinRowsV1,
    row_index: usize,
    dst: ValueId,
) -> Result<PreparedPhiCompletionV1, String> {
    rows.reverify(builder)?;
    let block = rows.merge();
    let cfg_rows = CfgReadyPhiRowsV1::verify(
        &rows.expected_predecessors(),
        rows.logical_inputs_at(row_index)?,
    )
    .map_err(render_preparation_error)?;
    PhiDraftV1::new(block, dst, None)
        .prepare_cfg_ready(
            cfg_rows,
            &builder.function_state.type_ctx.value_types,
            builder.function_state.type_ctx.value_types.get(&dst),
        )
        .map_err(render_preparation_error)
}
