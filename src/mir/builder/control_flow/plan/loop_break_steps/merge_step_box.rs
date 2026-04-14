//! MergeStepBox (Phase 106)
//!
//! Responsibility: run JoinIR conversion pipeline (JoinIR → MIR merge) and return void.

use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::ValueId;

use super::merge_step_helpers::emit_loop_break_completion_void;

pub(crate) struct MergeStepBox;

impl MergeStepBox {
    pub(crate) fn merge(
        builder: &mut MirBuilder,
        join_module: crate::mir::join_ir::JoinModule,
        boundary: JoinInlineBoundary,
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::builder::control_flow::plan::conversion_pipeline::JoinIRConversionPipeline;

        let _ = JoinIRConversionPipeline::execute(
            builder,
            join_module,
            Some(&boundary),
            "loop_break",
            debug,
        )?;

        emit_loop_break_completion_void(builder)
    }
}
