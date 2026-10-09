//! Selected loop-body scalar source joined to the existing physical signature.
//! This is source/shape evidence only; no ValueId or executable ABI is issued.

use std::rc::Rc;

use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::definitions::MirCall;
use crate::mir::function::MirFunction;
use crate::mir::normal_callable_semantic_package::physical_signature::{
    PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1,
};
use crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, OwnedExprSiteV1, ResolvedLoopPlacementV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::super::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use super::lexical_instance_call::project_current_owner_loop_scalar_source_v1;
use super::loop_static_source_loan::LoopStaticSourceCallLoanV1;
use super::static_loop_packet_source::corroborate_unpublished_physical_shape;
use super::OrdinaryNewClaimLedgerV1;

#[derive(Debug)]
pub(in crate::mir) struct VerifiedLoopStaticBodyScalarSourceV1 {
    original: Rc<StaticIncomingSourceV1>,
    complete_incoming: Box<[Rc<StaticIncomingSourceV1>]>,
    binding: BindingRefV1,
    site: SourceExprSiteV1,
    formal: BindingRefV1,
}

impl VerifiedLoopStaticBodyScalarSourceV1 {
    /// Only the original selected call may lend a target to the unpublished
    /// canonical body. The per-iteration ValueId remains the SSA owner's.
    pub(in crate::mir) fn materialize_unpublished_call(
        &self,
        binding: BindingRefV1,
        site: &SourceExprSiteV1,
        actual: crate::mir::ValueId,
    ) -> Result<MirCall, String> {
        let reject = || "[freeze:contract][callable-loop/body-scalar-call-drift]".to_owned();
        if self.binding != binding
            || &self.site != site
            || self
                .complete_incoming
                .iter()
                .filter(|row| Rc::ptr_eq(row, &self.original))
                .count()
                != 1
        {
            return Err(reject());
        }
        let target = self
            .original
            .target()
            .canonical_global_target_v1()
            .map_err(|_| reject())?;
        Ok(MirCall::global(None, target, vec![actual]))
    }

    pub(in crate::mir) fn corroborate_unpublished_physical_call(
        &self,
        binding: BindingRefV1,
        site: &SourceExprSiteV1,
        actual: crate::mir::ValueId,
        function: &MirFunction,
        entry: crate::mir::BasicBlockId,
        normal: crate::mir::BasicBlockId,
        result: crate::mir::ValueId,
    ) -> Result<(), String> {
        let expected = self.materialize_unpublished_call(binding, site, actual)?;
        corroborate_unpublished_physical_shape(&expected, actual, function, entry, normal, result)
    }

    pub(in crate::mir) fn corroborates(
        &self,
        loan: &LoopStaticSourceCallLoanV1,
        binding: BindingRefV1,
        site: &SourceExprSiteV1,
    ) -> bool {
        Rc::ptr_eq(&self.original, loan.original())
            && self
                .complete_incoming
                .iter()
                .filter(|row| Rc::ptr_eq(row, &self.original))
                .count()
                == 1
            && self.binding == binding
            && &self.site == site
            && self.original.argument_sites() == [site.clone()]
            && self.original.parameters().len() == 1
            && self.original.parameters()[0].binding == self.formal
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn retain_loop_static_body_scalar_sources_v1(
        &mut self,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let mut rows = std::collections::BTreeMap::new();
        for ((loop_site, site), loan) in self.loop_static_source_loans.borrow().iter() {
            let Ok(loan) = loan else { continue };
            if loan.placement() != &ResolvedLoopPlacementV1::Body
                || loan.argument_sites().len() != 1
            {
                continue;
            }
            if rows
                .insert(
                    (loop_site.clone(), site.clone()),
                    issue_scalar_source(self, signatures, results, loan),
                )
                .is_some()
            {
                return Err("[freeze:contract][callable-loop/body-scalar-duplicate]".into());
            }
        }
        *self.loop_static_body_scalar_sources.get_mut() = rows;
        Ok(())
    }

    pub(in crate::mir) fn take_loop_static_body_scalar_source_v1(
        &self,
        loop_site: &SourceStmtSiteV1,
        site: &OwnedExprSiteV1,
    ) -> Option<Result<VerifiedLoopStaticBodyScalarSourceV1, String>> {
        self.loop_static_body_scalar_sources
            .borrow_mut()
            .remove(&(loop_site.clone(), site.clone()))
    }
}

fn issue_scalar_source(
    ledger: &OrdinaryNewClaimLedgerV1,
    signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    results: &VerifiedCallableResultContractCohortV1,
    loan: &LoopStaticSourceCallLoanV1,
) -> Result<VerifiedLoopStaticBodyScalarSourceV1, String> {
    let reject = || "[freeze:contract][callable-loop/body-scalar-source-unavailable]".to_owned();
    let original = loan.original();
    let incoming = ledger.borrowed_formal_source.as_ref().ok_or_else(reject)?;
    let Some((candidate_source, binding, site)) = project_current_owner_loop_scalar_source_v1(
        incoming,
        &ledger.borrowed_formal_actuals,
        loan.call_site(),
        loan.claim(),
    )?
    else {
        return Err(reject());
    };
    let signature = signatures
        .row(original.target_batch_slot())
        .ok_or_else(reject)?;
    let source = incoming.as_ref().map_err(Clone::clone)?;
    let completion = ledger
        .completion_for_owner(original.callee_owner())
        .ok_or_else(reject)?;
    let result = results
        .row(original.target_batch_slot())
        .ok_or_else(reject)?;
    let result_ref = result.borrow();
    let [formal] = original.parameters() else {
        return Err(reject());
    };
    let [lane] = signature.lanes() else {
        return Err(reject());
    };
    if !Rc::ptr_eq(&candidate_source, original)
        || original.argument_sites() != [site.clone()]
        || signature.owner() != original.callee_owner()
        || signature.mode() != CallableParameterDeclarationModeV1::StaticBoxMethod
        || signature.source_logical_arity() != 1
        || signature.receiver_lane_count() != 0
        || signature.physical_formal_lane_count() != 1
        || signature.physical_callable_lane_count() != 1
        || lane.index() != 0
        || lane.role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
        || lane.logical_ordinal() != Some(0)
        || lane.binding() != formal.binding
        || formal.ordinal != 0
        || formal.kind != CallableParameterContractKindV1::OpaqueHandle
        || !source.source_only_formal_origin(formal.binding)
        || !source.checked_static_input(formal.binding)
        || completion.owner() != original.callee_owner()
        || !completion.returns_value()
        || result.owner() != original.callee_owner()
        || !std::ptr::eq(completion, result_ref.completion())
        || !result_ref.declared_result_agrees_with_i64_source()
    {
        return Err(reject());
    }
    // This is the source cohort, not executable transport. A mixed
    // CurrentOwner/qualified callee must keep every original incoming row.
    let complete_incoming = source.static_incoming_cohort_v1(original)?;
    Ok(VerifiedLoopStaticBodyScalarSourceV1 {
        original: Rc::clone(original),
        complete_incoming,
        binding,
        site,
        formal: formal.binding,
    })
}
