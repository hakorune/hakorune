//! The SAME checked condition projected once into the portable closed region.
//! References and a direct root call use imports; nested calls remain effect
//! nodes in the Bool tree so the source BoolCondition owner preserves reach.
use super::{LoopBoolCompareV1 as Compare, LoopBoolCompareDomainV1 as Domain,
    LoopBoolNodeV1 as Node, LoopBoolRegionV1, LoopValueClassV2 as Class,
    LoopOperationV2, LoopValueKeyV1};
use crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1;
use crate::mir::resolved_semantics::{ResolvedBinaryOperatorV1 as Op, ResolvedLiteralSourceV1 as Literal};
use crate::mir::resolved_value_profile::{
    expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
    expression_plan::ExpressionNodeSourceV1 as Source,
    expression_projection::ExpressionLeafClassV1 as Leaf, expression_class::ExpressionValueClassV1 as Value};
use crate::mir::dynamic_operator_contract::{DynamicOperatorFamilyV1 as Family, DynamicOperatorValueClassV1 as Operand};
use std::{collections::{BTreeMap, BTreeSet}, rc::Rc};

#[derive(Debug)]
pub(crate) struct CheckedLoopBoolExpressionV1 {
    frame: Rc<CheckedBoolConditionFrameV1>,
    region: LoopBoolRegionV1,
    imports: Box<[(usize, Class)]>,
    calls: Box<[(usize, Class)]>,
}
impl CheckedLoopBoolExpressionV1 {
    pub(crate) fn frame(&self) -> &Rc<CheckedBoolConditionFrameV1> { &self.frame }
    pub(crate) fn imports(&self) -> &[(usize, Class)] { &self.imports }
    pub(crate) fn calls(&self) -> &[(usize, Class)] { &self.calls }
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
        Some(Leaf::Value(Value::Dynamic)) => Ok(Class::Dynamic),
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

fn owned_results_are_source_covered(frame: &CheckedBoolConditionFrameV1) -> bool {
    use crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1 as NormalResult;
    use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1;

    let evaluation = frame.evaluation();
    let mut expected = Vec::new();
    for &node in evaluation.ordered_nodes() {
        let core_owns = evaluation.cores().get(&node).is_some_and(|core| core.owns_result());
        let call_owns = frame.calls().iter().any(|call| {
            let ordinary_text_result = matches!(call.result_source(),
                Some(LocalCallResultSourceV1::Ordinary(loan))
                    if call.result() == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Text
                        && loan.result() == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Text);
            call.owner() == evaluation.source().owner()
                && evaluation.source().nodes().get(node).is_some_and(|source| call.site().site() == source.site())
                && (ordinary_text_result || call.dynamic_invocation_result_lifecycle_v1().is_some())
        });
        let operation_owns = evaluation.operations().iter().any(|loan| loan.node() == node
            && loan.normal_result_v1() == Some(NormalResult::OwnedStringValue));
        let owners = usize::from(core_owns) + usize::from(call_owns) + usize::from(operation_owns);
        if owners > 1 { return false; }
        if owners == 1 { expected.push(node); }
    }
    expected.as_slice() == evaluation.owned_result_nodes()
}

pub(crate) fn project_source_loop_bool_v1(frame: Rc<CheckedBoolConditionFrameV1>) -> Result<CheckedLoopBoolExpressionV1, String> {
    let evaluation = frame.evaluation();
    if !evaluation.reference_coverage_is_exact() {
        return Err(reject("opaque-or-owned-subtree"));
    }
    let source = frame.source();
    let mut nodes = Vec::new();
    let mut indices = BTreeMap::new();
    let mut imports = Vec::new();
    let mut calls = Vec::new();
    let mut represented_calls = BTreeSet::new();
    let mut eager_calls = 0;
    let mut transferred_nodes = Vec::new();
    let mut operations = 0;
    let mut logical = 0;
    let mut cores = 0;
    for &index in frame.ordered_nodes() {
        let original = source.nodes().get(index).ok_or_else(|| reject("source-node"))?;
        if frame.transfers_node(index) {
            transferred_nodes.push(index);
            continue;
        }
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
            Source::MethodCall => {
                if let Some(core) = evaluation.cores().get(&index) {
                    use crate::mir::core_method_op::CoreMethodOp as Core;
                    if !core.matches_plan(source, index)
                        || core.owns_result() != evaluation.owned_result_nodes().contains(&index) {
                        return Err(reject("source-coverage"));
                    }
                    let original = core.source().source();
                    let child = |site| source.nodes().iter().position(|node| node.site() == site)
                        .and_then(|node| indices.get(&node).copied()).ok_or_else(|| reject("child-order"));
                    let value = child(original.receiver_site())?;
                    let arguments = original.arguments().iter().map(|argument| child(argument.site()))
                        .collect::<Result<Vec<_>, _>>()?;
                    cores += 1;
                    match (core.source().target().row().row().op, arguments.as_slice(), current_class) {
                        (Core::StringLen, [], Class::I64) => Node::TextLength { value },
                        (Core::StringSubstring, [start, end], Class::Text) => Node::TextSubstring { value, start: *start, end: *end },
                        (Core::StringIndexOf, [needle], Class::I64) => Node::TextIndexOf { value, needle: *needle },
                        _ => return Err(reject("hidden-effectful-node")),
                    }
                } else {
                    let mut call_rows = frame.calls().iter().filter(|call| call.site().site() == original.site());
                    let call = call_rows.next().ok_or_else(|| reject("hidden-effectful-node"))?;
                    let call_class = match call.result() {
                        crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Dynamic => Class::Dynamic,
                        crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64 => Class::I64,
                        crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Bool => Class::Bool,
                        crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Text => Class::Text,
                        _ => return Err(reject("call-result-class")),
                    };
                    if call_rows.next().is_some()
                        || call.owner() != source.owner()
                        || call.statement() != source.root().statement()
                        || !call.is_expression_value()
                        || call.result_source().is_none()
                        || call_class != current_class
                        || !represented_calls.insert(index)
                    {
                        return Err(reject("call-source-identity"));
                    }
                    if index == 0 && matches!(call_class, Class::I64 | Class::Bool) {
                        let ordinal = u32::try_from(imports.len()).map_err(|_| reject("import-overflow"))?;
                        imports.push((index, call_class));
                        eager_calls += 1;
                        Node::Import { ordinal, class: call_class }
                    } else {
                        let ordinal = u32::try_from(calls.len()).map_err(|_| reject("call-overflow"))?;
                        calls.push((index, call_class));
                        Node::Call { ordinal, class: call_class }
                    }
                }
            }
            Source::Binary(binary) => {
                let child = |site| source.nodes().iter().position(|node| node.site() == site)
                    .and_then(|node| indices.get(&node).copied()).ok_or_else(|| reject("child-order"));
                let left = child(binary.lhs())?;
                let right = child(binary.rhs())?;
                if matches!(binary.operator(), Op::And | Op::Or) {
                    if !frame.logical_nodes().contains(&index) || current_class != Class::Bool { return Err(reject("logical-source")); }
                    let ExpressionChildrenV1::Known(children) = original.children() else {
                        return Err(reject("logical-reach-source"));
                    };
                    let reach_value = binary.operator() == Op::And;
                    if children.len() != 2 || &children[0].site != binary.lhs()
                        || children[0].reach != ExpressionChildReachV1::Always
                        || &children[1].site != binary.rhs()
                        || !matches!(&children[1].reach, ExpressionChildReachV1::When { condition, value }
                            if condition == binary.lhs() && *value == reach_value) {
                        return Err(reject("logical-reach-source"));
                    }
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
                            (Operand::Dynamic, Operand::I64) if matches!(binary.operator(), Op::Less | Op::Greater) => Domain::DynamicI64,
                            (Operand::I64, Operand::Dynamic) if binary.operator() == Op::Less => Domain::I64Dynamic,
                            (Operand::Dynamic, Operand::String) => Domain::DynamicText,
                            (Operand::String, Operand::Dynamic) => Domain::TextDynamic,
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
    let covered_transfer = |index: usize| {
        let site = &source.nodes()[index].site();
        frame.calls().iter().any(|call| {
            source.nodes().iter().any(|node| node.site() == call.site().site())
                && site.node().segments().starts_with(call.site().site().node().segments())
        }) || evaluation.cores().keys().any(|call| {
            source.nodes().get(*call).is_some_and(|node|
                site.node().segments().starts_with(node.site().node().segments()))
        })
    };
    let coverage_ok = operations == frame.operations().len() && logical == frame.logical_nodes().len()
        && cores == evaluation.cores().len() && owned_results_are_source_covered(&frame)
        && represented_calls.len() == frame.calls().len()
        && imports.len() == frame.references().len() + eager_calls
        && calls.len() + eager_calls == frame.calls().len()
        && transferred_nodes.iter().all(|index| covered_transfer(*index))
        && indices.len() + transferred_nodes.len() == source.nodes().len();
    if !coverage_ok {
        return Err(reject("source-coverage"));
    }
    let root = *indices.get(&0).ok_or_else(|| reject("root"))?;
    let region = LoopBoolRegionV1 { nodes, root };
    region.verify_with_calls(
        &imports.iter().map(|(_, class)| *class).collect::<Vec<_>>(),
        &calls.iter().map(|(_, class)| *class).collect::<Vec<_>>(),
    )
        .map_err(|reason| format!("{} {reason:?}", reject("typed-region")))?;
    Ok(CheckedLoopBoolExpressionV1 { frame, region, imports: imports.into_boxed_slice(), calls: calls.into_boxed_slice() })
}
