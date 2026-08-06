//! Candidate-owned source-bound static-call publication rows.
//!
//! The owner is issued from one sealed declaration catalog and keeps only
//! move-only handoffs.  It is intentionally AST/Builder/MIR-free; the raw
//! terminal may consume one row by exact caller/site/target identity.

use std::collections::BTreeMap;

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1;

use super::{
    project_static_exact_i64_requirement_v1, StaticExactI64RequirementErrorV1,
    VerifiedSameModuleCallableResultCatalogV1, VerifiedStaticCallResultPublicationHandoffV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StaticCallResultPublicationOwnerErrorV1 {
    Projection {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        cause: StaticExactI64RequirementErrorV1,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StaticCallResultPublicationOwnerTakeErrorV1 {
    CatalogBrandMismatch,
    TargetMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        expected: CanonicalSameModuleCallableKeyV1,
        actual: CanonicalSameModuleCallableKeyV1,
    },
}

/// One candidate-local owner for all currently provable exact static rows.
///
/// Rows are keyed by the source identity that was sealed before lowering.  A
/// consumer must provide the already-resolved canonical target; this owner
/// never selects by source text or method name.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedStaticCallResultPublicationOwnerV1 {
    catalog_identity: usize,
    rows: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        VerifiedStaticCallResultPublicationHandoffV1,
    >,
}

impl VerifiedStaticCallResultPublicationOwnerV1 {
    pub(crate) fn issue(
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        targets: &VerifiedSourceStaticCallTargetCatalogV1<'_>,
        results: &VerifiedSameModuleCallableResultCatalogV1<'_, '_>,
    ) -> Result<Self, StaticCallResultPublicationOwnerErrorV1> {
        let mut rows = BTreeMap::new();
        for ((caller, site), _target) in targets.rows() {
            let requirement = match project_static_exact_i64_requirement_v1(
                declarations,
                caller,
                site,
                targets,
                results,
            ) {
                Ok(requirement) => requirement,
                Err(
                    StaticExactI64RequirementErrorV1::TargetResultUnavailable
                    | StaticExactI64RequirementErrorV1::GeneralCallResultAlreadyAvailable,
                ) => continue,
                Err(cause) => {
                    return Err(StaticCallResultPublicationOwnerErrorV1::Projection {
                        caller: caller.clone(),
                        site: site.clone(),
                        cause,
                    });
                }
            };
            let handoff = VerifiedStaticCallResultPublicationHandoffV1::from_exact_i64_requirement(
                requirement,
            );
            let key = (caller.clone(), site.clone());
            if rows.insert(key.clone(), handoff).is_some() {
                return Err(StaticCallResultPublicationOwnerErrorV1::Projection {
                    caller: key.0,
                    site: key.1,
                    cause: StaticExactI64RequirementErrorV1::SourceTargetUnavailable,
                });
            }
        }
        Ok(Self {
            catalog_identity: declarations as *const _ as usize,
            rows,
        })
    }

    pub(crate) fn len(&self) -> usize {
        self.rows.len()
    }

    pub(crate) fn take(
        &mut self,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
        target: &CanonicalSameModuleCallableKeyV1,
    ) -> Result<
        Option<VerifiedStaticCallResultPublicationHandoffV1>,
        StaticCallResultPublicationOwnerTakeErrorV1,
    > {
        if self.catalog_identity != declarations as *const _ as usize {
            return Err(StaticCallResultPublicationOwnerTakeErrorV1::CatalogBrandMismatch);
        }
        let key = (caller.clone(), site.clone());
        let Some(handoff) = self.rows.get(&key) else {
            return Ok(None);
        };
        if handoff.target() != target {
            return Err(
                StaticCallResultPublicationOwnerTakeErrorV1::TargetMismatch {
                    caller: caller.clone(),
                    site: site.clone(),
                    expected: handoff.target().clone(),
                    actual: target.clone(),
                },
            );
        }
        Ok(self.rows.remove(&key))
    }
}
