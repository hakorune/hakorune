//! P0c-F-S0 topology-only acyclic callable graph proof.
//!
//! P0c-MR-G0 owns resolved-target projection and site/edge inventory in the
//! shared inventory product. This product consumes that inventory by value and
//! adds only self-edge rejection plus a deterministic Kahn DAG witness.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, SourceExprSiteV1};

use super::callable_graph_inventory::{
    CallableGraphInventoryErrorV1, VerifiedCallableGraphEdgeV1, VerifiedCallableGraphInventoryV1,
    VerifiedCallableGraphSiteV1,
};
use super::callable_scc_partition::VerifiedCallableSccPartitionV1;
use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedAcyclicCallableGraphV1 {
    inventory: VerifiedCallableGraphInventoryV1,
    topological_order: Box<[CanonicalCallableKeyV1]>,
}

impl VerifiedAcyclicCallableGraphV1 {
    pub(crate) fn verify(
        module: &VerifiedResolvedCallableModuleV1,
    ) -> Result<Self, AcyclicCallableGraphErrorV1> {
        let inventory = VerifiedCallableGraphInventoryV1::verify(module)
            .map_err(AcyclicCallableGraphErrorV1::Inventory)?;
        verify_inventory(inventory)
    }

    pub(in crate::mir) fn from_nonrecursive_partition(
        partition: VerifiedCallableSccPartitionV1,
    ) -> Result<Self, AcyclicCallableGraphErrorV1> {
        if partition.recursive_component_count() != 0 {
            return Err(AcyclicCallableGraphErrorV1::PartitionContainsRecursion);
        }
        verify_inventory(partition.into_inventory())
    }

    pub(crate) fn nodes(&self) -> &[CanonicalCallableKeyV1] {
        self.inventory.nodes()
    }

    pub(crate) fn call_sites(&self) -> &[VerifiedCallableGraphSiteV1] {
        self.inventory.call_sites()
    }

    pub(crate) fn unique_edges(&self) -> &[VerifiedCallableGraphEdgeV1] {
        self.inventory.unique_edges()
    }

    pub(crate) fn topological_order(&self) -> &[CanonicalCallableKeyV1] {
        &self.topological_order
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AcyclicCallableGraphErrorV1 {
    Inventory(CallableGraphInventoryErrorV1),
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
    PartitionContainsRecursion,
}

fn verify_inventory(
    inventory: VerifiedCallableGraphInventoryV1,
) -> Result<VerifiedAcyclicCallableGraphV1, AcyclicCallableGraphErrorV1> {
    for row in inventory.call_sites() {
        if row.caller() == row.target() {
            return Err(AcyclicCallableGraphErrorV1::SelfEdge {
                caller: row.caller().clone(),
                site: row.site().clone(),
            });
        }
    }

    let mut indegree = inventory
        .nodes()
        .iter()
        .cloned()
        .map(|key| (key, 0usize))
        .collect::<BTreeMap<_, _>>();
    let mut outgoing = inventory
        .nodes()
        .iter()
        .cloned()
        .map(|key| (key, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for edge in inventory.unique_edges() {
        let degree = indegree
            .get_mut(edge.target())
            .ok_or(AcyclicCallableGraphErrorV1::TopologicalCardinalityMismatch)?;
        *degree = degree
            .checked_add(1)
            .ok_or(AcyclicCallableGraphErrorV1::TopologicalIndexOverflow)?;
        outgoing
            .get_mut(edge.caller())
            .ok_or(AcyclicCallableGraphErrorV1::TopologicalCardinalityMismatch)?
            .insert(edge.target().clone());
    }

    // Deterministic Kahn order: the ready frontier is canonical-key sorted.
    let mut ready = indegree
        .iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(key, _)| key.clone())
        .collect::<BTreeSet<_>>();
    let mut order = Vec::with_capacity(inventory.nodes().len());
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

    if order.len() != inventory.nodes().len() {
        let residual = indegree
            .iter()
            .filter(|(_, degree)| **degree != 0)
            .map(|(key, _)| key.clone())
            .collect::<BTreeSet<_>>();
        let witness_sites = inventory
            .call_sites()
            .iter()
            .filter(|row| residual.contains(row.caller()) && residual.contains(row.target()))
            .cloned()
            .collect::<Vec<_>>();
        return Err(AcyclicCallableGraphErrorV1::Cycle {
            residual_nodes: residual.into_iter().collect::<Vec<_>>().into_boxed_slice(),
            witness_sites: witness_sites.into_boxed_slice(),
        });
    }

    Ok(VerifiedAcyclicCallableGraphV1 {
        inventory,
        topological_order: order.into_boxed_slice(),
    })
}

#[cfg(test)]
mod tests;
