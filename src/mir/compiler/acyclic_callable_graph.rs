//! P0c-F-S0 topology-only exact callable graph.
//!
//! Targets are already resolved by the complete catalog. This product only
//! projects them to canonical keys, preserves call-site multiplicity, derives
//! unique edges, and proves acyclicity. It owns no ABI, evaluation order,
//! effect, MIR, publication, or SCC policy.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::{
    CanonicalCallableKeyV1, ResolvedCallableRefV1, SourceExprSiteV1,
};

use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedCallableGraphSiteV1 {
    caller: CanonicalCallableKeyV1,
    site: SourceExprSiteV1,
    target: CanonicalCallableKeyV1,
}

impl VerifiedCallableGraphSiteV1 {
    pub(crate) const fn caller(&self) -> &CanonicalCallableKeyV1 {
        &self.caller
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn target(&self) -> &CanonicalCallableKeyV1 {
        &self.target
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct VerifiedCallableGraphEdgeV1 {
    caller: CanonicalCallableKeyV1,
    target: CanonicalCallableKeyV1,
}

impl VerifiedCallableGraphEdgeV1 {
    pub(crate) const fn caller(&self) -> &CanonicalCallableKeyV1 {
        &self.caller
    }

    pub(crate) const fn target(&self) -> &CanonicalCallableKeyV1 {
        &self.target
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedAcyclicCallableGraphV1 {
    nodes: Box<[CanonicalCallableKeyV1]>,
    call_sites: Box<[VerifiedCallableGraphSiteV1]>,
    unique_edges: Box<[VerifiedCallableGraphEdgeV1]>,
    topological_order: Box<[CanonicalCallableKeyV1]>,
}

impl VerifiedAcyclicCallableGraphV1 {
    pub(crate) fn verify(
        module: &VerifiedResolvedCallableModuleV1,
    ) -> Result<Self, AcyclicCallableGraphErrorV1> {
        let catalog = module.source().catalog();
        let functions = module.functions_by_key();
        if catalog.len() != functions.len()
            || functions
                .keys()
                .any(|key| catalog.index().lookup(key).is_none())
        {
            return Err(AcyclicCallableGraphErrorV1::NodeSetMismatch {
                catalog: catalog.len(),
                functions: functions.len(),
            });
        }

        let nodes = functions.keys().cloned().collect::<Vec<_>>();
        let mut call_sites = Vec::new();
        for (caller, unit) in functions {
            let [root] = unit.forest().roots() else {
                return Err(AcyclicCallableGraphErrorV1::RootCardinality {
                    caller: caller.clone(),
                    actual: unit.forest().roots().len(),
                });
            };
            let function = unit.forest().owner(*root).ok_or_else(|| {
                AcyclicCallableGraphErrorV1::MissingCallerFunction {
                    caller: caller.clone(),
                }
            })?;
            for (site, target) in function.direct_call_targets() {
                let target_key = catalog
                    .index()
                    .header_for_callable(target.callable())
                    .map_err(
                        |_| AcyclicCallableGraphErrorV1::TargetIdentityOutsideCatalog {
                            caller: caller.clone(),
                            site: site.clone(),
                            target: target.callable(),
                        },
                    )?
                    .source_key()
                    .clone();
                call_sites.push(VerifiedCallableGraphSiteV1 {
                    caller: caller.clone(),
                    site: site.clone(),
                    target: target_key,
                });
            }
        }
        seal_inventory(nodes, call_sites)
    }

    pub(crate) fn nodes(&self) -> &[CanonicalCallableKeyV1] {
        &self.nodes
    }

    pub(crate) fn call_sites(&self) -> &[VerifiedCallableGraphSiteV1] {
        &self.call_sites
    }

    pub(crate) fn unique_edges(&self) -> &[VerifiedCallableGraphEdgeV1] {
        &self.unique_edges
    }

    pub(crate) fn topological_order(&self) -> &[CanonicalCallableKeyV1] {
        &self.topological_order
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AcyclicCallableGraphErrorV1 {
    NodeSetMismatch {
        catalog: usize,
        functions: usize,
    },
    RootCardinality {
        caller: CanonicalCallableKeyV1,
        actual: usize,
    },
    MissingCallerFunction {
        caller: CanonicalCallableKeyV1,
    },
    TargetIdentityOutsideCatalog {
        caller: CanonicalCallableKeyV1,
        site: SourceExprSiteV1,
        target: ResolvedCallableRefV1,
    },
    TargetOutsideNodeSet {
        caller: CanonicalCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalCallableKeyV1,
    },
    DuplicateGraphSite {
        caller: CanonicalCallableKeyV1,
        site: SourceExprSiteV1,
    },
    SelfEdge {
        caller: CanonicalCallableKeyV1,
        site: SourceExprSiteV1,
    },
    TopologicalIndexOverflow,
    TopologicalCardinalityMismatch,
    Cycle {
        residual_nodes: Box<[CanonicalCallableKeyV1]>,
        witness_sites: Box<[VerifiedCallableGraphSiteV1]>,
    },
}

fn seal_inventory(
    mut nodes: Vec<CanonicalCallableKeyV1>,
    mut call_sites: Vec<VerifiedCallableGraphSiteV1>,
) -> Result<VerifiedAcyclicCallableGraphV1, AcyclicCallableGraphErrorV1> {
    nodes.sort();
    nodes.dedup();
    call_sites.sort_by(|left, right| {
        (&left.caller, &left.site, &left.target).cmp(&(&right.caller, &right.site, &right.target))
    });

    let node_set = nodes.iter().cloned().collect::<BTreeSet<_>>();
    let mut site_set = BTreeSet::new();
    let mut edge_set = BTreeSet::new();
    for row in &call_sites {
        if !node_set.contains(&row.caller) || !node_set.contains(&row.target) {
            return Err(AcyclicCallableGraphErrorV1::TargetOutsideNodeSet {
                caller: row.caller.clone(),
                site: row.site.clone(),
                target: row.target.clone(),
            });
        }
        if !site_set.insert((row.caller.clone(), row.site.clone())) {
            return Err(AcyclicCallableGraphErrorV1::DuplicateGraphSite {
                caller: row.caller.clone(),
                site: row.site.clone(),
            });
        }
        if row.caller == row.target {
            return Err(AcyclicCallableGraphErrorV1::SelfEdge {
                caller: row.caller.clone(),
                site: row.site.clone(),
            });
        }
        edge_set.insert(VerifiedCallableGraphEdgeV1 {
            caller: row.caller.clone(),
            target: row.target.clone(),
        });
    }

    let mut indegree = nodes
        .iter()
        .cloned()
        .map(|key| (key, 0usize))
        .collect::<BTreeMap<_, _>>();
    let mut outgoing = nodes
        .iter()
        .cloned()
        .map(|key| (key, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for edge in &edge_set {
        let degree = indegree
            .get_mut(&edge.target)
            .ok_or(AcyclicCallableGraphErrorV1::TopologicalCardinalityMismatch)?;
        *degree = degree
            .checked_add(1)
            .ok_or(AcyclicCallableGraphErrorV1::TopologicalIndexOverflow)?;
        outgoing
            .get_mut(&edge.caller)
            .ok_or(AcyclicCallableGraphErrorV1::TopologicalCardinalityMismatch)?
            .insert(edge.target.clone());
    }

    // Deterministic Kahn order: the ready frontier is always canonical-key sorted.
    let mut ready = indegree
        .iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(key, _)| key.clone())
        .collect::<BTreeSet<_>>();
    let mut order = Vec::with_capacity(nodes.len());
    while let Some(node) = ready.iter().next().cloned() {
        ready.remove(&node);
        order.push(node.clone());
        for target in outgoing
            .get(&node)
            .ok_or(AcyclicCallableGraphErrorV1::TopologicalCardinalityMismatch)?
        {
            let degree = indegree
                .get_mut(target)
                .ok_or(AcyclicCallableGraphErrorV1::TopologicalCardinalityMismatch)?;
            *degree = degree
                .checked_sub(1)
                .ok_or(AcyclicCallableGraphErrorV1::TopologicalCardinalityMismatch)?;
            if *degree == 0 {
                ready.insert(target.clone());
            }
        }
    }

    if order.len() != nodes.len() {
        let residual = indegree
            .iter()
            .filter(|(_, degree)| **degree != 0)
            .map(|(key, _)| key.clone())
            .collect::<BTreeSet<_>>();
        let witness_sites = call_sites
            .iter()
            .filter(|row| residual.contains(&row.caller) && residual.contains(&row.target))
            .cloned()
            .collect::<Vec<_>>();
        return Err(AcyclicCallableGraphErrorV1::Cycle {
            residual_nodes: residual.into_iter().collect::<Vec<_>>().into_boxed_slice(),
            witness_sites: witness_sites.into_boxed_slice(),
        });
    }

    Ok(VerifiedAcyclicCallableGraphV1 {
        nodes: nodes.into_boxed_slice(),
        call_sites: call_sites.into_boxed_slice(),
        unique_edges: edge_set.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        topological_order: order.into_boxed_slice(),
    })
}

#[cfg(test)]
mod tests;
