use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::{
    StaticCallResultPublicationOwnerTakeErrorV1, StaticCallResultPublicationTakeV1,
    VerifiedStaticCallResultPublicationHandoffV1, VerifiedStaticCallResultPublicationOwnerV1,
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
    ) -> Result<StaticCallResultPublicationTakeV1, StaticCallResultPublicationOwnerTakeErrorV1>
    {
        let Some(owner) = self.static_result_publication_owner.as_mut() else {
            return Err(StaticCallResultPublicationOwnerTakeErrorV1::OwnerUnavailable);
        };
        owner.take_for_source(declarations, caller, site)
    }

    pub(in crate::mir::builder) fn target_for_source(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<CanonicalSameModuleCallableKeyV1> {
        self.static_result_publication_owner
            .as_ref()
            .and_then(|owner| owner.target_for_source(caller, site))
            .cloned()
    }

    /// Borrowed peek at one selected publication row.  The row stays owned by
    /// the publication owner; `take_static_result_publication_handoff` remains
    /// the sole consumption boundary.
    pub(in crate::mir::builder) fn selected_static_result_handoff_for_source(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedStaticCallResultPublicationHandoffV1> {
        self.static_result_publication_owner
            .as_ref()
            .and_then(|owner| owner.selected_handoff_for_source(caller, site))
    }
}
