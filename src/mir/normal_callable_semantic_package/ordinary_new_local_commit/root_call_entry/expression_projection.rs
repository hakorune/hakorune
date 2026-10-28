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
    BranchScalarPhiObservationV1, BranchTextPhiObservationV1, ExpressionReferenceReadV1, ReferenceProducerV1,
};
#[path = "expression_terminal_paths.rs"]
mod terminal_paths;
pub(in crate::mir) use terminal_paths::{
    ExpressionCallArgumentSourceV1, ExpressionCallTransientPrefixV1,
};

/// Internal evaluation values keep Bool separate from String/I64 operands.
/// Conversion borrows an existing domain; it never casts or stringifies Bool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EvaluatedExpressionValueV1 {
    Operand(Operand),
    Bool(ValueId),
}
impl From<Operand> for EvaluatedExpressionValueV1 {
    fn from(value: Operand) -> Self {
        Self::Operand(value)
    }
}
impl EvaluatedExpressionValueV1 {
    pub(in crate::mir) fn as_operand(&self) -> Result<&Operand, String> {
        match self {
            Self::Operand(value) => Ok(value),
            Self::Bool(_) => Err(freeze("expression/scalar-continuation-result")),
        }
    }
    pub(in crate::mir) fn value(&self) -> Option<ValueId> {
        match self {
            Self::Operand(value) => value.value(),
            Self::Bool(value) => Some(*value),
        }
    }
}

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
    comparison: Option<(usize, ValueId)>,
    scalar_receiver: Option<&'a crate::mir::builder::ExactLexicalReadV1>,
}
impl<'a> ExpressionPhysicalProjectionRefV1<'a> {
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
            comparison: None,
            scalar_receiver: None,
        }
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
            if let Some(tail) = source.transparent_tail_node_v1(index) {
                if value.is_none() || self.values[index] != self.values[tail] {
                    return Err(freeze("print-expression/node-value"));
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
                    if let Some(receiver) = core.array_count_receiver_v1(self.values)? {
                        let Operand::NormalInteger(dst) = value else { return Err(freeze("expression-core/result-domain")); };
                        self.require_unique(|row| matches!(row, MirInstruction::ArrayRead { dst: Some(actual), receiver: array,
                            operation: crate::mir::instruction::ArrayReadOperationV1::Count, .. } if actual == dst && *array == receiver))?;
                        continue;
                    }
                    let expected = checked_expression_core_operation_v1(core, self.values)?;
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

    pub(in crate::mir) fn ordered_bindings_in_context(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let mut ordered = Vec::new();
        let source = self.frame.source();
        for &index in self
            .frame
            .ordered_nodes()
            .iter()
            .filter(|node| self.node_selected(**node))
        {
            if self.frame.transfers_node(index) && !self.frame.direct_array_receiver_v1(index)
                .is_some_and(|core| self.node_selected(core.node())) {
                continue;
            }
            if self.frame.node_is_identity(index)
                && matches!(
                    source.nodes()[index].source(),
                    Node::Shape(
                        crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }
                    )
                )
            {
                ordered.extend(context.ledger().expression_field_read_bindings_v1(
                    &OwnedExprSiteV1::new(source.owner(), source.nodes()[index].site().clone()),
                    source,
                    context,
                )?);
                continue;
            }
            if self.frame.cores().contains_key(&index) {
                let value = self.values[index]
                    .as_ref()
                    .and_then(EvaluatedExpressionValueV1::value)
                    .ok_or_else(|| freeze("expression-core/result-missing"))?;
                let result = self.require_unique(|row| row.dst_value() == Some(value))?;
                if self.frame.cores()[&index].array_count_receiver_v1(self.values)?.is_some() {
                    ordered.push(result.clone());
                    continue;
                }
                let MirInstruction::InvokeNormalResult { invoke_block, .. } = result.1 else {
                    return Err(freeze("expression-core/projection"));
                };
                let invoke = self
                    .original
                    .iter()
                    .find(|(block, row)| {
                        *block == invoke_block && matches!(row, MirInstruction::Invoke { .. })
                    })
                    .ok_or_else(|| freeze("expression-core/invoke"))?;
                ordered.extend([invoke.clone(), result.clone()]);
                continue;
            }
            if source.transparent_tail_node_v1(index).is_some() {
                continue;
            }
            if matches!(source.nodes()[index].source(), Node::Reference(_)) {
                continue;
            }
            if let Some(packet) = self
                .calls()
                .find(|packet| packet.call_site().site() == source.nodes()[index].site())
            {
                let call = self
                    .frame
                    .calls()
                    .iter()
                    .find(|call| call.site() == packet.call_site())
                    .ok_or_else(|| freeze("print-expression/call-source"))?;
                ordered.extend(packet.ordered_call_bindings_in_context_v1(
                    source.owner(),
                    call.arguments(),
                    context,
                )?);
                continue;
            }
            if let Some(packet) = self
                .dynamic_calls()
                .find(|packet| packet.source().site().site() == source.nodes()[index].site())
            {
                ordered.extend(
                    packet
                        .scope_bindings_in_context_v1(context)?
                        .0,
                );
                continue;
            }
            if let Some(EvaluatedExpressionValueV1::Bool(value)) = self.values[index].as_ref() {
                ordered.push(
                    self.require_unique(|row| row.dst_value() == Some(*value))?
                        .clone(),
                );
                continue;
            }
            match self.values[index]
                .as_ref()
                .map(|value| value.as_operand())
                .transpose()?
            {
                Some(Operand::NormalInteger(value)) => {
                    if matches!(
                        source.nodes()[index].source(),
                        Node::Shape(
                            crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }
                        )
                    ) {
                        ordered.extend(context.ledger().expression_field_read_bindings_v1(
                            &OwnedExprSiteV1::new(
                                source.owner(),
                                source.nodes()[index].site().clone(),
                            ),
                            source,
                            context,
                        )?);
                    } else {
                        let scalar = self
                            .frame
                            .operations()
                            .iter()
                            .find(|loan| loan.node() == index)
                            .and_then(|loan| loan.scalar_source_v1());
                        if let Some(scalar) = scalar {
                            let original = context.ledger().scalar_operation_original_v1(scalar)?;
                            for relation in original.operand_relations() {
                                if let Some(target) = relation.target_binding() {
                                    if !ordered.contains(target) {
                                        ordered.push(target.clone());
                                    }
                                }
                            }
                        }
                        ordered.push(
                            self.require_unique(|row| row.dst_value() == Some(*value))?
                                .clone(),
                        );
                    }
                }
                Some(Operand::OwnedString(value)) => {
                    let result = self.require_unique(|row| row.dst_value() == Some(*value))?;
                    let MirInstruction::InvokeNormalResult { invoke_block, .. } = result.1 else {
                        return Err(freeze("print-expression/add-result"));
                    };
                    let invoke = self
                        .original
                        .iter()
                        .find(|(id, row)| {
                            *id == invoke_block && matches!(row, MirInstruction::Invoke { .. })
                        })
                        .ok_or_else(|| freeze("print-expression/add-invoke"))?;
                    ordered.extend([invoke.clone(), result.clone()]);
                }
                Some(Operand::Dynamic(value)) => {
                    ordered.push(self.dynamic_add_original_v1(index, *value)?);
                }
                Some(Operand::DynamicCarrier(value)) => {
                    let node = self.frame.source().nodes().get(index)
                        .ok_or_else(|| freeze("print-expression/dynamic-sub-node"))?;
                    let Node::Binary(binary) = node.source() else {
                        return Err(freeze("print-expression/dynamic-sub-node"));
                    };
                    let operand = |site| -> Result<Operand, String> {
                        let value = self.frame.source().nodes().iter()
                            .position(|node| node.site() == site)
                            .and_then(|node| self.values[node].as_ref())
                            .ok_or_else(|| freeze("print-expression/dynamic-sub-operand"))?;
                        Ok(value.as_operand()?.clone())
                    };
                    let (Operand::Dynamic(carrier), Operand::NormalInteger(rhs)) =
                        (operand(binary.lhs())?, operand(binary.rhs())?)
                    else { return Err(freeze("print-expression/dynamic-sub-operands")); };
                    ordered.extend(self.dynamic_sub_original_v1(index, *value, carrier, rhs)?);
                }
                _ => {}
            }
        }
        if let Some(pair) = self.comparison_pair()? {
            ordered.extend(pair);
        }
        Ok(ordered)
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

    /// A genuine entry Jump delimits evaluation from preceding statements.
    /// Check the complete original scope before capture. Final contraction and
    /// insertion detection then belong to the existing PhysicalBoundary's
    /// complete sequence comparison, including these same structural Jumps.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_original_scope(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        covered: &std::collections::BTreeSet<BasicBlockId>,
        entry_binding: &Binding,
        terminal: &Binding,
        continuation: &Binding,
        ordered: &[Binding],
    ) -> Result<(), String> {
        use std::collections::BTreeSet;
        let MirInstruction::Jump {
            target: entry,
            edge_args: None,
        } = entry_binding.1
        else {
            return Err(freeze("print-expression/entry-shape"));
        };
        if entry == entry_binding.0 || entry_binding.0 == continuation.0 {
            return Err(freeze("print-expression/entry-boundary"));
        }
        if !super::super::physical_boundary::check_binding(
            function,
            None,
            entry_binding.0,
            &entry_binding.1,
        )? {
            return Err(freeze("print-expression/entry-missing"));
        }
        let mut calls = Vec::new();
        let mut copies = Vec::new();
        for (observation, packet) in self.source_calls().zip(self.calls()) {
            copies.extend(
                packet
                    .copy_dependencies(ledger)?
                    .into_iter()
                    .map(|(_, binding)| binding),
            );
            packet.visit_original_nodes_v1(
                self.frame.source().owner(),
                observation.arguments(),
                ledger,
                &mut |node, _| {
                    calls.extend(
                        self.original
                            .iter()
                            .filter(|binding| {
                                node.has_original_scope_binding_at(node.call_site(), binding)
                            })
                            .cloned(),
                    );
                    Ok(())
                },
            )?;
        }
        for (index, node) in self
            .frame
            .source()
            .nodes()
            .iter()
            .enumerate()
            .filter(|(node, _)| self.node_selected(*node))
        {
            copies.extend(ledger.checked_field_copy_dependencies_v1(
                &OwnedExprSiteV1::new(self.frame.source().owner(), node.site().clone()),
                CallValidationContextV1::Lowering(ledger),
            )?);
        }
        copies.extend(self.scalar_copy_dependencies_v1(CallValidationContextV1::Lowering(ledger))?);
        for packet in self.dynamic_calls() {
            copies.extend(
                packet
                    .scope_bindings_in_context_v1(CallValidationContextV1::Lowering(ledger))?
                    .1,
            );
        }
        for read in self.path_reads {
            copies.extend_from_slice(read.original_copy_dependencies_v1()?);
        }
        for read in self.reference_reads.values() {
            copies.extend(read.dependencies()?);
        }
        let mut scope = BTreeSet::new();
        let mut pending = vec![entry];
        while let Some(id) = pending.pop() {
            if !scope.insert(id) {
                continue;
            }
            let block = function
                .blocks
                .get(&id)
                .ok_or_else(|| freeze("print-expression/scope-block"))?;
            for instruction in block.all_instructions() {
                let binding = (id, instruction.clone());
                let structural = matches!(
                    instruction,
                    MirInstruction::Jump {
                        edge_args: None,
                        ..
                    } | MirInstruction::ReturnFault { .. }
                );
                let approved = structural
                    || ordered.contains(&binding)
                    || copies.contains(&binding)
                    || calls.contains(&binding)
                    || covered.contains(&id)
                        && matches!(instruction, MirInstruction::Invoke { .. });
                if !approved || !self.original.contains(&binding) && !copies.contains(&binding) {
                    return Err(freeze("print-expression/unowned-scope-instruction"));
                }
            }
            let terminal = block
                .terminator
                .as_ref()
                .ok_or_else(|| freeze("print-expression/scope-terminal"))?;
            if id == continuation.0 {
                if terminal != &continuation.1 || !block.instructions.is_empty() {
                    return Err(freeze("print-expression/continuation-body"));
                }
                continue;
            }
            match terminal {
                MirInstruction::Jump {
                    target,
                    edge_args: None,
                } => pending.push(*target),
                MirInstruction::Invoke {
                    normal_landing,
                    fault_landing,
                    ..
                } => pending.extend([*normal_landing, *fault_landing]),
                MirInstruction::ReturnFault { .. } => {}
                _ => return Err(freeze("print-expression/scope-escape")),
            }
        }
        if !scope.contains(&terminal.0)
            || !scope.contains(&continuation.0)
            || ordered.iter().any(|(id, _)| !scope.contains(id))
            || covered.iter().any(|id| !scope.contains(id))
        {
            return Err(freeze("print-expression/scope-disconnected"));
        }
        if let Some(pair) = self.comparison_pair()? {
            let MirInstruction::Invoke { fault_frame, .. } = pair[0].1 else {
                unreachable!()
            };
            let frame = self.require_unique(|row| matches!(row, MirInstruction::FaultFrameEnter { dst, .. } if *dst == fault_frame))?;
            if self
                .original
                .iter()
                .any(|row| !scope.contains(&row.0) && row != entry_binding && row != frame)
            {
                return Err(freeze("print-expression/unowned-scope-instruction"));
            }
        }
        for (id, block) in &function.blocks {
            if scope.contains(id) {
                continue;
            }
            for edge in block.out_edges() {
                if scope.contains(&edge.target)
                    && !(*id == entry_binding.0
                        && edge.target == entry
                        && block.terminator.as_ref() == Some(&entry_binding.1))
                {
                    return Err(freeze("print-expression/scope-foreign-entry"));
                }
            }
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_recorded(
        &self,
        bindings: &[Binding],
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        for original in self.original {
            let mapped = match finishing {
                Some(projection) => projection.binding(original.0, &original.1)?,
                None => Some(original.clone()),
            };
            // Constants may be removed only by the existing captured boundary.
            if let Some(mapped) = mapped {
                if bindings
                    .iter()
                    .filter(|binding| **binding == mapped)
                    .count()
                    != 1
                {
                    return Err(freeze("print-expression/binding-membership"));
                }
            } else if !matches!(
                original.1,
                MirInstruction::Const { .. }
                    | MirInstruction::Jump {
                        edge_args: None,
                        ..
                    }
            ) {
                return Err(freeze("print-expression/required-binding-removed"));
            }
        }
        for packet in self.calls() {
            packet.validate_recorded_projected(bindings, finishing)?;
        }
        for packet in self.dynamic_calls() {
            packet.validate_recorded_projected_v1(bindings, finishing)?;
        }
        Ok(())
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

/// The sole source-qualified Core-operation projection, shared by the writer
/// and both independent source consumers. It does not issue source permission.
pub(crate) fn checked_expression_core_operation_v1(
    core: &crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1,
    values: &[Option<EvaluatedExpressionValueV1>],
) -> Result<crate::mir::instruction::InvokeOperation, String> {
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
    use crate::mir::instruction::InvokeOperation as Invoke;
    let operation = match (core.source().target().row().row().op, arguments.as_slice()) {
        (Core::StringLen, []) => Invoke::String(Operation::Length { value: receiver }),
        (
            Core::StringSubstring,
            [start @ (Operand::NormalInteger(_) | Operand::Dynamic(_)), end @ (Operand::NormalInteger(_) | Operand::Dynamic(_))],
        ) => Invoke::String(Operation::Substring {
            value: receiver,
            start: start.clone(),
            end: end.clone(),
        }),
        (Core::StringIndexOf, [needle]) => Invoke::String(Operation::IndexOf {
            value: receiver,
            needle: needle.clone(),
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
    Ok(operation)
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
