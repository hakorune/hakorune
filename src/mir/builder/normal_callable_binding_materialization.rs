//! Exact root-owner BindingRef materialization for selected callable bodies.
//! Direct child Lambda captures consume the forest's ordered BindingRef
//! receipt through this root ledger. Lambda bodies and descendant-owner
//! materialization remain outside this authority.

use crate::ast::ASTNode;
use crate::mir::builder::stmts::{drive_local_statement_with_placement_v1, RawLegacyLocalInputV1};
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::{MirBuilder, ValueId};

use super::normal_callable_binding_materialization_port::{
    CallableBindingMaterializationPortV1, CallableEntryShapeV1,
};
use super::raw_invocation_source_transport::RawSourceTransportPortV1;
use super::recursive_child_lowering::{
    lower_raw_expression_with_recursion_guard_v1, RawInvocationChildPortV1,
};

impl CallableBindingMaterializationPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn callable_result_owner_v1(&self) -> Result<Option<crate::mir::resolved_semantics::FunctionOwnerIdV1>, String> {
        let owner = self.callable_owner_v1();
        if self.ordinary_new_claim_ledger.is_some() && owner.is_none() {
            return Err(freeze("result-source-owner-missing"));
        }
        Ok(owner)
    }
    fn prepare_callable_result_source_v1(
        &self,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::CallableResultSourceV1>, String> {
        let Some(owner) = self.callable_result_owner_v1()? else { return Ok(None); };
        let ledger = self.ordinary_new_claim_ledger.as_ref()
            .ok_or_else(|| freeze("result-source-ledger-missing"))?;
        let source = ledger.callable_result_source_v1(owner)?;
        if let Some(scalar) = source.as_ref().and_then(|source| source.scalar()) { ledger.stage_result_field_paths_v1(scalar)?; }
        Ok(source)
    }
    fn prepare_implicit_callable_end_v1(
        &self,
    ) -> Result<Option<super::port_aware_function_draft_impl::ImplicitCallableEndV1>, String> {
        let Some(owner) = self.callable_result_owner_v1()? else { return Ok(None); };
        let ledger = self.ordinary_new_claim_ledger.as_ref()
            .ok_or_else(|| freeze("implicit-end-ledger-missing"))?;
        let Some(completion) = ledger.implicit_unit_completion_source_v1(owner)? else { return Ok(None); };
        let state = self.callable_ledger.as_ref()
            .ok_or_else(|| freeze("implicit-end-state-missing"))?;
        super::port_aware_function_draft_impl::ImplicitCallableEndV1::new(
            owner, completion, ledger.clone(), state.clone(),
        ).map(Some)
    }
    fn adopt_callable_entry_values_v1(
        &mut self,
        builder: &mut MirBuilder,
        shape: CallableEntryShapeV1,
    ) -> Result<(), String> {
        let Some(ledger) = self.callable_ledger.clone() else {
            return Ok(());
        };
        let has_physical_formals = shape.has_physical_formals();
        let values = shape.prepare_values(builder)?;
        let exact_usize = ledger
            .borrow()
            .prepare_exact_usize_entry_projection(&values, builder)?;
        let carriers = ledger
            .borrow()
            .prepare_borrowed_entry_carriers_for_shape(&values, builder, has_physical_formals)?;
        ledger.borrow_mut().install_entry_values(&values)?;
        let declared = carriers
            .as_ref()
            .map(|(_, values)| values.as_slice())
            .unwrap_or(&[]);
        for (index, value) in exact_usize.into_iter().chain(declared.iter().copied()) {
            builder
                .function_state
                .current_function
                .as_mut()
                .unwrap()
                .signature
                .params[index] = crate::mir::MirType::Integer;
            builder
                .function_state
                .type_ctx
                .value_types
                .insert(value, crate::mir::MirType::Integer);
        }
        if let Some((carriers, _)) = carriers {
            // All checks precede entry installation; metadata commits last.
            builder
                .function_state
                .current_function
                .as_mut()
                .unwrap()
                .metadata
                .physical_param_carriers = Some(carriers);
        }
        let owner = ledger.borrow().checked_compare_entry_owner_v1()?;
        if self
            .ordinary_new_claim_ledger
            .as_ref()
            .is_some_and(|news| news.has_borrowed_compare_source_v1(owner))
        {
            builder
                .function_state
                .checked_compare_reuse
                .adopt_entry(ledger)?;
        }
        Ok(())
    }
}

impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn lower_callable_binding_rebind_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        if let (Some(claims), Some(state)) = (&self.ordinary_new_claim_ledger, &self.callable_ledger) {
            let site = self.current_callable_site_v1("rebind-expression/statement-site")?;
            let frame = claims.rebind_expression_frame_v1(state.borrow().owner(), &site)?;
            if let Some(frame) = frame {
                let consume = self.lexical_self_receiver_locator_consumer_v1()?;
                return super::ordinary_new_admission::selected::binding_rebind_expression::emit(
                    builder, &mut state.borrow_mut(), claims, frame, &consume);
            }
        }
        let role = match &input {
            ASTNode::Assignment { target, .. }
                if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
            {
                ExprChildRoleV1::AssignmentTarget
            }
            ASTNode::CompoundAssignment { target, .. }
                if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
            {
                ExprChildRoleV1::CompoundAssignmentTarget
            }
            ASTNode::GroupedAssignmentExpr { .. } => ExprChildRoleV1::GroupedAssignmentTarget,
            _ => return Err(freeze("rebind-source-drift")),
        };
        let target_site = self
            .current_source_context_v1()
            .ok_or_else(|| freeze("rebind-parent-site"))?
            .child_expression(&input, role)?
            .site()
            .cloned()
            .ok_or_else(|| freeze("rebind-target-site"))?;
        let value = lower_raw_expression_with_recursion_guard_v1(builder, self, input)?;
        self.record_callable_rebind_v1(&target_site, value)?;
        Ok(value)
    }
    pub(in crate::mir::builder) fn emit_checked_local_initializer_v1(&mut self, builder: &mut MirBuilder,
        source: &super::raw_structured_child_scope::PreparedRawChildSourceV1, ordinal: u32,
    ) -> Result<Option<super::ordinary_new_admission::selected::initializer_expression::PendingInitializerExpressionV1>, String> {
        let (Some(claims), Some(state)) = (self.ordinary_new_claim_ledger.clone(), self.callable_ledger.clone()) else { return Ok(None) };
        let site = self.current_callable_site_v1("initializer-expression/statement-site")?;
        let owner = state.borrow().owner();
        if let Some(frame) = claims.stored_map_get_local_source_v1(owner, &site)? {
            let super::raw_structured_child_scope::PreparedRawChildSourceV1::Exact(context) = source else {
                return Err(freeze("initializer-expression/exact-source-required"));
            };
            let relation = state.borrow().expression_initializer(frame.site().site().node())?;
            if context.site() != Some(frame.site().site().node()) || relation.binding() != frame.binding()
                || relation.declaration_site() != frame.declaration()
                || !matches!(frame.declaration(), crate::mir::resolved_semantics::SourceBindingSiteV1::Local { ordinal: original, .. } if *original == ordinal) {
                return Err(freeze("initializer-expression/source-drift"));
            }
            if relation.declared_type_name().is_some() {
                return Err(freeze("initializer-expression/local-contract-receipt-pending"));
            }
            let pending = super::ordinary_new_admission::selected::terminal_map_get::emit_local(
                builder, &mut state.borrow_mut(), &claims, frame, ordinal)?;
            return Ok(Some(super::ordinary_new_admission::selected::initializer_expression::PendingInitializerExpressionV1::StoredMapGet(pending)));
        }
        let Some(frame) = claims.initializer_expression_frame_v1(state.borrow().owner(), &site)? else { return Ok(None) };
        let super::raw_structured_child_scope::PreparedRawChildSourceV1::Exact(context) = source else {
            return Err(freeze("initializer-expression/exact-source-required"));
        };
        if context.site() != Some(frame.source().nodes()[0].site().node())
            || !matches!(frame.source().root().initializer().map(|row| row.declaration_site()),
                Some(crate::mir::resolved_semantics::SourceBindingSiteV1::Local { ordinal: source_ordinal, .. }) if *source_ordinal == ordinal) {
            return Err(freeze("initializer-expression/source-drift"));
        }
        let consume = self.lexical_self_receiver_locator_consumer_v1()?;
        let port = std::cell::RefCell::new(self);
        let issue_static = |builder: &mut MirBuilder,
            original: &std::rc::Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>,
            frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
            observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1| {
            super::static_result_publication_ingress::take_expression_static_packet_v1(
                &mut port.borrow_mut(), builder, original, frame, observation, &claims)
        };
        let result = super::ordinary_new_admission::selected::initializer_expression::emit(builder, &mut state.borrow_mut(), &claims,
            frame, ordinal, &consume, Some(&issue_static)).map(Some);
        result
    }
    pub(super) fn lower_callable_local_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        self.lower_callable_local_with_receipt_v2(builder, input)
            .map(|completed| completed.result())
    }

    /// The sole Local writer and publication tail return their real receipt.
    /// Checked initializer leases have been consumed before the caller observes
    /// it; source-scheduled callers must not recreate or republish those leases.
    pub(in crate::mir::builder) fn lower_callable_local_with_receipt_v2(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<crate::mir::builder::stmts::CompletedLocalStatementV1, String> {
        let ledger = self
            .callable_ledger
            .clone()
            .expect("callable local lowering requires its scoped ledger");
        let site = self.current_callable_site_v1("local-site")?;
        let claims_for_placement = self.ordinary_new_claim_ledger.clone();
        let mut completed = drive_local_statement_with_placement_v1(
            builder,
            self,
            RawLegacyLocalInputV1::with_initializer_observer(
                input,
                self.local_initializer_observation_sink(),
            ),
            |ordinal, value| match &claims_for_placement {
                Some(claims) => ledger
                    .borrow()
                    .local_placement(&site, ordinal, value, claims),
                None => Ok(crate::mir::builder::stmts::variable_stmt::LocalValuePlacement::Copy),
            },
        )?;
        ledger
            .borrow_mut()
            .record_completed_local(&site, &completed)?;
        if let Some(claims) = self.ordinary_new_claim_ledger.as_ref() {
            ledger
                .borrow()
                .record_completed_ordinary_new_local(&site, &completed, claims)?;
        }
        let pending = completed.take_checked_initializers();
        if let Some(claims) = self.ordinary_new_claim_ledger.as_ref() {
            for emission in pending {
                let receipt = completed.bindings().iter().find(|row| row.ordinal() == emission.ordinal())
                    .ok_or_else(|| freeze("initializer-expression/completed-ordinal"))?;
                emission.publish(receipt, claims, builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?;
            }
        } else if !pending.is_empty() { return Err(freeze("initializer-expression/claim-ledger-missing")); }
        Ok(completed)
    }

    pub(super) fn lower_callable_map_v1(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<ValueId, String> {
        use crate::mir::resolved_semantics::home_new_prefix::MapDestinationV1;
        let state = self
            .callable_ledger
            .clone()
            .ok_or_else(|| freeze("map-no-state"))?;
        let claims = self
            .ordinary_new_claim_ledger
            .clone()
            .ok_or_else(|| freeze("map-no-claims"))?;
        let site = self.current_callable_site_v1("map-site")?;
        let mut state = state.borrow_mut();
        let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            state.owner(),
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site.clone()),
        );
        // The sealed flow destination selects the consumer: a local
        // initializer keeps the install path, `return %{...}` transfers the
        // lease through the function boundary. Other destinations (call
        // arguments, nested slots, containment) stay unadmitted here.
        match claims.map_flow(&owned)?.destination() {
            MapDestinationV1::LocalBinding(_) => {
                let relation = state.expression_initializer(&site)?;
                crate::mir::builder::ordinary_new_admission::selected::map::emit(
                    builder, &mut state, &claims, &owned, &relation,
                )
            }
            MapDestinationV1::ReturnBoundary(_) => {
                crate::mir::builder::ordinary_new_admission::selected::map::emit_return(
                    builder, &mut state, &claims, &owned,
                )
            }
            MapDestinationV1::EntrySlot { .. }
            | MapDestinationV1::CallArgument { .. }
            | MapDestinationV1::ContainedIn { .. } => Err(freeze("map-destination-unsupported")),
        }
    }

    pub(super) fn read_callable_variable_v1(&self) -> Result<ValueId, String> {
        let ledger = self
            .callable_ledger
            .as_ref()
            .expect("callable variable lowering requires its scoped ledger");
        let site = self.current_callable_site_v1("variable-site")?;
        ledger.borrow_mut().read_variable(&site)
    }

    pub(super) fn record_callable_rebind_v1(
        &mut self,
        site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        value: ValueId,
    ) -> Result<(), String> {
        self.callable_ledger
            .as_ref()
            .expect("callable rebind lowering requires its scoped ledger")
            .borrow_mut()
            .rebind(site, value)
    }

    fn current_callable_site_v1(
        &self,
        reason: &str,
    ) -> Result<crate::mir::resolved_semantics::SourceNodeSiteV1, String> {
        self.current_source_context_v1()
            .and_then(|context| context.site().cloned())
            .ok_or_else(|| format!("[freeze:contract][callable-semantic-lowering/{reason}]"))
    }
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][callable-semantic-lowering/{reason}]")
}
