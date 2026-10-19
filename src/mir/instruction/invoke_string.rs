//! Physical expression operands; source loans supply the exact original bytes
//! or scalar domain. These tags never infer source permissions or stringify Box.
use super::InvokeNormalResultKind;
use crate::mir::{Effect, EffectMask, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringInvokeOperand {
    StaticText { utf8: String },
    OwnedString(ValueId),
    /// Readonly tagged Text formal/view; never an owned acquisition.
    BorrowedText(ValueId),
    NormalInteger(ValueId),
}
impl StringInvokeOperand {
    pub fn value(&self) -> Option<ValueId> {
        match self {
            Self::StaticText { .. } => None,
            Self::OwnedString(value) | Self::BorrowedText(value) | Self::NormalInteger(value) => Some(*value),
        }
    }
    pub fn rewrite_value(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::StaticText { .. } => {}
            Self::OwnedString(value) | Self::BorrowedText(value) | Self::NormalInteger(value) => rewrite(value),
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
    /// Borrow exact Text operands and publish one Bool, with no owned result.
    Compare { op: crate::mir::CompareOp, left: StringInvokeOperand, right: StringInvokeOperand },
    /// Borrow Text and publish a fresh independent pair without consuming input.
    Materialize { value: StringInvokeOperand },
    /// Fixed CodePoint operations; the source issuer retains the exact Core law.
    Length { value: StringInvokeOperand },
    Substring { value: StringInvokeOperand, start: ValueId, end: ValueId },
    IndexOf { value: StringInvokeOperand, needle: StringInvokeOperand },
    /// Borrow the root value and write exact bytes followed by a newline.
    Print { value: StringInvokeOperand },
    /// Consume exactly this pair; never an object or raw-handle release.
    End { value: ValueId },
}
impl StringInvokeOperation {
    pub fn normal_result_kind(&self) -> Option<InvokeNormalResultKind> {
        match self {
            Self::Add { .. } | Self::Substring { .. } | Self::Materialize { .. } => Some(InvokeNormalResultKind::OwnedString),
            Self::Length { .. } | Self::IndexOf { .. } => Some(InvokeNormalResultKind::I64),
            Self::Compare { .. } => Some(InvokeNormalResultKind::Bool),
            Self::Print { .. } | Self::End { .. } => None,
        }
    }
    pub fn effects(&self) -> EffectMask {
        match self {
            Self::Add { .. } | Self::Substring { .. } | Self::Materialize { .. } => EffectMask::READ.add(Effect::Alloc).add(Effect::Control),
            Self::Length { .. } | Self::IndexOf { .. } | Self::Compare { .. } => EffectMask::READ.add(Effect::Control),
            // Runtime output preparation may allocate; End consumes residence.
            Self::Print { .. } => EffectMask::IO.add(Effect::Alloc).add(Effect::Control),
            Self::End { .. } => EffectMask::WRITE.add(Effect::Control),
        }
    }
    pub fn operands(&self) -> Vec<&StringInvokeOperand> {
        match self {
            Self::Add { left, right } | Self::Compare { left, right, .. } => vec![left, right],
            Self::IndexOf { value, needle } => vec![value, needle],
            Self::Materialize { value } | Self::Length { value } | Self::Substring { value, .. } | Self::Print { value } => vec![value],
            Self::End { .. } => vec![],
        }
    }
    pub fn used_values(&self) -> Vec<ValueId> {
        match self {
            Self::End { value } => vec![*value],
            _ => self.operands().into_iter().filter_map(StringInvokeOperand::value)
                .chain(match self { Self::Substring { start, end, .. } => vec![*start, *end], _ => vec![] }).collect(),
        }
    }
    pub fn rewrite_values(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::Add { left, right } | Self::Compare { left, right, .. } => {
                left.rewrite_value(&mut rewrite);
                right.rewrite_value(rewrite);
            }
            Self::IndexOf { value, needle } => { value.rewrite_value(&mut rewrite); needle.rewrite_value(rewrite); }
            Self::Substring { value, start, end } => { value.rewrite_value(&mut rewrite); rewrite(start); rewrite(end); }
            Self::Materialize { value } | Self::Length { value } | Self::Print { value } => value.rewrite_value(rewrite),
            Self::End { value } => rewrite(value),
        }
    }
    pub fn owns_normal_result(&self) -> bool {
        self.normal_result_kind() == Some(InvokeNormalResultKind::OwnedString)
    }
    pub fn valid_operand_domains(&self) -> bool {
        match self {
            Self::Add { left, .. } => left.is_text(),
            Self::Compare { left, right, .. } => left.is_text() && right.is_text(),
            Self::Materialize { value } | Self::Length { value } | Self::Substring { value, .. } => value.is_text(),
            Self::IndexOf { value, needle } => value.is_text() && needle.is_text(),
            Self::Print { .. } | Self::End { .. } => true,
        }
    }
}
