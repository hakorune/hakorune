//! The Local-initializer call route, decided once per site.
//!
//! Kind comes from name resolution only (direct-call inventory, or the
//! method-call receiver); the result class comes from the callee's own class
//! source for that kind. A kind never asks another kind's class source, and no
//! answer for one kind falls through to another.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{LocalCallRouteV1, StaticI64CallClaimV1};
use crate::mir::resolved_semantics::{
    BindingKindV1, ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1,
};

pub(super) struct LocalRouteSourcesV1<'a> {
    pub(super) owner_loan: Option<&'a crate::mir::normal_callable_semantic_package::direct_call_loan::DirectCallDispositionLoanV1>,
    pub(super) selected: &'a VerifiedSelectedCallableBatchMapV1,
    pub(super) batch: &'a VerifiedResolvedCallableSemanticBatchV1,
    pub(super) parameter_contracts: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    pub(super) callable_result_classes: &'a super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    pub(super) borrowed_i64_results: &'a super::super::lexical_instance_call::BorrowedResultCohortV1,
    pub(super) candidates: &'a [OrdinaryNewCandidate],
    pub(super) receiver_call_observations: &'a BTreeMap<OwnedExprSiteV1, crate::mir::normal_callable_semantic_package::ReceiverCallClassObservationV1>,
    pub(super) input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>,
}

impl LocalRouteSourcesV1<'_> {
    pub(super) fn route<E>(
        &self,
        site: &OwnedExprSiteV1,
        static_claim: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    ) -> Result<Option<LocalCallRouteV1>, E> {
        use crate::mir::normal_callable_semantic_package::direct_call_loan::lifecycle::DirectLocalResultV1;
        let function = self.input.function();
        if site.owner() != self.input.owner() {
            return Ok(None);
        }
        if function.direct_call_observation(site.site()).is_some() {
            return Ok(self
                .owner_loan
                .and_then(|loan| {
                    loan.local_result_class_v1(self.batch, self.parameter_contracts, self.input, site)
                })
                .map(|class| match class {
                    DirectLocalResultV1::I64 => LocalCallRouteV1::DirectI64,
                    DirectLocalResultV1::Map => LocalCallRouteV1::DirectMap,
                    DirectLocalResultV1::Handle => LocalCallRouteV1::DirectHandle,
                }));
        }
        let Some(call) = function.method_call(site.site()) else {
            return Ok(None);
        };
        match call.receiver() {
            ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) => {
                Ok(match function.binding(binding).map(|row| row.kind()) {
                    Some(BindingKindV1::Receiver) => self.receiver_route(site),
                    Some(BindingKindV1::Local { .. }) => super::lexical::lexical_local_route(
                        self.selected,
                        self.batch,
                        self.parameter_contracts,
                        self.callable_result_classes,
                        self.borrowed_i64_results,
                        self.candidates,
                        self.input,
                        site,
                    ),
                    _ => None,
                })
            }
            ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Upvar(_))
            | ResolvedMethodCallReceiverSourceV1::Other => Ok(None),
            ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
            | ResolvedMethodCallReceiverSourceV1::CurrentOwner => {
                Ok(static_claim(site)?.map(LocalCallRouteV1::Static))
            }
        }
    }

    /// `me.m(..)`: the entry-receiver observation names the class — a
    /// `NullableObject` claim with its stored-slot source is Nullable, an
    /// `Object` claim stays on the Handle lane.
    fn receiver_route(&self, site: &OwnedExprSiteV1) -> Option<LocalCallRouteV1> {
        use crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1;
        if super::lexical::has_object_receiver_call_at_v1(self.receiver_call_observations, site) {
            return Some(LocalCallRouteV1::ReceiverHandle);
        }
        let row = self.receiver_call_observations.get(site)?;
        (matches!(row.class(), OrdinaryNewResultClassV1::NullableObject(_))
            && row.map_stored_source().is_some())
        .then_some(LocalCallRouteV1::ReceiverNullable)
    }
}
