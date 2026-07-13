//! Exact BindingRef environment plus adoption and source-coverage ledgers.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, ResolvedAssignmentTargetV1, ResolvedExitSiteV1,
    ResolvedLexicalRefV1, SourceBindingSiteV1, SourceExprSiteV1, VerifiedResolvedFunctionV1,
};
use crate::mir::ValueId;

use super::branch_transaction::{AuthorizedBranchRebindV1, BranchValueStoreV1};
use super::if_materialization::{DefinedJoinPublishV1, DefinedJoinValueStoreV1};

#[derive(Debug)]
struct ResolvedValueEnvironmentV1 {
    values: BTreeMap<BindingRefV1, ValueId>,
}

impl ResolvedValueEnvironmentV1 {
    fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    fn publish(&mut self, binding: BindingRefV1, value: ValueId) -> Result<(), String> {
        if self.values.insert(binding, value).is_some() {
            return Err(format!(
                "[freeze:contract][canonical_identity/value_republished] binding={binding:?}"
            ));
        }
        Ok(())
    }

    fn value(&self, binding: BindingRefV1) -> Result<ValueId, String> {
        self.values.get(&binding).copied().ok_or_else(|| {
            format!(
                "[freeze:contract][canonical_identity/value_unmaterialized] binding={binding:?}"
            )
        })
    }

    fn rebind(&mut self, binding: BindingRefV1, value: ValueId) -> Result<ValueId, String> {
        let previous = self.values.get_mut(&binding).ok_or_else(|| {
            format!(
                "[freeze:contract][canonical_identity/rebind_unmaterialized] binding={binding:?}"
            )
        })?;
        let old = *previous;
        *previous = value;
        Ok(old)
    }

    fn remove(&mut self, binding: BindingRefV1) -> Option<ValueId> {
        self.values.remove(&binding)
    }

    fn bindings(&self) -> BTreeSet<BindingRefV1> {
        self.values.keys().copied().collect()
    }
}

#[derive(Debug)]
struct ResolvedIdentityAdoptionLedgerV1 {
    adopted: BTreeSet<BindingRefV1>,
}

impl ResolvedIdentityAdoptionLedgerV1 {
    fn new() -> Self {
        Self {
            adopted: BTreeSet::new(),
        }
    }

    fn adopt(&mut self, binding: BindingRefV1) -> Result<(), String> {
        if !self.adopted.insert(binding) {
            return Err(format!(
                "[freeze:contract][canonical_identity/duplicate_adoption] binding={binding:?}"
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
struct LoweringSourceCoverageV1 {
    declarations: BTreeSet<SourceBindingSiteV1>,
    variable_uses: BTreeSet<SourceExprSiteV1>,
    assignment_targets: BTreeSet<SourceExprSiteV1>,
    exits: BTreeSet<ResolvedExitSiteV1>,
}

impl LoweringSourceCoverageV1 {
    fn new() -> Self {
        Self {
            declarations: BTreeSet::new(),
            variable_uses: BTreeSet::new(),
            assignment_targets: BTreeSet::new(),
            exits: BTreeSet::new(),
        }
    }

    fn mark<T: Ord + Clone>(set: &mut BTreeSet<T>, site: &T, kind: &str) -> Result<(), String> {
        if !set.insert(site.clone()) {
            return Err(format!(
                "[freeze:contract][canonical_coverage/duplicate] kind={kind}"
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct ResolvedIdentityStateV1<'a> {
    product: &'a VerifiedResolvedFunctionV1,
    values: ResolvedValueEnvironmentV1,
    adoption: ResolvedIdentityAdoptionLedgerV1,
    coverage: LoweringSourceCoverageV1,
    retired: BTreeSet<BindingRefV1>,
}

impl<'a> ResolvedIdentityStateV1<'a> {
    pub(super) fn new(product: &'a VerifiedResolvedFunctionV1) -> Self {
        Self {
            product,
            values: ResolvedValueEnvironmentV1::new(),
            adoption: ResolvedIdentityAdoptionLedgerV1::new(),
            coverage: LoweringSourceCoverageV1::new(),
            retired: BTreeSet::new(),
        }
    }

    pub(super) fn publish_declaration(
        &mut self,
        site: &SourceBindingSiteV1,
        expected_kind: BindingKindV1,
        expected_name: &str,
        value: ValueId,
    ) -> Result<BindingRefV1, String> {
        let binding = self.product.declaration_binding(site).ok_or_else(|| {
            format!("[freeze:contract][canonical_identity/declaration_missing] site={site:?}")
        })?;
        self.verify_record(binding, expected_kind, expected_name)?;
        self.adoption.adopt(binding)?;
        self.values.publish(binding, value)?;
        LoweringSourceCoverageV1::mark(&mut self.coverage.declarations, site, "declaration")?;
        Ok(binding)
    }

    pub(super) fn variable_value(
        &mut self,
        site: &SourceExprSiteV1,
        expected_name: &str,
    ) -> Result<ValueId, String> {
        let reference = self.product.variable_ref(site).ok_or_else(|| {
            format!("[freeze:contract][canonical_identity/use_missing] site={site:?}")
        })?;
        let ResolvedLexicalRefV1::Local(binding) = reference else {
            return Err(format!(
                "[freeze:contract][canonical_identity/upvar_not_activated] site={site:?}"
            ));
        };
        self.verify_name(binding, expected_name)?;
        LoweringSourceCoverageV1::mark(&mut self.coverage.variable_uses, site, "variable_use")?;
        self.values.value(binding)
    }

    pub(super) fn resolve_assignment_binding(
        &self,
        site: &SourceExprSiteV1,
        expected_name: &str,
    ) -> Result<BindingRefV1, String> {
        let target = self.product.assignment_target(site).ok_or_else(|| {
            format!("[freeze:contract][canonical_identity/assignment_missing] site={site:?}")
        })?;
        let ResolvedAssignmentTargetV1::BindingRebind(binding) = target else {
            return Err(format!(
                "[freeze:contract][canonical_identity/non_binding_assignment] site={site:?}"
            ));
        };
        self.verify_name(*binding, expected_name)?;
        Ok(*binding)
    }

    pub(super) fn claim_assignment_binding(
        &mut self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Result<(), String> {
        let expected = self.product.assignment_target(site).ok_or_else(|| {
            format!("[freeze:contract][canonical_identity/assignment_missing] site={site:?}")
        })?;
        if expected != &ResolvedAssignmentTargetV1::BindingRebind(binding) {
            return Err(format!(
                "[freeze:contract][canonical_identity/assignment_claim_mismatch] site={site:?} binding={binding:?}"
            ));
        }
        LoweringSourceCoverageV1::mark(
            &mut self.coverage.assignment_targets,
            site,
            "assignment_target",
        )
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
        if self.product.resolved_exit(&site).is_none() {
            return Err(format!(
                "[freeze:contract][canonical_coverage/return_missing] site={site:?}"
            ));
        }
        LoweringSourceCoverageV1::mark(&mut self.coverage.exits, &site, "return")
    }

    pub(super) fn retire_scope_success(
        &mut self,
        declarations: &[BindingRefV1],
    ) -> Result<(), String> {
        for binding in declarations {
            if !self.adoption.adopted.contains(binding)
                || !self.values.values.contains_key(binding)
                || self.retired.contains(binding)
            {
                return Err(format!(
                    "[freeze:contract][canonical_scope/declaration_not_active] binding={binding:?}"
                ));
            }
        }
        self.retire_materialized(declarations);
        Ok(())
    }

    pub(super) fn retire_scope_error(&mut self, declarations: &[BindingRefV1]) {
        self.retire_materialized(declarations);
    }

    fn retire_materialized(&mut self, declarations: &[BindingRefV1]) -> Vec<ValueId> {
        let mut values = Vec::new();
        for binding in declarations {
            if let Some(value) = self.values.remove(*binding) {
                self.retired.insert(*binding);
                values.push(value);
            }
        }
        values
    }

    pub(super) fn finish(self) -> Result<(), String> {
        let expected_declarations = self
            .product
            .declaration_sites()
            .cloned()
            .collect::<BTreeSet<_>>();
        let expected_bindings = expected_declarations
            .iter()
            .filter_map(|site| self.product.declaration_binding(site))
            .collect::<BTreeSet<_>>();
        let expected_uses = self
            .product
            .variable_refs()
            .map(|(site, _)| site.clone())
            .collect::<BTreeSet<_>>();
        let expected_targets = self
            .product
            .assignment_targets()
            .map(|(site, _)| site.clone())
            .collect::<BTreeSet<_>>();
        let expected_exits = self
            .product
            .resolved_exits()
            .map(|(site, _)| site.clone())
            .collect::<BTreeSet<_>>();

        let active_bindings = self.values.bindings();
        let mut disposed_bindings = active_bindings.clone();
        disposed_bindings.extend(self.retired.iter().copied());
        if self.adoption.adopted != expected_bindings
            || self.coverage.declarations != expected_declarations
            || self.coverage.variable_uses != expected_uses
            || self.coverage.assignment_targets != expected_targets
            || self.coverage.exits != expected_exits
            || disposed_bindings != expected_bindings
            || !active_bindings.is_disjoint(&self.retired)
        {
            return Err(format!(
                "[freeze:contract][canonical_coverage/finish_mismatch] declarations={}/{} bindings={}/{} uses={}/{} assignments={}/{} exits={}/{} active_values={} retired={} disposed={}/{}",
                self.coverage.declarations.len(),
                expected_declarations.len(),
                self.adoption.adopted.len(),
                expected_bindings.len(),
                self.coverage.variable_uses.len(),
                expected_uses.len(),
                self.coverage.assignment_targets.len(),
                expected_targets.len(),
                self.coverage.exits.len(),
                expected_exits.len(),
                active_bindings.len(),
                self.retired.len(),
                disposed_bindings.len(),
                expected_bindings.len(),
            ));
        }
        Ok(())
    }

    fn verify_record(
        &self,
        binding: BindingRefV1,
        expected_kind: BindingKindV1,
        expected_name: &str,
    ) -> Result<(), String> {
        let record = self.product.binding(binding).ok_or_else(|| {
            format!("[freeze:contract][canonical_identity/foreign_binding] binding={binding:?}")
        })?;
        if record.kind() != expected_kind || record.diagnostic_name() != expected_name {
            return Err(format!(
                "[freeze:contract][canonical_identity/declaration_mismatch] binding={binding:?} expected_kind={expected_kind:?} actual_kind={:?} expected_name={expected_name} actual_name={}",
                record.kind(),
                record.diagnostic_name(),
            ));
        }
        Ok(())
    }

    fn verify_name(&self, binding: BindingRefV1, expected_name: &str) -> Result<(), String> {
        let record = self.product.binding(binding).ok_or_else(|| {
            format!("[freeze:contract][canonical_identity/foreign_binding] binding={binding:?}")
        })?;
        if record.diagnostic_name() != expected_name {
            return Err(format!(
                "[freeze:contract][canonical_identity/diagnostic_name_mismatch] binding={binding:?} expected={expected_name} actual={}",
                record.diagnostic_name()
            ));
        }
        Ok(())
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
