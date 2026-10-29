//! One source-derived scalar terminal plan for emission and independent validation.
use super::*;

/// Central terminal operation projection shared by the Bool writer and both
/// final source consumers. Bool results never enter an Integer/Text operand.
fn checked_scalar_base_operation_v1(
    frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
    node: usize,
    values: &[Option<EvaluatedExpressionValueV1>],
    receiver: Option<&crate::mir::builder::ExactLexicalReadV1>,
) -> Result<InvokeOperation, String> {
    use crate::mir::instruction::{
        StringInvokeOperand as Operand, StringInvokeOperation as Operation,
    };
    if values.len() != frame.source().nodes().len() {
        return Err(freeze("bool-condition/text-comparison-source"));
    }
    if let Some(core) = frame.cores().get(&node) {
        if frame.scalar_invoke_input_order_v1(node).is_none() {
            return Err(freeze("bool-condition/text-comparison-source"));
        }
        let field = core
            .field_map()
            .ok_or_else(|| freeze("expression-core/source-identity"))?;
        let read = receiver.ok_or_else(|| freeze("expression-core/source-identity"))?;
        let base = read
            .value_for(
                frame.source().owner(),
                field.receiver_site().node(),
                field.receiver(),
            )
            .map_err(|error| error.to_string())?;
        let [argument] = core.source().source().arguments() else {
            return Err(freeze("expression-core/operand-domain"));
        };
        let key = frame
            .source()
            .nodes()
            .iter()
            .position(|row| row.site() == argument.site())
            .and_then(|node| values[node].as_ref())
            .map(|value| value.as_operand())
            .transpose()?
            .cloned()
            .filter(Operand::is_text)
            .ok_or_else(|| freeze("expression-core/operand-domain"))?;
        return Ok(InvokeOperation::Map(
            crate::mir::instruction::MapInvokeOperation::FieldHas {
                field: field.field(),
                base,
                key,
            },
        ));
    }
    if receiver.is_some() {
        return Err(freeze("expression-core/source-identity"));
    }
    frame.text_comparison_operation_v1(node, true)
        .ok_or_else(|| freeze("bool-condition/text-comparison-source"))?;
    let binary = frame.source().nodes()[node]
        .binary()
        .ok_or_else(|| freeze("bool-condition/text-comparison-source"))?;
    let operand = |site| {
        use crate::mir::resolved_value_profile::{
            expression_class::ExpressionValueClassV1 as Class,
            expression_projection::ExpressionLeafClassV1 as Leaf,
        };
        let index = frame
            .source()
            .nodes()
            .iter()
            .position(|row| row.site() == site)
            .ok_or_else(|| freeze("bool-condition/text-operand-order"))?;
        let value = values[index]
            .as_ref()
            .map(|value| value.as_operand())
            .transpose()?
            .cloned()
            .ok_or_else(|| freeze("bool-condition/text-operand-order"))?;
        match (frame.class(index), &value) {
            (Some(Leaf::Value(Class::String)), operand) if operand.is_text() => Ok(value),
            (Some(Leaf::Value(Class::Dynamic)), Operand::Dynamic(value))
                if frame.dynamic_local_carrier_read_v1(index) =>
            {
                Ok(Operand::DynamicCarrier(*value))
            }
            (Some(Leaf::Value(Class::Dynamic)), Operand::DynamicCarrier(_))
                if frame.owned_result_nodes().contains(&index)
                    && frame.calls().iter().any(|call| {
                        frame.source().nodes().get(index).is_some_and(|source| {
                            call.site().site() == source.site()
                                && call.dynamic_invocation_result_lifecycle_v1().is_some()
                        })
                    }) => Ok(value),
            _ => Err(freeze("bool-condition/text-operand-order")),
        }
    };
    use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1 as Op;
    let op = match binary.operator() {
        Op::Equal => crate::mir::CompareOp::Eq,
        Op::NotEqual => crate::mir::CompareOp::Ne,
        Op::Less => crate::mir::CompareOp::Lt,
        Op::LessEqual => crate::mir::CompareOp::Le,
        Op::Greater => crate::mir::CompareOp::Gt,
        Op::GreaterEqual => crate::mir::CompareOp::Ge,
        _ => return Err(freeze("bool-condition/text-operator")),
    };
    Ok(InvokeOperation::String(Operation::Compare {
        op,
        left: operand(binary.lhs())?,
        right: operand(binary.rhs())?,
    }))
}

pub(crate) struct CheckedScalarConditionOperationV1 {
    operation: InvokeOperation,
    comparison: Option<(crate::mir::CompareOp, ValueId, bool)>,
}
impl CheckedScalarConditionOperationV1 {
    pub(crate) fn operation(&self) -> &InvokeOperation { &self.operation }
    pub(crate) fn is_integer_comparison(&self) -> bool { self.comparison.is_some() }
    pub(crate) fn comparison_instruction(&self, integer: ValueId, result: ValueId) -> Option<MirInstruction> {
        self.comparison.map(|(op, other, carrier_left)| MirInstruction::Compare {
            dst: result, op, lhs: if carrier_left { integer } else { other },
            rhs: if carrier_left { other } else { integer },
        })
    }
}

pub(crate) fn checked_scalar_invoke_operation_v1(
    frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
    node: usize, values: &[Option<EvaluatedExpressionValueV1>],
    receiver: Option<&crate::mir::builder::ExactLexicalReadV1>,
) -> Result<CheckedScalarConditionOperationV1, String> {
    if let Some((carrier_node, integer_node, carrier_left)) = frame.carrier_integer_comparison_nodes_v1(node) {
        use crate::mir::instruction::StringInvokeOperand as Operand;
        if receiver.is_some() || values.len() != frame.source().nodes().len() {
            return Err(freeze("bool-condition/text-comparison-source"));
        }
        let operand = |node: usize| values.get(node).and_then(Option::as_ref)
            .and_then(|value| value.as_operand().ok()).ok_or_else(|| freeze("bool-condition/text-operand-order"));
        let Operand::Dynamic(carrier) = operand(carrier_node)? else {
            return Err(freeze("bool-condition/text-operand-order"));
        };
        let Operand::NormalInteger(other) = operand(integer_node)? else {
            return Err(freeze("bool-condition/text-operand-order"));
        };
        let op = match frame.source().nodes()[node].binary().map(|binary| binary.operator()) {
            Some(crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Greater) => crate::mir::CompareOp::Gt,
            Some(crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Less) => crate::mir::CompareOp::Lt,
            _ => return Err(freeze("bool-condition/text-operator")),
        };
        return Ok(CheckedScalarConditionOperationV1 {
            operation: InvokeOperation::DynamicCarrierInteger { carrier: *carrier },
            comparison: Some((op, *other, carrier_left)),
        });
    }
    Ok(CheckedScalarConditionOperationV1 {
        operation: checked_scalar_base_operation_v1(frame, node, values, receiver)?, comparison: None,
    })
}

/// The same source plan checks either a direct Bool Invoke or an Integer
/// extraction followed by the original comparison in that Normal landing.
pub(crate) fn scalar_condition_bindings_v1(
    frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
    node: usize, values: &[Option<EvaluatedExpressionValueV1>],
    receiver: Option<&crate::mir::builder::ExactLexicalReadV1>,
    result: ValueId, bindings: &[Binding],
) -> Result<Vec<Binding>, String> {
    let plan = checked_scalar_invoke_operation_v1(frame, node, values, receiver)?;
    let unique = |predicate: &dyn Fn(&Binding) -> bool| -> Result<&Binding, String> {
        let mut matching = bindings.iter().filter(|row| predicate(row));
        let row = matching.next().ok_or_else(|| freeze("bool-condition/text-compare-invoke"))?;
        if matching.next().is_some() { return Err(freeze("bool-condition/text-compare-duplicate")); }
        Ok(row)
    };
    let invoke = unique(&|(_, row)| matches!(row, MirInstruction::Invoke { operation, .. } if operation == plan.operation()))?;
    let MirInstruction::Invoke { normal_landing, .. } = invoke.1 else { unreachable!() };
    let normal = unique(&|(_, row)| matches!(row, MirInstruction::InvokeNormalResult { invoke_block, .. } if *invoke_block == invoke.0))?;
    let MirInstruction::InvokeNormalResult { dst, .. } = normal.1 else { unreachable!() };
    if normal.0 != normal_landing || normal.0 == invoke.0 { return Err(freeze("bool-condition/text-compare-projection")); }
    let mut ordered = vec![invoke.clone(), normal.clone()];
    if let Some(expected) = plan.comparison_instruction(dst, result) {
        let comparison = unique(&|(_, row)| row == &expected)?;
        if comparison.0 != normal.0 { return Err(freeze("bool-condition/text-compare-projection")); }
        ordered.push(comparison.clone());
    } else if dst != result {
        return Err(freeze("bool-condition/text-compare-projection"));
    }
    Ok(ordered)
}

impl CallValidationContextV1<'_> {
    /// Lend the original has Invoke, Normal and sealed true/false Branch.
    /// Matching source identity selects the group; MIR never discovers it.
    pub(in crate::mir::normal_callable_semantic_package) fn finished_map_presence_v1(
        self,
        source: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
        function: &MirFunction,
        finishing: &Finished,
    ) -> Result<Vec<Binding>, String> {
        self.with_local_call_group_tree_v2(source.owner(), |groups| {
            let mut found = None;
            for group in groups {
                let Some(packet) = group.bool_condition() else { continue };
                for node in &packet.nodes {
                    let Some(core) = packet.frame.evaluation().cores().get(&node.node) else { continue };
                    if !std::sync::Arc::ptr_eq(core.source().source(), source) { continue }
                    if found.is_some() || core.field_map().is_none()
                        || !core.matches_plan(packet.frame.source(), node.node)
                        || !matches!(packet.materialization, BoolConditionOuterV2::If(_)) {
                        return Err(freeze("bool-condition/reference-source"));
                    }
                    group.validate_recorded_in_context(self, function, Some(finishing))?;
                    let BoolConditionValueProofV1::ScalarInvocation {
                        input_values, receiver, bindings, ..
                    } = &node.proof else { return Err(freeze("bool-condition/source-proof-kind")) };
                    let (invoke, normal) = packet.text_comparison_producers_v1(
                        node, input_values, receiver.as_ref(), bindings,
                    )?;
                    let outer = packet.outer.borrow();
                    let outer = outer.as_ref().ok_or_else(|| freeze("bool-condition/required-producer-removed"))?;
                    packet.verify_outer(function, outer, Some(finishing))?;
                    let project = |row: &Binding| mapped(row, Some(finishing))?
                        .ok_or_else(|| freeze("bool-condition/required-producer-removed"));
                    let materialization = packet.materialization.physical();
                    if materialization.values().0 != node.value {
                        return Err(freeze("bool-condition/outer-branch-source"));
                    }
                    materialization.relation().require_completed_v1()?;
                    materialization.relation().verify_projected(function,
                        &mut |binding| mapped(binding, Some(finishing)))?;
                    let mut rows = vec![project(invoke)?, project(normal)?];
                    let mut value = rows[1].1.dst_value()
                        .ok_or_else(|| freeze("bool-condition/outer-branch-source"))?;
                    for original in materialization.bindings() {
                        if let Some(actual) = mapped(original, Some(finishing))? {
                            let MirInstruction::Copy { src, dst } = actual.1 else {
                                return Err(freeze("bool-condition/outer-branch-source"));
                            };
                            if src != value { return Err(freeze("bool-condition/outer-branch-source")); }
                            value = dst;
                            rows.push(actual);
                        }
                    }
                    let branch = project(outer)?;
                    if !matches!(&branch.1, MirInstruction::Branch { condition, .. } if *condition == value) {
                        return Err(freeze("bool-condition/outer-branch-source"));
                    }
                    rows.push(branch);
                    found = Some(rows);
                }
            }
            found.ok_or_else(|| freeze("bool-condition/reference-source"))
        })
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn visit_map_presence_groups_v1(
        &self,
        owner: FunctionOwnerIdV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        projection: &Finished,
        module: &crate::mir::MirModule,
        mut visit: impl FnMut(Binding) -> Result<(), String>,
    ) -> Result<(), String> {
        self.visit_verified_condition_groups_v1(owner, context, function, projection, |packet| {
            for node in &packet.nodes {
                let Some(core) = packet.frame.evaluation().cores().get(&node.node) else {
                    continue;
                };
                let Some(field) = core.field_map() else {
                    continue;
                };
                if let CallValidationContextV1::Finalized(source) = context {
                    field.require_finished_birth_v1(source, module)?;
                }
                let BoolConditionValueProofV1::ScalarInvocation {
                    input_values,
                    receiver,
                    bindings,
                    ..
                } = &node.proof
                else {
                    return Err(freeze("bool-condition/source-proof-kind"));
                };
                let (invoke, _) = packet.text_comparison_producers_v1(
                    node,
                    input_values,
                    receiver.as_ref(),
                    bindings,
                )?;
                visit(
                    projection
                        .binding(invoke.0, &invoke.1)?
                        .ok_or_else(|| freeze("bool-condition/required-producer-removed"))?,
                )?;
            }
            Ok(())
        })
    }
}
