use super::ModuleLoweringPortV1;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

impl ModuleLoweringPortV1<'_> {
    pub(in crate::mir::builder) fn target_for_source(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<CanonicalSameModuleCallableKeyV1> {
        self.collector.target_for_source(caller, site)
    }
}
