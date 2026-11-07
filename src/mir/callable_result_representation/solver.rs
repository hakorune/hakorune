use std::collections::BTreeMap;

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1;

use super::call_row::{CallableResultCallRowsV1, VerifiedCallableResultCallSiteV1};
use super::function_proof::{declares_bool_result, prove_function, FunctionProofOutcomeV1};
use super::{
    CallableResultCatalogErrorV1, CallableResultUnavailableReasonV1,
    VerifiedCallableResultDispositionV1,
};

#[derive(Debug)]
pub(crate) struct VerifiedSameModuleCallableResultCatalogV1<'targets, 'catalog> {
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    targets: &'targets VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    rows_by_key: BTreeMap<CanonicalSameModuleCallableKeyV1, VerifiedCallableResultDispositionV1>,
    source_value_classes: BTreeMap<CanonicalSameModuleCallableKeyV1, super::CallableSourceValueClassV1>,
    call_rows_by_site: CallableResultCallRowsV1<'targets>,
}

impl<'targets, 'catalog> VerifiedSameModuleCallableResultCatalogV1<'targets, 'catalog> {
    pub(crate) fn verify(
        declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
        targets: &'targets VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    ) -> Result<Self, CallableResultCatalogErrorV1> {
        Self::verify_scope(declarations, targets, false)
    }

    pub(crate) fn verify_for_package(
        declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
        targets: &'targets VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    ) -> Result<Self, CallableResultCatalogErrorV1> {
        Self::verify_scope(declarations, targets, true)
    }

    fn verify_scope(
        declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
        targets: &'targets VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
        include_free_functions: bool,
    ) -> Result<Self, CallableResultCatalogErrorV1> {
        if !targets.is_branded_by(declarations) {
            return Err(CallableResultCatalogErrorV1::SourceTargetCatalogBrandMismatch);
        }

        let static_declarations = declarations.static_declarations().collect::<Vec<_>>();
        let all_declarations = declarations.declarations().collect::<Vec<_>>();
        // Bare and static methods use the SAME result proof. Static target
        // membership below remains restricted to its original namespace.
        let proof_declarations = all_declarations.iter().copied().filter(|(key, _)| {
            key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                || (include_free_functions && key.namespace() == hakorune_mir_defs::SameModuleCallableNamespaceV1::FreeFunction)
        }).collect::<Vec<_>>();
        let static_count = proof_declarations.len();
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
        let mut source_value_classes = BTreeMap::new();
        let budget = static_declarations.len().saturating_add(1);
        let source_budget = static_count.saturating_mul(2).saturating_add(1);
        let mut construction_stopped = false;
        for _ in 0..source_budget {
            let mut progress = false;
            for (key, declaration) in &proof_declarations {
                let closed = rows_by_key.contains_key(*key);
                if closed { continue; }
                let product = prove_function(declaration, targets, &rows_by_key, &source_value_classes)?;
                if let Some(class) = product.source_value_class {
                    if source_value_classes.get(*key).is_some_and(|stored| stored != &class) {
                        return Err(CallableResultCatalogErrorV1::StableResultDrift { key: (*key).clone() });
                    }
                    if source_value_classes.insert((*key).clone(), class).is_none() { progress = true; }
                }
                if let Some(disposition) = disposition(key, product.outcome)? {
                    rows_by_key.insert((*key).clone(), disposition);
                    progress = true;
                }
            }
            if !progress {
                construction_stopped = true;
                break;
            }
        }
        if !construction_stopped {
            return Err(CallableResultCatalogErrorV1::ResultWorklistDidNotConverge {
                static_declarations: static_count,
            });
        }

        // R1: a stalled group of declared `Bool` callables (a recursion) is closed from its
        // declarations: callers read the declared kind first, and each body must then prove
        // exactly that kind. The requirement sets start empty and only grow, so the
        // iteration converges; any body that does not come out `ExactBool` drops every seed
        // and the group stays recursive-unavailable as before.
        let mut seeds = BTreeMap::new();
        for (key, declaration) in &static_declarations {
            if !rows_by_key.contains_key(*key) && declares_bool_result(declaration) {
                seeds.insert(
                    (*key).clone(),
                    VerifiedCallableResultDispositionV1::exact_bool(key, std::iter::empty())?,
                );
            }
        }
        if !seeds.is_empty() {
            let mut closed = false;
            for _ in 0..budget.saturating_add(seeds.len()) {
                let mut view = rows_by_key.clone();
                view.extend(seeds.iter().map(|(key, row)| (key.clone(), row.clone())));
                let mut changed = false;
                let mut agrees = true;
                for (key, declaration) in &static_declarations {
                    if !seeds.contains_key(*key) {
                        continue;
                    }
                    let product = prove_function(declaration, targets, &view, &source_value_classes)?;
                    match disposition(key, product.outcome)? {
                        Some(row @ VerifiedCallableResultDispositionV1::ExactBool { .. }) => {
                            if seeds.get(*key) != Some(&row) {
                                seeds.insert((*key).clone(), row);
                                changed = true;
                            }
                        }
                        _ => agrees = false,
                    }
                }
                if !agrees {
                    break;
                }
                if !changed {
                    closed = true;
                    break;
                }
            }
            if closed {
                rows_by_key.extend(seeds);
            }
        }

        let stalled_keys = proof_declarations
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
        for (key, declaration) in proof_declarations {
            let product = prove_function(declaration, targets, &rows_by_key, &source_value_classes)?;
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
            source_value_classes,
            call_rows_by_site,
        })
    }

    pub(crate) fn len(&self) -> usize {
        self.rows_by_key.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.rows_by_key.is_empty()
    }

    pub(crate) fn disposition(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&VerifiedCallableResultDispositionV1> {
        self.rows_by_key.get(key)
    }

    /// Move the SAME verified function and call-site rows into the package.
    /// No second solver run, source walk, or representation issuer is involved.
    pub(crate) fn into_source_rows(
        self,
    ) -> (
        BTreeMap<CanonicalSameModuleCallableKeyV1, VerifiedCallableResultDispositionV1>,
        BTreeMap<
            (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
            super::call_row::OwnedCallableResultCallSiteV1,
        >,
        BTreeMap<CanonicalSameModuleCallableKeyV1, super::CallableSourceValueClassV1>,
    ) {
        (
            self.rows_by_key,
            self.call_rows_by_site
                .into_iter()
                .map(|(site, row)| (site, row.into_owned()))
                .collect(),
            self.source_value_classes,
        )
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
        FunctionProofOutcomeV1::ExactBool(requirements) => Ok(Some(
            VerifiedCallableResultDispositionV1::exact_bool(key, requirements)?,
        )),
        FunctionProofOutcomeV1::ExactString => {
            Ok(Some(VerifiedCallableResultDispositionV1::ExactString))
        }
        FunctionProofOutcomeV1::ExactNominalBox(box_name, requirements) => Ok(Some(
            VerifiedCallableResultDispositionV1::exact_nominal_box(key, box_name, requirements)?,
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
