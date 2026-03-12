//! Phase 33-22: JoinIR Conversion Pipeline
//!
//! Unifies the conversion flow: JoinIR → MIR → Merge
//!
//! ## Responsibility
//!
//! - Convert JoinModule to MirModule
//! - Merge MirModule blocks into current function
//! - Handle boundary mapping and exit PHI generation
//! - **Phase 284 P1**: Handle return statements via return_collector SSOT
//!
//! ## Usage
//!
//! ```rust
//! let exit_phi_result = JoinIRConversionPipeline::execute(
//!     builder,
//!     join_module,
//!     Some(&boundary),
//!     "loop_simple",
//!     debug,
//! )?;
//! ```
//!
//! ## Benefits
//!
//! - **Single conversion path**: All routes use same JoinIR→MIR→Merge flow
//! - **Consistent error handling**: Unified error messages
//! - **Testability**: Can test conversion independently
//! - **Reduces duplication**: Eliminates 120 lines across loop route families
//! - **Phase 284 P1**: SSOT for return statement handling (not scattered in route lowerers)

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::join_ir::lowering::return_collector::{collect_return_from_body, ReturnInfo};
use crate::mir::join_ir::JoinModule;
use crate::mir::ValueId;
use std::collections::BTreeMap;

pub(crate) struct JoinIRConversionPipeline;

impl JoinIRConversionPipeline {
    /// Execute unified conversion pipeline
    ///
    /// Flow: JoinModule → MirModule → merge_joinir_mir_blocks
    ///
    /// # Arguments
    ///
    /// - `builder`: MirBuilder instance for merging blocks
    /// - `join_module`: JoinIR module to convert
    /// - `boundary`: Optional boundary mapping for input/output values
    /// - `route_label`: Label for debug messages (e.g., "loop_simple", "loop_break")
    /// - `debug`: Enable debug output
    ///
    /// # Returns
    ///
    /// - `Ok(Some(ValueId))`: Exit PHI result if boundary has exit bindings
    /// - `Ok(None)`: No exit PHI generated (simple loops)
    /// - `Err(String)`: Conversion or merge failure
    ///
    /// # Example
    ///
    /// ```rust
    /// // loop_simple_while: simple loop (no exit PHI)
    /// let _ = JoinIRConversionPipeline::execute(
    ///     builder,
    ///     join_module,
    ///     Some(&boundary),
    ///     "loop_simple",
    ///     false,
    /// )?;
    ///
    /// // if_phi_join: loop with carriers (exit PHI generated)
    /// let exit_phi = JoinIRConversionPipeline::execute(
    ///     builder,
    ///     join_module,
    ///     Some(&boundary),
    ///     "if_phi_join",
    ///     true,
    /// )?;
    /// ```
    pub fn execute(
        builder: &mut MirBuilder,
        join_module: JoinModule,
        boundary: Option<&JoinInlineBoundary>,
        route_label: &str,
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        // Phase 284 P1: Delegate to execute_with_body with None (backward compatibility)
        Self::execute_with_body(builder, join_module, boundary, route_label, debug, None)
    }

    /// Execute unified conversion pipeline with optional body for return detection
    ///
    /// Phase 284 P1: This is the SSOT for return statement handling.
    /// Route lowerers should call this with body to enable return detection.
    ///
    /// # Arguments
    ///
    /// - `builder`: MirBuilder instance for merging blocks
    /// - `join_module`: JoinIR module to convert
    /// - `boundary`: Optional boundary mapping for input/output values
    /// - `route_label`: Label for debug messages (e.g., "loop_simple", "loop_continue")
    /// - `debug`: Enable debug output
    /// - `body`: Optional loop body for return detection (Phase 284 P1)
    ///
    /// # Returns
    ///
    /// - `Ok(Some(ValueId))`: Exit PHI result or return value
    /// - `Ok(None)`: No exit PHI generated (simple loops without return)
    /// - `Err(String)`: Conversion or merge failure, or unsupported return route shape
    pub fn execute_with_body(
        builder: &mut MirBuilder,
        join_module: JoinModule,
        boundary: Option<&JoinInlineBoundary>,
        route_label: &str,
        debug: bool,
        body: Option<&[ASTNode]>,
    ) -> Result<Option<ValueId>, String> {
        // Phase 284 P1: Check for return statements in body (SSOT)
        let return_info = if let Some(body) = body {
            match collect_return_from_body(body) {
                Ok(info) => info,
                Err(e) => {
                    return Err(format!(
                        "[{}/pipeline] Return detection failed: {}",
                        route_label, e
                    ))
                }
            }
        } else {
            None
        };

        // Phase 284 P1: If return found, generate Return terminator
        if let Some(ret_info) = &return_info {
            // For now, we generate a Return terminator with the literal value
            // This is added after the normal loop processing
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[{}/pipeline] Phase 284 P1: Return detected with value {}",
                    route_label, ret_info.value
                ));
            }
        }
        use crate::mir::builder::control_flow::joinir::trace;
        use crate::mir::join_ir::frontend::JoinFuncMetaMap;
        use crate::mir::join_ir_vm_bridge::bridge_joinir_to_mir_with_meta;

        // Step 1: Log JoinIR stats (functions and blocks)
        trace::trace().joinir_stats(
            route_label,
            join_module.functions.len(),
            join_module.functions.values().map(|f| f.body.len()).sum(),
        );

        // Step 1.5: Run all pipeline contract checks (Phase 256 P1.5-DBG + P1.6)
        if let Some(boundary) = boundary {
            use crate::mir::builder::control_flow::joinir::merge::run_all_pipeline_checks;
            run_all_pipeline_checks(&join_module, boundary)?;
        }

        // Step 2: JoinModule → MirModule conversion
        // Phase 256 P1.5: Pass boundary to bridge for ValueId remapping
        let empty_meta: JoinFuncMetaMap = BTreeMap::new();
        let mir_module = bridge_joinir_to_mir_with_meta(&join_module, &empty_meta, boundary)
            .map_err(|e| format!("[{}/pipeline] MIR conversion failed: {:?}", route_label, e))?;

        // Task 3.1-2: Dump bridge output for diagnosis (dev-only)
        if crate::config::env::is_joinir_debug() {
            use crate::mir::printer::MirPrinter;
            use std::io::Write;

            let mir_text = MirPrinter::new().print_module(&mir_module);
            if let Ok(mut file) = std::fs::File::create("/tmp/joinir_bridge_split.mir") {
                let _ = writeln!(file, "; Bridge output for {}", route_label);
                let _ = writeln!(file, "; JoinIR → MIR conversion (before merge)\n");
                let _ = write!(file, "{}", mir_text);
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[trace:bridge] Dumped bridge MIR to /tmp/joinir_bridge_split.mir"
                ));
            }
        }

        // Step 3: Log MIR stats (functions and blocks)
        trace::trace().joinir_stats(
            route_label,
            mir_module.functions.len(),
            mir_module.functions.values().map(|f| f.blocks.len()).sum(),
        );

        // Step 4: Merge into current function
        let exit_phi_result = builder.merge_joinir_mir_blocks(&mir_module, boundary, debug)?;

        // Phase 284 P1: Log return detection (actual handling is in JoinIR lowerer)
        // The JoinIR lowerer should have already processed the return and added JoinInst::Ret
        // to the JoinModule. The bridge converts JoinInst::Ret to MIR Return terminator.
        if return_info.is_some() && debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[{}/pipeline] Phase 284 P1: Return was detected in body (processed by JoinIR lowerer)",
                route_label
            ));
        }

        Ok(exit_phi_result)
    }

    /// Get return info from loop body (Phase 284 P1 SSOT)
    ///
    /// This is the SSOT for return detection. Route lowerers should use this
    /// before constructing JoinModule to know if return handling is needed.
    pub fn detect_return(body: &[ASTNode]) -> Result<Option<ReturnInfo>, String> {
        collect_return_from_body(body)
    }
}
