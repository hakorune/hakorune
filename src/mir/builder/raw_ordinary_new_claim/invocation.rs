//! Invocation-backed ordinary-New emission; the parent owns the port contract.
use super::*;

impl RawOrdinaryNewClaimPortV1 for super::super::RawInvocationChildPortV1<'_, '_> {
    fn try_emit_unit_discard(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
        method: &str,
        arity: usize,
    ) -> Result<bool, String> {
        let (Some(ledger), Some(node)) = (
            self.ordinary_new_claim_ledger.as_ref(),
            self.current_source_site_v1(),
        ) else {
            return Ok(false);
        };
        let owner = self
            .callable_owner_v1()
            .ok_or("[freeze:contract][unit-call/owner-missing]")?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node),
        );
        let constructor = match &self.callable_ledger {
            Some(state) => state.borrow().has_next_constructor_call_v1(&site)?,
            None => false,
        };
        if constructor {
            self.check_new_emission_scope(&site)?;
            let row = ledger
                .take_lexical_instance_call(owner, site.site())?
                .ok_or("[freeze:contract][constructor-unit/disposition-unavailable]")?;
            if row.target().name() != method || arity != 0 || !row.argument_sites().is_empty() {
                return Err("[freeze:contract][constructor-unit/syntax-drift]".into());
            }
            self.consume_lexical_self_receiver_locator_v1(&row)?;
            let state = self
                .callable_ledger
                .as_ref()
                .ok_or("[freeze:contract][constructor-unit/state-missing]")?;
            state
                .borrow_mut()
                .emit_constructor_unit_call_v1(builder, ledger, row)?;
            return Ok(true);
        }
        let Some(observation) = ledger.unit_discard_source_v1(&site) else {
            return Ok(false);
        };
        let original = observation
            .unit_source()
            .ok_or("[freeze:contract][unit-call/original-missing]")?;
        if method != original.target().name()
            || arity != original.target().arity() as usize
            || original.argument_sites().len() != arity
            || observation.arguments().len() != arity
        {
            return Err("[freeze:contract][unit-call/syntax-drift]".into());
        }
        self.check_new_emission_scope(&site)?;
        let row = ledger
            .take_lexical_instance_call(owner, site.site())?
            .ok_or("[freeze:contract][unit-call/disposition-unavailable]")?;
        self.consume_lexical_self_receiver_locator_v1(&row)?;
        let consume_receiver = self.lexical_self_receiver_locator_consumer_v1()?;
        let state = std::rc::Rc::clone(
            self.callable_ledger
                .as_ref()
                .ok_or("[freeze:contract][unit-call/state-missing]")?,
        );
        let ledger = std::rc::Rc::clone(ledger);
        let port = std::cell::RefCell::new(self);
        let issue_static = |builder: &mut crate::mir::MirBuilder,
            original: &std::rc::Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>,
            frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
            observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1| {
            crate::mir::builder::static_result_publication_ingress::take_expression_static_packet_v1(
                &mut port.borrow_mut(), builder, original, frame, observation, &ledger)
        };
        crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_unit_discard(
            builder,
            &mut state.borrow_mut(),
            &ledger,
            row,
            &consume_receiver,
            Some(&issue_static),
        )?;
        Ok(true)
    }

    fn try_emit_owned_array_append(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
        method: &str,
        arity: usize,
    ) -> Result<bool, String> {
        self.emit_owned_array_append_v1(builder, method, arity, None, &[])
            .map(|packet| packet.is_some())
    }
    fn emit_terminal_i64_call_exit(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        terminal_call::emit_terminal_i64_call_exit(self, builder)
    }
    fn prepare_terminal_field_read(
        &mut self,
        object: crate::ast::ASTNode,
        field: &str,
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
        if ledger.reserve_result_path_field_v1(&site, &resolve)? {
            return Ok(Some(PreparedRawFieldReadV1::result_path(
                object,
                site,
                ledger.clone(),
            )));
        }
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
        if let Some(input) = self.source_input {
            ledger.stage_source_array_receiver_v1(input, &site, field)?;
        }
        let checked = ledger.local_field_read_requires_checked_v1(&site);
        let mut original_read = None;
        let receiver_site =
            crate::mir::resolved_semantics::SourcePathV1::from_node(site.site().node())
                .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Receiver)
                .expr();
        if let Some((base, source)) = ledger.take_local_field_read(&site, |binding| {
            if !checked {
                return resolve(binding);
            }
            let read = state
                .borrow_mut()
                .take_exact_lexical_read(owner, receiver_site.node(), binding)
                .map_err(|issue| issue.to_string())?;
            let value = read
                .value_for(owner, receiver_site.node(), binding)
                .map_err(|issue| issue.to_string())?;
            original_read = Some(std::rc::Rc::new(read));
            Ok(value)
        })? {
            let field = source.field();
            let claim = if checked {
                PreparedExactFieldReadClaimV1::CheckedLocal {
                    source,
                    read: original_read
                        .ok_or("[freeze:contract][ordinary-field-read/checked-receiver-missing]")?,
                    state: std::rc::Rc::clone(state),
                }
            } else {
                PreparedExactFieldReadClaimV1::Local(source)
            };
            return Ok(Some(PreparedRawFieldReadV1::exact_object(
                object,
                base,
                field,
                site,
                ledger.clone(),
                claim,
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

    fn emit_terminal_scalar_literal_return(
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
        let Some(value) = ledger.prepare_terminal_scalar_literal_return(owner, &site)? else {
            return Ok(None);
        };
        let emitted = match value {
            crate::mir::ConstValue::Integer(value) => {
                crate::mir::builder::emission::constant::emit_integer(builder, value)?
            }
            crate::mir::ConstValue::Bool(value) => {
                crate::mir::builder::emission::constant::emit_bool(builder, value)?
            }
            _ => return Err("[freeze:contract][ordinary-literal/non-scalar-source]".into()),
        };
        ledger.record_terminal_scalar_literal_return(owner, &site, emitted)?;
        let block = builder
            .function_state
            .current_block
            .ok_or("[ordinary-literal/block-missing]")?;
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or("[ordinary-literal/function-missing]")?;
        let instruction = function
            .blocks
            .get(&block)
            .and_then(|block| block.instructions.last())
            .ok_or("[ordinary-literal/producer-missing]")?
            .clone();
        ledger.record_terminal_literal_producer_v1(
            owner,
            &crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site),
            (block, instruction),
            function,
        )?;
        Ok(Some(emitted))
    }

    fn complete_terminal_scalar_literal_return(
        &mut self,
        builder: &crate::mir::MirBuilder,
    ) -> Result<(), String> {
        let Some(ledger) = &self.ordinary_new_claim_ledger else {
            return Ok(());
        };
        let owner = self
            .callable_owner_v1()
            .ok_or("[ordinary-literal/owner-missing]")?;
        let site = self
            .current_source_site_v1()
            .ok_or("[ordinary-literal/site-missing]")?;
        let block = builder
            .function_state
            .current_block
            .ok_or("[ordinary-literal/block-missing]")?;
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or("[ordinary-literal/function-missing]")?;
        let returned = function
            .blocks
            .get(&block)
            .and_then(|block| block.terminator.as_ref())
            .ok_or("[ordinary-literal/return-missing]")?
            .clone();
        ledger.complete_terminal_literal_return_v1(
            owner,
            &crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site),
            &[(block, returned)],
        )
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
        if let Some(source) = ledger.stored_map_get_return_source_v1(
            owner,
            &crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone()),
        )? {
            return crate::mir::builder::ordinary_new_admission::selected::terminal_map_get::emit(
                builder,
                &mut state.borrow_mut(),
                ledger,
                source,
            )
            .map(Some);
        }
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
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .expect("checked ledger")
            .clone();
        let state = self
            .callable_ledger
            .as_ref()
            .expect("checked state")
            .clone();
        let consume = self.lexical_self_receiver_locator_consumer_v1()?;
        let port = std::cell::RefCell::new(self);
        let issue_static = |builder: &mut crate::mir::MirBuilder,
            original: &std::rc::Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>,
            frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
            observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1| {
            crate::mir::builder::static_result_publication_ingress::take_expression_static_packet_v1(
                &mut port.borrow_mut(), builder, original, frame, observation, &ledger)
        };
        let result = crate::mir::builder::ordinary_new_admission::selected::emit(
            builder,
            &mut state.borrow_mut(),
            &ledger,
            claim,
            &consume,
            Some(&issue_static),
        );
        result
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
            result_claim_trace::missing("ledger-or-site-unavailable");
            return Ok(None);
        };
        let Some(site) = self.current_source_site_v1() else {
            result_claim_trace::missing("ledger-or-site-unavailable");
            return Ok(None);
        };
        let Some(owner) = self.callable_owner_v1() else {
            return Err("[freeze:contract][raw-ordinary-new/claim-owner-missing]".to_owned());
        };
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site),
        );
        let result = ledger.try_take_result(&site, class, argument_count);
        result_claim_trace::take(&site, class, argument_count, &result);
        result
            .map_err(|error| format!("[freeze:contract][raw-ordinary-new/result-claim] {error:?}"))
    }

    fn prepare_result_new_emission(
        &mut self,
        _builder: &crate::mir::MirBuilder,
        claim: &crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1,
    ) -> Result<bool, String> {
        self.check_new_emission_scope(claim.site())?;
        let result = self
            .ordinary_new_claim_ledger
            .as_ref()
            .expect("checked ledger")
            .prepare_result_new_emission(claim);
        result_claim_trace::prepare(claim, &result);
        result
    }

    fn emit_result_new_claim(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
        claim: crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1,
    ) -> Result<crate::mir::ValueId, String> {
        self.check_new_emission_scope(claim.site())?;
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .expect("checked ledger")
            .clone();
        let state = self
            .callable_ledger
            .as_ref()
            .expect("checked state")
            .clone();
        let consume = self.lexical_self_receiver_locator_consumer_v1()?;
        let port = std::cell::RefCell::new(self);
        let issue_static = |builder: &mut crate::mir::MirBuilder,
            original: &std::rc::Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>,
            frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
            observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1| {
            crate::mir::builder::static_result_publication_ingress::take_expression_static_packet_v1(
                &mut port.borrow_mut(), builder, original, frame, observation, &ledger)
        };
        let result = crate::mir::builder::ordinary_new_admission::selected::emit_result(
            builder,
            &mut state.borrow_mut(),
            &ledger,
            claim,
            &consume,
            Some(&issue_static),
        );
        result
    }

    fn prepare_owned_array_construction(
        &mut self,
        class: &str,
        argument_count: usize,
        field_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::PreparedOwnedArrayConstructionV1>,
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
            .ok_or("[freeze:contract][owned-array/owner-missing]")?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node),
        );
        if !ledger.has_owned_array_construction(&site) {
            return Ok(None);
        }
        let source = self.owned_array_source;
        if let Some(source) = source {
            source.require_same_ledger_v1(ledger)?;
        }
        self.check_new_emission_scope(&site)?;
        if crate::runtime::CoreBoxId::from_name(class) != Some(crate::runtime::CoreBoxId::Array)
            || argument_count != 0
            || field_count != 0
        {
            return Err("[freeze:contract][owned-array/construction-syntax-drift]".into());
        }
        let relation = self
            .callable_ledger
            .as_ref()
            .ok_or("[freeze:contract][owned-array/callable-state-missing]")?
            .borrow()
            .expression_initializer(site.site().node())?;
        let typed = source
            .map(|source| source.prepare_construction_v1(&site, relation.clone()))
            .transpose()?
            .flatten();
        match typed {
            Some(prepared) => Ok(Some(prepared)),
            None => match ledger.prepare_dynamic_carrier_array_construction_v1(&site, relation)? {
                Some(prepared) => Ok(Some(prepared)),
                None if ledger.has_owned_array_construction(&site) => {
                    Err("[freeze:contract][owned-array/construction-source-missing]".into())
                }
                None => Ok(None),
            },
        }
    }

    fn emit_owned_array_construction(
        &mut self,
        builder: &mut crate::mir::MirBuilder,
        source: crate::mir::normal_callable_semantic_package::PreparedOwnedArrayConstructionV1,
    ) -> Result<crate::mir::ValueId, String> {
        self.check_new_emission_scope(&source.site())?;
        crate::mir::builder::ordinary_new_admission::selected::owned_array::emit_construction(
            builder,
            &mut self
                .callable_ledger
                .as_ref()
                .expect("checked callable state")
                .borrow_mut(),
            self.ordinary_new_claim_ledger
                .as_ref()
                .expect("checked ledger"),
            source,
        )
    }

    fn validate_named_array_construction_route(&self, named_route: bool) -> Result<(), String> {
        let (Some(ledger), Some(site)) =
            (self.callable_ledger.as_ref(), self.current_source_site_v1())
        else {
            return Ok(());
        };
        let site = crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site);
        if !named_route
            && (ledger.borrow().requires_named_array_allocation(&site)
                || self
                    .ordinary_new_claim_ledger
                    .as_ref()
                    .is_some_and(|claims| {
                        claims.has_owned_array_construction(
                            &crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                ledger.borrow().owner(),
                                site.clone(),
                            ),
                        )
                    }))
        {
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

impl super::super::RawInvocationChildPortV1<'_, '_> {
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
