//! Analysis borrowed only while this exact immutable function is verified.
//! This is not a proof of domain/ownership validity, nor a cross-epoch cache.
use super::{uses, utils};
use crate::mir::value_origin::ValueDefMap;
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::cell::OnceCell;
use std::collections::{BTreeMap, HashMap, HashSet};

type Definitions = (ValueDefMap, Vec<(ValueId, BasicBlockId, BasicBlockId)>);
pub(super) type ModuleGeometry<'a> = HashMap<*const MirFunction, FunctionVerificationGeometry<'a>>;

#[derive(Debug, Default)]
pub(in crate::mir) struct FunctionVerificationFacts {
    definitions: OnceCell<Definitions>,
    predecessors: OnceCell<HashMap<BasicBlockId, Vec<BasicBlockId>>>,
    actual_predecessors: OnceCell<BTreeMap<BasicBlockId, Vec<BasicBlockId>>>,
    dominators: OnceCell<utils::DominatorTree>,
    reachable: OnceCell<HashSet<BasicBlockId>>,
    use_errors: OnceCell<uses::FunctionUseErrors>,
}
pub(in crate::mir) struct FunctionVerificationGeometry<'a> {
    function: &'a MirFunction,
    facts: std::rc::Rc<FunctionVerificationFacts>,
}
impl<'a> std::ops::Deref for FunctionVerificationGeometry<'a> {
    type Target = MirFunction;
    fn deref(&self) -> &MirFunction {
        self.function
    }
}
impl<'a> FunctionVerificationGeometry<'a> {
    pub(in crate::mir) fn new(function: &'a MirFunction) -> Self {
        Self {
            function,
            facts: std::rc::Rc::default(),
        }
    }
    pub(in crate::mir) fn borrow(
        function: &'a MirFunction,
        facts: &std::rc::Rc<FunctionVerificationFacts>,
    ) -> Self {
        Self {
            function,
            facts: std::rc::Rc::clone(facts),
        }
    }
    pub(in crate::mir) fn definitions(&self) -> &ValueDefMap {
        &self.definition_image().0
    }
    fn definition_image(&self) -> &Definitions {
        self.facts
            .definitions
            .get_or_init(|| crate::mir::value_origin::indexed_value_definitions(self.function))
    }
    pub(in crate::mir) fn conflicts(&self) -> &[(ValueId, BasicBlockId, BasicBlockId)] {
        &self.definition_image().1
    }
    pub(in crate::mir) fn definition_count(&self, value: ValueId) -> usize {
        usize::from(self.definitions().contains_key(&value))
            + self
                .conflicts()
                .iter()
                .filter(|(old, _, _)| *old == value)
                .count()
    }
    /// Exact instruction identity; callers retain their parameter-collision policy.
    pub(in crate::mir) fn matches_exact_definition(
        &self,
        block: BasicBlockId,
        row: &MirInstruction,
        forbid_parameters: bool,
    ) -> bool {
        row.dst_value().is_some_and(|value| {
            self.instruction_definition_count(value) == 1
                && (!forbid_parameters || !self.params.contains(&value))
                && self
                    .definition(value)
                    .is_some_and(|(actual, _, instruction)| actual == block && instruction == row)
        })
    }
    /// Original instruction-only uniqueness, distinct from parameter collisions.
    pub(in crate::mir) fn instruction_definition_count(&self, value: ValueId) -> usize {
        self.definition_count(value)
            - self
                .function
                .params
                .iter()
                .filter(|old| **old == value)
                .count()
    }
    pub(in crate::mir) fn parameters_have_unique_definitions(&self) -> bool {
        self.conflicts()
            .iter()
            .all(|(value, _, _)| !self.params.contains(value))
    }
    pub(in crate::mir) fn ambiguous(&self, value: ValueId) -> bool {
        self.conflicts().iter().any(|(old, _, _)| *old == value)
    }
    pub(in crate::mir) fn definition(
        &self,
        value: ValueId,
    ) -> Option<(BasicBlockId, usize, &'a MirInstruction)> {
        let &(block, index) = self.definitions().get(&value)?;
        let body = self.function.blocks.get(&block)?;
        let row = body.instructions.get(index).or_else(|| {
            (index == body.instructions.len())
                .then_some(body.terminator.as_ref())
                .flatten()
        })?;
        Some((block, index, row))
    }
    pub(in crate::mir) fn definition_block(&self, value: ValueId) -> Option<BasicBlockId> {
        self.definitions().get(&value).map(|(block, _)| *block)
    }
    pub(in crate::mir) fn predecessors(&self) -> &HashMap<BasicBlockId, Vec<BasicBlockId>> {
        self.facts
            .predecessors
            .get_or_init(|| utils::compute_predecessors(self.function))
    }
    pub(super) fn actual_predecessors(&self) -> &BTreeMap<BasicBlockId, Vec<BasicBlockId>> {
        self.facts.actual_predecessors.get_or_init(|| {
            let mut result = BTreeMap::<_, Vec<_>>::new();
            for (id, block) in &self.function.blocks {
                for target in block.successors_from_terminator() {
                    result.entry(target).or_default().push(*id);
                }
            }
            result
        })
    }
    pub(in crate::mir) fn dominators(&self) -> &utils::DominatorTree {
        self.facts.dominators.get_or_init(|| {
            utils::compute_dominators_with_predecessors(self.function, self.predecessors())
        })
    }
    pub(in crate::mir) fn reachable(&self) -> &HashSet<BasicBlockId> {
        self.facts
            .reachable
            .get_or_init(|| utils::compute_reachable_blocks(self.function))
    }
    pub(in crate::mir) fn definition_errors(&self) -> Vec<VerificationError> {
        self.conflicts()
            .iter()
            .map(
                |&(value, first_block, second_block)| VerificationError::MultipleDefinition {
                    value,
                    first_block,
                    second_block,
                },
            )
            .collect()
    }
    fn use_errors(&self) -> &uses::FunctionUseErrors {
        self.facts.use_errors.get_or_init(|| {
            uses::check(
                self.function,
                self.definitions(),
                self.definition_errors(),
                self.dominators(),
                self.predecessors(),
            )
        })
    }
    pub(in crate::mir) fn ssa_errors(&self) -> &[VerificationError] {
        &self.use_errors().ssa
    }
    pub(in crate::mir) fn dominance_errors(&self) -> &[VerificationError] {
        &self.use_errors().dominance
    }
    pub(super) fn merge_errors(&self) -> &[VerificationError] {
        &self.use_errors().merge
    }
}
