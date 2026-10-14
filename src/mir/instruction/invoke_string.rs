//! Physical expression operands; source loans supply the exact original bytes
//! or scalar domain. These tags never infer source permissions or stringify Box.
use super::InvokeNormalResultKind;
use crate::mir::{Effect, EffectMask, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringInvokeOperand {
    StaticText { utf8: String },
    OwnedString(ValueId),
    NormalInteger(ValueId),
}
impl StringInvokeOperand {
    pub fn value(&self) -> Option<ValueId> {
        match self {
            Self::StaticText { .. } => None,
            Self::OwnedString(value) | Self::NormalInteger(value) => Some(*value),
        }
    }
    pub fn rewrite_value(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::StaticText { .. } => {}
            Self::OwnedString(value) | Self::NormalInteger(value) => rewrite(value),
        }
    }
    pub fn is_text(&self) -> bool {
        !matches!(self, Self::NormalInteger(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringInvokeOperation {
    /// Borrow operands; only Normal publishes one fresh independent String.
    Add {
        left: StringInvokeOperand,
        right: StringInvokeOperand,
    },
    /// Borrow the root value and write exact bytes followed by a newline.
    Print { value: StringInvokeOperand },
    /// Consume exactly this pair; never an object or raw-handle release.
    End { value: ValueId },
}
impl StringInvokeOperation {
    pub fn normal_result_kind(&self) -> Option<InvokeNormalResultKind> {
        match self {
            Self::Add { .. } => Some(InvokeNormalResultKind::OwnedString),
            Self::Print { .. } | Self::End { .. } => None,
        }
    }
    pub fn effects(&self) -> EffectMask {
        match self {
            Self::Add { .. } => EffectMask::READ.add(Effect::Alloc).add(Effect::Control),
            // Runtime output preparation may allocate; End consumes residence.
            Self::Print { .. } => EffectMask::IO.add(Effect::Alloc).add(Effect::Control),
            Self::End { .. } => EffectMask::WRITE.add(Effect::Control),
        }
    }
    pub fn operands(&self) -> Vec<&StringInvokeOperand> {
        match self {
            Self::Add { left, right } => vec![left, right],
            Self::Print { value } => vec![value],
            Self::End { .. } => vec![],
        }
    }
    pub fn used_values(&self) -> Vec<ValueId> {
        match self {
            Self::End { value } => vec![*value],
            _ => self
                .operands()
                .into_iter()
                .filter_map(StringInvokeOperand::value)
                .collect(),
        }
    }
    pub fn rewrite_values(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::Add { left, right } => {
                left.rewrite_value(&mut rewrite);
                right.rewrite_value(rewrite);
            }
            Self::Print { value } => value.rewrite_value(rewrite),
            Self::End { value } => rewrite(value),
        }
    }
}
