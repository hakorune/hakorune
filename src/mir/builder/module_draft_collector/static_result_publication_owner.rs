use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::{
    StaticCallResultPublicationOwnerTakeErrorV1, VerifiedStaticCallResultPublicationHandoffV1,
    VerifiedStaticCallResultPublicationOwnerV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::ModuleDraftCollectorV1;

impl ModuleDraftCollectorV1 {
    pub(in crate::mir::builder) fn install_static_result_publication_owner(
        &mut self,
        owner: VerifiedStaticCallResultPublicationOwnerV1,
    ) -> Result<(), &'static str> {
        if self.static_result_publication_owner.is_some() {
            return Err("[freeze:contract][module_draft/static-result-owner-duplicate]");
        }
        self.static_result_publication_owner = Some(owner);
        Ok(())
    }

    pub(in crate::mir::builder) fn take_static_result_publication_handoff(
        &mut self,
        declarations: &crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
        target: &CanonicalSameModuleCallableKeyV1,
    ) -> Result<
        Option<VerifiedStaticCallResultPublicationHandoffV1>,
        StaticCallResultPublicationOwnerTakeErrorV1,
    > {
        let Some(owner) = self.static_result_publication_owner.as_mut() else {
            return Ok(None);
        };
        owner.take(declarations, caller, site, target)
    }
}
