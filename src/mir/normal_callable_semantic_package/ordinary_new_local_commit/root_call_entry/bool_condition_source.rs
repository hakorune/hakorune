//! Independent original source/loan to actual producer correspondence.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use crate::mir::resolved_semantics::{ResolvedLiteralSourceV1 as Literal, SourceExprSiteV1};

impl EmittedBoolConditionProjectionV1 {
    pub(super) fn validate_source(&self, context: CallValidationContextV1<'_>,
        function: &MirFunction, finishing: Option<&Finished>,
    ) -> Result<(), String> {
        self.materialization.validate_source(&self.frame, context)?;
        let ledger = context.ledger();
        let source = self.frame.source();
        let text_leaves = self.frame.text_comparison_input_nodes_v1();
        let expected: Vec<_> = self.frame.ordered_nodes().iter().copied().filter(|index|
            !text_leaves.contains(index) && !self.frame.transfers_node(*index) && self.frame.class(*index) != Some(
                crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Identity)).collect();
        if self.nodes.iter().map(|node| node.node).ne(expected)
            || self.node(0)?.value != self.materialization.physical().values().0 {
            return Err(freeze("bool-condition/source-node-coverage"));
        }
        let mut observed_calls = Vec::new();
        let mut observed_text = std::collections::BTreeSet::new();
        let mut observed_references = std::collections::BTreeSet::new();
        for node in &self.nodes {
            let original = &source.nodes()[node.node];
            match (original.source(), &node.proof) {
                (Node::Literal(literal), BoolConditionValueProofV1::Literal(binding)) => {
                    if !matches!((literal, &binding.1),
                        (Literal::Integer(expected), MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(actual) })
                            if *dst == node.value && expected == actual)
                        && !matches!((literal, &binding.1),
                        (Literal::Bool(expected), MirInstruction::Const { dst, value: crate::mir::ConstValue::Bool(actual) })
                            if *dst == node.value && expected == actual)
                    { return Err(freeze("bool-condition/literal-source")); }
                    // The captured finisher alone may remove a redundant Const.
                    if mapped(binding, finishing)?.is_some() { require_binding(function, binding, finishing)?; }
                },
                (Node::Reference(_), BoolConditionValueProofV1::Reference { read, producer }) => {
                    if !observed_references.insert(node.node) { return Err(freeze("bool-condition/reference-duplicate")); }
                    let Some(crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::Scalar(reference)) = self.frame.references().get(&node.node)
                        else { return Err(freeze("bool-condition/reference-source")); };
                    if reference.original_read().is_some() { return Err(freeze("bool-condition/reference-source")); }
                    let binding = reference.binding();
                    if read.value_for(source.owner(), original.site().node(), binding)
                        .map_err(|error| error.to_string())? != node.value {
                        return Err(freeze("bool-condition/reference-value"));
                    }
                    if let Some(producer) = producer {
                        if producer.1.dst_value() != Some(node.value) { return Err(freeze("bool-condition/reference-producer")); }
                        if mapped(producer, finishing)?.is_some() { require_binding(function, producer, finishing)?; }
                    } else if !function.params.contains(&node.value) {
                        return Err(freeze("bool-condition/reference-parameter"));
                    }
                },
                (Node::Reference(_), BoolConditionValueProofV1::OriginalReference(read)) => {
                    if !observed_references.insert(node.node) { return Err(freeze("bool-condition/reference-duplicate")); }
                    let Some(crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::Scalar(reference)) = self.frame.references().get(&node.node)
                        else { return Err(freeze("bool-condition/reference-source")); };
                    if reference.original_read().is_none()
                        || read.operand(self.frame.evaluation(), node.node, context)?
                            != crate::mir::instruction::StringInvokeOperand::NormalInteger(node.value) {
                        return Err(freeze("bool-condition/original-reference-value"));
                    }
                    read.validate_dependencies_v1(self.frame.evaluation(), node.node, context, function, finishing)?;
                },
                (Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }),
                    BoolConditionValueProofV1::Field(binding)) => {
                    if ledger.expression_field_read_emission_v1(
                        &OwnedExprSiteV1::new(source.owner(), original.site().clone()), source, context)? != *binding
                        || binding.1.dst_value() != Some(node.value) {
                        return Err(freeze("bool-condition/field-source"));
                    }
                    require_binding(function, binding, finishing)?;
                },
                (Node::MethodCall, BoolConditionValueProofV1::Call { packet, bindings }) => {
                    let observation = self.frame.calls().iter().find(|call| call.site().site() == original.site())
                        .ok_or_else(|| freeze("bool-condition/call-source"))?;
                    if packet.call_site() != observation.site()
                        || ledger.lexical_i64_call_source(packet.call_site()) != Some(observation)
                        || packet.value_in_context_v1(source.owner(), observation.arguments(), context)? != node.value {
                        return Err(freeze("bool-condition/call-value"));
                    }
                    observed_calls.push(observation.site());
                    let projected = bindings.iter().map(|binding| mapped(binding, finishing))
                        .filter_map(|row| match row { Ok(Some(row)) => Some(Ok(row)), Ok(None) => None, Err(error) => Some(Err(error)) })
                        .collect::<Result<Vec<_>, String>>()?;
                    packet.validate_recorded_projected(&projected, finishing)?;
                },
                (Node::Binary(binary), BoolConditionValueProofV1::Binary { append, operands }) => {
                    let loan = self.frame.operations().iter().find(|loan| loan.node() == node.node)
                        .ok_or_else(|| freeze("bool-condition/operator-loan"))?;
                    if !Rc::ptr_eq(loan.source(), source) { return Err(freeze("bool-condition/operator-source")); }
                    let lhs = self.value_at(binary.lhs())?;
                    let rhs = self.value_at(binary.rhs())?;
                    let expected = match binary.operator() {
                        ResolvedBinaryOperatorV1::Equal => Some(crate::mir::CompareOp::Eq),
                        ResolvedBinaryOperatorV1::NotEqual => Some(crate::mir::CompareOp::Ne),
                        ResolvedBinaryOperatorV1::Less => Some(crate::mir::CompareOp::Lt),
                        ResolvedBinaryOperatorV1::LessEqual => Some(crate::mir::CompareOp::Le),
                        ResolvedBinaryOperatorV1::Greater => Some(crate::mir::CompareOp::Gt),
                        ResolvedBinaryOperatorV1::GreaterEqual => Some(crate::mir::CompareOp::Ge),
                        ResolvedBinaryOperatorV1::Add => None,
                        _ => return Err(freeze("bool-condition/operator-kind")),
                    };
                    let (a, b) = match &append.1 {
                        MirInstruction::Compare { dst, op, lhs, rhs } if *dst == node.value && Some(*op) == expected => (*lhs, *rhs),
                        MirInstruction::BinOp { dst, op: crate::mir::BinaryOp::Add, lhs, rhs }
                            if *dst == node.value && binary.operator() == ResolvedBinaryOperatorV1::Add => (*lhs, *rhs),
                        _ => return Err(freeze("bool-condition/operator-append")),
                    };
                    if operands[0].values() != (lhs, a) || operands[1].values() != (rhs, b) {
                        return Err(freeze("bool-condition/ordered-operands"));
                    }
                    for relation in operands {
                        relation.verify_projected(function, &mut |binding| mapped(binding, finishing))?;
                    }
                    require_binding(function, append, finishing)?;
                },
                (Node::Binary(binary), BoolConditionValueProofV1::Logical { completed, conditions }) => {
                    self.validate_logical(node, binary, completed, conditions, function, finishing)?;
                },
                (Node::Binary(_), BoolConditionValueProofV1::TextComparison { operands, bindings, input_values,
                    input_reads, entry, terminal, continuation }) => {
                    self.text_comparison_producers_v1(node, operands, bindings)?;
                    let inputs = self.frame.evaluation().text_comparison_input_order_v1(node.node)
                        .ok_or_else(|| freeze("bool-condition/text-comparison-source"))?;
                    for input in inputs {
                        if !observed_text.insert(input) { return Err(freeze("bool-condition/text-leaf-duplicate")); }
                    }
                    for operand in operands {
                        if input_values.get(operand.node) != Some(&Some(operand.operand.clone())) {
                            return Err(freeze("bool-condition/text-operand-order"));
                        }
                    }
                    for &input in input_reads.keys() {
                        if !observed_references.insert(input) { return Err(freeze("bool-condition/reference-duplicate")); }
                    }
                    let projection = self.comparison_inputs_v1(node)?;
                    projection.validate_scalar_continuation_v1(context, function, finishing,
                        entry, terminal, continuation, None)?;
                    for binding in bindings.iter() {
                        if mapped(binding, finishing)?.is_none() && matches!(binding.1,
                            MirInstruction::Jump { .. } | MirInstruction::Const { .. } | MirInstruction::Copy { .. }) { continue; }
                        require_binding(function, binding, finishing)?;
                    }
                },
                _ => return Err(freeze("bool-condition/source-proof-kind")),
            }
        }
        if observed_text != text_leaves || observed_references.iter().copied().ne(self.frame.references().keys().copied()) {
            return Err(freeze("bool-condition/reference-leaf-coverage"));
        }
        if observed_calls.into_iter().ne(self.frame.calls().iter().map(|call| call.site())) {
            return Err(freeze("bool-condition/call-coverage"));
        }
        self.materialization.physical().relation().verify_projected(function, &mut |binding| mapped(binding, finishing))?;
        for binding in self.materialization.physical().bindings() {
            match mapped(binding, finishing)? {
                Some(_) => require_binding(function, binding, finishing)?,
                None if matches!(binding.1, MirInstruction::Const { .. } | MirInstruction::Copy { .. }) => {},
                None => return Err(freeze("bool-condition/materialization-producer-removed")),
            }
        }
        Ok(())
    }
    pub(super) fn value_at(&self, site: &SourceExprSiteV1) -> Result<ValueId, String> {
        let index = self.frame.source().nodes().iter().position(|node| node.site() == site)
            .ok_or_else(|| freeze("bool-condition/child-source"))?;
        Ok(self.node(index)?.value)
    }

    pub(super) fn text_comparison_producers_v1<'a>(&self, node: &BoolConditionNodeProjectionV1,
        operands: &[BoolConditionTextOperandV1; 2], bindings: &'a [Binding],
    ) -> Result<(&'a Binding, &'a Binding), String> {
        let operation = BoolConditionTextOperandV1::comparison_operation_v1(&self.frame, node.node, operands)?;
        let mut invokes = bindings.iter().filter(|(_, row)| matches!(row,
            MirInstruction::Invoke { operation: InvokeOperation::String(actual), .. } if actual == &operation));
        let invoke = invokes.next().ok_or_else(|| freeze("bool-condition/text-compare-invoke"))?;
        if invokes.next().is_some() { return Err(freeze("bool-condition/text-compare-duplicate")); }
        let MirInstruction::Invoke { normal_landing, .. } = invoke.1 else { unreachable!() };
        let mut results = bindings.iter().filter(|(id, row)| *id == normal_landing && matches!(row,
            MirInstruction::InvokeNormalResult { invoke_block, dst } if *invoke_block == invoke.0 && *dst == node.value));
        let result = results.next().ok_or_else(|| freeze("bool-condition/text-compare-projection"))?;
        if results.next().is_some() || invoke.0 == result.0 { return Err(freeze("bool-condition/text-compare-projection-duplicate")); }
        Ok((invoke, result))
    }
}
