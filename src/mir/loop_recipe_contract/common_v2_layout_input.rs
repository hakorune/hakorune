//! V2 physical-layout input transport.
//!
//! This is a source-backed, physical-ID-free projection.  It copies only
//! logical keys and borrows the source block item slices; it never allocates a
//! MIR block, reads the Builder cursor, or reissues a JoinSig.

use std::collections::BTreeSet;

use super::ids::{LoopBindingKeyV1, LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1};
use super::s6c_prephysical_ingress::S6CPrephysicalIngressRefV2;
use super::schema_v2::LoopValueClassV2;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PreparedLoopV2LayoutLoopV1 {
    key: LoopNodeKeyV1,
    parent: Option<LoopNodeKeyV1>,
    condition_block: LoopBlockKeyV1,
    body: LoopBlockKeyV1,
}

impl PreparedLoopV2LayoutLoopV1 {
    pub(crate) const fn key(self) -> LoopNodeKeyV1 {
        self.key
    }

    pub(crate) const fn parent(self) -> Option<LoopNodeKeyV1> {
        self.parent
    }

    pub(crate) const fn condition_block(self) -> LoopBlockKeyV1 {
        self.condition_block
    }

    pub(crate) const fn body(self) -> LoopBlockKeyV1 {
        self.body
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PreparedLoopV2LayoutSegmentRefV1<'source> {
    loop_key: LoopNodeKeyV1,
    block: LoopBlockKeyV1,
    split_ordinal: u32,
    items: &'source [LoopItemKeyV1],
}

impl PreparedLoopV2LayoutSegmentRefV1<'_> {
    pub(crate) const fn loop_key(&self) -> LoopNodeKeyV1 {
        self.loop_key
    }

    pub(crate) const fn block(&self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(crate) const fn split_ordinal(&self) -> u32 {
        self.split_ordinal
    }

    pub(crate) fn items(&self) -> &[LoopItemKeyV1] {
        self.items
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LayoutInputRejectV1 {
    ForeignOwner,
    MissingTopology,
    DuplicateLoop(LoopNodeKeyV1),
    DuplicateBlock(LoopBlockKeyV1),
    UnknownParent(LoopNodeKeyV1),
    UnknownLoopForBlock(LoopBlockKeyV1),
    BlockOwnerMismatch(LoopBlockKeyV1),
    DuplicateItem(LoopItemKeyV1),
    UnknownAfterLoop(LoopNodeKeyV1),
    SegmentOrdinalOverflow,
}

#[derive(Debug)]
pub(crate) struct PreparedLoopV2PhysicalLayoutInputV1<'source> {
    owner: FunctionOwnerIdV1,
    loops: Box<[PreparedLoopV2LayoutLoopV1]>,
    segments: Box<[PreparedLoopV2LayoutSegmentRefV1<'source>]>,
    after: (LoopNodeKeyV1, LoopBindingKeyV1, LoopValueClassV2),
}

impl PreparedLoopV2PhysicalLayoutInputV1<'_> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn loops(&self) -> &[PreparedLoopV2LayoutLoopV1] {
        &self.loops
    }

    pub(crate) fn segments(&self) -> &[PreparedLoopV2LayoutSegmentRefV1<'_>] {
        &self.segments
    }

    pub(crate) const fn after(&self) -> (LoopNodeKeyV1, LoopBindingKeyV1, LoopValueClassV2) {
        self.after
    }

    pub(crate) fn has_block(&self, block: LoopBlockKeyV1) -> bool {
        self.segments.iter().any(|segment| segment.block == block)
    }

    pub(crate) fn segment_for_block(
        &self,
        block: LoopBlockKeyV1,
    ) -> Option<&PreparedLoopV2LayoutSegmentRefV1<'_>> {
        self.segments.iter().find(|segment| segment.block == block)
    }

    pub(crate) fn item_count(&self) -> usize {
        self.segments
            .iter()
            .map(|segment| segment.items.len())
            .sum()
    }

    pub(crate) fn segment_count(&self) -> usize {
        self.segments.len()
    }

    pub(crate) fn loop_count(&self) -> usize {
        self.loops.len()
    }
}

pub(crate) fn issue_s6c_v2_layout_input<'rows, 'facts>(
    ingress: S6CPrephysicalIngressRefV2<'_, 'rows, 'facts>,
    expected_owner: FunctionOwnerIdV1,
) -> Result<PreparedLoopV2PhysicalLayoutInputV1<'rows>, LayoutInputRejectV1> {
    if ingress.source_owner() != expected_owner {
        return Err(LayoutInputRejectV1::ForeignOwner);
    }
    let source_loops = ingress.logical_loops();
    let source_blocks = ingress.logical_blocks();
    if source_loops.is_empty() || source_blocks.is_empty() {
        return Err(LayoutInputRejectV1::MissingTopology);
    }

    let mut loop_keys = BTreeSet::new();
    for row in source_loops {
        if !loop_keys.insert(row.key) {
            return Err(LayoutInputRejectV1::DuplicateLoop(row.key));
        }
        if let Some(parent) = row.parent {
            if !source_loops.iter().any(|candidate| candidate.key == parent) {
                return Err(LayoutInputRejectV1::UnknownParent(parent));
            }
        }
    }

    let mut block_keys = BTreeSet::new();
    let mut item_keys = BTreeSet::new();
    let mut loops = Vec::with_capacity(source_loops.len());
    for row in source_loops {
        loops.push(PreparedLoopV2LayoutLoopV1 {
            key: row.key,
            parent: row.parent,
            condition_block: row.condition_block,
            body: row.body,
        });
    }

    let mut segments = Vec::with_capacity(source_blocks.len());
    for (index, row) in source_blocks.iter().enumerate() {
        if !block_keys.insert(row.key) {
            return Err(LayoutInputRejectV1::DuplicateBlock(row.key));
        }
        if !loop_keys.contains(&row.owner_loop) {
            return Err(LayoutInputRejectV1::UnknownLoopForBlock(row.key));
        }
        for item in &row.items {
            if !item_keys.insert(*item) {
                return Err(LayoutInputRejectV1::DuplicateItem(*item));
            }
        }
        let split_ordinal =
            u32::try_from(index).map_err(|_| LayoutInputRejectV1::SegmentOrdinalOverflow)?;
        segments.push(PreparedLoopV2LayoutSegmentRefV1 {
            loop_key: row.owner_loop,
            block: row.key,
            split_ordinal,
            items: &row.items,
        });
    }

    for row in &loops {
        for block in [row.condition_block, row.body] {
            let Some(segment) = segments.iter().find(|segment| segment.block == block) else {
                return Err(LayoutInputRejectV1::UnknownLoopForBlock(block));
            };
            if segment.loop_key != row.key {
                return Err(LayoutInputRejectV1::BlockOwnerMismatch(block));
            }
        }
    }
    let after = ingress.after();
    if !loop_keys.contains(&after.0) {
        return Err(LayoutInputRejectV1::UnknownAfterLoop(after.0));
    }
    Ok(PreparedLoopV2PhysicalLayoutInputV1 {
        owner: expected_owner,
        loops: loops.into_boxed_slice(),
        segments: segments.into_boxed_slice(),
        after,
    })
}
