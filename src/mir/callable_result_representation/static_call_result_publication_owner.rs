//! Candidate-owned source-bound static-call publication rows.
//!
//! The owner is issued from one sealed declaration catalog and keeps only
//! move-only handoffs.  It is intentionally AST/Builder/MIR-free; the raw
//! terminal may consume one row by exact caller/site/target identity.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1;

use super::{
    project_static_exact_i64_requirement_v1, CallableResultUnavailableReasonV1,
    StaticExactI64RequirementErrorV1, VerifiedCallableResultDispositionV1,
    VerifiedSameModuleCallableResultCatalogV1, VerifiedStaticCallResultPublicationHandoffV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StaticCallResultTargetOnlyV1 {
    target: CanonicalSameModuleCallableKeyV1,
    reason: CallableResultUnavailableReasonV1,
}

impl StaticCallResultTargetOnlyV1 {
    pub(crate) fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }

    pub(crate) fn reason(&self) -> &CallableResultUnavailableReasonV1 {
        &self.reason
    }
}

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
    TargetOnlyDispositionMissing {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    },
    TargetOnlyDispositionMustBeUnavailable {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
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
    RowAlreadyConsumed {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    },
    TargetDispositionMissing {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StaticCallResultPublicationOwnerFinishErrorV1 {
    UnconsumedSelected {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    },
    UnconsumedTargetOnly {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
        reason: CallableResultUnavailableReasonV1,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StaticCallResultPublicationTakeV1 {
    NoExactStaticTarget,
    TargetOnly(StaticCallResultTargetOnlyV1),
    Selected(VerifiedStaticCallResultPublicationHandoffV1),
}

/// One candidate-local owner for all currently provable exact static rows.
///
/// Rows are keyed by the source identity that was sealed before lowering.  A
/// consumer selects only by that exact source key; this owner never selects
/// by source text or method name.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedStaticCallResultPublicationOwnerV1 {
    catalog_identity: usize,
    exact_targets: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        CanonicalSameModuleCallableKeyV1,
    >,
    selected_targets: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        CanonicalSameModuleCallableKeyV1,
    >,
    target_only_targets: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        StaticCallResultTargetOnlyV1,
    >,
    rows: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        VerifiedStaticCallResultPublicationHandoffV1,
    >,
    consumed_sites: BTreeSet<(CanonicalSameModuleCallableKeyV1, SourceExprSiteV1)>,
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
        let catalog_identity = declarations.brand().identity();
        let mut exact_targets = BTreeMap::new();
        let mut selected_targets = BTreeMap::new();
        let mut target_only_targets = BTreeMap::new();
        let mut rows = BTreeMap::new();
        for ((caller, site), source_target) in targets.rows() {
            let key = (caller.clone(), site.clone());
            let expected = source_target.target().clone();
            insert_exact_target(&mut exact_targets, key.clone(), expected.clone())?;
            if let Some(general) = results.call_result(caller, site) {
                let actual = general.static_target_key().ok_or_else(|| {
                    StaticCallResultPublicationOwnerErrorV1::GeneralRowMustBeSameModuleStatic {
                        caller: caller.clone(),
                        site: site.clone(),
                    }
                })?;
                if actual != &expected {
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
                insert_selected(&mut selected_targets, &mut rows, key, expected, handoff)?;
                continue;
            }
            if matches!(
                results.disposition(&expected),
                Some(
                    VerifiedCallableResultDispositionV1::ExactString
                        | VerifiedCallableResultDispositionV1::ExactBool
                )
            ) {
                let handoff =
                    VerifiedStaticCallResultPublicationHandoffV1::project_unconditional_result(
                        declarations,
                        caller,
                        site,
                        targets,
                        results,
                    )
                    .map_err(|cause| {
                        StaticCallResultPublicationOwnerErrorV1::Projection {
                            caller: caller.clone(),
                            site: site.clone(),
                            cause,
                        }
                    })?;
                insert_selected(&mut selected_targets, &mut rows, key, expected, handoff)?;
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
                Err(StaticExactI64RequirementErrorV1::TargetResultUnavailable) => {
                    let disposition = results.disposition(&expected).ok_or_else(|| {
                        StaticCallResultPublicationOwnerErrorV1::TargetOnlyDispositionMissing {
                            caller: caller.clone(),
                            site: site.clone(),
                            target: expected.clone(),
                        }
                    })?;
                    let VerifiedCallableResultDispositionV1::Unavailable(reason) = disposition
                    else {
                        return Err(
                            StaticCallResultPublicationOwnerErrorV1::TargetOnlyDispositionMustBeUnavailable {
                                caller: caller.clone(),
                                site: site.clone(),
                                target: expected.clone(),
                            },
                        );
                    };
                    insert_target_only(
                        &mut target_only_targets,
                        key,
                        StaticCallResultTargetOnlyV1 {
                            target: expected,
                            reason: reason.clone(),
                        },
                    )?;
                    continue;
                }
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
            insert_selected(&mut selected_targets, &mut rows, key, expected, handoff)?;
        }
        Ok(Self {
            catalog_identity,
            exact_targets,
            selected_targets,
            target_only_targets,
            rows,
            consumed_sites: BTreeSet::new(),
        })
    }

    pub(crate) fn pending_len(&self) -> usize {
        self.rows.len() + self.target_only_targets.len()
    }

    pub(crate) fn pending_selected_len(&self) -> usize {
        self.rows.len()
    }

    pub(crate) fn pending_target_only_len(&self) -> usize {
        self.target_only_targets.len()
    }

    pub(crate) fn target_for_source(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<&CanonicalSameModuleCallableKeyV1> {
        self.exact_targets.get(&(caller.clone(), site.clone()))
    }

    /// Borrowed peek at one selected publication row.  This never consumes
    /// the row; `take_for_source` remains the sole consumption boundary for
    /// the later physical consumer.
    pub(crate) fn selected_handoff_for_source(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedStaticCallResultPublicationHandoffV1> {
        self.rows.get(&(caller.clone(), site.clone()))
    }

    pub(crate) fn finish_empty(&self) -> Result<(), StaticCallResultPublicationOwnerFinishErrorV1> {
        if let Some(((caller, site), _handoff)) = self.rows.iter().next() {
            let target = self
                .selected_targets
                .get(&(caller.clone(), site.clone()))
                .cloned()
                .expect("selected publication row must retain its target index");
            return Err(
                StaticCallResultPublicationOwnerFinishErrorV1::UnconsumedSelected {
                    caller: caller.clone(),
                    site: site.clone(),
                    target,
                },
            );
        }
        if let Some(((caller, site), target_only)) = self.target_only_targets.iter().next() {
            return Err(
                StaticCallResultPublicationOwnerFinishErrorV1::UnconsumedTargetOnly {
                    caller: caller.clone(),
                    site: site.clone(),
                    target: target_only.target.clone(),
                    reason: target_only.reason.clone(),
                },
            );
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn target_only_for_test(
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    ) -> Self {
        let key = (caller, site);
        let mut exact_targets = BTreeMap::new();
        exact_targets.insert(key.clone(), target.clone());
        let mut target_only_targets = BTreeMap::new();
        target_only_targets.insert(
            key,
            StaticCallResultTargetOnlyV1 {
                target,
                reason: CallableResultUnavailableReasonV1::KnownNonI64Return,
            },
        );
        Self {
            catalog_identity: 0,
            exact_targets,
            selected_targets: BTreeMap::new(),
            target_only_targets,
            rows: BTreeMap::new(),
            consumed_sites: BTreeSet::new(),
        }
    }

    pub(crate) fn take_for_source(
        &mut self,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Result<StaticCallResultPublicationTakeV1, StaticCallResultPublicationOwnerTakeErrorV1>
    {
        if self.catalog_identity != declarations.brand().identity() {
            return Err(StaticCallResultPublicationOwnerTakeErrorV1::CatalogBrandMismatch);
        }
        let key = (caller.clone(), site.clone());
        if let Some(expected) = self.exact_targets.get(&key).cloned() {
            if self.consumed_sites.contains(&key) {
                return Err(
                    StaticCallResultPublicationOwnerTakeErrorV1::RowAlreadyConsumed {
                        caller: caller.clone(),
                        site: site.clone(),
                        target: expected,
                    },
                );
            }
            if let Some(target_only) = self.target_only_targets.remove(&key) {
                self.consumed_sites.insert(key);
                return Ok(StaticCallResultPublicationTakeV1::TargetOnly(target_only));
            }
            if self.selected_targets.contains_key(&key) {
                let handoff = self.rows.remove(&key).ok_or_else(|| {
                    StaticCallResultPublicationOwnerTakeErrorV1::TargetDispositionMissing {
                        caller: caller.clone(),
                        site: site.clone(),
                        target: expected.clone(),
                    }
                })?;
                self.consumed_sites.insert(key);
                return Ok(StaticCallResultPublicationTakeV1::Selected(handoff));
            }
            return Err(
                StaticCallResultPublicationOwnerTakeErrorV1::TargetDispositionMissing {
                    caller: caller.clone(),
                    site: site.clone(),
                    target: expected,
                },
            );
        }
        Ok(StaticCallResultPublicationTakeV1::NoExactStaticTarget)
    }
}

fn insert_exact_target(
    exact_targets: &mut BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        CanonicalSameModuleCallableKeyV1,
    >,
    key: (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
    target: CanonicalSameModuleCallableKeyV1,
) -> Result<(), StaticCallResultPublicationOwnerErrorV1> {
    if exact_targets.contains_key(&key) {
        return Err(
            StaticCallResultPublicationOwnerErrorV1::DuplicateSelection {
                caller: key.0.clone(),
                site: key.1.clone(),
            },
        );
    }
    exact_targets.insert(key, target);
    Ok(())
}

fn insert_target_only(
    target_only_targets: &mut BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        StaticCallResultTargetOnlyV1,
    >,
    key: (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
    target_only: StaticCallResultTargetOnlyV1,
) -> Result<(), StaticCallResultPublicationOwnerErrorV1> {
    if target_only_targets.contains_key(&key) {
        return Err(
            StaticCallResultPublicationOwnerErrorV1::DuplicateSelection {
                caller: key.0.clone(),
                site: key.1.clone(),
            },
        );
    }
    target_only_targets.insert(key, target_only);
    Ok(())
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
