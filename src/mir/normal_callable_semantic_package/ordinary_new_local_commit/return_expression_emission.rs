//! Return-specific result and residence contract over the common expression record.
use super::*;
/// One Return-owned expression. Residence carries only its additional ownership
/// events; scalar results reuse the same values, reads and independent replay.
#[derive(Debug)]
pub(crate) struct ReturnExpressionContractV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) source:
        std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) value:
        ValueId,
    pub(super) residence: Option<ReturnExpressionResidenceV1>,
    after_residency: Option<crate::mir::builder::BindingValueObservationV2>,
}
impl super::call_entry::ExpressionEmissionSourceV1 for ReturnExpressionContractV1 {
    fn evaluation(
        &self,
    ) -> &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1 {
        &self.source
    }
}
#[derive(Debug)]
pub(super) struct ReturnExpressionResidenceV1 {
    materialize: Option<[(BasicBlockId, MirInstruction); 2]>,
    pub(super) install: (BasicBlockId, MirInstruction),
}
impl super::ExpressionEmissionV1<ReturnExpressionContractV1> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn borrowed_root_boundary_v1(
        &self,
    ) -> Option<(BasicBlockId, &[(BasicBlockId, MirInstruction)])> {
        self.root_fault
            .map(|entry| (entry, self.original.as_slice()))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn require_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
    ) -> Result<(), String> {
        if self.contract.source.source().owner() != owner
            || self.contract.source.source().root().statement() != exit
        {
            return Err(freeze("finished-terminal-call/owner"));
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn visit_calls_v1(
        &self,
        visit: impl FnMut(
            &crate::mir::normal_callable_semantic_package::EmittedLexicalCallProjectionV1,
            &[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1],
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        self.expression().visit_calls_v1(visit)
    }

    pub(super) fn has_call_binding_v1(
        &self,
        context: CallValidationContextV1<'_>,
        site: &OwnedExprSiteV1,
        original: &(BasicBlockId, MirInstruction),
        copy: bool,
    ) -> Result<bool, String> {
        let mut count = 0;
        self.visit_calls_v1(|packet, arguments| {
            packet.visit_original_nodes_in_context_v1(
                site.owner(),
                arguments,
                context,
                &mut |node, _| {
                    if node.call_site() != site {
                        return Ok(());
                    }
                    let matched = if copy {
                        node.copy_dependencies_in_context(context)?
                            .iter()
                            .any(|(source, binding)| source == site && binding == original)
                    } else {
                        node.has_producer_at(site, original)
                    };
                    if matched {
                        count += 1;
                    }
                    Ok(())
                },
            )
        })?;
        if count > 1 {
            return Err(freeze("final-call-visit/source-duplicate"));
        }
        Ok(count == 1)
    }

    /// The body lends source correspondence only after the Return continuation checks.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn array_operation_bindings_v1(
        &self,
        operation: &std::rc::Rc<
            crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1,
        >,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        projection: &FinishedBindings,
    ) -> Result<
        Option<(
            (BasicBlockId, MirInstruction),
            (BasicBlockId, MirInstruction),
        )>,
        String,
    > {
        let mut pair = self
            .expression()
            .array_operation_bindings_v1(operation, context)?;
        for call in &self.calls {
            if let Some(argument) = call.array_argument_operation_bindings_v1(operation, context)? {
                if pair.is_some() {
                    return Err(freeze("finished-array-field/source-membership"));
                }
                pair = Some(argument);
            }
        }
        if pair.is_some() {
            self.validate(context, function, Some(projection))?;
        }
        Ok(pair)
    }

    pub(super) fn validate(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        projection: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        let expected_after = self.contract.residence.is_some()
            && self
                .contract
                .source
                .return_text_after_read_v2()
                .map_err(freeze)?
                .is_some();
        if expected_after != self.contract.after_residency.is_some() {
            return Err(freeze("expression/residence-original-pair"));
        }
        let (displaced, unwind) = match &self.contract.after_residency {
            Some(read) => {
                let (old, unwind) = context.ledger().return_text_after_unwind_v2(
                    &self.contract.source,
                    read,
                    context,
                    function,
                    projection,
                )?;
                (Some(old), Some(unwind))
            }
            None => (None, None),
        };
        let expression = self.expression();
        let expression = match &unwind {
            Some(unwind) => expression.with_prior_home_unwind(unwind),
            None => expression,
        };
        expression.validate_source(context)?;
        expression.validate_effective_root_v1(
            self.contract.value,
            self.contract.residence.is_some(),
            self.contract.residence.is_some()
                && self.contract.source.owned_text_root_node_v1().is_none(),
            self.contract
                .residence
                .as_ref()
                .and_then(|row| row.materialize.as_ref()),
        )?;
        let recorded = projection
            .map(|projection| projection.bindings(&self.original))
            .transpose()?;
        expression.validate_recorded(recorded.as_deref().unwrap_or(&self.original), projection)?;
        let covered = match &self.contract.residence {
            Some(row) => expression.validate_residence_continuation_v1(
                context,
                function,
                projection,
                &self.entry,
                &self.terminal,
                &self.continuation,
                Some(&row.install),
                displaced.as_ref(),
                row.materialize.as_ref(),
            )?,
            None => expression.validate_scalar_continuation_v1(
                context,
                function,
                projection,
                &self.entry,
                &self.terminal,
                &self.continuation,
                None,
            )?,
        };
        if projection.is_none() {
            let mut ordered = expression.ordered_bindings_in_context(context)?;
            if let Some(row) = &self.contract.residence {
                if let Some(pair) = &row.materialize {
                    ordered.extend(pair.iter().cloned());
                }
                ordered.push(row.install.clone());
            }
            ordered.extend(self.terminal.iter().cloned());
            expression.validate_original_scope(
                context,
                function,
                &covered,
                &self.entry,
                &self.terminal,
                &self.continuation,
                &ordered,
                None,
            )?;
        }
        Ok(())
    }
}
impl RootHomeCleanupOrderV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn return_expression_v1(
        &self,
    ) -> Option<&super::ExpressionEmissionV1<ReturnExpressionContractV1>> {
        self.return_expression.as_deref()
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn return_value_bindings_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let mut bindings = match &self.scalar_return_read {
            Some(proof) => proof.dependencies(
                self.return_read
                    .as_ref()
                    .ok_or_else(|| freeze("expression/scalar-binding-observation"))?,
            )?,
            None => Vec::new(),
        };
        let Some(expression) = &self.return_expression else {
            return Ok(bindings);
        };
        bindings.extend_from_slice(&expression.original);
        // Calls retain their source-qualified read adapters separately from
        // the enclosing expression. Lend the same dependencies to capture,
        // final validation and DCE; a Plain return has no direct Call entry.
        expression.visit_calls_v1(|packet, _| {
            for (_, binding) in packet.copy_dependencies_in_context(context)? {
                if !bindings.contains(&binding) {
                    bindings.push(binding);
                }
            }
            Ok(())
        })?;
        Ok(bindings)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn text_expression_install_v1(
        &self,
    ) -> Option<&(BasicBlockId, MirInstruction)> {
        self.return_expression
            .as_ref()
            .and_then(|row| row.contract.residence.as_ref())
            .map(|row| &row.install)
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn retain_return_expression_v1(
        &self,
        source: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        >,
        value: ValueId,
        body: crate::mir::builder::EmittedExpressionBodyV1,
        terminal: Option<(BasicBlockId, MirInstruction)>,
        continuation: Option<(BasicBlockId, MirInstruction)>,
        materialize: Option<[(BasicBlockId, MirInstruction); 2]>,
        after_residency: Option<crate::mir::builder::BindingValueObservationV2>,
        function: &MirFunction,
    ) -> Result<(), String> {
        let owner = source.source().owner();
        let site = source.source().root().statement();
        if self
            .return_expression_evaluation_v1(owner, site)?
            .is_none_or(|original| !std::rc::Rc::ptr_eq(&original, &source))
        {
            return Err(freeze("expression/residence-original-pair"));
        }
        let installs: Vec<_> = body
            .bindings
            .iter()
            .filter(|(_, row)| matches!(row, MirInstruction::StringResidenceInstall { .. }))
            .collect();
        use crate::mir::resolved_value_profile::{
            expression_class::ExpressionValueClassV1 as Class,
            expression_projection::ExpressionLeafClassV1 as Leaf,
        };
        let residence = match source.root_class() {
            Leaf::Value(Class::String) => {
                if installs.len() != 1
                    || !matches!(installs[0].1, MirInstruction::StringResidenceInstall {
                    value: installed, replaces, returning: false }
                        if installed == value && replaces.is_some() == after_residency.is_some())
                {
                    return Err(freeze("expression/residence-original-pair"));
                }
                Some(ReturnExpressionResidenceV1 {
                    install: (*installs[0]).clone(),
                    materialize,
                })
            }
            Leaf::Value(Class::I64 | Class::Bool)
                if installs.is_empty() && materialize.is_none() && after_residency.is_none() =>
            {
                None
            }
            _ => return Err(freeze("expression/residence-original-pair")),
        };
        let packet = super::ExpressionEmissionV1::from_body(
            ReturnExpressionContractV1 {
                residence,
                after_residency,
                source: source.clone(),
                value,
            },
            body,
            terminal,
            continuation,
        );
        packet.validate(CallValidationContextV1::Lowering(self), function, None)?;
        let mut rows = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Prepared(order)) =
            rows.get_mut(&(owner, site.clone().into()))
        else {
            return Err(freeze("root-exit-record-without-prepare"));
        };
        if order.return_expression.is_some() {
            return Err(freeze("expression/residence-original-pair"));
        }
        order.return_expression = Some(Box::new(packet));
        Ok(())
    }
}
