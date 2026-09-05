//! Affine Raw ordinary-`New` claim capability.

pub(in crate::mir::builder) trait RawOrdinaryNewClaimPortV1 {
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
