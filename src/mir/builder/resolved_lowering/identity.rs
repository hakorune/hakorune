//! Behavior-neutral facade between exact identity claims and pre-SSA values.
//!
//! SSA-S2 physically separates the two owners while preserving the existing
//! canonical Lower API. Binding SSA remains disconnected until SSA-I1.

mod ledger;
mod value_environment;

use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1,
    VerifiedResolvedFunctionV1,
};
use crate::mir::ValueId;

use super::branch_transaction::{AuthorizedBranchRebindV1, BranchValueStoreV1};
use super::if_materialization::{DefinedJoinPublishV1, DefinedJoinValueStoreV1};
use ledger::ResolvedIdentityLedgerV2;
use value_environment::PreSsaValueEnvironmentV1;

/// Compatibility facade for the pre-SSA production path.
///
/// `ledger` is the only source/claim/lifetime authority. `values` is the only
/// current reaching-value authority. This facade adds no second map and never
/// synchronizes with the disconnected function-owned SSA box.
#[derive(Debug)]
pub(super) struct ResolvedIdentityStateV1<'a> {
    ledger: ResolvedIdentityLedgerV2<'a>,
    values: PreSsaValueEnvironmentV1,
}

impl<'a> ResolvedIdentityStateV1<'a> {
    pub(super) fn new(product: &'a VerifiedResolvedFunctionV1) -> Self {
        Self {
            ledger: ResolvedIdentityLedgerV2::new(product),
            values: PreSsaValueEnvironmentV1::new(),
        }
    }

    pub(super) fn publish_declaration(
        &mut self,
        site: &SourceBindingSiteV1,
        expected_kind: BindingKindV1,
        expected_name: &str,
        value: ValueId,
    ) -> Result<BindingRefV1, String> {
        let binding = self
            .ledger
            .adopt_declaration(site, expected_kind, expected_name)?;
        self.values.publish(binding, value)?;
        self.ledger.mark_declaration(site)?;
        Ok(binding)
    }

    pub(super) fn variable_value(
        &mut self,
        site: &SourceExprSiteV1,
        expected_name: &str,
    ) -> Result<ValueId, String> {
        let binding = self.ledger.claim_variable_use(site, expected_name)?;
        self.values.value(binding)
    }

    pub(super) fn resolve_assignment_binding(
        &self,
        site: &SourceExprSiteV1,
        expected_name: &str,
    ) -> Result<BindingRefV1, String> {
        self.ledger.resolve_assignment_binding(site, expected_name)
    }

    pub(super) fn claim_assignment_binding(
        &mut self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Result<(), String> {
        self.ledger.claim_assignment_binding(site, binding)
    }

    pub(super) fn current_value(&self, binding: BindingRefV1) -> Result<ValueId, String> {
        self.values.value(binding)
    }

    pub(super) fn rebind(
        &mut self,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Result<ValueId, String> {
        self.values.rebind(binding, value)
    }

    pub(super) fn mark_return(&mut self, site: ResolvedExitSiteV1) -> Result<(), String> {
        self.ledger.mark_return(site)
    }

    pub(super) fn retire_scope_success(
        &mut self,
        declarations: &[BindingRefV1],
    ) -> Result<(), String> {
        for binding in declarations {
            self.ledger.verify_scope_active(*binding)?;
            if !self.values.contains(*binding) {
                return Err(format!(
                    "[freeze:contract][canonical_scope/declaration_not_active] binding={binding:?}"
                ));
            }
        }
        let unique = declarations
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        let unique = unique.into_iter().collect::<Vec<_>>();
        for binding in &unique {
            let _ = self.values.remove(*binding);
        }
        self.ledger.retire_scope_success(&unique);
        Ok(())
    }

    pub(super) fn retire_scope_error(&mut self, declarations: &[BindingRefV1]) {
        for binding in declarations {
            if self.values.remove(*binding).is_some() {
                self.ledger.retire_materialized(*binding);
            }
        }
    }

    pub(super) fn finish(self) -> Result<(), String> {
        self.ledger.finish(self.values.bindings())
    }
}

impl BranchValueStoreV1 for ResolvedIdentityStateV1<'_> {
    fn branch_current_value(&self, binding: BindingRefV1) -> Result<ValueId, String> {
        self.current_value(binding)
    }

    fn branch_rebind_authorized(
        &mut self,
        authorization: AuthorizedBranchRebindV1,
    ) -> Result<ValueId, String> {
        self.values
            .rebind(authorization.binding(), authorization.value())
    }
}

impl DefinedJoinValueStoreV1 for ResolvedIdentityStateV1<'_> {
    fn defined_join_current_value(&self, binding: BindingRefV1) -> Result<ValueId, String> {
        self.current_value(binding)
    }

    fn publish_defined_join_batch(
        &mut self,
        publishes: Vec<DefinedJoinPublishV1>,
    ) -> Result<(), String> {
        for publish in &publishes {
            let actual = self.current_value(publish.binding())?;
            if actual != publish.expected_entry() {
                return Err(format!(
                    "[freeze:contract][canonical_if/join_entry_changed_during_publish] binding={:?}",
                    publish.binding()
                ));
            }
        }
        for publish in publishes {
            self.values.rebind(publish.binding(), publish.value())?;
        }
        Ok(())
    }
}
