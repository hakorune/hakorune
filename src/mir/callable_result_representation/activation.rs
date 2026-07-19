use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{
    observe_method_calls_shadow_view_v0, FunctionSyntaxViewV1, ReceiverPolicyV1, SourceExprSiteV1,
};
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1;

use super::{
    classify_activation_source_site_v1, CallableResultActivationErrorV1,
    CallableResultActivationSourceDecisionV1, VerifiedSameModuleCallableResultCatalogV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableResultActivationDispositionV1 {
    SelectedExactI64 {
        target: CanonicalSameModuleCallableKeyV1,
        required_i64_arguments: Box<[u32]>,
    },
    Unselected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedCallableResultActivationSiteV1 {
    site: SourceExprSiteV1,
    disposition: CallableResultActivationDispositionV1,
}

impl VerifiedCallableResultActivationSiteV1 {
    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn disposition(&self) -> &CallableResultActivationDispositionV1 {
        &self.disposition
    }
}

/// Owned normalization of one exact borrowed target/result proof chain.
///
/// The catalog identity is construction-only.  `seal` consumes this draft
/// only beside the same boxed catalog allocation, after all borrowed proofs
/// have been dropped.
#[derive(Debug)]
pub(crate) struct VerifiedCallableResultActivationRowsV1 {
    catalog_identity: usize,
    rows_by_caller:
        BTreeMap<CanonicalSameModuleCallableKeyV1, Box<[VerifiedCallableResultActivationSiteV1]>>,
}

impl VerifiedCallableResultActivationRowsV1 {
    pub(crate) fn verify(
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        targets: &VerifiedSourceStaticCallTargetCatalogV1<'_>,
        results: &VerifiedSameModuleCallableResultCatalogV1<'_, '_>,
    ) -> Result<Self, CallableResultActivationErrorV1> {
        if !results.is_branded_by(declarations, targets) {
            return Err(CallableResultActivationErrorV1::BorrowedResultCatalogBrandMismatch);
        }

        let mut rows_by_caller = BTreeMap::new();
        let mut observed_sites = BTreeSet::new();
        for (caller, declaration) in declarations.declarations() {
            let receiver_policy = match caller.namespace() {
                SameModuleCallableNamespaceV1::StaticBoxMethod => ReceiverPolicyV1::Absent,
                SameModuleCallableNamespaceV1::InstanceBoxMethod => {
                    ReceiverPolicyV1::DeclaredInstance
                }
            };
            let view = FunctionSyntaxViewV1::from_borrowed_function_parts(
                declaration.params(),
                declaration.body(),
                receiver_policy,
            );
            let observations = observe_method_calls_shadow_view_v0(view).map_err(|error| {
                CallableResultActivationErrorV1::MethodCallInventory {
                    caller: caller.clone(),
                    error,
                }
            })?;
            let mut caller_rows = Vec::with_capacity(observations.len());
            for (site, _) in observations {
                observed_sites.insert((caller.clone(), site.clone()));
                let disposition = match classify_activation_source_site_v1(
                    declarations,
                    caller,
                    &site,
                    targets,
                    results,
                )? {
                    CallableResultActivationSourceDecisionV1::Selected(selected) => {
                        CallableResultActivationDispositionV1::SelectedExactI64 {
                            target: selected.target().clone(),
                            required_i64_arguments: selected
                                .required_i64_arguments()
                                .to_vec()
                                .into_boxed_slice(),
                        }
                    }
                    CallableResultActivationSourceDecisionV1::Unselected(_) => {
                        CallableResultActivationDispositionV1::Unselected
                    }
                };
                caller_rows.push(VerifiedCallableResultActivationSiteV1 { site, disposition });
            }
            rows_by_caller.insert(caller.clone(), caller_rows.into_boxed_slice());
        }

        if let Some(((caller, site), _)) = targets
            .rows()
            .find(|(key, _)| !observed_sites.contains(*key))
        {
            return Err(
                CallableResultActivationErrorV1::SourceTargetRowOutsideInventory {
                    caller: caller.clone(),
                    site: site.clone(),
                },
            );
        }

        Ok(Self {
            catalog_identity: declarations as *const _ as usize,
            rows_by_caller,
        })
    }

    pub(crate) fn len(&self) -> usize {
        self.rows_by_caller.len()
    }
}

/// Owned, non-Clone, single-use callable-result activation input.
#[derive(Debug)]
pub(crate) struct VerifiedCallableResultActivationPlanV1 {
    declaration_catalog: Box<VerifiedSameModuleCallableDeclarationCatalogV1>,
    rows: VerifiedCallableResultActivationRowsV1,
}

impl VerifiedCallableResultActivationPlanV1 {
    pub(crate) fn seal(
        declaration_catalog: Box<VerifiedSameModuleCallableDeclarationCatalogV1>,
        rows: VerifiedCallableResultActivationRowsV1,
    ) -> Result<Self, CallableResultActivationErrorV1> {
        if rows.catalog_identity != declaration_catalog.as_ref() as *const _ as usize {
            return Err(CallableResultActivationErrorV1::ActivationRowsCatalogBrandMismatch);
        }
        Ok(Self {
            declaration_catalog,
            rows,
        })
    }

    pub(crate) fn declaration_catalog(&self) -> &VerifiedSameModuleCallableDeclarationCatalogV1 {
        &self.declaration_catalog
    }

    pub(crate) fn rows_for(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&[VerifiedCallableResultActivationSiteV1]> {
        self.rows.rows_by_caller.get(caller).map(Box::as_ref)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        Box<VerifiedSameModuleCallableDeclarationCatalogV1>,
        VerifiedCallableResultActivationRowsV1,
    ) {
        (self.declaration_catalog, self.rows)
    }
}
