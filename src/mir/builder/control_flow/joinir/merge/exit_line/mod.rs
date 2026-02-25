//! Phase 33-10-Refactor: Exit Line Module
//!
//! Modularizes exit line handling (Phase 6 from merge/mod.rs) into focused Boxes.
//!
//! **Architecture**:
//! ```text
//! ExitLineOrchestrator (facade)
//!   ├── ExitMetaCollector (collects exit_bindings)
//!   └── ExitLineReconnector (updates variable_map)
//! ```
//!
//! # Phase 33-10 Refactoring Strategy
//!
//! This module exemplifies the **Box Theory Modularization** pattern:
//!
//! ## Single Responsibility
//! Each Box handles one concern:
//! - ExitLineReconnector: Updates variable_map with exit values
//! - ExitMetaCollector: Constructs exit_bindings from ExitMeta
//! - ExitLineOrchestrator: Orchestrates Phase 6 reconnection
//!
//! ## Why This Pattern?
//! Before modularization, reconnect_boundary() was a 87-line monolithic function
//! in merge/mod.rs. Extracting into Boxes enables:
//! - Unit testing individual concerns (connection vs collection)
//! - Reusability across pattern lowerers (Pattern 3, 4, etc.)
//! - Easier debugging (isolated responsibilities)
//! - Future optimization without touching merge/mod.rs
//!
//! ## Design Philosophy
//! Following Phase 33 Box Theory principles:
//! 1. Extract each major concern → separate Box
//! 2. Keep boundaries explicit (public/private, inputs/outputs)
//! 3. Maintain backward compatibility (no breaking changes)
//! 4. Enable independent testing and evolution
//!
//! ## Phase 33-13: Carrier PHI Integration
//! The ExitLineOrchestrator now receives carrier_phis from exit_phi_builder
//! and uses them in ExitLineReconnector instead of the remapped exit values.
//! This ensures variable_map points to PHI-defined values (SSA-correct).
//!
//! ## Future Extensions
//! When implementing Pattern 4 (continue), new pattern lowerers can:
//! ```rust
//! let exit_bindings = ExitMetaCollector::collect(self, &exit_meta, debug);
//! let boundary = JoinInlineBoundary::new_with_exits(...);
//! exit_line::ExitLineOrchestrator::execute(builder, &boundary, &carrier_phis, debug);
//! ```
//! No changes to exit_line module needed!

pub mod meta_collector;
pub mod reconnector;

pub use meta_collector::ExitMetaCollector;
pub use reconnector::ExitLineReconnector;

use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Phase 33-10-Refactor-P2: ExitLineOrchestrator facade
///
/// Coordinates the entire exit line reconnection process for Phase 6.
/// Acts as a single-entry-point for merge/mod.rs.
pub struct ExitLineOrchestrator;

impl ExitLineOrchestrator {
    /// Orchestrate the complete exit line reconnection
    ///
    /// # Inputs
    /// - builder: MirBuilder with variable_map to update
    /// - boundary: JoinInlineBoundary with exit_bindings
    /// - carrier_phis: Map from carrier name to PHI dst ValueId (Phase 33-13)
    /// - remapped_exit_values: Map from carrier name to remapped ValueId (Phase 131 P1.5)
    /// - debug: Debug logging enabled
    ///
    /// # Returns
    /// - Result<(), String>
    ///
    /// # Process
    /// 1. Validate exit_bindings (empty case)
    /// 2. Delegate to ExitLineReconnector with carrier_phis and remapped_exit_values
    ///
    /// # Phase 131 P1.5: DirectValue Mode Support
    /// When boundary.exit_reconnect_mode == DirectValue, uses remapped_exit_values instead of carrier_phis
    pub fn execute(
        builder: &mut crate::mir::builder::MirBuilder,
        boundary: &crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary,
        carrier_phis: &BTreeMap<String, ValueId>,
        remapped_exit_values: &BTreeMap<String, ValueId>, // Phase 131 P1.5
        debug: bool,
    ) -> Result<(), String> {
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();
        let verbose = debug || crate::config::env::joinir_dev_enabled();
        if verbose {
            trace.stderr_if(
                &format!(
                    "[joinir/exit-line] orchestrator start: {} carrier PHIs",
                    carrier_phis.len()
                ),
                verbose,
            );
        }

        // Phase 33-13 + Phase 131 P1.5: Delegate to ExitLineReconnector with carrier_phis and remapped_exit_values
        ExitLineReconnector::reconnect(builder, boundary, carrier_phis, remapped_exit_values, debug)?;

        if verbose {
            trace.stderr_if("[joinir/exit-line] orchestrator complete", verbose);
        }

        Ok(())
    }
}
