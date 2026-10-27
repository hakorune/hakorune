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
    /// A closed nominal source may include null but must contain one class.
    /// This is a value-class fact, not a live or non-null read permission.
    pub(in crate::mir) fn nullable_object_class_v1(&self) -> Option<(&str, bool)> {
        Self::nullable_object_alternatives_v1(&self.0)
    }
    fn nullable_object_alternatives_v1(values: &BTreeSet<SourceValueAlternativeV1>) -> Option<(&str, bool)> {
        let class = values.iter().find_map(|value| match value {
            SourceValueAlternativeV1::Object(name) => Some(name.as_ref()),
            _ => None,
        })?;
        values.iter().all(|value| matches!(value, SourceValueAlternativeV1::Null)
            || matches!(value, SourceValueAlternativeV1::Object(actual) if actual.as_ref() == class))
            .then_some((class, values.contains(&SourceValueAlternativeV1::Null)))
    }
    pub(in crate::mir) fn object_class_v1(&self) -> Option<&str> {
        if self.0.len() != 1 { return None; }
        match self.0.first()? {
            SourceValueAlternativeV1::Object(class) => Some(class),
            _ => None,
        }
    }
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
    /// Close only the graph's original Object-result demands. This reuses the
    /// result owner's exact call/root loan before live input eligibility exists.
    pub(super) fn join_original_object_results_v1(
        &mut self, inventory: &BorrowedIncomingInventoryV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        facts: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::OrdinaryNewResultClassClaimsV1,
    ) -> Result<(), String> {
        use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::{OrdinaryNewResultClassV1 as Class, ResultValueOriginV1 as Origin};
        let mut classes = BTreeMap::new();
        for reference in self.instance_result_references_v1() {
            if classes.contains_key(&reference.call_site) { continue; }
            let mut rows = inventory.source_rows_v1().filter(|row| row.call == reference.call_site);
            let row = rows.next().ok_or_else(|| freeze("source-sum/caller-owner"))?;
            if rows.next().is_some() || row.callee != reference.callee_owner
                || row.source.original_instance_reference_v1().as_ref() != Some(reference) {
                return Err(freeze("source-sum/caller-owner"));
            }
            if !matches!(facts.get(row.source.target()), Some(Class::Object(_) | Class::NullableObject(_))) { continue; }
            let roots = if let Some(target) = row.source.instance() {
                facts.object_call_source_v1(batch, selected, target).map_err(freeze)?;
                facts.checked_original_source_roots_v1(target.target(), target.callee_owner()).map_err(freeze)?
            } else {
                // The pending Stored source already retains its exact owning
                // result loan. Lending its roots does not select that receiver.
                let Some(loan) = row.source.original_owning_object_input_v1() else { continue; };
                if loan.call() != &reference.call_site || loan.key() != &reference.target
                    || loan.callee_owner() != reference.callee_owner
                    || loan.receiver() != Some(&reference.receiver_site)
                    || !loan.matches_arguments_v1(&reference.argument_sites) {
                    return Err(freeze("source-sum/caller-owner"));
                }
                facts.recheck_owning_object_local_input_roots_v1(loan).map_err(freeze)?;
                facts.checked_original_source_roots_v1(loan.key(), loan.callee_owner()).map_err(freeze)?
            };
            let mut values = BTreeSet::new();
            for root in roots {
                values.insert(match root.origin() {
                    Origin::Fresh(name) => SourceValueAlternativeV1::Object(name.clone()),
                    Origin::Null => SourceValueAlternativeV1::Null,
                    _ => return Err(freeze("source-sum/caller-owner")),
                });
            }
            classes.insert(reference.call_site.clone(), (reference.clone(), values));
        }
        let classes: Vec<_> = classes.into_values().collect();
        if classes.is_empty() { return Ok(()); }
        if !self.join_instance_result_classes_v1(&classes) { return Err(freeze("source-sum/caller-owner")); }
        // These immutable original roots remain ground when later scalar
        // summaries re-solve the same graph; they grant no execution capability.
        for node in &mut self.graph.nodes {
            if let Some((_, values)) = classes.iter().find(|(reference, _)| node.instance_result.as_ref() == Some(reference)) {
                node.atoms.extend(values.iter().cloned());
                node.unconditional = true;
            }
        }
        Ok(())
    }
    pub(super) fn field_object_projection_v1(&self, source: &Rc<FieldSource>) -> FormalActualSeedV1 {
        let Some((original, sum)) = self.fields.get(&source.field()) else { return FormalActualSeedV1::Conflict; };
        if !Rc::ptr_eq(original, source) { return FormalActualSeedV1::Conflict; }
        if sum.nullable_object_class_v1().is_none()
            && !(sum.alternatives().len() == 1
                && sum.alternatives().contains(&SourceValueAlternativeV1::Null)) {
            return FormalActualSeedV1::Conflict;
        }
        let mut alternatives = Vec::new();
        for value in sum.alternatives() {
            match value {
                SourceValueAlternativeV1::Null => alternatives.push(FormalActualSeedV1::Null),
                SourceValueAlternativeV1::Object(name) => {
                    alternatives.push(FormalActualSeedV1::Class(name.clone()));
                }
                _ => return FormalActualSeedV1::Conflict,
            }
        }
        if alternatives.is_empty() { FormalActualSeedV1::Conflict }
        else { FormalActualSeedV1::All(alternatives.into_boxed_slice()) }
    }
    pub(super) fn join_instance_result_classes_v1(
        &mut self,
        classes: &[(
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::CallTargetReferenceV1,
            BTreeSet<SourceValueAlternativeV1>,
        )],
    ) -> bool {
        let mut nodes = self.graph.nodes.clone();
        for node in &mut nodes {
            let Some(reference) = &node.instance_result else { continue; };
            let mut matches = classes.iter().filter(|(candidate, _)| candidate == reference);
            let Some((_, class)) = matches.next() else { continue; };
            let primitive = class.len() == 1 && class.iter().all(|value| matches!(value,
                SourceValueAlternativeV1::SignedI64 | SourceValueAlternativeV1::Bool | SourceValueAlternativeV1::Text));
            let nominal = SourceValueSumV1::nullable_object_alternatives_v1(class).is_some();
            if matches.next().is_some() || !(primitive || nominal) { return false; }
            // One complete original result cohort, including its Null exits.
            node.atoms.extend(class.iter().cloned());
            node.unconditional = true;
        }
        let (all, points, formals, mut core_calls) = project_graph_v1(
            &nodes,
            &self.graph.points,
            &self.graph.formals,
            &self.graph.core_requests,
            &self.graph.array_fields,
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
    result: SourceValueSumV1,
    method: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    request: OriginalCoreOperandRequestV1,
    field_map: Option<Rc<crate::mir::normal_callable_semantic_package::core_method_source::FieldMapReceiverSourceV1>>,
    receiver_core: Option<Rc<ClosedOriginalCoreSourceV1>>,
    field_array_access: Option<Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>>,
    text: Box<[SourceValueSumV1]>,
    i64: Box<[SourceValueSumV1]>,
}
impl ClosedOriginalCoreSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn result_sum_v1(&self) -> &SourceValueSumV1 { &self.result }
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
            && self.result == other.result
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
    array_field_roots: &BTreeMap<
        hakorune_mir_defs::CanonicalFieldRefV1,
        Vec<OriginalFormalActualSeedV1>,
    >,
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
        array_fields: BTreeMap::new(),
        fields: BTreeMap::new(),
        core_requests: Vec::new(),
        map_receiver,
        expand_returns,
        returns: BTreeMap::new(),
        static_sources: BTreeMap::new(),
        return_sites: BTreeMap::new(),
        flow_owners: flows.keys().copied().collect(),
    };
    // Reserve every original all-writer field before expanding any actual.
    // Field reads and mutually dependent writers share these same nodes.
    for (source, _) in field_roots {
        let node = graph.nodes.len();
        if graph.fields.insert(source.field(), (Rc::clone(source), node)).is_some() {
            return Err(freeze("source-sum/caller-owner"));
        }
        graph.nodes.push(Node::default());
    }
    for field in array_field_roots.keys() {
        if graph.array_fields.insert(*field, graph.nodes.len()).is_some() {
            return Err(freeze("source-sum/array-field-duplicate"));
        }
        graph.nodes.push(Node::default());
    }
    for (field, seeds) in array_field_roots {
        let node = *graph.array_fields.get(field)
            .ok_or_else(|| freeze("source-sum/array-field-owner"))?;
        if seeds.is_empty() {
            graph.nodes[node].invalid = true;
            continue;
        }
        let union = seeds.iter().map(|seed| graph.seed(seed)).collect();
        graph.nodes[node].union = union;
    }
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
    for (source, seeds) in field_roots {
        let node = graph.fields.get(&source.field())
            .filter(|(original, _)| Rc::ptr_eq(original, source))
            .map(|(_, node)| *node).ok_or_else(|| freeze("source-sum/caller-owner"))?;
        let union = seeds.iter().map(|seed| graph.seed(seed)).collect();
        graph.nodes[node] = Node { union, invalid: seeds.is_empty(), ..Node::default() };
    }
    // A Core-method text operand can keep only its formal dependency, whose node
    // carries no source when the formal itself is unregistered or unfounded.
    // OR in the SAME completed flow read at that exact argument site.
    let core_requests = graph.core_requests.clone();
    for request in &core_requests {
        let owner = request.method.owner();
        let Some(flow) = flows.get(&owner).filter(|flow| flow.is_complete()) else { continue };
        if flow.owner() != owner { continue; }
        for (ordinal, node) in request.text.iter().enumerate() {
            let Some(argument) = request.method.arguments().get(ordinal) else { continue };
            let Some((binding, value)) = flow.read_at(argument.site()) else { continue };
            if binding.owner() != owner { continue; }
            let point = graph.point(owner, value);
            if point != *node && !graph.nodes[*node].union.contains(&point) {
                graph.nodes[*node].union.push(point);
            }
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
        fields: graph.fields,
        array_fields: graph.array_fields,
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
        &resolved_graph.array_fields,
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
