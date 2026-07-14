use crate::mir::BasicBlockId;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalCfgBlockRoleV1 {
    Source,
    Target,
    Seal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalCfgErrorV1 {
    MissingBlock {
        block: BasicBlockId,
        role: CanonicalCfgBlockRoleV1,
    },
    SourceAlreadyTerminated {
        source: BasicBlockId,
    },
    DuplicateEdge {
        source: BasicBlockId,
        target: BasicBlockId,
    },
    EdgeAfterSeal {
        source: BasicBlockId,
        target: BasicBlockId,
    },
    SealTwice {
        block: BasicBlockId,
    },
    DanglingTerminatorTarget {
        source: BasicBlockId,
        target: BasicBlockId,
    },
    CachedSuccessorsMismatch {
        block: BasicBlockId,
        terminator: Box<[BasicBlockId]>,
        cached: Box<[BasicBlockId]>,
    },
    CachedPredecessorsMismatch {
        block: BasicBlockId,
        terminator: Box<[BasicBlockId]>,
        cached: Box<[BasicBlockId]>,
    },
    UnsealedBlockAtFinish {
        block: BasicBlockId,
    },
    SealStateMismatch {
        block: BasicBlockId,
    },
    SealedPredecessorsChanged {
        block: BasicBlockId,
        sealed: Box<[BasicBlockId]>,
        current: Box<[BasicBlockId]>,
    },
    SealedBlockRemoved {
        block: BasicBlockId,
    },
}

impl fmt::Display for CanonicalCfgErrorV1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingBlock { block, role } => {
                write!(f, "canonical CFG missing {role:?} block {block}")
            }
            Self::SourceAlreadyTerminated { source } => {
                write!(f, "canonical CFG source block {source} already has a terminator")
            }
            Self::DuplicateEdge { source, target } => {
                write!(f, "canonical CFG duplicate edge {source} -> {target}")
            }
            Self::EdgeAfterSeal { source, target } => {
                write!(f, "canonical CFG late edge {source} -> sealed block {target}")
            }
            Self::SealTwice { block } => {
                write!(f, "canonical CFG block {block} was sealed twice")
            }
            Self::DanglingTerminatorTarget { source, target } => {
                write!(f, "canonical CFG terminator {source} targets missing block {target}")
            }
            Self::CachedSuccessorsMismatch {
                block,
                terminator,
                cached,
            } => write!(
                f,
                "canonical CFG successor cache mismatch at {block}: terminator={terminator:?}, cached={cached:?}"
            ),
            Self::CachedPredecessorsMismatch {
                block,
                terminator,
                cached,
            } => write!(
                f,
                "canonical CFG predecessor cache mismatch at {block}: terminator={terminator:?}, cached={cached:?}"
            ),
            Self::UnsealedBlockAtFinish { block } => {
                write!(f, "canonical CFG finish found unsealed block {block}")
            }
            Self::SealStateMismatch { block } => {
                write!(f, "canonical CFG seal witness disagrees with block {block}")
            }
            Self::SealedPredecessorsChanged {
                block,
                sealed,
                current,
            } => write!(
                f,
                "canonical CFG predecessors changed after sealing {block}: sealed={sealed:?}, current={current:?}"
            ),
            Self::SealedBlockRemoved { block } => {
                write!(f, "canonical CFG sealed block {block} was removed")
            }
        }
    }
}

impl Error for CanonicalCfgErrorV1 {}
