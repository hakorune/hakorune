//! EmitJoinIRStepBox (Phase 106)
//!
//! Responsibility: call loop_break route JoinIR lowerer and build inline boundary.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;

use super::super::loop_break_prep_box::{LoopBreakDebugLog, LoopBreakPrepInputs};
use super::emit_joinir_helpers::{
    build_loop_break_inline_boundary, lower_loop_break_joinir_fragment,
};
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;

use std::collections::BTreeMap;

pub(crate) struct EmitJoinIRStepOutput {
    pub join_module: crate::mir::join_ir::JoinModule,
    pub boundary: JoinInlineBoundary,
}

pub(crate) struct EmitJoinIRStepBox;

impl EmitJoinIRStepBox {
    pub(crate) fn emit(
        builder: &mut MirBuilder,
        condition: &ASTNode,
        body_ast: &[ASTNode],
        effective_break_condition: &ASTNode,
        carrier_updates: &BTreeMap<
            String,
            crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr,
        >,
        inputs: &mut LoopBreakPrepInputs,
        debug: bool,
        verbose: bool,
        skeleton: Option<&crate::mir::loop_canonicalizer::LoopSkeleton>,
    ) -> Result<EmitJoinIRStepOutput, String> {
        let log = LoopBreakDebugLog::new(verbose);
        let (join_module, fragment_meta) = match lower_loop_break_joinir_fragment(
            condition,
            body_ast,
            effective_break_condition,
            carrier_updates,
            inputs,
            skeleton,
        ) {
            Ok(result) => result,
            Err(e) => {
                crate::mir::builder::control_flow::joinir::trace::trace()
                    .debug("loop_break", &format!("LoopBreak lowerer failed: {}", e));
                return Err(format!("[cf_loop/loop_break] Lowering failed: {}", e));
            }
        };

        let boundary =
            build_loop_break_inline_boundary(builder, &join_module, &fragment_meta, inputs, debug)?;

        log.log("emit", "JoinIR module + boundary constructed");

        Ok(EmitJoinIRStepOutput {
            join_module,
            boundary,
        })
    }
}
