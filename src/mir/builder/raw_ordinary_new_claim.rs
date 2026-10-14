//! Affine Raw ordinary-`New` claim capability.
use crate::mir::builder::fields::{PreparedExactFieldReadClaimV1, PreparedRawFieldReadV1};

#[path = "raw_ordinary_new_claim/terminal_call.rs"]
mod terminal_call;

#[path = "raw_ordinary_new_claim/result_claim_trace.rs"]
mod result_claim_trace;

pub(in crate::mir::builder) trait RawOrdinaryNewClaimPortV1 {
    fn emit_terminal_i64_call_exit(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        Ok(None)
    }
    fn prepare_terminal_field_read(
        &mut self,
        _object: crate::ast::ASTNode,
    ) -> Result<Option<PreparedRawFieldReadV1>, String> {
        Ok(None)
    }
    fn prepare_root_home_exit(
        &mut self,
        _builder: &crate::mir::MirBuilder,
    ) -> Result<bool, String> {
        Ok(false)
    }
    fn emit_terminal_i64_add_return(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        Ok(None)
    }
    fn emit_terminal_integer_literal_return(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        Ok(None)
    }

    fn emit_terminal_i64_field_return(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        Ok(None)
    }

    fn emit_terminal_map_get_return(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        Ok(None)
    }

    fn emit_root_home_exit(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
        _value: crate::mir::ValueId,
    ) -> Result<crate::mir::ValueId, String> {
        Err("[freeze:contract][root-home-exit/no-physical-owner]".into())
    }
    fn emit_root_home_unit_exit(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
    ) -> Result<crate::mir::ValueId, String> {
        Err("[freeze:contract][root-home-unit-exit/no-physical-owner]".into())
    }
    fn prepare_ordinary_new_emission(
        &mut self,
        _builder: &crate::mir::MirBuilder,
        _claim: &crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
    ) -> Result<bool, String> {
        Err("[freeze:contract][raw-ordinary-new/no-physical-owner]".into())
    }

    fn emit_ordinary_new_claim(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
        _claim: crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
    ) -> Result<crate::mir::ValueId, String> {
        Err("[freeze:contract][raw-ordinary-new/no-physical-owner]".into())
    }

    /// Return-position `new` claim take. The claim map is self-gating by
    /// exact site — ports without a ledger simply answer `None`.
    fn try_take_result_new_claim(
        &mut self,
        _class: &str,
        _argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1>,
        String,
    > {
        Ok(None)
    }

    fn prepare_result_new_emission(
        &mut self,
        _builder: &crate::mir::MirBuilder,
        _claim: &crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1,
    ) -> Result<bool, String> {
        Err("[freeze:contract][raw-ordinary-new/no-physical-owner]".into())
    }

    fn emit_result_new_claim(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
        _claim: crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1,
    ) -> Result<crate::mir::ValueId, String> {
        Err("[freeze:contract][raw-ordinary-new/no-physical-owner]".into())
    }

    fn validate_named_array_construction_route(&self, _named_route: bool) -> Result<(), String> {
        Ok(())
    }

    fn try_take_ordinary_new_claim(
        &mut self,
        class: &str,
        argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1>,
        String,
    >;

    /// Destination-less verified `Birth` recipe for a `new` site outside the
    /// local-commit claim lane.  `Ok(None)` keeps the caller's existing
    /// terminal; it never substitutes a claim.
    fn try_take_ordinary_new_birth_recipe(
        &mut self,
        _class: &str,
        _argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::VerifiedOrdinaryNewBirthRecipeV1>,
        String,
    > {
        Ok(None)
    }

    fn complete_ordinary_new_expression(
        &mut self,
        class: &str,
        value: crate::mir::ValueId,
    ) -> Result<(), String>;

    /// `Some((site, field))` when the `new` site just completed is a claimed
    /// birth-side provider for a field-resident named-array requirement.
    /// Unscoped compatibility facades return `None` because they never carry
    /// a callable-ledger provider claim.
    fn named_array_field_provider_recording(
        &mut self,
    ) -> Result<
        Option<(
            crate::mir::resolved_semantics::SourceExprSiteV1,
            hakorune_mir_defs::CanonicalFieldRefV1,
        )>,
        String,
    >;
}

impl RawOrdinaryNewClaimPortV1 for super::RawLegacyChildLoweringPortV1 {
    fn complete_ordinary_new_expression(
        &mut self,
        _class: &str,
        _value: crate::mir::ValueId,
    ) -> Result<(), String> {
        Ok(())
    }
    fn named_array_field_provider_recording(
        &mut self,
    ) -> Result<
        Option<(
            crate::mir::resolved_semantics::SourceExprSiteV1,
            hakorune_mir_defs::CanonicalFieldRefV1,
        )>,
        String,
    > {
        Ok(None)
    }
    fn try_take_ordinary_new_claim(
        &mut self,
        _class: &str,
        _argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1>,
        String,
    > {
        Ok(None)
    }
}

#[path = "raw_ordinary_new_claim/invocation.rs"]
mod invocation;

impl super::RawInvocationChildPortV1<'_, '_> {
    fn check_new_emission_scope(
        &self,
        claim_site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    ) -> Result<(), String> {
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or("[freeze:contract][raw-ordinary-new/callable-state-missing]")?;
        let node = self
            .current_source_site_v1()
            .ok_or("[freeze:contract][raw-ordinary-new/source-site-missing]")?;
        let owner = state.borrow().owner();
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node),
        );
        if self.ordinary_new_claim_ledger.is_none()
            || self.callable_owner_v1() != Some(owner)
            || &site != claim_site
        {
            return Err("[freeze:contract][raw-ordinary-new/emission-scope-mismatch]".into());
        }
        Ok(())
    }
}

impl super::RawInvocationChildPortV1<'_, '_> {
    pub(in crate::mir::builder) fn record_named_array_allocation_v1(
        &self,
        destination: crate::mir::ValueId,
    ) -> Result<(), String> {
        if let (Some(ledger), Some(site)) =
            (self.callable_ledger.as_ref(), self.current_source_site_v1())
        {
            ledger.borrow_mut().record_named_array_allocation(
                &crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site),
                destination,
            )?;
        }
        Ok(())
    }
}
