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

/// Exact source identity plus the sole reaching-value authority for SSA-I1-T.
pub(super) struct ResolvedSsaIdentityStateV2<'source> {
    ledger: ResolvedIdentityLedgerV2<'source>,
    ssa: BindingSsaBuilderV1<PhiToken>,
    active: BTreeSet<BindingRefV1>,
}

impl<'source> ResolvedSsaIdentityStateV2<'source> {
    pub(super) fn new(product: &'source VerifiedResolvedFunctionV1) -> Self {
        Self {
            ledger: ResolvedIdentityLedgerV2::new(product),
            ssa: BindingSsaBuilderV1::new(product.owner()),
            active: BTreeSet::new(),
        }
    }

    pub(super) fn publish_declaration(
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
        Ok(binding)
    }

    pub(super) fn variable_value(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        block: BasicBlockId,
        site: &SourceExprSiteV1,
        expected_name: &str,
    ) -> Result<(BindingRefV1, ValueId), String> {
        let binding = self.ledger.claim_variable_use(site, expected_name)?;
        self.require_active(binding)?;
        let mut adapter = MirBindingSsaAdapterV1::new(builder, phis);
        let value = self
            .ssa
            .read(&mut adapter, binding, block)
            .map_err(|error| error.to_string())?;
        Ok((binding, value))
    }

    pub(super) fn resolve_assignment_binding(
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

    pub(super) fn define_assignment(
        &mut self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.ledger.claim_assignment_binding(site, binding)?;
        self.ssa
            .define(binding, block, value)
            .map_err(|error| error.to_string())
    }

    pub(super) fn seal_block(
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

    pub(super) fn mark_return(&mut self, site: ResolvedExitSiteV1) -> Result<(), String> {
        self.ledger.mark_return(site)
    }

    pub(super) fn finish(self) -> Result<(), String> {
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
}

impl ResolvedScopeRetirementV1 for ResolvedSsaIdentityStateV2<'_> {
    fn retire_scope_success(&mut self, declarations: &[BindingRefV1]) -> Result<(), String> {
        let declarations = declarations.iter().copied().collect::<BTreeSet<_>>();
        for binding in declarations.iter().copied() {
            self.require_active(binding)?;
            self.active.remove(&binding);
        }
        self.ledger
            .retire_scope_success(&declarations.into_iter().collect::<Vec<_>>());
        Ok(())
    }

    fn retire_scope_error(&mut self, declarations: &[BindingRefV1]) {
        for binding in declarations.iter().copied() {
            if self.active.remove(&binding) {
                self.ledger.retire_materialized(binding);
            }
        }
    }
}
