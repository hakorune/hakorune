//! Original Static observations retained beside borrowed-formal source facts.
use super::*;

impl PreparedBorrowedFormalIngressV1 {
    /// Read the original source observation without selecting executable actuals.
    pub(in crate::mir::normal_callable_semantic_package) fn static_observation_for_source_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Option<
        &Result<
            std::rc::Rc<crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1>,
            String,
        >,
    >{
        self.source_incoming.static_observations().get(site)
    }
}
