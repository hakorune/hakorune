//! Mechanical append for checked packets with forward cleanup landings.
//! Source permission remains in the packet. Final CFG close verifies every
//! target and cached incoming edge after all forward blocks have been built.
use super::{CanonicalCfgSessionV1, CanonicalCfgErrorV1};
use super::error::{require_block, CanonicalCfgBlockRoleV1};
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction};

impl CanonicalCfgSessionV1 {
    pub(in crate::mir::builder) fn append_packet_instruction(
        &self, function: &mut MirFunction, source: BasicBlockId,
        instruction: MirInstruction, span: crate::ast::Span,
    ) -> Result<(), CanonicalCfgErrorV1> {
        let block = require_block(function, source, CanonicalCfgBlockRoleV1::Source)?;
        if block.is_terminated() { return Err(CanonicalCfgErrorV1::SourceAlreadyTerminated { source }); }
        let mut projection = BasicBlock::new(source);
        projection.add_instruction_with_span(instruction.clone(), span.clone());
        let targets = projection.successors_from_terminator();
        if projection.terminator.is_some() {
            if block.is_sealed() || self.sealed.contains_key(&source) {
                return Err(CanonicalCfgErrorV1::SourceAfterSeal { source });
            }
            for target in &targets {
                if self.sealed.contains_key(target) || function.get_block(*target).is_some_and(|block| block.is_sealed()) {
                    return Err(CanonicalCfgErrorV1::EdgeAfterSeal { source, target: *target });
                }
            }
        }
        Self::commit_instruction(function, source, instruction, Some(span));
        Ok(())
    }

    /// Private mechanical commit shared by all typed CFG writers after their
    /// existing preflight. A missing span retains BasicBlock's old fallback.
    pub(super) fn commit_instruction(function: &mut MirFunction, source: BasicBlockId,
        instruction: MirInstruction, span: Option<crate::ast::Span>,
    ) {
        let block = function.get_block_mut(source).expect("prepared CFG source");
        match span {
            Some(span) => block.add_instruction_with_span(instruction, span),
            None => block.set_terminator(instruction),
        }
        let targets = block.successors_from_terminator();
        for target in targets {
            if let Some(block) = function.get_block_mut(target) { block.add_predecessor(source); }
        }
    }
}
