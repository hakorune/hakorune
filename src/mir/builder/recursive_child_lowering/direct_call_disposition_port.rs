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

    fn emit_local_static_lifecycle_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        owner_name: &str,
        method: &str,
        arity: usize,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::builder::static_result_publication_ingress::{
            StaticResultPublicationIngressPortV1, StaticResultPublicationIngressV1,
        };
        let Some(owner) = self.callable_owner_v1() else {
            return Ok(None);
        };
        let Some(site) = self.current_source_site_v1() else {
            return Ok(None);
        };
        let site = crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site);
        let Some(ledger) = self.ordinary_new_claim_ledger.clone() else {
            return Ok(None);
        };
        let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
        let Some(original) = ledger.selected_static_local_source_v1(&owned)? else {
            return Ok(None);
        };
        if original.target().owner() != owner_name
            || original.target().name() != method
            || original.target().arity() as usize != arity
        {
            return Err("[freeze:contract][borrowed-static/local-target-drift]".into());
        }
        let ingress = self
            .take_static_result_publication_ingress_v1(
                builder.comp_ctx.callable_declaration_catalog().ok(),
                owner_name,
                method,
                arity,
            )
            .map_err(|error| error.to_string())?;
        let StaticResultPublicationIngressV1::Selected(publication) = ingress else {
            return Err("[freeze:contract][borrowed-static/local-publication-not-selected]".into());
        };
        let source =
            crate::mir::normal_callable_semantic_package::CallPacketSourceV1::qualified_static(
                original,
                publication,
                &ledger,
            )?;
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][borrowed-static/local-state-missing]".to_owned())?;
        let value = crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_local_lexical_source(
            builder, &mut state.borrow_mut(), &ledger, owner, &site, source,
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
        let handle = ledger.handle_call_source(&owned).is_some();
        let scalar = ledger.lexical_i64_call_source(&owned).is_some();
        let nullable = ledger.nullable_call_source(&owned).is_some();
        if !handle && !scalar && !nullable {
            return Ok(None);
        }
        // The caller-side scan sealed a local-call observation at this
        // site: the disposition row must exist, agree on the selector, and
        // carry the same result contract. Any half-sealed edge freezes.
        let Some(row) = ledger.take_lexical_instance_call(owner, &site)? else {
            return Err("[freeze:contract][lexical-instance-call/disposition-missing]".to_owned());
        };
        if row.target().name() != method {
            return Err("[freeze:contract][lexical-instance-call/target-mismatch]".to_owned());
        }
        let expected = if handle {
            crate::mir::instruction::InvokeCallResultKind::Handle
        } else if nullable {
            crate::mir::instruction::InvokeCallResultKind::NullableHandle
        } else {
            crate::mir::instruction::InvokeCallResultKind::I64
        };
        if row.result() != Some(expected) {
            return Err("[freeze:contract][lexical-instance-call/result-mismatch]".to_owned());
        }
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][lexical-instance-call/state-missing]".to_owned())?;
        let mut state = state.borrow_mut();
        let value = if nullable || (handle && row.source_target().has_object_source_requirement()) {
            crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_local_lexical_object(
                builder,
                &mut state,
                ledger,
                owner,
                &site,
                row,
            )?
        } else if handle {
            crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_local_lexical(
                builder,
                &mut state,
                ledger,
                owner,
                &site,
                row,
            )?
        } else {
            crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_local_lexical_i64(
                builder,
                &mut state,
                ledger,
                owner,
                &site,
                row,
            )?
        };
        Ok(Some(value))
    }

    /// Emit one `me.m(..)` receiver call the caller-side scan sealed as a
    /// `Nullable` local call. Sealed membership without the matching
    /// package observation, a target/destination disagreement, or a failed
    /// emission contract freezes — the generic canonical-instance path
    /// must never silently consume a nullable observation.
    fn emit_receiver_nullable_lifecycle_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver: ValueId,
        read: Option<crate::mir::builder::ExactLexicalReadV1>,
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
            .ok_or_else(|| "[freeze:contract][nullable-receiver/owner-missing]".to_owned())?;
        let Some(ledger) = self.ordinary_new_claim_ledger.as_ref() else {
            return Ok(None);
        };
        let site = crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site.clone());
        let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
        if let Some(row) = ledger.take_receiver_object_packet_v1(owner, &site, key)? {
            let read =
                read.ok_or_else(|| "[freeze:contract][receiver-object/read-missing]".to_owned())?;
            let state = self
                .callable_ledger
                .as_ref()
                .ok_or_else(|| "[freeze:contract][receiver-object/state-missing]".to_owned())?;
            let value = crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_receiver_object(
                builder, &mut state.borrow_mut(), ledger, owner, &site, row, key, receiver, read,
            )?;
            return Ok(Some(value));
        }
        if ledger.nullable_call_source(&owned).is_none() {
            return Ok(None);
        }
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][nullable-receiver/state-missing]".to_owned())?;
        let value =
            crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_receiver_nullable(
                builder,
                &mut state.borrow_mut(),
                ledger,
                owner,
                &site,
                key,
                receiver,
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
