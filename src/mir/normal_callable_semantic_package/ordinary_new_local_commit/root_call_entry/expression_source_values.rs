//! Original source/value correspondence and checked core-operation projection.
use super::*;
use std::ops::ControlFlow;
use crate::mir::core_method_result_kind::CoreMethodSemanticLawV2 as Law;

impl<'a> ExpressionPhysicalProjectionRefV1<'a> {
    pub(in crate::mir) fn validate_source(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        let ledger = context.ledger();
        let source = self.frame.source();
        if !self.frame.reference_coverage_is_exact()
            || self.values.len() != source.nodes().len()
            || self.calls.len() != self.source_calls().count()
            || self.dynamic_calls.len() != self.source_dynamic_calls().count()
        {
            return Err(freeze("print-expression/source-coverage"));
        }
        if self.reference_reads.len()
            != self
                .frame
                .references()
                .keys()
                .filter(|node| self.node_selected(**node))
                .count()
            || self.reference_reads.keys().any(|node| {
                !self.node_selected(*node) || !self.frame.references().contains_key(node)
            })
            || self
                .values
                .iter()
                .enumerate()
                .any(|(node, value)| !self.node_selected(node) && value.is_some())
        {
            return Err(freeze("expression/reference-read-census"));
        }
        let roots = self.original_field_roots_v1(context)?;
        if roots.len() != self.path_reads.len() {
            return Err(freeze("print-expression/path-root-census"));
        }
        let mut seen = std::collections::BTreeSet::new();
        for read in self.path_reads {
            let (site, _, _) = read.source_value();
            let (binding, first) = roots
                .get(&site)
                .ok_or_else(|| freeze("print-expression/path-root-source"))?;
            if !seen.insert(site.clone()) {
                return Err(freeze("print-expression/path-root-duplicate"));
            }
            let value = read
                .value_for(source.owner(), site.site().node(), *binding)
                .map_err(|error| error.to_string())?;
            if let Some(access) = self.frame.cores().values().filter_map(|core| core.field_array_access_v1())
                .find(|access| access.receiver_alias_v1().is_some() && access.receiver() == &site) {
                let (actual, _) = ledger.array_alias_operation_read_v1(access, read, context)?;
                let node = source.nodes().iter().position(|node| node.site() == site.site())
                    .ok_or_else(|| freeze("print-expression/path-root-source"))?;
                if actual != value || self.values[node] != Some(Operand::NormalInteger(value).into()) {
                    return Err(freeze("print-expression/path-root-value"));
                }
                continue;
            }
            ledger.validate_statement_field_root_read_v1(first, read, context)?;
            let emitted = ledger.expression_field_read_bindings_v1(first, source, context)?;
            let mut operations = emitted.iter().filter_map(|(_, row)| match row {
                MirInstruction::ObjectFieldGet { base, .. }
                | MirInstruction::Invoke {
                    operation: InvokeOperation::FieldGet { base, .. }
                        | InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::FieldMaterialize { base, .. }),
                    ..
                } => Some(*base),
                _ => None,
            });
            if operations.next() != Some(value) || operations.next().is_some() {
                if crate::config::env::builder_debug_enabled() {
                    eprintln!("[path-root-value] root value {value:?}; emitted field read rows: {emitted:?}");
                }
                return Err(freeze("print-expression/path-root-value"));
            }
        }
        for (observed, packet) in self.source_calls().zip(self.calls()) {
            if packet.call_site() != observed.site()
                || (ledger
                    .local_call_for_owner(packet.call_site().owner(), packet.call_site().site())
                    != Some(observed)
                    && !packet
                        .original_source()
                        .retains_expression_observation(observed))
                || observed.prior_homes() != self.frame.prior_homes()
            {
                return Err(freeze("print-expression/call-source"));
            }
            let index = source
                .nodes()
                .iter()
                .position(|node| node.site() == observed.site().site())
                .ok_or_else(|| freeze("print-expression/call-node"))?;
            let value =
                packet.value_in_context_v1(source.owner(), observed.arguments(), context)?;
            let expected = match observed.result() {
                crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Text
                    if packet.original_source().result()
                        == Some(crate::mir::instruction::InvokeCallResultKind::OwnedString)
                        && self.frame.owned_result_nodes().contains(&index) =>
                {
                    Operand::OwnedString(value)
                }
                crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64 => {
                    Operand::NormalInteger(value)
                }
                crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Dynamic
                    if observed.dynamic_invocation_source_v1().is_none()
                        && observed.dynamic_invocation_result_lifecycle_v1().is_some()
                        && packet.original_source().result() == Some(crate::mir::instruction::InvokeCallResultKind::DynamicCarrier)
                        && self.frame.owned_result_nodes().contains(&index) => Operand::DynamicCarrier(value),
                _ => return Err(freeze("print-expression/call-value")),
            };
            if self.values[index] != Some(expected.into()) {
                return Err(freeze("print-expression/call-value"));
            }
        }
        for (observed, packet) in self.source_dynamic_calls().zip(self.dynamic_calls()) {
            if observed.prior_homes() != self.frame.prior_homes() {
                return Err(freeze("dynamic-expression/prior-homes"));
            }
            packet.validate_source_v1(self.frame, self.values, observed, context, self.original)?;
        }
        let find_value = |site| {
            source
                .nodes()
                .iter()
                .position(|node| node.site() == site)
                .and_then(|index| self.values[index].as_ref())
                .map(|value| value.as_operand().cloned())
                .transpose()?
                .ok_or_else(|| freeze("print-expression/operand-missing"))
        };
        for &index in self
            .frame
            .ordered_nodes()
            .iter()
            .filter(|node| self.node_selected(**node))
        {
            let node = &source.nodes()[index];
            let value = self.values[index].as_ref();
            let alias_receiver = self.frame.cores().values().filter(|core| self.node_selected(core.node()))
                .filter_map(|core| core.field_array_access_v1()).any(|access|
                    access.receiver_alias_v1().is_some() && access.receiver().site() == node.site());
            let direct_receiver = self.frame.direct_array_receiver_v1(index)
                .is_some_and(|core| self.node_selected(core.node()));
            if self.frame.transfers_node(index) && !alias_receiver && !direct_receiver {
                if value.is_some() {
                    return Err(freeze("print-expression/transferred-value"));
                }
                continue;
            }
            if let Some(operation) = self.frame.operations().iter().find(|loan| loan.node() == index && loan.scalar_copy_v1().is_some()) {
                let (operand, kind) = operation.scalar_copy_v1().ok_or_else(|| freeze("expression/copy-contract"))?;
                let Some(input) = self.values[operand].as_ref() else { return Err(freeze("expression/copy-contract")); };
                if !matches!((kind, input, value),
                    (crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer, EvaluatedExpressionValueV1::Operand(Operand::NormalInteger(_)), Some(EvaluatedExpressionValueV1::Operand(Operand::NormalInteger(_)))) |
                    (crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool, EvaluatedExpressionValueV1::Bool(_) | EvaluatedExpressionValueV1::Operand(Operand::NormalInteger(_)), Some(EvaluatedExpressionValueV1::Bool(_)))) {
                    return Err(freeze("expression/copy-contract"));
                }
                self.require_unique(|row| matches!(row, MirInstruction::Copy { dst, src } if Some(*dst) == value.as_ref().and_then(|value| value.value()) && Some(*src) == input.value()))?;
                continue;
            }
            if let Some(tail) = source.transparent_tail_node_v1(index) {
                if value.is_none() || self.values[index] != self.values[tail] {
                    return Err(freeze("print-expression/node-value"));
                }
                continue;
            }
            if let (Node::Reference(_), Some(EvaluatedExpressionValueV1::DeclaredRead(value))) = (node.source(), value) {
                let Some(crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::NullableResult(
                    crate::mir::resolved_semantics::home_new_prefix::NullableResultReferenceV1::DeclaredFrom(_))) = self.frame.references().get(&index) else {
                    return Err(freeze("expression/reference-source-node"));
                };
                let read = self.reference_reads.get(&index).ok_or_else(|| freeze("expression/reference-source-node"))?;
                if read.declared_read_value_v1(self.frame, index, context)? != *value {
                    return Err(freeze("expression/reference-source-node"));
                }
                continue;
            }
            if let (
                Node::Literal(Literal::Bool(expected)),
                Some(EvaluatedExpressionValueV1::Bool(dst)),
            ) = (node.source(), value)
            {
                self.require_unique(|row| matches!(row, MirInstruction::Const { dst: actual, value: crate::mir::ConstValue::Bool(value) }
                    if actual == dst && value == expected))?;
                continue;
            }
            if let (Node::Binary(_), Some(EvaluatedExpressionValueV1::Bool(result))) = (node.source(), value) {
                scalar_condition_bindings_v1(self.frame, index, self.values, None, *result, self.original)?;
                continue;
            }
            let value = value.map(|value| value.as_operand()).transpose()?;
            match (node.source(), value) {
                (Node::Literal(Literal::String(text)), Some(Operand::StaticText { utf8 }))
                    if utf8.as_str() == text.as_ref() => {}
                (Node::Reference(_), Some(Operand::NormalInteger(_))) if self.frame.node_is_identity(index)
                    && self.frame.cores().values().filter_map(|core| core.field_array_access_v1())
                        .any(|access| access.receiver_alias_v1().is_some() && access.receiver().site() == node.site()) => {}
                (Node::Reference(_), Some(value)) => {
                    let observed = self
                        .reference_reads
                        .get(&index)
                        .ok_or_else(|| freeze("expression/reference-read-missing"))?;
                    if &observed.operand(self.frame, index, context)? != value {
                        return Err(freeze("expression/reference-source-value"));
                    }
                }
                (Node::Literal(Literal::Integer(expected)), Some(Operand::NormalInteger(dst))) => {
                    self.require_unique(|row| matches!(row, MirInstruction::Const { dst: actual, value: crate::mir::ConstValue::Integer(value) }
                        if actual == dst && value == expected))?;
                }
                (Node::MethodCall, Some(value)) if self.frame.cores().contains_key(&index) => {
                    let core = &self.frame.cores()[&index];
                    if !core.matches_plan(source, index) {
                        return Err(freeze("expression-core/source-identity"));
                    }
                    let expected = match checked_expression_core_operation_v1(core, self.values)? {
                        ControlFlow::Continue(read) => {
                            let Operand::NormalInteger(dst) = value else { return Err(freeze("expression-core/result-domain")); };
                            self.require_unique(|row| read.matches_instruction_v1(*dst, row))?;
                            continue;
                        }
                        ControlFlow::Break(operation) => operation,
                    };
                    let dst = match (value, expected.normal_result_kind()) {
                        (
                            Operand::OwnedString(dst),
                            Some(crate::mir::instruction::InvokeNormalResultKind::OwnedString),
                        ) if self.frame.owned_result_nodes().contains(&index) => *dst,
                        (
                            Operand::NormalInteger(dst),
                            Some(crate::mir::instruction::InvokeNormalResultKind::I64),
                        ) if !self.frame.owned_result_nodes().contains(&index) => *dst,
                        (
                            Operand::DynamicCarrier(dst),
                            Some(crate::mir::instruction::InvokeNormalResultKind::DynamicCarrier),
                        ) if self.frame.owned_result_nodes().contains(&index) => *dst,
                        _ => return Err(freeze("expression-core/result-domain")),
                    };
                    let result = self.require_unique(|row| matches!(row, MirInstruction::InvokeNormalResult { dst: actual, .. } if *actual == dst))?;
                    let MirInstruction::InvokeNormalResult { invoke_block, .. } = result.1 else {
                        unreachable!()
                    };
                    let invoke = self.require_unique(|row| {
                        matches!(row, MirInstruction::Invoke {
                        operation: actual, normal_landing, .. }
                        if actual == &expected && *normal_landing == result.0)
                    })?;
                    if invoke.0 != invoke_block || invoke.0 == result.0 {
                        return Err(freeze("expression-core/producer"));
                    }
                }
                (Node::MethodCall, Some(Operand::NormalInteger(_) | Operand::OwnedString(_) | Operand::DynamicCarrier(_)))
                    if self
                        .calls()
                        .any(|packet| packet.call_site().site() == node.site()) => {}
                (Node::MethodCall, Some(Operand::DynamicCarrier(value)))
                    if self.dynamic_calls().any(|packet| {
                        packet.source().site().site() == node.site() && packet.value() == *value
                    }) => {}
                (
                    Node::Shape(
                        crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess {
                            ..
                        },
                    ),
                    Some(operand @ (Operand::NormalInteger(dst) | Operand::OwnedString(dst))),
                ) => {
                    if matches!(operand, Operand::OwnedString(_)) != self.frame.owned_result_nodes().contains(&index) {
                        return Err(freeze("print-expression/field-source-value"));
                    }
                    let emitted = ledger.expression_field_read_bindings_v1(
                        &OwnedExprSiteV1::new(source.owner(), node.site().clone()),
                        source,
                        context,
                    )?;
                    if !matches!(emitted.last().map(|binding| &binding.1), Some(MirInstruction::ObjectFieldGet { dst: actual, .. }
                        | MirInstruction::InvokeNormalResult { dst: actual, .. }) if actual == dst)
                        || emitted
                            .iter()
                            .any(|binding| !self.original.contains(binding))
                    {
                        return Err(freeze("print-expression/field-source-value"));
                    }
                }
                (Node::Binary(binary), Some(value)) => {
                    let left = find_value(binary.lhs())?;
                    let right = find_value(binary.rhs())?;
                    let right = self.frame.string_add_right_operand_v1(binary.rhs(), right);
                    match (value, left, right) {
                        (
                            Operand::NormalInteger(dst),
                            Operand::NormalInteger(lhs),
                            Operand::NormalInteger(rhs),
                        ) => {
                            let loan = self
                                .frame
                                .operations()
                                .iter()
                                .find(|loan| {
                                    loan.node() == index && Rc::ptr_eq(loan.source(), source)
                                })
                                .ok_or_else(|| {
                                    freeze("print-expression/operator-source-missing")
                                })?;
                            let operation = loan.integer_operator_v1().ok_or_else(|| {
                                freeze("print-expression/operator-source-missing")
                            })?;
                            if let Some(scalar) = loan.scalar_source_v1() {
                                let original =
                                    context.ledger().scalar_operation_original_v1(scalar)?;
                                original.validate_expression_operands_v1(
                                    (lhs, rhs),
                                    *dst,
                                    self.original,
                                )?;
                                self.require_unique(|row| row == &original.original().1)?;
                            } else {
                                self.require_unique(|row| matches!(row, MirInstruction::BinOp { dst: actual, op, lhs: a, rhs: b }
                                    if actual == dst && *op == operation && *a == lhs && *b == rhs))?;
                            }
                        }
                        (Operand::OwnedString(dst), left, right)
                            if self.frame.owned_result_nodes().contains(&index) =>
                        {
                            let projection = self.require_unique(|row| matches!(row, MirInstruction::InvokeNormalResult { dst: actual, .. } if actual == dst))?;
                            let MirInstruction::InvokeNormalResult { invoke_block, .. } =
                                projection.1
                            else {
                                unreachable!()
                            };
                            let invoke = self.original.iter().find(|(block, row)| block == &invoke_block
                                && matches!(row, MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Add { left: a, right: b }), normal_landing, .. }
                                    if a == &left && b == &right && *normal_landing == projection.0))
                                .ok_or_else(|| freeze("print-expression/add-producer"))?;
                            if !self.frame.operations().iter().any(|operation| {
                                operation.node() == index && Rc::ptr_eq(operation.source(), source)
                            }) || invoke.0 == projection.0
                            {
                                return Err(freeze("print-expression/add-source"));
                            }
                        }
                        (Operand::Dynamic(dst), _, _) => {
                            self.dynamic_add_original_v1(index, *dst)?;
                        }
                        (Operand::DynamicCarrier(dst), Operand::Dynamic(lhs), Operand::NormalInteger(rhs)) => {
                            self.dynamic_sub_original_v1(index, *dst, lhs, rhs)?;
                        }
                        _ => return Err(freeze("print-expression/add-values")),
                    }
                }
                // Receiver identity is owned by its field/call source relation.
                (
                    Node::Shape(
                        crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess {
                            ..
                        },
                    ),
                    None,
                ) if self.frame.node_is_identity(index) => {
                    let site = OwnedExprSiteV1::new(source.owner(), node.site().clone());
                    if !ledger
                        .statement_field_path_v1(&site, context)?
                        .is_some_and(|path| path.is_intermediate())
                    {
                        return Err(freeze("print-expression/identity-field-source"));
                    }
                    let emitted =
                        ledger.expression_field_read_bindings_v1(&site, source, context)?;
                    if !matches!(
                        emitted.last().map(|binding| &binding.1),
                        Some(MirInstruction::ObjectFieldGet { .. })
                    ) || emitted
                        .iter()
                        .any(|binding| !self.original.contains(binding))
                    {
                        return Err(freeze("print-expression/identity-field-binding"));
                    }
                }
                (_, None) if self.frame.node_is_identity(index) => {}
                _ => return Err(freeze("print-expression/node-value")),
            }
        }
        self.comparison_pair()?;
        Ok(())
    }
}

/// Source-qualified read projection. Only the common Core factory constructs it.
/// This retains evaluated operands; it never re-evaluates syntax or issues a loan.
#[derive(Debug, PartialEq)]
pub(crate) struct FrameFreeExpressionCoreReadV1 {
    law: Law,
    value: Operand,
    needle: Option<Operand>,
}
impl FrameFreeExpressionCoreReadV1 {
    pub(crate) fn array_receiver_v1(&self) -> Option<crate::mir::ValueId> {
        match (&self.law, &self.value, &self.needle) {
            (Law::ArrayCount, Operand::NormalInteger(array), None) => Some(*array),
            _ => None,
        }
    }
    pub(crate) fn text_instruction_v1(&self, dst: crate::mir::ValueId) -> Option<MirInstruction> {
        match (&self.law, &self.needle) {
            (Law::CodePointCount, None) => Some(MirInstruction::StringLength { dst, value: self.value.clone() }),
            (Law::CodePointSearchFromZero, Some(needle)) => Some(MirInstruction::StringIndexOf {
                dst, value: self.value.clone(), needle: needle.clone() }),
            _ => None,
        }
    }
    fn matches_instruction_v1(&self, dst: crate::mir::ValueId, row: &MirInstruction) -> bool {
        if let Some(expected) = self.text_instruction_v1(dst) { return expected == *row; }
        matches!(row, MirInstruction::ArrayRead { dst: Some(actual), receiver,
            operation: crate::mir::instruction::ArrayReadOperationV1::Count, .. }
            if *actual == dst && self.array_receiver_v1() == Some(*receiver))
    }
}

/// The sole source-qualified Core-operation projection, shared by the writer
/// and both independent source consumers. It does not issue source permission.
pub(crate) fn checked_expression_core_operation_v1(
    core: &crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1,
    values: &[Option<EvaluatedExpressionValueV1>],
) -> Result<ControlFlow<crate::mir::instruction::InvokeOperation, FrameFreeExpressionCoreReadV1>, String> {
    use crate::mir::core_method_op::CoreMethodOp as Core;
    let source = core.plan();
    if !core.matches_plan(source, core.node()) || values.len() != source.nodes().len() {
        return Err(freeze("expression-core/source-identity"));
    }
    let original = core.source().source();
    let operand = |site| {
        source
            .nodes()
            .iter()
            .position(|node| node.site() == site)
            .and_then(|node| values[node].as_ref())
            .map(|value| value.as_operand().cloned())
            .transpose()?
            .ok_or_else(|| {
                if crate::config::env::builder_debug_enabled() { eprintln!("[core-operand] owner={:?} call={:?} missing={site:?}", source.owner(), original.site()); }
                freeze("expression-core/operand-missing")
            })
    };
    let receiver = operand(original.receiver_site())?;
    let arguments = original
        .arguments()
        .iter()
        .map(|arg| operand(arg.site()))
        .collect::<Result<Vec<_>, _>>()?;
    if let Some(law) = core.source().target().frame_free_read_law_v1() {
        if core.owns_result() { return Err(freeze("expression-core/source-identity")); }
        let needle = match (law, arguments.as_slice()) {
            (Law::ArrayCount, []) if matches!(receiver, Operand::NormalInteger(_)) => None,
            (Law::CodePointCount, []) if receiver.is_text() => None,
            (Law::CodePointSearchFromZero, [needle]) if receiver.is_text() && needle.is_text() => Some(needle.clone()),
            (Law::ArrayCount | Law::CodePointCount, args) if !args.is_empty() =>
                return Err(freeze("expression-core/source-identity")),
            _ => return Err(freeze("expression-core/operand-domain")),
        };
        return Ok(ControlFlow::Continue(FrameFreeExpressionCoreReadV1 { law, value: receiver, needle }));
    }
    use crate::mir::instruction::InvokeOperation as Invoke;
    let operation = match (core.source().target().row().row().op, arguments.as_slice()) {
        (
            Core::StringSubstring,
            [start @ (Operand::NormalInteger(_) | Operand::Dynamic(_)), end @ (Operand::NormalInteger(_) | Operand::Dynamic(_))],
        ) => Invoke::String(Operation::Substring {
            value: receiver,
            start: start.clone(),
            end: end.clone(),
        }),

        (Core::ArrayGet, [Operand::NormalInteger(index)]) if core.is_array_dynamic_get_v1() => {
            let Operand::NormalInteger(array) = receiver else {
                return Err(freeze("expression-core/array-receiver-domain"));
            };
            Invoke::DynamicArrayGet { array, index: *index }
        }
        _ => return Err(freeze("expression-core/operand-domain")),
    };
    let result_kind = operation.normal_result_kind();
    let owned_result = matches!(result_kind, Some(
        crate::mir::instruction::InvokeNormalResultKind::OwnedString
            | crate::mir::instruction::InvokeNormalResultKind::DynamicCarrier));
    let operation_valid = match &operation {
        Invoke::String(operation) => operation.valid_operand_domains(),
        Invoke::DynamicArrayGet { .. } => core.is_array_dynamic_get_v1(),
        _ => false,
    };
    if !operation_valid || result_kind.is_none() || owned_result != core.owns_result() {
        return Err(freeze("expression-core/operand-domain"));
    }
    Ok(ControlFlow::Break(operation))
}

