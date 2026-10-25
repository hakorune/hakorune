//! Orthogonal union of SAME original seeds; no admission or source classifier.
use super::*;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1 as Rep;
use crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1;
#[path = "ordinary_new_borrowed_formal_source_sum_binary.rs"]
mod binary;
use binary::SourceBinaryTransferV1;
#[path = "ordinary_new_borrowed_formal_source_sum_loans.rs"]
mod loans;
pub(in crate::mir) use loans::SourceValueClassFactsV1;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use loans::OriginalIntegerBasisV1;

pub(super) enum SourcePointExpansionV1 {
    Entry(BindingRefV1),
    Join(Vec<SourceValueRefV1>),
    Seed(OriginalFormalActualSeedV1),
    Unknown,
}
type MapReceiverLender<'a> = dyn FnMut(&std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>) -> Option<Rc<crate::mir::normal_callable_semantic_package::core_method_source::FieldMapReceiverSourceV1>> + 'a;
type StaticSource = crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
type ReturnExpander<'a> = dyn FnMut(
        &Rc<StaticSource>,
        bool,
    ) -> Result<Option<Vec<(OwnedExprSiteV1, OriginalFormalActualSeedV1)>>, String>
    + 'a;
type PointExpander<'a> =
    dyn FnMut(FunctionOwnerIdV1, SourceValueRefV1) -> Result<SourcePointExpansionV1, String> + 'a;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir) enum SourceValueAlternativeV1 {
    Text,
    SignedI64,
    /// An unannotated formal with no source callers still has its declared
    /// tagged carrier. This is a runtime class, not a concrete payload tag.
    TaggedDynamic,
    Bool,
    Null,
    Other,
    Object(Box<str>),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct SourceValueSumV1(BTreeSet<SourceValueAlternativeV1>);
impl SourceValueSumV1 {
    pub(in crate::mir) fn alternatives(&self) -> &BTreeSet<SourceValueAlternativeV1> {
        &self.0
    }
    pub(in crate::mir) fn signed_only(&self) -> bool {
        self.0.len() == 1 && self.0.contains(&SourceValueAlternativeV1::SignedI64)
    }
    pub(in crate::mir) fn text_only(&self) -> bool {
        self.0.len() == 1 && self.0.contains(&SourceValueAlternativeV1::Text)
    }
    pub(in crate::mir) fn tagged_dynamic_only(&self) -> bool {
        self.0.len() == 1 && self.0.contains(&SourceValueAlternativeV1::TaggedDynamic)
    }
    pub(in crate::mir) fn bool_only(&self) -> bool {
        self.0.len() == 1 && self.0.contains(&SourceValueAlternativeV1::Bool)
    }
    pub(in crate::mir) fn finite_input(&self) -> bool {
        !self.0.is_empty()
            && self.0.iter().all(|value| {
                matches!(
                    value,
                    SourceValueAlternativeV1::Text
                        | SourceValueAlternativeV1::SignedI64
                        | SourceValueAlternativeV1::Bool
                        | SourceValueAlternativeV1::Null
                )
            })
    }
    fn permits_object(&self, name: &str) -> bool {
        self.0.iter().all(|value| matches!(value, SourceValueAlternativeV1::Null)
            || matches!(value, SourceValueAlternativeV1::Object(actual) if actual.as_ref() == name))
    }
}

#[derive(Debug)]
pub(super) struct OriginalSourceSumsV1 {
    pub(super) new_calls: Rc<new_seeds::OriginalNewSeedsV1>,
    pub(super) values: BTreeMap<BindingRefV1, Rc<SourceValueSumV1>>,
    pub(super) core_calls: BTreeMap<OwnedExprSiteV1, Rc<ClosedOriginalCoreSourceV1>>,
    pub(super) static_sources: BTreeMap<OwnedExprSiteV1, Rc<StaticSource>>,
    pub(super) return_sites: BTreeMap<FunctionOwnerIdV1, Box<[OwnedExprSiteV1]>>,
    pub(super) points: BTreeMap<(FunctionOwnerIdV1, SourceValueRefV1), Rc<SourceValueSumV1>>,
    pub(super) flows: BTreeMap<
        FunctionOwnerIdV1,
        std::sync::Arc<
            crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1,
        >,
    >,
    graph: OriginalSourceSumGraphV1,
}
#[derive(Debug, Clone)]
struct OriginalSourceSumGraphV1 {
    nodes: Vec<Node>,
    points: BTreeMap<(FunctionOwnerIdV1, SourceValueRefV1), usize>,
    formals: BTreeMap<BindingRefV1, usize>,
    core_requests: Vec<CoreRequestNode>,
}
impl OriginalSourceSumsV1 {
    pub(super) fn join_instance_result_classes_v1(
        &mut self,
        classes: &[(
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::CallTargetReferenceV1,
            SourceValueAlternativeV1,
        )],
    ) -> bool {
        let mut nodes = self.graph.nodes.clone();
        for node in &mut nodes {
            let Some(reference) = &node.instance_result else { continue; };
            let mut matches = classes.iter().filter(|(candidate, _)| candidate == reference);
            let Some((_, class)) = matches.next() else { continue; };
            if matches.next().is_some()
                || !matches!(class, SourceValueAlternativeV1::SignedI64 | SourceValueAlternativeV1::Bool)
            {
                return false;
            }
            node.atoms.insert(class.clone());
            node.unconditional = true;
        }
        let (_, points, formals, mut core_calls) = project_graph_v1(
            &nodes,
            &self.graph.points,
            &self.graph.formals,
            &self.graph.core_requests,
        );
        // Already-closed Core receipts retain this exact canonical allocation.
        // Re-solving unchanged premises must not reissue their source proof.
        for (site, current) in &mut core_calls {
            if let Some(original) = self.core_calls.get(site).filter(|original| original.matches(current)) {
                *current = Rc::clone(original);
            }
        }
        self.values = formals;
        self.points = points;
        self.core_calls = core_calls;
        true
    }
    pub(super) fn instance_result_references_v1(
        &self,
    ) -> impl Iterator<Item = &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::CallTargetReferenceV1>
    {
        self.graph.nodes.iter().filter_map(|node| node.instance_result.as_ref())
    }
}
/// Closed source premises, not an evaluation, ABI or cleanup capability.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package)
struct ClosedOriginalCoreSourceV1 {
    method: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    request: OriginalCoreOperandRequestV1,
    field_map: Option<Rc<crate::mir::normal_callable_semantic_package::core_method_source::FieldMapReceiverSourceV1>>,
    receiver_core: Option<Rc<ClosedOriginalCoreSourceV1>>,
    text: Box<[SourceValueSumV1]>,
    i64: Box<[SourceValueSumV1]>,
}
impl ClosedOriginalCoreSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn receiver_core(&self) -> Option<&Rc<Self>> {
        self.receiver_core.as_ref()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn method(
        &self,
    ) -> &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1> {
        &self.method
    }
    pub(in crate::mir::normal_callable_semantic_package) fn manifest(
        &self,
    ) -> crate::mir::core_method_result_kind::CoreMethodManifestRowRefV2 {
        self.request.manifest()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn field_map(&self)
    -> Option<&Rc<crate::mir::normal_callable_semantic_package::core_method_source::FieldMapReceiverSourceV1>>{
        self.field_map.as_ref()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn dynamic_i64_operand_contains(
        &self,
        site: &SourceExprSiteV1,
    ) -> bool {
        self.request
            .i64_ordinals()
            .iter()
            .zip(self.i64.iter())
            .any(|(ordinal, sum)| {
                sum.tagged_dynamic_only()
                    && self
                        .method
                        .arguments()
                        .get(*ordinal as usize)
                        .is_some_and(|argument| {
                            site.node()
                                .segments()
                                .starts_with(argument.site().node().segments())
                        })
            })
    }
    fn matches(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.method, &other.method)
            && std::ptr::eq(self.manifest().row(), other.manifest().row())
            && self.manifest().arity() == other.manifest().arity()
            && self.text == other.text
            && self.i64 == other.i64
            && match (&self.receiver_core, &other.receiver_core) {
                (None, None) => true,
                (Some(left), Some(right)) => left.matches(right),
                _ => false,
            }
            && match (&self.field_map, &other.field_map) {
                (None, None) => true,
                (Some(left), Some(right)) => left.matches_source(right),
                _ => false,
            }
    }
}
#[derive(Debug, Clone)]
struct CoreRequestNode {
    node: usize,
    field_map: Option<Rc<crate::mir::normal_callable_semantic_package::core_method_source::FieldMapReceiverSourceV1>>,
    receiver_core: Option<std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>>,
    text: Vec<usize>,
    i64: Vec<usize>,
    method: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    request: OriginalCoreOperandRequestV1,
}
#[derive(Debug, Default, Clone)]
struct Node {
    binary: Option<SourceBinaryTransferV1>,
    atoms: BTreeSet<SourceValueAlternativeV1>,
    union: Vec<usize>,
    required_i64: Vec<usize>,
    required_text: Vec<usize>,
    required_core: Vec<usize>,
    constraints: Vec<Box<str>>,
    invalid: bool,
    unconditional: bool,
    instance_result: Option<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::CallTargetReferenceV1>,
}
impl Node {
    fn dependencies(&self) -> impl Iterator<Item = usize> + '_ {
        self.union
            .iter()
            .chain(&self.required_i64)
            .chain(&self.required_text)
            .chain(&self.required_core)
            .copied()
            .chain(self.binary.iter().flat_map(|binary| binary.operands))
    }
}
struct Graph<'a> {
    points: BTreeMap<(FunctionOwnerIdV1, SourceValueRefV1), usize>,
    expand: &'a mut PointExpander<'a>,
    expand_returns: &'a mut ReturnExpander<'a>,
    returns: BTreeMap<FunctionOwnerIdV1, usize>,
    static_sources: BTreeMap<OwnedExprSiteV1, Rc<StaticSource>>,
    return_sites: BTreeMap<FunctionOwnerIdV1, Box<[OwnedExprSiteV1]>>,
    flow_owners: BTreeSet<FunctionOwnerIdV1>,
    expansion_error: Option<String>,
    nodes: Vec<Node>,
    formals: BTreeMap<BindingRefV1, usize>,
    constructors: &'a crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    core_requests: Vec<CoreRequestNode>,
    map_receiver: &'a mut MapReceiverLender<'a>,
}
impl Graph<'_> {
    fn point(&mut self, owner: FunctionOwnerIdV1, value: SourceValueRefV1) -> usize {
        if let Some(node) = self.points.get(&(owner, value)) {
            return *node;
        }
        let id = self.nodes.len();
        self.nodes.push(Node::default());
        self.points.insert((owner, value), id);
        let mut node = Node::default();
        match (self.expand)(owner, value) {
            Ok(SourcePointExpansionV1::Entry(binding)) => node.union.push(self.formal(binding)),
            Ok(SourcePointExpansionV1::Join(incoming)) => {
                node.invalid = incoming.is_empty();
                for incoming in incoming {
                    node.union.push(self.point(owner, incoming));
                }
            }
            Ok(SourcePointExpansionV1::Seed(seed)) => node.union.push(self.seed(&seed)),
            Ok(SourcePointExpansionV1::Unknown) => node.invalid = true,
            Err(error) => {
                self.expansion_error.get_or_insert(error);
                node.invalid = true;
            }
        }
        self.nodes[id] = node;
        id
    }

    fn static_return(&mut self, source: &Rc<StaticSource>) -> usize {
        // Validate every participating incoming Rc before sharing its callee node.
        if let Some(same) = self.static_sources.get(source.call_site()) {
            if !Rc::ptr_eq(same, source) {
                self.expansion_error
                    .get_or_insert_with(|| freeze("source-sum/caller-owner"));
            }
            if let Some(&id) = self.returns.get(&source.callee_owner()) {
                return id;
            }
        }
        let expanded =
            (self.expand_returns)(source, !self.returns.contains_key(&source.callee_owner()));
        self.static_sources
            .insert(source.call_site().clone(), Rc::clone(source));
        if let Some(&id) = self.returns.get(&source.callee_owner()) {
            match &expanded {
                Err(error) => {
                    self.expansion_error.get_or_insert(error.clone());
                    self.nodes[id].invalid = true;
                }
                Ok(None) => self.nodes[id].invalid = true,
                _ => {}
            }
            return id;
        }
        let id = self.nodes.len();
        self.nodes.push(Node::default());
        self.returns.insert(source.callee_owner(), id);
        let mut node = Node::default();
        match expanded {
            Ok(Some(seeds))
                if !seeds.is_empty() && self.flow_owners.contains(&source.callee_owner()) =>
            {
                self.return_sites.insert(
                    source.callee_owner(),
                    seeds.iter().map(|(site, _)| site.clone()).collect(),
                );
                for (_, seed) in seeds {
                    node.union.push(self.seed(&seed));
                }
            }
            Err(error) => {
                self.expansion_error.get_or_insert(error);
                node.invalid = true;
            }
            _ => node.invalid = true,
        }
        self.nodes[id] = node;
        id
    }

    fn formal(&mut self, binding: BindingRefV1) -> usize {
        if let Some(node) = self.formals.get(&binding) {
            return *node;
        }
        let node = self.nodes.len();
        self.nodes.push(Node::default());
        self.formals.insert(binding, node);
        node
    }
    fn seed(&mut self, seed: &OriginalFormalActualSeedV1) -> usize {
        use SourceValueAlternativeV1 as Value;
        let mut row = Node::default();
        let id = self.nodes.len();
        self.nodes.push(Node::default());
        match seed {
            OriginalFormalActualSeedV1::PointValue { owner, value, .. } => {
                if let Some(value) = value {
                    row.union.push(self.point(*owner, *value));
                } else {
                    row.invalid = true;
                }
            }
            OriginalFormalActualSeedV1::Binary {
                owner,
                source,
                lhs,
                rhs,
            } => {
                row.binary = Some(SourceBinaryTransferV1 {
                    owner: *owner,
                    source: source.clone(),
                    operands: [self.seed(lhs), self.seed(rhs)],
                });
            }
            OriginalFormalActualSeedV1::Text => {
                row.atoms.insert(Value::Text);
            }
            OriginalFormalActualSeedV1::Scalar(kind) => {
                row.atoms.insert(match kind {
                    crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer => {
                        Value::SignedI64
                    }
                    crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool => {
                        Value::Bool
                    }
                });
            }
            OriginalFormalActualSeedV1::SourceInteger | OriginalFormalActualSeedV1::Other => {
                row.atoms.insert(Value::Other);
            }
            OriginalFormalActualSeedV1::Null => {
                row.atoms.insert(Value::Null);
            }
            OriginalFormalActualSeedV1::Class(name) => {
                row.atoms.insert(Value::Object(name.clone()));
            }
            OriginalFormalActualSeedV1::ReturnedField(source) => {
                if let Some(name) = source.class() {
                    row.atoms.insert(Value::Object(name.into()));
                }
                if source.nullable() || source.class().is_none() {
                    row.atoms.insert(Value::Null);
                }
            }
            OriginalFormalActualSeedV1::Mixed { source, anchor } => {
                row.atoms.insert(Value::Object(source.fresh_class().into()));
                row.union.push(self.seed(anchor));
            }
            OriginalFormalActualSeedV1::Forward {
                origin,
                declared_constraint,
            } => {
                row.union.push(self.formal(*origin));
                row.constraints.extend(declared_constraint.iter().cloned());
            }
            OriginalFormalActualSeedV1::SourceOnly(source) => {
                row.union.push(self.seed(source));
            }
            OriginalFormalActualSeedV1::ProviderStatic(source) => {
                if source.matches_constructor(self.constructors) {
                    row.atoms.insert(Value::SignedI64);
                } else {
                    row.invalid = true;
                }
            }
            OriginalFormalActualSeedV1::MethodResult(source) => {
                if source
                    .method()
                    .arguments()
                    .iter()
                    .enumerate()
                    .any(|(ordinal, argument)| argument.ordinal() as usize != ordinal)
                {
                    row.invalid = true;
                }
                if let Some(original) = source.static_source_v1() {
                    row.union.push(self.static_return(original));
                } else if let Some(reference) = source.instance_result_v1() {
                    row.instance_result = Some(reference.clone());
                } else {
                    match source.normal_representation() {
                        Some(Rep::ExactString) => {
                            row.atoms.insert(Value::Text);
                        }
                        Some(Rep::ExactI64) => {
                            row.atoms.insert(Value::SignedI64);
                        }
                        Some(Rep::ExactBool) => {
                            row.atoms.insert(Value::Bool);
                        }
                        Some(Rep::ExactNominalBox { box_name }) => {
                            row.atoms.insert(Value::Object(box_name.into()));
                        }
                        None if source.core_operand_request().is_some_and(|request|
                            request.manifest().row().op == crate::mir::core_method_op::CoreMethodOp::MapKeys) => {
                            // No primitive class is lent. The exact structural
                            // result and its receiver relation live on Core.
                            row.atoms.insert(Value::Other);
                        }
                        None => row.invalid = true,
                    }
                }
                let i64: Vec<_> = source
                    .required_i64()
                    .iter()
                    .map(|seed| self.seed(seed))
                    .collect();
                let text: Vec<_> = source
                    .required_text()
                    .iter()
                    .map(|seed| self.seed(seed))
                    .collect();
                if let Some(request) = source.core_operand_request() {
                    let field_map = if matches!(request.manifest().row().op,
                        crate::mir::core_method_op::CoreMethodOp::MapHas | crate::mir::core_method_op::CoreMethodOp::MapKeys)
                    {
                        let proof = (self.map_receiver)(source.method());
                        row.invalid |= proof.is_none();
                        proof
                    } else {
                        None
                    };
                    let receiver_core = if request.manifest().row().op == crate::mir::core_method_op::CoreMethodOp::ArrayLen {
                        match source.receiver_core() {
                            Some((original, seed)) if original.owner() == source.method().owner()
                                && original.result_site() == source.method().receiver_site() => {
                                row.required_core.push(self.seed(seed));
                                Some(std::sync::Arc::clone(original))
                            }
                            _ => { row.invalid = true; None }
                        }
                    } else { None };
                    self.core_requests.push(CoreRequestNode {
                        node: id,
                        field_map,
                        receiver_core,
                        method: std::sync::Arc::clone(source.method()),
                        request,
                        text: text.clone(),
                        i64: i64.clone(),
                    });
                }
                // Catalog result representation stays independent of operand
                // execution premises; conditional results need those premises.
                if source.core_operand_request().is_none() || source.conditional_result() {
                    row.required_i64 = i64;
                    row.required_text = text;
                }
            }
            OriginalFormalActualSeedV1::Conflict | OriginalFormalActualSeedV1::SumUnknown(_) => {
                row.invalid = true
            }
        }
        row.unconditional =
            row.required_i64.is_empty() && row.required_text.is_empty() && !row.atoms.is_empty();
        self.nodes[id] = row;
        id
    }
    fn actual(&mut self, formal: BindingRefV1, seed: Option<&OriginalFormalActualSeedV1>) {
        let node = self.formal(formal);
        match seed {
            Some(seed) => {
                let actual = self.seed(seed);
                self.nodes[node].union.push(actual);
            }
            None => self.nodes[node].invalid = true,
        }
    }
}

fn project_graph_v1(
    nodes: &[Node],
    point_nodes: &BTreeMap<(FunctionOwnerIdV1, SourceValueRefV1), usize>,
    formal_nodes: &BTreeMap<BindingRefV1, usize>,
    core_requests: &[CoreRequestNode],
) -> (
    Vec<Option<SourceValueSumV1>>,
    BTreeMap<(FunctionOwnerIdV1, SourceValueRefV1), Rc<SourceValueSumV1>>,
    BTreeMap<BindingRefV1, Rc<SourceValueSumV1>>,
    BTreeMap<OwnedExprSiteV1, Rc<ClosedOriginalCoreSourceV1>>,
) {
    let values = solve(nodes);
    let mut core_calls: BTreeMap<_, Option<Rc<ClosedOriginalCoreSourceV1>>> = BTreeMap::new();
    for request in core_requests.iter().cloned() {
        let dynamic_integer_boundary = request.request.manifest().row().op
            == crate::mir::core_method_op::CoreMethodOp::StringSubstring;
        let premises_closed = request.text.iter().all(|dependency| {
            values[*dependency]
                .as_ref()
                .is_some_and(SourceValueSumV1::text_only)
        }) && request.i64.iter().all(|dependency| {
            values[*dependency].as_ref().is_some_and(|sum| {
                sum.signed_only() || (dynamic_integer_boundary && sum.tagged_dynamic_only())
            })
        });
        let receiver_core = request.receiver_core.as_ref().and_then(|method|
            core_calls.get(&OwnedExprSiteV1::new(method.owner(), method.site().clone()))
                .and_then(Option::as_ref)
                .filter(|inner| std::sync::Arc::ptr_eq(inner.method(), method)
                    && inner.manifest().row().op == crate::mir::core_method_op::CoreMethodOp::MapKeys
                    && inner.field_map().is_some())).cloned();
        let receiver_closed = request.receiver_core.is_some() == receiver_core.is_some();
        let closed = values[request.node]
            .as_ref()
            .filter(|_| premises_closed && receiver_closed)
            .and_then(|_| {
                Some(Rc::new(ClosedOriginalCoreSourceV1 {
                    method: std::sync::Arc::clone(&request.method),
                    request: request.request,
                    field_map: request.field_map.clone(),
                    receiver_core,
                    text: request
                        .text
                        .iter()
                        .map(|dependency| values[*dependency].clone())
                        .collect::<Option<Vec<_>>>()?
                        .into_boxed_slice(),
                    i64: request
                        .i64
                        .iter()
                        .map(|dependency| values[*dependency].clone())
                        .collect::<Option<Vec<_>>>()?
                        .into_boxed_slice(),
                }))
            });
        let site = OwnedExprSiteV1::new(request.method.owner(), request.method.site().clone());
        match core_calls.entry(site) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(closed);
            }
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                if !matches!((entry.get(), &closed), (Some(a), Some(b)) if a.matches(b)) {
                    entry.insert(None);
                }
            }
        }
    }
    let points = point_nodes
        .iter()
        .filter_map(|(point, node)| values[*node].clone().map(|value| (point.clone(), Rc::new(value))))
        .collect();
    let formals = formal_nodes
        .iter()
        .filter_map(|(binding, node)| values[*node].clone().map(|value| (*binding, Rc::new(value))))
        .collect();
    // A later conflicting demand may invalidate an earlier inner request.
    // An outer receipt must retain the final canonical dependency, not a
    // snapshot of an inner entry that the merge subsequently withdrew.
    loop {
        let invalid: Vec<_> = core_calls.iter().filter_map(|(site, value)| {
            let inner = value.as_ref()?.receiver_core()?;
            let key = OwnedExprSiteV1::new(inner.method().owner(), inner.method().site().clone());
            (!core_calls.get(&key).and_then(Option::as_ref)
                .is_some_and(|current| Rc::ptr_eq(current, inner))).then(|| site.clone())
        }).collect();
        if invalid.is_empty() { break; }
        for site in invalid { core_calls.insert(site, None); }
    }
    let core_calls = core_calls
        .into_iter()
        .filter_map(|(site, value)| value.map(|value| (site, value)))
        .collect();
    (values, points, formals, core_calls)
}

pub(super) fn resolve_original_source_sums_v1(
    source: &OriginalFormalSeedProductV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    initializer_roots: &[OriginalFormalActualSeedV1],
    flows: BTreeMap<
        FunctionOwnerIdV1,
        std::sync::Arc<
            crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1,
        >,
    >,
    expand: &mut PointExpander<'_>,
    expand_returns: &mut ReturnExpander<'_>,
    map_receiver: &mut MapReceiverLender<'_>,
) -> Result<OriginalSourceSumsV1, String> {
    let mut graph = Graph {
        points: BTreeMap::new(),
        expand,
        expansion_error: None,
        nodes: Vec::new(),
        formals: BTreeMap::new(),
        constructors,
        core_requests: Vec::new(),
        map_receiver,
        expand_returns,
        returns: BTreeMap::new(),
        static_sources: BTreeMap::new(),
        return_sites: BTreeMap::new(),
        flow_owners: flows.keys().copied().collect(),
    };
    let mut registered = BTreeSet::new();
    for contract in contracts
        .iter()
        .filter(|row| source.source_owners.contains(&row.owner))
    {
        for formal in &contract.parameters {
            let node = graph.formal(formal.binding);
            registered.insert(formal.binding);
            graph.nodes[node].invalid |= source.sum_vetoes.contains(&contract.owner);
            if let CallableParameterContractKindV1::DeclaredObject(name) = &formal.kind {
                graph.nodes[node].constraints.push(name.clone());
            }
        }
    }
    for ctor in constructors
        .rows()
        .iter()
        .filter(|row| row.kind() == crate::parser::ConstructorSourceKindV1::Birth)
    {
        for formal in ctor.formal_contracts() {
            graph.formal(formal.binding());
            registered.insert(formal.binding());
        }
    }
    for row in &source.rows {
        for ((_, _, formal), seed) in row.incoming.arguments.iter().zip(&row.seeds) {
            graph.actual(*formal, Some(seed));
        }
        // A checked guard may classify an argument as Scalar while its source
        // forward still retains the exact read. Demand that point separately:
        // the guard's class must not erase the original reaching-value proof.
        if let Some(target) = row.incoming.source.instance() {
            for forward in target.object_source_forwards().into_iter().flatten() {
                if forward.call() != &row.incoming.call
                    || forward.target() != target.target()
                    || target.argument_sites().get(forward.ordinal() as usize)
                        != Some(forward.site().site())
                    || forward.binding().owner() != row.incoming.call.owner()
                {
                    return Err(freeze("source-sum/caller-owner"));
                }
                if let Some(value) = flows.get(&row.incoming.call.owner())
                    .filter(|flow| flow.owner() == row.incoming.call.owner() && flow.is_complete())
                    .and_then(|flow| flow.read(forward.site().site(), forward.binding()))
                {
                    graph.point(row.incoming.call.owner(), value);
                }
            }
        }
    }
    source
        .new_calls
        .visit_raw_source_seeds_v1(constructors, |formal, seed| graph.actual(formal, seed));
    for root in initializer_roots {
        graph.seed(root);
    }
    if let Some(error) = graph.expansion_error.take() {
        return Err(error);
    }
    for (binding, node) in &graph.formals {
        let row = &mut graph.nodes[*node];
        if !registered.contains(binding) {
            row.invalid = true;
        } else if row.union.is_empty() && !row.invalid {
            let untyped = contracts
                .iter()
                .filter(|contract| contract.owner == binding.owner())
                .flat_map(|contract| &contract.parameters)
                .any(|formal| {
                    formal.binding == *binding
                        && formal.kind == CallableParameterContractKindV1::OpaqueHandle
                });
            if untyped {
                row.atoms.insert(SourceValueAlternativeV1::TaggedDynamic);
                row.unconditional = true;
            } else {
                row.invalid = true;
            }
        }
    }
    let static_sources = graph.static_sources;
    let return_sites = graph.return_sites;
    let resolved_graph = OriginalSourceSumGraphV1 {
        nodes: graph.nodes,
        points: graph.points,
        formals: graph.formals,
        core_requests: graph.core_requests,
    };
    let (_, points, values, core_calls) = project_graph_v1(
        &resolved_graph.nodes,
        &resolved_graph.points,
        &resolved_graph.formals,
        &resolved_graph.core_requests,
    );
    Ok(OriginalSourceSumsV1 {
        new_calls: Rc::clone(&source.new_calls),
        static_sources,
        return_sites,
        flows,
        points,
        core_calls,
        values,
        graph: resolved_graph,
    })
}

#[path = "ordinary_new_borrowed_formal_source_sum_solver.rs"]
mod solver;
use solver::solve;
