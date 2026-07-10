//! Typed shadow graph for module route-family convergence.
//!
//! The existing full refresh remains authoritative. This module only models
//! family dependencies, deterministic invalidation closure, and trace parity.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum RouteFamily {
    GenericMethod,
    GlobalCall,
    UserBoxMethod,
    MapLookupFusion,
    MapRepresentation,
    TypedObjectFieldValueTypes,
    TypedObjectCollectionElementTypes,
    OrderedMapResultOrigins,
}

impl RouteFamily {
    const ALL: [Self; 8] = [
        Self::GenericMethod,
        Self::GlobalCall,
        Self::UserBoxMethod,
        Self::MapLookupFusion,
        Self::MapRepresentation,
        Self::TypedObjectFieldValueTypes,
        Self::TypedObjectCollectionElementTypes,
        Self::OrderedMapResultOrigins,
    ];

    const fn tag(self) -> &'static str {
        match self {
            Self::GenericMethod => "generic_method",
            Self::GlobalCall => "global_call",
            Self::UserBoxMethod => "user_box_method",
            Self::MapLookupFusion => "map_lookup_fusion",
            Self::MapRepresentation => "map_representation",
            Self::TypedObjectFieldValueTypes => "typed_object_field_value_types",
            Self::TypedObjectCollectionElementTypes => "typed_object_collection_element_types",
            Self::OrderedMapResultOrigins => "ordered_map_result_origins",
        }
    }
}

const CURRENT_EDGES: &[(RouteFamily, RouteFamily)] = &[
    (RouteFamily::GenericMethod, RouteFamily::UserBoxMethod),
    (RouteFamily::GlobalCall, RouteFamily::GenericMethod),
    (RouteFamily::GlobalCall, RouteFamily::MapLookupFusion),
    (RouteFamily::GlobalCall, RouteFamily::MapRepresentation),
    (RouteFamily::UserBoxMethod, RouteFamily::GlobalCall),
    (
        RouteFamily::UserBoxMethod,
        RouteFamily::OrderedMapResultOrigins,
    ),
    (
        RouteFamily::MapLookupFusion,
        RouteFamily::TypedObjectFieldValueTypes,
    ),
    (
        RouteFamily::MapRepresentation,
        RouteFamily::TypedObjectFieldValueTypes,
    ),
    (
        RouteFamily::TypedObjectFieldValueTypes,
        RouteFamily::GenericMethod,
    ),
    (
        RouteFamily::TypedObjectCollectionElementTypes,
        RouteFamily::GenericMethod,
    ),
    (
        RouteFamily::OrderedMapResultOrigins,
        RouteFamily::TypedObjectCollectionElementTypes,
    ),
    (
        RouteFamily::OrderedMapResultOrigins,
        RouteFamily::GenericMethod,
    ),
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RouteDependencyGraphError {
    MissingNode,
    DuplicateEdge,
    SelfDependency,
    FullRefreshCoverageMismatch,
    FullRefreshOrderMismatch,
}

impl RouteDependencyGraphError {
    pub(crate) const fn stable_tag(&self) -> &'static str {
        match self {
            Self::MissingNode => "mir/convergence_dependency_missing",
            Self::DuplicateEdge | Self::SelfDependency => "mir/convergence_dirty_edge_missing",
            Self::FullRefreshCoverageMismatch | Self::FullRefreshOrderMismatch => {
                "mir/convergence_full_refresh_parity_mismatch"
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RouteDependencyGraph {
    nodes: BTreeSet<RouteFamily>,
    edges: BTreeSet<(RouteFamily, RouteFamily)>,
    dependents: BTreeMap<RouteFamily, BTreeSet<RouteFamily>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RouteInvalidationPlan {
    pub(crate) function_worklist: Vec<String>,
    pub(crate) affected_families: Vec<RouteFamily>,
}

impl RouteDependencyGraph {
    pub(crate) fn current() -> Result<Self, RouteDependencyGraphError> {
        Self::from_parts(RouteFamily::ALL, CURRENT_EDGES.iter().copied())
    }

    fn from_parts(
        nodes: impl IntoIterator<Item = RouteFamily>,
        edges: impl IntoIterator<Item = (RouteFamily, RouteFamily)>,
    ) -> Result<Self, RouteDependencyGraphError> {
        let nodes = nodes.into_iter().collect::<BTreeSet<_>>();
        let mut unique_edges = BTreeSet::new();
        let mut dependents = BTreeMap::<RouteFamily, BTreeSet<RouteFamily>>::new();
        for (producer, consumer) in edges {
            if producer == consumer {
                return Err(RouteDependencyGraphError::SelfDependency);
            }
            if !nodes.contains(&producer) || !nodes.contains(&consumer) {
                return Err(RouteDependencyGraphError::MissingNode);
            }
            if !unique_edges.insert((producer, consumer)) {
                return Err(RouteDependencyGraphError::DuplicateEdge);
            }
            dependents.entry(producer).or_default().insert(consumer);
        }
        Ok(Self {
            nodes,
            edges: unique_edges,
            dependents,
        })
    }

    pub(crate) fn affected_worklist(
        &self,
        dirty: impl IntoIterator<Item = RouteFamily>,
    ) -> Vec<RouteFamily> {
        let mut affected = dirty.into_iter().collect::<BTreeSet<_>>();
        let mut pending = affected.clone();
        while let Some(family) = pending.pop_first() {
            let Some(dependents) = self.dependents.get(&family) else {
                continue;
            };
            for dependent in dependents {
                if affected.insert(*dependent) {
                    pending.insert(*dependent);
                }
            }
        }
        affected.into_iter().collect()
    }

    pub(crate) fn invalidation_plan(
        &self,
        dirty_functions: impl IntoIterator<Item = String>,
        dirty_families: impl IntoIterator<Item = RouteFamily>,
    ) -> RouteInvalidationPlan {
        RouteInvalidationPlan {
            function_worklist: dirty_functions
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            affected_families: self.affected_worklist(dirty_families),
        }
    }

    fn verify_full_refresh_trace(
        &self,
        trace: &[RouteFamily],
    ) -> Result<(), RouteDependencyGraphError> {
        let visited = trace.iter().copied().collect::<BTreeSet<_>>();
        if visited != self.nodes {
            return Err(RouteDependencyGraphError::FullRefreshCoverageMismatch);
        }
        for (producer, consumer) in &self.edges {
            let ordered = trace.iter().enumerate().any(|(producer_index, family)| {
                family == producer
                    && trace[producer_index + 1..]
                        .iter()
                        .any(|later| later == consumer)
            });
            if !ordered {
                return Err(RouteDependencyGraphError::FullRefreshOrderMismatch);
            }
        }
        Ok(())
    }

    pub(crate) fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RouteRefreshShadowReport {
    pub(crate) dirty_function_count: usize,
    pub(crate) recomputed_function_count: usize,
    pub(crate) unchanged_function_recompute_count: usize,
    pub(crate) route_family_recompute_count: usize,
    pub(crate) dependency_edge_count: usize,
    pub(crate) worklist_determinism_hash: usize,
    pub(crate) full_refresh_parity_mismatch_count: usize,
}

pub(crate) struct RouteRefreshShadow {
    graph: RouteDependencyGraph,
    trace: Vec<RouteFamily>,
    full_refresh_function_count: usize,
}

impl RouteRefreshShadow {
    pub(crate) fn current(
        full_refresh_function_count: usize,
    ) -> Result<Self, RouteDependencyGraphError> {
        let graph = RouteDependencyGraph::current()?;
        let baseline = graph.invalidation_plan(
            std::iter::empty::<String>(),
            std::iter::empty::<RouteFamily>(),
        );
        debug_assert!(baseline.function_worklist.is_empty());
        debug_assert!(baseline.affected_families.is_empty());
        Ok(Self {
            graph,
            trace: Vec::new(),
            full_refresh_function_count,
        })
    }

    pub(crate) fn record(&mut self, family: RouteFamily) {
        self.trace.push(family);
    }

    pub(crate) fn finish(self) -> Result<RouteRefreshShadowReport, RouteDependencyGraphError> {
        self.graph.verify_full_refresh_trace(&self.trace)?;
        let worklist = self.graph.affected_worklist(self.trace.iter().copied());
        Ok(RouteRefreshShadowReport {
            dirty_function_count: 0,
            recomputed_function_count: self.full_refresh_function_count,
            unchanged_function_recompute_count: self.full_refresh_function_count,
            route_family_recompute_count: self.trace.len(),
            dependency_edge_count: self.graph.edge_count(),
            worklist_determinism_hash: deterministic_worklist_hash(&worklist),
            full_refresh_parity_mismatch_count: 0,
        })
    }
}

fn deterministic_worklist_hash(worklist: &[RouteFamily]) -> usize {
    let mut hash = 0xcbf29ce484222325_u64;
    for family in worklist {
        for byte in family.tag().bytes().chain([0]) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    hash as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_graph_closes_all_route_families_deterministically() {
        let graph = RouteDependencyGraph::current().expect("current route graph");
        let first = graph.affected_worklist([RouteFamily::GlobalCall]);
        let second = graph.affected_worklist([RouteFamily::GlobalCall]);

        assert_eq!(first, second);
        assert_eq!(first.len(), RouteFamily::ALL.len());
        assert_eq!(
            deterministic_worklist_hash(&first),
            deterministic_worklist_hash(&second)
        );
    }

    #[test]
    fn invalidation_plan_sorts_functions_without_using_names_as_policy() {
        let graph = RouteDependencyGraph::current().expect("current route graph");
        let first = graph.invalidation_plan(
            ["Helper.z/0", "Helper.a/0", "Helper.a/0"].map(str::to_owned),
            [RouteFamily::GlobalCall],
        );
        let renamed = graph.invalidation_plan(
            ["Other.y/0", "Other.b/0"].map(str::to_owned),
            [RouteFamily::GlobalCall],
        );

        assert_eq!(
            first.function_worklist,
            ["Helper.a/0".to_owned(), "Helper.z/0".to_owned()]
        );
        assert_eq!(first.affected_families, renamed.affected_families);
    }

    #[test]
    fn graph_rejects_missing_duplicate_and_self_edges() {
        assert_eq!(
            RouteDependencyGraph::from_parts(
                [RouteFamily::GenericMethod],
                [(RouteFamily::GenericMethod, RouteFamily::GlobalCall)]
            )
            .unwrap_err(),
            RouteDependencyGraphError::MissingNode
        );
        assert_eq!(
            RouteDependencyGraph::from_parts(
                [RouteFamily::GenericMethod, RouteFamily::GlobalCall],
                [
                    (RouteFamily::GenericMethod, RouteFamily::GlobalCall),
                    (RouteFamily::GenericMethod, RouteFamily::GlobalCall),
                ]
            )
            .unwrap_err(),
            RouteDependencyGraphError::DuplicateEdge
        );
        assert_eq!(
            RouteDependencyGraph::from_parts(
                [RouteFamily::GenericMethod],
                [(RouteFamily::GenericMethod, RouteFamily::GenericMethod)]
            )
            .unwrap_err(),
            RouteDependencyGraphError::SelfDependency
        );
    }

    #[test]
    fn full_refresh_trace_requires_coverage_and_dependency_order() {
        let graph = RouteDependencyGraph::from_parts(
            [RouteFamily::GenericMethod, RouteFamily::GlobalCall],
            [(RouteFamily::GlobalCall, RouteFamily::GenericMethod)],
        )
        .expect("two-family graph");

        assert_eq!(
            graph.verify_full_refresh_trace(&[RouteFamily::GlobalCall]),
            Err(RouteDependencyGraphError::FullRefreshCoverageMismatch)
        );
        assert_eq!(
            graph
                .verify_full_refresh_trace(&[RouteFamily::GenericMethod, RouteFamily::GlobalCall,]),
            Err(RouteDependencyGraphError::FullRefreshOrderMismatch)
        );
        assert!(graph
            .verify_full_refresh_trace(&[RouteFamily::GlobalCall, RouteFamily::GenericMethod,])
            .is_ok());
    }
}
