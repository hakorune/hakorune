use super::*;
use crate::mir::builder::recursive_child_lowering_port::{
    DeclaredInstanceReceiverIngressV1, ScriptDirectStaticClaimCompletionErrorV1,
};

impl RecursiveChildLoweringPortV1 for RawInvocationChildPortV1<'_, '_> {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn prepare_borrowed_compare_source_v1(
        &mut self,
        operator: &crate::ast::BinaryOperator,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::BorrowedCompareSourceLoanV1>,
        String,
    > {
        let (Some(ledger), Some(owner)) =
            (&self.ordinary_new_claim_ledger, self.callable_owner_v1())
        else {
            return Ok(None);
        };
        if !ledger.has_borrowed_compare_source_v1(owner) {
            return Ok(None);
        }
        let node = self
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][borrowed-compare/source-site-missing]".to_owned())?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node),
        );
        let operator =
            match super::super::ops::converters::convert_binary_operator(operator.clone())? {
                super::super::ops::converters::BinaryOpType::Comparison(operator) => Some(operator),
                super::super::ops::converters::BinaryOpType::Arithmetic(_) => None,
            };
        ledger.prepare_borrowed_compare_source_v1(owner, &site, operator)
    }

    fn complete_borrowed_compare_source_v1(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::BorrowedCompareSourceLoanV1,
        children: (ValueId, ValueId),
        completed: &super::super::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<std::rc::Rc<crate::mir::normal_callable_semantic_package::BorrowedCompareMaterializationV1>, String> {
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][borrowed-compare/ledger-missing]".to_owned())?;
        if self.callable_owner_v1() != Some(loan.owner()) {
            return Err("[freeze:contract][borrowed-compare/owner-drift]".into());
        }
        let node = self
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][borrowed-compare/source-site-missing]".to_owned())?;
        if loan.site().site() != &crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node)
        {
            return Err("[freeze:contract][borrowed-compare/source-site-drift]".into());
        }
        ledger.record_borrowed_compare_v1(loan, children, completed)
    }

    fn prepare_compare_integer_literal_v1(
        &mut self,
        value: i64,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::BorrowedCompareIntegerLiteralLoanV1>, String> {
        let (Some(ledger), Some(owner)) = (&self.ordinary_new_claim_ledger, self.callable_owner_v1()) else {
            return Ok(None);
        };
        if !ledger.has_borrowed_compare_integer_literal_source_v1(owner) {
            return Ok(None);
        }
        let site = self.current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][borrowed-literal/source-site-missing]".to_owned())?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner, crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site),
        );
        ledger.prepare_borrowed_compare_integer_literal_v1(owner, &site, value)
    }

    fn complete_compare_integer_literal_v1(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::BorrowedCompareIntegerLiteralLoanV1,
        completed: &crate::mir::builder::emission::constant::CompletedConstV1,
    ) -> Result<std::rc::Rc<crate::mir::normal_callable_semantic_package::BorrowedCompareIntegerLiteralMaterializationV1>, String> {
        let ledger = self.ordinary_new_claim_ledger.as_ref()
            .ok_or_else(|| "[freeze:contract][borrowed-literal/ledger-missing]".to_owned())?;
        if self.callable_owner_v1() != Some(loan.owner()) {
            return Err("[freeze:contract][borrowed-literal/owner-drift]".into());
        }
        let node = self.current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][borrowed-literal/source-site-missing]".to_owned())?;
        if loan.site().site() != &crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node) {
            return Err("[freeze:contract][borrowed-literal/source-site-drift]".into());
        }
        ledger.record_borrowed_compare_integer_literal_v1(loan, completed)
    }

    fn take_construction_store_v1(&mut self) -> Result<Option<crate::mir::builder::normal_callable_semantic_lowering_state::construction::TakenConstructionStore>, String>{
        let Some(ledger) = self.callable_ledger.as_ref() else {
            return Ok(None);
        };
        let site = self
            .current_source_site_v1()
            .ok_or("[freeze:contract][construction-store/no-site]")?;
        ledger.borrow_mut().take_construction_store(&site)
    }

    fn emit_construction_store_v1(
        &mut self,
        builder: &mut MirBuilder,
        store: crate::mir::builder::normal_callable_semantic_lowering_state::construction::TakenConstructionStore,
        provider_value: Option<ValueId>,
    ) -> Result<ValueId, String> {
        let ledger = self
            .callable_ledger
            .as_ref()
            .ok_or("[freeze:contract][construction-store/no-ledger]")?;
        if let crate::mir::normal_callable_semantic_package::ConstructionStoreRhsV1::ProviderConstruction {
            caller,
            arguments,
            ..
        } = store.rhs()
        {
            let has_call_argument = arguments.iter().any(|argument| {
                matches!(
                    argument.kind(),
                    crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall { .. }
                )
            });
            if has_call_argument {
                let declarations = builder
                    .comp_ctx
                    .callable_declaration_catalog()
                    .map_err(|_| {
                        "[freeze:contract][construction-store/declarations-missing]"
                            .to_owned()
                    })?;
                for argument in arguments.iter() {
                    let crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall {
                        target,
                        ..
                    } = argument.kind()
                    else {
                        continue;
                    };
                    // The sole consumption boundary for the sealed
                    // `(caller, site)` publication row: exactly one
                    // `Selected` handoff may ride into the ledger, and
                    // its target must equal the claim's sealed target.
                    let take = self
                        .module_port
                        .take_static_result_publication_handoff(
                            declarations,
                            caller,
                            argument.site(),
                        )
                        .map_err(|error| {
                            format!(
                                "[freeze:contract][construction-store/publication-take/{error:?}]"
                            )
                        })?;
                    let crate::mir::callable_result_representation::StaticCallResultPublicationTakeV1::Selected(
                        handoff,
                    ) = take
                    else {
                        return Err(
                            "[freeze:contract][construction-store/publication-not-selected]"
                                .to_owned(),
                        );
                    };
                    if handoff.target() != target || handoff.site() != argument.site() {
                        return Err(
                            "[freeze:contract][construction-store/publication-relation-drift]"
                                .to_owned(),
                        );
                    }
                    ledger
                        .borrow_mut()
                        .install_source_static_result_publication(
                            argument.site(),
                            handoff,
                        )?;
                }
            }
        }
        ledger
            .borrow_mut()
            .emit_construction_store(builder, store, provider_value)
    }

    fn complete_construction_stores_v1(&mut self, builder: &MirBuilder) -> Result<(), String> {
        let Some(ledger) = self.callable_ledger.as_ref() else {
            return Ok(());
        };
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or("[freeze:contract][construction-store/no-function]")?;
        builder.function_state.checked_compare_reuse.require_same_entry(ledger)?;
        builder.function_state.checked_compare_reuse.verify(builder)?;
        let mut state = ledger.borrow_mut();
        state.complete_construction_stores(function)?;
        if let Some(news) = &self.ordinary_new_claim_ledger {
            news.verify_borrowed_compare_reuse_v1(state.owner(),
                builder.function_state.checked_compare_reuse.records())?;
            news.record_borrowed_compare_literal_consumers_v1(state.owner(), function,
                builder.function_state.checked_compare_reuse.literal_observations())?;
            news.record_borrowed_compare_consumers_v1(state.owner(), function,
                builder.function_state.checked_compare_reuse.observations())?;
            news.complete_new_emissions(state.owner(), function)?;
        }
        Ok(())
    }

    fn script_direct_static_claim_ingress_v1(
        &mut self,
        _box_name: &str,
        _method: &str,
        _argument_count: usize,
    ) -> Result<ScriptDirectStaticClaimIngressV1, String> {
        self.script_direct_static_claim_ingress_inner_v1(_box_name, _method, _argument_count)
    }

    fn take_script_direct_static_claim_v1(
        &mut self,
        box_name: &str,
        method: &str,
        _receiver: &ASTNode,
        arguments: &[ASTNode],
    ) -> Result<ScriptDirectStaticClaimTakeV1, String> {
        self.take_script_direct_static_claim_inner_v1(box_name, method, _receiver, arguments)
    }

    fn complete_script_direct_static_claim_v1(
        &mut self,
        claimed: ScriptDirectStaticClaimedRowV1,
    ) -> Result<(), ScriptDirectStaticClaimCompletionErrorV1> {
        self.complete_script_direct_static_claim_inner_v1(claimed)
    }

    fn take_declared_instance_receiver_value_v1(
        &mut self,
        builder: &MirBuilder,
    ) -> Result<DeclaredInstanceReceiverIngressV1, String> {
        self.take_declared_instance_receiver_value_inner_v1(builder)
    }

    fn lower_me_expression_v1(&mut self, builder: &mut MirBuilder) -> Result<ValueId, String> {
        if self.callable_ledger.is_some() {
            return self.read_callable_variable_v1();
        }
        crate::mir::builder::stmts::variable_stmt::build_me_expression(builder)
    }

    fn cleanup_exit_policy_v1(
        &self,
    ) -> crate::mir::builder::control_flow::cleanup::CleanupExitPolicyV1 {
        self.cleanup_exit_policy
    }

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        match self.current_source_context_v1() {
            Some(context) => {
                crate::mir::builder::raw_invocation_body::drive_located_invocation_body_v1(
                    builder, self, input, context,
                )
            }
            None => Err("[freeze:contract][raw-invocation/missing-root-body-receipt]".to_owned()),
        }
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        if self.active_source.is_none() {
            return Err(
                "[freeze:contract][raw-invocation/missing-statement-source-receipt]".to_owned(),
            );
        }
        if self.callable_ledger.is_some() && matches!(input, ASTNode::Local { .. }) {
            return self.lower_callable_local_v1(builder, input);
        }
        if self.semantic_ledger.is_some() {
            return match input {
                local @ ASTNode::Local { .. } => self.lower_script_local_v1(builder, local),
                nowait @ ASTNode::Nowait { .. } => self.lower_script_nowait_v1(builder, nowait),
                outbox @ ASTNode::Outbox { .. } => self.lower_script_outbox_v1(builder, outbox),
                other => crate::mir::builder::stmts::block_stmt::build_statement_with_port_v1(
                    builder, self, other,
                ),
            };
        }
        crate::mir::builder::stmts::block_stmt::build_statement_with_port_v1(builder, self, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        if self.active_source.is_none() {
            return Err(
                "[freeze:contract][raw-invocation/missing-expression-source-receipt]".to_owned(),
            );
        }
        if self.semantic_ledger.is_some() && matches!(input, ASTNode::Return { .. }) {
            return self.lower_script_return_v1(builder, input);
        }
        if self.semantic_ledger.is_some() && matches!(input, ASTNode::Local { .. }) {
            return self.lower_script_local_v1(builder, input);
        }
        if self.callable_ledger.is_some() && matches!(input, ASTNode::Local { .. }) {
            return self.lower_callable_local_v1(builder, input);
        }
        if self.semantic_ledger.is_some() && matches!(input, ASTNode::Nowait { .. }) {
            return self.lower_script_nowait_v1(builder, input);
        }
        if self.semantic_ledger.is_some() && matches!(input, ASTNode::Outbox { .. }) {
            return self.lower_script_outbox_v1(builder, input);
        }
        if self.semantic_ledger.is_some()
            && (matches!(
                &input,
                ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::Variable { .. })
            ) || matches!(&input, ASTNode::GroupedAssignmentExpr { .. }))
        {
            return self.lower_script_binding_rebind_v1(builder, input);
        }
        if self.callable_ledger.is_some()
            && (matches!(
                &input,
                ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::Variable { .. })
            ) || matches!(&input, ASTNode::GroupedAssignmentExpr { .. }))
        {
            return self.lower_callable_binding_rebind_v1(builder, input);
        }
        if self.callable_ledger.is_some() && matches!(input, ASTNode::MapLiteral { .. }) {
            return self.lower_callable_map_v1(builder);
        }
        if self.callable_ledger.is_some() && matches!(input, ASTNode::Variable { .. }) {
            return self.read_callable_variable_v1();
        }
        if let (Some(ledger), ASTNode::Variable { .. }) = (&self.semantic_ledger, &input) {
            let site = self
                .current_source_context_v1()
                .and_then(|context| context.site().cloned())
                .ok_or_else(|| "[freeze:contract][script-lexical/variable-site]".to_owned())?;
            let binding = ledger
                .borrow()
                .variable_binding(&site)
                .ok_or_else(|| "[freeze:contract][script-lexical/variable-binding]".to_owned())?;
            return ledger
                .borrow()
                .value(binding)
                .ok_or_else(|| "[freeze:contract][script-lexical/variable-value]".to_owned());
        }
        lower_raw_expression_with_recursion_guard_v1(builder, self, input)
    }

    fn try_lower_map_read_method_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        receiver: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
        receiver_source: PreparedRawChildSourceV1,
    ) -> Result<Option<ValueId>, String> {
        RawInvocationChildPortV1::try_lower_map_read_method_call_v1(
            self,
            builder,
            receiver,
            method,
            arguments,
            receiver_source,
        )
    }

    fn prepare_expression_child_source_v1(
        &self,
        parent: &ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        let context = self
            .current_source_context_v1()
            .ok_or_else(|| {
                "[freeze:contract][raw-invocation/missing-parent-expression-receipt]".to_owned()
            })?
            .child_expression(parent, role)?;
        Ok(PreparedRawChildSourceV1::Exact(context))
    }

    fn prepare_body_child_source_v1(
        &self,
        parent: &ASTNode,
        role: BodyChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        let context = self
            .current_source_context_v1()
            .ok_or_else(|| {
                "[freeze:contract][raw-invocation/missing-parent-body-receipt]".to_owned()
            })?
            .child_body(parent, role)?;
        Ok(PreparedRawChildSourceV1::Exact(context))
    }

    fn prepare_body_statement_source_v1(
        &self,
        statement: &ASTNode,
        index: usize,
    ) -> Result<PreparedRawChildSourceV1, String> {
        let context = self
            .current_source_context_v1()
            .ok_or_else(|| {
                "[freeze:contract][raw-invocation/missing-parent-statement-receipt]".to_owned()
            })?
            .child_statement(statement, index)?;
        Ok(PreparedRawChildSourceV1::Exact(context))
    }

    fn with_prepared_child_source_v1<R>(
        &mut self,
        source: PreparedRawChildSourceV1,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        match source {
            PreparedRawChildSourceV1::Preserve => execute(self),
            PreparedRawChildSourceV1::Exact(source) => {
                let parent = self.active_source.replace(source);
                let result = execute(self);
                self.active_source = parent;
                result
            }
        }
    }

    fn with_call_argument_source_v1<R>(
        &mut self,
        index: usize,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let source = self
            .active_source
            .as_ref()
            .map(|source| source.child_call_argument(index));
        let parent = source.and_then(|source| self.active_source.replace(source));
        let result = execute(self);
        self.active_source = parent;
        result
    }
}
