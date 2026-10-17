use super::*;
use crate::mir::builder::recursive_child_lowering_port::{
    DeclaredInstanceReceiverIngressV1, ScriptDirectStaticClaimCompletionErrorV1,
};

impl RecursiveChildLoweringPortV1 for RawInvocationChildPortV1<'_, '_> {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn observe_borrowed_field_write_guard_binary_v1(&mut self, builder: &mut MirBuilder,
        children: (ValueId, ValueId), completed: &super::super::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<(), String> { self.observe_write_guard_binary_v1(builder, children, completed) }
    fn prepare_borrowed_field_write_guard_observer_v1(&mut self)
        -> Result<Option<Box<dyn super::super::if_form::IfFormPhysicalObserverV1>>, String> {
        self.write_guard_observer_v1()
    }

    fn prepare_borrowed_field_write_v1(&mut self) -> Result<Option<crate::mir::normal_callable_semantic_package::FieldWriteSourceLoanV1>, String> {
        self.borrowed_field_write_source_v1()
    }
    fn read_borrowed_field_write_operand_v1(&mut self, builder: &mut MirBuilder, source: &crate::mir::normal_callable_semantic_package::FieldWriteSourceLoanV1,
        receiver: bool, input: ASTNode,
    ) -> Result<crate::mir::normal_callable_semantic_package::FieldWriteValueV1, String> { self.read_borrowed_field_write_source_v1(builder, source, receiver, input) }
    fn emit_borrowed_field_write_v1(&mut self, builder: &mut MirBuilder,
        source: crate::mir::normal_callable_semantic_package::FieldWriteSourceLoanV1, field: &str, reads: [crate::mir::normal_callable_semantic_package::FieldWriteValueV1; 2],
    ) -> Result<(), String> { self.emit_borrowed_field_write_source_v1(builder, source, field, reads) }

    fn prepare_borrowed_array_set_v1(&mut self, method: &str, arity: usize)
        -> Result<Option<crate::mir::normal_callable_semantic_package::BorrowedArraySetSourceLoanV1>, String> {
        self.borrowed_array_set_source_v1(method, arity)
    }
    fn complete_borrowed_array_set_v1(&mut self, builder: &mut MirBuilder,
        source: crate::mir::normal_callable_semantic_package::BorrowedArraySetSourceLoanV1,
        children: [ValueId; 3], write: crate::mir::builder::CompletedArrayElementWriteV1,
    ) -> Result<(), String> { self.complete_array_set_v1(builder, source, children, write) }

    fn prepare_field_array_read_v1(&mut self, method: &str, arity: usize) -> Result<Option<std::rc::Rc<crate::mir::normal_callable_semantic_package::FieldArrayReadRecipeV1>>, String> {
        let (Some(ledger), Some(owner)) = (&self.ordinary_new_claim_ledger, self.callable_owner_v1()) else { return Ok(None) };
        let Some(node) = self.current_source_site_v1() else { return Ok(None) };
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node));
        let recipe = ledger.field_array_read_recipe_v1(owner, &site)?;
        if recipe.is_some() && crate::boxes::array::ArrayMethodId::from_name_and_arity(method, arity) != Some(crate::boxes::array::ArrayMethodId::Get) {
            return Err("[freeze:contract][named-array/read-call-shape]".into());
        }
        Ok(recipe)
    }
    fn complete_field_array_read_v1(&mut self, recipe: std::rc::Rc<crate::mir::normal_callable_semantic_package::FieldArrayReadRecipeV1>, read: crate::mir::ArrayReadSiteId, receiver: ValueId, index: ValueId, dst: ValueId) -> Result<(), String> {
        let node = self.current_source_site_v1().ok_or_else(|| "[freeze:contract][named-array/read-source-missing]".to_owned())?;
        if self.callable_owner_v1() != Some(recipe.owner()) || recipe.call_site() != &crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node) { return Err("[freeze:contract][named-array/read-source-drift]".into()); }
        self.callable_ledger.as_ref().ok_or_else(|| "[freeze:contract][named-array/read-state-missing]".to_owned())?.borrow_mut().record_field_array_read_v1(recipe, read, receiver, index, dst)
    }

    fn prepare_checked_bool_condition_v1(&mut self) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1>>, String> {
        let (Some(ledger), Some(owner)) = (&self.ordinary_new_claim_ledger, self.callable_owner_v1()) else { return Ok(None) };
        let node = self.current_source_site_v1().ok_or_else(|| "[freeze:contract][bool-condition/statement-site-missing]".to_owned())?;
        ledger.bool_condition_frame_v1(owner, &crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(node))
    }
    fn lower_checked_bool_condition_v1(&mut self, builder: &mut MirBuilder,
        frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1>, condition: ASTNode,
    ) -> Result<super::super::ordinary_new_admission::selected::bool_condition::LoweredBoolConditionV1, String> {
        self.lower_whole_bool_condition_v1(builder, frame, condition)
    }
    fn complete_ordinary_binary_expression_v1(&mut self, completed: &super::super::ops::CompletedOrdinaryBinaryV1) -> Result<(), String> {
        self.observe_bool_binary_v1(completed)?;
        self.observe_field_write_binary_v1(completed)
    }
    fn complete_short_circuit_expression_v1(&mut self, completed: &super::super::ops::CompletedLogicalShortCircuitV1) -> Result<(), String> {
        self.observe_bool_logical_v1(completed)
    }

    fn prepare_checked_print_expression_v1(&mut self) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1>>, String> {
        let (Some(ledger), Some(owner)) = (&self.ordinary_new_claim_ledger, self.callable_owner_v1()) else { return Ok(None) };
        let node = self.current_source_site_v1().ok_or_else(|| "[freeze:contract][print-expression/statement-site-missing]".to_owned())?;
        let statement = crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(node);
        ledger.print_expression_frame_v1(owner, &statement)
    }
    fn lower_checked_print_value_v1(&mut self, builder: &mut MirBuilder,
        frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1>,
    ) -> Result<ValueId, String> {
        let node = self.current_source_site_v1().ok_or_else(|| "[freeze:contract][print-expression/value-site-missing]".to_owned())?;
        if self.callable_owner_v1() != Some(frame.source().owner()) || frame.source().nodes()[0].site().node() != &node {
            return Err("[freeze:contract][print-expression/value-source-drift]".into());
        }
        let ledger = self.ordinary_new_claim_ledger.as_ref().ok_or_else(|| "[freeze:contract][print-expression/ledger-missing]".to_owned())?;
        let state = self.callable_ledger.as_ref().ok_or_else(|| "[freeze:contract][print-expression/state-missing]".to_owned())?;
        let consume_receiver = self.lexical_self_receiver_locator_consumer_v1()?;
        super::super::ordinary_new_admission::selected::print_expression::emit(builder, &mut state.borrow_mut(), ledger, frame,
            &consume_receiver)?;
        // Print has a genuine Unit statement result, outside the expression's
        // Continue boundary. Never invent an SSA String for inline StaticText.
        super::super::emission::constant::emit_void(builder)
    }

    fn prepare_ordinary_scalar_source_v1(
        &mut self, operator: &crate::ast::BinaryOperator,
    ) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::ExpressionScalarOperationLoanV1>>, String> {
        let (Some(ledger), Some(owner)) = (&self.ordinary_new_claim_ledger, self.callable_owner_v1()) else { return Ok(None); };
        if !ledger.has_scalar_operation_source_v1(owner) { return Ok(None); }
        let node = self.current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][scalar-original/source-site-missing]".to_owned())?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node));
        let operator = match super::super::ops::converters::convert_binary_operator(operator.clone())? {
            super::super::ops::converters::BinaryOpType::Arithmetic(op) => Some(op),
            _ => None,
        };
        ledger.prepare_scalar_operation_source_v1(owner, &site, operator)
    }

    fn complete_ordinary_scalar_source_v1(
        &mut self, source: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::ExpressionScalarOperationLoanV1>, children: (ValueId, ValueId),
        relations: [std::rc::Rc<crate::mir::builder::ssa::local::LocalSsaPhysicalRelationV1>; 2],
        completed: &super::super::ops::CompletedOrdinaryBinaryV1,
        index: &crate::mir::builder::ssa::local::LocalSsaPhysicalIndexV1,
        function: &crate::mir::MirFunction,
    ) -> Result<std::rc::Rc<crate::mir::normal_callable_semantic_package::OriginalScalarAppendV1>, String> {
        let ledger = self.ordinary_new_claim_ledger.as_ref()
            .ok_or_else(|| "[freeze:contract][scalar-original/ledger-missing]".to_owned())?;
        let node = self.current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][scalar-original/source-site-missing]".to_owned())?;
        if self.callable_owner_v1() != Some(source.owner())
            || source.binary().site() != &crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node) {
            return Err("[freeze:contract][scalar-original/source-site-drift]".into());
        }
        ledger.record_scalar_operation_append_v1(source, children, relations, completed, index, function)
    }

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

    fn prepare_borrowed_compare_operands_v1(
        &mut self,
        builder: &mut MirBuilder,
        source: &crate::mir::normal_callable_semantic_package::BorrowedCompareSourceLoanV1,
        children: (ValueId, ValueId),
    ) -> Result<(), String> {
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][borrowed-compare/carrier-ledger]".to_owned())?;
        if self.callable_owner_v1() != Some(source.owner()) {
            return Err("[freeze:contract][borrowed-compare/carrier-owner]".into());
        }
        let loans = ledger.borrow_compare_carrier_operands_v1(source)?;
        super::super::ssa::local::checked_compare::install_carriers(
            builder, source, children, loans,
        )
    }

    fn complete_borrowed_compare_source_v1(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::BorrowedCompareSourceLoanV1,
        children: (ValueId, ValueId),
        completed: &super::super::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<
        std::rc::Rc<crate::mir::normal_callable_semantic_package::BorrowedCompareMaterializationV1>,
        String,
    > {
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

    fn prepare_borrowed_mul_source_v1(
        &mut self,
        operator: &crate::ast::BinaryOperator,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::BorrowedMulSourceLoanV1>, String>
    {
        let (Some(ledger), Some(owner)) =
            (&self.ordinary_new_claim_ledger, self.callable_owner_v1())
        else {
            return Ok(None);
        };
        if !ledger.has_borrowed_mul_source_v1(owner) {
            return Ok(None);
        }
        let node = self
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][borrowed-mul/source-site-missing]".to_owned())?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node),
        );
        let operator =
            match super::super::ops::converters::convert_binary_operator(operator.clone())? {
                super::super::ops::converters::BinaryOpType::Arithmetic(operator) => Some(operator),
                super::super::ops::converters::BinaryOpType::Comparison(_) => None,
            };
        ledger.prepare_borrowed_mul_source_v1(owner, &site, operator)
    }

    fn complete_borrowed_mul_source_v1(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::BorrowedMulSourceLoanV1,
        children: (ValueId, ValueId),
        completed: &super::super::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<
        std::rc::Rc<crate::mir::normal_callable_semantic_package::BorrowedMulMaterializationV1>,
        String,
    > {
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][borrowed-mul/ledger-missing]".to_owned())?;
        if self.callable_owner_v1() != Some(loan.owner()) {
            return Err("[freeze:contract][borrowed-mul/owner-drift]".into());
        }
        let node = self
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][borrowed-mul/source-site-missing]".to_owned())?;
        if loan.site().site() != &crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node)
        {
            return Err("[freeze:contract][borrowed-mul/source-site-drift]".into());
        }
        ledger.record_borrowed_mul_v1(loan, children, completed)
    }

    fn prepare_terminal_null_literal_v1(
        &mut self,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::TerminalNullReturnSourceLoanV1>,
        String,
    > {
        let (Some(ledger), Some(owner)) =
            (&self.ordinary_new_claim_ledger, self.callable_owner_v1())
        else {
            return Ok(None);
        };
        let node = self
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][terminal-null/source-site-missing]".to_owned())?;
        ledger.prepare_terminal_null_literal_v1(
            crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                owner,
                crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node),
            ),
        )
    }
    fn complete_terminal_null_literal_v1(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::TerminalNullReturnSourceLoanV1,
        completed: &crate::mir::builder::emission::constant::CompletedConstV1,
    ) -> Result<(), String> {
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][terminal-null/ledger-missing]".to_owned())?;
        let owner = self
            .callable_owner_v1()
            .ok_or_else(|| "[freeze:contract][terminal-null/owner-missing]".to_owned())?;
        let node = self
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][terminal-null/source-site-missing]".to_owned())?;
        ledger.complete_terminal_null_literal_v1(
            loan,
            crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                owner,
                crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node),
            ),
            completed,
        )
    }

    fn prepare_compare_integer_literal_v1(
        &mut self,
        value: i64,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::BorrowedCompareIntegerLiteralLoanV1>,
        String,
    > {
        let (Some(ledger), Some(owner)) =
            (&self.ordinary_new_claim_ledger, self.callable_owner_v1())
        else {
            return Ok(None);
        };
        if !ledger.has_borrowed_compare_integer_literal_source_v1(owner) {
            return Ok(None);
        }
        let site = self
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][borrowed-literal/source-site-missing]".to_owned())?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site),
        );
        ledger.prepare_borrowed_compare_integer_literal_v1(owner, &site, value)
    }

    fn complete_compare_integer_literal_v1(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::BorrowedCompareIntegerLiteralLoanV1,
        completed: &crate::mir::builder::emission::constant::CompletedConstV1,
    ) -> Result<std::rc::Rc<crate::mir::normal_callable_semantic_package::BorrowedCompareIntegerLiteralMaterializationV1>, String>{
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .ok_or_else(|| "[freeze:contract][borrowed-literal/ledger-missing]".to_owned())?;
        if self.callable_owner_v1() != Some(loan.owner()) {
            return Err("[freeze:contract][borrowed-literal/owner-drift]".into());
        }
        let node = self
            .current_source_site_v1()
            .ok_or_else(|| "[freeze:contract][borrowed-literal/source-site-missing]".to_owned())?;
        if loan.site().site() != &crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node)
        {
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
        let has_selected_scalar = builder.local_ssa_physical_correspondence_v1()
            .scalar_originals().next().is_some();
        if has_selected_scalar && self.ordinary_new_claim_ledger.is_none() {
            return Err("[freeze:contract][scalar-original/ledger-missing]".into());
        }
        let Some(ledger) = self.callable_ledger.as_ref() else {
            if has_selected_scalar {
                return Err("[freeze:contract][scalar-original/entry-missing]".into());
            }
            if !builder.function_state.checked_mul_reuse.is_empty() {
                return Err("[freeze:contract][borrowed-mul/entry-missing]".into());
            }
            return Ok(());
        };
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or("[freeze:contract][construction-store/no-function]")?;
        builder
            .function_state
            .checked_compare_reuse
            .require_same_entry(ledger)?;
        builder
            .function_state
            .checked_compare_reuse
            .verify(builder)?;
        builder.function_state.checked_mul_reuse.verify(builder)?;
        let mut state = ledger.borrow_mut();
        state.complete_construction_stores(function)?;
        if let Some(news) = &self.ordinary_new_claim_ledger {
            news.verify_borrowed_compare_reuse_v1(
                state.owner(),
                builder.function_state.checked_compare_reuse.records(),
            )?;
            news.verify_borrowed_mul_reuse_v1(
                state.owner(),
                builder.function_state.checked_mul_reuse.records(),
            )?;
            news.record_borrowed_mul_consumers_v1(
                state.owner(),
                builder.function_state.checked_mul_reuse.observations(),
            )?;
            news.record_borrowed_compare_carrier_consumers_v1(
                state.owner(),
                function,
                builder
                    .function_state
                    .checked_compare_reuse
                    .carrier_observations(),
            )?;
            news.record_borrowed_compare_literal_consumers_v1(
                state.owner(),
                function,
                builder
                    .function_state
                    .checked_compare_reuse
                    .literal_observations(),
            )?;
            news.record_borrowed_compare_consumers_v1(
                state.owner(),
                function,
                builder.function_state.checked_compare_reuse.observations(),
            )?;
            news.record_scalar_operation_consumers_v1(state.owner(), function,
                builder.local_ssa_physical_correspondence_v1())?;
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
        self.lower_with_field_write_rhs_v1(builder, input)
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

impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn lower_expression_unobserved_v1(&mut self, builder: &mut MirBuilder, input: ASTNode)
        -> Result<ValueId, String> {
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
}
