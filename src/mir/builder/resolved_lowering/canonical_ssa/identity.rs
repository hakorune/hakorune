use std::collections::BTreeSet;

use crate::mir::builder::emission::phi_lifecycle::{PhiToken, PhiTxn};
use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedPredecessorsV1;
use crate::mir::builder::ssa::binding::{BindingSsaBuilderV1, MirBindingSsaAdapterV1};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1,
    VerifiedResolvedFunctionV1,
};
use crate::mir::{BasicBlockId, ValueId};

use super::super::identity::ledger::ResolvedIdentityLedgerV2;
use super::super::semantic_stack::ResolvedScopeRetirementV1;

/// Exact source identity plus the sole reaching-value authority for canonical
/// function profiles.
pub(in crate::mir::builder::resolved_lowering) struct ResolvedSsaIdentityStateV2<'source> {
    ledger: ResolvedIdentityLedgerV2<'source>,
    ssa: BindingSsaBuilderV1<PhiToken>,
    active: BTreeSet<BindingRefV1>,
    initialized: BTreeSet<BindingRefV1>,
}

impl<'source> ResolvedSsaIdentityStateV2<'source> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        product: &'source VerifiedResolvedFunctionV1,
    ) -> Self {
        Self {
            ledger: ResolvedIdentityLedgerV2::new(product),
            ssa: BindingSsaBuilderV1::new(product.owner()),
            active: BTreeSet::new(),
            initialized: BTreeSet::new(),
        }
    }

    pub(in crate::mir::builder::resolved_lowering) fn publish_declaration(
        &mut self,
        site: &SourceBindingSiteV1,
        expected_kind: BindingKindV1,
        expected_name: &str,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<BindingRefV1, String> {
        let binding = self
            .ledger
            .adopt_declaration(site, expected_kind, expected_name)?;
        self.ssa
            .define(binding, block, value)
            .map_err(|error| error.to_string())?;
        if !self.active.insert(binding) {
            return Err(format!(
                "[freeze:contract][canonical_binding_ssa/duplicate_active] binding={binding:?}"
            ));
        }
        self.ledger.mark_declaration(site)?;
        self.initialized.insert(binding);
        Ok(binding)
    }

    /// Activate a declaration whose first reaching value is issued later by
    /// an exact assignment claim. This seam never calls the SSA builder and
    /// therefore cannot allocate a value or provisional PHI.
    pub(in crate::mir::builder::resolved_lowering) fn activate_declaration_without_value(
        &mut self,
        site: &SourceBindingSiteV1,
        expected_kind: BindingKindV1,
        expected_name: &str,
    ) -> Result<BindingRefV1, String> {
        let binding = self
            .ledger
            .adopt_declaration(site, expected_kind, expected_name)?;
        if !self.active.insert(binding) {
            return Err(format!(
                "[freeze:contract][canonical_binding_ssa/duplicate_active] binding={binding:?}"
            ));
        }
        self.ledger.mark_declaration(site)?;
        Ok(binding)
    }

    pub(in crate::mir::builder::resolved_lowering) fn variable_value(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        block: BasicBlockId,
        site: &SourceExprSiteV1,
        expected_name: &str,
    ) -> Result<(BindingRefV1, ValueId), String> {
        let binding = self.ledger.resolve_variable_binding(site, expected_name)?;
        self.require_active(binding)?;
        self.require_initialized(binding)?;
        self.ledger.claim_variable_use_binding(site, binding)?;
        let mut adapter = MirBindingSsaAdapterV1::new(builder, phis);
        let value = self
            .ssa
            .read(&mut adapter, binding, block)
            .map_err(|error| error.to_string())?;
        Ok((binding, value))
    }

    /// Consume an exact witness claim without consulting names or rebuilding
    /// source identity. The ledger and active-binding check remain one API
    /// boundary so an adapter cannot write a retired binding.
    pub(in crate::mir::builder::resolved_lowering) fn claim_variable_use_binding(
        &mut self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Result<(), String> {
        self.require_active(binding)?;
        self.require_initialized(binding)?;
        self.ledger.claim_variable_use_binding(site, binding)
    }

    /// Read an already-adopted entry binding without claiming another source
    /// site. This is the non-claim seed/read operation for a loop adapter.
    pub(in crate::mir::builder::resolved_lowering) fn read_entry(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        block: BasicBlockId,
        binding: BindingRefV1,
    ) -> Result<ValueId, String> {
        self.require_active(binding)?;
        self.require_initialized(binding)?;
        let mut adapter = MirBindingSsaAdapterV1::new(builder, phis);
        self.ssa
            .read(&mut adapter, binding, block)
            .map_err(|error| error.to_string())
    }

    pub(in crate::mir::builder::resolved_lowering) fn resolve_assignment_binding(
        &self,
        site: &SourceExprSiteV1,
        expected_name: &str,
    ) -> Result<BindingRefV1, String> {
        let binding = self
            .ledger
            .resolve_assignment_binding(site, expected_name)?;
        self.require_active(binding)?;
        Ok(binding)
    }

    pub(in crate::mir::builder::resolved_lowering) fn define_assignment(
        &mut self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.ledger.claim_assignment_binding(site, binding)?;
        self.ssa
            .define(binding, block, value)
            .map_err(|error| error.to_string())?;
        self.initialized.insert(binding);
        Ok(())
    }

    /// Exact assignment claim plus active-binding check for execution
    /// adapters. The legacy lowerer keeps its existing two-step API.
    pub(in crate::mir::builder::resolved_lowering) fn define_assignment_exact(
        &mut self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.require_active(binding)?;
        self.ledger.claim_assignment_binding(site, binding)?;
        self.ssa
            .define(binding, block, value)
            .map_err(|error| error.to_string())?;
        self.initialized.insert(binding);
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn seal_block(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        block: BasicBlockId,
        witness: &VerifiedPredecessorsV1,
    ) -> Result<(), String> {
        let mut adapter = MirBindingSsaAdapterV1::new(builder, phis);
        self.ssa
            .seal(&mut adapter, block, witness)
            .map_err(|error| error.to_string())
    }

    pub(in crate::mir::builder::resolved_lowering) fn mark_return(
        &mut self,
        site: ResolvedExitSiteV1,
    ) -> Result<(), String> {
        self.ledger.mark_return(site)
    }

    pub(in crate::mir::builder::resolved_lowering) fn finish(self) -> Result<(), String> {
        self.ssa.finish().map_err(|error| error.to_string())?;
        self.ledger.finish(self.active)
    }

    fn require_active(&self, binding: BindingRefV1) -> Result<(), String> {
        self.ledger.verify_scope_active(binding)?;
        if !self.active.contains(&binding) {
            return Err(format!(
                "[freeze:contract][canonical_binding_ssa/inactive_binding] binding={binding:?}"
            ));
        }
        Ok(())
    }

    fn require_initialized(&self, binding: BindingRefV1) -> Result<(), String> {
        if !self.initialized.contains(&binding) {
            return Err(format!(
                "[freeze:contract][canonical_binding_ssa/uninitialized_binding] binding={binding:?}"
            ));
        }
        Ok(())
    }
}

impl ResolvedScopeRetirementV1 for ResolvedSsaIdentityStateV2<'_> {
    fn retire_scope_success(&mut self, declarations: &[BindingRefV1]) -> Result<(), String> {
        let declarations = declarations.iter().copied().collect::<BTreeSet<_>>();
        for binding in declarations.iter().copied() {
            self.require_active(binding)?;
            self.active.remove(&binding);
            self.initialized.remove(&binding);
        }
        self.ledger
            .retire_scope_success(&declarations.into_iter().collect::<Vec<_>>());
        Ok(())
    }

    fn retire_scope_error(&mut self, declarations: &[BindingRefV1]) {
        for binding in declarations.iter().copied() {
            if self.active.remove(&binding) {
                self.initialized.remove(&binding);
                self.ledger.retire_materialized(binding);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
    use crate::mir::compiler::direct_accum_projection::direct_accum_function_for_test;
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
    use crate::mir::resolved_semantics::{ResolvedAssignmentTargetV1, ResolvedLexicalRefV1};

    fn fixture() -> VerifiedResolvedSourceUnitV1 {
        VerifiedResolvedSourceUnitV1::resolve_function(direct_accum_function_for_test())
            .expect("DirectAccum fixture resolves")
    }

    fn first_local(function: &VerifiedResolvedFunctionV1) -> (SourceBindingSiteV1, BindingRefV1) {
        let site = function
            .declaration_sites()
            .find(|site| matches!(site, SourceBindingSiteV1::Local { ordinal: 0, .. }))
            .cloned()
            .expect("first local declaration");
        let binding = function
            .declaration_binding(&site)
            .expect("first local binding");
        (site, binding)
    }

    fn first_use(function: &VerifiedResolvedFunctionV1, binding: BindingRefV1) -> SourceExprSiteV1 {
        function
            .variable_refs()
            .find_map(|(site, reference)| {
                (reference == &ResolvedLexicalRefV1::Local(binding)).then_some(site.clone())
            })
            .expect("first local use")
    }

    fn first_assignment(
        function: &VerifiedResolvedFunctionV1,
        binding: BindingRefV1,
    ) -> SourceExprSiteV1 {
        function
            .assignment_targets()
            .find_map(|(site, target)| {
                (target == &ResolvedAssignmentTargetV1::BindingRebind(binding))
                    .then_some(site.clone())
            })
            .expect("first local assignment")
    }

    #[test]
    fn declaration_only_activation_has_no_ssa_definition_or_phi() {
        let unit = fixture();
        let function = unit.root_function_input().expect("input").function();
        let (site, binding) = first_local(function);
        let record = function.binding(binding).expect("binding record");
        let mut state = ResolvedSsaIdentityStateV2::new(function);

        assert_eq!(
            state.activate_declaration_without_value(
                &site,
                record.kind(),
                record.diagnostic_name()
            ),
            Ok(binding)
        );
        assert!(state.active.contains(&binding));
        assert!(!state.initialized.contains(&binding));
    }

    #[test]
    fn uninitialized_read_rejects_before_ssa_can_open_a_phi() {
        let unit = fixture();
        let function = unit.root_function_input().expect("input").function();
        let (site, binding) = first_local(function);
        let record = function.binding(binding).expect("binding record");
        let use_site = first_use(function, binding);
        let mut state = ResolvedSsaIdentityStateV2::new(function);
        state
            .activate_declaration_without_value(&site, record.kind(), record.diagnostic_name())
            .expect("declaration-only activation");

        let mut builder = MirBuilder::new();
        let mut phis = PhiTxn::begin("canonical-identity-test");
        let error = state
            .claim_variable_use_binding(&use_site, binding)
            .expect_err("read before first definition must reject");
        assert!(error.contains("uninitialized_binding"));

        let error = state
            .read_entry(&mut builder, &mut phis, BasicBlockId::new(0), binding)
            .expect_err("entry read before first definition must reject");
        assert!(error.contains("uninitialized_binding"));
    }

    #[test]
    fn first_assignment_initializes_once_and_retirement_keeps_history() {
        let unit = fixture();
        let function = unit.root_function_input().expect("input").function();
        let (site, binding) = first_local(function);
        let record = function.binding(binding).expect("binding record");
        let assignment = first_assignment(function, binding);
        let mut state = ResolvedSsaIdentityStateV2::new(function);
        state
            .activate_declaration_without_value(&site, record.kind(), record.diagnostic_name())
            .expect("declaration-only activation");
        state
            .define_assignment_exact(&assignment, binding, BasicBlockId::new(0), ValueId::new(7))
            .expect("first assignment defines the reaching value");
        assert!(state.initialized.contains(&binding));
        let mut builder = MirBuilder::new();
        let mut phis = PhiTxn::begin("canonical-identity-test");
        assert_eq!(
            state
                .read_entry(&mut builder, &mut phis, BasicBlockId::new(0), binding)
                .expect("first assignment is the reaching value"),
            ValueId::new(7)
        );
        assert!(state
            .define_assignment_exact(&assignment, binding, BasicBlockId::new(0), ValueId::new(8),)
            .is_err());

        ResolvedScopeRetirementV1::retire_scope_success(&mut state, &[binding])
            .expect("scope retirement");
        assert!(!state.active.contains(&binding));
        assert!(!state.initialized.contains(&binding));
    }
}
