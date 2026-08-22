//! Prepared canonical branch terminator owned by one CFG session.
//!
//! Preparation verifies the current MIR shape and both open targets without
//! mutating the function.  Commit is the only mutation path for a prepared
//! branch; selected Dynamic uses it after all fallible Compare preparation and
//! before its last-fallible result reservation.

use super::error::CanonicalCfgErrorV1;
use super::session::CanonicalCfgSessionV1;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct PreparedCanonicalBranchV1<'session> {
    session: &'session CanonicalCfgSessionV1,
    source: BasicBlockId,
    condition: ValueId,
    then_block: BasicBlockId,
    else_block: BasicBlockId,
    _seal: PreparedCanonicalBranchSealV1,
}

#[derive(Debug)]
struct PreparedCanonicalBranchSealV1;

impl<'session> PreparedCanonicalBranchV1<'session> {
    pub(in crate::mir::builder::resolved_lowering::canonical_cfg) const fn from_session(
        session: &'session CanonicalCfgSessionV1,
        source: BasicBlockId,
        condition: ValueId,
        then_block: BasicBlockId,
        else_block: BasicBlockId,
    ) -> Self {
        Self {
            session,
            source,
            condition,
            then_block,
            else_block,
            _seal: PreparedCanonicalBranchSealV1,
        }
    }

    pub(in crate::mir::builder::resolved_lowering) fn commit(self, function: &mut MirFunction) {
        let Self {
            session,
            source,
            condition,
            then_block,
            else_block,
            _seal,
        } = self;
        session.commit_prepared_branch(function, source, condition, then_block, else_block);
    }
}

impl CanonicalCfgSessionV1 {
    pub(in crate::mir::builder) fn emit_branch(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        condition: ValueId,
        then_block: BasicBlockId,
        else_block: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        let prepared = self.prepare_branch(function, source, condition, then_block, else_block)?;
        prepared.commit(function);
        Ok(())
    }

    /// Prepare one branch without changing MIR. The CFG session is the sole
    /// owner of the open-target and predecessor proof used by the product.
    pub(in crate::mir::builder::resolved_lowering) fn prepare_branch(
        &self,
        function: &MirFunction,
        source: BasicBlockId,
        condition: ValueId,
        then_block: BasicBlockId,
        else_block: BasicBlockId,
    ) -> Result<PreparedCanonicalBranchV1<'_>, CanonicalCfgErrorV1> {
        if then_block == else_block {
            return Err(CanonicalCfgErrorV1::DuplicateEdge {
                source,
                target: then_block,
            });
        }
        self.preflight_edge(function, source, &[then_block, else_block])?;
        Ok(PreparedCanonicalBranchV1::from_session(
            self, source, condition, then_block, else_block,
        ))
    }

    /// Commit the already-bound branch fields. The prepared product carries
    /// the exact session reference, so it cannot be handed to another CFG
    /// session's commit method.
    fn commit_prepared_branch(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        condition: ValueId,
        then_block: BasicBlockId,
        else_block: BasicBlockId,
    ) {
        function
            .get_block_mut(source)
            .expect("source was checked")
            .set_terminator(MirInstruction::Branch {
                condition,
                then_bb: then_block,
                else_bb: else_block,
                then_edge_args: None,
                else_edge_args: None,
            });
        for target in [then_block, else_block] {
            function
                .get_block_mut(target)
                .expect("target was checked")
                .add_predecessor(source);
        }
    }
}
