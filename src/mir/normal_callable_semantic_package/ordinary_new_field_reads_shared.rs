//! Shared-handle field reads: support carriage and the Normal-edge source recheck.
use super::*;

impl LocalFieldReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn with_shared_support(
        original: &Rc<Self>, support: &(OwnedExprSiteV1, OwnedExprSiteV1),
    ) -> Option<Rc<Self>> {
        if original.result != crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar
            || original.declared_type_name.is_none() || original.received.is_some()
            || original.expression_path.is_some() || original.returned.is_some() || original.owned_ready.is_some()
            || support.0.owner() != support.1.owner() { return None; }
        let mut source = original.as_ref().clone();
        source.shared_support = Some(support.clone());
        Some(Rc::new(source))
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn recheck_shared_field_read_v1(&self, site: &OwnedExprSiteV1, row: &LocalFieldRead) -> Result<(), String> {
        let support = row.source.shared_support.as_ref().ok_or_else(|| fault("shared-field/support-missing"))?;
        let required = self.source_field_requirement_v1(site)?.ok_or_else(|| fault("shared-field/source-missing"))?;
        let request = required.request();
        if request.shared_support.as_ref() != Some(support) || request.receiver != row.receiver
            || request.home != row.home || request.receiver_site != row.receiver_site
            || request.owned_ready.is_some() || request.path.is_some() {
            return Err(fault("shared-field/source-drift"));
        }
        let (descriptor, _, _) = self.prepare_shared_clone_handle_v1(row.receiver, &support.0, required.prior_homes())?;
        if descriptor.payload != row.source.field().object() { return Err(fault("shared-field/payload-drift")); }
        Ok(())
    }
}
