//! Direct-call disposition port: one authority for scope-checked emission.
use super::*;
use crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1 as InstanceRow;

impl<'port, 'collector> RawInvocationChildPortV1<'port, 'collector> {
    pub(in crate::mir::builder) fn consume_lexical_self_receiver_locator_v1(
        &self,
        row: &crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1,
    ) -> Result<(), String> {
        self.lexical_self_receiver_locator_consumer_v1()?(row)
    }

    /// Capture the existing source scope before the emitter mutably borrows
    /// callable state. The callback never reads that state or the receiver.
    pub(in crate::mir::builder) fn lexical_self_receiver_locator_consumer_v1(
        &self,
    ) -> Result<impl Fn(&InstanceRow) -> Result<(), String> + 'port, String> {
        let owner = self.callable_owner_v1()
            .ok_or("[freeze:contract][terminal-call/owner-missing]")?;
        let locator = self.declared_instance_locator;
        Ok(move |row: &InstanceRow| {
            if row.source_target().is_self_receiver() {
                // Source-selected emission bypasses generic method descent.
                // Consume its original locator without reading the receiver twice.
                let locator = locator.as_ref()
                    .ok_or("[freeze:contract][terminal-call/receiver-locator-missing]")?;
                locator
                    .take_exact_relation(row.call_site(), |relation| {
                        if relation.caller_owner() != owner
                            || relation.call_site() != row.call_site().site()
                            || relation.receiver_site() != row.receiver_site()
                            || relation.receiver_binding() != row.receiver_binding()?
                            || relation.target_key() != row.target()
                        {
                            return Err(
                                "[freeze:contract][terminal-call/receiver-locator-drift]".into()
                            );
                        }
                        Ok(())
                    })
                    .map_err(|error| {
                        format!("[freeze:contract][terminal-call/receiver-locator/{error:?}]")
                    })?;
            }
            Ok(())
        })
    }
}

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

    fn has_static_execution_for_owner_v1(&self, owner: crate::mir::resolved_semantics::FunctionOwnerIdV1) -> bool {
        self.direct_call_loans.as_deref().is_some_and(|loans| loans.has_static_execution_for_owner_v1(owner))
    }
    fn take_optional_static_execution_row_v1(&mut self) -> Result<Option<crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1>, String> {
        if !self.is_direct_call_scope_v1() { return Ok(None); }
        let Some(owner) = self.callable_owner_v1() else { return Ok(None) };
        let Some(loan) = self.direct_call_loans.as_deref_mut().and_then(|loans| loans.get_mut(owner)) else { return Ok(None) };
        let site = self.active_source.as_ref().and_then(|source| match source {
            RawInvocationSourceContextV1::Located { site, .. } => Some(site.clone()), _ => None,
        }).ok_or_else(|| "[freeze:contract][static-execution/site-missing]".to_owned())?;
        loan.take_optional_static_execution_v1(owner, crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site))
            .map_err(|error| format!("[freeze:contract][static-execution/{error:?}]"))
    }

    fn emit_static_argument_expression_v1(&mut self, builder: &mut MirBuilder,
        call: &crate::mir::resolved_semantics::OwnedExprSiteV1, ordinal: u32, site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<(ValueId, crate::mir::normal_callable_semantic_package::LexicalCallArgumentProjectionV1), String> {
        self.validate_current_call_argument_site_v1(site)?;
        if self.callable_owner_v1() != Some(call.owner()) { return Err("[freeze:contract][argument-expression/owner]".into()); }
        let ledger = self.ordinary_new_claim_ledger.as_ref().ok_or("[freeze:contract][argument-expression/ledger]")?;
        let state = self.callable_ledger.as_ref().ok_or("[freeze:contract][argument-expression/state]")?;
        let consume_receiver = self.lexical_self_receiver_locator_consumer_v1()?;
        crate::mir::builder::ordinary_new_admission::selected::emit_argument_expression_v1(
            builder, &mut state.borrow_mut(), ledger, call, ordinal, site, &consume_receiver)
    }
    fn static_execution_argument_source_v1(&self, row: &crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1)
        -> Result<crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1, String> {
        let original = row.static_source_v1().ok_or("[freeze:contract][static-execution/argument-source-origin]")?;
        let ledger = self.ordinary_new_claim_ledger.as_ref().ok_or("[freeze:contract][static-execution/argument-ledger]")?;
        self.validate_current_call_argument_site_v1(original.call_site().site())?;
        if self.callable_owner_v1() != Some(original.call_site().owner()) || row.taken_site() != Some(original.call_site()) {
            return Err("[freeze:contract][static-execution/argument-owner]".into());
        }
        row.validate_static_source_v1(ledger)?;
        let source = ledger.lexical_i64_call_source(original.call_site())
            .ok_or("[freeze:contract][static-execution/argument-observation]")?;
        if source.local_binding().is_none() || source.arguments().len() != row.argument_sites().len() {
            return Err("[freeze:contract][static-execution/argument-observation-drift]".into());
        }
        Ok(source.clone())
    }
    fn take_exact_static_argument_read_v1(&mut self,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        binding: crate::mir::resolved_semantics::BindingRefV1,
    ) -> Result<crate::mir::builder::ExactLexicalReadV1, String> {
        self.validate_current_call_argument_site_v1(site)?;
        let owner = self.callable_owner_v1().ok_or("[freeze:contract][static-execution/argument-read-owner]")?;
        if binding.owner() != owner { return Err("[freeze:contract][static-execution/argument-binding-owner]".into()); }
        self.callable_ledger.as_ref().ok_or("[freeze:contract][static-execution/argument-read-state]")?
            .borrow_mut().take_exact_lexical_read(owner, site.node(), binding)
            .map_err(|error| format!("[freeze:contract][static-execution/argument-read/{error:?}]"))
    }

    fn prepare_source_required_static_scalar_call_v1(&self) -> Result<Option<(std::rc::Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>, crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1)>, String> {
        let (Some(ledger), Some(owner)) = (&self.ordinary_new_claim_ledger, self.callable_owner_v1()) else { return Ok(None) };
        let site = self.current_source_site_v1().ok_or_else(|| "[freeze:contract][static-scalar/source-site]".to_owned())?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site));
        let Some(source) = ledger.source_required_static_scalar_call_v1(&site)? else { return Ok(None) };
        let observation = ledger.lexical_i64_call_source(&site)
            .ok_or("[freeze:contract][static-scalar/local-source]")?.clone();
        Ok(Some((source, observation)))
    }
    fn record_source_required_static_scalar_call_v1(&mut self, builder: &mut MirBuilder,
        source: std::rc::Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>, observations: Vec<(u32, crate::mir::resolved_semantics::SourceExprSiteV1, ValueId)>,
        projections: Vec<crate::mir::normal_callable_semantic_package::LexicalCallArgumentProjectionV1>,
        operand_bindings: Vec<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>,
        binding: (crate::mir::BasicBlockId, crate::mir::MirInstruction),
    ) -> Result<(), String> {
        let ledger = self.ordinary_new_claim_ledger.as_ref().ok_or_else(|| "[freeze:contract][static-scalar/ledger-missing]".to_owned())?;
        let function = builder.function_state.current_function.as_ref()
            .ok_or("[freeze:contract][static-scalar/no-function]")?;
        let crate::mir::MirInstruction::Call(call) = &binding.1 else {
            return Err("[freeze:contract][static-scalar/physical-call]".into());
        };
        if call.args.len() != observations.len() { return Err("[freeze:contract][static-scalar/physical-arity]".into()); }
        let transport = observations.iter().zip(&call.args).map(|((_, _, original), actual)|
            builder.function_state.local_ssa_correspondence.loan(function, *original, *actual)
                .ok_or_else(|| "[freeze:contract][static-scalar/argument-correspondence]".to_owned()))
            .collect::<Result<Vec<_>, _>>()?;
        ledger.record_source_required_static_scalar_call_v1(source, observations, projections, transport, operand_bindings, binding, function)
    }

    fn record_scalar_direct_call_v1(&mut self, builder: &mut MirBuilder,
        row: crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1,
        arguments: Vec<ValueId>, binding: (crate::mir::BasicBlockId, crate::mir::MirInstruction),
    ) -> Result<(), String> {
        let Some(ledger) = &self.ordinary_new_claim_ledger else { return Ok(()) };
        ledger.record_source_required_plain_call_v1(row, arguments, binding,
            builder.function_state.current_function.as_ref().ok_or_else(|| "[freeze:contract][direct-call/no-function]".to_owned())?)
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
        let source = crate::mir::normal_callable_semantic_package::CallPacketSourceV1::static_i64(
            original,
            publication,
            &ledger,
        )?;
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][borrowed-static/local-state-missing]".to_owned())?;
        let consume_receiver = self.lexical_self_receiver_locator_consumer_v1()?;
        let value = crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_local_lexical_source(
            builder, &mut state.borrow_mut(), &ledger, owner, &site, source, &consume_receiver,
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
        // The same source census also retains Static lifecycle calls. Their
        // exact source belongs to the existing Static publication owner;
        // absence of an Instance slot alone never authorizes another route.
        if ledger.selected_static_local_source_v1(&owned)?.is_some() { return Ok(None); }
        if !ledger.is_lifecycle_local_call_site_v1(&owned)? { return Ok(None); }
        let handle = ledger.handle_call_source(&owned).is_some();
        let scalar = ledger.lexical_i64_call_source(&owned).is_some();
        let nullable = ledger.nullable_call_source(&owned).is_some();
        let borrowed = ledger.borrowed_nullable_call_source_v1(&owned).is_some();
        let mixed = ledger.mixed_call_source_v1(&owned).is_some();
        if !handle && !scalar && !nullable && !borrowed && !mixed {
            return Err("[freeze:contract][lexical-instance-call/observation-missing]".into());
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
        let expected = if mixed {
            ledger.corroborate_mixed_call_v1(&row)?
        } else if borrowed {
            ledger.corroborate_borrowed_nullable_call_v1(&row)?
        } else if handle {
            crate::mir::instruction::InvokeCallResultKind::Handle
        } else if nullable {
            crate::mir::instruction::InvokeCallResultKind::NullableHandle
        } else {
            ledger.checked_local_scalar_call_result_v1(&row)?
        };
        if row.result() != Some(expected) {
            return Err("[freeze:contract][lexical-instance-call/result-mismatch]".to_owned());
        }
        self.consume_lexical_self_receiver_locator_v1(&row)?;
        let consume_receiver = self.lexical_self_receiver_locator_consumer_v1()?;
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][lexical-instance-call/state-missing]".to_owned())?;
        let mut state = state.borrow_mut();
        let value = if nullable || (handle && ledger.uses_object_packet_protocol_v1(&row)?) {
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
                &consume_receiver,
            )?
        };
        Ok(Some(value))
    }

    /// Emit a source-selected `me.m(..)` object call through the common
    /// lexical object packet. Selected Nullable membership without its
    /// complete packet freezes before the generic instance path.
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
        if ledger.nullable_call_source(&owned).is_some() {
            return Err("[freeze:contract][receiver-object/packet-unavailable]".to_owned());
        }
        Ok(None)
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
