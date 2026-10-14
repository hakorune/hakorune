//! Original Static observations retained beside borrowed-formal source facts.
use super::*;

impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn is_main_static_terminal_site_v1(
        &self,
        site: &OwnedExprSiteV1,
    ) -> bool {
        self.source_incoming
            .static_observations()
            .get(site)
            .is_some_and(|row| {
                row.as_ref()
                    .is_ok_and(|original| self.is_main_static_terminal_source_v1(original))
            })
    }

    /// Exact Main terminal membership is source-only, not entry permission.
    pub(in crate::mir::normal_callable_semantic_package) fn is_main_static_terminal_source_v1(
        &self,
        original: &crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1,
    ) -> bool {
        self.app_main_owner == Some(original.call_site().owner())
            && original.is_qualified()
            && self.app_main_terminal_sites.contains(original.call_site())
            && original.argument_sites().len() == 1
            && original.parameters().len() == 1
            && original.parameters()[0].kind == CallableParameterContractKindV1::OpaqueHandle
    }

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
        let mut rows = self
            .source_incoming
            .exact_rows()
            .filter(|row| &row.call == site);
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
        self.original_formal_seeds.candidate_integer_agreement_v1(&self.source_incoming, formal)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn candidate_i64_source_agreement(&self, formal: BindingRefV1) -> bool {
        self.original_formal_seeds.candidate_i64_source_agreement_v1(&self.source_incoming, formal)
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
            self.source_incoming
                .exact_rows()
                .filter(|row| row.callee == owner)
                .count(),
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

/// Retain original ReturnValue sites, without granting a call or entry route.
pub(super) fn app_main_terminal_sites_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    app_main: Option<&BorrowedAppMainSourceLoanV1<'_>>,
) -> Result<BTreeSet<OwnedExprSiteV1>, String> {
    match app_main {
        Some(main) => batch
            .with_lowering_input(main.batch_slot(), |input| {
                use crate::mir::resolved_semantics::{BodyStatementShapeV1, SourcePathSegmentV1};
                input
                    .body_shape()
                    .into_iter()
                    .flat_map(|shape| shape.statements())
                    .filter_map(|statement| match statement {
                        BodyStatementShapeV1::Return {
                            value: Some(value), ..
                        } if matches!(
                            value.node().segments(),
                            [SourcePathSegmentV1::Body(_), SourcePathSegmentV1::Value]
                        ) =>
                        {
                            Some(OwnedExprSiteV1::new(input.owner(), value.clone()))
                        }
                        _ => None,
                    })
                    .collect()
            })
            .map_err(|_| freeze("borrowed-static/main-terminal-source-loan")),
        None => Ok(BTreeSet::new()),
    }
}
