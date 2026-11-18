//! Neutral MIR edge-argument vocabulary.
//!
//! CFG consumers own this transport independently of JoinModule lowering.

use crate::mir::ValueId;

/// Layout of values carried by one MIR control-flow edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JumpArgsLayout {
    /// Edge arguments contain carrier values only.
    CarriersOnly,
    /// The expression result precedes the carrier values.
    ExprResultPlusCarriers,
}

/// Arguments carried by one MIR control-flow edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeArgs {
    pub layout: JumpArgsLayout,
    pub values: Vec<ValueId>,
}

impl EdgeArgs {
    /// Both structural edge spellings carry zero values.
    pub(crate) fn absent_or_empty_carriers(args: &Option<Self>) -> bool {
        args.as_ref().is_none_or(Self::is_empty_carriers)
    }

    /// The canonical empty marker carries no value or ownership responsibility.
    pub(crate) fn is_empty_carriers(&self) -> bool {
        self.layout == JumpArgsLayout::CarriersOnly && self.values.is_empty()
    }
}
