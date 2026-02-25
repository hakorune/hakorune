//! RewrittenBlocks: Plan stage output (pure transformation)
//!
//! Plan stage generates new blocks and mappings WITHOUT touching the builder.

use crate::mir::{BasicBlock, BasicBlockId, ValueId};
use std::collections::BTreeMap;

/// Rewritten blocks: New blocks, PHI inputs, carrier inputs (ready to apply)
#[derive(Debug, Clone)]
pub struct RewrittenBlocks {
    /// New blocks to add to the builder
    pub new_blocks: Vec<BasicBlock>,

    /// Block replacements: Map<block_id, new_block>
    pub block_replacements: BTreeMap<BasicBlockId, BasicBlock>,

    /// PHI inputs for exit block: Vec<(from_block, value)>
    pub phi_inputs: Vec<(BasicBlockId, ValueId)>,

    /// Carrier inputs: Map<carrier_name, Vec<(from_block, value)>>
    pub carrier_inputs: BTreeMap<String, Vec<(BasicBlockId, ValueId)>>,
}
