//! One-Program helper catalog preparation for normal callable source.
//!
//! Candidate and owner issuance planning borrow the retained source. Only
//! complete plans may consume it through infallible commits.

use crate::mir::resolved_semantics::{
    CallableCatalogCandidateSealErrorV1, CallableCatalogOwnerSealErrorV1,
    CallableCatalogSealOutcomeV1, PreparedCallableCatalogSealV1,
    PreparedOwnerFreeCallableCatalogV1, VerifiedOwnerFreeCallableCatalogSourceUnitV1,
};

use super::callable_source::VerifiedNormalCallableSourceUnitV1;
use super::product::{NormalMainMethodSiteV1, NormalSourceIdentityV1, NormalTopLevelSiteV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalCallableCatalogSourceStageV1 {
    OwnerFreeCandidates,
    CallableOwners,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NormalCallableCatalogSourceErrorV1 {
    OwnerFreeCandidates(CallableCatalogCandidateSealErrorV1),
    CallableOwners(CallableCatalogOwnerSealErrorV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalCallableCatalogSourceUnitV1 {
    catalog: CallableCatalogSealOutcomeV1,
    identity: NormalSourceIdentityV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    _seal: VerifiedNormalCallableCatalogSourceUnitSealV1,
}

#[derive(Debug)]
struct VerifiedNormalCallableCatalogSourceUnitSealV1;

impl VerifiedNormalCallableCatalogSourceUnitV1 {
    pub(crate) fn source_identity(&self) -> &str {
        self.identity.display_name()
    }

    pub(crate) fn main_statement_index(&self) -> usize {
        self.main_box.statement_index()
    }

    pub(crate) fn main_method_key(&self) -> &str {
        self.main_method.method_key()
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        CallableCatalogSealOutcomeV1,
        NormalSourceIdentityV1,
        NormalTopLevelSiteV1,
        NormalMainMethodSiteV1,
    ) {
        (self.catalog, self.identity, self.main_box, self.main_method)
    }

    pub(super) fn restore(
        catalog: CallableCatalogSealOutcomeV1,
        identity: NormalSourceIdentityV1,
        main_box: NormalTopLevelSiteV1,
        main_method: NormalMainMethodSiteV1,
    ) -> Self {
        Self {
            catalog,
            identity,
            main_box,
            main_method,
            _seal: VerifiedNormalCallableCatalogSourceUnitSealV1,
        }
    }
}

#[derive(Debug)]
enum RetainedNormalCallableCatalogSourceV1 {
    Source(VerifiedNormalCallableSourceUnitV1),
    OwnerFree {
        source: VerifiedOwnerFreeCallableCatalogSourceUnitV1,
        identity: NormalSourceIdentityV1,
        main_box: NormalTopLevelSiteV1,
        main_method: NormalMainMethodSiteV1,
    },
}

#[derive(Debug)]
pub(crate) struct RejectedNormalCallableCatalogSourceV1 {
    owner: RetainedNormalCallableCatalogSourceV1,
    stage: NormalCallableCatalogSourceStageV1,
    error: NormalCallableCatalogSourceErrorV1,
}

impl RejectedNormalCallableCatalogSourceV1 {
    pub(crate) const fn stage(&self) -> NormalCallableCatalogSourceStageV1 {
        self.stage
    }

    pub(crate) const fn error(&self) -> &NormalCallableCatalogSourceErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

impl VerifiedNormalCallableSourceUnitV1 {
    pub(crate) fn prepare_helper_catalog(
        self,
        compilation_unit_ordinal: u32,
    ) -> Result<VerifiedNormalCallableCatalogSourceUnitV1, RejectedNormalCallableCatalogSourceV1>
    {
        let candidate_plan = match PreparedOwnerFreeCallableCatalogV1::prepare(self.helper_source())
        {
            Ok(prepared) => prepared,
            Err(error) => {
                return Err(RejectedNormalCallableCatalogSourceV1 {
                    owner: RetainedNormalCallableCatalogSourceV1::Source(self),
                    stage: NormalCallableCatalogSourceStageV1::OwnerFreeCandidates,
                    error: NormalCallableCatalogSourceErrorV1::OwnerFreeCandidates(error),
                })
            }
        };
        let (source, identity, main_box, main_method) = self.into_parts();
        let owner_free = candidate_plan.commit(source);
        let catalog_plan =
            match PreparedCallableCatalogSealV1::prepare(&owner_free, compilation_unit_ordinal) {
                Ok(prepared) => prepared,
                Err(error) => {
                    return Err(RejectedNormalCallableCatalogSourceV1 {
                        owner: RetainedNormalCallableCatalogSourceV1::OwnerFree {
                            source: owner_free,
                            identity,
                            main_box,
                            main_method,
                        },
                        stage: NormalCallableCatalogSourceStageV1::CallableOwners,
                        error: NormalCallableCatalogSourceErrorV1::CallableOwners(error),
                    })
                }
            };
        Ok(VerifiedNormalCallableCatalogSourceUnitV1 {
            catalog: catalog_plan.commit(owner_free),
            identity,
            main_box,
            main_method,
            _seal: VerifiedNormalCallableCatalogSourceUnitSealV1,
        })
    }
}
