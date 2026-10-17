//! Independent original source/loan to actual producer correspondence.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use crate::mir::resolved_semantics::{ResolvedLiteralSourceV1 as Literal, SourceExprSiteV1};

impl EmittedBoolConditionProjectionV1 {
    pub(super) fn validate_source(&self, context: CallValidationContextV1<'_>,
        function: &MirFunction, finishing: Option<&Finished>,
    ) -> Result<(), String> {
        let ledger = context.ledger();
        let source = self.frame.source();
        let expected: Vec<_> = self.frame.ordered_nodes().iter().copied().filter(|index|
            !self.frame.transfers_node(*index) && self.frame.class(*index) != Some(
                crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Identity)).collect();
        if self.nodes.iter().map(|node| node.node).ne(expected)
            || self.node(0)?.value != self.materialization.values().0 {
            return Err(freeze("bool-condition/source-node-coverage"));
        }
        let mut observed_calls = Vec::new();
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
                    let &(binding, _) = self.frame.references().get(&node.node)
                        .ok_or_else(|| freeze("bool-condition/reference-source"))?;
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
                _ => return Err(freeze("bool-condition/source-proof-kind")),
            }
        }
        if observed_calls.into_iter().ne(self.frame.calls().iter().map(|call| call.site())) {
            return Err(freeze("bool-condition/call-coverage"));
        }
        self.materialization.relation().verify_projected(function, &mut |binding| mapped(binding, finishing))?;
        for binding in self.materialization.bindings() {
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
}
