//! Affine Raw ordinary-`New` claim capability.
use crate::mir::builder::fields::{PreparedExactFieldReadClaimV1, PreparedRawFieldReadV1};

#[path = "raw_ordinary_new_claim/terminal_call.rs"]
mod terminal_call;

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

impl RawOrdinaryNewClaimPortV1 for super::RawInvocationChildPortV1<'_, '_> {
    fn emit_terminal_i64_call_exit(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        terminal_call::emit_terminal_i64_call_exit(self, builder)
    }
    fn prepare_terminal_field_read(
        &mut self,
        object: crate::ast::ASTNode,
    ) -> Result<Option<PreparedRawFieldReadV1>, String> {
        let Some(ledger) = &self.ordinary_new_claim_ledger else {
            return Ok(None);
        };
        let owner = self
            .callable_owner_v1()
            .ok_or("[ordinary-field-read/owner-missing]")?;
        let node = self
            .current_source_site_v1()
            .ok_or("[ordinary-field-read/site-missing]")?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node),
        );
        if ledger.terminal_result_blocks_raw_field_read(&site) {
            return Err(
                "[freeze:contract][ordinary-terminal-result/raw-field-read-reentry]".into(),
            );
        }
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or("[ordinary-field-read/state-missing]")?;
        let resolve = |binding| {
            state
                .borrow()
                .value_for_exact_binding(owner, binding)
                .map_err(|error| {
                    format!("[freeze:contract][ordinary-field-read/receiver-binding] {error:?}")
                })
        };
        if let Some((base, field)) = ledger.take_terminal_field_read(&site, &resolve)? {
            return Ok(Some(PreparedRawFieldReadV1::exact_object(
                object,
                base,
                field,
                site,
                ledger.clone(),
                PreparedExactFieldReadClaimV1::Terminal,
            )));
        }
        // A claimed `local x = recv.field` initializer reaches the same
        // raw FieldAccess dispatch — its staged row carries the sealed
        // declared-type classification that decides the destination type.
        if let Some((base, field, result)) = ledger.take_local_field_read(&site, &resolve)? {
            return Ok(Some(PreparedRawFieldReadV1::exact_object(
                object,
                base,
                field,
                site,
                ledger.clone(),
                PreparedExactFieldReadClaimV1::Local(result),
            )));
        }
        Ok(None)
    }
    fn prepare_root_home_exit(&mut self, builder: &crate::mir::MirBuilder) -> Result<bool, String> {
        let Some(ledger) = &self.ordinary_new_claim_ledger else {
            return Ok(false);
        };
        let owner = self
            .callable_owner_v1()
            .ok_or("[root-home-exit/owner-missing]")?;
        let site = self
            .current_source_site_v1()
            .ok_or("[root-home-exit/site-missing]")?;
        let selected = ledger.prepare_root_home_exit(owner, &site)?;
        if selected
            && builder
                .function_state
                .protected_region
                .return_defer
                .is_active()
        {
            return Err("[freeze:contract][root-home-exit/protected-region]".into());
        }
        Ok(selected)
    }

    fn emit_terminal_i64_add_return(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        let Some(ledger) = &self.ordinary_new_claim_ledger else {
            return Ok(None);
        };
        let owner = self
            .callable_owner_v1()
            .ok_or("[ordinary-terminal-result/owner-missing]")?;
        let site = self
            .current_source_site_v1()
            .ok_or("[ordinary-terminal-result/site-missing]")?;
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or("[ordinary-terminal-result/state-missing]")?;
        let Some(prepared) = ledger.prepare_terminal_i64_add_return(owner, &site, |binding, source_site| {
            let value = state.borrow_mut().read_variable(source_site)?;
            let expected = state.borrow().value_for_exact_binding(owner, binding).map_err(|error|
                format!("[freeze:contract][ordinary-terminal-result/receiver-binding] {error:?}"))?;
            if value != expected { return Err("[freeze:contract][ordinary-terminal-result/receiver-binding-drift]".into()); }
            Ok(value)
        })? else { return Ok(None); };
        crate::mir::builder::ordinary_new_admission::selected::emit_terminal_i64_add_return(
            builder, ledger, prepared,
        )
        .map(Some)
    }

    fn emit_terminal_integer_literal_return(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        let Some(ledger) = &self.ordinary_new_claim_ledger else {
            return Ok(None);
        };
        let owner = self
            .callable_owner_v1()
            .ok_or("[ordinary-literal/owner-missing]")?;
        let site = self
            .current_source_site_v1()
            .ok_or("[ordinary-literal/site-missing]")?;
        let Some(value) = ledger.prepare_terminal_integer_literal_return(owner, &site)? else {
            return Ok(None);
        };
        let emitted = crate::mir::builder::emission::constant::emit_integer(builder, value)?;
        ledger.record_terminal_integer_literal_return(owner, &site, emitted)?;
        Ok(Some(emitted))
    }

    fn emit_terminal_i64_field_return(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        let Some(ledger) = &self.ordinary_new_claim_ledger else {
            return Ok(None);
        };
        let owner = self
            .callable_owner_v1()
            .ok_or("[ordinary-field-return/owner-missing]")?;
        let site = self
            .current_source_site_v1()
            .ok_or("[ordinary-field-return/site-missing]")?;
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or("[ordinary-field-return/state-missing]")?;
        let Some(prepared) =
            ledger.prepare_terminal_i64_field_return(owner, &site, |binding, source_site| {
                let value = state.borrow_mut().read_variable(source_site)?;
                let expected = state
                    .borrow()
                    .value_for_exact_binding(owner, binding)
                    .map_err(|error| {
                        format!(
                            "[freeze:contract][ordinary-field-return/receiver-binding] {error:?}"
                        )
                    })?;
                if value != expected {
                    return Err(
                        "[freeze:contract][ordinary-field-return/receiver-binding-drift]".into(),
                    );
                }
                Ok(value)
            })?
        else {
            return Ok(None);
        };
        crate::mir::builder::ordinary_new_admission::selected::emit_terminal_i64_field_return(
            builder, ledger, prepared,
        )
        .map(Some)
    }

    fn emit_terminal_map_get_return(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        let Some(ledger) = &self.ordinary_new_claim_ledger else {
            return Ok(None);
        };
        let owner = self
            .callable_owner_v1()
            .ok_or("[ordinary-map-get-return/owner-missing]")?;
        let site = self
            .current_source_site_v1()
            .ok_or("[ordinary-map-get-return/site-missing]")?;
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or("[ordinary-map-get-return/state-missing]")?;
        let Some(prepared) =
            ledger.prepare_terminal_map_get_return(owner, &site, |binding, source_site| {
                let value = state.borrow_mut().read_variable(source_site)?;
                let expected = state
                    .borrow()
                    .value_for_exact_binding(owner, binding)
                    .map_err(|error| {
                        format!(
                            "[freeze:contract][ordinary-map-get-return/receiver-binding] {error:?}"
                        )
                    })?;
                if value != expected {
                    return Err(
                        "[freeze:contract][ordinary-map-get-return/receiver-binding-drift]".into(),
                    );
                }
                Ok(value)
            })?
        else {
            return Ok(None);
        };
        crate::mir::builder::ordinary_new_admission::selected::emit_terminal_map_get_return(
            builder,
            &mut state.borrow_mut(),
            ledger,
            owner,
            prepared,
        )
        .map(Some)
    }

    fn emit_root_home_exit(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
        value: crate::mir::ValueId,
    ) -> Result<crate::mir::ValueId, String> {
        let owner = self
            .callable_owner_v1()
            .ok_or("[root-home-exit/owner-missing]")?;
        let site = self
            .current_source_site_v1()
            .ok_or("[root-home-exit/site-missing]")?;
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or("[root-home-exit/state-missing]")?;
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .ok_or("[root-home-exit/ledger-missing]")?;
        crate::mir::builder::ordinary_new_admission::selected::emit_root_home_exit(
            builder,
            &mut state.borrow_mut(),
            ledger,
            owner,
            &crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site),
            value,
        )
    }
    fn emit_root_home_unit_exit(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
    ) -> Result<crate::mir::ValueId, String> {
        let owner = self
            .callable_owner_v1()
            .ok_or("[root-home-unit-exit/owner-missing]")?;
        let site = self
            .current_source_site_v1()
            .ok_or("[root-home-unit-exit/site-missing]")?;
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or("[root-home-unit-exit/state-missing]")?;
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .ok_or("[root-home-unit-exit/ledger-missing]")?;
        if !ledger.prepare_terminal_unit_return(owner, &site)? {
            return Err("[freeze:contract][root-home-unit-exit/source-unavailable]".into());
        }
        crate::mir::builder::ordinary_new_admission::selected::emit_root_home_unit_exit(
            builder,
            &mut state.borrow_mut(),
            ledger,
            owner,
            &crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site),
        )
    }
    fn prepare_ordinary_new_emission(
        &mut self,
        _builder: &crate::mir::MirBuilder,
        claim: &crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
    ) -> Result<bool, String> {
        self.check_new_emission_scope(claim.site())?;
        self.ordinary_new_claim_ledger
            .as_ref()
            .expect("checked ledger")
            .prepare_new_emission(claim)
    }

    fn emit_ordinary_new_claim(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
        claim: crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
    ) -> Result<crate::mir::ValueId, String> {
        self.check_new_emission_scope(claim.site())?;
        crate::mir::builder::ordinary_new_admission::selected::emit(
            builder,
            &mut self
                .callable_ledger
                .as_ref()
                .expect("checked state")
                .borrow_mut(),
            self.ordinary_new_claim_ledger
                .as_ref()
                .expect("checked ledger"),
            claim,
        )
    }

    fn try_take_result_new_claim(
        &mut self,
        class: &str,
        argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1>,
        String,
    > {
        let Some(ledger) = self.ordinary_new_claim_ledger.as_ref() else {
            return Ok(None);
        };
        let Some(site) = self.current_source_site_v1() else {
            return Ok(None);
        };
        let Some(owner) = self.callable_owner_v1() else {
            return Err("[freeze:contract][raw-ordinary-new/claim-owner-missing]".to_owned());
        };
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site),
        );
        ledger
            .try_take_result(&site, class, argument_count)
            .map_err(|error| format!("[freeze:contract][raw-ordinary-new/result-claim] {error:?}"))
    }

    fn prepare_result_new_emission(
        &mut self,
        _builder: &crate::mir::MirBuilder,
        claim: &crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1,
    ) -> Result<bool, String> {
        self.check_new_emission_scope(claim.site())?;
        self.ordinary_new_claim_ledger
            .as_ref()
            .expect("checked ledger")
            .prepare_result_new_emission(claim)
    }

    fn emit_result_new_claim(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
        claim: crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1,
    ) -> Result<crate::mir::ValueId, String> {
        self.check_new_emission_scope(claim.site())?;
        crate::mir::builder::ordinary_new_admission::selected::emit_result(
            builder,
            &mut self
                .callable_ledger
                .as_ref()
                .expect("checked state")
                .borrow_mut(),
            self.ordinary_new_claim_ledger
                .as_ref()
                .expect("checked ledger"),
            claim,
        )
    }

    fn validate_named_array_construction_route(&self, named_route: bool) -> Result<(), String> {
        let (Some(ledger), Some(site)) =
            (self.callable_ledger.as_ref(), self.current_source_site_v1())
        else {
            return Ok(());
        };
        let site = crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site);
        if !named_route && ledger.borrow().requires_named_array_allocation(&site) {
            return Err("[freeze:contract][named-array/non-named-construction-route]".into());
        }
        Ok(())
    }

    fn complete_ordinary_new_expression(
        &mut self,
        class: &str,
        value: crate::mir::ValueId,
    ) -> Result<(), String> {
        self.record_named_array_allocation_v1(value)?;
        let Some(ledger) = self.ordinary_new_claim_ledger.as_ref() else {
            return Ok(());
        };
        let Some(site) = self.current_source_site_v1() else {
            return Ok(());
        };
        let owner = self
            .callable_owner_v1()
            .ok_or_else(|| "[freeze:contract][raw-ordinary-new/claim-owner-missing]".to_owned())?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site),
        );
        // Forward claim completions only: local-initializer claims keep the
        // exact statement shape, and return-position claims forward when a
        // taken commit row exists at this exact site (a `Value` segment is
        // shared by assignment/print/nowait children that hold no claim).
        if !matches!(
            site.site().node().segments(),
            [
                crate::mir::resolved_semantics::SourcePathSegmentV1::Body(_),
                crate::mir::resolved_semantics::SourcePathSegmentV1::Initializer(_),
            ]
        ) && !ledger.has_result_new_commit(&site)
        {
            return Ok(());
        }
        ledger.complete_new_expression(&site, class, value)
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
        let Some(site) = self.current_source_site_v1() else {
            return Ok(None);
        };
        let Some(ledger) = self.callable_ledger.as_ref() else {
            return Ok(None);
        };
        let site = crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site);
        Ok(ledger
            .borrow()
            .named_array_field_provider(&site)
            .map(|field| (site, field)))
    }

    fn try_take_ordinary_new_claim(
        &mut self,
        class: &str,
        argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1>,
        String,
    > {
        let Some(ledger) = self.ordinary_new_claim_ledger.as_ref() else {
            return Ok(None);
        };
        let Some(site) = self.current_source_site_v1() else {
            return Ok(None);
        };
        if !matches!(
            site.segments(),
            [
                crate::mir::resolved_semantics::SourcePathSegmentV1::Body(_),
                crate::mir::resolved_semantics::SourcePathSegmentV1::Initializer(_)
            ]
        ) {
            return Ok(None);
        }
        let Some(owner) = self.callable_owner_v1() else {
            return Err("[freeze:contract][raw-ordinary-new/claim-owner-missing]".to_owned());
        };
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site),
        );
        ledger
            .try_take(&site, class, argument_count)
            .map_err(|error| format!("[freeze:contract][raw-ordinary-new/claim] {error:?}"))
    }

    fn try_take_ordinary_new_birth_recipe(
        &mut self,
        class: &str,
        argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::VerifiedOrdinaryNewBirthRecipeV1>,
        String,
    > {
        let Some(ledger) = self.ordinary_new_claim_ledger.as_ref() else {
            return Ok(None);
        };
        let (Some(owner), Some(node)) = (self.callable_owner_v1(), self.current_source_site_v1())
        else {
            return Ok(None);
        };
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node),
        );
        // The raw lane keeps its prior contract: it consumes only the
        // recipe half of the indexed pair; the sealed handoff is dropped
        // because this emission path has no ABI handoff consumer.
        ledger
            .take_birth_site_recipe(&site, class, argument_count)
            .map(|entry| entry.map(|(recipe, _handoff)| recipe))
            .map_err(|error| format!("[freeze:contract][raw-ordinary-new/birth-site] {error:?}"))
    }
}

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
