//! Phase 286C-5: CarrierInputsCollector - DRY extraction for carrier_inputs collection
//!
//! This module extracts duplicated carrier_inputs collection logic from:
//! - Return terminator fallback path (lines 740-763)
//! - ExitJump skippable continuation path (lines 878-909)
//!
//! # Design Philosophy
//!
//! - **DRY Principle**: Single source of truth for carrier_inputs collection
//! - **Box Theory**: Encapsulates the carrier PHI fallback logic
//! - **Single Responsibility**: Only collects carrier inputs from header PHIs
//!
//! # Usage
//!
//! ```ignore
//! let collector = CarrierInputsCollector::new(boundary, loop_header_phi_info);
//! let inputs = collector.collect(new_block_id);
//! for (carrier_name, block_id, value_id) in inputs {
//!     result.carrier_inputs
//!         .entry(carrier_name)
//!         .or_insert_with(Vec::new)
//!         .push((block_id, value_id));
//! }
//! ```

use crate::mir::join_ir::lowering::carrier_info::{CarrierRole, ExitReconnectMode};
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlockId, ValueId};

use super::super::loop_header_phi_info::LoopHeaderPhiInfo;

/// Phase 286C-5: Box for collecting carrier inputs from loop header PHIs
///
/// This box implements the fallback logic for collecting carrier values
/// when edge-args are not available. It queries the loop header PHI info
/// to get the PHI dst for each carrier.
///
/// # Contract
///
/// - Filters out ConditionOnly carriers (they don't need PHI inputs)
/// - Uses header PHI dst if available
/// - Falls back to host_slot for DirectValue mode
/// - Returns Vec<(carrier_name, block_id, value_id)> for easy insertion
pub struct CarrierInputsCollector<'a> {
    boundary: &'a JoinInlineBoundary,
    loop_header_phi_info: &'a LoopHeaderPhiInfo,
}

impl<'a> CarrierInputsCollector<'a> {
    /// Create a new CarrierInputsCollector
    pub fn new(
        boundary: &'a JoinInlineBoundary,
        loop_header_phi_info: &'a LoopHeaderPhiInfo,
    ) -> Self {
        Self {
            boundary,
            loop_header_phi_info,
        }
    }

    /// Collect carrier inputs for a given block
    ///
    /// # Arguments
    ///
    /// * `block_id` - The source block for these carrier inputs
    ///
    /// # Returns
    ///
    /// Vec<(carrier_name, block_id, value_id)> for each carrier binding
    ///
    /// # Logic
    ///
    /// For each exit_binding:
    /// 1. Skip ConditionOnly carriers (they don't participate in exit PHIs)
    /// 2. Try to get carrier value from header PHI dst
    /// 3. If no PHI dst and DirectValue mode, fall back to host_slot
    pub fn collect(&self, block_id: BasicBlockId) -> Vec<(String, BasicBlockId, ValueId)> {
        let mut result = Vec::new();

        for binding in &self.boundary.exit_bindings {
            // Skip ConditionOnly carriers
            if binding.role == CarrierRole::ConditionOnly {
                continue;
            }

            // Try to get carrier value from header PHI
            if let Some(phi_dst) = self.loop_header_phi_info.get_carrier_phi(&binding.carrier_name) {
                result.push((binding.carrier_name.clone(), block_id, phi_dst));
            } else if self.boundary.exit_reconnect_mode == ExitReconnectMode::DirectValue {
                // DirectValue fallback: use host_slot
                result.push((binding.carrier_name.clone(), block_id, binding.host_slot));
            }
        }

        result
    }
}
