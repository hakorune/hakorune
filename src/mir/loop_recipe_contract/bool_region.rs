//! Closed, typed Bool expression tree. Internal short-circuit merges do not
//! write Loop bindings or invent Loop exits. Imports are ordinary value uses;
//! source correspondence is sealed separately by the same checked-frame owner.
use serde::{Deserialize, Serialize};
use super::schema_v2::LoopValueClassV2 as Class;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoopBoolCompareV1 { Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoopBoolCompareDomainV1 { I64, Text, DynamicI64 }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum LoopBoolNodeV1 {
    Import { ordinal: u32, class: Class },
    I64 { value: i64 },
    Bool { value: bool },
    Text { value: String },
    TextLength { value: u32 },
    TextSubstring { value: u32, start: u32, end: u32 },
    TextIndexOf { value: u32, needle: u32 },
    AddI64 { left: u32, right: u32 },
    Compare { op: LoopBoolCompareV1, domain: LoopBoolCompareDomainV1, left: u32, right: u32 },
    And { left: u32, right: u32 },
    Or { left: u32, right: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopBoolRegionV1 {
    /// Canonical postorder; each non-root node has exactly one parent.
    pub(crate) nodes: Vec<LoopBoolNodeV1>,
    pub(crate) root: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopBoolRegionRejectV1 {
    Root, ChildOrder, SharedChild, Orphan, Postorder, ImportOrder, ImportClass, ImportCoverage, Domain,
}
impl LoopBoolNodeV1 {
    fn children(&self) -> impl DoubleEndedIterator<Item = u32> {
        let children = match self {
            Self::TextLength { value } => [Some(*value), None, None],
            Self::TextSubstring { value, start, end } => [Some(*value), Some(*start), Some(*end)],
            Self::TextIndexOf { value, needle } => [Some(*value), Some(*needle), None],
            Self::AddI64 { left, right } | Self::Compare { left, right, .. }
            | Self::And { left, right } | Self::Or { left, right } => [Some(*left), Some(*right), None],
            _ => [None; 3],
        };
        children.into_iter().flatten()
    }
}
impl LoopBoolRegionV1 {
    pub(crate) fn may_fault(&self) -> bool {
        self.nodes.iter().any(|node| matches!(node,
            LoopBoolNodeV1::Compare { domain: LoopBoolCompareDomainV1::Text | LoopBoolCompareDomainV1::DynamicI64, .. }
            | LoopBoolNodeV1::TextLength { .. } | LoopBoolNodeV1::TextSubstring { .. }
            | LoopBoolNodeV1::TextIndexOf { .. }))
    }

    /// No Call/field or opaque owned subtree can hide behind a local node kind.
    /// Closed Core nodes retain their exact typed children; source loans and
    /// acquisition/End scopes are checked by the original source owner.
    /// Each import ordinal names exactly one supplied outer value class.
    pub(crate) fn verify(&self, inputs: &[Class]) -> Result<(), LoopBoolRegionRejectV1> {
        use LoopBoolRegionRejectV1 as Reject;
        if self.nodes.is_empty() || self.root as usize != self.nodes.len() - 1 { return Err(Reject::Root); }
        let mut classes = Vec::with_capacity(self.nodes.len());
        let mut parents = vec![0u8; self.nodes.len()];
        let mut imports = 0;
        for (index, node) in self.nodes.iter().enumerate() {
            for child in node.children() {
                if child as usize >= index { return Err(Reject::ChildOrder); }
                let count = &mut parents[child as usize];
                if *count != 0 { return Err(Reject::SharedChild); }
                *count = 1;
            }
            let class = match node {
                LoopBoolNodeV1::Import { ordinal, class } => {
                    if *ordinal as usize != imports { return Err(Reject::ImportOrder); }
                    if !matches!(class, Class::I64 | Class::Bool | Class::Text | Class::Dynamic)
                        || inputs.get(imports) != Some(class) { return Err(Reject::ImportClass); }
                    imports += 1;
                    *class
                }
                LoopBoolNodeV1::I64 { .. } => Class::I64,
                LoopBoolNodeV1::Bool { .. } => Class::Bool,
                LoopBoolNodeV1::Text { .. } => Class::Text,
                LoopBoolNodeV1::TextLength { value } => {
                    if classes[*value as usize] != Class::Text { return Err(Reject::Domain); }
                    Class::I64
                }
                LoopBoolNodeV1::TextSubstring { value, start, end } => {
                    if classes[*value as usize] != Class::Text || classes[*start as usize] != Class::I64
                        || classes[*end as usize] != Class::I64 { return Err(Reject::Domain); }
                    Class::Text
                }
                LoopBoolNodeV1::TextIndexOf { value, needle } => {
                    if classes[*value as usize] != Class::Text || classes[*needle as usize] != Class::Text { return Err(Reject::Domain); }
                    Class::I64
                }
                LoopBoolNodeV1::AddI64 { left, right } => {
                    if classes[*left as usize] != Class::I64 || classes[*right as usize] != Class::I64 { return Err(Reject::Domain); }
                    Class::I64
                }
                LoopBoolNodeV1::Compare { domain, left, right, .. } => {
                    let (expected_left, expected_right) = match domain {
                        LoopBoolCompareDomainV1::I64 => (Class::I64, Class::I64),
                        LoopBoolCompareDomainV1::Text => (Class::Text, Class::Text),
                        LoopBoolCompareDomainV1::DynamicI64 => (Class::Dynamic, Class::I64),
                    };
                    if classes[*left as usize] != expected_left || classes[*right as usize] != expected_right {
                        return Err(Reject::Domain);
                    }
                    Class::Bool
                }
                LoopBoolNodeV1::And { left, right } | LoopBoolNodeV1::Or { left, right } => {
                    if classes[*left as usize] != Class::Bool || classes[*right as usize] != Class::Bool { return Err(Reject::Domain); }
                    Class::Bool
                }
            };
            classes.push(class);
        }
        if imports != inputs.len() { return Err(Reject::ImportCoverage); }
        if classes.last() != Some(&Class::Bool) { return Err(Reject::Root); }
        if parents[..self.nodes.len() - 1].iter().any(|count| *count != 1) { return Err(Reject::Orphan); }
        // Check canonical LEFT -> RIGHT postorder structurally. The node arena
        // is never an eager execution schedule for conditional RHS children.
        let mut stack = vec![(self.root, false)];
        let mut next = 0usize;
        while let Some((node, visited)) = stack.pop() {
            let mut children = self.nodes[node as usize].children().rev().peekable();
            if !visited && children.peek().is_some() {
                stack.push((node, true));
                stack.extend(children.map(|child| (child, false)));
            } else {
                if node as usize != next { return Err(Reject::Postorder); }
                next += 1;
            }
        }
        if next != self.nodes.len() { return Err(Reject::Postorder); }
        Ok(())
    }
}
