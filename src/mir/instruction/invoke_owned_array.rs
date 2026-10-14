//! Physical owned-slot operations. Source admission and transfer permission
//! remain on the retained holder Recipe; these operands cannot issue either.
use super::InvokeNormalResultKind;
use crate::mir::{Effect, EffectMask, ValueId};
use hakorune_mir_defs::CanonicalObjectIdV1;

/// All operations on one holder use this same immutable descriptor. The final
/// ABI resolves its canonical layout to a runtime type and complete End thunk;
/// neither the handle's bits nor a numeric Array contract may supply them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnedArraySlotDescriptorV1 {
    object: CanonicalObjectIdV1,
    nullable: bool,
}
impl OwnedArraySlotDescriptorV1 {
    pub(crate) fn new(object: CanonicalObjectIdV1, nullable: bool) -> Self {
        Self { object, nullable }
    }
    pub fn object(self) -> CanonicalObjectIdV1 { self.object }
    pub fn nullable(self) -> bool { self.nullable }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnedArrayInvokeOperation {
    /// Claim already-acquired Array storage; this transfers no child.
    Claim { array: ValueId, descriptor: OwnedArraySlotDescriptorV1 },
    /// Only Normal moves the candidate's Home into a slot. On Fault the
    /// candidate stays caller-owned and its complete cleanup remains owed.
    Append { array: ValueId, descriptor: OwnedArraySlotDescriptorV1, candidate: ValueId },
    /// Normal borrows a nullable child while this holder remains live. It
    /// creates neither a child Home nor independent End authority.
    Get { array: ValueId, descriptor: OwnedArraySlotDescriptorV1, index: ValueId },
    /// Both outcomes consume the holder after complete reverse child End.
    End { array: ValueId, descriptor: OwnedArraySlotDescriptorV1 },
}
impl OwnedArrayInvokeOperation {
    pub fn descriptor(&self) -> OwnedArraySlotDescriptorV1 {
        match self {
            Self::Claim { descriptor, .. } | Self::Append { descriptor, .. }
            | Self::Get { descriptor, .. } | Self::End { descriptor, .. } => *descriptor,
        }
    }
    pub fn array(&self) -> ValueId {
        match self {
            Self::Claim { array, .. } | Self::Append { array, .. }
            | Self::Get { array, .. } | Self::End { array, .. } => *array,
        }
    }
    pub fn normal_result_kind(&self) -> Option<InvokeNormalResultKind> {
        match self {
            Self::Get { .. } => Some(InvokeNormalResultKind::BorrowedNullableHandle),
            Self::Claim { .. } | Self::Append { .. } | Self::End { .. } => None,
        }
    }
    pub fn effects(&self) -> EffectMask {
        match self {
            Self::Claim { .. } => EffectMask::WRITE.add(Effect::Control),
            Self::Append { .. } => EffectMask::WRITE.add(Effect::Alloc).add(Effect::Control),
            Self::Get { .. } => EffectMask::READ.add(Effect::Control),
            Self::End { .. } => EffectMask::WRITE.union(EffectMask::MUT)
                .union(EffectMask::IO).add(Effect::Control),
        }
    }
    pub fn used_values(&self) -> Vec<ValueId> {
        match self {
            Self::Claim { array, .. } | Self::End { array, .. } => vec![*array],
            Self::Append { array, candidate, .. } => vec![*array, *candidate],
            Self::Get { array, index, .. } => vec![*array, *index],
        }
    }
    pub fn rewrite_values(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::Claim { array, .. } | Self::End { array, .. } => rewrite(array),
            Self::Append { array, candidate, .. } => { rewrite(array); rewrite(candidate); }
            Self::Get { array, index, .. } => { rewrite(array); rewrite(index); }
        }
    }
}
