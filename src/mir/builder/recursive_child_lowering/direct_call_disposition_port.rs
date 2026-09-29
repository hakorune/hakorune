//! Direct-call disposition port: one authority for scope-checked emission.
use super::*;

impl DirectCallDispositionPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn take_direct_call_disposition_v1(
        &mut self,
    ) -> Result<crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1, String>
    {
        if !self.is_direct_call_scope_v1() {
            return Err("[freeze:contract][direct-call/scope-mismatch]".to_owned());
        }
        self.take_direct_call_disposition_inner_v1()
    }

    fn emit_local_lifecycle_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        row: crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1,
        arguments: Vec<ValueId>,
    ) -> Result<Option<ValueId>, String> {
        if !self.is_direct_call_scope_v1() {
            return Ok(None);
        }
        let owner = self
            .callable_owner_v1()
            .ok_or_else(|| "[freeze:contract][local-call/owner-missing]".to_owned())?;
        let site = self
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][local-call/site-missing]".to_owned())?;
        let site = crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site);
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][local-call/ledger-missing]".to_owned())?;
        if ledger.local_call_for_owner(owner, &site).is_none() {
            return Ok(None);
        }
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][local-call/state-missing]".to_owned())?;
        let value =
            crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_local(
                builder,
                &mut state.borrow_mut(),
                ledger,
                owner,
                &site,
                row,
                arguments,
            )?;
        Ok(Some(value))
    }

    fn emit_local_lexical_lifecycle_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        method: &str,
    ) -> Result<Option<ValueId>, String> {
        let Some(RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Cataloged(_),
            site,
            ..
        }) = self.active_source.as_ref()
        else {
            return Ok(None);
        };
        let owner = self
            .callable_owner_v1()
            .ok_or_else(|| "[freeze:contract][lexical-handle/owner-missing]".to_owned())?;
        let Some(ledger) = self.ordinary_new_claim_ledger.as_ref() else {
            return Ok(None);
        };
        let site = crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site.clone());
        let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
        if ledger.handle_call_source(&owned).is_none() {
            return Ok(None);
        }
        // The caller-side scan sealed a Handle observation at this site:
        // the disposition row must exist, agree on the selector, and carry
        // the same Handle contract. Any half-sealed edge freezes.
        let Some(row) = ledger.take_lexical_instance_call(owner, &site)? else {
            return Err(
                "[freeze:contract][lexical-handle/disposition-missing]".to_owned()
            );
        };
        if row.result()
            != Some(crate::mir::instruction::InvokeCallResultKind::Handle)
            || row.target().name() != method
        {
            return Err("[freeze:contract][lexical-handle/result-mismatch]".to_owned());
        }
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][lexical-handle/state-missing]".to_owned())?;
        let value = crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_local_lexical(
            builder,
            &mut state.borrow_mut(),
            ledger,
            owner,
            &site,
            row,
        )?;
        Ok(Some(value))
    }

    fn validate_current_call_argument_site_v1(
        &self,
        expected: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<(), String> {
        let actual = self
            .current_source_site_v1()
            .map(crate::mir::resolved_semantics::SourceExprSiteV1::from_node)
            .ok_or_else(|| "[freeze:contract][direct-call/argument-site-missing]".to_owned())?;
        if &actual != expected {
            return Err("[freeze:contract][direct-call/argument-site-mismatch]".to_owned());
        }
        Ok(())
    }
}
