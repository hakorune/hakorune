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
