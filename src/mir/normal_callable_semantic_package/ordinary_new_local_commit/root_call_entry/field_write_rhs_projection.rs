//! Immutable whole-RHS observations from the existing recursive writers.
//! Correspondence only: source class never grants execution or Home permission.
use super::*;
use crate::mir::normal_callable_semantic_package::{OriginalScalarAppendV1, BoolConditionCursorV1 as Cursor};
use crate::mir::resolved_semantics::{home_new_prefix::QualifiedFieldWriteRhsV1, ResolvedLiteralSourceV1 as Literal,
    ResolvedBinaryOperatorV1 as Operator, SourceExprSiteV1};
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
    Reference { read: Rc<ExactLexicalReadV1>, producer: Option<Binding> },
    Field(Vec<Binding>),
    Binary { append: Binding, operands: [Relation; 2], scalar: Option<Rc<OriginalScalarAppendV1>> },
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
pub(in crate::mir) struct EmittedFieldWriteRhsV1 {
    recipe: Rc<QualifiedFieldWriteRhsV1>,
    nodes: Box<[FieldWriteRhsNodeV1]>,
    value: crate::mir::ValueId,
}
#[path = "field_write_rhs_scope.rs"]
mod scope;
impl EmittedFieldWriteRhsV1 {
    pub(in crate::mir) fn new(recipe: Rc<QualifiedFieldWriteRhsV1>, nodes: Vec<FieldWriteRhsNodeV1>,
        value: crate::mir::ValueId, ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<Self, String> {
        let result = Self { recipe, nodes: nodes.into_boxed_slice(), value };
        result.validate(CallValidationContextV1::Lowering(ledger), function, None)?;
        Ok(result)
    }
    pub(in crate::mir) fn value(&self) -> crate::mir::ValueId { self.value }
    pub(in crate::mir) fn recipe(&self) -> &Rc<QualifiedFieldWriteRhsV1> { &self.recipe }
    pub(in crate::mir) fn relations(&self) -> Vec<Relation> {
        self.nodes.iter().filter_map(|node| match &node.proof {
            FieldWriteRhsProofV1::Binary { operands, .. } => Some(operands.iter().cloned()), _ => None,
        }).flatten().collect()
    }
    pub(in crate::mir) fn bindings(&self) -> Vec<Binding> {
        let mut rows = Vec::new();
        for node in &self.nodes {
            match &node.proof {
                FieldWriteRhsProofV1::Literal(row) => rows.push(row.clone()),
                FieldWriteRhsProofV1::Reference { producer, .. } => rows.extend(producer.iter().cloned()),
                FieldWriteRhsProofV1::Field(field) => rows.extend(field.iter().cloned()),
                FieldWriteRhsProofV1::Binary { append, operands, scalar } => {
                    rows.push(append.clone());
                    for relation in operands { relation.append_observed_bindings_v1(&mut rows); }
                    if let Some(scalar) = scalar { rows.extend(scalar.divisor().cloned()); }
                },
            }
        }
        let mut unique = Vec::new();
        for row in rows { if !unique.contains(&row) { unique.push(row); } }
        unique
    }
    fn value_at(&self, site: &SourceExprSiteV1) -> Result<crate::mir::ValueId, String> {
        let index = self.recipe.source().nodes().iter().position(|node| node.site() == site)
            .ok_or_else(|| freeze("field-write-rhs/child-site"))?;
        self.nodes.iter().find(|node| node.node == index).map(|node| node.value)
            .ok_or_else(|| freeze("field-write-rhs/child-value"))
    }
    pub(in crate::mir) fn validate_lowering(&self, ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction) -> Result<(), String> {
        self.validate(CallValidationContextV1::Lowering(ledger), function, None)
    }
    pub(super) fn validate(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        let source = self.recipe.source();
        if !self.recipe.is_consistent(source) || self.nodes.iter().map(|node| node.node).ne(
            self.recipe.ordered_nodes().iter().copied().filter(|index|
                self.recipe.node_class(*index) != Some(crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Identity)))
            || self.nodes.iter().find(|node| node.node == 0).map(|node| node.value) != Some(self.value) {
            return Err(freeze("field-write-rhs/source-coverage"));
        }
        let mut ordered = Vec::new();
        let mut require = |row: &Binding, mandatory: bool| -> Result<(), String> {
            let mapped = match finishing {
                Some(view) if row.1.dst_value().is_some() => view.relation_binding(function, row)?,
                Some(view) => Some(match context {
                    CallValidationContextV1::Lowering(_) => view.captured_effect_binding(function, row),
                    CallValidationContextV1::Finalized(_) => view.required_effect_binding(function, row),
                }.map_err(|error| format!("{error} rhs-original={row:?}"))?),
                None => Some(row.clone()),
            };
            match mapped {
                Some(actual) => {
                    if !super::super::super::super::physical_boundary::check_binding(function, None, actual.0, &actual.1)? {
                        return Err(freeze("field-write-rhs/producer-missing"));
                    }
                    if mandatory { ordered.push(actual); }
                },
                None if !mandatory && matches!(row.1, MirInstruction::Const { .. } | MirInstruction::Copy { .. }) => {},
                None => return Err(freeze("field-write-rhs/producer-removed")),
            }
            Ok(())
        };
        for node in &self.nodes {
            let original = &source.nodes()[node.node];
            match (original.source(), &node.proof) {
                (Node::Literal(literal), FieldWriteRhsProofV1::Literal(row)) => {
                    let expected = match literal { Literal::Integer(value) | Literal::TypedInteger { value, .. } => *value,
                        _ => return Err(freeze("field-write-rhs/literal-kind")) };
                    if !matches!(row.1, MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(value) }
                        if dst == node.value && value == expected) { return Err(freeze("field-write-rhs/literal-value")); }
                    if let Literal::TypedInteger { declared_type_name, value } = literal {
                        if function.metadata.exact_numeric_const_facts.get(&node.value).is_none_or(|fact|
                            fact.declared_type_name != declared_type_name.as_ref() || fact.value != i128::from(*value)) {
                            return Err(freeze("field-write-rhs/exact-literal-fact"));
                        }
                    }
                    require(row, true)?;
                },
                (Node::Reference(_), FieldWriteRhsProofV1::Reference { read, producer }) => {
                    let (binding, _) = self.recipe.reference(node.node).ok_or_else(|| freeze("field-write-rhs/reference-kind"))?;
                    if read.value_for(source.owner(), original.site().node(), binding).map_err(|issue| issue.to_string())? != node.value {
                        return Err(freeze("field-write-rhs/reference-value"));
                    }
                    if let Some(row) = producer {
                        if row.1.dst_value() != Some(node.value) { return Err(freeze("field-write-rhs/reference-producer")); }
                        require(row, false)?;
                    } else if !function.params.contains(&node.value) { return Err(freeze("field-write-rhs/reference-parameter")); }
                },
                (Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }), FieldWriteRhsProofV1::Field(rows)) => {
                    let expected = context.ledger().expression_field_read_bindings_v1(
                        &OwnedExprSiteV1::new(source.owner(), original.site().clone()), source, context)?;
                    if &expected != rows || rows.last().and_then(|row| row.1.dst_value()) != Some(node.value) {
                        return Err(freeze("field-write-rhs/field-producer"));
                    }
                    for row in rows { require(row, true)?; }
                },
                (Node::Binary(binary), FieldWriteRhsProofV1::Binary { append, operands, scalar }) => {
                    let MirInstruction::BinOp { dst, op, lhs, rhs } = append.1 else { return Err(freeze("field-write-rhs/operator-producer")); };
                    if dst != node.value || op != arithmetic_operator(binary.operator())?
                        || operands[0].values() != (self.value_at(binary.lhs())?, lhs)
                        || operands[1].values() != (self.value_at(binary.rhs())?, rhs) {
                        return Err(freeze("field-write-rhs/operator-operands"));
                    }
                    for relation in operands { relation.verify_projected(function, &mut |row|
                        match finishing { Some(view) => view.relation_binding(function, row), None => Ok(Some(row.clone())) })?; }
                    if matches!(binary.operator(), Operator::Subtract | Operator::Divide) {
                        let scalar = scalar.as_ref().ok_or_else(|| freeze("field-write-rhs/scalar-original"))?;
                        if !Rc::ptr_eq(scalar.source().plan(), source) || scalar.source().binary().site() != original.site()
                            || scalar.original() != append || scalar.children() != (self.value_at(binary.lhs())?, self.value_at(binary.rhs())?)
                            || !scalar.operand_relations().iter().zip(operands).all(|(a, b)| Rc::ptr_eq(a, b)) {
                            return Err(freeze("field-write-rhs/scalar-source"));
                        }
                    } else if scalar.is_some() { return Err(freeze("field-write-rhs/unexpected-scalar")); }
                    require(append, true)?;
                },
                _ => return Err(freeze("field-write-rhs/source-proof-kind")),
            }
        }
        super::super::print_expression_projection::normal_order(function, &ordered)?;
        if finishing.is_none() { self.validate_original_scope(function)?; }
        Ok(())
    }
}
fn arithmetic_operator(operator: Operator) -> Result<crate::mir::BinaryOp, String> {
    use crate::mir::BinaryOp as Op;
    Ok(match operator {
        Operator::Add => Op::Add, Operator::Subtract => Op::Sub, Operator::Multiply => Op::Mul,
        Operator::Divide => Op::Div, Operator::Modulo => Op::Mod, Operator::BitAnd => Op::BitAnd,
        Operator::BitOr => Op::BitOr, Operator::BitXor => Op::BitXor, Operator::Shl => Op::Shl,
        Operator::Shr => Op::Shr, _ => return Err(freeze("field-write-rhs/non-arithmetic-operator")),
    })
}

#[cfg(test)]
#[path = "field_write_rhs_projection_tests.rs"]
mod tests;
