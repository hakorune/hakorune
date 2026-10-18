//! Exact cleanup graph coverage independent of any F=N node-count formula.
//! Source-order path validation separately certifies every release obligation.
use super::super::root_home::RootHomeExitEntry;
use super::*;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_original(
    function: &MirFunction,
    bindings: &Bindings,
    entry: &RootHomeExitEntry,
    expected_kind: crate::mir::instruction::InvokeCallResultKind,
) -> Result<(), String> {
    validate_original_with_wrapper(function, bindings, entry, expected_kind, &[])
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_original_with_wrapper(
    function: &MirFunction, bindings: &Bindings, entry: &RootHomeExitEntry,
    expected_kind: crate::mir::instruction::InvokeCallResultKind,
    wrapper_suffix: &[(BasicBlockId, MirInstruction)],
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
            let (clean, pending, _, _) =
                call::ingress(function, bindings, invoke, projection, expected_kind)?;
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
            // Plain ingress is the original source continuation: its prefix
            // belongs to the source/Loop owner, not this cleanup DAG. The shared
            // PhysicalBoundary independently verifies its exact source Phi role.
            || (Some(*id) != plain_entry && block
                .instructions
                .iter()
                .any(|i| matches!(i, MirInstruction::Phi { .. })))
        {
            return Err(fault("ordered-structure/original-node"));
        }
        if Some(*id) != plain_entry {
            let expected = projected_result
                .filter(|(base, _)| *base == *id)
                .map(|(_, i)| std::slice::from_ref(i))
                .unwrap_or(&[]);
            let suffix = wrapper_suffix.iter().filter(|(block, _)| block == id)
                .map(|(_, instruction)| instruction).collect::<Vec<_>>();
            if suffix.len() > 1 || block.instructions.iter().collect::<Vec<_>>()
                != expected.iter().chain(suffix.into_iter()).collect::<Vec<_>>() {
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
    expected_kind: crate::mir::instruction::InvokeCallResultKind,
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
            let (clean, pending, _, _) =
                call::ingress(function, &bindings, &invoke, &result, expected_kind)?;
            (vec![clean, pending], None)
        }
    };
    // Existing PhysicalBoundary::validate_complete independently checks merged
    // instruction sequences before the owner can enter FinishingChecked.
    topology(function, &nodes, starts, plain_entry)
}
/// Root Call payloads are already rebound before their affine final handoff.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_finished_call(
    function: &MirFunction,
    bindings: &Bindings,
    entry: &RootHomeExitEntry,
    expected_kind: crate::mir::instruction::InvokeCallResultKind,
) -> Result<(), String> {
    let RootHomeExitEntry::Call {
        invoke, projection, ..
    } = entry
    else {
        return Err(fault("ordered-structure/final-entry"));
    };
    let nodes = collect_nodes(bindings)?;
    let (clean, pending, _, _) =
        call::ingress(function, bindings, invoke, projection, expected_kind)?;
    topology(function, &nodes, vec![clean, pending], None)
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
    topology_until(function, nodes, starts, plain_entry, None)
}
fn topology_until(
    function: &MirFunction, nodes: &BTreeMap<BasicBlockId, &MirInstruction>,
    starts: Vec<BasicBlockId>, plain_entry: Option<BasicBlockId>, stop: Option<BasicBlockId>,
) -> Result<(), String> {
    let mut reached = BTreeSet::new();
    let mut work = starts;
    while let Some(id) = work.pop() {
        if Some(id) == stop { continue; }
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

/// Exact owned terminals with one caller-owned continuation. Prefixes and
/// source operation order are corroborated by their existing physical owners.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_to_continuation_v2(
    function: &MirFunction, terminals: &Bindings, entry: BasicBlockId, continuation: BasicBlockId,
) -> Result<(), String> {
    if !function.blocks.contains_key(&continuation) { return Err(fault("ordered-structure/missing-stop")); }
    let mut nodes = BTreeMap::new();
    for (id, terminal) in terminals {
        if matches!(terminal, MirInstruction::Return { .. }) { return Err(fault("ordered-structure/loop-return")); }
        if nodes.insert(*id, terminal).is_some() { return Err(fault("ordered-structure/duplicate-node")); }
        edges(terminal)?;
    }
    require_acyclic_until(&nodes, Some(continuation))?;
    topology_until(function, &nodes, vec![entry], Some(entry), Some(continuation))
}
