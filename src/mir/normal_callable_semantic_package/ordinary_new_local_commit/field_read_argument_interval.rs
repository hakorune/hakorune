//! Physical liveness between the original parent Normal and the argument use.
use super::*;
use crate::mir::instruction::OwnedArrayInvokeOperation;
use crate::mir::value_origin::AcyclicUnanimousValueOrigins;
use std::cell::OnceCell;
use std::collections::{BTreeSet, HashMap};
use crate::mir::verification::utils::{compute_dominators, DominatorTree};

/// Invocation-local geometry; borrowing the function prevents mutation while used.
/// Laziness keeps non-Field packets from paying for analysis they never consume.
pub(crate) struct IntervalViewV1<'f> {
    function: &'f MirFunction,
    analysis: OnceCell<IntervalAnalysisV1>,
}
struct IntervalAnalysisV1 {
    origins: AcyclicUnanimousValueOrigins,
    dominators: DominatorTree,
    predecessors: HashMap<BasicBlockId, Vec<BasicBlockId>>,
}
pub(crate) struct ReadIntervalV1<'v, 'f> {
    view: &'v IntervalViewV1<'f>,
    parent: ValueId,
    children: BTreeSet<ValueId>,
    field: hakorune_mir_defs::CanonicalFieldRefV1,
}
impl<'f> IntervalViewV1<'f> {
    pub(crate) fn new(function: &'f MirFunction) -> Self {
        Self { function, analysis: OnceCell::new() }
    }
    pub(crate) fn function(&self) -> &'f MirFunction { self.function }
    fn analysis(&self) -> &IntervalAnalysisV1 {
        self.analysis.get_or_init(|| {
            // Preserve the interval owner's terminator edge authority, including
            // malformed inputs whose stored successor list differs.
            let mut predecessors: HashMap<_, Vec<_>> = HashMap::new();
            for (id, block) in &self.function.blocks {
                for next in block.successors_from_terminator() {
                    predecessors.entry(next).or_default().push(*id);
                }
            }
            IntervalAnalysisV1 { origins: AcyclicUnanimousValueOrigins::new(self.function),
                dominators: compute_dominators(self.function), predecessors }
        })
    }
    pub(crate) fn require_before(&self, before: &Binding, after: &Binding) -> Result<(), String> {
        let (before, after) = (locate(self.function, before)?, locate(self.function, after)?);
        let dom = &self.analysis().dominators;
        if !dom.is_reachable(before.0) || !dom.is_reachable(after.0)
            || !dom.dominates(before.0, after.0) || (before.0 == after.0 && before.1 >= after.1) {
            return Err(freeze("field-argument/evaluation-order"));
        }
        Ok(())
    }
    pub(crate) fn read(&self, parent: ValueId, child: ValueId,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
    ) -> Result<ReadIntervalV1<'_, 'f>, String> {
        let origins = &self.analysis().origins;
        let parent = origins.origin(parent).ok_or_else(|| freeze("field-argument/parent-origin"))?;
        let child = origins.origin(child).ok_or_else(|| freeze("field-argument/child-origin"))?;
        // A repeated unchanged slot denotes the same retained child, even when
        // its fresh SSA destination or unknown base cannot prove non-aliasing.
        let mut children = BTreeSet::from([child]);
        for instruction in self.function.blocks.values().flat_map(|block| block.all_instructions()) {
            if let MirInstruction::ObjectFieldGet { dst, base, field: read_field } = instruction {
                if *read_field == field && origins.origin(*base).is_none_or(|origin| origin == parent) {
                    let origin = origins.origin(*dst).ok_or_else(|| freeze("field-argument/repeated-read-origin"))?;
                    children.insert(origin);
                }
            }
        }
        Ok(ReadIntervalV1 { view: self, parent, children, field })
    }
}

pub(super) fn locate(function: &MirFunction, binding: &Binding) -> Result<(BasicBlockId, usize), String> {
    let block = function.blocks.get(&binding.0).ok_or_else(|| freeze("field-argument/block-missing"))?;
    let mut found = block.all_instructions().enumerate().filter(|(_, row)| *row == &binding.1);
    let (position, _) = found.next().ok_or_else(|| freeze("field-argument/producer-missing"))?;
    if found.next().is_some() { return Err(freeze("field-argument/producer-duplicate")); }
    Ok((binding.0, position))
}
#[cfg(test)]
pub(super) fn check(
    function: &MirFunction, start: &Binding, finish: &Binding,
    parent: ValueId, child: ValueId, field: hakorune_mir_defs::CanonicalFieldRefV1,
) -> Result<(), String> {
    IntervalViewV1::new(function).read(parent, child, field)?.check(start, finish)
}
impl ReadIntervalV1<'_, '_> {
    pub(crate) fn check(&self, start: &Binding, finish: &Binding) -> Result<(), String> {
        let function = self.view.function;
        let start = locate(function, start)?;
        let finish = locate(function, finish)?;
        let analysis = self.view.analysis();
        let origins = &analysis.origins;
        let (parent, children, field) = (self.parent, &self.children, self.field);
        // Unknown origins cannot prove non-aliasing of a destructive operand.
        let aliases = |value| -> Result<bool, String> {
            let origin = origins.origin(value).ok_or_else(|| freeze("field-argument/destructive-origin"))?;
            Ok(origin == parent || children.contains(&origin))
        };
        let parent_alias = |value| -> Result<bool, String> {
            Ok(origins.origin(value).ok_or_else(|| freeze("field-argument/mutation-origin"))? == parent)
        };
        // Restrict to paths reaching the use. A terminating Fault unwind does not
        // invalidate the Normal path, but a cleanup edge that rejoins it does.
        let mut reaching = BTreeSet::from([finish.0]);
        let mut reverse_pending = vec![finish.0];
        while let Some(id) = reverse_pending.pop() {
            if let Some(predecessors) = analysis.predecessors.get(&id) {
                for predecessor in predecessors {
                    if reaching.insert(*predecessor) { reverse_pending.push(*predecessor); }
                }
            }
        }
        let mut pending = vec![(start.0, start.1 + 1)];
        let mut visited = BTreeSet::new();
        while let Some((id, from)) = pending.pop() {
            if !reaching.contains(&id) || !visited.insert((id, from)) { continue; }
            let block = function.blocks.get(&id).ok_or_else(|| freeze("field-argument/interval-block"))?;
            for (position, instruction) in block.all_instructions().enumerate().skip(from) {
                if (id, position) == finish { break; }
                let invalid = match instruction {
                    MirInstruction::Invoke { operation, .. } => match operation {
                        InvokeOperation::HomeRelease { value, .. }
                        | InvokeOperation::HomeReleaseIfLive { value, .. }
                        | InvokeOperation::ObjectEndIfOwned { value, .. }
                        | InvokeOperation::ReclaimUnpublished { value, .. } => aliases(*value)?,
                        InvokeOperation::OwnedObjectFieldRelease { base, .. }
                        | InvokeOperation::OwnedFieldResidenceRelease { base, .. } => aliases(*base)?,
                        InvokeOperation::FieldSet { base, field: written, .. } => *written == field && parent_alias(*base)?,
                        InvokeOperation::ObjectFieldSet { base, field: written, value, .. } =>
                            (*written == field && parent_alias(*base)?) || aliases(*value)?,
                        InvokeOperation::OwnedArray(OwnedArrayInvokeOperation::Append { candidate, .. }) => aliases(*candidate)?,
                        _ => false,
                    },
                    MirInstruction::DestroyOwned { value } => aliases(*value)?,
                    MirInstruction::ReleaseStrong { values } => {
                        let mut invalid = false;
                        for value in values { invalid |= aliases(*value)?; }
                        invalid
                    }
                    // Compatibility writes cannot establish a canonical field identity.
                    MirInstruction::FieldSet { base, .. } => parent_alias(*base)?,
                    _ => false,
                };
                if invalid { return Err(freeze("field-argument/parent-or-child-invalidated")); }
            }
            if id != finish.0 {
                pending.extend(block.successors_from_terminator().into_iter().map(|next| (next, 0)));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType};
    use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};

    #[test]
    fn field_argument_interval_rejects_early_end_mutation_and_transfer_but_allows_terminating_fault() {
        let object = CanonicalObjectIdV1::from_declaration_index(0).unwrap();
        let field = CanonicalFieldRefV1::from_declaration_ordinal(object, 0).unwrap();
        let other = CanonicalFieldRefV1::from_declaration_ordinal(object, 1).unwrap();
        let mut function = MirFunction::new(FunctionSignature { name: "field-interval".into(),
            params: vec![], return_type: MirType::Void, effects: EffectMask::CONTROL }, BasicBlockId(0));
        function.params = vec![ValueId(3), ValueId(4)];
        let start = (BasicBlockId(0), MirInstruction::Const { dst: ValueId(1), value: ConstValue::Null });
        let read = MirInstruction::ObjectFieldGet { dst: ValueId(2), base: ValueId(1), field };
        let finish = (BasicBlockId(1), MirInstruction::Const { dst: ValueId(9), value: ConstValue::Integer(0) });
        let mut entry = BasicBlock::new(BasicBlockId(0));
        entry.add_instruction(start.1.clone());
        entry.add_instruction(read);
        entry.add_instruction(MirInstruction::Copy { dst: ValueId(5), src: ValueId(2) });
        entry.add_instruction(MirInstruction::ObjectFieldGet { dst: ValueId(6), base: ValueId(1), field });
        // Releasing an unrelated object on Normal is fine. Parent cleanup on
        // a terminating Fault path must not poison the surviving argument use.
        entry.set_terminator(MirInstruction::Invoke {
            operation: InvokeOperation::HomeRelease { object, value: ValueId(3) },
            fault_frame: ValueId(4), normal_landing: BasicBlockId(1), fault_landing: BasicBlockId(2) });
        let mut normal = BasicBlock::new(BasicBlockId(1));
        normal.add_instruction(finish.1.clone());
        normal.set_terminator(MirInstruction::Return { value: None });
        let mut fault = BasicBlock::new(BasicBlockId(2));
        fault.add_instruction(MirInstruction::DestroyOwned { value: ValueId(1) });
        fault.set_terminator(MirInstruction::ReturnFault { fault_frame: ValueId(4) });
        for block in [entry, normal, fault] { function.add_block(block); }
        let verify = |function: &MirFunction| check(function, &start, &finish, ValueId(1), ValueId(2), field);
        verify(&function).unwrap();
        for operation in [
            InvokeOperation::HomeRelease { object, value: ValueId(1) },
            InvokeOperation::HomeReleaseIfLive { object, value: ValueId(5) },
            InvokeOperation::ReclaimUnpublished { object, value: ValueId(6) },
            InvokeOperation::OwnedObjectFieldRelease { base: ValueId(1), field: other, child: object },
            InvokeOperation::FieldSet { base: ValueId(1), field, value: ValueId(3) },
            InvokeOperation::ObjectFieldSet { base: ValueId(3), field: other, value: ValueId(2), child: object },
        ] {
            let mut changed = function.clone();
            let Some(MirInstruction::Invoke { operation: actual, .. }) =
                &mut changed.blocks.get_mut(&BasicBlockId(0)).unwrap().terminator else { unreachable!() };
            *actual = operation;
            assert!(verify(&changed).is_err());
        }
        for instruction in [
            MirInstruction::DestroyOwned { value: ValueId(5) },
            MirInstruction::ReleaseStrong { values: vec![ValueId(6)] },
            MirInstruction::DestroyOwned { value: ValueId(99) },
        ] {
            let mut changed = function.clone();
            changed.blocks.get_mut(&BasicBlockId(0)).unwrap().add_instruction(instruction);
            assert!(verify(&changed).is_err());
        }
        // A valid non-unanimous Phi cannot prove that a reread's base is a
        // different parent. Its fresh SSA destination is a potential child.
        let mut ambiguous = function.clone();
        ambiguous.blocks.get_mut(&BasicBlockId(0)).unwrap().set_terminator(
            MirInstruction::Branch { condition: ValueId(3), then_bb: BasicBlockId(3),
                else_bb: BasicBlockId(4), then_edge_args: None, else_edge_args: None });
        for id in [3, 4] {
            let mut branch = BasicBlock::new(BasicBlockId(id));
            branch.set_terminator(MirInstruction::Jump { target: BasicBlockId(5), edge_args: None });
            ambiguous.add_block(branch);
        }
        let mut join = BasicBlock::new(BasicBlockId(5));
        join.add_instruction(MirInstruction::Phi { dst: ValueId(7),
            inputs: vec![(BasicBlockId(3), ValueId(1)), (BasicBlockId(4), ValueId(3))],
            type_hint: None });
        join.add_instruction(MirInstruction::ObjectFieldGet { dst: ValueId(8), base: ValueId(7), field });
        join.add_instruction(MirInstruction::DestroyOwned { value: ValueId(8) });
        join.set_terminator(MirInstruction::Jump { target: BasicBlockId(1), edge_args: None });
        ambiguous.add_block(join);
        assert!(AcyclicUnanimousValueOrigins::new(&ambiguous).origin(ValueId(7)).is_none());
        assert!(verify(&ambiguous).is_err(), "unknown base of same-slot reread cannot prove non-aliasing");
        let mut rejoins = function.clone();
        rejoins.blocks.get_mut(&BasicBlockId(2)).unwrap().set_terminator(
            MirInstruction::Jump { target: BasicBlockId(1), edge_args: None });
        assert!(verify(&rejoins).is_err(), "Fault cleanup that rejoins the use is not terminating");
    }
}
