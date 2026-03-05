//! GatherFactsStepBox (Phase 106)
//!
//! Responsibility: gather Pattern2 analysis-only inputs.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;

use super::super::pattern2_inputs_facts_box::{Pattern2Facts, Pattern2InputsFactsBox};

pub(crate) struct GatherFactsStepBox;

impl GatherFactsStepBox {
    pub(crate) fn gather(
        builder: &MirBuilder,
        condition: &ASTNode,
        body: &[ASTNode],
        fn_body: Option<&[ASTNode]>,
        ctx: &crate::mir::builder::control_flow::plan::route_prep_pipeline::RoutePrepContext,
        verbose: bool,
    ) -> Result<Pattern2Facts, String> {
        Pattern2InputsFactsBox::analyze(builder, condition, body, fn_body, ctx, verbose)
    }
}
