//! MergeStepBox (Phase 106)
//!
//! Responsibility: run JoinIR conversion pipeline (JoinIR → MIR merge) and return void.

use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::ValueId;

pub(crate) struct MergeStepBox;

impl MergeStepBox {
    pub(crate) fn merge(
        builder: &mut MirBuilder,
        join_module: crate::mir::join_ir::JoinModule,
        boundary: JoinInlineBoundary,
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::builder::control_flow::plan::conversion_pipeline::JoinIRConversionPipeline;
        use crate::mir::builder::emission::constant::emit_void;

        let _ = JoinIRConversionPipeline::execute(
            builder,
            join_module,
            Some(&boundary),
            "loop_break",
            debug,
        )?;

        let void_val = emit_void(builder)?;
        crate::mir::builder::control_flow::joinir::trace::trace().debug(
            "loop_break",
            &format!("Loop complete, returning Void {:?}", void_val),
        );
        Ok(Some(void_val))
    }
}
