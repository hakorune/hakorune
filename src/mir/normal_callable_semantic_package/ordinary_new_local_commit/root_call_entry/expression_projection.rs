//! Borrowed physical correspondence for one checked original expression body.
//! Purpose wrappers retain source identity, terminal effects and lifetime policy.
use super::*;
use crate::mir::instruction::{StringInvokeOperand as Operand, StringInvokeOperation as Operation};
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1;
use crate::mir::resolved_semantics::ResolvedLiteralSourceV1 as Literal;
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);
#[path = "expression_reference_read.rs"]
mod reference_read;
pub(in crate::mir) use reference_read::{
    BranchScalarPhiObservationV1, BranchTextPhiObservationV1, BranchReadPhiObservationV1, ExpressionReferenceReadV1, ReferenceProducerV1, ScalarBindingReadProofV1,
};
#[path = "expression_terminal_paths.rs"]
mod terminal_paths;
pub(in crate::mir) use terminal_paths::{
    ExpressionCallArgumentSourceV1, ExpressionCallTransientPrefixV1, ExpressionOwnedTemporaryV1,
};

#[path = "expression_read_value.rs"]
mod read_value;
pub(crate) use read_value::EvaluatedExpressionValueV1;

#[path = "expression_source_values.rs"]
mod source_values;
pub(crate) use source_values::checked_expression_core_operation_v1;
#[path = "expression_effect_order.rs"]
mod effect_order;

/// A view of the purpose's original observations, never a second packet pool.
/// Successful correspondence grants no terminal, residence or cleanup rights.
pub(in crate::mir) struct ExpressionPhysicalProjectionRefV1<'a> {
    frame: &'a CheckedExpressionEvaluationV1,
    values: &'a [Option<EvaluatedExpressionValueV1>],
    calls: &'a [Rc<EmittedLexicalCallProjectionV1>],
    dynamic_calls: &'a [Rc<EmittedDynamicInvocationCallProjectionV1>],
    path_reads: &'a [std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>],
    reference_reads: &'a std::collections::BTreeMap<usize, ExpressionReferenceReadV1>,
    original: &'a [Binding],
    prior_home_unwind: Option<&'a [InvokeOperation]>,
    inline_range: Option<(BasicBlockId, usize, usize, bool)>,
    comparison: Option<(usize, ValueId)>,
    root_fault: Option<BasicBlockId>,
    scalar_receiver: Option<&'a crate::mir::builder::ExactLexicalReadV1>,
}
impl<'a> ExpressionPhysicalProjectionRefV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn with_root_fault_v1(mut self, root_fault: Option<BasicBlockId>) -> Self {
        self.root_fault = root_fault;
        self
    }
    pub(super) fn permits_empty_bindings_v1(&self, ledger: &OrdinaryNewClaimLedgerV1) -> bool {
        self.original.is_empty() && self.inline_range.is_some()
            && !ledger.expression_requires_fault_frame_v1(self.frame)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_normal_order_v1(
        function: &MirFunction,
        ordered: &[Binding],
    ) -> Result<(), String> {
        normal_order(function, ordered)
    }
    pub(in crate::mir) fn new(
        frame: &'a CheckedExpressionEvaluationV1,
        values: &'a [Option<EvaluatedExpressionValueV1>],
        calls: &'a [Rc<EmittedLexicalCallProjectionV1>],
        path_reads: &'a [std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>],
        reference_reads: &'a std::collections::BTreeMap<usize, ExpressionReferenceReadV1>,
        original: &'a [Binding],
    ) -> Self {
        Self::new_with_dynamic(
            frame,
            values,
            calls,
            &[],
            path_reads,
            reference_reads,
            original,
        )
    }

    pub(in crate::mir) fn new_with_dynamic(
        frame: &'a CheckedExpressionEvaluationV1,
        values: &'a [Option<EvaluatedExpressionValueV1>],
        calls: &'a [Rc<EmittedLexicalCallProjectionV1>],
        dynamic_calls: &'a [Rc<EmittedDynamicInvocationCallProjectionV1>],
        path_reads: &'a [std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>],
        reference_reads: &'a std::collections::BTreeMap<usize, ExpressionReferenceReadV1>,
        original: &'a [Binding],
    ) -> Self {
        Self {
            frame,
            values,
            calls,
            dynamic_calls,
            path_reads,
            reference_reads,
            original,
            prior_home_unwind: None,
            inline_range: None,
            comparison: None,
            root_fault: None,
            scalar_receiver: None,
        }
    }

    pub(super) fn corroborate_outer_scope_v1(
        &self, ledger: &OrdinaryNewClaimLedgerV1, entry: &Option<Binding>,
        continuation: &Option<Binding>, start: BasicBlockId, end: BasicBlockId,
    ) -> Result<(), String> {
        let matches = if ledger.expression_requires_fault_frame_v1(self.frame) {
            self.inline_range.is_none() && entry.as_ref().is_some_and(|row| row.0 == start)
                && matches!(continuation.as_ref(), Some((_, MirInstruction::Jump { target, edge_args: None })) if *target == end)
        } else {
            entry.is_none() && continuation.is_none() && start == end
                && self.inline_range.is_some_and(|(block, _, _, _)| block == start)
        };
        if !matches { return Err(freeze("source-loop/binding-frame-correspondence")); }
        Ok(())
    }

    pub(in crate::mir) fn with_inline_range_v1(mut self, range: Option<(BasicBlockId, usize, usize, bool)>) -> Self {
        self.inline_range = range;
        self
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn with_prior_home_unwind(mut self, unwind: &'a [InvokeOperation]) -> Self {
        self.prior_home_unwind = Some(unwind);
        self
    }

    pub(in crate::mir) fn for_comparison_inputs(
        mut self,
        node: usize,
        result: ValueId,
        receiver: Option<&'a crate::mir::builder::ExactLexicalReadV1>,
    ) -> Result<Self, String> {
        self.frame
            .scalar_invoke_input_order_v1(node)
            .ok_or_else(|| freeze("bool-condition/text-comparison-source"))?;
        self.comparison = Some((node, result));
        self.scalar_receiver = receiver;
        Ok(self)
    }
    /// Shared purpose-acquisition correspondence. The source evaluation's root
    /// remains unchanged when a destination materializes its independent Text.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_effective_root_v1(
        &self,
        value: ValueId,
        text: bool,
        materializes: bool,
        acquisition: Option<&[Binding; 2]>,
    ) -> Result<(), String> {
        let expected = match self.frame.root_class() {
            crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
                crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Bool) =>
                    EvaluatedExpressionValueV1::Bool(value),
            crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
                crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic) =>
                    EvaluatedExpressionValueV1::Operand(Operand::Dynamic(value)),
            crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
                crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Opaque)
                if matches!(self.frame.references().get(&0), Some(crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::NullableResult(
                    crate::mir::resolved_semantics::home_new_prefix::NullableResultReferenceV1::DeclaredFrom(_)))) => EvaluatedExpressionValueV1::DeclaredRead(value),
            _ => (if text { Operand::OwnedString(value) } else { Operand::NormalInteger(value) }).into(),
        };
        match acquisition {
            Some([invoke, normal]) if materializes && text => {
                let Some(Some(input)) = self.values.first() else {
                    return Err(freeze("rebind-expression/materialize-input"));
                };
                let input = input.as_operand()?;
                let snapshot = matches!(input, Operand::DynamicCarrier(_))
                    && self.frame.text_snapshot_root_v1().is_some_and(|node|
                        self.values.get(node).and_then(Option::as_ref).and_then(|value| value.as_operand().ok()) == Some(input));
                if !(input.is_text() || snapshot)
                    || !matches!(&invoke.1, MirInstruction::Invoke {
                    operation: InvokeOperation::String(Operation::Materialize { value }), normal_landing, .. }
                        if value == input && *normal_landing == normal.0)
                    || normal.1
                        != (MirInstruction::InvokeNormalResult {
                            invoke_block: invoke.0,
                            dst: value,
                        })
                    || !self.original.contains(invoke)
                    || !self.original.contains(normal)
                {
                    return Err(freeze("rebind-expression/materialize-result"));
                }
            }
            None if !materializes && self.values.first() == Some(&Some(expected)) => {}
            _ => return Err(freeze("rebind-expression/purpose-result")),
        }
        Ok(())
    }

    fn node_selected(&self, node: usize) -> bool {
        self.comparison.is_none_or(|(root, _)| {
            node != root
                && self.frame.source().nodes()[node]
                    .site()
                    .node()
                    .segments()
                    .starts_with(self.frame.source().nodes()[root].site().node().segments())
        })
    }
    fn source_calls(
        &self,
    ) -> impl Iterator<Item = &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1>
    {
        self.frame.calls().iter().filter(|call| {
            self.frame
                .source()
                .nodes()
                .iter()
                .position(|node| node.site() == call.site().site())
                .is_some_and(|node| self.node_selected(node))
                && call.dynamic_invocation_source_v1().is_none()
        })
    }
    fn source_dynamic_calls(
        &self,
    ) -> impl Iterator<Item = &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1>
    {
        self.frame.calls().iter().filter(|call| {
            self.frame
                .source()
                .nodes()
                .iter()
                .position(|node| node.site() == call.site().site())
                .is_some_and(|node| self.node_selected(node))
                && call.dynamic_invocation_source_v1().is_some()
        })
    }
    /// Borrow original Array field/operation pairs from the same complete body.
    /// Purpose owners must validate their full continuation before publishing it.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn array_operation_bindings_v1(
        &self, operation: &Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>,
        context: CallValidationContextV1<'_>,
    ) -> Result<Option<(Binding, Binding)>, String> {
        let mut cores = self.frame.cores().values().filter(|core| self.node_selected(core.node())
            && !self.frame.transfers_node(core.node()) && core.field_array_access_v1().is_some_and(|source| source.as_ref() == operation.as_ref()));
        let Some(core) = cores.next() else { return Ok(None); };
        if cores.next().is_some() { return Err(freeze("finished-array-field/source-membership")); }
        let (array, binding) = if operation.is_count() {
            let std::ops::ControlFlow::Continue((crate::mir::core_method_result_kind::CoreMethodSemanticLawV2::ArrayCount, Operand::NormalInteger(array))) =
                checked_expression_core_operation_v1(core, self.values)? else { return Err(freeze("finished-array-field/source-membership")); };
            let Some(Some(EvaluatedExpressionValueV1::Operand(Operand::NormalInteger(value)))) = self.values.get(core.node())
                else { return Err(freeze("finished-array-field/source-membership")); };
            (array, self.require_unique(|row| matches!(row, MirInstruction::ArrayRead {
                dst: Some(dst), receiver, operation: crate::mir::instruction::ArrayReadOperationV1::Count, ..
            } if dst == value && *receiver == array))?.clone())
        } else {
            let expected = checked_expression_core_operation_v1(core, self.values)?;
            let std::ops::ControlFlow::Break(InvokeOperation::DynamicArrayGet { array, .. }) = expected else { return Err(freeze("finished-array-field/source-membership")); };
            let Some(Some(EvaluatedExpressionValueV1::Operand(Operand::DynamicCarrier(value)))) = self.values.get(core.node())
                else { return Err(freeze("finished-array-field/source-membership")); };
            let normal = self.require_unique(|row| matches!(row, MirInstruction::InvokeNormalResult { dst, .. } if dst == value))?;
            let MirInstruction::InvokeNormalResult { invoke_block, .. } = normal.1 else { unreachable!() };
            let invoke = self.require_unique(|row| matches!(row, MirInstruction::Invoke { operation, normal_landing, .. }
                if std::ops::ControlFlow::Break(operation.clone()) == expected && *normal_landing == normal.0))?;
            if invoke.0 != invoke_block { return Err(freeze("finished-array-field/source-membership")); }
            (array, invoke.clone())
        };
        let field = if operation.receiver_alias_v1().is_some() {
            let mut reads = self.path_reads.iter().filter(|read| read.source_value().0 == *operation.receiver());
            let read = reads.next().ok_or_else(|| freeze("finished-array-field/source-membership"))?;
            if reads.next().is_some() { return Err(freeze("finished-array-field/source-membership")); }
            let (value, original) = context.ledger().array_alias_operation_read_v1(operation, read, context)?;
            if value != array { return Err(freeze("finished-array-field/source-membership")); }
            original
        } else {
            let original = context.ledger().expression_field_read_emission_v1(operation.receiver(), self.frame.source(), context)?;
            if original.1.dst_value() != Some(array) { return Err(freeze("finished-array-field/source-membership")); }
            original
        };
        Ok(Some((field, binding)))
    }

    fn comparison_pair(&self) -> Result<Option<Vec<Binding>>, String> {
        let Some((node, result)) = self.comparison else { return Ok(None); };
        scalar_condition_bindings_v1(self.frame, node, self.values, self.scalar_receiver,
            result, self.original).map(Some)
    }

    /// Borrow the same selected source/physical call pairs; nested argument
    /// packets remain owned by their enclosing call rather than a second pool.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn visit_calls_v1(
        &self, mut visit: impl FnMut(&EmittedLexicalCallProjectionV1,
            &[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1]) -> Result<(), String>,
    ) -> Result<(), String> {
        let sources: Vec<_> = self.source_calls().collect();
        if sources.len() != self.calls.len() { return Err(freeze("print-expression/source-coverage")); }
        for (source, packet) in sources.into_iter().zip(self.calls()) {
            if source.site() != packet.call_site() { return Err(freeze("print-expression/call-source")); }
            visit(packet, source.arguments())?;
        }
        Ok(())
    }

    fn calls(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        self.calls.iter().map(Rc::as_ref)
    }
    fn dynamic_calls(&self) -> impl Iterator<Item = &EmittedDynamicInvocationCallProjectionV1> {
        self.dynamic_calls.iter().map(Rc::as_ref)
    }

    /// One transient census of original field geometry, shared by source and
    /// physical dependencies. It grants no receiver or nonnull permission.
    fn original_field_roots_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<
        std::collections::BTreeMap<
            OwnedExprSiteV1,
            (
                crate::mir::resolved_semantics::BindingRefV1,
                OwnedExprSiteV1,
            ),
        >,
        String,
    > {
        let source = self.frame.source();
        let ledger = context.ledger();
        let mut roots = std::collections::BTreeMap::new();
        for (_, node) in source
            .nodes()
            .iter()
            .enumerate()
            .filter(|(node, _)| self.node_selected(*node))
        {
            let site = OwnedExprSiteV1::new(source.owner(), node.site().clone());
            if let Some(path) = ledger.statement_field_path_v1(&site, context)? {
                let root_site = OwnedExprSiteV1::new(source.owner(), path.root_site().clone());
                let expected = (
                    path.root(),
                    OwnedExprSiteV1::new(source.owner(), path.first_site().clone()),
                );
                if roots
                    .insert(root_site, expected.clone())
                    .is_some_and(|prior| prior != expected)
                {
                    return Err(freeze("print-expression/path-root-source"));
                }
            }
        }
        for (index, _) in source.nodes().iter().enumerate().filter(|(index, _)| self.node_selected(*index)) {
            let Some(core) = self.frame.direct_array_receiver_v1(index).filter(|core| self.node_selected(core.node())) else { continue; };
            let access = core.field_array_access_v1().ok_or_else(|| freeze("print-expression/path-root-source"))?;
            let required = ledger.source_field_requirement_v1(access.receiver())?
                .ok_or_else(|| freeze("print-expression/path-root-source"))?;
            if !Rc::ptr_eq(required.plan(), source) || required.request().node_in_plan(source) != Some(index) {
                return Err(freeze("print-expression/path-root-source"));
            }
            let root = OwnedExprSiteV1::new(source.owner(), required.request().receiver_site.clone());
            let expected = (required.request().receiver, access.receiver().clone());
            if roots.insert(root, expected.clone()).is_some_and(|previous| previous != expected) {
                return Err(freeze("print-expression/path-root-source"));
            }
        }
        for access in self.frame.cores().values().filter(|core| self.node_selected(core.node()))
            .filter_map(|core| core.field_array_access_v1()) {
            let Some(alias) = access.receiver_alias_v1() else { continue; };
            let (field, _) = alias.fixed_source_v1().ok_or_else(|| freeze("print-expression/path-root-source"))?;
            let expected = (alias.binding(), field.site().clone());
            if roots.insert(access.receiver().clone(), expected.clone()).is_some_and(|previous| previous != expected) {
                return Err(freeze("print-expression/path-root-source"));
            }
        }
        Ok(roots)
    }


    /// Initializer producers precede the expression entry. They are captured
    /// dependencies, never additional instructions in its evaluation order.
    #[cfg(test)]
    pub(in crate::mir) fn validate_original_reference_dependencies(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_reference_dependencies(context, function, None)
    }

    #[cfg(test)]
    pub(in crate::mir) fn assert_reference_finishing_for_test(&self, function: &MirFunction) {
        if self.reference_reads.is_empty() {
            return;
        }
        // Close the common evaluation's open Normal block for this structural
        // boundary test; no terminal purpose or executable input is issued.
        let mut original = function.clone();
        for block in original.blocks.values_mut() {
            if block.terminator.is_none() {
                block.set_terminator(MirInstruction::Return { value: None });
            }
        }
        let bindings = self.original.to_vec();
        let boundary =
            super::super::physical_boundary::PhysicalBoundary::capture(&original, &bindings)
                .unwrap();
        let mut projection = boundary.project(&original).unwrap();
        boundary
            .validate_complete(&original, &mut projection, &bindings)
            .unwrap();
        self.reference_reads
            .values()
            .try_for_each(|read| read.validate_finished_dependencies_v1(&original, &projection))
            .unwrap();
        for observed in self.reference_reads.values() {
            for dependency in observed.dependencies().unwrap() {
                let mut changed = original.clone();
                changed
                    .blocks
                    .get_mut(&dependency.0)
                    .unwrap()
                    .instructions
                    .retain(|row| row != &dependency.1);
                assert!(
                    self.reference_reads
                        .values()
                        .try_for_each(
                            |read| read.validate_finished_dependencies_v1(&changed, &projection)
                        )
                        .is_err(),
                    "unapproved dependency removal must fail the final consumer"
                );
            }
        }
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn require_unique(
        &self,
        matches: impl Fn(&MirInstruction) -> bool,
    ) -> Result<&'a Binding, String> {
        let mut rows = self.original.iter().filter(|(_, row)| matches(row));
        let result = rows
            .next()
            .ok_or_else(|| freeze("print-expression/producer-missing"))?;
        if rows.next().is_some() {
            return Err(freeze("print-expression/producer-duplicate"));
        }
        Ok(result)
    }

    fn dynamic_add_original_v1(&self, index: usize, result: ValueId) -> Result<Binding, String> {
        use crate::mir::dynamic_operator_contract::{
            DynamicOperatorFamilyV1 as Family, DynamicOperatorNormalResultV1 as Normal,
            DynamicOperatorValueClassV1 as Class,
        };
        let source = self.frame.source();
        let node = source
            .nodes()
            .get(index)
            .ok_or_else(|| freeze("print-expression/dynamic-add-node"))?;
        let Node::Binary(binary) = node.source() else {
            return Err(freeze("print-expression/dynamic-add-node"));
        };
        let operand = |site| -> Result<Operand, String> {
            let value = source
                .nodes()
                .iter()
                .position(|node| node.site() == site)
                .and_then(|node| self.values[node].as_ref())
                .ok_or_else(|| freeze("print-expression/dynamic-add-operand"))?;
            Ok(value.as_operand()?.clone())
        };
        let (Operand::Dynamic(lhs), Operand::NormalInteger(rhs)) =
            (operand(binary.lhs())?, operand(binary.rhs())?)
        else {
            return Err(freeze("print-expression/dynamic-add-operands"));
        };
        if self
            .values
            .get(index)
            .and_then(Option::as_ref)
            .and_then(|value| value.as_operand().ok())
            != Some(&Operand::Dynamic(result))
        {
            return Err(freeze("print-expression/dynamic-add-result"));
        }
        let loan = self
            .frame
            .operations()
            .iter()
            .find(|loan| loan.node() == index && Rc::ptr_eq(loan.source(), source))
            .ok_or_else(|| freeze("print-expression/operator-source-missing"))?;
        let envelope = loan
            .single_envelope()
            .ok_or_else(|| freeze("print-expression/operator-envelope"))?;
        let domain = envelope.domain();
        if loan.normal_result_v1() != Some(Normal::SelfContainedNonAliasingDynamicCarrier)
            || domain.family() != Family::Add
            || domain.left() != Class::Dynamic
            || domain.right() != Class::I64
            || self.frame.owned_result_nodes().contains(&index)
        {
            return Err(freeze("print-expression/dynamic-add-contract"));
        }
        Ok(self
            .require_unique(|row| {
                matches!(row, MirInstruction::BinOp {
            dst, op: crate::mir::BinaryOp::Add, lhs: actual_lhs, rhs: actual_rhs,
        } if *dst == result && *actual_lhs == lhs && *actual_rhs == rhs)
            })?
            .clone())
    }

    fn dynamic_sub_original_v1(
        &self,
        index: usize,
        result: ValueId,
        carrier: ValueId,
        rhs: ValueId,
    ) -> Result<[Binding; 2], String> {
        use crate::mir::dynamic_operator_contract::{
            DynamicOperatorFamilyV1 as Family,
            DynamicOperatorNormalResultV1 as Normal,
            DynamicOperatorValueClassV1 as Class,
        };
        let source = self.frame.source();
        let node = source.nodes().get(index)
            .ok_or_else(|| freeze("print-expression/dynamic-sub-node"))?;
        let Node::Binary(binary) = node.source() else {
            return Err(freeze("print-expression/dynamic-sub-node"));
        };
        let operand = |site| -> Result<Operand, String> {
            let value = source.nodes().iter().position(|node| node.site() == site)
                .and_then(|node| self.values[node].as_ref())
                .ok_or_else(|| freeze("print-expression/dynamic-sub-operand"))?;
            Ok(value.as_operand()?.clone())
        };
        if operand(binary.lhs())? != Operand::Dynamic(carrier)
            || operand(binary.rhs())? != Operand::NormalInteger(rhs)
            || self.values.get(index).and_then(Option::as_ref)
                .and_then(|value| value.as_operand().ok()) != Some(&Operand::DynamicCarrier(result))
        {
            return Err(freeze("print-expression/dynamic-sub-operands"));
        }
        let loan = self.frame.operations().iter()
            .find(|loan| loan.node() == index && Rc::ptr_eq(loan.source(), source))
            .ok_or_else(|| freeze("print-expression/dynamic-sub-source"))?;
        let envelope = loan.single_envelope()
            .ok_or_else(|| freeze("print-expression/dynamic-sub-envelope"))?;
        let domain = envelope.domain();
        if loan.normal_result_v1() != Some(Normal::SelfContainedNonAliasingDynamicCarrier)
            || domain.family() != Family::Sub
            || domain.left() != Class::Dynamic
            || domain.right() != Class::I64
            || !self.frame.owned_result_nodes().contains(&index)
        {
            return Err(freeze("print-expression/dynamic-sub-contract"));
        }
        let projection = self.require_unique(|row|
            matches!(row, MirInstruction::InvokeNormalResult { dst, .. } if *dst == result))?.clone();
        let MirInstruction::InvokeNormalResult { invoke_block, .. } = projection.1 else {
            return Err(freeze("print-expression/dynamic-sub-projection"));
        };
        let invoke = self.require_unique(|row|
            matches!(row, MirInstruction::Invoke {
                operation: InvokeOperation::DynamicCarrierSubI64 { carrier: actual_carrier, rhs: actual_rhs },
                normal_landing,
                ..
            } if *actual_carrier == carrier && *actual_rhs == rhs && *normal_landing == projection.0)
        )?.clone();
        if invoke.0 != invoke_block || invoke.0 == projection.0 {
            return Err(freeze("print-expression/dynamic-sub-invoke"));
        }
        Ok([invoke, projection])
    }


    /// LocalSSA operand materializations are dependencies of the checked
    /// scalar operation. Keep them in the enclosing scope inventory without
    /// treating earlier source definitions as expression evaluation steps.
    pub(in crate::mir) fn scalar_copy_dependencies_v1(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let mut copies = Vec::new();
        for (index, _) in self
            .frame
            .source()
            .nodes()
            .iter()
            .enumerate()
            .filter(|(index, _)| self.node_selected(*index))
        {
            let Some(scalar) = self
                .frame
                .operations()
                .iter()
                .find(|loan| loan.node() == index)
                .and_then(|loan| loan.scalar_source_v1())
            else {
                continue;
            };
            let original = context.ledger().scalar_operation_original_v1(scalar)?;
            for relation in original.operand_relations() {
                let mut observed = Vec::new();
                relation.append_observed_bindings_v1(&mut observed);
                for binding in observed
                    .into_iter()
                    .filter(|(_, instruction)| matches!(instruction, MirInstruction::Copy { .. }))
                {
                    if !copies.contains(&binding) {
                        copies.push(binding);
                    }
                }
            }
        }
        Ok(copies)
    }


}

/// Check actual instruction order along the single Normal expression path.
/// This does not infer evaluation order from block allocation or SSA numbers.
pub(super) fn normal_order(function: &MirFunction, ordered: &[Binding]) -> Result<(), String> {
    let mut previous = None;
    let mut closed = std::collections::BTreeSet::new();
    for (id, instruction) in ordered {
        let block = function
            .blocks
            .get(id)
            .ok_or_else(|| freeze("print-expression/ordered-block"))?;
        let mut positions = block
            .all_instructions()
            .enumerate()
            .filter(|(_, row)| *row == instruction);
        let index = positions
            .next()
            .ok_or_else(|| freeze("print-expression/ordered-instruction"))?
            .0;
        if positions.next().is_some() {
            return Err(freeze("print-expression/ordered-duplicate"));
        }
        if let Some((mut before, previous_index, previous_was_invoke)) = previous {
            if before == *id {
                if previous_index >= index {
                    return Err(freeze("print-expression/source-order"));
                }
            } else {
                let initial = before;
                if closed.contains(id) {
                    return Err(freeze("print-expression/normal-cycle"));
                }
                while before != *id {
                    if !closed.insert(before) {
                        return Err(freeze("print-expression/normal-cycle"));
                    }
                    before = match super::super::root_cleanup_graph::ordered_paths::terminal(
                        function, before,
                    )? {
                        MirInstruction::Jump {
                            target,
                            edge_args: None,
                        } => *target,
                        MirInstruction::Invoke { normal_landing, .. }
                            if before == initial && previous_was_invoke =>
                        {
                            *normal_landing
                        }
                        _ => return Err(freeze("print-expression/source-order")),
                    };
                }
            }
        }
        previous = Some((
            *id,
            index,
            matches!(instruction, MirInstruction::Invoke { .. }),
        ));
    }
    Ok(())
}

/// `None` checks the completed evaluation; a Call boundary checks exactly the
/// acquisitions preceding that SAME source observation in original postorder.
pub(in crate::mir) fn validate_expression_acquisition_prefix_v1(
    frame: &CheckedExpressionEvaluationV1,
    before: Option<&crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1>,
    acquired: &[(usize, ValueId)],
    values: &[Option<EvaluatedExpressionValueV1>],
    function: &MirFunction,
) -> Result<(), String> {
    validate_expression_acquisition_scope_v1(frame, None, before, acquired, values, function)
}

pub(in crate::mir) fn validate_expression_acquisition_scope_v1(
    frame: &CheckedExpressionEvaluationV1,
    comparison: Option<usize>,
    before: Option<&crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1>,
    acquired: &[(usize, ValueId)],
    values: &[Option<EvaluatedExpressionValueV1>],
    function: &MirFunction,
) -> Result<(), String> {
    if values.len() != frame.source().nodes().len() {
        return Err(freeze("expression-call/value-coverage"));
    }
    let order = match comparison {
        Some(node) => frame
            .scalar_invoke_input_order_v1(node)
            .ok_or_else(|| freeze("bool-condition/text-comparison-source"))?,
        None => frame.ordered_nodes().to_vec(),
    };
    let end = if let Some(call) = before {
        if !frame
            .calls()
            .iter()
            .any(|original| std::ptr::eq(original, call))
        {
            return Err(freeze("expression-call/original-observation"));
        }
        order
            .iter()
            .position(|node| frame.source().nodes()[*node].site() == call.site().site())
            .ok_or_else(|| freeze("expression-call/source-node"))?
    } else {
        order.len()
    };
    let expected = order[..end].iter().copied().filter(|node| {
        frame.owned_result_nodes().contains(node)
            // Text acquisitions and opaque carriers have distinct lifetime
            // pools. Dynamic results remain checked by the owned-flow owner.
            && !frame.cores().get(node).is_some_and(|core| core.is_array_dynamic_get_v1())
            && !frame.operations().iter().any(|operation| operation.node() == *node
                && operation.normal_result_v1() == Some(crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1::SelfContainedNonAliasingDynamicCarrier))
            && !frame.calls().iter().any(|call| {
                call.site().site() == frame.source().nodes()[*node].site()
                    && call.dynamic_invocation_result_lifecycle_v1().is_some()
            })
    });
    if acquired.iter().map(|(node, _)| *node).ne(expected.clone()) {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[acquisition-prefix] function={} acquired={:?} expected={:?} owned_results={:?}", function.signature.name,
                acquired.iter().map(|(node, _)| *node).collect::<Vec<_>>(), expected.collect::<Vec<_>>(), frame.owned_result_nodes());
        }
        return Err(freeze("expression-call/acquisition-prefix"));
    }
    let mut seen_values = std::collections::BTreeSet::new();
    for (node, value) in acquired {
        if values.get(*node) != Some(&Some(Operand::OwnedString(*value).into())) {
            return Err(freeze("expression-call/acquisition-node-value"));
        }
        if !seen_values.insert(*value) || function.params.contains(value) {
            return Err(freeze("expression-call/duplicate-acquisition"));
        }
        let mut producers = function
            .blocks
            .iter()
            .flat_map(|(id, block)| block.all_instructions().map(move |row| (*id, row)))
            .filter(|(_, row)| row.dst_value() == Some(*value));
        let Some((landing, MirInstruction::InvokeNormalResult { invoke_block, .. })) =
            producers.next()
        else {
            return Err(freeze("expression-call/acquisition-projection"));
        };
        if producers.next().is_some()
            || !matches!(function.blocks.get(invoke_block)
            .and_then(|block| block.terminator.as_ref()),
            Some(MirInstruction::Invoke { operation, normal_landing, .. })
                if operation.normal_result_kind() == Some(crate::mir::instruction::InvokeNormalResultKind::OwnedString) && *normal_landing == landing)
        {
            return Err(freeze("expression-call/acquisition-producer"));
        }
    }
    Ok(())
}
