//! Exact segment-to-block receipt for the R3 recursive physicalizer.
//!
//! The R1 layout is the only logical authority. Operation placement consumes
//! this receipt by segment key and never looks up a logical block again. The
//! receipt also retains the explicit entry segment and root After block.

use std::collections::BTreeSet;

use super::topology::LoopPhysicalBlockRoleV1;
use crate::mir::loop_recipe_contract::LoopPhysicalSegmentKeyV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::BasicBlockId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LoopPhysicalSegmentBlockRowV1 {
    segment: LoopPhysicalSegmentKeyV1,
    role: LoopPhysicalBlockRoleV1,
    physical_block: BasicBlockId,
}

impl LoopPhysicalSegmentBlockRowV1 {
    pub(super) const fn new(
        segment: LoopPhysicalSegmentKeyV1,
        role: LoopPhysicalBlockRoleV1,
        physical_block: BasicBlockId,
    ) -> Self {
        Self {
            segment,
            role,
            physical_block,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopPhysicalSegmentBlockReceiptRejectV1 {
    EmptySegments,
    ForeignSegment(LoopPhysicalSegmentKeyV1),
    DuplicateSegment(LoopPhysicalSegmentKeyV1),
    DuplicatePhysicalBlock(BasicBlockId),
    MissingSegment(LoopPhysicalSegmentKeyV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct LoopPhysicalSegmentBlockReceiptV1 {
    owner: FunctionOwnerIdV1,
    preheader: BasicBlockId,
    entry_segment: LoopPhysicalSegmentKeyV1,
    root_after: BasicBlockId,
    rows: Box<[LoopPhysicalSegmentBlockRowV1]>,
}

impl LoopPhysicalSegmentBlockReceiptV1 {
    pub(super) fn issue(
        owner: FunctionOwnerIdV1,
        preheader: BasicBlockId,
        expected_segments: &[LoopPhysicalSegmentKeyV1],
        rows: Vec<LoopPhysicalSegmentBlockRowV1>,
    ) -> Result<Self, LoopPhysicalSegmentBlockReceiptRejectV1> {
        let entry_segment = expected_segments
            .first()
            .copied()
            .ok_or(LoopPhysicalSegmentBlockReceiptRejectV1::EmptySegments)?;
        Self::issue_with_boundary(
            owner,
            preheader,
            entry_segment,
            preheader,
            expected_segments,
            rows,
        )
    }

    pub(super) fn issue_with_boundary(
        owner: FunctionOwnerIdV1,
        preheader: BasicBlockId,
        entry_segment: LoopPhysicalSegmentKeyV1,
        root_after: BasicBlockId,
        expected_segments: &[LoopPhysicalSegmentKeyV1],
        rows: Vec<LoopPhysicalSegmentBlockRowV1>,
    ) -> Result<Self, LoopPhysicalSegmentBlockReceiptRejectV1> {
        let expected = expected_segments.iter().copied().collect::<BTreeSet<_>>();
        if !expected.contains(&entry_segment) {
            return Err(LoopPhysicalSegmentBlockReceiptRejectV1::MissingSegment(
                entry_segment,
            ));
        }
        let mut segments = BTreeSet::new();
        let mut physical_blocks = BTreeSet::new();
        for row in &rows {
            if !expected.contains(&row.segment) {
                return Err(LoopPhysicalSegmentBlockReceiptRejectV1::ForeignSegment(
                    row.segment,
                ));
            }
            if !segments.insert(row.segment) {
                return Err(LoopPhysicalSegmentBlockReceiptRejectV1::DuplicateSegment(
                    row.segment,
                ));
            }
            if !physical_blocks.insert(row.physical_block) {
                return Err(
                    LoopPhysicalSegmentBlockReceiptRejectV1::DuplicatePhysicalBlock(
                        row.physical_block,
                    ),
                );
            }
        }
        for &segment in expected_segments {
            if !segments.contains(&segment) {
                return Err(LoopPhysicalSegmentBlockReceiptRejectV1::MissingSegment(
                    segment,
                ));
            }
        }
        Ok(Self {
            owner,
            preheader,
            entry_segment,
            root_after,
            rows: rows.into_boxed_slice(),
        })
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn preheader(&self) -> BasicBlockId {
        self.preheader
    }

    pub(super) const fn entry_segment(&self) -> LoopPhysicalSegmentKeyV1 {
        self.entry_segment
    }

    pub(super) const fn root_after(&self) -> BasicBlockId {
        self.root_after
    }

    pub(super) fn rows(&self) -> &[LoopPhysicalSegmentBlockRowV1] {
        &self.rows
    }

    pub(super) fn contains_physical_block(&self, block: BasicBlockId) -> bool {
        self.rows.iter().any(|row| row.physical_block == block)
    }

    pub(super) fn lookup(&self, segment: LoopPhysicalSegmentKeyV1) -> Option<BasicBlockId> {
        self.rows
            .iter()
            .find(|row| row.segment == segment)
            .map(|row| row.physical_block)
    }

    pub(super) fn role(
        &self,
        segment: LoopPhysicalSegmentKeyV1,
    ) -> Option<LoopPhysicalBlockRoleV1> {
        self.rows
            .iter()
            .find(|row| row.segment == segment)
            .map(|row| row.role)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::callable_single_loop_operation_effect::callable_operation_demand_parts_for_test;
    use crate::mir::loop_recipe_contract::VerifiedLoopOperationPhysicalDemandV1;

    fn callable_segments() -> (FunctionOwnerIdV1, Vec<LoopPhysicalSegmentKeyV1>) {
        let (effect, context, continuation) = callable_operation_demand_parts_for_test();
        let layout = VerifiedLoopOperationPhysicalDemandV1::issue(context, effect, continuation)
            .expect("callable demand")
            .prepare_all()
            .expect("callable program")
            .prepare_physical_layout()
            .expect("callable layout");
        (
            layout.program().demand().context().owner(),
            layout.segments().iter().map(|row| row.key()).collect(),
        )
    }

    #[test]
    fn receipt_requires_exact_segment_coverage_and_unique_blocks() {
        let (owner, segments) = callable_segments();
        let result = LoopPhysicalSegmentBlockReceiptV1::issue(
            owner,
            BasicBlockId::new(0),
            &segments,
            vec![
                LoopPhysicalSegmentBlockRowV1::new(
                    segments[0],
                    LoopPhysicalBlockRoleV1::Header,
                    BasicBlockId::new(1),
                ),
                LoopPhysicalSegmentBlockRowV1::new(
                    segments[1],
                    LoopPhysicalBlockRoleV1::Body,
                    BasicBlockId::new(2),
                ),
            ],
        )
        .expect("exact receipt");
        assert_eq!(result.lookup(segments[0]), Some(BasicBlockId::new(1)));
        assert_eq!(result.rows().len(), 2);
    }

    #[test]
    fn receipt_rejects_segment_aliasing_one_physical_block() {
        let (owner, segments) = callable_segments();
        let error = LoopPhysicalSegmentBlockReceiptV1::issue(
            owner,
            BasicBlockId::new(0),
            &segments,
            vec![
                LoopPhysicalSegmentBlockRowV1::new(
                    segments[0],
                    LoopPhysicalBlockRoleV1::Header,
                    BasicBlockId::new(1),
                ),
                LoopPhysicalSegmentBlockRowV1::new(
                    segments[1],
                    LoopPhysicalBlockRoleV1::Body,
                    BasicBlockId::new(1),
                ),
            ],
        )
        .expect_err("segment alias must reject");
        assert_eq!(
            error,
            LoopPhysicalSegmentBlockReceiptRejectV1::DuplicatePhysicalBlock(BasicBlockId::new(1))
        );
    }
}
