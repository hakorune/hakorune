//! Orthogonal union of SAME original seeds; no admission or source classifier.
use super::*;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1 as Rep;
use crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1;
#[path = "ordinary_new_borrowed_formal_source_sum_binary.rs"]
mod binary;
use binary::SourceBinaryTransferV1;

pub(super) enum SourcePointExpansionV1 {
    Entry(BindingRefV1),
    Join(Vec<SourceValueRefV1>),
    Seed(OriginalFormalActualSeedV1),
    Unknown,
}
type PointExpander<'a> = dyn FnMut(FunctionOwnerIdV1, SourceValueRefV1) -> Result<SourcePointExpansionV1, String> + 'a;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir)
enum SourceValueAlternativeV1 {
    Text,
    SignedI64,
    Null,
    Other,
    Object(Box<str>),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir)
struct SourceValueSumV1(BTreeSet<SourceValueAlternativeV1>);
impl SourceValueSumV1 {
    pub(in crate::mir) fn alternatives(
        &self,
    ) -> &BTreeSet<SourceValueAlternativeV1> {
        &self.0
    }
    fn signed_only(&self) -> bool {
        self.0.len() == 1 && self.0.contains(&SourceValueAlternativeV1::SignedI64)
    }
    pub(in crate::mir) fn text_only(&self) -> bool {
        self.0.len() == 1 && self.0.contains(&SourceValueAlternativeV1::Text)
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
}
/// Closed source premises, not an evaluation, ABI or cleanup capability.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package)
struct ClosedOriginalCoreSourceV1 {
    method: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    request: OriginalCoreOperandRequestV1,
    text: Box<[SourceValueSumV1]>,
    i64: Box<[SourceValueSumV1]>,
}
impl ClosedOriginalCoreSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn method(&self)
        -> &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1> { &self.method }
    pub(in crate::mir::normal_callable_semantic_package) fn manifest(&self)
        -> crate::mir::core_method_result_kind::CoreMethodManifestRowRefV2 { self.request.manifest() }
    fn matches(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.method, &other.method)
            && std::ptr::eq(self.manifest().row(), other.manifest().row())
            && self.manifest().arity() == other.manifest().arity()
            && self.text == other.text && self.i64 == other.i64
    }
}
struct CoreRequestNode {
    node: usize,
    text: Vec<usize>,
    i64: Vec<usize>,
    method: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    request: OriginalCoreOperandRequestV1,
}
#[derive(Default)]
struct Node {
    binary: Option<SourceBinaryTransferV1>,
    atoms: BTreeSet<SourceValueAlternativeV1>,
    union: Vec<usize>,
    required_i64: Vec<usize>,
    required_text: Vec<usize>,
    constraints: Vec<Box<str>>,
    invalid: bool,
    unconditional: bool,
}
impl Node {
    fn dependencies(&self) -> impl Iterator<Item = usize> + '_ {
        self.union.iter().chain(&self.required_i64).chain(&self.required_text)
            .copied().chain(self.binary.iter().flat_map(|binary| binary.operands))
    }
}
struct Graph<'a> {
    points: BTreeMap<(FunctionOwnerIdV1, SourceValueRefV1), usize>,
    expand: &'a mut PointExpander<'a>,
    expansion_error: Option<String>,
    nodes: Vec<Node>,
    formals: BTreeMap<BindingRefV1, usize>,
    constructors: &'a crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    core_requests: Vec<CoreRequestNode>,
}
impl Graph<'_> {
    fn point(&mut self, owner: FunctionOwnerIdV1, value: SourceValueRefV1) -> usize {
        if let Some(node) = self.points.get(&(owner, value)) { return *node; }
        let id = self.nodes.len();
        self.nodes.push(Node::default());
        self.points.insert((owner, value), id);
        let mut node = Node::default();
        match (self.expand)(owner, value) {
            Ok(SourcePointExpansionV1::Entry(binding)) => node.union.push(self.formal(binding)),
            Ok(SourcePointExpansionV1::Join(incoming)) => {
                node.invalid = incoming.is_empty();
                for incoming in incoming { node.union.push(self.point(owner, incoming)); }
            }
            Ok(SourcePointExpansionV1::Seed(seed)) => node.union.push(self.seed(&seed)),
            Ok(SourcePointExpansionV1::Unknown) => node.invalid = true,
            Err(error) => { self.expansion_error.get_or_insert(error); node.invalid = true; }
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
                if let Some(value) = value { row.union.push(self.point(*owner, *value)); }
                else { row.invalid = true; }
            }
            OriginalFormalActualSeedV1::Binary { owner, source, lhs, rhs } => {
                row.binary = Some(SourceBinaryTransferV1 { owner: *owner, source: source.clone(), operands: [self.seed(lhs), self.seed(rhs)] });
            }
            OriginalFormalActualSeedV1::Text => {
                row.atoms.insert(Value::Text);
            }
            OriginalFormalActualSeedV1::Integer => {
                row.atoms.insert(Value::SignedI64);
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
                match source.normal_representation() {
                    Some(Rep::ExactString) => {
                        row.atoms.insert(Value::Text);
                    }
                    Some(Rep::ExactI64) => {
                        row.atoms.insert(Value::SignedI64);
                    }
                    Some(Rep::ExactBool) => {
                        row.atoms.insert(Value::Other);
                    }
                    Some(Rep::ExactNominalBox { box_name }) => {
                        row.atoms.insert(Value::Object(box_name.into()));
                    }
                    None => row.invalid = true,
                }
                let i64: Vec<_> = source.required_i64().iter().map(|seed| self.seed(seed)).collect();
                let text: Vec<_> = source.required_text().iter().map(|seed| self.seed(seed)).collect();
                if let Some(request) = source.core_operand_request() {
                    self.core_requests.push(CoreRequestNode { node: id,
                        method: std::sync::Arc::clone(source.method()), request,
                        text: text.clone(), i64: i64.clone() });
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
        row.unconditional = row.required_i64.is_empty() && row.required_text.is_empty() && !row.atoms.is_empty();
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

pub(super) fn resolve_original_source_sums_v1(
    source: &OriginalFormalSeedProductV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    initializer_roots: &[OriginalFormalActualSeedV1],
    expand: &mut PointExpander<'_>,
) -> Result<OriginalSourceSumsV1, String> {
    let mut graph = Graph {
        points: BTreeMap::new(), expand, expansion_error: None,
        nodes: Vec::new(),
        formals: BTreeMap::new(),
        constructors,
        core_requests: Vec::new(),
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
    }
    source
        .new_calls
        .visit_raw_source_seeds_v1(constructors, |formal, seed| graph.actual(formal, seed));
    for root in initializer_roots { graph.seed(root); }
    if let Some(error) = graph.expansion_error.take() { return Err(error); }
    for (binding, node) in &graph.formals {
        graph.nodes[*node].invalid |=
            !registered.contains(binding) || graph.nodes[*node].union.is_empty();
    }
    let values = solve(&graph.nodes);
    let mut core_calls: BTreeMap<_, Option<ClosedOriginalCoreSourceV1>> = BTreeMap::new();
    for request in graph.core_requests {
        let premises_closed = request.text.iter().all(|dependency| values[*dependency].as_ref().is_some_and(SourceValueSumV1::text_only))
            && request.i64.iter().all(|dependency| values[*dependency].as_ref().is_some_and(SourceValueSumV1::signed_only));
        let closed = values[request.node].as_ref().filter(|_| premises_closed).and_then(|_| {
            Some(ClosedOriginalCoreSourceV1 {
                method: std::sync::Arc::clone(&request.method), request: request.request,
                text: request.text.iter().map(|dependency| values[*dependency].clone()).collect::<Option<Vec<_>>>()?.into_boxed_slice(),
                i64: request.i64.iter().map(|dependency| values[*dependency].clone()).collect::<Option<Vec<_>>>()?.into_boxed_slice(),
            })
        });
        let site = OwnedExprSiteV1::new(request.method.owner(), request.method.site().clone());
        match core_calls.entry(site) {
            std::collections::btree_map::Entry::Vacant(entry) => { entry.insert(closed); }
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                if !matches!((entry.get(), &closed), (Some(a), Some(b)) if a.matches(b)) { entry.insert(None); }
            }
        }
    }
    Ok(OriginalSourceSumsV1 {
        new_calls: Rc::clone(&source.new_calls),
        core_calls: core_calls.into_iter().filter_map(|(site, value)| value.map(|value| (site, Rc::new(value)))).collect(),
        values: graph
            .formals
            .into_iter()
            .filter_map(|(binding, node)| values[node].clone().map(|value| (binding, Rc::new(value))))
            .collect(),
    })
}

#[path = "ordinary_new_borrowed_formal_source_sum_solver.rs"]
mod solver;
use solver::solve;

#[cfg(test)]
mod tests {
    use super::*;
    fn atom(value: SourceValueAlternativeV1) -> Node {
        Node {
            atoms: BTreeSet::from([value]),
            unconditional: true,
            ..Default::default()
        }
    }
    #[test]
    fn source_sum_components_require_independent_ground_and_all_i64_premises() {
        use SourceValueAlternativeV1::{Other, SignedI64, Text};
        // P=Text|Q, Q=R, R=Q: upstream Text cannot rescue a seedless SCC.
        let mut p = atom(Text);
        p.union.push(1);
        let mut q = Node::default();
        q.union.push(2);
        let mut r = Node::default();
        r.union.push(1);
        let mut nodes = vec![p, q, r];
        assert!(solve(&nodes).iter().all(Option::is_none));
        nodes[1].atoms.insert(Text);
        nodes[1].unconditional = true;
        assert!(solve(&nodes)
            .iter()
            .all(|value| value.as_ref().unwrap().0 == BTreeSet::from([Text])));
        // P=f(P,7): a spare closed argument must not prove its cyclic premise.
        let p = Node {
            union: vec![1],
            ..Default::default()
        };
        let f = Node {
            atoms: BTreeSet::from([SignedI64]),
            required_i64: vec![0, 2],
            ..Default::default()
        };
        let mut nodes = vec![p, f, atom(SignedI64)];
        assert!(solve(&nodes)[0].is_none());
        // f(7) has independently closed premises and a conditional result.
        nodes[1].required_i64 = vec![2];
        assert!(solve(&nodes)[0].as_ref().unwrap().signed_only());
        // An independently grounded recursive I64 cohort is different.
        nodes[0].union.push(2);
        nodes[1].required_i64 = vec![0, 2];
        assert!(solve(&nodes)[0].as_ref().unwrap().signed_only());
        nodes[2] = atom(Other);
        assert!(
            solve(&nodes)[0].is_none(),
            "all I64 premises must close exactly"
        );
        nodes[2] = Node {
            invalid: true,
            ..Default::default()
        };
        assert!(
            solve(&nodes)[0].is_none(),
            "unknown siblings survive condensation"
        );
        // P=P.substring(0,1): closed bounds cannot ground receiver Text.
        let mut nodes = vec![Node { union: vec![1], ..Default::default() }, Node {
            atoms: BTreeSet::from([Text]), required_text: vec![0], required_i64: vec![2],
            ..Default::default()
        }, atom(SignedI64)];
        assert!(solve(&nodes)[0].is_none());
        nodes[0].union.push(3);
        nodes.push(atom(Text));
        assert!(solve(&nodes)[0].as_ref().unwrap().text_only(), "independent Text grounds its recursive cohort");
        nodes[3] = atom(Other);
        assert!(solve(&nodes)[0].is_none(), "mixed receiver is not Text-only");
        nodes[3] = atom(Text);
        nodes[2] = atom(Text);
        assert!(solve(&nodes)[0].is_none(), "Text bounds are not signed I64");
        // A binary contributes no speculative atom. External operands may
        // ground it; a cyclic operand plus a spare literal may not.
        let binary = |lhs, rhs| {
            use crate::mir::resolved_semantics::{FunctionOwnerIssuerV1, SourceNodeSiteV1, SourcePathSegmentV1 as Path, ResolvedBinaryExpressionSourceV1, ResolvedBinaryOperatorV1};
            let site = |roles| SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(roles));
            Node { binary: Some(SourceBinaryTransferV1 {
                owner: FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap(),
                source: ResolvedBinaryExpressionSourceV1::from_parts_for_test(
                    site(vec![Path::Body(0)]), ResolvedBinaryOperatorV1::Add,
                    site(vec![Path::Body(0), Path::Lhs]), site(vec![Path::Body(0), Path::Rhs])),
                operands: [lhs, rhs],
            }), ..Default::default() }
        };
        let nodes = vec![binary(1, 2), atom(Text), atom(Text)];
        assert!(solve(&nodes)[0].as_ref().unwrap().text_only());
        let mut nodes = vec![Node { union: vec![1], ..Default::default() }, binary(0, 2), atom(Text)];
        assert!(solve(&nodes)[0].is_none(), "cyclic concatenation has no initial source value");
        nodes[0].union.push(3);
        nodes.push(atom(Text));
        assert!(solve(&nodes)[0].as_ref().unwrap().text_only());
        nodes[2].atoms.insert(Other);
        assert!(solve(&nodes)[0].is_none(), "unsupported reachable pair cannot be dropped");
        nodes[2] = atom(SignedI64);
        assert!(solve(&nodes)[0].is_none(), "String*Dynamic is not selected");
        nodes[3] = atom(SignedI64);
        assert!(solve(&nodes)[0].as_ref().unwrap().signed_only());

    }
}
