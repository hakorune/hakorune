//! Rebind purpose of the existing evaluated-binding prefix group.
//! Expression evaluation/cleanup is shared; only the assignment receipt differs.
use super::expression_projection::{ExpressionReferenceReadV1, ReferenceProducerV1};
use super::*;
use crate::mir::builder::CompletedVariableAssignmentV1;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(crate) struct BindingRebindExpressionContractV1 {
    frame: Rc<CheckedBindingRebindFrameV1>,
    receipt: CompletedVariableAssignmentV1,
    root: ReferenceProducerV1,
    install: Option<Binding>,
    purpose_acquisition: Option<[Binding; 2]>,
    loop_residency: Option<(
        Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        crate::mir::builder::BindingValueObservationV2,
    )>,
}
impl super::ExpressionEmissionSourceV1 for BindingRebindExpressionContractV1 {
    fn evaluation(
        &self,
    ) -> &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1 {
        self.frame.evaluation()
    }
}
impl ExpressionEmissionV1<BindingRebindExpressionContractV1> {
    pub(super) fn corroborate_loop_frame_v2(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        entry: BasicBlockId,
        end: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        if self.value() != value {
            return Err(freeze("source-loop/binding-frame-correspondence"));
        }
        self.expression().corroborate_outer_scope_v1(
            ledger,
            &self.entry,
            &self.continuation,
            entry,
            end,
        )
    }
    pub(crate) fn frame(&self) -> &Rc<CheckedBindingRebindFrameV1> {
        &self.contract.frame
    }
    pub(crate) fn value(&self) -> ValueId {
        self.contract.receipt.assigned()
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
        &self.contract.root
    }
    fn validate_effective_root(&self) -> Result<(), String> {
        self.expression().validate_effective_root_v1(
            self.value(),
            self.contract.frame.is_text_residence(),
            self.contract.frame.materializes_root(),
            self.contract.purpose_acquisition.as_ref(),
        )
    }

    pub(in crate::mir::normal_callable_semantic_package) fn install_binding(
        &self,
    ) -> Option<&Binding> {
        self.contract.install.as_ref()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn has_text_home(&self) -> bool {
        self.contract.frame.is_text_residence() && self.contract.receipt.rhs() == self.value()
            && self.contract.install.as_ref().is_some_and(|(_, row)| matches!(row,
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
    pub(in crate::mir) fn new_rebind_v1(
        frame: Rc<CheckedBindingRebindFrameV1>,
        receipt: CompletedVariableAssignmentV1,
        body: crate::mir::builder::EmittedExpressionBodyV1,
        terminal: Option<Binding>,
        continuation: Option<Binding>,
        purpose_acquisition: Option<[Binding; 2]>,
        loop_residency: Option<(
            Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
            crate::mir::builder::BindingValueObservationV2,
        )>,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        if receipt.rhs() != receipt.assigned() {
            return Err(freeze("rebind-expression/assignment-result"));
        }
        let root = if let Some(reference) = body
            .reference_reads
            .get(&0)
            .filter(|_| purpose_acquisition.is_none())
        {
            reference.result_producer_v1(receipt.rhs())?
        } else {
            let mut definitions = body
                .bindings
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
        let install = body
            .bindings
            .iter()
            .find(|(_, row)| matches!(row, MirInstruction::StringResidenceInstall { .. }))
            .cloned();
        let mut packet = Self::from_body(
            BindingRebindExpressionContractV1 {
                frame,
                receipt,
                root,
                install,
                purpose_acquisition,
                loop_residency,
            },
            body,
            terminal,
            continuation,
        );
        // Rebind's prior-home unwind is a destination contract, not a Return root Fault.
        packet.root_fault = None;
        let _physical_scope = ledger.physical_validation_scope_v1(function, None, false);
        let context = CallValidationContextV1::Lowering(ledger);
        let covered = packet.validate_paths(context, function, None)?;
        let mut ordered = packet.expression().ordered_bindings_in_context(context)?;
        if let Some(pair) = &packet.contract.purpose_acquisition {
            ordered.extend(pair.iter().cloned());
        }
        if let Some(install) = &packet.contract.install {
            ordered.push(install.clone());
        }
        ordered.extend(packet.terminal.iter().cloned());
        packet.expression().validate_original_scope(
            CallValidationContextV1::Lowering(ledger),
            function,
            &covered,
            &packet.entry,
            &packet.terminal,
            &packet.continuation,
            &ordered,
            None,
        )?;
        Ok(packet)
    }
    pub(super) fn validate_recorded(
        &self,
        bindings: &[Binding],
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.expression().validate_recorded(bindings, finishing)?;
        if [&self.entry, &self.terminal, &self.continuation]
            .into_iter()
            .flatten()
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
                self.contract.frame.source().owner(),
                self.contract.frame.source().root().statement().node(),
            )?
            .ok_or_else(|| freeze("rebind-expression/source-missing"))?;
        let dynamic_root = self.contract.frame.evaluation().root_class()
            == crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
                crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic);
        if dynamic_root != self.contract.frame.is_dynamic_loop_carrier_v1() {
            return Err(freeze("rebind-expression/dynamic-carrier-source"));
        }
        self.validate_effective_root()?;
        if !Rc::ptr_eq(&selected, &self.contract.frame)
            || self.contract.receipt.rhs() != self.contract.receipt.assigned()
        {
            return Err(freeze("rebind-expression/source-or-value"));
        }
        if !self
            .contract
            .receipt
            .matches_rebind_frame_v1(&self.contract.frame)
        {
            return Err(freeze("rebind-expression/target-name"));
        }
        self.expression().validate_source(context)?;
        let installs = self
            .original
            .iter()
            .filter(|(_, row)| matches!(row, MirInstruction::StringResidenceInstall { .. }))
            .count();
        if installs != usize::from(self.contract.frame.is_text_residence())
            || self.contract.frame.is_text_residence() && !self.has_text_home()
        {
            return Err(freeze("rebind-expression/install-census"));
        }
        if self.contract.frame.text_loop_carrier_v1().is_some()
            != self.contract.loop_residency.is_some()
        {
            return Err(freeze("rebind-expression/old-text-end"));
        }
        let (displaced, unwind) = match &self.contract.loop_residency {
            Some((product, read)) => {
                let (old, unwind) = context.ledger().loop_text_rebind_unwind_v2(
                    &self.contract.frame,
                    product,
                    read,
                    context,
                    function,
                    finishing,
                )?;
                (Some(old), Some(unwind))
            }
            None => (
                context
                    .ledger()
                    .text_rebind_displaced_end_v1(&self.contract.frame)?,
                None,
            ),
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
            self.contract.install.as_ref(),
            displaced.as_ref(),
            self.contract.purpose_acquisition.as_ref(),
        )?;
        Ok(covered)
    }
}
impl OrdinaryNewClaimLedgerV1 {
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
            || !self
                .branch_text_sources_v1(source.binding().owner(), source.site())
                .iter()
                .any(|original| original == source)
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
                .branch_scalar_sources_v1(source.binding().owner(), source.site())?
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
        packet: Rc<ExpressionEmissionV1<BindingRebindExpressionContractV1>>,
        bindings: Vec<Binding>,
    ) -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        self.prepare_local_call_binding_group(
            packet.contract.frame.source().owner(),
            RootLifecyclePrefixBindingGroupV1::new_rebind(packet, bindings)?,
        )
    }
}

#[cfg(test)]
#[path = "binding_rebind_projection_tests.rs"]
mod tests;
