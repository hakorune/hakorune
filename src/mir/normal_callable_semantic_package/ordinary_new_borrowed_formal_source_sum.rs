//! Orthogonal union of SAME original seeds; no admission or source classifier.
use super::*;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1 as Rep;
use crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1;
#[path = "ordinary_new_borrowed_formal_source_sum_binary.rs"]
mod binary;
use binary::SourceBinaryTransferV1;
#[path = "ordinary_new_borrowed_formal_source_sum_graph.rs"]
mod graph;
use graph::{Graph, Node, OriginalSourceSumGraphV1, project_graph_v1, project_field_sums_v1};
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

type FieldSource = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::DeclaredFieldValueSourceV1;
pub(super) type FieldSourceSeedsV1 = (Rc<FieldSource>, Vec<OriginalFormalActualSeedV1>);
type ClosedFieldSumsV1 = BTreeMap<hakorune_mir_defs::CanonicalFieldRefV1, (Rc<FieldSource>, Rc<SourceValueSumV1>)>;

#[derive(Debug)]
pub(super) struct OriginalSourceSumsV1 {
    pub(super) new_calls: Rc<new_seeds::OriginalNewSeedsV1>,
    pub(super) values: BTreeMap<BindingRefV1, Rc<SourceValueSumV1>>,
    fields: ClosedFieldSumsV1,
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
        let (all, points, formals, mut core_calls) = project_graph_v1(
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
        self.fields = project_field_sums_v1(&self.graph, &all, Some(&self.fields));
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
    field_array_access: Option<Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>>,
    text: Box<[SourceValueSumV1]>,
    i64: Box<[SourceValueSumV1]>,
}
impl ClosedOriginalCoreSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn field_array_access_v1(&self)
        -> Option<&Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>> { self.field_array_access.as_ref() }
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
            && self.field_array_access == other.field_array_access
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
    field_array_access: Option<Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>>,
    text: Vec<usize>,
    i64: Vec<usize>,
    method: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    request: OriginalCoreOperandRequestV1,
}
pub(super) fn resolve_original_source_sums_v1(
    source: &OriginalFormalSeedProductV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    initializer_roots: &[OriginalFormalActualSeedV1],
    field_roots: &[FieldSourceSeedsV1],
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
    let mut fields = BTreeMap::new();
    for (source, seeds) in field_roots {
        let union = seeds.iter().map(|seed| graph.seed(seed)).collect();
        let node = graph.nodes.len();
        graph.nodes.push(Node { union, invalid: seeds.is_empty(), ..Node::default() });
        if fields.insert(source.field(), (Rc::clone(source), node)).is_some() {
            return Err(freeze("source-sum/caller-owner"));
        }
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
        fields,
        nodes: graph.nodes,
        points: graph.points,
        formals: graph.formals,
        core_requests: graph.core_requests,
    };
    let (all, points, values, core_calls) = project_graph_v1(
        &resolved_graph.nodes,
        &resolved_graph.points,
        &resolved_graph.formals,
        &resolved_graph.core_requests,
    );
    let fields = project_field_sums_v1(&resolved_graph, &all, None);
    Ok(OriginalSourceSumsV1 {
        fields,
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
