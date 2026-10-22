//! Physical expression operands; source loans supply the exact original bytes,
//! scalar domain, or tagged Dynamic carrier. Physical tags never infer source permission.
use super::InvokeNormalResultKind;
use crate::mir::{Effect, EffectMask, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringInvokeOperand {
    StaticText { utf8: String },
    OwnedString(ValueId),
    /// Readonly tagged Text formal/view; never an owned acquisition.
    BorrowedText(ValueId),
    /// Internal by-value Text descriptor selected by a typed loop PHI.
    /// This is neither a formal borrow nor an owned String Home.
    TextOperand(ValueId),
    NormalInteger(ValueId),
    /// Original tag/payload pair from a verified untyped formal; not Text.
    Dynamic(ValueId),
    /// Opaque result of a checked Dynamic invocation; never a tag/payload pair.
    DynamicCarrier(ValueId),
}
// One descriptor spelling for canonical transport and passive write census.
impl serde::Serialize for StringInvokeOperand {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = match self {
            Self::StaticText { utf8 } => serde_json::json!({"kind":"static_text", "utf8":utf8}),
            Self::BorrowedText(value) => serde_json::json!({"kind":"borrowed_text", "value":value.as_u32()}),
            Self::OwnedString(value) => serde_json::json!({"kind":"owned_string", "value":value.as_u32()}),
            Self::TextOperand(value) => serde_json::json!({"kind":"text_operand", "value":value.as_u32()}),
            Self::NormalInteger(value) => serde_json::json!({"kind":"normal_integer", "value":value.as_u32()}),
            Self::Dynamic(value) => serde_json::json!({"kind":"tagged_dynamic", "value":value.as_u32()}),
            Self::DynamicCarrier(value) => serde_json::json!({"kind":"dynamic_carrier", "value":value.as_u32()}),
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl StringInvokeOperand {
    pub fn value(&self) -> Option<ValueId> {
        match self {
            Self::StaticText { .. } => None,
            Self::OwnedString(value) | Self::BorrowedText(value) | Self::TextOperand(value)
                | Self::NormalInteger(value) | Self::Dynamic(value) => Some(*value),
            Self::DynamicCarrier(value) => Some(*value),
        }
    }
    pub fn rewrite_value(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::StaticText { .. } => {}
            Self::OwnedString(value) | Self::BorrowedText(value) | Self::TextOperand(value)
                | Self::NormalInteger(value) | Self::Dynamic(value) => rewrite(value),
            Self::DynamicCarrier(value) => rewrite(value),
        }
    }
    pub fn is_text(&self) -> bool {
        !matches!(self, Self::NormalInteger(_) | Self::Dynamic(_) | Self::DynamicCarrier(_))
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
    Substring { value: StringInvokeOperand, start: StringInvokeOperand, end: StringInvokeOperand },
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
                .chain(match self { Self::Substring { start, end, .. } => {
                    start.value().into_iter().chain(end.value()).collect()
                }, _ => vec![] }).collect(),
        }
    }
    pub fn rewrite_values(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::Add { left, right } | Self::Compare { left, right, .. } => {
                left.rewrite_value(&mut rewrite);
                right.rewrite_value(rewrite);
            }
            Self::IndexOf { value, needle } => { value.rewrite_value(&mut rewrite); needle.rewrite_value(rewrite); }
            Self::Substring { value, start, end } => {
                value.rewrite_value(&mut rewrite);
                start.rewrite_value(&mut rewrite);
                end.rewrite_value(rewrite);
            }
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
            Self::Compare { left, right, .. } => {
                (left.is_text() || matches!(left, StringInvokeOperand::DynamicCarrier(_)))
                    && (right.is_text() || matches!(right, StringInvokeOperand::DynamicCarrier(_)))
            }
            Self::Materialize { value } | Self::Length { value } => value.is_text(),
            Self::Substring { value, start, end } => value.is_text()
                && matches!(start, StringInvokeOperand::NormalInteger(_) | StringInvokeOperand::Dynamic(_))
                && matches!(end, StringInvokeOperand::NormalInteger(_) | StringInvokeOperand::Dynamic(_)),
            Self::IndexOf { value, needle } => value.is_text() && needle.is_text(),
            Self::Print { value } => !matches!(value, StringInvokeOperand::Dynamic(_) | StringInvokeOperand::DynamicCarrier(_)),
            Self::End { .. } => true,
        }
    }
}
