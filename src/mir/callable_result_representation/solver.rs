use std::collections::BTreeMap;

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
    VerifiedSameModuleCallableDeclarationV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1;

use super::body_proof_issue::{
    CallableBodyProofIssueErrorV1, VerifiedUnannotatedCallableBodyResultOutcomeV1,
    VerifiedUnannotatedCallableBodyResultProofV1,
};
use super::call_row::{CallableResultCallRowsV1, VerifiedCallableResultCallSiteV1};
use super::function_proof::{prove_function, FunctionProofOutcomeV1};
use super::{
    CallableResultCatalogErrorV1, CallableResultUnavailableReasonV1,
    VerifiedCallableResultDispositionV1,
};

#[derive(Debug)]
pub(crate) struct VerifiedSameModuleCallableResultCatalogV1<'targets, 'catalog> {
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    targets: &'targets VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    rows_by_key: BTreeMap<CanonicalSameModuleCallableKeyV1, VerifiedCallableResultDispositionV1>,
    call_rows_by_site: CallableResultCallRowsV1<'targets>,
}

impl<'targets, 'catalog> VerifiedSameModuleCallableResultCatalogV1<'targets, 'catalog> {
    pub(crate) fn verify(
        declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
        targets: &'targets VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    ) -> Result<Self, CallableResultCatalogErrorV1> {
        if !targets.is_branded_by(declarations) {
            return Err(CallableResultCatalogErrorV1::SourceTargetCatalogBrandMismatch);
        }

        let static_declarations = declarations.static_declarations().collect::<Vec<_>>();
        let all_declarations = declarations.declarations().collect::<Vec<_>>();
        let static_count = static_declarations.len();
        let static_keys = static_declarations
            .iter()
            .map(|(key, _)| ((*key).clone(), ()))
            .collect::<BTreeMap<_, _>>();
        let all_keys = all_declarations
            .iter()
            .map(|(key, _)| ((*key).clone(), ()))
            .collect::<BTreeMap<_, _>>();
        validate_target_pairing(targets, &all_keys, &static_keys)?;

        // Absence from this construction-only map means Pending. Public result
        // vocabulary is created only once a proof closes permanently.
        let mut rows_by_key = BTreeMap::new();
        let budget = static_count.saturating_add(1);
        let mut construction_stopped = false;
        for _ in 0..budget {
            let mut progress = false;
            for (key, declaration) in &static_declarations {
                if rows_by_key.contains_key(*key) {
                    continue;
                }
                let product = prove_function(declaration, targets, &rows_by_key)?;
                if let Some(disposition) = disposition(key, product.outcome)? {
                    rows_by_key.insert((*key).clone(), disposition);
                    progress = true;
                }
            }
            if rows_by_key.len() == static_count || !progress {
                construction_stopped = true;
                break;
            }
        }
        if !construction_stopped {
            return Err(CallableResultCatalogErrorV1::ResultWorklistDidNotConverge {
                static_declarations: static_count,
            });
        }

        let stalled_keys = static_declarations
            .iter()
            .filter_map(|(key, _)| (!rows_by_key.contains_key(*key)).then_some((*key).clone()))
            .collect::<std::collections::BTreeSet<_>>();
        for key in &stalled_keys {
            rows_by_key.entry(key.clone()).or_insert_with(|| {
                VerifiedCallableResultDispositionV1::Unavailable(
                    CallableResultUnavailableReasonV1::RecursiveDependency,
                )
            });
        }
        if rows_by_key.len() != static_count {
            return Err(CallableResultCatalogErrorV1::ResultRowCardinalityMismatch {
                static_declarations: static_count,
                rows: rows_by_key.len(),
            });
        }

        let mut call_rows_by_site = CallableResultCallRowsV1::new();
        for (key, declaration) in static_declarations {
            let product = prove_function(declaration, targets, &rows_by_key)?;
            let stable = disposition(key, product.outcome)?;
            let Some(stored) = rows_by_key.get(key) else {
                return Err(CallableResultCatalogErrorV1::StableResultDrift { key: key.clone() });
            };
            if !final_disposition_matches(stored, stable.as_ref(), stalled_keys.contains(key)) {
                return Err(CallableResultCatalogErrorV1::StableResultDrift { key: key.clone() });
            }
            for (row_key, row) in product.call_rows {
                if let Some(existing) = call_rows_by_site.get(&row_key) {
                    if !existing.semantically_matches(&row) {
                        return Err(CallableResultCatalogErrorV1::DuplicateCallResultSite {
                            caller: row_key.0,
                            site: row_key.1,
                        });
                    }
                } else {
                    call_rows_by_site.insert(row_key, row);
                }
            }
        }

        Ok(Self {
            declarations,
            targets,
            rows_by_key,
            call_rows_by_site,
        })
    }

    pub(crate) fn len(&self) -> usize {
        self.rows_by_key.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.rows_by_key.is_empty()
    }

    /// Issues one opaque body proof without widening this static-row catalog.
    ///
    /// The caller must already hold an exact declaration from this catalog.
    /// Instance/current-owner route selection intentionally remains outside
    /// this module.
    pub(in crate::mir) fn issue_unannotated_body_proof(
        &self,
        target: &'catalog VerifiedSameModuleCallableDeclarationV1,
    ) -> Result<VerifiedUnannotatedCallableBodyResultProofV1<'catalog>, CallableBodyProofIssueErrorV1>
    {
        let Some(owned_target) = self.declarations.declaration(target.key()) else {
            return Err(CallableBodyProofIssueErrorV1::TargetOutsideCatalog {
                target: target.key().clone(),
            });
        };
        if !std::ptr::eq(owned_target, target) {
            return Err(CallableBodyProofIssueErrorV1::TargetOutsideCatalog {
                target: target.key().clone(),
            });
        }
        if target.return_type_name().is_some() {
            return Err(
                CallableBodyProofIssueErrorV1::DeclaredResultAuthorityForbidden {
                    target: target.key().clone(),
                },
            );
        }

        let product = prove_function(target, self.targets, &self.rows_by_key)
            .map_err(CallableBodyProofIssueErrorV1::Catalog)?;
        let outcome = match product.outcome {
            FunctionProofOutcomeV1::Exact(requirements) => {
                VerifiedUnannotatedCallableBodyResultOutcomeV1::ExactI64 {
                    required_i64_arguments: requirements.into_iter().collect(),
                }
            }
            FunctionProofOutcomeV1::Unavailable(reason) => {
                VerifiedUnannotatedCallableBodyResultOutcomeV1::Unavailable(reason)
            }
            FunctionProofOutcomeV1::PendingDependency => {
                VerifiedUnannotatedCallableBodyResultOutcomeV1::PendingDependency
            }
        };
        Ok(VerifiedUnannotatedCallableBodyResultProofV1::new(
            target, outcome,
        ))
    }

    pub(crate) fn disposition(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&VerifiedCallableResultDispositionV1> {
        self.rows_by_key.get(key)
    }

    pub(crate) fn call_result(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedCallableResultCallSiteV1<'targets>> {
        self.call_rows_by_site.get(&(caller.clone(), site.clone()))
    }

    pub(crate) fn rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &CanonicalSameModuleCallableKeyV1,
            &VerifiedCallableResultDispositionV1,
        ),
    > {
        self.rows_by_key.iter()
    }

    pub(crate) fn call_rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &(CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
            &VerifiedCallableResultCallSiteV1<'targets>,
        ),
    > {
        self.call_rows_by_site.iter()
    }

    pub(crate) fn is_branded_by(
        &self,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        targets: &VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    ) -> bool {
        std::ptr::eq(self.declarations, declarations) && std::ptr::eq(self.targets, targets)
    }
}

fn final_disposition_matches(
    stored: &VerifiedCallableResultDispositionV1,
    observed: Option<&VerifiedCallableResultDispositionV1>,
    stalled: bool,
) -> bool {
    if stalled {
        return matches!(
            (stored, observed),
            (
                VerifiedCallableResultDispositionV1::Unavailable(
                    CallableResultUnavailableReasonV1::RecursiveDependency,
                ),
                Some(VerifiedCallableResultDispositionV1::Unavailable(_)),
            )
        );
    }
    Some(stored) == observed
}

fn disposition(
    key: &CanonicalSameModuleCallableKeyV1,
    outcome: FunctionProofOutcomeV1,
) -> Result<Option<VerifiedCallableResultDispositionV1>, CallableResultCatalogErrorV1> {
    match outcome {
        FunctionProofOutcomeV1::Exact(requirements) => Ok(Some(
            VerifiedCallableResultDispositionV1::exact_i64(key, requirements)?,
        )),
        FunctionProofOutcomeV1::Unavailable(reason) => Ok(Some(
            VerifiedCallableResultDispositionV1::Unavailable(reason),
        )),
        FunctionProofOutcomeV1::PendingDependency => Ok(None),
    }
}

#[cfg(test)]
mod final_disposition_tests {
    use super::{
        final_disposition_matches, CallableResultUnavailableReasonV1,
        VerifiedCallableResultDispositionV1,
    };

    fn recursive() -> VerifiedCallableResultDispositionV1 {
        VerifiedCallableResultDispositionV1::Unavailable(
            CallableResultUnavailableReasonV1::RecursiveDependency,
        )
    }

    #[test]
    fn stalled_rows_retain_recursive_closure_only_while_reproof_stays_unavailable() {
        let stored = recursive();
        let different_unavailable = VerifiedCallableResultDispositionV1::Unavailable(
            CallableResultUnavailableReasonV1::UnknownExpression,
        );
        let exact = VerifiedCallableResultDispositionV1::ExactI64 {
            required_i64_arguments: Box::new([]),
        };

        assert!(final_disposition_matches(
            &stored,
            Some(&different_unavailable),
            true,
        ));
        assert!(!final_disposition_matches(&stored, Some(&exact), true));
        assert!(!final_disposition_matches(&stored, None, true));
        assert!(!final_disposition_matches(
            &different_unavailable,
            Some(&stored),
            false,
        ));
    }
}

fn validate_target_pairing(
    targets: &VerifiedSourceStaticCallTargetCatalogV1<'_>,
    all_keys: &BTreeMap<CanonicalSameModuleCallableKeyV1, ()>,
    static_keys: &BTreeMap<CanonicalSameModuleCallableKeyV1, ()>,
) -> Result<(), CallableResultCatalogErrorV1> {
    for ((caller, site), row) in targets.rows() {
        if !all_keys.contains_key(caller) {
            return Err(
                CallableResultCatalogErrorV1::SourceTargetCallerOutsideResultCatalog {
                    caller: caller.clone(),
                    site: site.clone(),
                },
            );
        }
        if !static_keys.contains_key(row.target()) {
            return Err(
                CallableResultCatalogErrorV1::SourceTargetOutsideResultCatalog {
                    caller: caller.clone(),
                    site: site.clone(),
                    target: row.target().clone(),
                },
            );
        }
    }
    Ok(())
}
