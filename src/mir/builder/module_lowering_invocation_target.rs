use super::ModuleLoweringPortV1;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

impl ModuleLoweringPortV1<'_> {
    pub(in crate::mir::builder) fn target_for_source(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<CanonicalSameModuleCallableKeyV1> {
        self.collector.target_for_source(caller, site)
    }

    /// Borrowed peek at one selected publication row for the LoopCond source
    /// relation issuer.  The row is never consumed here; the physical
    /// consumer still takes it exactly once.
    pub(in crate::mir::builder) fn selected_static_result_handoff_for_source(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedStaticCallResultPublicationHandoffV1> {
        self.collector
            .selected_static_result_handoff_for_source(caller, site)
    }
}
