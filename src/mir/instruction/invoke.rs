//! Canonical fallible control reuses existing Call and allocation operands.
//! This enum is not a target resolver or an arbitrary instruction wrapper.

use crate::mir::definitions::MirCall;
use crate::mir::{Effect, EffectMask, ValueId};

impl crate::mir::MirInstruction {
    /// Identifies physical sites needing retained lifecycle validation.
    /// Presence is an obligation, never source eligibility or permission.
    pub(crate) fn requires_lifecycle_validation(&self) -> bool {
        matches!(
            self,
            Self::Invoke { .. }
                | Self::InvokeNormalResult { .. }
                | Self::ReturnFault { .. }
                | Self::FaultFrameEnter { .. }
                | Self::ArrayResidenceRelease { .. }
        ) || matches!(self, Self::Call(call)
                if matches!(call.callee, crate::mir::Callee::BirthConstructor { .. }))
    }
}

/// Hidden physical entry role, never a source parameter or runtime box type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultFrameMode {
    RootOwned,
    Borrowed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvokeOperation {
    /// The embedded destination must be absent; the Normal projection owns it.
    Call(MirCall),
    /// Allocation only; constructor arguments belong to the subsequent Birth.
    NewBox {
        object: hakorune_mir_defs::CanonicalObjectIdV1,
    },
    /// Intrinsic Array allocation; one value exists only on Normal.
    IntrinsicArrayNew,
    /// Adopt the source-issued numeric state contract; Unit on Normal.
    ArrayStateContractClaim { contract_id: String, array: ValueId },
    /// Checked mutation; Unit on Normal and no mutation on Fault.
    ArrayElementWrite {
        site_id: crate::mir::ArrayWriteSiteId,
        kind: crate::mir::ArrayElementWriteKind,
        producer: crate::mir::ArrayWriteProducerKind,
        receiver: ValueId,
        index: Option<ValueId>,
        value: ValueId,
    },
    /// Exact declaration field; Unit on Normal, no mutation on Fault.
    FieldSet {
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        base: ValueId,
        value: ValueId,
    },
    /// Consume a completed Home; both outcomes continue the cleanup chain.
    HomeRelease {
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        value: ValueId,
    },
    /// Reclaim only incomplete outer storage; never invoke the parent's fini.
    ReclaimUnpublished {
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        value: ValueId,
    },
}

impl InvokeOperation {
    pub fn effects(&self) -> EffectMask {
        match self {
            Self::Call(call) => call.effects.add(Effect::Control),
            Self::NewBox { .. } | Self::IntrinsicArrayNew => EffectMask::CONTROL.add(Effect::Alloc),
            Self::FieldSet { .. }
            | Self::ArrayStateContractClaim { .. }
            | Self::ArrayElementWrite { .. } => EffectMask::WRITE.add(Effect::Control),
            Self::HomeRelease { .. } | Self::ReclaimUnpublished { .. } => EffectMask::WRITE
                .union(EffectMask::MUT)
                .union(EffectMask::IO)
                .add(Effect::Control),
        }
    }

    pub fn used_values(&self) -> Vec<ValueId> {
        match self {
            Self::Call(call) => {
                let mut values = Vec::new();
                call.callee
                    .for_each_value_operand(|value| values.push(value));
                values.extend(call.args.iter().copied());
                values
            }
            Self::NewBox { .. } | Self::IntrinsicArrayNew => Vec::new(),
            Self::ArrayStateContractClaim { array, .. } => vec![*array],
            Self::ArrayElementWrite {
                receiver,
                index,
                value,
                ..
            } => {
                let mut values = vec![*receiver];
                values.extend(index.iter().copied());
                values.push(*value);
                values
            }
            Self::FieldSet { base, value, .. } => vec![*base, *value],
            Self::HomeRelease { value, .. } | Self::ReclaimUnpublished { value, .. } => {
                vec![*value]
            }
        }
    }

    pub fn rewrite_values(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::Call(call) => {
                call.callee.rewrite_value_operands(|value| rewrite(value));
                for value in &mut call.args {
                    rewrite(value);
                }
            }
            Self::NewBox { .. } | Self::IntrinsicArrayNew => {}
            Self::ArrayStateContractClaim { array, .. } => rewrite(array),
            Self::ArrayElementWrite {
                receiver,
                index,
                value,
                ..
            } => {
                rewrite(receiver);
                if let Some(index) = index {
                    rewrite(index);
                }
                rewrite(value);
            }
            Self::HomeRelease { value, .. } | Self::ReclaimUnpublished { value, .. } => {
                rewrite(value)
            }
            Self::FieldSet { base, value, .. } => {
                rewrite(base);
                rewrite(value);
            }
        }
    }
}
