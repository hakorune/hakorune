//! The sole original SUM dependency graph and its closed projections.
//! Admission and source loans remain in the parent domain owner.
use super::*;

#[derive(Debug, Clone)]
pub(super) struct OriginalSourceSumGraphV1 {
    pub(super) fields: BTreeMap<hakorune_mir_defs::CanonicalFieldRefV1, (Rc<FieldSource>, usize)>,
    pub(super) array_fields: BTreeMap<hakorune_mir_defs::CanonicalFieldRefV1, usize>,
    pub(super) nodes: Vec<Node>,
    pub(super) points: BTreeMap<(FunctionOwnerIdV1, SourceValueRefV1), usize>,
    pub(super) formals: BTreeMap<BindingRefV1, usize>,
    pub(super) core_requests: Vec<CoreRequestNode>,
}
#[derive(Debug, Default, Clone)]
pub(super) struct Node {
    pub(super) binary: Option<SourceBinaryTransferV1>,
    pub(super) atoms: BTreeSet<SourceValueAlternativeV1>,
    pub(super) union: Vec<usize>,
    pub(super) required_i64: Vec<usize>,
    pub(super) required_text: Vec<usize>,
    pub(super) required_core: Vec<usize>,
    pub(super) required_object: Vec<(usize, Box<str>)>,
    pub(super) constraints: Vec<Box<str>>,
    pub(super) invalid: bool,
    pub(super) unconditional: bool,
    pub(super) instance_result: Option<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::CallTargetReferenceV1>,
}
impl Node {
    pub(super) fn dependencies(&self) -> impl Iterator<Item = usize> + '_ {
        self.union
            .iter()
            .chain(&self.required_i64)
            .chain(&self.required_text)
            .chain(&self.required_core)
            .copied()
            .chain(self.required_object.iter().map(|(node, _)| *node))
            .chain(self.binary.iter().flat_map(|binary| binary.operands))
    }
}
pub(super) struct Graph<'a> {
    pub(super) points: BTreeMap<(FunctionOwnerIdV1, SourceValueRefV1), usize>,
    pub(super) expand: &'a mut PointExpander<'a>,
    pub(super) expand_returns: &'a mut ReturnExpander<'a>,
    pub(super) returns: BTreeMap<FunctionOwnerIdV1, usize>,
    pub(super) static_sources: BTreeMap<OwnedExprSiteV1, Rc<StaticSource>>,
    pub(super) return_sites: BTreeMap<FunctionOwnerIdV1, Box<[OwnedExprSiteV1]>>,
    pub(super) flow_owners: BTreeSet<FunctionOwnerIdV1>,
    pub(super) expansion_error: Option<String>,
    pub(super) nodes: Vec<Node>,
    pub(super) formals: BTreeMap<BindingRefV1, usize>,
    pub(super) constructors: &'a crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    pub(super) fields: BTreeMap<hakorune_mir_defs::CanonicalFieldRefV1, (Rc<FieldSource>, usize)>,
    pub(super) array_fields: BTreeMap<hakorune_mir_defs::CanonicalFieldRefV1, usize>,
    pub(super) core_requests: Vec<CoreRequestNode>,
    pub(super) map_receiver: &'a mut MapReceiverLender<'a>,
}
/// Default-off (builder debug flag): why a graph node was marked invalid.
/// Kept out of line so the recursive callers do not grow their stack frames.
#[inline(never)]
fn note_invalid(node: usize, why: &str, detail: &dyn std::fmt::Debug) {
    if crate::config::env::builder_debug_enabled() {
        let detail: String = format!("{detail:?}").chars().take(1400).collect();
        eprintln!("[source-sum] node {node} invalid: {why} {detail}");
    }
}

impl Graph<'_> {
    pub(super) fn point(&mut self, owner: FunctionOwnerIdV1, value: SourceValueRefV1) -> usize {
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
                if node.invalid { note_invalid(id, "point join without incoming values", &()); }
                for incoming in incoming {
                    node.union.push(self.point(owner, incoming));
                }
            }
            Ok(SourcePointExpansionV1::Seed(seed)) => node.union.push(self.seed(&seed)),
            Ok(SourcePointExpansionV1::Unknown) => {
                note_invalid(id, "point expansion unknown", &());
                node.invalid = true;
            }
            Err(error) => {
                note_invalid(id, "point expansion error", &error);
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
                    note_invalid(id, "callee return expansion error", &error);
                    self.expansion_error.get_or_insert(error.clone());
                    self.nodes[id].invalid = true;
                }
                Ok(None) => {
                    note_invalid(id, "later demand: callee return has no source", &());
                    self.nodes[id].invalid = true;
                }
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
                note_invalid(id, "callee return expansion error", &error);
                self.expansion_error.get_or_insert(error);
                node.invalid = true;
            }
            _ => {
                note_invalid(id, "callee return: no seeds or owner has no complete flow", &source.callee_owner());
                node.invalid = true;
            }
        }
        self.nodes[id] = node;
        id
    }

    pub(super) fn formal(&mut self, binding: BindingRefV1) -> usize {
        if let Some(node) = self.formals.get(&binding) {
            return *node;
        }
        let node = self.nodes.len();
        self.nodes.push(Node::default());
        self.formals.insert(binding, node);
        node
    }
    pub(super) fn seed(&mut self, seed: &OriginalFormalActualSeedV1) -> usize {
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
            OriginalFormalActualSeedV1::MapField { get, source, .. } => {
                // Keep the same canonical source, never a field-name class guess.
                if source.field().object() == get.payload_source_v1().payload_object_v1() {
                    match self.fields.get(&source.field()) {
                        Some((original, node)) if Rc::ptr_eq(original, source) => row.union.push(*node),
                        _ => row.invalid = true,
                    }
                } else { row.invalid = true; }
            }
            OriginalFormalActualSeedV1::FormalField { read, formal, class, source } => {
                if read.owner() != formal.owner() {
                    row.invalid = true;
                } else {
                    match self.fields.get(&source.field()) {
                        Some((original, node)) if Rc::ptr_eq(original, source) => row.union.push(*node),
                        _ => row.invalid = true,
                    }
                    row.required_object.push((self.formal(*formal), class.clone()));
                }
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
                        None if source.core_operand_request().is_some_and(|request|
                            request.manifest().row().op == crate::mir::core_method_op::CoreMethodOp::ArrayGet) => {
                            let access = source.field_array_access_v1().filter(|access| access.is_dynamic_get());
                            let field = access.map(|access| access.field());
                            if let Some(elements) = field.and_then(|field| self.array_fields.get(&field).copied()) {
                                row.union.push(elements);
                                // Preserve both OOB results unless the same
                                // original field/count/guard proves bounds.
                                if access.is_none_or(|access| !access.get_is_in_bounds_v1()) {
                                    row.atoms.insert(Value::Null);
                                    row.atoms.insert(Value::Text);
                                }
                            } else {
                                row.atoms.insert(Value::Other);
                            }
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
                    let field_array_access = source.field_array_access_v1().cloned();
                    if let Some(access) = &field_array_access {
                        row.invalid |= access.read_manifest_v1().is_none_or(|manifest|
                            !std::ptr::eq(manifest.row(), request.manifest().row()) || manifest.arity() != request.manifest().arity())
                            || access.call().owner() != source.method().owner()
                            || access.call().site() != source.method().site()
                            || access.receiver().site() != source.method().receiver_site()
                            || !matches!(request.manifest().row().op, crate::mir::core_method_op::CoreMethodOp::ArrayLen | crate::mir::core_method_op::CoreMethodOp::ArrayGet);
                    }
                    let receiver_core = if request.manifest().row().op == crate::mir::core_method_op::CoreMethodOp::ArrayLen && field_array_access.is_none() {
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
                        field_array_access,
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
        if row.invalid {
            note_invalid(id, "seed", seed);
        }
        self.nodes[id] = row;
        id
    }
    pub(super) fn actual(&mut self, formal: BindingRefV1, seed: Option<&OriginalFormalActualSeedV1>) {
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

/// The first operand premise of a Core request whose dependency sum does not
/// satisfy the operand contract. `None` means every premise is closed.
#[derive(Debug)]
enum OpenPremise {
    Text(usize),
    I64(usize),
}

fn first_open_premise(
    request: &CoreRequestNode,
    values: &[Option<SourceValueSumV1>],
) -> Option<OpenPremise> {
    let dynamic_integer_boundary = request.request.manifest().row().op
        == crate::mir::core_method_op::CoreMethodOp::StringSubstring;
    let open_text = request.text.iter().position(|dependency| {
        !values[*dependency].as_ref().is_some_and(SourceValueSumV1::text_only)
    });
    let open_i64 = || {
        request.i64.iter().position(|dependency| {
            !values[*dependency].as_ref().is_some_and(|sum| {
                sum.signed_only() || (dynamic_integer_boundary && sum.tagged_dynamic_only())
            })
        })
    };
    open_text.map(OpenPremise::Text).or_else(|| open_i64().map(OpenPremise::I64))
}

/// Default-off diagnostic (builder debug flag): names the open premise and walks
/// its dependency tree, so a mixed sum shows which node contributes each alternative.
fn explain_open_premise(
    nodes: &[Node],
    formal_nodes: &BTreeMap<BindingRefV1, usize>,
    values: &[Option<SourceValueSumV1>],
    request: &CoreRequestNode,
    open: &OpenPremise,
) {
    if !crate::config::env::builder_debug_enabled() {
        return;
    }
    let dependency = match open {
        OpenPremise::Text(ordinal) => request.text[*ordinal],
        OpenPremise::I64(ordinal) => request.i64[*ordinal],
    };
    eprintln!(
        "[source-sum] open premise {open:?} op={:?} owner={:?} site={:?}",
        request.request.manifest().row().op,
        request.method.owner(),
        request.method.site(),
    );
    let mut visited = BTreeSet::new();
    let mut pending = vec![(dependency, 0usize)];
    while let Some((node, depth)) = pending.pop() {
        if depth > 8 || !visited.insert(node) {
            continue;
        }
        let formal = formal_nodes.iter().find(|(_, index)| **index == node).map(|(binding, _)| binding);
        let row = &nodes[node];
        eprintln!(
            "[source-sum] {:indent$}node {node} sum={:?} own_atoms={:?} invalid={} formal={formal:?}",
            "",
            values[node],
            row.atoms,
            row.invalid,
            indent = depth * 2,
        );
        pending.extend(row.union.iter().rev().map(|next| (*next, depth + 1)));
    }
}

/// Default-off (builder debug flag): the solved SUM of every formal.
#[inline(never)]
fn report_formal_sums(formal_nodes: &BTreeMap<BindingRefV1, usize>, values: &[Option<SourceValueSumV1>]) {
    if crate::config::env::builder_debug_enabled() {
        for (binding, node) in formal_nodes {
            eprintln!("[source-sum] formal {binding:?} sum={:?}", values[*node]);
        }
    }
}

pub(super) fn project_graph_v1(
    nodes: &[Node],
    point_nodes: &BTreeMap<(FunctionOwnerIdV1, SourceValueRefV1), usize>,
    formal_nodes: &BTreeMap<BindingRefV1, usize>,
    core_requests: &[CoreRequestNode],
    array_fields: &BTreeMap<hakorune_mir_defs::CanonicalFieldRefV1, usize>,
) -> (
    Vec<Option<SourceValueSumV1>>,
    BTreeMap<(FunctionOwnerIdV1, SourceValueRefV1), Rc<SourceValueSumV1>>,
    BTreeMap<BindingRefV1, Rc<SourceValueSumV1>>,
    BTreeMap<OwnedExprSiteV1, Rc<ClosedOriginalCoreSourceV1>>,
) {
    let values = solve(nodes);
    let mut core_calls: BTreeMap<_, Option<Rc<ClosedOriginalCoreSourceV1>>> = BTreeMap::new();
    for request in core_requests.iter().cloned() {
        // A retained ArrayGet that has a field element node must use that
        // same node. An absent field remains the existing unknown path.
        let array_source_matches = request.field_array_access.as_ref()
            .filter(|source| source.is_dynamic_get())
            .and_then(|source| array_fields.get(&source.field()))
            .is_none_or(|element| nodes.get(request.node).is_some_and(|node| node.union.contains(element)));
        let open_premise = first_open_premise(&request, &values);
        if let Some(open) = &open_premise {
            explain_open_premise(nodes, formal_nodes, &values, &request, open);
        }
        let premises_closed = open_premise.is_none();
        let receiver_core = request.receiver_core.as_ref().and_then(|method|
            core_calls.get(&OwnedExprSiteV1::new(method.owner(), method.site().clone()))
                .and_then(Option::as_ref)
                .filter(|inner| std::sync::Arc::ptr_eq(inner.method(), method)
                    && inner.manifest().row().op == crate::mir::core_method_op::CoreMethodOp::MapKeys
                    && inner.field_map().is_some())).cloned();
        let receiver_closed = request.receiver_core.is_some() == receiver_core.is_some();
        let closed = values[request.node]
            .as_ref()
            .filter(|_| premises_closed && receiver_closed && array_source_matches)
            .and_then(|_| {
                Some(Rc::new(ClosedOriginalCoreSourceV1 {
                    result: values[request.node].clone()?,
                    method: std::sync::Arc::clone(&request.method),
                    request: request.request,
                    field_map: request.field_map.clone(),
                    receiver_core,
                    field_array_access: request.field_array_access.clone(),
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
    report_formal_sums(formal_nodes, &values);
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

/// Project the SAME solve's field union nodes. Re-solving unchanged original
/// writers retains the existing fact allocation; no second classifier/solve.
pub(super) fn project_field_sums_v1(
    graph: &OriginalSourceSumGraphV1, values: &[Option<SourceValueSumV1>],
    previous: Option<&ClosedFieldSumsV1>,
) -> ClosedFieldSumsV1 {
    graph.fields.iter().filter_map(|(field, (source, node))| {
        let sum = values[*node].as_ref()?;
        let retained = previous.and_then(|rows| rows.get(field))
            .filter(|(original, old)| Rc::ptr_eq(original, source) && old.as_ref() == sum)
            .map(|(_, sum)| Rc::clone(sum)).unwrap_or_else(|| Rc::new(sum.clone()));
        Some((*field, (Rc::clone(source), retained)))
    }).collect()
}
