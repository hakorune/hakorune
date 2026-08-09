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
    TargetCatalogBrandMismatch,
    ResultCatalogBrandMismatch,
    Projection {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        cause: StaticExactI64RequirementErrorV1,
    },
    GeneralRowMustBeSameModuleStatic {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    GeneralRowTargetMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        expected: CanonicalSameModuleCallableKeyV1,
        actual: CanonicalSameModuleCallableKeyV1,
    },
    DuplicateSelection {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StaticCallResultPublicationOwnerTakeErrorV1 {
    OwnerUnavailable,
    CatalogBrandMismatch,
    TargetMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        expected: CanonicalSameModuleCallableKeyV1,
        actual: CanonicalSameModuleCallableKeyV1,
    },
    SelectedRowAlreadyConsumed {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StaticCallResultPublicationTakeV1 {
    Unselected,
    Selected(VerifiedStaticCallResultPublicationHandoffV1),
}

/// One candidate-local owner for all currently provable exact static rows.
///
/// Rows are keyed by the source identity that was sealed before lowering.  A
/// consumer must provide the already-resolved canonical target; this owner
/// never selects by source text or method name.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedStaticCallResultPublicationOwnerV1 {
    catalog_identity: usize,
    selected_targets: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        CanonicalSameModuleCallableKeyV1,
    >,
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
        if !targets.is_branded_by(declarations) {
            return Err(StaticCallResultPublicationOwnerErrorV1::TargetCatalogBrandMismatch);
        }
        if !results.is_branded_by(declarations, targets) {
            return Err(StaticCallResultPublicationOwnerErrorV1::ResultCatalogBrandMismatch);
        }
        let catalog_identity = declarations as *const _ as usize;
        let mut selected_targets = BTreeMap::new();
        let mut rows = BTreeMap::new();
        for ((caller, site), source_target) in targets.rows() {
            let key = (caller.clone(), site.clone());
            if let Some(general) = results.call_result(caller, site) {
                let actual = general.static_target_key().ok_or_else(|| {
                    StaticCallResultPublicationOwnerErrorV1::GeneralRowMustBeSameModuleStatic {
                        caller: caller.clone(),
                        site: site.clone(),
                    }
                })?;
                let expected = source_target.target();
                if actual != expected {
                    return Err(
                        StaticCallResultPublicationOwnerErrorV1::GeneralRowTargetMismatch {
                            caller: caller.clone(),
                            site: site.clone(),
                            expected: expected.clone(),
                            actual: actual.clone(),
                        },
                    );
                }
                let handoff =
                    VerifiedStaticCallResultPublicationHandoffV1::from_general_call_result(
                        catalog_identity,
                        caller,
                        site,
                        general,
                    )
                    .ok_or_else(|| {
                        StaticCallResultPublicationOwnerErrorV1::GeneralRowMustBeSameModuleStatic {
                            caller: caller.clone(),
                            site: site.clone(),
                        }
                    })?;
                insert_selected(
                    &mut selected_targets,
                    &mut rows,
                    key,
                    expected.clone(),
                    handoff,
                )?;
                continue;
            }
            let requirement = match project_static_exact_i64_requirement_v1(
                declarations,
                caller,
                site,
                targets,
                results,
            ) {
                Ok(requirement) => requirement,
                Err(StaticExactI64RequirementErrorV1::TargetResultUnavailable) => continue,
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
            insert_selected(
                &mut selected_targets,
                &mut rows,
                key,
                source_target.target().clone(),
                handoff,
            )?;
        }
        Ok(Self {
            catalog_identity,
            selected_targets,
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
    ) -> Result<StaticCallResultPublicationTakeV1, StaticCallResultPublicationOwnerTakeErrorV1>
    {
        if self.catalog_identity != declarations as *const _ as usize {
            return Err(StaticCallResultPublicationOwnerTakeErrorV1::CatalogBrandMismatch);
        }
        let key = (caller.clone(), site.clone());
        let Some(expected) = self.selected_targets.get(&key) else {
            return Ok(StaticCallResultPublicationTakeV1::Unselected);
        };
        if expected != target {
            return Err(
                StaticCallResultPublicationOwnerTakeErrorV1::TargetMismatch {
                    caller: caller.clone(),
                    site: site.clone(),
                    expected: expected.clone(),
                    actual: target.clone(),
                },
            );
        }
        let handoff = self.rows.remove(&key).ok_or_else(|| {
            StaticCallResultPublicationOwnerTakeErrorV1::SelectedRowAlreadyConsumed {
                caller: caller.clone(),
                site: site.clone(),
                target: target.clone(),
            }
        })?;
        Ok(StaticCallResultPublicationTakeV1::Selected(handoff))
    }
}

fn insert_selected(
    selected_targets: &mut BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        CanonicalSameModuleCallableKeyV1,
    >,
    rows: &mut BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        VerifiedStaticCallResultPublicationHandoffV1,
    >,
    key: (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
    target: CanonicalSameModuleCallableKeyV1,
    handoff: VerifiedStaticCallResultPublicationHandoffV1,
) -> Result<(), StaticCallResultPublicationOwnerErrorV1> {
    if selected_targets.contains_key(&key) || rows.contains_key(&key) {
        return Err(
            StaticCallResultPublicationOwnerErrorV1::DuplicateSelection {
                caller: key.0.clone(),
                site: key.1.clone(),
            },
        );
    }
    selected_targets.insert(key.clone(), target);
    rows.insert(key, handoff);
    Ok(())
}
