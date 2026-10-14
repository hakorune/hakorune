//! Independent physical holder/candidate lifetimes. Source admission remains
//! on the original Recipe; a borrowed Get never creates a caller-owned lease.
//! This checks affine outer resources, not child teardown completeness: the
//! existing source/root cleanup correspondence must still prove complete End.
use crate::mir::instruction::{
    InvokeCallResultKind, InvokeOperation, OwnedArrayInvokeOperation as Op,
    OwnedArraySlotDescriptorV1 as Descriptor,
};
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{BasicBlockId, ConstValue, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct State {
    outer: BTreeSet<ValueId>,
    holders: BTreeMap<ValueId, Descriptor>,
    candidates: BTreeSet<ValueId>,
}

struct Inventory<'a> {
    function: &'a MirFunction,
    defs: ValueDefMap,
    arrays: BTreeSet<ValueId>,
    candidates: BTreeMap<ValueId, BTreeSet<ValueId>>,
    candidate_objects: BTreeMap<ValueId, hakorune_mir_defs::CanonicalObjectIdV1>,
    borrows: BTreeMap<ValueId, (ValueId, Descriptor)>,
    results: BTreeMap<BasicBlockId, ValueId>,
}
impl<'a> Inventory<'a> {
    fn origin(&self, value: ValueId) -> ValueId {
        resolve_value_origin(self.function, &self.defs, value)
    }
    fn definition(&self, value: ValueId) -> Option<&'a MirInstruction> {
        let (block, row) = self.defs.get(&value)?;
        self.function.blocks.get(block)?.instructions.get(*row)
    }
    fn producer(&self, value: ValueId) -> Option<&'a InvokeOperation> {
        let MirInstruction::InvokeNormalResult { invoke_block, .. } = self.definition(value)?
        else {
            return None;
        };
        let MirInstruction::Invoke { operation, .. } = self
            .function
            .blocks
            .get(invoke_block)?
            .terminator
            .as_ref()?
        else {
            return None;
        };
        Some(operation)
    }
    fn null(&self, value: ValueId) -> bool {
        matches!(
            self.definition(value),
            Some(MirInstruction::Const {
                value: ConstValue::Null,
                ..
            })
        )
    }
    fn issue(function: &'a MirFunction) -> Result<Option<Self>, &'static str> {
        if !function.blocks.values().any(|b| {
            b.all_instructions().any(|i| {
                matches!(
                    i,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::OwnedArray(_),
                        ..
                    }
                )
            })
        }) {
            return Ok(None);
        }
        let mut row = Self {
            function,
            defs: build_value_def_map(function),
            arrays: BTreeSet::new(),
            candidates: BTreeMap::new(),
            candidate_objects: BTreeMap::new(),
            borrows: BTreeMap::new(),
            results: BTreeMap::new(),
        };
        for block in function.blocks.values() {
            for instruction in &block.instructions {
                if let MirInstruction::InvokeNormalResult { invoke_block, dst } = instruction {
                    if row.results.insert(*invoke_block, *dst).is_some() {
                        return Err("owned-array-duplicate-result");
                    }
                }
            }
        }
        for block in function.blocks.values() {
            let Some(MirInstruction::Invoke {
                operation: InvokeOperation::OwnedArray(operation),
                ..
            }) = block.terminator.as_ref()
            else {
                continue;
            };
            let array = row.origin(operation.array());
            if !matches!(
                row.producer(array),
                Some(InvokeOperation::IntrinsicArrayNew)
            ) {
                return Err("owned-array-original-allocation");
            }
            row.arrays.insert(array);
            match operation {
                Op::Get {
                    descriptor, index, ..
                } => {
                    // This original append-only source cohort has literal
                    // integer indices. Copies preserve that producer; a
                    // handle's physical word cannot serve as an index.
                    if !matches!(
                        row.definition(row.origin(*index)),
                        Some(MirInstruction::Const {
                            value: ConstValue::Integer(_),
                            ..
                        })
                    ) {
                        return Err("owned-array-index-source");
                    }
                    let dst = *row
                        .results
                        .get(&block.id)
                        .ok_or("owned-array-get-result-missing")?;
                    row.borrows.insert(dst, (array, *descriptor));
                }
                Op::Append {
                    candidate,
                    descriptor,
                    ..
                } => {
                    let candidate = row.origin(*candidate);
                    if row.null(candidate) {
                        if !descriptor.nullable() {
                            return Err("owned-array-null-candidate");
                        }
                    } else {
                        match row.producer(candidate) {
                            Some(InvokeOperation::NewBox { object })
                                if *object == descriptor.object() => {}
                            Some(InvokeOperation::Call {
                                result: InvokeCallResultKind::Handle,
                                ..
                            }) => {}
                            Some(InvokeOperation::Call {
                                result: InvokeCallResultKind::NullableHandle,
                                ..
                            }) if descriptor.nullable() => {}
                            _ => return Err("owned-array-candidate-producer"),
                        }
                        if row
                            .candidate_objects
                            .insert(candidate, descriptor.object())
                            .is_some_and(|object| object != descriptor.object())
                        {
                            return Err("owned-array-candidate-descriptor-drift");
                        }
                        row.candidates.entry(candidate).or_default().insert(array);
                    }
                }
                Op::Claim { .. } | Op::End { .. } => {}
            }
        }
        // Copy never changes a Get's holder dependence. A Get cannot become
        // an owned candidate merely by acquiring an equal-looking Box type.
        if row.borrows.keys().any(|v| row.candidates.contains_key(v)) {
            return Err("owned-array-borrow-transfer");
        }
        Ok(Some(row))
    }
    fn borrow_use(&self, instruction: &MirInstruction, value: ValueId) -> bool {
        match instruction {
            MirInstruction::Copy { src, .. } => *src == value,
            MirInstruction::Invoke {
                operation: InvokeOperation::Call { call, .. },
                ..
            }
            | MirInstruction::Call(call) => {
                // Structural corroboration only. Module verification below
                // additionally demands the exact original Call argument slot.
                super::cataloged_edge_key(call).is_some() && call.args.contains(&value) && {
                    let mut receiver = false;
                    call.callee
                        .for_each_value_operand(|v| receiver |= v == value);
                    !receiver
                }
            }
            _ => false,
        }
    }
    fn uses(
        &self,
        instruction: &MirInstruction,
        state: &State,
        faulted: bool,
    ) -> Result<(), &'static str> {
        for value in instruction.used_values() {
            let root = self.origin(value);
            if let Some((holder, descriptor)) = self.borrows.get(&root) {
                if faulted {
                    return Err("owned-array-borrow-use-after-fault");
                }
                if state.holders.get(holder) != Some(descriptor) {
                    return Err("owned-array-borrow-not-live");
                }
                if !self.borrow_use(instruction, value) {
                    return Err("owned-array-borrow-escape");
                }
            }
            if self.arrays.contains(&root) {
                if !state.outer.contains(&root) && !state.holders.contains_key(&root) {
                    return Err("owned-array-holder-not-live");
                }
                let allowed = matches!(instruction, MirInstruction::Copy { src, .. } if *src == value)
                    || matches!(instruction, MirInstruction::Invoke { operation: InvokeOperation::OwnedArray(op), .. } if op.array() == value)
                    || matches!(instruction, MirInstruction::ArrayResidenceRelease { value: released } if *released == value && state.outer.contains(&root));
                if !allowed {
                    return Err("owned-array-holder-escape");
                }
            }
            if self.candidates.contains_key(&root) {
                if !state.candidates.contains(&root) {
                    return Err("owned-array-candidate-not-live");
                }
                let allowed = matches!(instruction, MirInstruction::Copy { src, .. } if *src == value)
                    || matches!(instruction, MirInstruction::Invoke { operation: InvokeOperation::OwnedArray(Op::Append { candidate, .. }), .. } if *candidate == value)
                    || matches!(instruction, MirInstruction::Invoke { operation:
                        InvokeOperation::HomeRelease { value: released, .. }
                        | InvokeOperation::HomeReleaseIfLive { value: released, .. }
                        | InvokeOperation::ReclaimUnpublished { value: released, .. }, .. } if *released == value)
                    || matches!(instruction, MirInstruction::Invoke { operation:
                        InvokeOperation::OwnedFieldResidenceRelease { base, .. }
                        | InvokeOperation::OwnedObjectFieldRelease { base, .. }, .. } if *base == value);
                if !allowed {
                    return Err("owned-array-candidate-escape");
                }
            }
        }
        Ok(())
    }
}

/// Exact original Call coordinates are lent before the existing verifier
/// consumes them. A cataloged callee or the new result tag is never authority.
pub(super) fn check_source_uses(
    function: &MirFunction,
    original_slot: impl Fn(BasicBlockId, usize, usize) -> bool,
) -> Result<(), &'static str> {
    let Some(row) = Inventory::issue(function)? else {
        return Ok(());
    };
    for (id, block) in &function.blocks {
        for (index, instruction) in block.all_instructions().enumerate() {
            let call = match instruction {
                MirInstruction::Call(call)
                | MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, .. },
                    ..
                } => call,
                _ => continue,
            };
            for (ordinal, argument) in call.args.iter().enumerate() {
                if row.borrows.contains_key(&row.origin(*argument))
                    && !original_slot(*id, index, ordinal)
                {
                    return Err("owned-array-original-borrow-slot-missing");
                }
            }
        }
    }
    Ok(())
}

pub(super) fn check(function: &MirFunction) -> Result<(), &'static str> {
    let Some(row) = Inventory::issue(function)? else {
        return Ok(());
    };
    let mut pending = vec![(function.entry_block, State::default(), false)];
    let mut seen = BTreeMap::new();
    while let Some((id, mut state, faulted)) = pending.pop() {
        if let Some(previous) = seen.get(&(id, faulted)) {
            if previous != &state {
                return Err("owned-array-lifetime-join");
            }
            continue;
        }
        seen.insert((id, faulted), state.clone());
        let block = function
            .blocks
            .get(&id)
            .ok_or("owned-array-block-missing")?;
        for instruction in &block.instructions {
            row.uses(instruction, &state, faulted)?;
            if let MirInstruction::ArrayResidenceRelease { value } = instruction {
                let root = row.origin(*value);
                if row.arrays.contains(&root) && !state.outer.remove(&root) {
                    return Err("owned-array-unclaimed-release");
                }
            }
        }
        if block.return_env.as_ref().is_some_and(|env| {
            env.iter().any(|v| {
                let v = row.origin(*v);
                row.arrays.contains(&v)
                    || row.candidates.contains_key(&v)
                    || row.borrows.contains_key(&v)
            })
        }) {
            return Err("owned-array-return-env-escape");
        }
        let term = block
            .terminator
            .as_ref()
            .ok_or("owned-array-terminator-missing")?;
        row.uses(term, &state, faulted)?;
        let mut normal = state.clone();
        if let MirInstruction::Invoke { operation, .. } = term {
            if let InvokeOperation::OwnedArray(operation) = operation {
                let array = row.origin(operation.array());
                let descriptor = operation.descriptor();
                if faulted && !matches!(operation, Op::End { .. }) {
                    return Err("owned-array-effect-after-fault");
                }
                match operation {
                    Op::Claim { .. } => {
                        if !normal.outer.remove(&array)
                            || normal.holders.insert(array, descriptor).is_some()
                        {
                            return Err("owned-array-duplicate-or-unacquired-claim");
                        }
                    }
                    Op::Append { candidate, .. } => {
                        if state.holders.get(&array) != Some(&descriptor) {
                            return Err("owned-array-descriptor-or-holder-drift");
                        }
                        let candidate = row.origin(*candidate);
                        if !row.null(candidate) && !normal.candidates.remove(&candidate) {
                            return Err("owned-array-candidate-not-live");
                        }
                    }
                    Op::Get { .. } => {
                        if state.holders.get(&array) != Some(&descriptor) {
                            return Err("owned-array-descriptor-or-holder-drift");
                        }
                    }
                    Op::End { .. } => {
                        if state.holders.get(&array) != Some(&descriptor) {
                            return Err("owned-array-descriptor-or-holder-drift");
                        }
                        if state
                            .candidates
                            .iter()
                            .any(|v| row.candidates[v].contains(&array))
                        {
                            return Err("owned-array-candidate-before-holder-end");
                        }
                        normal.holders.remove(&array);
                        state.holders.remove(&array);
                    }
                }
            }
            if let Some(result) = row.results.get(&id) {
                if row.arrays.contains(result) {
                    if faulted || !normal.outer.insert(*result) {
                        return Err("owned-array-allocation-not-fresh");
                    }
                }
                if row.candidates.contains_key(result) {
                    if faulted || !normal.candidates.insert(*result) {
                        return Err("owned-array-candidate-not-fresh");
                    }
                }
            }
            if let InvokeOperation::HomeRelease { value, object }
            | InvokeOperation::HomeReleaseIfLive { value, object }
            | InvokeOperation::ReclaimUnpublished { value, object } = operation
            {
                let candidate = row.origin(*value);
                if row.candidates.contains_key(&candidate) {
                    if row.candidate_objects.get(&candidate) != Some(object)
                        || matches!(operation, InvokeOperation::ReclaimUnpublished { .. })
                            && !matches!(
                                row.producer(candidate),
                                Some(InvokeOperation::NewBox { .. })
                            )
                        || matches!(
                            row.producer(candidate),
                            Some(InvokeOperation::Call {
                                result: InvokeCallResultKind::NullableHandle,
                                ..
                            })
                        ) && !matches!(operation, InvokeOperation::HomeReleaseIfLive { .. })
                    {
                        return Err("owned-array-candidate-cleanup-drift");
                    }
                    state.candidates.remove(&candidate);
                    normal.candidates.remove(&candidate);
                }
            }
        }
        let targets = block.successors_from_terminator();
        if targets.is_empty()
            && (!state.outer.is_empty()
                || !state.holders.is_empty()
                || !state.candidates.is_empty())
        {
            return Err("owned-array-missing-end");
        }
        for target in targets.into_iter().rev() {
            let (next, next_faulted) = match term {
                MirInstruction::Invoke { normal_landing, .. } if target == *normal_landing => {
                    (normal.clone(), faulted)
                }
                MirInstruction::Invoke { fault_landing, .. } if target == *fault_landing => {
                    (state.clone(), true)
                }
                _ => (state.clone(), faulted),
            };
            pending.push((target, next, next_faulted));
        }
    }
    // This original append-only holder cohort has no loop-carried state.
    // A fixed-point revisit must not let a live holder cycle forever.
    let mut active = BTreeSet::new();
    let mut complete = BTreeSet::new();
    let mut stack = vec![(function.entry_block, false)];
    while let Some((id, exit)) = stack.pop() {
        if exit {
            active.remove(&id);
            complete.insert(id);
            continue;
        }
        if complete.contains(&id) {
            continue;
        }
        if !active.insert(id) {
            return Err("owned-array-lifetime-cycle");
        }
        stack.push((id, true));
        for next in function
            .blocks
            .get(&id)
            .ok_or("owned-array-block-missing")?
            .successors_from_terminator()
            .into_iter()
            .rev()
        {
            stack.push((next, false));
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "invoke_owned_array_tests.rs"]
mod tests;
