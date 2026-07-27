//! Owned one-shot bridge from a sealed nested Integer contract back to the
//! existing borrowed contract vocabulary.
//!
//! The witness stores only identities already proven by the original contract.
//! Rebinding checks one exact shared catalog and one exact source MethodCall;
//! it never rebuilds callable-result evidence or inspects Builder state.

use std::ptr;
use std::sync::Arc;

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::VerifiedSourceMethodCallSiteV1;

use super::{
    CurrentOwnerInstanceResultTargetErrorV1, SealedNestedInstanceResultContractV1,
    VerifiedCurrentOwnerInstanceResultTargetV1,
};

#[derive(Debug)]
pub(crate) struct OwnedNestedInstanceResultRebindWitnessV1 {
    catalog_identity: usize,
    caller: CanonicalSameModuleCallableKeyV1,
    site: SourceExprSiteV1,
    target: CanonicalSameModuleCallableKeyV1,
    seal: OwnedNestedInstanceResultRebindWitnessSealV1,
}

#[derive(Debug)]
pub(super) struct OwnedNestedInstanceResultRebindWitnessSealV1(());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedInstanceResultRebindStageV1 {
    CatalogAllocation,
    Caller,
    TargetRelation,
    Target,
    Site,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NestedInstanceResultRebindErrorV1 {
    ForeignCatalog,
    CallerMismatch,
    TargetRelation(CurrentOwnerInstanceResultTargetErrorV1),
    TargetMismatch,
    SiteMismatch,
}

#[derive(Debug)]
pub(crate) struct RejectedNestedInstanceResultRebindV1<'site, 'catalog> {
    witness: OwnedNestedInstanceResultRebindWitnessV1,
    call: &'site VerifiedSourceMethodCallSiteV1<'catalog>,
    stage: NestedInstanceResultRebindStageV1,
    cause: NestedInstanceResultRebindErrorV1,
}

/// Terminal owner retained after a failed source reconstruction.
///
/// It deliberately exposes no witness recovery or rebind terminal. Higher
/// layers may retain and discard it, but cannot retry the rejected authority.
#[derive(Debug)]
pub(crate) struct RetainedNestedInstanceResultRebindAuthorityV1 {
    witness: OwnedNestedInstanceResultRebindWitnessV1,
}

impl<'site, 'catalog> SealedNestedInstanceResultContractV1<'site, 'catalog> {
    pub(crate) fn into_owned_rebind_witness(self) -> OwnedNestedInstanceResultRebindWitnessV1 {
        let target = self.into_rebind_target();
        OwnedNestedInstanceResultRebindWitnessV1 {
            catalog_identity: target.call().catalog() as *const _ as usize,
            caller: target.call().caller().clone(),
            site: target.call().site().clone(),
            target: target.target().key().clone(),
            seal: OwnedNestedInstanceResultRebindWitnessSealV1(()),
        }
    }
}

impl OwnedNestedInstanceResultRebindWitnessV1 {
    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.caller
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }
}

impl<'site, 'catalog> RejectedNestedInstanceResultRebindV1<'site, 'catalog> {
    pub(crate) const fn stage(&self) -> NestedInstanceResultRebindStageV1 {
        self.stage
    }

    pub(crate) const fn cause(&self) -> &NestedInstanceResultRebindErrorV1 {
        &self.cause
    }

    pub(crate) fn discard(self) {
        let _ = (self.witness, self.call);
    }

    pub(crate) fn into_retained_authority(self) -> RetainedNestedInstanceResultRebindAuthorityV1 {
        RetainedNestedInstanceResultRebindAuthorityV1 {
            witness: self.witness,
        }
    }
}

impl RetainedNestedInstanceResultRebindAuthorityV1 {
    pub(crate) fn discard(self) {
        let _ = self.witness;
    }

    pub(super) fn from_witness(witness: OwnedNestedInstanceResultRebindWitnessV1) -> Self {
        Self { witness }
    }
}

pub(crate) fn rebind_nested_instance_result_contract_v1<'site, 'catalog>(
    witness: OwnedNestedInstanceResultRebindWitnessV1,
    catalog: &Arc<VerifiedSameModuleCallableDeclarationCatalogV1>,
    call: &'site VerifiedSourceMethodCallSiteV1<'catalog>,
) -> Result<
    SealedNestedInstanceResultContractV1<'site, 'catalog>,
    RejectedNestedInstanceResultRebindV1<'site, 'catalog>,
> {
    if witness.catalog_identity != Arc::as_ptr(catalog) as usize
        || !ptr::eq(catalog.as_ref(), call.catalog())
    {
        return Err(reject(
            witness,
            call,
            NestedInstanceResultRebindStageV1::CatalogAllocation,
            NestedInstanceResultRebindErrorV1::ForeignCatalog,
        ));
    }
    if witness.caller != *call.caller() {
        return Err(reject(
            witness,
            call,
            NestedInstanceResultRebindStageV1::Caller,
            NestedInstanceResultRebindErrorV1::CallerMismatch,
        ));
    }
    let target = match VerifiedCurrentOwnerInstanceResultTargetV1::seal(call) {
        Ok(target) => target,
        Err(cause) => {
            return Err(reject(
                witness,
                call,
                NestedInstanceResultRebindStageV1::TargetRelation,
                NestedInstanceResultRebindErrorV1::TargetRelation(cause),
            ))
        }
    };
    if witness.target != *target.target().key() {
        return Err(reject(
            witness,
            call,
            NestedInstanceResultRebindStageV1::Target,
            NestedInstanceResultRebindErrorV1::TargetMismatch,
        ));
    }
    if witness.site != *call.site() {
        return Err(reject(
            witness,
            call,
            NestedInstanceResultRebindStageV1::Site,
            NestedInstanceResultRebindErrorV1::SiteMismatch,
        ));
    }
    Ok(SealedNestedInstanceResultContractV1::from_owned_rebind(
        target,
        witness.seal,
    ))
}

fn reject<'site, 'catalog>(
    witness: OwnedNestedInstanceResultRebindWitnessV1,
    call: &'site VerifiedSourceMethodCallSiteV1<'catalog>,
    stage: NestedInstanceResultRebindStageV1,
    cause: NestedInstanceResultRebindErrorV1,
) -> RejectedNestedInstanceResultRebindV1<'site, 'catalog> {
    RejectedNestedInstanceResultRebindV1 {
        witness,
        call,
        stage,
        cause,
    }
}

#[cfg(test)]
#[path = "owned_rebind_tests.rs"]
mod tests;
