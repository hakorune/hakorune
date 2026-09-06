//! Ordinary-New package adapter; physical consumption delegates to the Raw owner.
use super::*;

impl RawOrdinaryNewClaimPortV1 for NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_> {
    fn prepare_terminal_field_read(&mut self, object: crate::ast::ASTNode)
        -> Result<Option<crate::mir::builder::fields::PreparedRawFieldReadV1>, String> {
        self.check_new_ledger_identity()?;
        self.inner.prepare_terminal_field_read(object)
    }
    fn prepare_root_home_exit(&mut self, builder: &MirBuilder) -> Result<bool, String> {
        self.check_new_ledger_identity()?;
        self.inner.prepare_root_home_exit(builder)
    }

    fn emit_root_home_exit(&mut self, builder: &mut MirBuilder, value: ValueId) -> Result<ValueId, String> {
        self.check_new_ledger_identity()?;
        self.inner.emit_root_home_exit(builder, value)
    }
    fn prepare_ordinary_new_emission(
        &mut self, builder: &MirBuilder,
        claim: &crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
    ) -> Result<bool, String> {
        self.check_new_ledger_identity()?;
        self.inner.prepare_ordinary_new_emission(builder, claim)
    }

    fn emit_ordinary_new_claim(
        &mut self, builder: &mut MirBuilder,
        claim: crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.check_new_ledger_identity()?;
        self.inner.emit_ordinary_new_claim(builder, claim, arguments)
    }

    fn complete_ordinary_new_expression(
        &mut self,
        class: &str,
        value: ValueId,
    ) -> Result<(), String> {
        let owner = self
            .inner
            .callable_owner_v1()
            .ok_or_else(|| "[freeze:contract][raw-ordinary-new/claim-owner-missing]".to_owned())?;
        let site = self
            .inner
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][raw-ordinary-new/claim-site-missing]".to_owned())?;
        if !matches!(
            site.segments(),
            [
                SourcePathSegmentV1::Body(_),
                SourcePathSegmentV1::Initializer(_)
            ]
        ) || !self.package.ordinary_box_is_covered(class)
        {
            return Ok(());
        }
        self.package
            .ordinary_new_claim_ledger()
            .complete_new_expression(
                &OwnedExprSiteV1::new(owner, SourceExprSiteV1::from_node(site)),
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
        let Some(owner) = self.inner.callable_owner_v1() else {
            return Err("[freeze:contract][raw-ordinary-new/claim-owner-missing]".to_owned());
        };
        let Some(site) = self.inner.current_source_site_v1() else {
            return Err("[freeze:contract][raw-ordinary-new/claim-site-missing]".to_owned());
        };
        if !matches!(
            site.segments(),
            [
                SourcePathSegmentV1::Body(_),
                SourcePathSegmentV1::Initializer(_)
            ]
        ) || !self.package.ordinary_box_is_covered(class)
        {
            return Ok(None);
        }
        let site = OwnedExprSiteV1::new(owner, SourceExprSiteV1::from_node(site));
        self.package
            .take_ordinary_new_claim(&site, class, argument_count)
            .map(Some)
            .map_err(package_issue)
    }
}

impl NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_> {
    fn check_new_ledger_identity(&self) -> Result<(), String> {
        let package = self.package.ordinary_new_claim_ledger();
        if !self.inner.ordinary_new_claim_ledger.as_ref()
            .is_some_and(|inner| Rc::ptr_eq(inner, &package)) {
            return Err("[freeze:contract][raw-ordinary-new/package-ledger-mismatch]".into());
        }
        Ok(())
    }
}
