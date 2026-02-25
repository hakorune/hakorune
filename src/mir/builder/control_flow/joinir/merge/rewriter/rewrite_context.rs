//! Rewrite Context - State Consolidation
//!
//! Phase 286C-3: Consolidated state for instruction rewriting.
//! Reduces scattered variables in merge_and_rewrite() into a single coherent structure.

use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

/// Consolidated state for instruction rewriting during JoinIR→MIR merge
pub(in crate::mir::builder::control_flow::joinir::merge) struct RewriteContext<'a> {
    /// Exit PHI inputs: Vec<(from_block, exit_value)>
    pub exit_phi_inputs: Vec<(BasicBlockId, ValueId)>,

    /// Carrier PHI inputs: Map<carrier_name, Vec<(from_block, value)>>
    pub carrier_inputs: BTreeMap<String, Vec<(BasicBlockId, ValueId)>>,

    /// Function entry map: Map<func_name, entry_block_id>
    pub function_entry_map: BTreeMap<String, BasicBlockId>,

    /// Skipped continuation entry redirects: Map<old_entry_block, exit_block_id>
    pub skipped_entry_redirects: BTreeMap<BasicBlockId, BasicBlockId>,

    /// Remapped exit values for DirectValue mode: Map<carrier_name, host_value_id>
    pub remapped_exit_values: BTreeMap<String, ValueId>,

    /// The exit block ID (allocated by block_allocator)
    pub exit_block_id: BasicBlockId,

    /// Optional boundary (for exit value collection, carrier management)
    pub boundary: Option<&'a JoinInlineBoundary>,

    /// Debug flag
    pub debug: bool,

    /// Verbose flag (debug || joinir_dev_enabled)
    pub verbose: bool,

    /// Strict exit mode
    pub strict_exit: bool,
}

impl<'a> RewriteContext<'a> {
    /// Create a new RewriteContext
    pub fn new(
        exit_block_id: BasicBlockId,
        boundary: Option<&'a JoinInlineBoundary>,
        debug: bool,
    ) -> Self {
        let verbose = debug || crate::config::env::joinir_dev_enabled();
        let strict_exit = crate::config::env::joinir_strict_enabled() || crate::config::env::joinir_dev_enabled();

        Self {
            exit_phi_inputs: Vec::new(),
            carrier_inputs: BTreeMap::new(),
            function_entry_map: BTreeMap::new(),
            skipped_entry_redirects: BTreeMap::new(),
            remapped_exit_values: BTreeMap::new(),
            exit_block_id,
            boundary,
            debug,
            verbose,
            strict_exit,
        }
    }

    /// Add an exit PHI input
    pub fn add_exit_phi_input(&mut self, from_block: BasicBlockId, value: ValueId) {
        self.exit_phi_inputs.push((from_block, value));
    }

    /// Add a carrier input
    pub fn add_carrier_input(&mut self, carrier_name: String, from_block: BasicBlockId, value: ValueId) {
        self.carrier_inputs
            .entry(carrier_name)
            .or_insert_with(Vec::new)
            .push((from_block, value));
    }

    /// Register a function entry block
    pub fn register_function_entry(&mut self, func_name: String, entry_block: BasicBlockId) {
        self.function_entry_map.insert(func_name, entry_block);
    }

    /// Register a skipped continuation redirect
    pub fn register_skipped_redirect(&mut self, old_entry: BasicBlockId, new_target: BasicBlockId) {
        self.skipped_entry_redirects.insert(old_entry, new_target);
    }

    /// Set a remapped exit value (DirectValue mode)
    pub fn set_remapped_exit_value(&mut self, carrier_name: String, host_value: ValueId) {
        self.remapped_exit_values.insert(carrier_name, host_value);
    }
}
