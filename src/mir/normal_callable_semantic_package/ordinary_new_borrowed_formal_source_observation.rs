//! Original Static observations retained beside borrowed-formal source facts.
use super::*;

impl PreparedBorrowedFormalIngressV1 {
    /// Source lookup borrows the one original draft map's disjoint partition.
    /// This does not make an excluded owner eligible for executable transport.
    pub(in crate::mir::normal_callable_semantic_package) fn source_definition_for(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Option<&BorrowedFormalUsesDraftV1> {
        self.definitions
            .get(&owner)
            .or_else(|| self.source_only_definitions.get(&owner))
    }

    /// Borrow one original raw Instance target after affine preparation is consumed.
    /// This lookup grants no incoming/domain or executable permission.
    pub(in crate::mir::normal_callable_semantic_package) fn object_source_target_at_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<&LexicalInstanceCallSourceTargetV1>, String> {
        let mut rows = self.source_incoming.exact_rows().filter(|row| &row.call == site);
        let Some(row) = rows.next() else {
            return Ok(None);
        };
        let target = row.source.require_instance()?;
        if rows.next().is_some()
            || target.call_site() != site
            || target.callee_owner() != row.callee
        {
            return Err(freeze("object-source/target-identity"));
        }
        Ok(Some(target))
    }

    /// Source agreement only; an explicit physical projection is still required.
    pub(in crate::mir::normal_callable_semantic_package) fn formal_integer_agreement(
        &self,
        formal: BindingRefV1,
    ) -> bool {
        self.definitions.contains_key(&formal.owner()) && self.integer_agreements.contains(&formal)
    }

    /// Complete input agreement independent of outgoing profile/transport permission.
    pub(in crate::mir::normal_callable_semantic_package) fn candidate_integer_agreement(
        &self,
        formal: BindingRefV1,
    ) -> bool {
        self.integer_agreements.contains(&formal)
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn contains_definition_for_test(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        self.definitions.contains_key(&owner)
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn candidate_input_inventory_for_test(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> (usize, bool) {
        (
            self.source_incoming.exact_rows().filter(|row| row.callee == owner).count(),
            self.source_incoming.vetoed_owners().contains(&owner),
        )
    }

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
