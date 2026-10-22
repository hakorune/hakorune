//! Physical Dynamic invocation operands and the source-issued call shape.
//!
//! The opaque carrier is a separate value class from tagged Dynamic input.
//! Runtime dispatch remains responsible for exact provider selection.

use crate::mir::ValueId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynamicInvocationOperandV1 {
    Null,
    Integer(ValueId),
    Bool(ValueId),
    TaggedDynamic(ValueId),
    Carrier(ValueId),
}

impl serde::Serialize for DynamicInvocationOperandV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = match self {
            Self::Null => serde_json::json!({"kind":"null"}),
            Self::Integer(value) => serde_json::json!({"kind":"integer", "value":value.as_u32()}),
            Self::Bool(value) => serde_json::json!({"kind":"bool", "value":value.as_u32()}),
            Self::TaggedDynamic(value) => {
                serde_json::json!({"kind":"tagged_dynamic", "value":value.as_u32()})
            }
            Self::Carrier(value) => {
                serde_json::json!({"kind":"dynamic_carrier", "value":value.as_u32()})
            }
        };
        serde::Serialize::serialize(&value, serializer)
    }
}

impl DynamicInvocationOperandV1 {
    pub fn value(&self) -> Option<ValueId> {
        match self {
            Self::Null => None,
            Self::Integer(value)
            | Self::Bool(value)
            | Self::TaggedDynamic(value)
            | Self::Carrier(value) => Some(*value),
        }
    }

    pub fn rewrite_value(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::Null => {}
            Self::Integer(value)
            | Self::Bool(value)
            | Self::TaggedDynamic(value)
            | Self::Carrier(value) => rewrite(value),
        }
    }
}

/// A selector and ordered actuals retained from one admitted source call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicInvocationOperationV1 {
    selector: Box<str>,
    receiver: DynamicInvocationOperandV1,
    arguments: Box<[DynamicInvocationOperandV1]>,
}

impl DynamicInvocationOperationV1 {
    pub(crate) fn from_source(
        source: &crate::mir::dynamic_invocation_contract::DynamicInvocationSourceLoanV1,
        receiver: DynamicInvocationOperandV1,
        arguments: Vec<DynamicInvocationOperandV1>,
    ) -> Result<Self, String> {
        let target = source.target();
        if arguments.len() != target.dispatch().arity() as usize
            || arguments.len() != source.argument_sites().len()
        {
            return Err("[freeze:contract][dynamic-invocation/arity-drift]".into());
        }
        Ok(Self {
            selector: target.dispatch().selector().into(),
            receiver,
            arguments: arguments.into_boxed_slice(),
        })
    }

    pub(crate) fn selector(&self) -> &str {
        &self.selector
    }
    pub(crate) fn receiver(&self) -> &DynamicInvocationOperandV1 {
        &self.receiver
    }
    pub(crate) fn arguments(&self) -> &[DynamicInvocationOperandV1] {
        &self.arguments
    }

    pub(crate) fn used_values(&self) -> Vec<ValueId> {
        std::iter::once(&self.receiver)
            .chain(self.arguments.iter())
            .filter_map(DynamicInvocationOperandV1::value)
            .collect()
    }

    pub(crate) fn rewrite_values(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        self.receiver.rewrite_value(&mut rewrite);
        self.arguments
            .iter_mut()
            .for_each(|argument| argument.rewrite_value(&mut rewrite));
    }
}
