//! Affine Raw ordinary-`New` claim capability.
use crate::mir::builder::fields::{PreparedExactFieldReadClaimV1, PreparedRawFieldReadV1};

#[path = "raw_ordinary_new_claim/terminal_call.rs"]
mod terminal_call;

#[path = "raw_ordinary_new_claim/result_claim_trace.rs"]
mod result_claim_trace;

pub(in crate::mir::builder) trait RawOrdinaryNewClaimPortV1 {
    fn try_emit_unit_discard(&mut self, _builder: &mut crate::mir::MirBuilder, _method: &str, _arity: usize)
        -> Result<bool, String> { Ok(false) }

    fn try_emit_owned_array_append(
        &mut self, _builder: &mut crate::mir::MirBuilder, _method: &str, _arity: usize,
    ) -> Result<bool, String> { Ok(false) }
    fn emit_terminal_i64_call_exit(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        Ok(None)
    }
    fn prepare_terminal_field_read(
        &mut self,
        _object: crate::ast::ASTNode,
        _field: &str,
    ) -> Result<Option<PreparedRawFieldReadV1>, String> {
        Ok(None)
    }
    fn prepare_root_home_exit(
        &mut self,
        _builder: &crate::mir::MirBuilder,
    ) -> Result<bool, String> {
        Ok(false)
    }
    fn emit_terminal_scalar_literal_return(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        Ok(None)
    }

    fn complete_terminal_scalar_literal_return(
        &mut self, _builder: &crate::mir::MirBuilder,
    ) -> Result<(), String> { Ok(()) }


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
    ) -> Result<crate::mir::normal_callable_semantic_package::NewEmissionSelectionV1, String> {
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
    ) -> Result<crate::mir::normal_callable_semantic_package::NewEmissionSelectionV1, String> {
        Err("[freeze:contract][raw-ordinary-new/no-physical-owner]".into())
    }

    fn emit_result_new_claim(
        &mut self,
        _builder: &mut crate::mir::MirBuilder,
        _claim: crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1,
    ) -> Result<crate::mir::ValueId, String> {
        Err("[freeze:contract][raw-ordinary-new/no-physical-owner]".into())
    }

    fn prepare_owned_array_construction(
        &mut self, _class: &str, _argument_count: usize, _field_count: usize,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::PreparedOwnedArrayConstructionV1>, String> { Ok(None) }

    fn emit_owned_array_construction(
        &mut self, _builder: &mut crate::mir::MirBuilder,
        _source: crate::mir::normal_callable_semantic_package::PreparedOwnedArrayConstructionV1,
    ) -> Result<crate::mir::ValueId, String> {
        Err("[freeze:contract][owned-array/no-physical-owner]".into())
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

impl super::RawInvocationChildPortV1<'_, '_> {
    /// Same source/receiver/candidate transaction for raw statements and the
    /// checked Loop schedule; the sole owned-array writer records the packet.
    pub(in crate::mir::builder) fn emit_owned_array_append_v1(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
        method: &str,
        arity: usize,
        argument_frame: Option<std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
        >>,
        nested_argument_frames: &[std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
        >],
    ) -> Result<
        Option<
            std::rc::Rc<crate::mir::normal_callable_semantic_package::EmittedOwnedArrayAppendV1>,
        >,
        String,
    > {
        let (Some(ledger), Some(node)) = (
            self.ordinary_new_claim_ledger.as_ref(),
            self.current_source_site_v1(),
        ) else {
            return Ok(None);
        };
        let owner = self
            .callable_owner_v1()
            .ok_or("[freeze:contract][array-append/owner-missing]")?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node),
        );
        if !ledger.has_owned_array_append_v1(&site)? {
            return Ok(None);
        }
        let source = self
            .owned_array_source
            .ok_or("[freeze:contract][array-append/source-scope-missing]")?;
        source.require_same_ledger_v1(ledger)?;
        let append = source
            .prepare_append_v1(&site)?
            .ok_or("[freeze:contract][array-append/source-missing]")?;
        let derived_argument_frames = if append.dynamic_carrier_source().is_some()
            && argument_frame.is_none()
        {
            Some(
                ledger
                    .dynamic_carrier_array_append_argument_frames_v1(&site)?
                    .ok_or("[freeze:contract][array-append/dynamic-argument-frame-missing]")?,
            )
        } else {
            None
        };
        let argument_frame = argument_frame.or_else(|| {
            derived_argument_frames
                .as_ref()
                .map(|(frame, _)| std::rc::Rc::clone(frame))
        });
        let nested_argument_frames = derived_argument_frames
            .as_ref()
            .map(|(_, frames)| frames.as_slice())
            .unwrap_or(nested_argument_frames);
        self.check_new_emission_scope(&site)?;
        if crate::boxes::array::ArrayMethodId::from_name_and_arity(method, arity)
            != Some(crate::boxes::array::ArrayMethodId::Push)
        {
            return Err("[freeze:contract][array-append/syntax-drift]".into());
        }
        let candidate = if append.null_candidate() || append.dynamic_carrier_source().is_some() {
            None
        } else {
            let site = append.candidate();
            let row = ledger
                .take_lexical_instance_call(site.owner(), site.site())?
                .ok_or("[freeze:contract][array-append/candidate-disposition-missing]")?;
            append.check_candidate_v1(&row)?;
            self.consume_lexical_self_receiver_locator_v1(&row)?;
            Some(row)
        };
        let consume_receiver = self.lexical_self_receiver_locator_consumer_v1()?;
        let packet =
            crate::mir::builder::ordinary_new_admission::selected::owned_array::emit_append(
                builder,
                &mut self
                    .callable_ledger
                    .as_ref()
                    .expect("checked source state")
                    .borrow_mut(),
                ledger,
                append,
                candidate,
                &consume_receiver,
                argument_frame,
                nested_argument_frames,
            )?;
        Ok(Some(packet))
    }
}
