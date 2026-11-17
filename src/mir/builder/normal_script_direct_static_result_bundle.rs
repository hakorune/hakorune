//! Source-bound Script direct-static result facts.
//!
//! This bundle is the only bridge between the resolver's Script MethodCall
//! rows and the later Script Facts/Recipe consumer.  It owns no Builder value,
//! MIR type, Recipe key, or physical block.  The source-package lookup
//! relation supplies the callee and exact source sites; resolver rows verify
//! the same source tree before the bundle is issued.

use std::collections::BTreeMap;

use crate::mir::builder::{CanonicalSameModuleCallableKeyV1};
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, SourceExprSiteV1,
};
use crate::mir::source_call_target::{
    VerifiedScriptDirectStaticCallLookupRowV1,
};



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


    /// Project rows already co-sealed by the private Script A issuer.
    ///
    /// This constructor deliberately performs no source observation.  The
    /// named C consumer is the only production caller and receives rows whose
    /// site, receiver, arguments, result, target, and representation were
    /// checked in the single A pass.
    pub(in crate::mir::builder) fn from_canonical_a_rows(
        source_owner: FunctionOwnerIdV1,
        source_identity: usize,
        rows: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallLookupRowV1>,
    ) -> Self {
        let rows = rows
            .into_iter()
            .map(|(site, lookup_row)| {
                let (row_site, receiver_site, argument_sites, target, representation, required) =
                    lookup_row.into_parts();
                (
                    site.clone(),
                    VerifiedScriptDirectStaticResultDemandV1 {
                        source_owner,
                        site,
                        receiver_site,
                        argument_sites,
                        result_site: row_site,
                        target,
                        representation,
                        required_callee_i64_arguments: required,
                    },
                )
            })
            .collect();
        Self {
            source_owner,
            source_identity,
            rows,
        }
    }

    pub(super) const fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.source_owner
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
