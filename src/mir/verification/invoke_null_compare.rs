//! Actual-MIR null observations: direct sentinel, available carriers and executable lifetime.
//! Source plans never authorize these checks; publication retains its separate source closure.
use super::*;
use crate::mir::{ConstValue, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

type Failure = (BasicBlockId, &'static str);
type Definitions<'a> = BTreeMap<ValueId, (BasicBlockId, &'a MirInstruction)>;

fn definitions(function: &MirFunction) -> Result<Definitions<'_>, Failure> {
    let mut definitions = BTreeMap::new();
    let params: BTreeSet<_> = function.params.iter().copied().collect();
    if params.len() != function.params.len() {
        return Err((function.entry_block, "null-comparison/parameter-duplicate"));
    }
    for (id, block) in &function.blocks {
        if block
            .successors_from_terminator()
            .iter()
            .any(|target| !function.blocks.contains_key(target))
        {
            return Err((*id, "null-comparison/successor-missing"));
        }
        for row in block.all_instructions() {
            if let Some(dst) = row.dst_value() {
                if params.contains(&dst) || definitions.insert(dst, (*id, row)).is_some() {
                    return Err((*id, "null-comparison/definition-duplicate"));
                }
            }
        }
    }
    Ok(definitions)
}
fn direct_null(definitions: &Definitions<'_>, value: ValueId) -> bool {
    matches!(
        definitions.get(&value),
        Some((
            _,
            MirInstruction::Const {
                value: ConstValue::Null,
                ..
            }
        ))
    )
}
fn carrier(row: &MirInstruction, definitions: &Definitions<'_>) -> Result<ValueId, &'static str> {
    let MirInstruction::NullCompare { op, lhs, rhs, .. } = row else {
        unreachable!()
    };
    if !matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne) {
        return Err("null-comparison/operator");
    }
    match (
        direct_null(definitions, *lhs),
        direct_null(definitions, *rhs),
    ) {
        (true, false) => Ok(*rhs),
        (false, true) => Ok(*lhs),
        _ => Err("null-comparison/exact-sentinel"),
    }
}

/// Independent actual value domains. Unknown formal carriers remain ABI
/// obligations; this does not admit them into a canonical executable.
fn check_kind(
    function: &MirFunction,
    definitions: &Definitions<'_>,
    value: ValueId,
    active: &mut BTreeSet<ValueId>,
) -> Result<(), &'static str> {
    if !active.insert(value) {
        return Err("null-comparison/carrier-cycle");
    }
    let result = (|| {
        if let Some(index) = function.params.iter().position(|param| *param == value) {
            return match function.signature.params.get(index) {
                Some(MirType::Integer | MirType::Bool | MirType::Box(_) | MirType::Unknown) => {
                    Ok(())
                }
                _ => Err("null-comparison/carrier-kind"),
            };
        }
        let (_, row) = definitions
            .get(&value)
            .ok_or("null-comparison/carrier-definition")?;
        match row {
            MirInstruction::Copy { src, .. } => check_kind(function, definitions, *src, active),
            MirInstruction::Phi { inputs, .. } if !inputs.is_empty() => {
                for (_, value) in inputs {
                    check_kind(function, definitions, *value, active)?;
                }
                Ok(())
            }
            MirInstruction::Const {
                value: ConstValue::Integer(_) | ConstValue::Bool(_) | ConstValue::Null,
                ..
            }
            | MirInstruction::Compare { .. }
            | MirInstruction::NullCompare { .. }
            | MirInstruction::StringLength { .. }
            | MirInstruction::StringIndexOf { .. }
            | MirInstruction::StringCompare { .. }
            | MirInstruction::ObjectFieldGet { .. }
            | MirInstruction::NewBox { .. } => Ok(()),
            MirInstruction::BinOp { lhs, rhs, .. } => {
                check_kind(function, definitions, *lhs, active)?;
                check_kind(function, definitions, *rhs, active)
            }
            MirInstruction::InvokeNormalResult { invoke_block, .. } => {
                let Some(MirInstruction::Invoke { operation, .. }) = function
                    .blocks
                    .get(invoke_block)
                    .and_then(|block| block.terminator.as_ref())
                else {
                    return Err("null-comparison/carrier-invoke");
                };
                use crate::mir::instruction::InvokeNormalResultKind as Kind;
                match operation.normal_result_kind() {
                    Some(
                        Kind::I64
                        | Kind::Bool
                        | Kind::DynamicCarrier
                        | Kind::Handle
                        | Kind::NullableHandle
                        | Kind::BorrowedNullableHandle
                        | Kind::MixedHandle,
                    ) => Ok(()),
                    _ => Err("null-comparison/carrier-kind"),
                }
            }
            _ => Err("null-comparison/carrier-kind"),
        }
    })();
    active.remove(&value);
    result
}

/// Source-free support dependencies. A field snapshot still borrows its base;
/// Copy/Phi transport does not create a fresh resource or hide an earlier End.
fn identity(
    definitions: &Definitions<'_>,
    aliases: &BTreeMap<ValueId, ValueId>,
    value: ValueId,
) -> ValueId {
    let mut current = value;
    let mut seen = BTreeSet::new();
    while seen.insert(current) {
        if let Some(next) = aliases.get(&current) {
            current = *next;
        } else if let Some((_, MirInstruction::Copy { src, .. })) = definitions.get(&current) {
            current = *src;
        } else {
            break;
        }
    }
    current
}
fn supports(
    function: &MirFunction,
    definitions: &Definitions<'_>,
    aliases: &BTreeMap<ValueId, ValueId>,
    value: ValueId,
    seen: &mut BTreeSet<ValueId>,
    roots: &mut BTreeSet<ValueId>,
    bases: bool,
) {
    let value = identity(definitions, aliases, value);
    if !seen.insert(value) {
        return;
    }
    roots.insert(value);
    if !bases {
        return;
    }
    let base = match definitions.get(&value).map(|(_, row)| *row) {
        Some(MirInstruction::ObjectFieldGet { base, .. }) => Some(*base),
        Some(MirInstruction::InvokeNormalResult { invoke_block, .. }) => {
            use crate::mir::instruction::{
                BorrowedResultSupportV1 as Support, InvokeCallResultKind as ResultKind,
            };
            match function
                .blocks
                .get(invoke_block)
                .and_then(|block| block.terminator.as_ref())
            {
                Some(MirInstruction::Invoke {
                    operation: InvokeOperation::ObjectFieldBorrow { base, .. },
                    ..
                }) => Some(*base),
                Some(MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, result, .. },
                    ..
                }) => match result {
                    ResultKind::BorrowedNullableHandle {
                        support: Support::Receiver | Support::ReceiverFieldSlot { .. },
                        ..
                    } => call.callee.receiver(),
                    ResultKind::BorrowedNullableHandle {
                        support: Support::Formal(ordinal),
                        ..
                    }
                    | ResultKind::MixedHandle {
                        formal_ordinal: ordinal,
                    } => call.args.get(*ordinal as usize).copied(),
                    _ => None,
                },
                _ => None,
            }
        }
        _ => None,
    };
    if let Some(base) = base {
        supports(function, definitions, aliases, base, seen, roots, true);
    }
}
fn ended(instruction: &MirInstruction) -> Option<ValueId> {
    use crate::mir::instruction::StringInvokeOperation as Text;
    match instruction {
        MirInstruction::Invoke {
            operation: InvokeOperation::DynamicCarrierEnd { carrier },
            ..
        } => Some(*carrier),
        MirInstruction::Invoke {
            operation:
                InvokeOperation::HomeRelease { value, .. }
                | InvokeOperation::HomeReleaseIfLive { value, .. }
                | InvokeOperation::ObjectEndIfOwned { value, .. }
                | InvokeOperation::ReclaimUnpublished { value, .. }
                | InvokeOperation::String(Text::End { value }),
            ..
        } => Some(*value),
        MirInstruction::DestroyOwned { value, .. } => Some(*value),
        _ => None,
    }
}

pub(super) fn check(function: &MirFunction) -> Result<(), Failure> {
    if !function.blocks.values().any(|block| {
        block
            .all_instructions()
            .any(|row| matches!(row, MirInstruction::NullCompare { .. }))
    }) {
        return Ok(());
    }
    let definitions = definitions(function)?;
    for (id, block) in &function.blocks {
        for row in block.all_instructions() {
            let MirInstruction::NullCompare { dst, lhs, rhs, .. } = row else {
                continue;
            };
            let value = carrier(row, &definitions).map_err(|why| (*id, why))?;
            if block
                .terminator
                .as_ref()
                .is_some_and(|term| std::ptr::eq(term, row))
            {
                return Err((*id, "null-comparison/terminator"));
            }
            if function
                .metadata
                .value_types
                .get(dst)
                .is_some_and(|ty| *ty != MirType::Bool)
            {
                return Err((*id, "null-comparison/result-type"));
            }
            for value in [*lhs, *rhs] {
                string::check_available(function, *id, row, value).map_err(|why| (*id, why))?;
            }
            check_kind(function, &definitions, value, &mut BTreeSet::new())
                .map_err(|why| (*id, why))?;
        }
    }
    // Retain incoming-edge Phi identity while checking every executable path.
    // Joining dead sets first would lose which resource a Phi actually selected.
    let initial = (
        function.entry_block,
        None,
        BTreeSet::new(),
        BTreeMap::new(),
        false,
    );
    let mut pending = VecDeque::from([initial]);
    let mut visited = BTreeSet::new();
    while let Some(state) = pending.pop_front() {
        if !visited.insert(state.clone()) {
            continue;
        }
        let (id, predecessor, mut dead, mut aliases, faulted) = state;
        let block = function
            .blocks
            .get(&id)
            .ok_or((id, "null-comparison/block-missing"))?;
        let mut incoming = Vec::new();
        for row in &block.instructions {
            if let MirInstruction::Phi { dst, inputs, .. } = row {
                let mut matches = inputs.iter().filter(|(from, _)| Some(*from) == predecessor);
                let (_, value) = matches.next().ok_or((id, "null-comparison/phi-incoming"))?;
                if matches.next().is_some() {
                    return Err((id, "null-comparison/phi-incoming"));
                }
                incoming.push((*dst, identity(&definitions, &aliases, *value)));
            }
        }
        aliases.extend(incoming);
        for row in block.all_instructions() {
            if let MirInstruction::InvokeNormalResult { dst, .. } = row {
                dead.remove(dst);
            }
            if matches!(row, MirInstruction::NullCompare { .. }) {
                let mut roots = BTreeSet::new();
                supports(
                    function,
                    &definitions,
                    &aliases,
                    carrier(row, &definitions).map_err(|why| (id, why))?,
                    &mut BTreeSet::new(),
                    &mut roots,
                    true,
                );
                if faulted || !dead.is_disjoint(&roots) {
                    return Err((id, "null-comparison/carrier-not-live"));
                }
            }
            if let Some(value) = ended(row) {
                supports(
                    function,
                    &definitions,
                    &aliases,
                    value,
                    &mut BTreeSet::new(),
                    &mut dead,
                    false,
                );
            }
        }
        for target in block.successors_from_terminator() {
            let fault_edge = matches!(block.terminator, Some(MirInstruction::Invoke { fault_landing, .. }) if target == fault_landing);
            pending.push_back((
                target,
                Some(id),
                dead.clone(),
                aliases.clone(),
                faulted || fault_edge,
            ));
        }
    }
    Ok(())
}

/// Called only for functions already selected for canonical physical publication.
/// Compatibility dumps do not obtain execution admission from this check.
pub(super) fn reject_legacy(function: &MirFunction) -> Result<(), &'static str> {
    let definitions = definitions(function).map_err(|(_, why)| why)?;
    for row in function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
    {
        if matches!(row, MirInstruction::Compare { lhs, rhs, .. }
            if direct_null(&definitions, *lhs) || direct_null(&definitions, *rhs))
        {
            return Err("null-comparison/legacy-compare");
        }
    }
    Ok(())
}
