//! The SAME checked condition projected once into the portable closed region.
//! Only genuine reference nodes become imports. Calls, fields and owned or
//! transferred subtrees cannot masquerade as an input to this expression.
use super::{LoopBoolCompareV1 as Compare, LoopBoolCompareDomainV1 as Domain,
    LoopBoolNodeV1 as Node, LoopBoolRegionV1, LoopValueClassV2 as Class,
    LoopOperationV2, LoopValueKeyV1};
use crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1;
use crate::mir::resolved_semantics::{ResolvedBinaryOperatorV1 as Op, ResolvedLiteralSourceV1 as Literal};
use crate::mir::resolved_value_profile::{expression_plan::ExpressionNodeSourceV1 as Source,
    expression_projection::ExpressionLeafClassV1 as Leaf, expression_class::ExpressionValueClassV1 as Value};
use crate::mir::dynamic_operator_contract::{DynamicOperatorFamilyV1 as Family, DynamicOperatorValueClassV1 as Operand};
use std::{collections::BTreeMap, rc::Rc};

#[derive(Debug)]
pub(crate) struct CheckedLoopBoolExpressionV1 {
    frame: Rc<CheckedBoolConditionFrameV1>,
    region: LoopBoolRegionV1,
    imports: Box<[(usize, Class)]>,
}
impl CheckedLoopBoolExpressionV1 {
    pub(crate) fn frame(&self) -> &Rc<CheckedBoolConditionFrameV1> { &self.frame }
    pub(crate) fn imports(&self) -> &[(usize, Class)] { &self.imports }
    pub(crate) fn region(&self) -> &LoopBoolRegionV1 { &self.region }
    pub(crate) fn operation(&self, inputs: Vec<LoopValueKeyV1>, result: LoopValueKeyV1) -> Result<LoopOperationV2, String> {
        if inputs.len() != self.imports.len() { return Err(reject("input-coverage")); }
        Ok(LoopOperationV2::BoolExpression { inputs, region: self.region.clone(), result })
    }
    pub(crate) fn matches_frame(&self, original: &Rc<CheckedBoolConditionFrameV1>) -> bool {
        Rc::ptr_eq(original, &self.frame)
    }
}
fn reject(reason: &str) -> String { format!("[freeze:contract][source-loop/bool/{reason}]") }
fn class(frame: &CheckedBoolConditionFrameV1, node: usize) -> Result<Class, String> {
    match frame.class(node) {
        Some(Leaf::Value(Value::I64)) => Ok(Class::I64),
        Some(Leaf::Value(Value::Bool)) => Ok(Class::Bool),
        Some(Leaf::Value(Value::String)) => Ok(Class::Text),
        _ => Err(reject("value-class")),
    }
}
fn compare(op: Op) -> Result<(Compare, Family), String> {
    Ok(match op {
        Op::Equal => (Compare::Equal, Family::Equal), Op::NotEqual => (Compare::NotEqual, Family::NotEqual),
        Op::Less => (Compare::Less, Family::Less), Op::LessEqual => (Compare::LessEqual, Family::LessEqual),
        Op::Greater => (Compare::Greater, Family::Greater), Op::GreaterEqual => (Compare::GreaterEqual, Family::GreaterEqual),
        _ => return Err(reject("comparison-family")),
    })
}

pub(crate) fn project_source_loop_bool_v1(frame: Rc<CheckedBoolConditionFrameV1>) -> Result<CheckedLoopBoolExpressionV1, String> {
    let evaluation = frame.evaluation();
    if !evaluation.reference_coverage_is_exact() || !evaluation.calls().is_empty()
        || !evaluation.cores().is_empty() || !evaluation.owned_result_nodes().is_empty() {
        return Err(reject("opaque-or-owned-subtree"));
    }
    let mut nodes = Vec::new();
    let mut indices = BTreeMap::new();
    let mut imports = Vec::new();
    let source = frame.source();
    let mut operations = 0;
    let mut logical = 0;
    for &index in frame.ordered_nodes() {
        if frame.transfers_node(index) { return Err(reject("transferred-subtree")); }
        let original = source.nodes().get(index).ok_or_else(|| reject("source-node"))?;
        let current_class = class(&frame, index)?;
        let node = match original.source() {
            Source::Reference(_) => {
                let proof = frame.references().get(&index).ok_or_else(|| reject("reference-proof"))?;
                if !proof.matches_node(source, index) { return Err(reject("reference-source")); }
                let ordinal = u32::try_from(imports.len()).map_err(|_| reject("import-overflow"))?;
                imports.push((index, current_class));
                Node::Import { ordinal, class: current_class }
            }
            Source::Literal(Literal::Integer(value)) if current_class == Class::I64 => Node::I64 { value: *value },
            Source::Literal(Literal::Bool(value)) if current_class == Class::Bool => Node::Bool { value: *value },
            Source::Literal(Literal::String(value)) if current_class == Class::Text => Node::Text { value: value.to_string() },
            Source::Binary(binary) => {
                let child = |site| source.nodes().iter().position(|node| node.site() == site)
                    .and_then(|node| indices.get(&node).copied()).ok_or_else(|| reject("child-order"));
                let left = child(binary.lhs())?;
                let right = child(binary.rhs())?;
                if matches!(binary.operator(), Op::And | Op::Or) {
                    if !frame.logical_nodes().contains(&index) || current_class != Class::Bool { return Err(reject("logical-source")); }
                    logical += 1;
                    if binary.operator() == Op::And { Node::And { left, right } } else { Node::Or { left, right } }
                } else {
                    let mut loans = frame.operations().iter().filter(|loan| loan.node() == index);
                    let loan = loans.next().ok_or_else(|| reject("operator-loan"))?;
                    if loans.next().is_some() || !Rc::ptr_eq(loan.source(), source) { return Err(reject("operator-identity")); }
                    let envelope = loan.single_envelope().ok_or_else(|| reject("operator-domain"))?;
                    let domain = envelope.domain();
                    operations += 1;
                    if binary.operator() == Op::Add {
                        if current_class != Class::I64 || domain.family() != Family::Add
                            || domain.left() != Operand::NormalInteger || domain.right() != Operand::NormalInteger {
                            return Err(reject("add-domain"));
                        }
                        Node::AddI64 { left, right }
                    } else {
                        let (op, family) = compare(binary.operator())?;
                        let operand = match (domain.left(), domain.right()) {
                            (Operand::NormalInteger, Operand::NormalInteger) => Domain::I64,
                            (Operand::String, Operand::String) => Domain::Text,
                            _ => return Err(reject("compare-domain")),
                        };
                        if current_class != Class::Bool || domain.family() != family { return Err(reject("compare-family")); }
                        Node::Compare { op, domain: operand, left, right }
                    }
                }
            }
            _ => return Err(reject("hidden-effectful-node")),
        };
        let key = u32::try_from(nodes.len()).map_err(|_| reject("node-overflow"))?;
        if indices.insert(index, key).is_some() { return Err(reject("duplicate-source-node")); }
        nodes.push(node);
    }
    if operations != frame.operations().len() || logical != frame.logical_nodes().len()
        || imports.len() != frame.references().len() || indices.len() != source.nodes().len() {
        return Err(reject("source-coverage"));
    }
    let root = *indices.get(&0).ok_or_else(|| reject("root"))?;
    let region = LoopBoolRegionV1 { nodes, root };
    region.verify(&imports.iter().map(|(_, class)| *class).collect::<Vec<_>>())
        .map_err(|reason| format!("{} {reason:?}", reject("typed-region")))?;
    Ok(CheckedLoopBoolExpressionV1 { frame, region, imports: imports.into_boxed_slice() })
}
