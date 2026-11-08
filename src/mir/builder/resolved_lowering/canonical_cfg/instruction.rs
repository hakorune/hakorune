//! Mechanical append for checked packets with forward cleanup landings.
//! Source permission remains in the packet. Final CFG close verifies every
//! target and cached incoming edge after all forward blocks have been built.
use super::error::{require_block, CanonicalCfgBlockRoleV1};
use super::predecessors::TerminatorPredecessorsV1;
use super::{CanonicalCfgErrorV1, CanonicalCfgSessionV1};
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction};
use std::cell::RefMut;
use std::collections::{BTreeMap, BTreeSet};

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
        let block = require_block(function, source, CanonicalCfgBlockRoleV1::Source)?;
        if block.is_terminated() {
            return Err(CanonicalCfgErrorV1::SourceAlreadyTerminated { source });
        }
        let mut projection = BasicBlock::new(source);
        projection.add_instruction_with_span(instruction.clone(), span.clone());
        let targets = projection.successors_from_terminator();
        if projection.terminator.is_some() {
            if block.is_sealed() || self.sealed.contains_key(&source) {
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
    pub(super) fn commit_instruction(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        instruction: MirInstruction,
        span: Option<crate::ast::Span>,
    ) {
        // Adopt any pre-session forward edges once, before this write.
        let mut pending = self.forward_predecessors(function);
        crate::mir::ssot::cf_common::append_instruction_core(function, source, instruction, span)
            .expect("prepared CFG source");
        let targets = function.blocks[&source].successors_from_terminator();
        for target in targets {
            if let Some(block) = function.get_block_mut(target) {
                block.add_predecessor(source);
            } else {
                pending.entry(target).or_default().insert(source);
            }
        }
    }
}
