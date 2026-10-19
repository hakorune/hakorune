//! Original per-call dependencies for a passive anchored-field End template.
//! No physical value or End readiness is manufactured here.
use super::*;
use crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1 as Source;
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn object_packet_field_support_parents_v1(
        &self, row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<Option<Box<[BindingRefV1]>>, String> {
        let seal = row.object_packet.as_ref().ok_or_else(|| freeze("field-end/seal-missing"))?;
        if !matches!(seal.teardown, ObjectReturnTeardownAvailabilityV1::AnchoredFields { .. }) { return Ok(None); }
        let actuals = row.checked_object_packet_actuals_v1(self)?;
        let observation = self.local_call_for_owner(row.call_site().owner(), row.call_site().site())
            .ok_or_else(|| freeze("field-end/local-source-missing"))?;
        if observation.local_binding().is_none() { return Err(freeze("field-end/local-required")); }
        let mut parents = Vec::new();
        let mut required = 0;
        crate::mir::normal_callable_semantic_package::ordinary_new_coseal::completion_index::ObjectCallFieldSupportRequirementV1::visit_actuals_v1(
            &seal.field_supports, row.source_target(), actuals, |requirement, actual| {
                required += 1;
                match &actual.source {
                    Source::Null => {},
                    Source::Read(source) => {
                        let (descriptor, parent, _) = source.field_parts_v1()
                            .ok_or_else(|| freeze("field-end/read-anchor-unconnected"))?;
                        let (call, ordinal, loan) = descriptor.returned_argument_v1()
                            .ok_or_else(|| freeze("field-end/original-field-missing"))?;
                        if call != row.call_site() || ordinal != requirement.ordinal
                            || loan.child() != requirement.relation.child()
                            || observation.prior_homes().iter().filter(|home| home.binding() == parent).count() != 1 {
                            return Err(freeze("field-end/parent-source-drift"));
                        }
                        self.recheck_returned_field_argument_source_v1(descriptor)?;
                        if self.handle_call_source(loan.acquisition()).and_then(|call| call.local_binding()).map(|(_, binding)| binding) != Some(parent) {
                            return Err(freeze("field-end/read-source-drift"));
                        }
                        if !parents.contains(&parent) { parents.push(parent); }
                    }
                    _ => return Err(freeze("field-end/actual-anchor-unconnected")),
                }
                Ok(())
            })?;
        if required == 0 { return Err(freeze("field-end/requirements-missing")); }
        Ok(Some(parents.into_boxed_slice()))
    }
}

impl LexicalInstanceCallDispositionRowV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn visit_object_field_support_actuals_v1(
        &self, ledger: &OrdinaryNewClaimLedgerV1,
        visit: impl FnMut(
            &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::completion_index::ObjectReturnFieldSupportV1,
            &PreparedBorrowedFormalActualV1,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        let actuals = self.checked_object_packet_actuals_v1(ledger)?;
        let seal = self.object_packet.as_ref().ok_or_else(|| freeze("field-end/seal-missing"))?;
        crate::mir::normal_callable_semantic_package::ordinary_new_coseal::completion_index::ObjectCallFieldSupportRequirementV1::visit_actuals_v1(
            &seal.field_supports, self.source_target(), actuals, visit)
    }
}
