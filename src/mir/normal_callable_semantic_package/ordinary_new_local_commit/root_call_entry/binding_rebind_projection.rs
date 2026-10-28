//! Rebind purpose of the existing evaluated-binding prefix group.
//! Expression evaluation/cleanup is shared; only the assignment receipt differs.
use super::expression_projection::{ExpressionReferenceReadV1, ReferenceProducerV1};
use super::*;
use crate::mir::builder::CompletedVariableAssignmentV1;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1;
use std::{collections::BTreeMap, rc::Rc};
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(crate) struct EmittedBindingRebindProjectionV1 {
    frame: Rc<CheckedBindingRebindFrameV1>,
    receipt: CompletedVariableAssignmentV1,
    root: ReferenceProducerV1,
    install: Option<Binding>,
    purpose_acquisition: Option<[Binding; 2]>,
    loop_residency: Option<(Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        crate::mir::builder::BindingValueObservationV2)>,
    values: Vec<Option<EvaluatedExpressionValueV1>>,
    calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
    path_reads: Vec<std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>>,
    reference_reads: BTreeMap<usize, ExpressionReferenceReadV1>,
    entry: Binding,
    terminal: Binding,
    continuation: Binding,
    original: Vec<Binding>,
}
impl EmittedBindingRebindProjectionV1 {
    pub(super) fn corroborate_loop_frame_v2(
        &self,
        entry: BasicBlockId,
        end: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        if self.entry.0 != entry
            || self.value() != value
            || !matches!(self.continuation.1, MirInstruction::Jump { target, edge_args: None } if target == end)
        {
            return Err(freeze("source-loop/binding-frame-correspondence"));
        }
        Ok(())
    }
    pub(crate) fn frame(&self) -> &Rc<CheckedBindingRebindFrameV1> {
        &self.frame
    }
    pub(crate) fn value(&self) -> ValueId {
        self.receipt.assigned()
    }
    pub(in crate::mir) fn reference_read_v1(
        &self,
        node: usize,
    ) -> Option<&ExpressionReferenceReadV1> {
        self.reference_reads.get(&node)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit::root_home::call_entry) fn root_producer(
        &self,
    ) -> &ReferenceProducerV1 {
        &self.root
    }
    fn validate_effective_root(&self) -> Result<(), String> {
        self.expression().validate_effective_root_v1(
            self.value(),
            self.frame.is_text_residence(),
            self.frame.materializes_root(),
            self.purpose_acquisition.as_ref(),
        )
    }

    pub(in crate::mir::normal_callable_semantic_package) fn install_binding(
        &self,
    ) -> Option<&Binding> {
        self.install.as_ref()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn has_text_home(&self) -> bool {
        self.frame.is_text_residence() && self.receipt.rhs() == self.value()
            && self.install.as_ref().is_some_and(|(_, row)| matches!(row,
                MirInstruction::StringResidenceInstall { value, returning: false, .. } if *value == self.value()))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn root_acquisition_binding_v1(
        &self,
    ) -> Result<&Binding, String> {
        if !self.has_text_home() {
            return Err(freeze("rebind-expression/no-text-home"));
        }
        self.expression().require_unique(|row| matches!(row, MirInstruction::InvokeNormalResult { dst, .. } if *dst == self.value()))
    }
    pub(crate) fn calls(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        self.calls.iter().map(Rc::as_ref)
    }
    pub(in crate::mir) fn new(
        frame: Rc<CheckedBindingRebindFrameV1>,
        receipt: CompletedVariableAssignmentV1,
        values: Vec<Option<EvaluatedExpressionValueV1>>,
        calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
        path_reads: Vec<std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>>,
        reference_reads: BTreeMap<usize, ExpressionReferenceReadV1>,
        entry: Binding,
        terminal: Binding,
        continuation: Binding,
        purpose_acquisition: Option<[Binding; 2]>,
        loop_residency: Option<(Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
            crate::mir::builder::BindingValueObservationV2)>,
        original: Vec<Binding>,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        if receipt.rhs() != receipt.assigned() {
            return Err(freeze("rebind-expression/assignment-result"));
        }
        let root = if let Some(reference) = reference_reads
            .get(&0)
            .filter(|_| purpose_acquisition.is_none())
        {
            reference.result_producer_v1(receipt.rhs())?
        } else {
            let mut definitions = original
                .iter()
                .filter(|(_, row)| row.dst_value() == Some(receipt.rhs()));
            let root = definitions
                .next()
                .ok_or_else(|| freeze("rebind-expression/result-definition"))?;
            if definitions.next().is_some() || function.params.contains(&receipt.rhs()) {
                return Err(freeze("rebind-expression/result-unique"));
            }
            ReferenceProducerV1::Definition(root.clone())
        };
        let install = original
            .iter()
            .find(|(_, row)| matches!(row, MirInstruction::StringResidenceInstall { .. }))
            .cloned();
        let packet = Self {
            frame,
            receipt,
            root,
            install,
            purpose_acquisition,
            loop_residency,
            values,
            calls,
            path_reads,
            reference_reads,
            entry,
            terminal,
            continuation,
            original,
        };
        let context = CallValidationContextV1::Lowering(ledger);
        let covered = packet.validate_paths(context, function, None)?;
        let mut ordered = packet.expression().ordered_bindings_in_context(context)?;
        if let Some(pair) = &packet.purpose_acquisition {
            ordered.extend(pair.iter().cloned());
        }
        if let Some(install) = &packet.install {
            ordered.push(install.clone());
        }
        ordered.push(packet.terminal.clone());
        packet.expression().validate_original_scope(
            ledger,
            function,
            &covered,
            &packet.entry,
            &packet.terminal,
            &packet.continuation,
            &ordered,
        )?;
        Ok(packet)
    }
    pub(super) fn expression(&self) -> super::expression_projection::ExpressionPhysicalProjectionRefV1<'_> {
        super::expression_projection::ExpressionPhysicalProjectionRefV1::new(
            self.frame.evaluation(),
            &self.values,
            &self.calls,
            &self.path_reads,
            &self.reference_reads,
            &self.original,
        )
    }
    pub(super) fn validate_recorded(
        &self,
        bindings: &[Binding],
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.expression().validate_recorded(bindings, finishing)?;
        if [&self.entry, &self.terminal, &self.continuation]
            .iter()
            .any(|row| !self.original.contains(row))
        {
            return Err(freeze("rebind-expression/structural-membership"));
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_paths(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        let selected = context
            .ledger()
            .rebind_expression_frame_v1(
                self.frame.source().owner(),
                self.frame.source().root().statement().node(),
            )?
            .ok_or_else(|| freeze("rebind-expression/source-missing"))?;
        let dynamic_root = self.frame.evaluation().root_class()
            == crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
                crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic);
        if dynamic_root != self.frame.is_dynamic_loop_carrier_v1() {
            return Err(freeze("rebind-expression/dynamic-carrier-source"));
        }
        self.validate_effective_root()?;
        if !Rc::ptr_eq(&selected, &self.frame) || self.receipt.rhs() != self.receipt.assigned() {
            return Err(freeze("rebind-expression/source-or-value"));
        }
        if !self.receipt.matches_rebind_frame_v1(&self.frame) {
            return Err(freeze("rebind-expression/target-name"));
        }
        self.expression().validate_source(context)?;
        let installs = self
            .original
            .iter()
            .filter(|(_, row)| matches!(row, MirInstruction::StringResidenceInstall { .. }))
            .count();
        if installs != usize::from(self.frame.is_text_residence())
            || self.frame.is_text_residence() && !self.has_text_home()
        {
            return Err(freeze("rebind-expression/install-census"));
        }
        if self.frame.text_loop_carrier_v1().is_some() != self.loop_residency.is_some() {
            return Err(freeze("rebind-expression/old-text-end"));
        }
        let (displaced, unwind) = match &self.loop_residency {
            Some((product, read)) => {
                let (old, unwind) = context.ledger().loop_text_rebind_unwind_v2(
                    &self.frame, product, read, context, function, finishing)?;
                (Some(old), Some(unwind))
            }
            None => (context.ledger().text_rebind_displaced_end_v1(&self.frame)?, None),
        };
        let expression = self.expression();
        let expression = match &unwind {
            Some(unwind) => expression.with_prior_home_unwind(unwind),
            None => expression,
        };
        let covered = expression.validate_residence_continuation_v1(
            context,
            function,
            finishing,
            &self.entry,
            &self.terminal,
            &self.continuation,
            self.install.as_ref(),
            displaced.as_ref(),
            self.purpose_acquisition.as_ref(),
        )?;
        Ok(covered)
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn branch_scalar_sources_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> Vec<Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>> {
        use crate::mir::resolved_semantics::home_new_prefix::{
            CheckedExpressionReferenceV1 as Reference, CurrentScalarProducerV1 as Producer,
        };
        fn visit(
            source: &Producer,
            owner: FunctionOwnerIdV1,
            site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
            output: &mut Vec<
                Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>,
            >,
        ) {
            if let Producer::BranchNormal(basis) = source {
                if basis.binding().owner() == owner
                    && basis.site() == site
                    && !output.iter().any(|old| old == basis)
                {
                    output.push(Rc::clone(basis));
                }
                for (_, _, input) in basis.incoming() {
                    visit(input, owner, site, output);
                }
            }
        }
        fn visit_evaluation(
            evaluation: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
            owner: FunctionOwnerIdV1,
            site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
            output: &mut Vec<
                Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>,
            >,
        ) {
            for reference in evaluation.references().values() {
                if let Reference::Scalar(read) = reference {
                    if let Some(read) = read.original_read() {
                        visit(read.producer(), owner, site, output);
                    }
                }
            }
        }
        fn visit_plan(
            plan: &crate::mir::resolved_semantics::home_new_prefix::HomeExpressionPlanV1,
            owner: FunctionOwnerIdV1,
            site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
            output: &mut Vec<
                Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>,
            >,
        ) {
            if let Some(evaluation) = plan.evaluation() {
                visit_evaluation(evaluation, owner, site, output);
            }
            // Nested frames stay in the SAME retained Loop product; the outer
            // flow does not flatten them into independent expression owners.
            if let Some(product) = plan.loop_recipe_v2() {
                visit_evaluation(product.source.condition.evaluation(), owner, site, output);
                for plan in &product.source.body_expressions {
                    visit_plan(plan, owner, site, output);
                }
            }
        }
        let mut output = Vec::new();
        if let Some(flow) = self
            .completion_for_owner(owner)
            .and_then(|row| row.cleanup().root_flow())
        {
            for plan in flow.expression_plans() {
                visit_plan(plan, owner, site, &mut output);
            }
        }
        for source in self.selected_source_loop_branch_joins_v2(owner, site) {
            visit(&Producer::BranchNormal(source), owner, site, &mut output);
        }
        output
    }

    pub(in crate::mir) fn branch_text_sources_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> Vec<Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1>> {
        self.selected_source_loop_text_branch_joins_v2(owner, site)
    }

    pub(in crate::mir) fn require_branch_text_source_v1(
        &self,
        source: &Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1>,
    ) -> Result<(), String> {
        if !source.corroborates_original()
            || !self.branch_text_sources_v1(source.binding().owner(), source.site())
                .iter().any(|original| original == source)
        {
            return Err(freeze("source-loop/text-branch-obligation"));
        }
        Ok(())
    }
    pub(in crate::mir) fn require_branch_scalar_source_v1(
        &self,
        source: &Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>,
    ) -> Result<(), String> {
        if !source.corroborates_original()
            || !self
                .branch_scalar_sources_v1(source.binding().owner(), source.site())
                .iter()
                .any(|original| original == source)
        {
            return Err(freeze("expression/scalar-producer-kind"));
        }
        Ok(())
    }
    pub(crate) fn rebind_expression_frame_v1(
        &self,
        owner: FunctionOwnerIdV1,
        statement: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<Rc<CheckedBindingRebindFrameV1>>, String> {
        let Some(flow) = self
            .completion_for_owner(owner)
            .and_then(|row| row.cleanup().root_flow())
        else {
            return Ok(None);
        };
        let mut frames = flow
            .expression_plans()
            .iter()
            .filter_map(|plan| plan.rebind_frame())
            .filter(|frame| frame.source().root().statement().node() == statement);
        let frame = frames.next().cloned();
        if frames.next().is_some()
            || frame
                .as_ref()
                .is_some_and(|frame| frame.source().owner() != owner)
        {
            return Err(freeze("rebind-expression/source-duplicate-or-owner"));
        }
        Ok(frame)
    }
    pub(crate) fn validate_rebind_expression_source_v1(
        &self,
        frame: &Rc<CheckedBindingRebindFrameV1>,
    ) -> Result<(), String> {
        let site = OwnedExprSiteV1::new(
            frame.source().owner(),
            frame.source().nodes()[0].site().clone(),
        );
        let source = self.next_prefix_source_v2(frame.source().owner(), &site)?;
        if !matches!(source.as_ref(), Some(LifecyclePrefixSourceV1::BindingRebind { frame: original, .. }) if Rc::ptr_eq(original, frame))
        {
            return Err(freeze("rebind-expression/source-prefix-order"));
        }
        Ok(())
    }
    pub(crate) fn prepare_rebind_expression_bindings(
        &self,
        packet: Rc<EmittedBindingRebindProjectionV1>,
        bindings: Vec<Binding>,
    ) -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        self.prepare_local_call_binding_group(
            packet.frame.source().owner(),
            RootLifecyclePrefixBindingGroupV1::new_rebind(packet, bindings)?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scalar_rebind_keeps_assignment_and_alias_provenance_through_both_final_consumers() {
        for rhs in ["x", "x + 1", "7", "(\"a\" + \"b\").length()"] {
            let text = format!(
                "static box Main {{ main() {{ local x = 0 local before = x
            x = {rhs} local alias = x x = x local after = x
            local text = \"\" + before + alias + after print(text) return 0 }} }}"
            );
            let (completed, ledger) =
                crate::mir::builder::lexical_call_projection_document_completion_fixture(&text);
            let (key, module, same, keys, cohort) =
                completed.document_preflight_parts_for_test(true).unwrap();
            assert!(Rc::ptr_eq(&same, &ledger));
            let handoff = ledger
                .seal_finalized_root_birth_handoff(key.clone(), &module, &keys, cohort, Vec::new())
                .unwrap();
            let root = handoff.root_source().unwrap();
            root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
                .unwrap();
            let packets: Vec<_> = root
                .local_calls
                .get(&root.owner())
                .unwrap()
                .iter()
                .filter_map(|group| group.rebind())
                .collect();
            assert_eq!(
                packets.len(),
                2,
                "both original rebinds remain mandatory: {rhs}"
            );
            assert_ne!(
                packets[0].frame().source().root().statement(),
                packets[1].frame().source().root().statement()
            );
            assert!(!packets[0]
                .receipt
                .matches_rebind_frame_v1(packets[1].frame()));
            assert!(!packets[1]
                .receipt
                .matches_rebind_frame_v1(packets[0].frame()));
            assert_eq!(
                packets[0].value(),
                packets[1].value(),
                "self-assignment reuses RHS, never a fresh definition"
            );
            assert!(
                !module
                    .functions
                    .get(&key)
                    .unwrap()
                    .blocks
                    .values()
                    .flat_map(|block| block.all_instructions())
                    .any(|row| matches!(row, MirInstruction::ReleaseStrong { .. })),
                "a scalar rebind owns no Home to release"
            );
            let mut changed = module.clone();
            let mut count = 0;
            for block in changed.functions.get_mut(&key).unwrap().blocks.values_mut() {
                for row in &mut block.instructions {
                    if let MirInstruction::Const {
                        value: crate::mir::ConstValue::Integer(value),
                        ..
                    } = row
                    {
                        *value += 1;
                        count += 1;
                    }
                }
            }
            assert!(count > 0);
            assert!(
                root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(()))
                    .is_err(),
                "changed producer: {rhs}"
            );
        }
    }
}
