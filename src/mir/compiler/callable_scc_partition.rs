//! P0c-MR-S0 disconnected deterministic callable SCC partition.
//!
//! This product consumes one verified graph inventory by value. It owns SCC
//! membership, stable component identity, recursion classification, and the
//! condensation DAG witness only. It owns no call target, ABI, effect, MIR,
//! publication, backend, or runtime authority.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::CanonicalCallableKeyV1;

use super::callable_graph_inventory::VerifiedCallableGraphInventoryV1;

mod algorithm;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CallableSccIdV1 {
    anchor: CanonicalCallableKeyV1,
}

impl CallableSccIdV1 {
    pub(crate) const fn anchor(&self) -> &CanonicalCallableKeyV1 {
        &self.anchor
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CallableSccRecursionKindV1 {
    NonRecursive,
    SelfRecursive,
    MutualRecursive { contains_self_edge: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedCallableSccV1 {
    id: CallableSccIdV1,
    members: Box<[CanonicalCallableKeyV1]>,
    recursion_kind: CallableSccRecursionKindV1,
}

impl VerifiedCallableSccV1 {
    pub(crate) const fn id(&self) -> &CallableSccIdV1 {
        &self.id
    }

    pub(crate) fn members(&self) -> &[CanonicalCallableKeyV1] {
        &self.members
    }

    pub(crate) const fn recursion_kind(&self) -> CallableSccRecursionKindV1 {
        self.recursion_kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct VerifiedCallableSccEdgeV1 {
    caller: CallableSccIdV1,
    target: CallableSccIdV1,
}

impl VerifiedCallableSccEdgeV1 {
    pub(crate) const fn caller(&self) -> &CallableSccIdV1 {
        &self.caller
    }

    pub(crate) const fn target(&self) -> &CallableSccIdV1 {
        &self.target
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedCallableSccPartitionV1 {
    inventory: VerifiedCallableGraphInventoryV1,
    components: Box<[VerifiedCallableSccV1]>,
    component_by_callable: BTreeMap<CanonicalCallableKeyV1, CallableSccIdV1>,
    condensation_edges: Box<[VerifiedCallableSccEdgeV1]>,
    condensation_order: Box<[CallableSccIdV1]>,
}

impl VerifiedCallableSccPartitionV1 {
    pub(crate) fn verify(
        inventory: VerifiedCallableGraphInventoryV1,
    ) -> Result<Self, CallableSccPartitionErrorV1> {
        let drafts = algorithm::partition(&inventory)?;
        seal_partition(inventory, drafts)
    }

    pub(crate) const fn inventory(&self) -> &VerifiedCallableGraphInventoryV1 {
        &self.inventory
    }

    pub(crate) fn components(&self) -> &[VerifiedCallableSccV1] {
        &self.components
    }

    pub(crate) fn component_for(
        &self,
        callable: &CanonicalCallableKeyV1,
    ) -> Option<&CallableSccIdV1> {
        self.component_by_callable.get(callable)
    }

    pub(crate) fn condensation_edges(&self) -> &[VerifiedCallableSccEdgeV1] {
        &self.condensation_edges
    }

    pub(crate) fn condensation_order(&self) -> &[CallableSccIdV1] {
        &self.condensation_order
    }

    pub(crate) fn recursive_component_count(&self) -> usize {
        self.components
            .iter()
            .filter(|component| {
                component.recursion_kind != CallableSccRecursionKindV1::NonRecursive
            })
            .count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableSccPartitionErrorV1 {
    MissingAdjacencyNode(CanonicalCallableKeyV1),
    EmptyComponent,
    DuplicateMember(CanonicalCallableKeyV1),
    UnknownMember(CanonicalCallableKeyV1),
    MissingMember(CanonicalCallableKeyV1),
    IdMismatch {
        actual: CallableSccIdV1,
        expected: CallableSccIdV1,
    },
    ComponentNotStronglyConnected(CallableSccIdV1),
    CondensationIndexOverflow,
    CondensationCardinalityMismatch,
    CondensationCycle {
        residual_components: Box<[CallableSccIdV1]>,
    },
}

#[derive(Debug)]
struct CallableSccDraftV1 {
    id: CallableSccIdV1,
    members: Vec<CanonicalCallableKeyV1>,
}

fn seal_partition(
    inventory: VerifiedCallableGraphInventoryV1,
    mut drafts: Vec<CallableSccDraftV1>,
) -> Result<VerifiedCallableSccPartitionV1, CallableSccPartitionErrorV1> {
    let node_set = inventory.nodes().iter().cloned().collect::<BTreeSet<_>>();
    let adjacency = algorithm::adjacency(&inventory, false)?;
    let reverse_adjacency = algorithm::adjacency(&inventory, true)?;
    let self_edges = inventory
        .unique_edges()
        .iter()
        .filter(|edge| edge.caller() == edge.target())
        .map(|edge| edge.caller().clone())
        .collect::<BTreeSet<_>>();

    let mut component_by_callable = BTreeMap::new();
    let mut components = Vec::with_capacity(drafts.len());
    for draft in &mut drafts {
        if draft.members.is_empty() {
            return Err(CallableSccPartitionErrorV1::EmptyComponent);
        }
        draft.members.sort();
        let expected = CallableSccIdV1 {
            anchor: draft.members[0].clone(),
        };
        if draft.id != expected {
            return Err(CallableSccPartitionErrorV1::IdMismatch {
                actual: draft.id.clone(),
                expected,
            });
        }
        let member_set = draft.members.iter().cloned().collect::<BTreeSet<_>>();
        if member_set.len() != draft.members.len() {
            let mut seen = BTreeSet::new();
            let duplicate = draft
                .members
                .iter()
                .find(|member| !seen.insert((*member).clone()))
                .expect("member cardinality proved a duplicate")
                .clone();
            return Err(CallableSccPartitionErrorV1::DuplicateMember(duplicate));
        }
        for member in &draft.members {
            if !node_set.contains(member) {
                return Err(CallableSccPartitionErrorV1::UnknownMember(member.clone()));
            }
            if component_by_callable
                .insert(member.clone(), draft.id.clone())
                .is_some()
            {
                return Err(CallableSccPartitionErrorV1::DuplicateMember(member.clone()));
            }
        }
        if !algorithm::is_strongly_connected(&member_set, &adjacency, &reverse_adjacency)? {
            return Err(CallableSccPartitionErrorV1::ComponentNotStronglyConnected(
                draft.id.clone(),
            ));
        }
        let recursion_kind = if draft.members.len() == 1 {
            if self_edges.contains(&draft.members[0]) {
                CallableSccRecursionKindV1::SelfRecursive
            } else {
                CallableSccRecursionKindV1::NonRecursive
            }
        } else {
            CallableSccRecursionKindV1::MutualRecursive {
                contains_self_edge: draft
                    .members
                    .iter()
                    .any(|member| self_edges.contains(member)),
            }
        };
        components.push(VerifiedCallableSccV1 {
            id: draft.id.clone(),
            members: draft.members.clone().into_boxed_slice(),
            recursion_kind,
        });
    }

    for node in inventory.nodes() {
        if !component_by_callable.contains_key(node) {
            return Err(CallableSccPartitionErrorV1::MissingMember(node.clone()));
        }
    }
    components.sort_by(|left, right| left.id.cmp(&right.id));

    let mut condensation_edges = BTreeSet::new();
    for edge in inventory.unique_edges() {
        let caller = component_by_callable
            .get(edge.caller())
            .ok_or_else(|| CallableSccPartitionErrorV1::MissingMember(edge.caller().clone()))?;
        let target = component_by_callable
            .get(edge.target())
            .ok_or_else(|| CallableSccPartitionErrorV1::MissingMember(edge.target().clone()))?;
        if caller != target {
            condensation_edges.insert(VerifiedCallableSccEdgeV1 {
                caller: caller.clone(),
                target: target.clone(),
            });
        }
    }
    let condensation_order = seal_condensation_order(&components, &condensation_edges)?;

    Ok(VerifiedCallableSccPartitionV1 {
        inventory,
        components: components.into_boxed_slice(),
        component_by_callable,
        condensation_edges: condensation_edges
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        condensation_order,
    })
}

fn seal_condensation_order(
    components: &[VerifiedCallableSccV1],
    edges: &BTreeSet<VerifiedCallableSccEdgeV1>,
) -> Result<Box<[CallableSccIdV1]>, CallableSccPartitionErrorV1> {
    let mut indegree = components
        .iter()
        .map(|component| (component.id.clone(), 0usize))
        .collect::<BTreeMap<_, _>>();
    let mut outgoing = components
        .iter()
        .map(|component| (component.id.clone(), BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for edge in edges {
        let degree = indegree
            .get_mut(&edge.target)
            .ok_or(CallableSccPartitionErrorV1::CondensationCardinalityMismatch)?;
        *degree = degree
            .checked_add(1)
            .ok_or(CallableSccPartitionErrorV1::CondensationIndexOverflow)?;
        outgoing
            .get_mut(&edge.caller)
            .ok_or(CallableSccPartitionErrorV1::CondensationCardinalityMismatch)?
            .insert(edge.target.clone());
    }

    let mut ready = indegree
        .iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(id, _)| id.clone())
        .collect::<BTreeSet<_>>();
    let mut order = Vec::with_capacity(components.len());
    while let Some(id) = ready.pop_first() {
        order.push(id.clone());
        for target in outgoing
            .get(&id)
            .ok_or(CallableSccPartitionErrorV1::CondensationCardinalityMismatch)?
        {
            let degree = indegree
                .get_mut(target)
                .ok_or(CallableSccPartitionErrorV1::CondensationCardinalityMismatch)?;
            *degree = degree
                .checked_sub(1)
                .ok_or(CallableSccPartitionErrorV1::CondensationCardinalityMismatch)?;
            if *degree == 0 {
                ready.insert(target.clone());
            }
        }
    }
    if order.len() != components.len() {
        let residual_components = indegree
            .into_iter()
            .filter(|(_, degree)| *degree != 0)
            .map(|(id, _)| id)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        return Err(CallableSccPartitionErrorV1::CondensationCycle {
            residual_components,
        });
    }
    Ok(order.into_boxed_slice())
}

#[cfg(test)]
mod tests;
