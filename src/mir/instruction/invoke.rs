//! Canonical fallible control reuses existing Call and allocation operands.
//! This enum is not a target resolver or an arbitrary instruction wrapper.

use crate::mir::definitions::MirCall;
use crate::mir::{Effect, EffectMask, ValueId};

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
    NewBox {
        box_type: String,
        args: Vec<ValueId>,
    },
}

impl InvokeOperation {
    pub fn effects(&self) -> EffectMask {
        match self {
            Self::Call(call) => call.effects.add(Effect::Control),
            Self::NewBox { .. } => EffectMask::CONTROL.add(Effect::Alloc),
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
            Self::NewBox { args, .. } => args.clone(),
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
            Self::NewBox { args, .. } => {
                for value in args {
                    rewrite(value);
                }
            }
        }
    }
}
