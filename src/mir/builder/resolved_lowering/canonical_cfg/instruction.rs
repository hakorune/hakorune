//! Mechanical append for checked packets with forward cleanup landings.
//! Source permission remains in the packet. Final CFG close verifies every
//! target and cached incoming edge after all forward blocks have been built.
use super::error::{require_block, CanonicalCfgBlockRoleV1};
use super::predecessors::TerminatorPredecessorsV1;
use super::{CanonicalCfgErrorV1, CanonicalCfgSessionV1};
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction};
use std::cell::RefMut;
use std::collections::{BTreeMap, BTreeSet};

/// Placement selected by the original emission API, not inferred from a shape.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum InstructionPlacementV1 {
    Packet,
    Body,
    Scheduled,
}

impl CanonicalCfgSessionV1 {
    /// Missing targets have no block-local predecessor cache yet. Adopt the
    /// prefix once, then let this session's sole writer retain forward edges.
    pub(super) fn forward_predecessors(
        &self,
        function: &MirFunction,
    ) -> RefMut<'_, TerminatorPredecessorsV1> {
        let mut pending = self.forward_predecessors.borrow_mut();
        if pending.is_none() {
            let mut incoming = BTreeMap::new();
            for (source, block) in &function.blocks {
                for target in block.successors_from_terminator() {
                    if !function.blocks.contains_key(&target) {
                        incoming
                            .entry(target)
                            .or_insert_with(BTreeSet::new)
                            .insert(*source);
                    }
                }
            }
            *pending = Some(incoming);
        }
        RefMut::map(pending, |rows| {
            rows.as_mut().expect("adopted forward edges")
        })
    }

    pub(in crate::mir::builder) fn append_packet_instruction(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        instruction: MirInstruction,
        span: crate::ast::Span,
    ) -> Result<(), CanonicalCfgErrorV1> {
        self.append_selected_instruction(
            function,
            source,
            instruction,
            span,
            InstructionPlacementV1::Packet,
        )
    }

    /// The source body cursor can continue after its incoming SSA edge set is
    /// sealed. Its own terminator still closes the cursor; target seals remain
    /// strict. Checked packets use their stronger open-source contract above.
    pub(in crate::mir::builder) fn append_body_instruction(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        instruction: MirInstruction,
        span: crate::ast::Span,
    ) -> Result<(), CanonicalCfgErrorV1> {
        self.append_selected_instruction(
            function,
            source,
            instruction,
            span,
            InstructionPlacementV1::Body,
        )
    }

    pub(in crate::mir::builder) fn append_selected_instruction(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        instruction: MirInstruction,
        span: crate::ast::Span,
        placement: InstructionPlacementV1,
    ) -> Result<(), CanonicalCfgErrorV1> {
        let block = require_block(function, source, CanonicalCfgBlockRoleV1::Source)?;
        let mut projection = BasicBlock::new(source);
        projection.add_instruction_with_span(instruction.clone(), span.clone());
        let targets = projection.successors_from_terminator();
        let scheduled_definition = projection.terminator.is_none()
            && (placement == InstructionPlacementV1::Scheduled
                || (placement == InstructionPlacementV1::Body
                    && matches!(instruction, MirInstruction::Phi { .. })));
        if block.is_terminated() && !scheduled_definition {
            return Err(CanonicalCfgErrorV1::SourceAlreadyTerminated { source });
        }
        if projection.terminator.is_some() {
            if placement == InstructionPlacementV1::Packet
                && (block.is_sealed() || self.sealed.contains_key(&source))
            {
                return Err(CanonicalCfgErrorV1::SourceAfterSeal { source });
            }
            for target in &targets {
                if self.sealed.contains_key(target)
                    || function
                        .get_block(*target)
                        .is_some_and(|block| block.is_sealed())
                {
                    return Err(CanonicalCfgErrorV1::EdgeAfterSeal {
                        source,
                        target: *target,
                    });
                }
            }
        }
        self.commit_instruction(function, source, instruction, Some(span));
        Ok(())
    }

    /// Private mechanical commit shared by all typed CFG writers after their
    /// existing preflight. A missing span retains BasicBlock's old fallback.
    pub(in crate::mir::builder) fn commit_instruction(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        instruction: MirInstruction,
        span: Option<crate::ast::Span>,
    ) {
        if let MirInstruction::Return { value } = instruction {
            return self.commit_return(function, source, value, span);
        }
        self.commit_with(function, source, |function| {
            crate::mir::ssot::cf_common::append_instruction_core(
                function,
                source,
                instruction,
                span,
            )
            .expect("prepared CFG source");
        });
    }

    /// All Return callers retain their existing preflight and span policy.
    /// Only the common terminal append and edge-truth update are committed here.
    pub(in crate::mir::builder) fn commit_return(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        value: Option<crate::mir::ValueId>,
        span: Option<crate::ast::Span>,
    ) {
        self.commit_with(function, source, |function| {
            crate::mir::ssot::cf_common::append_return_core(function, source, value, span)
                .expect("prepared CFG source");
        });
    }

    fn commit_with(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        append: impl FnOnce(&mut MirFunction),
    ) {
        drop(self.forward_predecessors(function));
        let previous = function.blocks[&source].successors_from_terminator();
        append(function);
        self.record_instruction_edges(function, source, &previous);
    }

    pub(super) fn record_instruction_edges(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        previous: &std::collections::BTreeSet<BasicBlockId>,
    ) {
        let mut pending = self.forward_predecessors(function);
        let targets = function.blocks[&source].successors_from_terminator();
        for target in previous.difference(&targets) {
            if let Some(block) = function.get_block_mut(*target) {
                block.remove_predecessor(source);
            }
            if let Some(incoming) = pending.get_mut(target) {
                incoming.remove(&source);
            }
        }
        for target in targets.difference(previous).copied() {
            if let Some(block) = function.get_block_mut(target) {
                block.add_predecessor(source);
            } else {
                pending.entry(target).or_default().insert(source);
            }
        }
    }
}
