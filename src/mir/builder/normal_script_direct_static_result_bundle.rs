//! Source-bound Script direct-static result facts.
//!
//! This bundle is the only bridge between the resolver's Script MethodCall
//! rows and the later Script Facts/Recipe consumer.  It owns no Builder value,
//! MIR type, Recipe key, or physical block.  The source-package lookup
//! relation supplies the callee and exact source sites; resolver rows verify
//! the same source tree before the bundle is issued.

use std::collections::BTreeMap;

use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::mir::source_call_target::VerifiedScriptDirectStaticCallLookupV1;

use super::normal_script_semantic_source::VerifiedScriptSemanticSourceV1;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ScriptDirectStaticResultBundleErrorV1 {
    ScriptRootMissing,
    ScriptRootNotScript,
    ResolverTargetSiteMissing(SourceExprSiteV1),
    ResolverOwnerMismatch(SourceExprSiteV1),
    ReceiverSiteMismatch(SourceExprSiteV1),
    ReceiverDispositionMismatch(SourceExprSiteV1),
    ArgumentSiteMismatch {
        site: SourceExprSiteV1,
        ordinal: u32,
    },
    ResultSiteMismatch(SourceExprSiteV1),
    TargetNamespaceMismatch(SourceExprSiteV1),
    TargetArityMismatch(SourceExprSiteV1),
    DuplicateSite(SourceExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VerifiedScriptDirectStaticResultDemandV1 {
    source_owner: FunctionOwnerIdV1,
    site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    argument_sites: Box<[SourceExprSiteV1]>,
    result_site: SourceExprSiteV1,
    target: CanonicalSameModuleCallableKeyV1,
    representation: VerifiedCallableResultRepresentationV1,
    required_callee_i64_arguments: Box<[u32]>,
}

impl VerifiedScriptDirectStaticResultDemandV1 {
    #[cfg(test)]
    pub(super) fn from_parts_for_test(
        source_owner: FunctionOwnerIdV1,
        site: SourceExprSiteV1,
        receiver_site: SourceExprSiteV1,
        argument_sites: Box<[SourceExprSiteV1]>,
        result_site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
        representation: VerifiedCallableResultRepresentationV1,
        required_callee_i64_arguments: Box<[u32]>,
    ) -> Self {
        Self {
            source_owner,
            site,
            receiver_site,
            argument_sites,
            result_site,
            target,
            representation,
            required_callee_i64_arguments,
        }
    }

    pub(super) const fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.source_owner
    }

    pub(super) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(super) const fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(super) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }

    pub(super) const fn result_site(&self) -> &SourceExprSiteV1 {
        &self.result_site
    }

    pub(super) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }

    pub(super) const fn representation(&self) -> &VerifiedCallableResultRepresentationV1 {
        &self.representation
    }

    pub(super) fn required_callee_i64_arguments(&self) -> &[u32] {
        &self.required_callee_i64_arguments
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct VerifiedScriptDirectStaticResultBundleV1 {
    source_owner: FunctionOwnerIdV1,
    source_identity: usize,
    rows: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticResultDemandV1>,
}

impl VerifiedScriptDirectStaticResultBundleV1 {
    #[cfg(test)]
    pub(super) fn from_parts_for_test(
        source_owner: FunctionOwnerIdV1,
        source_identity: usize,
        rows: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticResultDemandV1>,
    ) -> Self {
        Self {
            source_owner,
            source_identity,
            rows,
        }
    }

    pub(super) fn issue(
        source: &VerifiedScriptSemanticSourceV1<'_>,
        _window: &VerifiedScriptRootDemandWindowV1,
        lookup: VerifiedScriptDirectStaticCallLookupV1,
    ) -> Result<Self, ScriptDirectStaticResultBundleErrorV1> {
        let [root] = source.forest().roots() else {
            return Err(ScriptDirectStaticResultBundleErrorV1::ScriptRootMissing);
        };
        let Some(product) = source
            .forest()
            .semantic_owner(*root)
            .and_then(crate::mir::resolved_semantics::VerifiedSemanticOwnerProductV1::as_script)
        else {
            return Err(ScriptDirectStaticResultBundleErrorV1::ScriptRootNotScript);
        };
        let source_owner = product.core().data().owner;
        let method_rows = product.method_calls().collect::<BTreeMap<_, _>>();
        let mut rows = BTreeMap::new();
        for (site, lookup_row) in lookup.rows() {
            let Some(method) = method_rows.get(site) else {
                return Err(
                    ScriptDirectStaticResultBundleErrorV1::ResolverTargetSiteMissing(site.clone()),
                );
            };
            if method.owner() != source_owner {
                return Err(
                    ScriptDirectStaticResultBundleErrorV1::ResolverOwnerMismatch(site.clone()),
                );
            }
            if method.receiver_site() != lookup_row.receiver_site() {
                return Err(ScriptDirectStaticResultBundleErrorV1::ReceiverSiteMismatch(
                    site.clone(),
                ));
            }
            if !matches!(
                method.receiver(),
                ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
            ) {
                return Err(
                    ScriptDirectStaticResultBundleErrorV1::ReceiverDispositionMismatch(
                        site.clone(),
                    ),
                );
            }
            if method.arguments().len() != lookup_row.argument_sites().len() {
                return Err(
                    ScriptDirectStaticResultBundleErrorV1::ArgumentSiteMismatch {
                        site: site.clone(),
                        ordinal: method.arguments().len() as u32,
                    },
                );
            }
            for (argument, expected) in method
                .arguments()
                .iter()
                .zip(lookup_row.argument_sites().iter())
            {
                if argument.site() != expected
                    || argument.ordinal() as usize >= method.arguments().len()
                {
                    return Err(
                        ScriptDirectStaticResultBundleErrorV1::ArgumentSiteMismatch {
                            site: site.clone(),
                            ordinal: argument.ordinal(),
                        },
                    );
                }
            }
            if method.result_site() != site {
                return Err(ScriptDirectStaticResultBundleErrorV1::ResultSiteMismatch(
                    site.clone(),
                ));
            }
            let target = lookup_row.target();
            if target.namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod {
                return Err(
                    ScriptDirectStaticResultBundleErrorV1::TargetNamespaceMismatch(site.clone()),
                );
            }
            if target.arity() != method.arity() {
                return Err(ScriptDirectStaticResultBundleErrorV1::TargetArityMismatch(
                    site.clone(),
                ));
            }
            if rows
                .insert(
                    site.clone(),
                    VerifiedScriptDirectStaticResultDemandV1 {
                        source_owner,
                        site: site.clone(),
                        receiver_site: method.receiver_site().clone(),
                        argument_sites: method
                            .arguments()
                            .iter()
                            .map(|argument| argument.site().clone())
                            .collect::<Vec<_>>()
                            .into_boxed_slice(),
                        result_site: method.result_site().clone(),
                        target: target.clone(),
                        representation: lookup_row.representation().clone(),
                        required_callee_i64_arguments: lookup_row
                            .required_callee_i64_arguments()
                            .to_vec()
                            .into_boxed_slice(),
                    },
                )
                .is_some()
            {
                return Err(ScriptDirectStaticResultBundleErrorV1::DuplicateSite(
                    site.clone(),
                ));
            }
        }
        Ok(Self {
            source_owner,
            source_identity: source.source() as *const _ as usize,
            rows,
        })
    }

    pub(super) const fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.source_owner
    }

    pub(super) fn demand(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedScriptDirectStaticResultDemandV1> {
        self.rows.get(site)
    }

    pub(super) fn rows(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &VerifiedScriptDirectStaticResultDemandV1)> {
        self.rows.iter()
    }

    pub(super) fn len(&self) -> usize {
        self.rows.len()
    }

    pub(super) const fn source_identity(&self) -> usize {
        self.source_identity
    }
}

#[cfg(test)]
#[path = "normal_script_direct_static_result_bundle_tests.rs"]
mod tests;
