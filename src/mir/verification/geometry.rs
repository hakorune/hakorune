//! Analysis borrowed only while this exact immutable function is verified.
//! This is not a proof of domain/ownership validity, nor a cross-epoch cache.
use super::{cfg, dom, ssa, utils};
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
    ssa_errors: OnceCell<Vec<VerificationError>>,
    dominance_errors: OnceCell<Vec<VerificationError>>,
}
enum Facts<'a> {
    Owned(Box<FunctionVerificationFacts>),
    Borrowed(&'a FunctionVerificationFacts),
}
pub(in crate::mir) struct FunctionVerificationGeometry<'a> {
    function: &'a MirFunction,
    facts: Facts<'a>,
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
            facts: Facts::Owned(Box::default()),
        }
    }
    pub(in crate::mir) fn borrow(
        function: &'a MirFunction,
        facts: &'a FunctionVerificationFacts,
    ) -> Self {
        Self {
            function,
            facts: Facts::Borrowed(facts),
        }
    }
    fn facts(&self) -> &FunctionVerificationFacts {
        match &self.facts {
            Facts::Owned(facts) => facts,
            Facts::Borrowed(facts) => facts,
        }
    }
    pub(in crate::mir) fn definitions(&self) -> &ValueDefMap {
        &self.definition_image().0
    }
    fn definition_image(&self) -> &Definitions {
        self.facts()
            .definitions
            .get_or_init(|| crate::mir::value_origin::indexed_value_definitions(self.function))
    }
    pub(in crate::mir) fn conflicts(&self) -> &[(ValueId, BasicBlockId, BasicBlockId)] {
        &self.definition_image().1
    }
    /// Original instruction-only uniqueness, distinct from parameter collisions.
    pub(in crate::mir) fn instruction_definition_count(&self, value: ValueId) -> usize {
        if !self.definitions().contains_key(&value) {
            return 0;
        }
        1 + self
            .conflicts()
            .iter()
            .filter(|(old, _, _)| *old == value)
            .count()
            - self
                .function
                .params
                .iter()
                .filter(|old| **old == value)
                .count()
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
        self.facts()
            .predecessors
            .get_or_init(|| utils::compute_predecessors(self.function))
    }
    pub(super) fn actual_predecessors(&self) -> &BTreeMap<BasicBlockId, Vec<BasicBlockId>> {
        self.facts().actual_predecessors.get_or_init(|| {
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
        self.facts().dominators.get_or_init(|| {
            utils::compute_dominators_with_predecessors(self.function, self.predecessors())
        })
    }
    pub(in crate::mir) fn reachable(&self) -> &HashSet<BasicBlockId> {
        self.facts()
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
    pub(in crate::mir) fn ssa_errors(&self) -> &[VerificationError] {
        self.facts().ssa_errors.get_or_init(|| {
            let mut errors = self.definition_errors();
            ssa::check_uses(self.function, self.definitions(), &mut errors);
            errors
        })
    }
    pub(in crate::mir) fn dominance_errors(&self) -> &[VerificationError] {
        self.facts().dominance_errors.get_or_init(|| {
            dom::check_dominance_by(self.function, self.dominators(), |v| {
                self.definition_block(v)
            })
            .err()
            .unwrap_or_default()
        })
    }
    pub(super) fn check_merge_uses(&self) -> Result<(), Vec<VerificationError>> {
        cfg::check_merge_uses_by(self.function, self.predecessors(), self.dominators(), |v| {
            self.definition_block(v)
        })
    }
}
