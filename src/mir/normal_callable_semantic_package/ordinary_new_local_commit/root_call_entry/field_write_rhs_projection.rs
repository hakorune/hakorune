//! Immutable whole-RHS observations from the existing recursive writers.
//! Correspondence only: source class never grants execution or Home permission.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    BoolConditionCursorV1 as Cursor, OriginalScalarAppendV1,
};
use crate::mir::resolved_semantics::{
    home_new_prefix::QualifiedFieldWriteRhsV1, ResolvedBinaryOperatorV1 as Operator,
    ResolvedLiteralSourceV1 as Literal, SourceExprSiteV1,
};
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;

type Relation = Rc<LocalSsaPhysicalRelationV1>;
#[derive(Debug)]
pub(in crate::mir) enum FieldWriteValueV1 {
    Lexical(Rc<ExactLexicalReadV1>),
    Expression(Rc<EmittedFieldWriteRhsV1>),
}
#[derive(Debug)]
pub(in crate::mir) enum FieldWriteRhsProofV1 {
    Literal(Binding),
    Reference {
        read: Rc<ExactLexicalReadV1>,
        producer: Option<Binding>,
        signed: Option<Rc<crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1>>,
    },
    ArrayHomeRead {
        read: Rc<ExactLexicalReadV1>,
        producer: Option<Binding>,
    },
    Field(Vec<Binding>),
    Binary {
        append: Binding,
        operands: [Relation; 2],
        scalar: Option<Rc<OriginalScalarAppendV1>>,
    },
}
#[derive(Debug)]
pub(in crate::mir) struct FieldWriteRhsNodeV1 {
    pub(in crate::mir) node: usize,
    pub(in crate::mir) entry: Cursor,
    pub(in crate::mir) exit: Cursor,
    pub(in crate::mir) value: crate::mir::ValueId,
    pub(in crate::mir) proof: FieldWriteRhsProofV1,
}
#[derive(Debug)]
enum FieldWriteRhsObservationsV1 {
    Recursive(Box<[FieldWriteRhsNodeV1]>),
    Owned(Box<OwnedFieldWriteRhsV1>),
}
#[derive(Debug)]
struct OwnedFieldWriteRhsV1 {
    body: crate::mir::builder::EmittedExpressionBodyV1,
    acquisition: Option<[Binding; 2]>,
    terminal: Binding,
    continuation: Binding,
}
#[derive(Debug)]
pub(in crate::mir) struct EmittedFieldWriteRhsV1 {
    recipe: Rc<QualifiedFieldWriteRhsV1>,
    observations: FieldWriteRhsObservationsV1,
    value: crate::mir::ValueId,
}
#[path = "field_write_rhs_scope.rs"]
mod scope;
impl EmittedFieldWriteRhsV1 {
    fn recursive_nodes(&self) -> &[FieldWriteRhsNodeV1] {
        match &self.observations {
            FieldWriteRhsObservationsV1::Recursive(nodes) => nodes,
            FieldWriteRhsObservationsV1::Owned(_) => &[],
        }
    }
    pub(in crate::mir) fn from_owned_expression_v1(
        recipe: Rc<QualifiedFieldWriteRhsV1>, body: crate::mir::builder::EmittedExpressionBodyV1,
        value: crate::mir::ValueId, acquisition: Option<[Binding; 2]>,
        terminal: Binding, continuation: Binding,
        ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<Self, String> {
        let result = Self { recipe, value, observations: FieldWriteRhsObservationsV1::Owned(
            Box::new(OwnedFieldWriteRhsV1 { body, acquisition, terminal, continuation })) };
        result.validate(CallValidationContextV1::Lowering(ledger), function, None)?;
        Ok(result)
    }
    fn owned_expression_ref_v1<'a>(&'a self, owned: &'a OwnedFieldWriteRhsV1)
        -> Result<super::super::expression_projection::ExpressionPhysicalProjectionRefV1<'a>, String> {
        let evaluation = self.recipe.evaluation().ok_or_else(|| freeze("field-write-rhs/source-coverage"))?;
        Ok(super::super::expression_projection::ExpressionPhysicalProjectionRefV1::new_with_dynamic(
            evaluation, &owned.body.values, &owned.body.calls, &owned.body.dynamic_calls,
            &owned.body.path_reads, &owned.body.reference_reads, &owned.body.bindings))
    }
    pub(in crate::mir) fn deferred_child_ends_v1(&self, context: CallValidationContextV1<'_>)
        -> Result<Vec<crate::mir::instruction::InvokeOperation>, String> {
        let FieldWriteRhsObservationsV1::Owned(owned) = &self.observations else { return Ok(Vec::new()); };
        if !self.recipe.is_consistent(self.recipe.source()) { return Err(freeze("field-write-rhs/source-coverage")); }
        let view = self.owned_expression_ref_v1(owned)?;
        view.validate_source(context)?;
        let materializes = self.recipe.evaluation().unwrap().owned_text_root_node_v1().is_none();
        view.validate_effective_root_v1(self.value, true, materializes, owned.acquisition.as_ref())?;
        let mut temporaries = owned.body.ordered_owned_temporaries_v1(
            self.recipe.evaluation().unwrap(), self.value, materializes)?;
        if temporaries.pop() != Some(crate::mir::builder::ExpressionOwnedTemporaryV1::String(self.value)) {
            return Err(freeze("field-write-rhs/root-scope"));
        }
        Ok(temporaries.into_iter().rev().map(crate::mir::builder::ExpressionOwnedTemporaryV1::end_operation).collect())
    }
    pub(in crate::mir) fn new(
        recipe: Rc<QualifiedFieldWriteRhsV1>,
        nodes: Vec<FieldWriteRhsNodeV1>,
        value: crate::mir::ValueId,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        let result = Self {
            recipe,
            observations: FieldWriteRhsObservationsV1::Recursive(nodes.into_boxed_slice()),
            value,
        };
        result.validate(CallValidationContextV1::Lowering(ledger), function, None)?;
        Ok(result)
    }
    pub(in crate::mir) fn lexical_calls_v1(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        let body = match &self.observations { FieldWriteRhsObservationsV1::Owned(owned) => Some(&owned.body), _ => None };
        body.into_iter().flat_map(|body| body.calls.iter().map(Rc::as_ref))
    }
    pub(in crate::mir) fn value(&self) -> crate::mir::ValueId {
        self.value
    }
    pub(in crate::mir) fn recipe(&self) -> &Rc<QualifiedFieldWriteRhsV1> {
        &self.recipe
    }
    pub(in crate::mir) fn relations(&self) -> Vec<Relation> {
        self.recursive_nodes()
            .iter()
            .filter_map(|node| match &node.proof {
                FieldWriteRhsProofV1::Binary { operands, .. } => Some(operands.iter().cloned()),
                _ => None,
            })
            .flatten()
            .collect()
    }
    pub(in crate::mir) fn bindings(&self) -> Result<Vec<Binding>, String> {
        if let FieldWriteRhsObservationsV1::Owned(owned) = &self.observations {
            return Ok(owned.body.bindings.clone());
        }
        let mut rows = Vec::new();
        for node in self.recursive_nodes() {
            match &node.proof {
                FieldWriteRhsProofV1::Literal(row) => rows.push(row.clone()),
                FieldWriteRhsProofV1::Reference {
                    producer, signed, ..
                } => {
                    rows.extend(producer.iter().cloned());
                    if let Some(signed) = signed {
                        rows.extend(signed.dependencies()?);
                    }
                }
                FieldWriteRhsProofV1::ArrayHomeRead { producer, .. } => {
                    rows.extend(producer.iter().cloned());
                }
                FieldWriteRhsProofV1::Field(field) => rows.extend(field.iter().cloned()),
                FieldWriteRhsProofV1::Binary {
                    append,
                    operands,
                    scalar,
                } => {
                    rows.push(append.clone());
                    for relation in operands {
                        relation.append_observed_bindings_v1(&mut rows);
                    }
                    if let Some(scalar) = scalar {
                        rows.extend(scalar.divisor().cloned());
                    }
                }
            }
        }
        let mut unique = Vec::new();
        for row in rows {
            if !unique.contains(&row) {
                unique.push(row);
            }
        }
        Ok(unique)
    }
    fn value_at(&self, site: &SourceExprSiteV1) -> Result<crate::mir::ValueId, String> {
        let index = self
            .recipe
            .source()
            .nodes()
            .iter()
            .position(|node| node.site() == site)
            .ok_or_else(|| freeze("field-write-rhs/child-site"))?;
        self.recursive_nodes()
            .iter()
            .find(|node| node.node == index)
            .map(|node| node.value)
            .ok_or_else(|| freeze("field-write-rhs/child-value"))
    }
    pub(in crate::mir) fn validate_lowering(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate(CallValidationContextV1::Lowering(ledger), function, None)
    }
    pub(super) fn validate(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_with_writer_prefix(context, function, finishing, &[])
    }
    pub(super) fn validate_with_writer_prefix(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&FinishedBindings>,
        post_rhs_prefix: &[Binding],
    ) -> Result<(), String> {
        let source = self.recipe.source();
        if let FieldWriteRhsObservationsV1::Owned(owned) = &self.observations {
            if !self.recipe.is_consistent(source) { return Err(freeze("field-write-rhs/source-coverage")); }
            let view = self.owned_expression_ref_v1(owned)?;
            let materializes = self.recipe.evaluation().unwrap().owned_text_root_node_v1().is_none();
            if owned.acquisition.is_some() != materializes { return Err(freeze("field-write-rhs/source-coverage")); }
            view.validate_effective_root_v1(self.value, true, materializes, owned.acquisition.as_ref())?;
            let covered = view.validate_deferred_owned_continuation_v1(context, function, finishing,
                &owned.body.entry, &owned.terminal, &owned.continuation, None, owned.acquisition.as_ref())?;
            if finishing.is_none() && post_rhs_prefix.is_empty() {
                let mut ordered = view.ordered_bindings_in_context(context)?;
                if let Some(pair) = &owned.acquisition { ordered.extend(pair.iter().cloned()); }
                ordered.push(owned.terminal.clone());
                view.validate_original_scope(context.ledger(), function, &covered,
                    &owned.body.entry, &owned.terminal, &owned.continuation, &ordered)?;
            }
            return Ok(());
        }
        if self.recipe.evaluation().is_some() { return Err(freeze("field-write-rhs/source-coverage")); }
        let expected_nodes = self.recipe.ordered_nodes().iter().copied().filter(|index|
            self.recipe.node_class(*index) != Some(crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Identity)
                || self.recipe.owned_array().is_some());
        if !self.recipe.is_consistent(source) || self.recursive_nodes().iter().map(|node| node.node).ne(expected_nodes)
            || self.recursive_nodes().iter().find(|node| node.node == 0).map(|node| node.value) != Some(self.value) {
            return Err(freeze("field-write-rhs/source-coverage"));
        }
        let mut ordered = Vec::new();
        let mut require = |row: &Binding, mandatory: bool| -> Result<(), String> {
            let mapped = match finishing {
                Some(view) if row.1.dst_value().is_some() => {
                    view.relation_binding(function, row)?
                }
                Some(view) => Some(
                    match context {
                        CallValidationContextV1::Lowering(_) => {
                            view.captured_effect_binding(function, row)
                        }
                        CallValidationContextV1::Finalized(_) => {
                            view.required_effect_binding(function, row)
                        }
                    }
                    .map_err(|error| format!("{error} rhs-original={row:?}"))?,
                ),
                None => Some(row.clone()),
            };
            match mapped {
                Some(actual) => {
                    if !super::super::super::super::physical_boundary::check_binding(
                        function, None, actual.0, &actual.1,
                    )? {
                        return Err(freeze("field-write-rhs/producer-missing"));
                    }
                    if mandatory {
                        ordered.push(actual);
                    }
                }
                None if !mandatory
                    && matches!(
                        row.1,
                        MirInstruction::Const { .. } | MirInstruction::Copy { .. }
                    ) => {}
                None => return Err(freeze("field-write-rhs/producer-removed")),
            }
            Ok(())
        };
        for node in self.recursive_nodes() {
            let original = &source.nodes()[node.node];
            match (original.source(), &node.proof) {
                (Node::Literal(literal), FieldWriteRhsProofV1::Literal(row)) => {
                    let expected = match literal {
                        Literal::Integer(value) | Literal::TypedInteger { value, .. } => *value,
                        _ => return Err(freeze("field-write-rhs/literal-kind")),
                    };
                    if !matches!(row.1, MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(value) }
                        if dst == node.value && value == expected)
                    {
                        return Err(freeze("field-write-rhs/literal-value"));
                    }
                    if let Literal::TypedInteger {
                        declared_type_name,
                        value,
                    } = literal
                    {
                        if function
                            .metadata
                            .exact_numeric_const_facts
                            .get(&node.value)
                            .is_none_or(|fact| {
                                fact.declared_type_name != declared_type_name.as_ref()
                                    || fact.value != i128::from(*value)
                            })
                        {
                            return Err(freeze("field-write-rhs/exact-literal-fact"));
                        }
                    }
                    require(row, true)?;
                }
                (
                    Node::Reference(_),
                    FieldWriteRhsProofV1::Reference {
                        read,
                        producer,
                        signed,
                    },
                ) => {
                    let (binding, _) = self
                        .recipe
                        .reference(node.node)
                        .ok_or_else(|| freeze("field-write-rhs/reference-kind"))?;
                    if read
                        .value_for(source.owner(), original.site().node(), binding)
                        .map_err(|issue| issue.to_string())?
                        != node.value
                    {
                        return Err(freeze("field-write-rhs/reference-value"));
                    }
                    let proof = self
                        .recipe
                        .reference_source(node.node)
                        .ok_or_else(|| freeze("field-write-rhs/reference-kind"))?;
                    let needs_source = self.recipe.signed_reference(node.node).is_some()
                        || matches!(proof, crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::FiniteEntry { .. });
                    match (needs_source, signed) {
                        (true, Some(signed)) => {
                            if signed.validate_source_reference_v1(
                                source, node.node, proof, context, function, finishing,
                            )? != node.value
                            {
                                return Err(freeze("field-write-rhs/reference-value"));
                            }
                            for row in signed.dependencies()? {
                                require(&row, false)?;
                            }
                        }
                        (false, None) => {}
                        _ => return Err(freeze("field-write-rhs/reference-kind")),
                    }
                    if let Some(row) = producer {
                        if row.1.dst_value() != Some(node.value) {
                            return Err(freeze("field-write-rhs/reference-producer"));
                        }
                        require(row, false)?;
                    } else if !function.params.contains(&node.value) {
                        return Err(freeze("field-write-rhs/reference-parameter"));
                    }
                }
                (
                    Node::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)),
                    FieldWriteRhsProofV1::ArrayHomeRead { read, producer },
                ) => {
                    let array = self.recipe.owned_array()
                        .ok_or_else(|| freeze("field-write-rhs/array-home-source"))?;
                    if *binding != array.binding()
                        || array.binding().owner() != source.owner()
                        || read.value_for(source.owner(), original.site().node(), *binding)
                            .map_err(|issue| issue.to_string())? != node.value
                    {
                        return Err(freeze("field-write-rhs/array-home-read"));
                    }
                    if let Some(row) = producer {
                        if row.1.dst_value() != Some(node.value) {
                            return Err(freeze("field-write-rhs/array-home-producer"));
                        }
                        require(row, false)?;
                    } else if !function.params.contains(&node.value) {
                        return Err(freeze("field-write-rhs/array-home-parameter"));
                    }
                }
                (
                    Node::Shape(
                        crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess {
                            ..
                        },
                    ),
                    FieldWriteRhsProofV1::Field(rows),
                ) => {
                    let expected = context.ledger().expression_field_read_bindings_v1(
                        &OwnedExprSiteV1::new(source.owner(), original.site().clone()),
                        source,
                        context,
                    )?;
                    if &expected != rows
                        || rows.last().and_then(|row| row.1.dst_value()) != Some(node.value)
                    {
                        return Err(freeze("field-write-rhs/field-producer"));
                    }
                    for row in rows {
                        require(row, true)?;
                    }
                }
                (
                    Node::Binary(binary),
                    FieldWriteRhsProofV1::Binary {
                        append,
                        operands,
                        scalar,
                    },
                ) => {
                    let MirInstruction::BinOp { dst, op, lhs, rhs } = append.1 else {
                        return Err(freeze("field-write-rhs/operator-producer"));
                    };
                    if dst != node.value
                        || op != arithmetic_operator(binary.operator())?
                        || operands[0].values() != (self.value_at(binary.lhs())?, lhs)
                        || operands[1].values() != (self.value_at(binary.rhs())?, rhs)
                    {
                        return Err(freeze("field-write-rhs/operator-operands"));
                    }
                    for relation in operands {
                        relation.verify_projected(function, &mut |row| match finishing {
                            Some(view) => view.relation_binding(function, row),
                            None => Ok(Some(row.clone())),
                        })?;
                    }
                    if matches!(binary.operator(), Operator::Subtract | Operator::Divide) {
                        let scalar = scalar
                            .as_ref()
                            .ok_or_else(|| freeze("field-write-rhs/scalar-original"))?;
                        if !Rc::ptr_eq(scalar.source().plan(), source)
                            || scalar.source().binary().site() != original.site()
                            || scalar.original() != append
                            || scalar.children()
                                != (self.value_at(binary.lhs())?, self.value_at(binary.rhs())?)
                            || !scalar
                                .operand_relations()
                                .iter()
                                .zip(operands)
                                .all(|(a, b)| Rc::ptr_eq(a, b))
                        {
                            return Err(freeze("field-write-rhs/scalar-source"));
                        }
                    } else if scalar.is_some() {
                        return Err(freeze("field-write-rhs/unexpected-scalar"));
                    }
                    require(append, true)?;
                }
                _ => return Err(freeze("field-write-rhs/source-proof-kind")),
            }
        }
        super::super::expression_projection::normal_order(function, &ordered)?;
        if finishing.is_none() {
            self.validate_original_scope(function, post_rhs_prefix)?;
        }
        Ok(())
    }
}
fn arithmetic_operator(operator: Operator) -> Result<crate::mir::BinaryOp, String> {
    use crate::mir::BinaryOp as Op;
    Ok(match operator {
        Operator::Add => Op::Add,
        Operator::Subtract => Op::Sub,
        Operator::Multiply => Op::Mul,
        Operator::Divide => Op::Div,
        Operator::Modulo => Op::Mod,
        Operator::BitAnd => Op::BitAnd,
        Operator::BitOr => Op::BitOr,
        Operator::BitXor => Op::BitXor,
        Operator::Shl => Op::Shl,
        Operator::Shr => Op::Shr,
        _ => return Err(freeze("field-write-rhs/non-arithmetic-operator")),
    })
}

#[cfg(test)]
#[path = "field_write_rhs_projection_tests.rs"]
mod tests;
