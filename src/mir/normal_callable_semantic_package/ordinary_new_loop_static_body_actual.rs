//! Selected loop-body scalar source joined to the existing physical signature.
//! This is source/shape evidence only; no ValueId or executable ABI is issued.

use std::rc::Rc;

use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::normal_callable_semantic_package::physical_signature::{
    PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, OwnedExprSiteV1, ResolvedLoopPlacementV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::super::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use super::lexical_instance_call::project_current_owner_loop_scalar_source_v1;
use super::loop_static_source_loan::LoopStaticSourceCallLoanV1;
use super::OrdinaryNewClaimLedgerV1;

#[derive(Debug)]
pub(in crate::mir) struct VerifiedLoopStaticBodyScalarSourceV1 {
    original: Rc<StaticIncomingSourceV1>,
    binding: BindingRefV1,
    site: SourceExprSiteV1,
    formal: BindingRefV1,
}

impl VerifiedLoopStaticBodyScalarSourceV1 {
    pub(in crate::mir) fn corroborates(
        &self,
        loan: &LoopStaticSourceCallLoanV1,
        binding: BindingRefV1,
        site: &SourceExprSiteV1,
    ) -> bool {
        Rc::ptr_eq(&self.original, loan.original())
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
                    issue_scalar_source(self, signatures, loan),
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
    loan: &LoopStaticSourceCallLoanV1,
) -> Result<VerifiedLoopStaticBodyScalarSourceV1, String> {
    let reject = || "[freeze:contract][callable-loop/body-scalar-source-unavailable]".to_owned();
    let original = loan.original();
    let Some((candidate_source, binding, site)) = project_current_owner_loop_scalar_source_v1(
        ledger.borrowed_formal_source.as_ref().ok_or_else(reject)?,
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
    {
        return Err(reject());
    }
    Ok(VerifiedLoopStaticBodyScalarSourceV1 {
        original: Rc::clone(original),
        binding,
        site,
        formal: formal.binding,
    })
}
