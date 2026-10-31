//! Typed physical operations for one thread-confined Shared payload.
//!
//! Object coordinates are scoped to the containing published module; they do
//! not themselves establish source membership or a globally unique type ID.
use super::{InvokeNormalResultKind, ObjectEndStepV1};
use crate::mir::{Effect, EffectMask, ValueId};
use hakorune_mir_defs::CanonicalObjectIdV1;

/// Source-selected payload declaration and its complete, ordered End plan.
/// The only current representation profile is the thread-confined V1 cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedDescriptorV1 {
    pub(crate) payload: CanonicalObjectIdV1,
    pub(crate) end: Box<[ObjectEndStepV1]>,
}

/// Each handle has a distinct End obligation even when cells are identical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedInvokeOperation {
    /// On Normal transfer the completed payload; on Fault it stays owned.
    New { descriptor: SharedDescriptorV1, payload: ValueId },
    /// On Normal acquire a distinct handle; on Fault acquire nothing.
    Clone { descriptor: SharedDescriptorV1, handle: ValueId },
    /// A read-only payload view supported by this exact handle.
    Borrow { descriptor: SharedDescriptorV1, handle: ValueId },
    /// Consume this handle on either edge; the last invokes payload End.
    Release { descriptor: SharedDescriptorV1, handle: ValueId },
}

impl SharedInvokeOperation {
    pub fn descriptor(&self) -> &SharedDescriptorV1 {
        match self {
            Self::New { descriptor, .. } | Self::Clone { descriptor, .. }
            | Self::Borrow { descriptor, .. } | Self::Release { descriptor, .. } => descriptor,
        }
    }

    pub fn normal_result_kind(&self) -> Option<InvokeNormalResultKind> {
        match self {
            Self::New { .. } | Self::Clone { .. } => Some(InvokeNormalResultKind::Handle),
            Self::Borrow { .. } => Some(InvokeNormalResultKind::Handle),
            Self::Release { .. } => None,
        }
    }

    pub fn effects(&self) -> EffectMask {
        match self {
            Self::New { .. } | Self::Clone { .. } => EffectMask::WRITE.add(Effect::Alloc).add(Effect::Control),
            Self::Borrow { .. } => EffectMask::READ.add(Effect::Control),
            Self::Release { .. } => EffectMask::WRITE.union(EffectMask::MUT).union(EffectMask::IO).add(Effect::Control),
        }
    }

    pub fn used_values(&self) -> Vec<ValueId> {
        match self {
            Self::New { payload, .. } => vec![*payload],
            Self::Clone { handle, .. } | Self::Borrow { handle, .. }
            | Self::Release { handle, .. } => vec![*handle],
        }
    }

    pub fn rewrite_values(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::New { payload, .. } => rewrite(payload),
            Self::Clone { handle, .. } | Self::Borrow { handle, .. }
            | Self::Release { handle, .. } => rewrite(handle),
        }
    }
}
