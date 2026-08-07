//! Builder-free physical segment layout derived from the verified Recipe.
//!
//! This is a private compatibility product.  `LoopRecipeV1` and
//! `LoopJoinSigV1` remain the logical authorities; this module only derives
//! ordered segments, item placement, and nested-loop resume targets.

use std::collections::{BTreeMap, BTreeSet};

use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::operation_physical_demand::PreparedLoopOperationProgramV1;
use super::schema::{LoopConditionV1, LoopRecipeItemV1, LoopRecipeV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct LoopPhysicalSegmentKeyV1 {
    loop_key: LoopNodeKeyV1,
    block: LoopBlockKeyV1,
    ordinal: u32,
}

impl LoopPhysicalSegmentKeyV1 {
    const fn new(loop_key: LoopNodeKeyV1, block: LoopBlockKeyV1, ordinal: u32) -> Self {
        Self {
            loop_key,
            block,
            ordinal,
        }
    }

    pub(crate) const fn loop_key(self) -> LoopNodeKeyV1 {
        self.loop_key
    }

    pub(crate) const fn block(self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(crate) const fn ordinal(self) -> u32 {
        self.ordinal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPhysicalTargetV1 {
    Segment(LoopPhysicalSegmentKeyV1),
    OpenRootAfter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPhysicalTransferV1 {
    Jump {
        target: LoopPhysicalTargetV1,
    },
    Predicate {
        condition: LoopValueKeyV1,
        on_true: LoopPhysicalSegmentKeyV1,
        on_false: LoopPhysicalTargetV1,
    },
    OpenNestedLoop {
        loop_key: LoopNodeKeyV1,
        entry: LoopPhysicalSegmentKeyV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedLoopControlSegmentV1 {
    key: LoopPhysicalSegmentKeyV1,
    operations: Box<[LoopItemKeyV1]>,
    transfer: LoopPhysicalTransferV1,
}

impl PreparedLoopControlSegmentV1 {
    pub(crate) const fn key(&self) -> LoopPhysicalSegmentKeyV1 {
        self.key
    }

    pub(crate) fn operations(&self) -> &[LoopItemKeyV1] {
        &self.operations
    }

    pub(crate) const fn transfer(&self) -> LoopPhysicalTransferV1 {
        self.transfer
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopPhysicalLayoutCoverageReceiptV1 {
    item_count: usize,
    operation_count: usize,
    segment_count: usize,
}

impl LoopPhysicalLayoutCoverageReceiptV1 {
    pub(crate) const fn item_count(self) -> usize {
        self.item_count
    }

    pub(crate) const fn operation_count(self) -> usize {
        self.operation_count
    }

    pub(crate) const fn segment_count(self) -> usize {
        self.segment_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopPhysicalLayoutRejectV1 {
    MissingLoop(LoopNodeKeyV1),
    MissingBlock(LoopBlockKeyV1),
    DuplicateLoop(LoopNodeKeyV1),
    DuplicateBlock(LoopBlockKeyV1),
    DuplicateItem(LoopItemKeyV1),
    UnsupportedAlways(LoopNodeKeyV1),
    UnsupportedIf(LoopItemKeyV1),
    UnsupportedExit(LoopItemKeyV1),
    ScheduleOrderMismatch {
        expected: Box<[LoopItemKeyV1]>,
        found: Box<[LoopItemKeyV1]>,
    },
    IncompleteCoverage {
        expected: usize,
        found: usize,
    },
}

#[derive(Debug)]
pub(crate) struct PreparedLoopPhysicalLayoutV1 {
    program: PreparedLoopOperationProgramV1,
    segments: Box<[PreparedLoopControlSegmentV1]>,
    coverage: LoopPhysicalLayoutCoverageReceiptV1,
}

impl PreparedLoopPhysicalLayoutV1 {
    pub(crate) fn from_program(
        program: PreparedLoopOperationProgramV1,
    ) -> Result<Self, LoopPhysicalLayoutRejectV1> {
        let recipe = program
            .demand()
            .operation_effect()
            .core()
            .recipe()
            .as_recipe();
        let (segments, visited_items, operation_items) = {
            let mut builder = LayoutBuilder::new(recipe);
            builder.build_loop(recipe.root_loop, LoopPhysicalTargetV1::OpenRootAfter)?;
            builder.finish()?
        };
        let schedule = program
            .schedule()
            .iter()
            .map(|row| row.item())
            .collect::<Vec<_>>();
        let derived = operation_items.iter().copied().collect::<Vec<_>>();
        if schedule != derived {
            return Err(LoopPhysicalLayoutRejectV1::ScheduleOrderMismatch {
                expected: derived.into_boxed_slice(),
                found: schedule.into_boxed_slice(),
            });
        }
        let expected_items = recipe
            .items
            .iter()
            .map(|row| row.key)
            .collect::<BTreeSet<_>>();
        if visited_items != expected_items {
            return Err(LoopPhysicalLayoutRejectV1::IncompleteCoverage {
                expected: recipe.items.len(),
                found: visited_items.len(),
            });
        }
        Ok(Self {
            coverage: LoopPhysicalLayoutCoverageReceiptV1 {
                item_count: visited_items.len(),
                operation_count: operation_items.len(),
                segment_count: segments.len(),
            },
            program,
            segments: segments.into_boxed_slice(),
        })
    }

    pub(crate) fn program(&self) -> &PreparedLoopOperationProgramV1 {
        &self.program
    }

    pub(crate) fn segments(&self) -> &[PreparedLoopControlSegmentV1] {
        &self.segments
    }

    pub(crate) const fn coverage(&self) -> LoopPhysicalLayoutCoverageReceiptV1 {
        self.coverage
    }
}

struct LayoutBuilder<'a> {
    recipe: &'a LoopRecipeV1,
    item_rows: BTreeMap<LoopItemKeyV1, LoopRecipeItemV1>,
    segments: Vec<PreparedLoopControlSegmentV1>,
    visited_loops: BTreeSet<LoopNodeKeyV1>,
    visited_blocks: BTreeSet<LoopBlockKeyV1>,
    visited_items: BTreeSet<LoopItemKeyV1>,
    operation_items: Vec<LoopItemKeyV1>,
}

impl<'a> LayoutBuilder<'a> {
    fn new(recipe: &'a LoopRecipeV1) -> Self {
        Self {
            recipe,
            item_rows: recipe
                .items
                .iter()
                .map(|row| (row.key, row.item.clone()))
                .collect(),
            segments: Vec::new(),
            visited_loops: BTreeSet::new(),
            visited_blocks: BTreeSet::new(),
            visited_items: BTreeSet::new(),
            operation_items: Vec::new(),
        }
    }

    fn finish(
        self,
    ) -> Result<
        (
            Vec<PreparedLoopControlSegmentV1>,
            BTreeSet<LoopItemKeyV1>,
            Vec<LoopItemKeyV1>,
        ),
        LoopPhysicalLayoutRejectV1,
    > {
        if self.visited_loops.len() != self.recipe.loops.len()
            || self.visited_blocks.len() != self.recipe.blocks.len()
            || self.visited_items.len() != self.recipe.items.len()
        {
            return Err(LoopPhysicalLayoutRejectV1::IncompleteCoverage {
                expected: self.recipe.items.len(),
                found: self.visited_items.len(),
            });
        }
        Ok((self.segments, self.visited_items, self.operation_items))
    }

    fn entry_key(
        &self,
        loop_key: LoopNodeKeyV1,
    ) -> Result<LoopPhysicalSegmentKeyV1, LoopPhysicalLayoutRejectV1> {
        let node = self
            .recipe
            .loops
            .iter()
            .find(|row| row.key == loop_key)
            .ok_or(LoopPhysicalLayoutRejectV1::MissingLoop(loop_key))?;
        let block = match node.condition {
            LoopConditionV1::Predicate { block, .. } => block,
            LoopConditionV1::Always => {
                return Err(LoopPhysicalLayoutRejectV1::UnsupportedAlways(loop_key));
            }
        };
        Ok(LoopPhysicalSegmentKeyV1::new(loop_key, block, 0))
    }

    fn build_loop(
        &mut self,
        loop_key: LoopNodeKeyV1,
        after_target: LoopPhysicalTargetV1,
    ) -> Result<(), LoopPhysicalLayoutRejectV1> {
        if !self.visited_loops.insert(loop_key) {
            return Err(LoopPhysicalLayoutRejectV1::DuplicateLoop(loop_key));
        }
        let node = self
            .recipe
            .loops
            .iter()
            .find(|row| row.key == loop_key)
            .ok_or(LoopPhysicalLayoutRejectV1::MissingLoop(loop_key))?;
        let (condition_block, condition_value) = match node.condition {
            LoopConditionV1::Predicate { block, value } => (block, value),
            LoopConditionV1::Always => {
                return Err(LoopPhysicalLayoutRejectV1::UnsupportedAlways(loop_key));
            }
        };
        let body_entry = LoopPhysicalSegmentKeyV1::new(loop_key, node.body, 0);
        self.build_block(
            loop_key,
            condition_block,
            LoopPhysicalTransferV1::Predicate {
                condition: condition_value,
                on_true: body_entry,
                on_false: after_target,
            },
        )?;
        let condition_entry = LoopPhysicalSegmentKeyV1::new(loop_key, condition_block, 0);
        self.build_block(
            loop_key,
            node.body,
            LoopPhysicalTransferV1::Jump {
                target: LoopPhysicalTargetV1::Segment(condition_entry),
            },
        )
    }

    fn build_block(
        &mut self,
        loop_key: LoopNodeKeyV1,
        block_key: LoopBlockKeyV1,
        finish_transfer: LoopPhysicalTransferV1,
    ) -> Result<(), LoopPhysicalLayoutRejectV1> {
        if !self.visited_blocks.insert(block_key) {
            return Err(LoopPhysicalLayoutRejectV1::DuplicateBlock(block_key));
        }
        let block = self
            .recipe
            .blocks
            .iter()
            .find(|row| row.key == block_key)
            .ok_or(LoopPhysicalLayoutRejectV1::MissingBlock(block_key))?;
        let items = block.items.clone();
        let mut ordinal = 0;
        let mut operations = Vec::new();
        for item in items {
            if !self.visited_items.insert(item) {
                return Err(LoopPhysicalLayoutRejectV1::DuplicateItem(item));
            }
            match self.item_rows.get(&item).cloned() {
                Some(LoopRecipeItemV1::Operation { .. }) => {
                    self.operation_items.push(item);
                    operations.push(item);
                }
                Some(LoopRecipeItemV1::Loop { loop_key: child }) => {
                    let current = LoopPhysicalSegmentKeyV1::new(loop_key, block_key, ordinal);
                    let resume = LoopPhysicalSegmentKeyV1::new(loop_key, block_key, ordinal + 1);
                    let child_entry = self.entry_key(child)?;
                    self.segments.push(PreparedLoopControlSegmentV1 {
                        key: current,
                        operations: operations.into_boxed_slice(),
                        transfer: LoopPhysicalTransferV1::OpenNestedLoop {
                            loop_key: child,
                            entry: child_entry,
                        },
                    });
                    operations = Vec::new();
                    ordinal += 1;
                    self.build_loop(child, LoopPhysicalTargetV1::Segment(resume))?;
                }
                Some(LoopRecipeItemV1::If { .. }) => {
                    return Err(LoopPhysicalLayoutRejectV1::UnsupportedIf(item));
                }
                Some(LoopRecipeItemV1::Exit { .. }) => {
                    return Err(LoopPhysicalLayoutRejectV1::UnsupportedExit(item));
                }
                None => {
                    return Err(LoopPhysicalLayoutRejectV1::MissingBlock(block_key));
                }
            }
        }
        self.segments.push(PreparedLoopControlSegmentV1 {
            key: LoopPhysicalSegmentKeyV1::new(loop_key, block_key, ordinal),
            operations: operations.into_boxed_slice(),
            transfer: finish_transfer,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::callable_single_loop_operation_effect::callable_operation_demand_parts_for_test;
    use crate::mir::loop_recipe_contract::generic_g0::generic_operation_demand_parts_for_test;
    use crate::mir::loop_recipe_contract::VerifiedLoopOperationPhysicalDemandV1;

    fn callable_layout() -> PreparedLoopPhysicalLayoutV1 {
        let (effect, context, continuation) = callable_operation_demand_parts_for_test();
        VerifiedLoopOperationPhysicalDemandV1::issue(context, effect, continuation)
            .expect("callable demand")
            .prepare_all()
            .expect("callable program")
            .prepare_physical_layout()
            .expect("callable layout")
    }

    fn generic_layout() -> PreparedLoopPhysicalLayoutV1 {
        let (effect, context, continuation) = generic_operation_demand_parts_for_test();
        VerifiedLoopOperationPhysicalDemandV1::issue(context, effect, continuation)
            .expect("generic demand")
            .prepare_all()
            .expect("generic program")
            .prepare_physical_layout()
            .expect("generic layout")
    }

    #[test]
    fn callable_layout_keeps_one_segment_per_logical_block() {
        let layout = callable_layout();
        assert_eq!(layout.coverage().item_count(), 7);
        assert_eq!(layout.coverage().operation_count(), 7);
        assert_eq!(layout.coverage().segment_count(), 2);
        assert_eq!(
            layout.segments()[0].operations(),
            [
                LoopItemKeyV1::new(0),
                LoopItemKeyV1::new(1),
                LoopItemKeyV1::new(2),
            ]
        );
        assert_eq!(layout.segments()[1].operations().len(), 4);
        assert!(matches!(
            layout.segments()[0].transfer(),
            LoopPhysicalTransferV1::Predicate { .. }
        ));
    }

    #[test]
    fn generic_layout_splits_parent_around_nested_loop_and_resumes() {
        let layout = generic_layout();
        let root = LoopNodeKeyV1::new(0);
        let child = LoopNodeKeyV1::new(1);
        let root_condition = LoopPhysicalSegmentKeyV1::new(root, LoopBlockKeyV1::new(0), 0);
        let root_before_child = LoopPhysicalSegmentKeyV1::new(root, LoopBlockKeyV1::new(1), 0);
        let child_condition = LoopPhysicalSegmentKeyV1::new(child, LoopBlockKeyV1::new(2), 0);
        let child_body = LoopPhysicalSegmentKeyV1::new(child, LoopBlockKeyV1::new(3), 0);
        let root_resume = LoopPhysicalSegmentKeyV1::new(root, LoopBlockKeyV1::new(1), 1);
        assert_eq!(layout.coverage().item_count(), 16);
        assert_eq!(layout.coverage().operation_count(), 15);
        assert_eq!(layout.coverage().segment_count(), 5);
        assert_eq!(layout.segments()[0].key(), root_condition);
        assert_eq!(layout.segments()[1].key(), root_before_child);
        assert_eq!(layout.segments()[2].key(), child_condition);
        assert_eq!(layout.segments()[3].key(), child_body);
        assert_eq!(layout.segments()[4].key(), root_resume);
        assert_eq!(layout.segments()[1].operations(), [LoopItemKeyV1::new(3)]);
        assert_eq!(
            layout.segments()[4].operations(),
            [
                LoopItemKeyV1::new(12),
                LoopItemKeyV1::new(13),
                LoopItemKeyV1::new(14),
                LoopItemKeyV1::new(15),
            ]
        );
        assert!(matches!(
            layout.segments()[1].transfer(),
            LoopPhysicalTransferV1::OpenNestedLoop {
                loop_key,
                entry
            } if loop_key == child && entry == child_condition
        ));
        assert!(matches!(
            layout.segments()[2].transfer(),
            LoopPhysicalTransferV1::Predicate {
                on_false: LoopPhysicalTargetV1::Segment(target), ..
            } if target == root_resume
        ));
    }
}
