//! Actual-MIR null observations: direct sentinel, available carriers and executable lifetime.
//! Source plans never authorize these checks; publication retains its separate source closure.
use super::*;
use crate::mir::{ConstValue, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

type Failure = (BasicBlockId, &'static str);
type Definitions = crate::mir::value_origin::ValueDefMap;

pub(super) fn structural_failure(
    function: &MirFunction,
    errors: &[VerificationError],
) -> Option<Failure> {
    errors
        .iter()
        .find_map(|error| match error {
            VerificationError::MultipleDefinition {
                value,
                second_block,
                ..
            } => Some((
                *second_block,
                if function.params.iter().filter(|v| *v == value).count() > 1 {
                    "null-comparison/parameter-duplicate"
                } else {
                    "null-comparison/definition-duplicate"
                },
            )),
            _ => None,
        })
        .or_else(|| {
            errors.iter().find_map(|error| match error {
                VerificationError::ControlFlowError { block, .. }
                    if function.blocks[block]
                        .successors_from_terminator()
                        .iter()
                        .any(|target| !function.blocks.contains_key(target)) =>
                {
                    Some((*block, "null-comparison/successor-missing"))
                }
                _ => None,
            })
        })
}
fn definition<'a>(
    function: &'a MirFunction,
    definitions: &Definitions,
    value: ValueId,
) -> Option<(BasicBlockId, &'a MirInstruction)> {
    let (block, index) = definitions.get(&value)?;
    let body = function.blocks.get(block)?;
    Some((
        *block,
        body.instructions.get(*index).or_else(|| {
            (*index == body.instructions.len())
                .then_some(body.terminator.as_ref())
                .flatten()
        })?,
    ))
}
fn direct_null(function: &MirFunction, definitions: &Definitions, value: ValueId) -> bool {
    matches!(
        definition(function, definitions, value),
        Some((
            _,
            MirInstruction::Const {
                value: ConstValue::Null,
                ..
            }
        ))
    )
}
fn carrier(
    function: &MirFunction,
    row: &MirInstruction,
    definitions: &Definitions,
) -> Result<ValueId, &'static str> {
    let MirInstruction::NullCompare { op, lhs, rhs, .. } = row else {
        unreachable!()
    };
    if !matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne) {
        return Err("null-comparison/operator");
    }
    match (
        direct_null(function, definitions, *lhs),
        direct_null(function, definitions, *rhs),
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
    definitions: &Definitions,
    value: ValueId,
    active: &mut BTreeSet<ValueId>,
    completed: &mut BTreeMap<ValueId, Result<(), &'static str>>,
) -> Result<(), &'static str> {
    if let Some(result) = completed.get(&value) {
        return *result;
    }
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
        let (_, row) =
            definition(function, definitions, value).ok_or("null-comparison/carrier-definition")?;
        match row {
            MirInstruction::Copy { src, .. } => {
                check_kind(function, definitions, *src, active, completed)
            }
            MirInstruction::Phi { inputs, .. } if !inputs.is_empty() => {
                for (_, value) in inputs {
                    check_kind(function, definitions, *value, active, completed)?;
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
            // A compatibility named field is only an opaque Box domain here.
            // It still needs the existing publication route/layout proof.
            MirInstruction::FieldGet {
                declared_type: Some(MirType::Box(_)),
                ..
            } => Ok(()),
            MirInstruction::BinOp { lhs, rhs, .. } => {
                check_kind(function, definitions, *lhs, active, completed)?;
                check_kind(function, definitions, *rhs, active, completed)
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
    completed.insert(value, result);
    result
}

/// Only acquisition establishes support; joined may-relations are never closed transitively.
fn support_base(
    function: &MirFunction,
    definitions: &Definitions,
    value: ValueId,
) -> Option<ValueId> {
    match definition(function, definitions, value).map(|(_, row)| row) {
        Some(
            MirInstruction::ObjectFieldGet { base, .. } | MirInstruction::FieldGet { base, .. },
        ) => Some(*base),
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
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
struct Live {
    bad: BTreeSet<ValueId>,
    depends: BTreeSet<(ValueId, ValueId)>,
}
impl Live {
    /// Simultaneous Copy/edge-PHI substitution of BOTH relation axes.
    fn substitute(
        &self,
        values: &BTreeSet<ValueId>,
        replacements: &BTreeMap<ValueId, ValueId>,
    ) -> Self {
        let mut inverse = BTreeMap::<ValueId, Vec<ValueId>>::new();
        for v in values {
            inverse
                .entry(*replacements.get(v).unwrap_or(v))
                .or_default()
                .push(*v);
        }
        let mut next = Self::default();
        for v in &self.bad {
            next.bad
                .extend(inverse.get(v).into_iter().flatten().copied());
        }
        for (v, u) in &self.depends {
            for lhs in inverse.get(v).into_iter().flatten() {
                next.depends
                    .extend(inverse.get(u).into_iter().flatten().map(|rhs| (*lhs, *rhs)));
            }
        }
        next
    }
    fn acquire(&mut self, value: ValueId, base: Option<ValueId>, fresh: bool) {
        let inherited: Vec<_> = self
            .depends
            .iter()
            .filter_map(|(v, u)| (Some(*v) == base).then_some(*u))
            .collect();
        let invalid =
            (!fresh && self.bad.contains(&value)) || base.is_some_and(|v| self.bad.contains(&v));
        self.depends.retain(|(v, u)| *v != value && *u != value);
        self.bad.remove(&value);
        self.depends.insert((value, value));
        self.depends
            .extend(inherited.into_iter().map(|u| (value, u)));
        if invalid {
            self.bad.insert(value);
        }
    }
    fn join(&mut self, other: Self) -> bool {
        let old = (self.bad.len(), self.depends.len());
        self.bad.extend(other.bad);
        self.depends.extend(other.depends);
        old != (self.bad.len(), self.depends.len())
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
    let (definitions, mut errors) = ssa::definitions(function);
    if let Err(mut found) = cfg::check_control_flow(function) {
        errors.append(&mut found);
    }
    if let Some(error) = structural_failure(function, &errors) {
        return Err(error);
    }
    check_with_definitions(function, &definitions)
}

pub(super) fn check_with_definitions(
    function: &MirFunction,
    definitions: &Definitions,
) -> Result<(), Failure> {
    let mut completed = BTreeMap::new();
    for (id, block) in &function.blocks {
        for row in block.all_instructions() {
            let MirInstruction::NullCompare { dst, lhs, rhs, .. } = row else {
                continue;
            };
            let value = carrier(function, row, definitions).map_err(|why| (*id, why))?;
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
            check_kind(
                function,
                &definitions,
                value,
                &mut BTreeSet::new(),
                &mut completed,
            )
            .map_err(|why| (*id, why))?;
        }
    }
    // A tail that cannot reach any observation cannot affect a null read.
    // Compute this once from actual Normal/Fault edges, retaining returning
    // loops and every prefix; never cap the number of executable states.
    let mut predecessors: BTreeMap<BasicBlockId, Vec<BasicBlockId>> = BTreeMap::new();
    let mut reaches_observation = BTreeSet::new();
    let mut reverse_pending = VecDeque::new();
    for (id, block) in &function.blocks {
        for target in block.successors_from_terminator() {
            predecessors.entry(target).or_default().push(*id);
        }
        if block
            .all_instructions()
            .any(|row| matches!(row, MirInstruction::NullCompare { .. }))
        {
            reaches_observation.insert(*id);
            reverse_pending.push_back(*id);
        }
    }
    while let Some(id) = reverse_pending.pop_front() {
        for predecessor in predecessors.get(&id).into_iter().flatten() {
            if reaches_observation.insert(*predecessor) {
                reverse_pending.push_back(*predecessor);
            }
        }
    }
    // Keep only resource-related components. Scalar PHIs unrelated to observations
    // do not participate in this lifetime lattice (their SSA/kind checks still run).
    let mut related = BTreeMap::<ValueId, BTreeSet<ValueId>>::new();
    let mut values = BTreeSet::new();
    for value in definitions.keys() {
        let Some((_, row)) = definition(function, definitions, *value) else {
            continue;
        };
        if matches!(row, MirInstruction::NullCompare { .. }) {
            values.insert(
                carrier(function, row, definitions).map_err(|why| (function.entry_block, why))?,
            );
        }
        if let Some(dst) = row.dst_value() {
            let inputs = match row {
                MirInstruction::Copy { src, .. } => vec![*src],
                MirInstruction::Phi { inputs, .. } => inputs.iter().map(|(_, v)| *v).collect(),
                _ => support_base(function, definitions, dst)
                    .into_iter()
                    .collect(),
            };
            for input in inputs {
                related.entry(dst).or_default().insert(input);
                related.entry(input).or_default().insert(dst);
            }
        }
    }
    let mut origins = crate::mir::value_origin::ValueOriginQueryContext::new(function, definitions);
    let mut pending_values: VecDeque<_> = values.iter().copied().collect();
    while let Some(v) = pending_values.pop_front() {
        if values.insert(origins.origin(v)) {
            pending_values.push_back(origins.origin(v));
        }
        for next in related.get(&v).into_iter().flatten() {
            if values.insert(*next) {
                pending_values.push_back(*next);
            }
        }
    }
    if function
        .blocks
        .get(&function.entry_block)
        .ok_or((function.entry_block, "null-comparison/block-missing"))?
        .instructions
        .iter()
        .any(|row| matches!(row, MirInstruction::Phi { .. }))
    {
        return Err((function.entry_block, "null-comparison/phi-incoming"));
    }
    let mut initial = Live::default();
    initial.depends.extend(values.iter().map(|v| (*v, *v)));
    let entry = (function.entry_block, false);
    let mut states = BTreeMap::from([(entry, initial)]);
    let mut pending = VecDeque::from([entry]);
    let mut queued = BTreeSet::from([entry]);
    while let Some(key @ (id, faulted)) = pending.pop_front() {
        queued.remove(&key);
        let mut live = states[&key].clone();
        let block = &function.blocks[&id];
        for row in block.all_instructions() {
            if let Some(dst) = row.dst_value().filter(|v| values.contains(v)) {
                match row {
                    MirInstruction::Phi { .. } => (), // edge simultaneous assignment
                    MirInstruction::Copy { src, .. } => {
                        live = live.substitute(&values, &BTreeMap::from([(dst, *src)]))
                    }
                    _ => live.acquire(
                        dst,
                        support_base(function, definitions, dst),
                        matches!(row, MirInstruction::InvokeNormalResult { .. }),
                    ),
                }
            }
            if matches!(row, MirInstruction::NullCompare { .. }) {
                let value = carrier(function, row, definitions).map_err(|why| (id, why))?;
                if faulted || live.bad.contains(&value) {
                    return Err((id, "null-comparison/carrier-not-live"));
                }
            }
            if let Some(value) = ended(row) {
                live.bad.extend(
                    live.depends
                        .iter()
                        .filter_map(|(v, u)| (*u == value).then_some(*v)),
                );
            }
        }
        for target in block.successors_from_terminator() {
            if !reaches_observation.contains(&target) {
                continue;
            }
            let mut replacements = BTreeMap::new();
            for row in &function.blocks[&target].instructions {
                if let MirInstruction::Phi { dst, inputs, .. } = row {
                    let mut incoming = inputs.iter().filter(|(from, _)| *from == id);
                    let (_, value) = incoming
                        .next()
                        .ok_or((target, "null-comparison/phi-incoming"))?;
                    if incoming.next().is_some() {
                        return Err((target, "null-comparison/phi-incoming"));
                    }
                    if values.contains(dst) {
                        replacements.insert(*dst, *value);
                    }
                }
            }
            let next = live.substitute(&values, &replacements);
            let fault_edge = matches!(block.terminator, Some(MirInstruction::Invoke { fault_landing, .. }) if target==fault_landing);
            let key = (target, faulted || fault_edge);
            let changed = match states.entry(key) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(next);
                    true
                }
                std::collections::btree_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().join(next)
                }
            };
            if changed && queued.insert(key) {
                pending.push_back(key);
            }
        }
    }
    Ok(())
}

/// Called only for functions already selected for canonical physical publication.
/// Compatibility dumps do not obtain execution admission from this check.
pub(super) fn reject_legacy(function: &MirFunction) -> Result<(), &'static str> {
    let (definitions, mut errors) = ssa::definitions(function);
    if let Err(mut found) = cfg::check_control_flow(function) {
        errors.append(&mut found);
    }
    if let Some((_, why)) = structural_failure(function, &errors) {
        return Err(why);
    }
    for row in function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
    {
        if matches!(row, MirInstruction::Compare { lhs, rhs, .. }
            if direct_null(function, &definitions, *lhs) || direct_null(function, &definitions, *rhs))
        {
            return Err("null-comparison/legacy-compare");
        }
    }
    Ok(())
}
