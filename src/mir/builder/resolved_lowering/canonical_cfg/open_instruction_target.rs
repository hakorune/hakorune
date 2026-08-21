//! Private proof for one instruction target created by the canonical CFG session.
//!
//! This witness is intentionally narrower than a dominance proof. It says
//! only that the owning CFG session created the block and that the current MIR
//! block is still open for a tail instruction. The Loop target receipt is
//! supplied by the scoped physical service; this module does not select or
//! reconstruct a target.

use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::BasicBlockId;
use std::collections::BTreeSet;

#[derive(Debug, Default)]
pub(super) struct CanonicalCfgCreationStateV1 {
    owner: Option<FunctionOwnerIdV1>,
    created: BTreeSet<BasicBlockId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct VerifiedCanonicalOpenInstructionTargetV1 {
    owner: FunctionOwnerIdV1,
    block: BasicBlockId,
    _seal: CanonicalOpenInstructionTargetSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalOpenInstructionTargetErrorV1 {
    FunctionMissing,
    SessionOwnerUnavailable,
    SessionOwnerMismatch,
    SessionDidNotCreate(BasicBlockId),
    TargetBlockMissing(BasicBlockId),
    TargetBlockSealed(BasicBlockId),
    TargetBlockTerminated(BasicBlockId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CanonicalOpenInstructionTargetSealV1;

impl CanonicalCfgCreationStateV1 {
    pub(super) fn new_for_owner(owner: FunctionOwnerIdV1) -> Self {
        Self {
            owner: Some(owner),
            created: BTreeSet::new(),
        }
    }

    pub(super) fn record_created(&mut self, block: BasicBlockId) {
        self.created.insert(block);
    }

    pub(super) fn prepare_open_target(
        &self,
        function: &crate::mir::MirFunction,
        is_sealed: bool,
        owner: FunctionOwnerIdV1,
        block: BasicBlockId,
    ) -> Result<VerifiedCanonicalOpenInstructionTargetV1, CanonicalOpenInstructionTargetErrorV1>
    {
        let session_owner = self
            .owner
            .ok_or(CanonicalOpenInstructionTargetErrorV1::SessionOwnerUnavailable)?;
        if session_owner != owner {
            return Err(CanonicalOpenInstructionTargetErrorV1::SessionOwnerMismatch);
        }
        if !self.created.contains(&block) {
            return Err(CanonicalOpenInstructionTargetErrorV1::SessionDidNotCreate(
                block,
            ));
        }
        let target = function.get_block(block).ok_or(
            CanonicalOpenInstructionTargetErrorV1::TargetBlockMissing(block),
        )?;
        if is_sealed || target.is_sealed() {
            return Err(CanonicalOpenInstructionTargetErrorV1::TargetBlockSealed(
                block,
            ));
        }
        if target.is_terminated() {
            return Err(CanonicalOpenInstructionTargetErrorV1::TargetBlockTerminated(block));
        }
        Ok(VerifiedCanonicalOpenInstructionTargetV1::from_session(
            owner, block,
        ))
    }
}

impl VerifiedCanonicalOpenInstructionTargetV1 {
    pub(in crate::mir::builder) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn block(self) -> BasicBlockId {
        self.block
    }

    pub(in crate::mir::builder::resolved_lowering::canonical_cfg) const fn from_session(
        owner: FunctionOwnerIdV1,
        block: BasicBlockId,
    ) -> Self {
        Self {
            owner,
            block,
            _seal: CanonicalOpenInstructionTargetSealV1,
        }
    }
}
