//! Caller-zero candidate-local physical input for Nested Predicate.
//!
//! The semantic Recipe/JoinSig/topology remain portable and symbolic. This
//! module adds only an owner-branded block projection; it does not allocate
//! blocks, mutate a Builder, or own PHI/SSA state.

use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{FunctionOwnerIdV1, LoopExecutionFrameKeyV1};
use crate::mir::BasicBlockId;

use super::nested_predicate_topology::{
    NestedParentResumePortV1, NestedPhysicalNodeRefV1, NestedPhysicalPortRefV1,
    NestedPhysicalStageV1, NestedPortAliasV1, VerifiedNestedPhysicalEmissionInputV1,
    VerifiedNestedPhysicalTopologyV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NestedPhysicalBlockProjectionRejectV1 {
    OwnerMismatch,
    FrameMismatch,
    TopologyAliasMismatch,
    TopologyResumeMismatch,
    DuplicatePhysicalBlock(BasicBlockId),
}

/// Candidate-local symbolic-to-physical block map.
///
/// There are eleven symbolic node references in the topology, but the child
/// preheader is an explicit alias of root body, so only ten physical blocks
/// are unique.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedPhysicalBlockProjectionV1 {
    owner: FunctionOwnerIdV1,
    root_frame_key: LoopExecutionFrameKeyV1,
    root_preheader: BasicBlockId,
    root_header: BasicBlockId,
    root_body: BasicBlockId,
    root_step: BasicBlockId,
    root_after: BasicBlockId,
    child_header: BasicBlockId,
    child_body: BasicBlockId,
    child_step: BasicBlockId,
    child_after: BasicBlockId,
    parent_resume: BasicBlockId,
}

impl VerifiedNestedPhysicalBlockProjectionV1 {
    pub(crate) fn try_new(
        input: &VerifiedNestedPhysicalEmissionInputV1,
        expected_owner: FunctionOwnerIdV1,
        expected_frame: &LoopExecutionFrameKeyV1,
        root_preheader: BasicBlockId,
        root_fresh: [BasicBlockId; 4],
        child_fresh: [BasicBlockId; 4],
        parent_resume: BasicBlockId,
    ) -> Result<Self, NestedPhysicalBlockProjectionRejectV1> {
        Self::try_new_with_shape(
            input.topology(),
            expected_owner,
            expected_frame,
            root_preheader,
            root_fresh,
            child_fresh,
            parent_resume,
            input.topology().child_preheader_alias(),
            input.topology().parent_resume(),
        )
    }

    fn try_new_with_shape(
        topology: &VerifiedNestedPhysicalTopologyV1,
        expected_owner: FunctionOwnerIdV1,
        expected_frame: &LoopExecutionFrameKeyV1,
        root_preheader: BasicBlockId,
        root_fresh: [BasicBlockId; 4],
        child_fresh: [BasicBlockId; 4],
        parent_resume: BasicBlockId,
        alias: NestedPortAliasV1,
        resume: NestedParentResumePortV1,
    ) -> Result<Self, NestedPhysicalBlockProjectionRejectV1> {
        if topology.owner() != expected_owner {
            return Err(NestedPhysicalBlockProjectionRejectV1::OwnerMismatch);
        }
        if topology.root_frame_key() != expected_frame {
            return Err(NestedPhysicalBlockProjectionRejectV1::FrameMismatch);
        }
        let expected_alias = topology.child_preheader_alias();
        if alias != expected_alias
            || alias.alias.loop_key != resume.child_loop
            || alias.canonical.loop_key != resume.parent_loop
            || alias.alias.stage != NestedPhysicalStageV1::Preheader
            || alias.canonical.stage != NestedPhysicalStageV1::Body
        {
            return Err(NestedPhysicalBlockProjectionRejectV1::TopologyAliasMismatch);
        }
        if resume != topology.parent_resume() {
            return Err(NestedPhysicalBlockProjectionRejectV1::TopologyResumeMismatch);
        }
        let mut seen = BTreeSet::new();
        for block in std::iter::once(root_preheader)
            .chain(root_fresh)
            .chain(child_fresh)
            .chain(std::iter::once(parent_resume))
        {
            if !seen.insert(block) {
                return Err(NestedPhysicalBlockProjectionRejectV1::DuplicatePhysicalBlock(block));
            }
        }
        Ok(Self {
            owner: expected_owner,
            root_frame_key: expected_frame.clone(),
            root_preheader,
            root_header: root_fresh[0],
            root_body: root_fresh[1],
            root_step: root_fresh[2],
            root_after: root_fresh[3],
            child_header: child_fresh[0],
            child_body: child_fresh[1],
            child_step: child_fresh[2],
            child_after: child_fresh[3],
            parent_resume,
        })
    }

    #[cfg(test)]
    pub(crate) fn try_new_with_alias_for_test(
        input: &VerifiedNestedPhysicalEmissionInputV1,
        expected_owner: FunctionOwnerIdV1,
        expected_frame: &LoopExecutionFrameKeyV1,
        root_preheader: BasicBlockId,
        root_fresh: [BasicBlockId; 4],
        child_fresh: [BasicBlockId; 4],
        parent_resume: BasicBlockId,
        alias: NestedPortAliasV1,
    ) -> Result<Self, NestedPhysicalBlockProjectionRejectV1> {
        Self::try_new_with_shape(
            input.topology(),
            expected_owner,
            expected_frame,
            root_preheader,
            root_fresh,
            child_fresh,
            parent_resume,
            alias,
            input.topology().parent_resume(),
        )
    }

    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn root_frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) fn symbolic_node_count(&self) -> usize {
        11
    }

    pub(crate) fn unique_physical_block_count(&self) -> usize {
        10
    }

    pub(crate) fn block(&self, node: NestedPhysicalNodeRefV1) -> BasicBlockId {
        match node {
            NestedPhysicalNodeRefV1::ParentResume(_) => self.parent_resume,
            NestedPhysicalNodeRefV1::Port(port) => self.port_block(port),
        }
    }

    fn port_block(&self, port: NestedPhysicalPortRefV1) -> BasicBlockId {
        match (port.loop_key.raw(), port.stage) {
            (0, NestedPhysicalStageV1::Preheader) => self.root_preheader,
            (0, NestedPhysicalStageV1::Header) => self.root_header,
            (0, NestedPhysicalStageV1::Body) => self.root_body,
            (0, NestedPhysicalStageV1::Step) => self.root_step,
            (0, NestedPhysicalStageV1::After) => self.root_after,
            (1, NestedPhysicalStageV1::Preheader) => self.root_body,
            (1, NestedPhysicalStageV1::Header) => self.child_header,
            (1, NestedPhysicalStageV1::Body) => self.child_body,
            (1, NestedPhysicalStageV1::Step) => self.child_step,
            (1, NestedPhysicalStageV1::After) => self.child_after,
            _ => panic!("verified Nested topology port"),
        }
    }
}

/// P0 candidate bundle: semantic pair, symbolic topology, and candidate-local
/// physical block map. It has no Builder or transaction ownership.
#[derive(Debug)]
pub(crate) struct VerifiedNestedPhysicalCandidateInputV1 {
    emission: VerifiedNestedPhysicalEmissionInputV1,
    blocks: VerifiedNestedPhysicalBlockProjectionV1,
}

impl VerifiedNestedPhysicalCandidateInputV1 {
    pub(crate) fn new(
        emission: VerifiedNestedPhysicalEmissionInputV1,
        blocks: VerifiedNestedPhysicalBlockProjectionV1,
    ) -> Self {
        Self { emission, blocks }
    }

    pub(crate) fn emission(&self) -> &VerifiedNestedPhysicalEmissionInputV1 {
        &self.emission
    }

    pub(crate) fn blocks(&self) -> &VerifiedNestedPhysicalBlockProjectionV1 {
        &self.blocks
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedNestedPhysicalEmissionInputV1,
        VerifiedNestedPhysicalBlockProjectionV1,
    ) {
        (self.emission, self.blocks)
    }
}
