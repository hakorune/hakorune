//! ScriptRoot source/Facts result-publication owner.
//!
//! This owner co-seals two already-issued products: the direct-static result
//! bundle owns target/representation, while the Script continuation owns the
//! exact destination.  It deliberately stops before Recipe keys, JoinSig,
//! ValueId, MIR type, or physical publication.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::{
    BodyShapeRelationV1, FunctionOwnerIdV1, SourceExprSiteV1, VerifiedSemanticOwnerProductV1,
};

use super::normal_script_direct_static_result_bundle::VerifiedScriptDirectStaticResultBundleV1;
use super::normal_script_semantic_source::VerifiedScriptSemanticSourceV1;
use super::normal_script_source_continuation::{
    ScriptSourceContinuationTerminalV1, VerifiedScriptSourceContinuationV1,
};
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ScriptDirectStaticResultPublicationOwnerIssueV1 {
    ScriptRootCardinality,
    ScriptRootProduct,
    SourceOwnerMismatch,
    BundleOwnerMismatch,
    BundleSourceMismatch,
    ContinuationOwnerMismatch,
    ContinuationMissing(SourceExprSiteV1),
    ContinuationRowOwnerMismatch(SourceExprSiteV1),
    ContinuationCallSiteMismatch(SourceExprSiteV1),
    DuplicateSite(SourceExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VerifiedScriptDirectStaticResultPublicationDemandV1 {
    source_owner: FunctionOwnerIdV1,
    call_site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    argument_sites: Box<[SourceExprSiteV1]>,
    result_site: SourceExprSiteV1,
    parent_relations: Box<[BodyShapeRelationV1]>,
    terminal: ScriptSourceContinuationTerminalV1,
    target: CanonicalSameModuleCallableKeyV1,
    representation: VerifiedCallableResultRepresentationV1,
    required_callee_i64_arguments: Box<[u32]>,
}

impl VerifiedScriptDirectStaticResultPublicationDemandV1 {
    #[cfg(test)]
    pub(super) fn from_parts_for_test(
        source_owner: FunctionOwnerIdV1,
        call_site: SourceExprSiteV1,
        receiver_site: SourceExprSiteV1,
        argument_sites: Box<[SourceExprSiteV1]>,
        result_site: SourceExprSiteV1,
        parent_relations: Box<[BodyShapeRelationV1]>,
        terminal: ScriptSourceContinuationTerminalV1,
        target: CanonicalSameModuleCallableKeyV1,
        representation: VerifiedCallableResultRepresentationV1,
        required_callee_i64_arguments: Box<[u32]>,
    ) -> Self {
        Self {
            source_owner,
            call_site,
            receiver_site,
            argument_sites,
            result_site,
            parent_relations,
            terminal,
            target,
            representation,
            required_callee_i64_arguments,
        }
    }

    pub(super) const fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.source_owner
    }

    pub(super) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
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

    pub(super) fn parent_relations(&self) -> &[BodyShapeRelationV1] {
        &self.parent_relations
    }

    pub(super) const fn terminal(&self) -> &ScriptSourceContinuationTerminalV1 {
        &self.terminal
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
pub(super) struct VerifiedScriptDirectStaticResultPublicationOwnerV1 {
    source_owner: FunctionOwnerIdV1,
    source_identity: usize,
    rows: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticResultPublicationDemandV1>,
}

impl VerifiedScriptDirectStaticResultPublicationOwnerV1 {
    #[cfg(test)]
    pub(super) fn from_parts_for_test(
        source_owner: FunctionOwnerIdV1,
        source_identity: usize,
        rows: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticResultPublicationDemandV1>,
    ) -> Self {
        Self {
            source_owner,
            source_identity,
            rows,
        }
    }

    pub(super) fn issue(
        source: &VerifiedScriptSemanticSourceV1<'_>,
        bundle: &VerifiedScriptDirectStaticResultBundleV1,
        continuation: &VerifiedScriptSourceContinuationV1,
    ) -> Result<Self, ScriptDirectStaticResultPublicationOwnerIssueV1> {
        let [root] = source.forest().roots() else {
            return Err(ScriptDirectStaticResultPublicationOwnerIssueV1::ScriptRootCardinality);
        };
        let Some(product) = source
            .forest()
            .semantic_owner(*root)
            .and_then(VerifiedSemanticOwnerProductV1::as_script)
        else {
            return Err(ScriptDirectStaticResultPublicationOwnerIssueV1::ScriptRootProduct);
        };
        if bundle.source_identity() != source.source() as *const _ as usize {
            return Err(ScriptDirectStaticResultPublicationOwnerIssueV1::BundleSourceMismatch);
        }
        let source_owner = product.core().data().owner;
        if source_owner != bundle.source_owner() {
            return Err(ScriptDirectStaticResultPublicationOwnerIssueV1::BundleOwnerMismatch);
        }
        if source_owner != continuation.owner() {
            return Err(ScriptDirectStaticResultPublicationOwnerIssueV1::ContinuationOwnerMismatch);
        }

        let mut rows = BTreeMap::new();
        for (site, demand) in bundle.rows() {
            let Some(destination) = continuation.row(site) else {
                return Err(
                    ScriptDirectStaticResultPublicationOwnerIssueV1::ContinuationMissing(
                        site.clone(),
                    ),
                );
            };
            if destination.owner() != source_owner {
                return Err(
                    ScriptDirectStaticResultPublicationOwnerIssueV1::ContinuationRowOwnerMismatch(
                        site.clone(),
                    ),
                );
            }
            if destination.call_site() != site {
                return Err(
                    ScriptDirectStaticResultPublicationOwnerIssueV1::ContinuationCallSiteMismatch(
                        site.clone(),
                    ),
                );
            }
            let row = VerifiedScriptDirectStaticResultPublicationDemandV1 {
                source_owner,
                call_site: site.clone(),
                receiver_site: demand.receiver_site().clone(),
                argument_sites: demand.argument_sites().to_vec().into_boxed_slice(),
                result_site: demand.result_site().clone(),
                parent_relations: destination.parent_relations().to_vec().into_boxed_slice(),
                terminal: destination.terminal().clone(),
                target: demand.target().clone(),
                representation: demand.representation().clone(),
                required_callee_i64_arguments: demand
                    .required_callee_i64_arguments()
                    .to_vec()
                    .into_boxed_slice(),
            };
            if rows.insert(site.clone(), row).is_some() {
                return Err(
                    ScriptDirectStaticResultPublicationOwnerIssueV1::DuplicateSite(site.clone()),
                );
            }
        }

        Ok(Self {
            source_owner,
            source_identity: source.source() as *const _ as usize,
            rows,
        })
    }

    /// Project the already-issued C rows without reopening the AST or
    /// resolver forest. Continuation relations remain the only new input:
    /// this owner attaches the terminal destination to each A-owned demand.
    pub(in crate::mir::builder) fn from_canonical_bundle(
        source_owner: FunctionOwnerIdV1,
        source_identity: usize,
        bundle: &VerifiedScriptDirectStaticResultBundleV1,
        continuation: &VerifiedScriptSourceContinuationV1,
    ) -> Result<Self, ScriptDirectStaticResultPublicationOwnerIssueV1> {
        if bundle.source_owner() != source_owner {
            return Err(ScriptDirectStaticResultPublicationOwnerIssueV1::BundleOwnerMismatch);
        }
        if bundle.source_identity() != source_identity {
            return Err(ScriptDirectStaticResultPublicationOwnerIssueV1::BundleSourceMismatch);
        }
        if continuation.owner() != source_owner {
            return Err(
                ScriptDirectStaticResultPublicationOwnerIssueV1::ContinuationOwnerMismatch,
            );
        }

        let mut rows = BTreeMap::new();
        for (site, demand) in bundle.rows() {
            let Some(destination) = continuation.row(site) else {
                return Err(
                    ScriptDirectStaticResultPublicationOwnerIssueV1::ContinuationMissing(
                        site.clone(),
                    ),
                );
            };
            if destination.owner() != source_owner {
                return Err(
                    ScriptDirectStaticResultPublicationOwnerIssueV1::ContinuationRowOwnerMismatch(
                        site.clone(),
                    ),
                );
            }
            if destination.call_site() != site {
                return Err(
                    ScriptDirectStaticResultPublicationOwnerIssueV1::ContinuationCallSiteMismatch(
                        site.clone(),
                    ),
                );
            }
            let row = VerifiedScriptDirectStaticResultPublicationDemandV1 {
                source_owner,
                call_site: site.clone(),
                receiver_site: demand.receiver_site().clone(),
                argument_sites: demand.argument_sites().to_vec().into_boxed_slice(),
                result_site: demand.result_site().clone(),
                parent_relations: destination.parent_relations().to_vec().into_boxed_slice(),
                terminal: destination.terminal().clone(),
                target: demand.target().clone(),
                representation: demand.representation().clone(),
                required_callee_i64_arguments: demand
                    .required_callee_i64_arguments()
                    .to_vec()
                    .into_boxed_slice(),
            };
            if rows.insert(site.clone(), row).is_some() {
                return Err(
                    ScriptDirectStaticResultPublicationOwnerIssueV1::DuplicateSite(site.clone()),
                );
            }
        }
        Ok(Self {
            source_owner,
            source_identity,
            rows,
        })
    }

    pub(super) const fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.source_owner
    }

    pub(super) const fn source_identity(&self) -> usize {
        self.source_identity
    }

    pub(super) fn demand(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedScriptDirectStaticResultPublicationDemandV1> {
        self.rows.get(site)
    }

    pub(super) fn rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &SourceExprSiteV1,
            &VerifiedScriptDirectStaticResultPublicationDemandV1,
        ),
    > {
        self.rows.iter()
    }

    pub(super) fn len(&self) -> usize {
        self.rows.len()
    }
}

#[cfg(test)]
#[path = "normal_script_direct_static_result_publication_owner_tests.rs"]
mod tests;
