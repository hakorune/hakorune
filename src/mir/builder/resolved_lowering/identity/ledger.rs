use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, ResolvedAssignmentTargetV1, ResolvedExitSiteV1,
    ResolvedLexicalRefV1, SourceBindingSiteV1, SourceExprSiteV1, VerifiedResolvedFunctionV1,
};

#[derive(Debug)]
struct ResolvedIdentityAdoptionLedgerV2 {
    adopted: BTreeSet<BindingRefV1>,
}

impl ResolvedIdentityAdoptionLedgerV2 {
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
struct LoweringSourceCoverageV2 {
    declarations: BTreeSet<SourceBindingSiteV1>,
    variable_uses: BTreeSet<SourceExprSiteV1>,
    assignment_targets: BTreeSet<SourceExprSiteV1>,
    exits: BTreeSet<ResolvedExitSiteV1>,
}

impl LoweringSourceCoverageV2 {
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

/// Exact source claims and lexical lifetime for one resolved function.
///
/// This owner deliberately has no MIR value or block dependency. Reaching
/// values remain in the temporary owner until the atomic cutover.
#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct ResolvedIdentityLedgerV2<'a> {
    product: &'a VerifiedResolvedFunctionV1,
    adoption: ResolvedIdentityAdoptionLedgerV2,
    coverage: LoweringSourceCoverageV2,
    retired: BTreeSet<BindingRefV1>,
}

impl<'a> ResolvedIdentityLedgerV2<'a> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        product: &'a VerifiedResolvedFunctionV1,
    ) -> Self {
        Self {
            product,
            adoption: ResolvedIdentityAdoptionLedgerV2::new(),
            coverage: LoweringSourceCoverageV2::new(),
            retired: BTreeSet::new(),
        }
    }

    pub(in crate::mir::builder::resolved_lowering) fn adopt_declaration(
        &mut self,
        site: &SourceBindingSiteV1,
        expected_kind: BindingKindV1,
        expected_name: &str,
    ) -> Result<BindingRefV1, String> {
        let binding = self.product.declaration_binding(site).ok_or_else(|| {
            format!("[freeze:contract][canonical_identity/declaration_missing] site={site:?}")
        })?;
        self.verify_record(binding, expected_kind, expected_name)?;
        self.adoption.adopt(binding)?;
        Ok(binding)
    }

    /// Adopt one declaration through an already sealed exact source relation.
    ///
    /// Unlike the legacy name/kind entry, this boundary never asks a physical
    /// adapter to repeat resolver-owned record fields. The declaration site is
    /// still required, so a raw `(BindingRef, ValueId)` pair cannot enter the
    /// canonical identity owner.
    pub(in crate::mir::builder::resolved_lowering) fn adopt_declaration_exact(
        &mut self,
        site: &SourceBindingSiteV1,
        expected_binding: BindingRefV1,
    ) -> Result<BindingRefV1, String> {
        let binding = self.product.declaration_binding(site).ok_or_else(|| {
            format!("[freeze:contract][canonical_identity/declaration_missing] site={site:?}")
        })?;
        if binding != expected_binding {
            return Err(format!(
                "[freeze:contract][canonical_identity/declaration_binding_mismatch] site={site:?} expected={expected_binding:?} actual={binding:?}"
            ));
        }
        if self.product.binding(binding).is_none() {
            return Err(format!(
                "[freeze:contract][canonical_identity/foreign_binding] binding={binding:?}"
            ));
        }
        self.adoption.adopt(binding)?;
        Ok(binding)
    }

    pub(in crate::mir::builder::resolved_lowering) fn mark_declaration(
        &mut self,
        site: &SourceBindingSiteV1,
    ) -> Result<(), String> {
        LoweringSourceCoverageV2::mark(&mut self.coverage.declarations, site, "declaration")?;
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn claim_variable_use(
        &mut self,
        site: &SourceExprSiteV1,
        expected_name: &str,
    ) -> Result<BindingRefV1, String> {
        let binding = self.resolve_variable_binding(site, expected_name)?;
        self.claim_variable_use_binding(site, binding)?;
        Ok(binding)
    }

    pub(in crate::mir::builder::resolved_lowering) fn resolve_variable_binding(
        &self,
        site: &SourceExprSiteV1,
        expected_name: &str,
    ) -> Result<BindingRefV1, String> {
        let reference = self.product.variable_ref(site).ok_or_else(|| {
            format!("[freeze:contract][canonical_identity/use_missing] site={site:?}")
        })?;
        let ResolvedLexicalRefV1::Local(binding) = reference else {
            return Err(format!(
                "[freeze:contract][canonical_identity/upvar_not_activated] site={site:?}"
            ));
        };
        self.verify_name(binding, expected_name)?;
        Ok(binding)
    }

    /// Exact source claim used by execution-scoped adapters. The caller has
    /// already obtained the BindingRef from a sealed witness; this method
    /// still rechecks the product so no foreign site/binding pair can bypass
    /// coverage.
    pub(in crate::mir::builder::resolved_lowering) fn claim_variable_use_binding(
        &mut self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Result<(), String> {
        let reference = self.product.variable_ref(site).ok_or_else(|| {
            format!("[freeze:contract][canonical_identity/use_missing] site={site:?}")
        })?;
        if reference != ResolvedLexicalRefV1::Local(binding) {
            return Err(format!(
                "[freeze:contract][canonical_identity/use_claim_mismatch] site={site:?} binding={binding:?}"
            ));
        }
        LoweringSourceCoverageV2::mark(&mut self.coverage.variable_uses, site, "variable_use")?;
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn resolve_assignment_binding(
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

    pub(in crate::mir::builder::resolved_lowering) fn claim_assignment_binding(
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
        LoweringSourceCoverageV2::mark(
            &mut self.coverage.assignment_targets,
            site,
            "assignment_target",
        )
    }

    pub(in crate::mir::builder::resolved_lowering) fn mark_return(
        &mut self,
        site: ResolvedExitSiteV1,
    ) -> Result<(), String> {
        if self.product.resolved_exit(&site).is_none() {
            return Err(format!(
                "[freeze:contract][canonical_coverage/return_missing] site={site:?}"
            ));
        }
        LoweringSourceCoverageV2::mark(&mut self.coverage.exits, &site, "return")
    }

    pub(in crate::mir::builder::resolved_lowering) fn verify_scope_active(
        &self,
        binding: BindingRefV1,
    ) -> Result<(), String> {
        if !self.adoption.adopted.contains(&binding) || self.retired.contains(&binding) {
            return Err(format!(
                "[freeze:contract][canonical_scope/declaration_not_active] binding={binding:?}"
            ));
        }
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn retire_scope_success(
        &mut self,
        declarations: &[BindingRefV1],
    ) {
        self.retired.extend(declarations.iter().copied());
    }

    pub(in crate::mir::builder::resolved_lowering) fn retire_materialized(
        &mut self,
        binding: BindingRefV1,
    ) {
        self.retired.insert(binding);
    }

    pub(in crate::mir::builder::resolved_lowering) fn finish(
        self,
        active_bindings: BTreeSet<BindingRefV1>,
    ) -> Result<(), String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::direct_accum_projection::direct_accum_function_for_test;
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

    #[test]
    fn exact_variable_claim_rechecks_binding_and_duplicate() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_accum_function_for_test())
            .expect("fixture resolves");
        let input = unit.root_function_input().expect("input");
        let function = input.function();
        let (site, reference) = function
            .variable_refs()
            .find(|(_, reference)| matches!(reference, ResolvedLexicalRefV1::Local(_)))
            .expect("local variable use");
        let ResolvedLexicalRefV1::Local(binding) = reference else {
            unreachable!("filtered local reference")
        };
        let mut ledger = ResolvedIdentityLedgerV2::new(function);
        ledger
            .claim_variable_use_binding(site, *binding)
            .expect("exact claim");
        assert!(ledger.claim_variable_use_binding(site, *binding).is_err());

        let foreign = function
            .variable_refs()
            .filter_map(|(_, reference)| match reference {
                ResolvedLexicalRefV1::Local(candidate) if candidate != binding => Some(*candidate),
                _ => None,
            })
            .next()
            .expect("second local binding");
        assert!(ledger.claim_variable_use_binding(site, foreign).is_err());
    }

    #[test]
    fn exact_assignment_claim_rechecks_binding() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_accum_function_for_test())
            .expect("fixture resolves");
        let input = unit.root_function_input().expect("input");
        let function = input.function();
        let (site, target) = function
            .assignment_targets()
            .next()
            .expect("assignment target");
        let ResolvedAssignmentTargetV1::BindingRebind(binding) = target else {
            unreachable!("DirectAccum assignment is a local rebind")
        };
        let mut ledger = ResolvedIdentityLedgerV2::new(function);
        ledger
            .claim_assignment_binding(site, *binding)
            .expect("exact assignment claim");
        assert!(ledger.claim_assignment_binding(site, *binding).is_err());
    }
}
