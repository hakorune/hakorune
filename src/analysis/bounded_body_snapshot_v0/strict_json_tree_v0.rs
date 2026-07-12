//! Generic immutable strict-JSON tree for the internal HHako bridge.
//!
//! This module deliberately knows nothing about ProgramV0 fields, tags,
//! operators, paths, budgets, or snapshots.  The existing strict parser owns
//! syntax validation; this layer only lowers its ordered recursive value into
//! a deterministic arena and exposes generic observations.

use super::strict_json::{parse_strict_json, StrictJsonValue};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct StrictJsonNodeIdV0(u32);

impl StrictJsonNodeIdV0 {
    pub(crate) fn from_i64(value: i64) -> Option<Self> {
        u32::try_from(value).ok().map(Self)
    }

    pub(crate) fn raw(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StrictJsonKindV0 {
    Null,
    Bool,
    I64,
    U64,
    F64,
    String,
    Array,
    Object,
}

#[derive(Clone, Debug, PartialEq)]
enum StrictJsonArenaNodeV0 {
    Null,
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
    Array(Vec<StrictJsonNodeIdV0>),
    Object(Vec<(String, StrictJsonNodeIdV0)>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StrictJsonArenaV0 {
    nodes: Vec<StrictJsonArenaNodeV0>,
    root: StrictJsonNodeIdV0,
}

impl StrictJsonArenaV0 {
    pub(crate) fn parse(input: &str) -> Result<Self, String> {
        Ok(Self::from_value(parse_strict_json(input)?))
    }

    fn from_value(value: StrictJsonValue) -> Self {
        let mut nodes = Vec::new();
        let root = Self::lower(value, &mut nodes);
        Self { nodes, root }
    }

    fn lower(value: StrictJsonValue, nodes: &mut Vec<StrictJsonArenaNodeV0>) -> StrictJsonNodeIdV0 {
        let id = StrictJsonNodeIdV0(nodes.len() as u32);
        nodes.push(StrictJsonArenaNodeV0::Null);
        let lowered = match value {
            StrictJsonValue::Null => StrictJsonArenaNodeV0::Null,
            StrictJsonValue::Bool(value) => StrictJsonArenaNodeV0::Bool(value),
            StrictJsonValue::I64(value) => StrictJsonArenaNodeV0::I64(value),
            StrictJsonValue::U64(value) => StrictJsonArenaNodeV0::U64(value),
            StrictJsonValue::F64(value) => StrictJsonArenaNodeV0::F64(value),
            StrictJsonValue::String(value) => StrictJsonArenaNodeV0::String(value),
            StrictJsonValue::Array(values) => StrictJsonArenaNodeV0::Array(
                values
                    .into_iter()
                    .map(|value| Self::lower(value, nodes))
                    .collect(),
            ),
            StrictJsonValue::Object(fields) => StrictJsonArenaNodeV0::Object(
                fields
                    .into_iter()
                    .map(|(key, value)| (key, Self::lower(value, nodes)))
                    .collect(),
            ),
        };
        nodes[id.raw() as usize] = lowered;
        id
    }

    pub(crate) fn root(&self) -> StrictJsonNodeIdV0 {
        self.root
    }

    pub(crate) fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn kind(&self, node: StrictJsonNodeIdV0) -> Option<StrictJsonKindV0> {
        self.node(node).map(|node| match node {
            StrictJsonArenaNodeV0::Null => StrictJsonKindV0::Null,
            StrictJsonArenaNodeV0::Bool(_) => StrictJsonKindV0::Bool,
            StrictJsonArenaNodeV0::I64(_) => StrictJsonKindV0::I64,
            StrictJsonArenaNodeV0::U64(_) => StrictJsonKindV0::U64,
            StrictJsonArenaNodeV0::F64(_) => StrictJsonKindV0::F64,
            StrictJsonArenaNodeV0::String(_) => StrictJsonKindV0::String,
            StrictJsonArenaNodeV0::Array(_) => StrictJsonKindV0::Array,
            StrictJsonArenaNodeV0::Object(_) => StrictJsonKindV0::Object,
        })
    }

    pub(crate) fn object_len(&self, node: StrictJsonNodeIdV0) -> Option<usize> {
        match self.node(node)? {
            StrictJsonArenaNodeV0::Object(fields) => Some(fields.len()),
            _ => None,
        }
    }

    pub(crate) fn object_key_at(&self, node: StrictJsonNodeIdV0, index: usize) -> Option<&str> {
        match self.node(node)? {
            StrictJsonArenaNodeV0::Object(fields) => fields.get(index).map(|(key, _)| key.as_str()),
            _ => None,
        }
    }

    pub(crate) fn object_value_at(
        &self,
        node: StrictJsonNodeIdV0,
        index: usize,
    ) -> Option<StrictJsonNodeIdV0> {
        match self.node(node)? {
            StrictJsonArenaNodeV0::Object(fields) => fields.get(index).map(|(_, value)| *value),
            _ => None,
        }
    }

    pub(crate) fn array_len(&self, node: StrictJsonNodeIdV0) -> Option<usize> {
        match self.node(node)? {
            StrictJsonArenaNodeV0::Array(values) => Some(values.len()),
            _ => None,
        }
    }

    pub(crate) fn array_at(
        &self,
        node: StrictJsonNodeIdV0,
        index: usize,
    ) -> Option<StrictJsonNodeIdV0> {
        match self.node(node)? {
            StrictJsonArenaNodeV0::Array(values) => values.get(index).copied(),
            _ => None,
        }
    }

    pub(crate) fn string_value(&self, node: StrictJsonNodeIdV0) -> Option<&str> {
        match self.node(node)? {
            StrictJsonArenaNodeV0::String(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn bool_value(&self, node: StrictJsonNodeIdV0) -> Option<bool> {
        match self.node(node)? {
            StrictJsonArenaNodeV0::Bool(value) => Some(*value),
            _ => None,
        }
    }

    pub(crate) fn i64_value(&self, node: StrictJsonNodeIdV0) -> Option<i64> {
        match self.node(node)? {
            StrictJsonArenaNodeV0::I64(value) => Some(*value),
            _ => None,
        }
    }

    pub(crate) fn u64_value(&self, node: StrictJsonNodeIdV0) -> Option<u64> {
        match self.node(node)? {
            StrictJsonArenaNodeV0::U64(value) => Some(*value),
            _ => None,
        }
    }

    fn node(&self, node: StrictJsonNodeIdV0) -> Option<&StrictJsonArenaNodeV0> {
        self.nodes.get(node.raw() as usize)
    }
}
