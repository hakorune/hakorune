//! Source selection for the same Local initializer physical consumer.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn initializer_expression_frame_v1(
        &self, owner: FunctionOwnerIdV1, statement: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1>>, String> {
        let Some(flow) = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow()) else { return Ok(None) };
        let mut frames = flow.expression_plans().iter().filter_map(|plan| plan.initializer_frame())
            .filter(|frame| frame.source().root().statement().node() == statement);
        let frame = frames.next().cloned();
        if frames.next().is_some() { return Err(freeze("text-initializer/duplicate-source-frame")); }
        if frame.as_ref().is_some_and(|frame| frame.source().owner() != owner) { return Err(freeze("text-initializer/source-owner")); }
        Ok(frame)
    }

}
