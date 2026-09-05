//! Affine Raw ordinary-`New` claim capability.

pub(in crate::mir::builder) trait RawOrdinaryNewClaimPortV1 {
    fn prepare_ordinary_new_emission(
        &mut self, _builder: &crate::mir::MirBuilder,
        _claim: &crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
    ) -> Result<bool, String> {
        Err("[freeze:contract][raw-ordinary-new/no-physical-owner]".into())
    }

    fn emit_ordinary_new_claim(
        &mut self, _builder: &mut crate::mir::MirBuilder,
        _claim: crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
        _arguments: Vec<crate::mir::ValueId>,
    ) -> Result<crate::mir::ValueId, String> {
        Err("[freeze:contract][raw-ordinary-new/no-physical-owner]".into())
    }

    fn try_take_ordinary_new_claim(
        &mut self,
        class: &str,
        argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1>,
        String,
    >;

    fn complete_ordinary_new_expression(
        &mut self,
        class: &str,
        value: crate::mir::ValueId,
    ) -> Result<(), String>;
}

impl RawOrdinaryNewClaimPortV1 for super::RawLegacyChildLoweringPortV1 {
    fn complete_ordinary_new_expression(
        &mut self,
        _class: &str,
        _value: crate::mir::ValueId,
    ) -> Result<(), String> {
        Ok(())
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
    fn prepare_ordinary_new_emission(
        &mut self, _builder: &crate::mir::MirBuilder,
        claim: &crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
    ) -> Result<bool, String> {
        self.check_new_emission_scope(claim)?;
        self.ordinary_new_claim_ledger.as_ref().expect("checked ledger")
            .prepare_new_emission(claim)
    }

    fn emit_ordinary_new_claim(
        &mut self, builder: &mut crate::mir::MirBuilder,
        claim: crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
        arguments: Vec<crate::mir::ValueId>,
    ) -> Result<crate::mir::ValueId, String> {
        self.check_new_emission_scope(&claim)?;
        crate::mir::builder::ordinary_new_admission::selected::emit(
            builder, &mut self.callable_ledger.as_ref().expect("checked state").borrow_mut(),
            self.ordinary_new_claim_ledger.as_ref().expect("checked ledger"), claim, arguments)
    }

    fn complete_ordinary_new_expression(
        &mut self,
        class: &str,
        value: crate::mir::ValueId,
    ) -> Result<(), String> {
        let Some(ledger) = self.ordinary_new_claim_ledger.as_ref() else {
            return Ok(());
        };
        let Some(site) = self.current_source_site_v1() else {
            return Ok(());
        };
        if !matches!(
            site.segments(),
            [
                crate::mir::resolved_semantics::SourcePathSegmentV1::Body(_),
                crate::mir::resolved_semantics::SourcePathSegmentV1::Initializer(_),
            ]
        ) {
            return Ok(());
        }
        let owner = self
            .callable_owner_v1()
            .ok_or_else(|| "[freeze:contract][raw-ordinary-new/claim-owner-missing]".to_owned())?;
        ledger.complete_new_expression(
            &crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                owner,
                crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site),
            ),
            class,
            value,
        )
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
}

impl super::RawInvocationChildPortV1<'_, '_> {
    fn check_new_emission_scope(
        &self, claim: &crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
    ) -> Result<(), String> {
        let state = self.callable_ledger.as_ref()
            .ok_or("[freeze:contract][raw-ordinary-new/callable-state-missing]")?;
        let node = self.current_source_site_v1()
            .ok_or("[freeze:contract][raw-ordinary-new/source-site-missing]")?;
        let owner = state.borrow().owner();
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node));
        if self.ordinary_new_claim_ledger.is_none()
            || self.callable_owner_v1() != Some(owner) || &site != claim.site() {
            return Err("[freeze:contract][raw-ordinary-new/emission-scope-mismatch]".into());
        }
        Ok(())
    }
}
