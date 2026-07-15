use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::CanonicalCallableKeyV1;

use super::{
    CallableSccDraftV1, CallableSccIdV1, CallableSccPartitionErrorV1,
    VerifiedCallableGraphInventoryV1,
};

pub(super) fn partition(
    inventory: &VerifiedCallableGraphInventoryV1,
) -> Result<Vec<CallableSccDraftV1>, CallableSccPartitionErrorV1> {
    let forward = adjacency(inventory, false)?;
    let reverse = adjacency(inventory, true)?;
    let finish_order = iterative_finish_order(inventory.nodes(), &forward)?;

    let mut assigned = BTreeSet::new();
    let mut drafts = Vec::new();
    for root in finish_order.into_iter().rev() {
        if !assigned.insert(root.clone()) {
            continue;
        }
        let mut members = Vec::new();
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            members.push(node.clone());
            let neighbors = reverse
                .get(&node)
                .ok_or_else(|| CallableSccPartitionErrorV1::MissingAdjacencyNode(node.clone()))?;
            for neighbor in neighbors.iter().rev() {
                if assigned.insert(neighbor.clone()) {
                    stack.push(neighbor.clone());
                }
            }
        }
        members.sort();
        drafts.push(CallableSccDraftV1 {
            id: CallableSccIdV1 {
                anchor: members[0].clone(),
            },
            members,
        });
    }
    Ok(drafts)
}

pub(super) fn adjacency(
    inventory: &VerifiedCallableGraphInventoryV1,
    reverse: bool,
) -> Result<
    BTreeMap<CanonicalCallableKeyV1, BTreeSet<CanonicalCallableKeyV1>>,
    CallableSccPartitionErrorV1,
> {
    let mut result = inventory
        .nodes()
        .iter()
        .cloned()
        .map(|node| (node, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for edge in inventory.unique_edges() {
        let (from, to) = if reverse {
            (edge.target(), edge.caller())
        } else {
            (edge.caller(), edge.target())
        };
        result
            .get_mut(from)
            .ok_or_else(|| CallableSccPartitionErrorV1::MissingAdjacencyNode(from.clone()))?
            .insert(to.clone());
    }
    Ok(result)
}

pub(super) fn is_strongly_connected(
    members: &BTreeSet<CanonicalCallableKeyV1>,
    forward: &BTreeMap<CanonicalCallableKeyV1, BTreeSet<CanonicalCallableKeyV1>>,
    reverse: &BTreeMap<CanonicalCallableKeyV1, BTreeSet<CanonicalCallableKeyV1>>,
) -> Result<bool, CallableSccPartitionErrorV1> {
    let Some(root) = members.first() else {
        return Ok(false);
    };
    Ok(reachable_within(root, members, forward)? == *members
        && reachable_within(root, members, reverse)? == *members)
}

fn iterative_finish_order(
    nodes: &[CanonicalCallableKeyV1],
    adjacency: &BTreeMap<CanonicalCallableKeyV1, BTreeSet<CanonicalCallableKeyV1>>,
) -> Result<Vec<CanonicalCallableKeyV1>, CallableSccPartitionErrorV1> {
    struct DfsFrame {
        node: CanonicalCallableKeyV1,
        neighbors: Vec<CanonicalCallableKeyV1>,
        next_neighbor: usize,
    }

    fn frame(
        node: CanonicalCallableKeyV1,
        adjacency: &BTreeMap<CanonicalCallableKeyV1, BTreeSet<CanonicalCallableKeyV1>>,
    ) -> Result<DfsFrame, CallableSccPartitionErrorV1> {
        let neighbors = adjacency
            .get(&node)
            .ok_or_else(|| CallableSccPartitionErrorV1::MissingAdjacencyNode(node.clone()))?
            .iter()
            .cloned()
            .collect();
        Ok(DfsFrame {
            node,
            neighbors,
            next_neighbor: 0,
        })
    }

    let mut visited = BTreeSet::new();
    let mut finish_order = Vec::with_capacity(nodes.len());
    for root in nodes {
        if !visited.insert(root.clone()) {
            continue;
        }
        let mut stack = vec![frame(root.clone(), adjacency)?];
        while let Some(active) = stack.last_mut() {
            if let Some(neighbor) = active.neighbors.get(active.next_neighbor).cloned() {
                active.next_neighbor += 1;
                if visited.insert(neighbor.clone()) {
                    stack.push(frame(neighbor, adjacency)?);
                }
                continue;
            }
            let finished = stack.pop().expect("active frame exists");
            finish_order.push(finished.node);
        }
    }
    Ok(finish_order)
}

fn reachable_within(
    root: &CanonicalCallableKeyV1,
    members: &BTreeSet<CanonicalCallableKeyV1>,
    adjacency: &BTreeMap<CanonicalCallableKeyV1, BTreeSet<CanonicalCallableKeyV1>>,
) -> Result<BTreeSet<CanonicalCallableKeyV1>, CallableSccPartitionErrorV1> {
    let mut visited = BTreeSet::from([root.clone()]);
    let mut stack = vec![root.clone()];
    while let Some(node) = stack.pop() {
        let neighbors = adjacency
            .get(&node)
            .ok_or_else(|| CallableSccPartitionErrorV1::MissingAdjacencyNode(node.clone()))?;
        for neighbor in neighbors.iter().rev() {
            if members.contains(neighbor) && visited.insert(neighbor.clone()) {
                stack.push(neighbor.clone());
            }
        }
    }
    Ok(visited)
}
