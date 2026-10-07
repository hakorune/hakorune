//! Exact cleanup graph coverage independent of any F=N node-count formula.
//! Source-order path validation separately certifies every release obligation.
use super::super::root_home::RootHomeExitEntry;
use super::*;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_original(
    function: &MirFunction,
    bindings: &Bindings,
    entry: &RootHomeExitEntry,
) -> Result<(), String> {
    let nodes = collect_nodes(bindings)?;
    let (starts, plain_entry, projected_result) = match entry {
        RootHomeExitEntry::Plain { .. } => {
            let (id, terminal) = bindings
                .last()
                .ok_or_else(|| fault("ordered-structure/entry"))?;
            if !matches!(
                terminal,
                MirInstruction::Jump {
                    edge_args: None,
                    ..
                }
            ) {
                return Err(fault("ordered-structure/entry"));
            }
            if *id == function.entry_block
                && bindings
                    .iter()
                    .any(|(_, i)| matches!(i, MirInstruction::Invoke { .. }))
            {
                return Err(fault("ordered-structure/entry"));
            }
            (vec![*id], Some(*id), None)
        }
        RootHomeExitEntry::Call {
            invoke, projection, ..
        }
        | RootHomeExitEntry::MapGet {
            invoke, projection, ..
        } => {
            let (clean, pending, _, _) = call::ingress(function, bindings, invoke, projection)?;
            (vec![clean, pending], None, Some(projection))
        }
    };
    topology(function, &nodes, starts, plain_entry)?;
    for (id, terminal) in &nodes {
        let block = function
            .blocks
            .get(id)
            .ok_or_else(|| fault("ordered-structure/missing-node"))?;
        if block.terminator.as_ref() != Some(*terminal)
            || block
                .instructions
                .iter()
                .any(|i| matches!(i, MirInstruction::Phi { .. }))
        {
            return Err(fault("ordered-structure/original-node"));
        }
        if Some(*id) != plain_entry {
            let expected = projected_result
                .filter(|(base, _)| *base == *id)
                .map(|(_, i)| std::slice::from_ref(i))
                .unwrap_or(&[]);
            if block.instructions != expected {
                return Err(fault("ordered-structure/instructions"));
            }
        }
    }
    Ok(())
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_projected(
    function: &MirFunction,
    original: &Bindings,
    entry: &RootHomeExitEntry,
    projection: &super::super::physical_boundary::FinishedBindings,
) -> Result<(), String> {
    let bindings = projection.bindings(original)?;
    let nodes = collect_nodes(&bindings)?;
    let (starts, plain_entry) = match entry {
        RootHomeExitEntry::Plain { .. } => {
            let original_entry = original
                .last()
                .ok_or_else(|| fault("ordered-structure/entry"))?
                .0;
            let entry = projection
                .destination(original_entry)
                .ok_or_else(|| fault("ordered-structure/missing-projection"))?;
            (vec![entry], Some(entry))
        }
        RootHomeExitEntry::Call {
            invoke,
            projection: result,
            ..
        }
        | RootHomeExitEntry::MapGet {
            invoke,
            projection: result,
            ..
        } => {
            let invoke = projection
                .binding(invoke.0, &invoke.1)?
                .ok_or_else(|| fault("ordered-structure/missing-invoke"))?;
            let result = projection
                .binding(result.0, &result.1)?
                .ok_or_else(|| fault("ordered-structure/missing-result"))?;
            let (clean, pending, _, _) = call::ingress(function, &bindings, &invoke, &result)?;
            (vec![clean, pending], None)
        }
    };
    // Existing PhysicalBoundary::validate_complete independently checks merged
    // instruction sequences before the owner can enter FinishingChecked.
    topology(function, &nodes, starts, plain_entry)
}
fn collect_nodes(bindings: &Bindings) -> Result<BTreeMap<BasicBlockId, &MirInstruction>, String> {
    let mut nodes = BTreeMap::new();
    for (id, terminal) in bindings {
        if nodes.insert(*id, terminal).is_some() {
            return Err(fault("ordered-structure/duplicate-node"));
        }
        edges(terminal)?;
    }
    require_acyclic(&nodes)?;
    Ok(nodes)
}
fn topology(
    function: &MirFunction,
    nodes: &BTreeMap<BasicBlockId, &MirInstruction>,
    starts: Vec<BasicBlockId>,
    plain_entry: Option<BasicBlockId>,
) -> Result<(), String> {
    let mut reached = BTreeSet::new();
    let mut work = starts;
    while let Some(id) = work.pop() {
        if reached.insert(id) {
            let terminal = nodes
                .get(&id)
                .ok_or_else(|| fault("ordered-structure/outward-edge"))?;
            work.extend(edges(terminal)?.into_iter().map(|(_, id)| id));
        }
    }
    if reached.len() != nodes.len() {
        return Err(fault("ordered-structure/unreachable-node"));
    }
    for (id, terminal) in nodes {
        if function
            .blocks
            .get(id)
            .and_then(|block| block.terminator.as_ref())
            != Some(*terminal)
        {
            return Err(fault("ordered-structure/original-node"));
        }
    }
    if let Some(entry) = plain_entry {
        let incoming = boundary_incoming(function, &reached, entry)?;
        if entry != function.entry_block && incoming.is_empty() {
            return Err(fault("ordered-structure/missing-incoming"));
        }
    }
    Ok(())
}
