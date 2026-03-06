//! GatherFactsStepBox (Phase 106)
//!
//! Responsibility: gather loop_break route analysis-only inputs
//! (legacy type labels: LoopBreakPrepFacts / LoopBreakPrepFactsBox).

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;

use super::super::loop_break_prep_box::{LoopBreakPrepFacts, LoopBreakPrepFactsBox};

pub(crate) struct GatherFactsStepBox;

impl GatherFactsStepBox {
    pub(crate) fn gather(
        builder: &MirBuilder,
        condition: &ASTNode,
        body: &[ASTNode],
        fn_body: Option<&[ASTNode]>,
        ctx: &crate::mir::builder::control_flow::plan::route_prep_pipeline::RoutePrepContext,
        verbose: bool,
    ) -> Result<LoopBreakPrepFacts, String> {
        LoopBreakPrepFactsBox::analyze(builder, condition, body, fn_body, ctx, verbose)
    }
}
