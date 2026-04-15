//! CarrierUpdatesStepBox (Phase 176+, extracted in Phase 5)
//!
//! Responsibility:
//! - Analyze carrier updates for loop_break route (or use policy override).
//! - Filter carriers to only those required by updates / condition-only / loop-local-zero.
//! - Ensure JoinValue env has join-ids for carriers referenced only from body updates.

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr;

use super::super::loop_break_prep_box::LoopBreakPrepInputs;
use super::carrier_updates_helpers::{
    bind_loop_break_update_only_carriers, resolve_loop_break_carrier_updates,
};

use std::collections::BTreeMap;

pub(crate) struct CarrierUpdatesStepBox;

impl CarrierUpdatesStepBox {
    pub(crate) fn analyze_and_filter(
        analysis_body: &[ASTNode],
        inputs: &mut LoopBreakPrepInputs,
        verbose: bool,
    ) -> BTreeMap<String, UpdateExpr> {
        let carrier_updates = resolve_loop_break_carrier_updates(analysis_body, inputs, verbose);
        bind_loop_break_update_only_carriers(inputs, verbose);
        carrier_updates
    }
}
