//! Finite opaque lifetime checks for the direct-local, acyclic Map cohort.
//! Source transfer order is validated by Completion, never inferred here.
use crate::mir::instruction::{InvokeNormalResultKind as Kind, InvokeOperation, MapInvokeOperation as Map};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet};

type Results = BTreeMap<ValueId, (Kind, BasicBlockId)>;

pub(super) fn check(function: &MirFunction) -> Result<(), &'static str> {
    if !function.blocks.values().any(|b| b.all_instructions().any(|i|
        matches!(i, MirInstruction::Invoke { operation: InvokeOperation::Map(_), .. }))) {
        return Ok(());
    }
    let mut results = Results::new();
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            let MirInstruction::InvokeNormalResult { invoke_block, dst } = instruction else { continue };
            let Some(MirInstruction::Invoke { operation, .. }) = function.blocks
                .get(invoke_block).and_then(|b| b.terminator.as_ref()) else { continue };
            if let Some(kind @ (Kind::Map | Kind::MapKey | Kind::MapOutcome)) = operation.normal_result_kind() {
                if results.insert(*dst, (kind, *invoke_block)).is_some() {
                    return Err("map-duplicate-result");
                }
            }
        }
    }
    let has_kind = |value: &ValueId, kind| results.get(value).is_some_and(|r| r.0 == kind);
    let mut uses = BTreeMap::<ValueId, usize>::new();
    for block in function.blocks.values() {
        for instruction in block.all_instructions() {
            if let MirInstruction::Invoke { operation: InvokeOperation::Map(operation), .. } = instruction {
                let valid = match operation {
                    Map::New | Map::PrepareKey { .. } => true,
                    Map::InstallIndexed { map, key, value, .. } | Map::InstallValue { map, key, value, .. } => has_kind(map, Kind::Map)
                        && has_kind(key, Kind::MapKey) && !results.contains_key(value),
                    Map::EndOutcome { outcome } => has_kind(outcome, Kind::MapOutcome),
                    Map::End { map } => has_kind(map, Kind::Map),
                };
                if !valid { return Err("map-operand-role"); }
            }
            for value in instruction.used_values().into_iter().filter(|v| results.contains_key(v)) {
                let allowed = matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::Map(Map::InstallIndexed { map, key, .. } | Map::InstallValue { map, key, .. }), .. }
                        if value == *map || value == *key)
                    || matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::Map(Map::End { map }), .. } if value == *map)
                    || matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::Map(Map::EndOutcome { outcome }), .. } if value == *outcome);
                if !allowed { return Err("map-opaque-escape"); }
                *uses.entry(value).or_default() += 1;
            }
        }
        if block.return_env.as_ref().is_some_and(|env| env.iter().any(|v| results.contains_key(v))) {
            return Err("map-opaque-escape");
        }
    }
    // These temporaries have one immediate consumer in this source cohort.
    // Fresh/nested child evaluation and its cancellation edges are not admitted.
    for (value, (kind, producer)) in &results {
        if *kind == Kind::Map { continue; }
        let Some(MirInstruction::Invoke { normal_landing, .. }) = function.blocks[producer].terminator.as_ref() else { unreachable!() };
        let normal = function.blocks.get(normal_landing).ok_or("map-normal-missing")?;
        let immediate = matches!(normal.terminator.as_ref(),
            Some(MirInstruction::Invoke { operation: InvokeOperation::Map(Map::InstallIndexed { key, .. } | Map::InstallValue { key, .. }), .. })
                if *kind == Kind::MapKey && key == value)
            || matches!(normal.terminator.as_ref(),
            Some(MirInstruction::Invoke { operation: InvokeOperation::Map(Map::EndOutcome { outcome }), .. })
                if *kind == Kind::MapOutcome && outcome == value);
        if normal.instructions.len() != 1 || !immediate || uses.get(value) != Some(&1) {
            return Err("map-temporary-consumption");
        }
    }
    visit(function, function.entry_block, BTreeSet::new(), &results,
        &mut BTreeMap::new(), &mut BTreeSet::new())
}

fn visit(
    function: &MirFunction,
    id: BasicBlockId,
    live: BTreeSet<ValueId>,
    results: &Results,
    seen: &mut BTreeMap<BasicBlockId, BTreeSet<ValueId>>,
    active: &mut BTreeSet<BasicBlockId>,
) -> Result<(), &'static str> {
    // Explicit DFS stack keeps long straight-line Map programs off the Rust stack.
    let mut pending = vec![(id, live, false)];
    while let Some((id, mut live, leaving)) = pending.pop() {
        if leaving { active.remove(&id); continue; }
        if active.contains(&id) { return Err("map-lifetime-cycle"); }
        if let Some(previous) = seen.get(&id) {
            if previous != &live { return Err("map-lifetime-join"); }
            continue;
        }
        seen.insert(id, live.clone());
        active.insert(id);
        let block = function.blocks.get(&id).ok_or("map-block-missing")?;
        let term = block.terminator.as_ref().ok_or("map-terminator-missing")?;
        if let MirInstruction::Invoke { operation: InvokeOperation::Map(operation), .. } = term {
            match operation {
                Map::End { map } if !live.remove(map) => return Err("map-end-not-live"),
                Map::InstallIndexed { map, .. } | Map::InstallValue { map, .. } if !live.contains(map) => return Err("map-install-not-live"),
                _ => {}
            }
        }
        let targets = block.successors_from_terminator();
        if targets.is_empty() && !live.is_empty() { return Err("map-missing-end"); }
        pending.push((id, BTreeSet::new(), true));
        for target in targets.into_iter().rev() {
            let mut next = live.clone();
            if matches!(term, MirInstruction::Invoke {
                operation: InvokeOperation::Map(Map::New), normal_landing, .. } if *normal_landing == target) {
                let mut values = results.iter().filter(|(_, (kind, producer))| *kind == Kind::Map && *producer == id);
                let (&value, _) = values.next().ok_or("map-result-missing")?;
                if values.next().is_some() || !next.insert(value) { return Err("map-result-duplicate"); }
            }
            pending.push((target, next, false));
        }
    }
    Ok(())
}
