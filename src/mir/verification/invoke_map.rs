//! Finite opaque lifetime checks for the direct-local, acyclic Map cohort.
//! Source transfer order is validated by Completion, never inferred here.
use crate::mir::instruction::{
    InvokeNormalResultKind as Kind, InvokeOperation, MapInvokeOperation as Map,
};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet};

type Results = BTreeMap<ValueId, (Kind, BasicBlockId)>;

/// A `: MapBox` declared formal — caller-owned storage borrowed read-only.
/// It is not a `Map` result but is a valid read operand: the checked-map
/// argument contract keeps it live for the whole call.  Where signature-
/// aligned carriers were issued, `CheckedMapStorage` is the authority and
/// the `Box("MapBox")` name must corroborate it; functions built outside
/// every signature issuer keep the existing name contract.
fn is_map_param(function: &MirFunction, value: &ValueId) -> bool {
    let Some(index) = function.params.iter().position(|param| param == value) else {
        return false;
    };
    let named_map = matches!(
        function.signature.params.get(index),
        Some(crate::mir::MirType::Box(name)) if name == "MapBox"
    );
    match function.metadata.physical_param_carriers.as_deref() {
        Some(carriers) => {
            carriers.get(index).copied()
                == Some(crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::CheckedMapStorage)
                && named_map
        }
        None => named_map,
    }
}

/// The borrowed-argument lane admits only edges `check_call_edge` can
/// corroborate. The module-aware lane requires the callee to resolve to its
/// cataloged definition; the function-only lane has no catalog to consult,
/// so it keeps the sealed-callee shape boundary and leaves unresolvable
/// keys to the module pass. Every other callee — constructors, dynamic or
/// foreign targets — is an unproven escape, never a borrow.
fn borrowed_map_call_edge(
    call: &crate::mir::definitions::MirCall,
    module: Option<&crate::mir::MirModule>,
) -> bool {
    match module {
        Some(module) => super::cataloged_call_target(module, call).is_some(),
        None => super::cataloged_edge_key(call).is_some(),
    }
}

pub(super) fn check(
    function: &MirFunction,
    module: Option<&crate::mir::MirModule>,
) -> Result<(), &'static str> {
    if !function.blocks.values().any(|b| {
        b.all_instructions().any(|i| {
            matches!(
                i,
                MirInstruction::Invoke {
                    operation: InvokeOperation::Map(_),
                    ..
                } | MirInstruction::Invoke {
                    operation: InvokeOperation::Call {
                        result: crate::mir::instruction::InvokeCallResultKind::Map,
                        ..
                    },
                    ..
                }
            )
        })
    }) {
        return Ok(());
    }
    let mut results = Results::new();
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            let MirInstruction::InvokeNormalResult { invoke_block, dst } = instruction else {
                continue;
            };
            let Some(MirInstruction::Invoke { operation, .. }) = function
                .blocks
                .get(invoke_block)
                .and_then(|b| b.terminator.as_ref())
            else {
                continue;
            };
            if let Some(kind @ (Kind::Map | Kind::MapKey | Kind::MapOutcome)) =
                operation.normal_result_kind()
            {
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
            if let MirInstruction::Invoke {
                operation: InvokeOperation::Map(operation),
                ..
            } = instruction
            {
                let valid = match operation {
                    Map::New | Map::PrepareKey { .. } => true,
                    Map::InstallIndexed {
                        map, key, value, ..
                    }
                    | Map::InstallValue {
                        map, key, value, ..
                    } => {
                        has_kind(map, Kind::Map)
                            && has_kind(key, Kind::MapKey)
                            && !results.contains_key(value)
                    }
                    Map::InstallText { map, key, .. } | Map::InstallEmptyArray { map, key } => {
                        has_kind(map, Kind::Map) && has_kind(key, Kind::MapKey)
                    }
                    Map::CheckedGetI64 { map, .. } => {
                        has_kind(map, Kind::Map) || is_map_param(function, map)
                    }
                    Map::EndOutcome { outcome } => has_kind(outcome, Kind::MapOutcome),
                    Map::End { map } => has_kind(map, Kind::Map),
                };
                if !valid {
                    return Err("map-operand-role");
                }
            }
            for value in instruction
                .used_values()
                .into_iter()
                .filter(|v| results.contains_key(v))
            {
                let allowed = matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::Map(Map::InstallIndexed { map, key, .. } | Map::InstallValue { map, key, .. } | Map::InstallText { map, key, .. } | Map::InstallEmptyArray { map, key }), .. }
                        if value == *map || value == *key)
                    || matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::Map(Map::End { map }), .. } if value == *map)
                    || matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::Map(Map::CheckedGetI64 { map, .. }), .. } if value == *map)
                    || matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::Map(Map::EndOutcome { outcome }), .. } if value == *outcome)
                    // A Map lease may cross the return boundary exactly once:
                    // the caller's out storage becomes the placement owner.
                    || (matches!(instruction, MirInstruction::Return { value: Some(returned) } if value == *returned)
                        && results[&value].0 == Kind::Map)
                    // A caller-owned Map lease borrowed across a sealed call
                    // edge: the callee reads it through its borrowed map
                    // formal while ownership stays here — the caller's
                    // cleanup chain still owes the single End. The edge must
                    // be one `check_call_edge` actually corroborates the
                    // actual/formal pair on; an uncorroborated callee is an
                    // escape, never a borrow.
                    || (matches!(instruction, MirInstruction::Invoke {
                            operation: InvokeOperation::Call { call, .. }, ..
                        } if call.args.contains(&value)
                            && borrowed_map_call_edge(call, module))
                        && results[&value].0 == Kind::Map);
                if !allowed {
                    return Err("map-opaque-escape");
                }
                *uses.entry(value).or_default() += 1;
            }
        }
        if block
            .return_env
            .as_ref()
            .is_some_and(|env| env.iter().any(|v| results.contains_key(v)))
        {
            return Err("map-opaque-escape");
        }
    }
    // These temporaries have one immediate consumer in this source cohort.
    // Fresh/nested child evaluation and its cancellation edges are not admitted.
    for (value, (kind, producer)) in &results {
        if *kind == Kind::Map {
            continue;
        }
        let Some(MirInstruction::Invoke { normal_landing, .. }) =
            function.blocks[producer].terminator.as_ref()
        else {
            unreachable!()
        };
        let normal = function
            .blocks
            .get(normal_landing)
            .ok_or("map-normal-missing")?;
        let immediate = matches!(normal.terminator.as_ref(),
            Some(MirInstruction::Invoke { operation: InvokeOperation::Map(Map::InstallIndexed { key, .. } | Map::InstallValue { key, .. } | Map::InstallText { key, .. } | Map::InstallEmptyArray { key, .. }), .. })
                if *kind == Kind::MapKey && key == value)
            || matches!(normal.terminator.as_ref(),
            Some(MirInstruction::Invoke { operation: InvokeOperation::Map(Map::EndOutcome { outcome }), .. })
                if *kind == Kind::MapOutcome && outcome == value);
        if normal.instructions.len() != 1 || !immediate || uses.get(value) != Some(&1) {
            return Err("map-temporary-consumption");
        }
    }
    visit(
        function,
        function.entry_block,
        BTreeSet::new(),
        &results,
        &mut BTreeMap::new(),
        &mut BTreeSet::new(),
    )
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
        if leaving {
            active.remove(&id);
            continue;
        }
        if active.contains(&id) {
            return Err("map-lifetime-cycle");
        }
        if let Some(previous) = seen.get(&id) {
            if previous != &live {
                return Err("map-lifetime-join");
            }
            continue;
        }
        seen.insert(id, live.clone());
        active.insert(id);
        let block = function.blocks.get(&id).ok_or("map-block-missing")?;
        let term = block.terminator.as_ref().ok_or("map-terminator-missing")?;
        if let MirInstruction::Invoke {
            operation: InvokeOperation::Map(operation),
            ..
        } = term
        {
            match operation {
                Map::End { map } if !live.remove(map) => return Err("map-end-not-live"),
                Map::CheckedGetI64 { map, .. }
                    if !live.contains(map) && !is_map_param(function, map) =>
                {
                    return Err("map-get-not-live")
                }
                Map::InstallIndexed { map, .. }
                | Map::InstallValue { map, .. }
                | Map::InstallText { map, .. }
                | Map::InstallEmptyArray { map, .. }
                    if !live.contains(map) =>
                {
                    return Err("map-install-not-live")
                }
                _ => {}
            }
        }
        // A returned Map lease transfers to the caller's out storage here;
        // it is consumed by the boundary, not by an End in this function.
        if let MirInstruction::Return {
            value: Some(returned),
        } = term
        {
            if results.get(returned).is_some_and(|r| r.0 == Kind::Map) && !live.remove(returned) {
                return Err("map-return-not-live");
            }
        }
        let targets = block.successors_from_terminator();
        if targets.is_empty() && !live.is_empty() {
            return Err("map-missing-end");
        }
        pending.push((id, BTreeSet::new(), true));
        for target in targets.into_iter().rev() {
            let mut next = live.clone();
            if matches!(term, MirInstruction::Invoke {
                operation, normal_landing, ..
            } if *normal_landing == target && operation.normal_result_kind() == Some(Kind::Map))
            {
                let mut values = results
                    .iter()
                    .filter(|(_, (kind, producer))| *kind == Kind::Map && *producer == id);
                let (&value, _) = values.next().ok_or("map-result-missing")?;
                if values.next().is_some() || !next.insert(value) {
                    return Err("map-result-duplicate");
                }
            }
            pending.push((target, next, false));
        }
    }
    Ok(())
}
