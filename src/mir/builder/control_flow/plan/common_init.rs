//! Phase 33-22: Common Route Initializer
//!
//! Consolidates initialization logic shared by the 4 primary loop routes
//! (`loop_simple_while`, `loop_break`, `if_phi_join`, `loop_continue_only`).
//!
//! ## Responsibility
//!
//! - Extract loop variable from condition AST
//! - Build CarrierInfo from variable_map (delegates to `CarrierInfo::from_variable_map`)
//! - Support route-specific carrier exclusions (e.g., loop_break excludes break-triggered vars)
//!
//! ## Usage
//!
//! ```rust
//! let (loop_var_name, loop_var_id, carrier_info) =
//!     CommonPatternInitializer::initialize_pattern(
//!         builder,
//!         condition,
//!         &builder.variable_map,
//!         None, // No exclusions
//!     )?;
//! ```
//!
//! ## Benefits
//!
//! - **Single source of truth**: All routes use same initialization logic
//! - **Testability**: Can be tested independently
//! - **Maintainability**: Changes to initialization only need to happen once
//! - **Reduces duplication**: Eliminates 80 lines across the primary loop-route branches
//!
//! # Phase 183-2: Delegation to CarrierInfo
//!
//! This module is now a thin wrapper around `CarrierInfo::from_variable_map()`.
//! The primary logic lives in `carrier_info.rs` for consistency across MIR and JoinIR contexts.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::ValueId;
use std::collections::BTreeMap;

pub(crate) struct CommonPatternInitializer;

impl CommonPatternInitializer {
    /// Initialize route context: extract loop var, build CarrierInfo
    ///
    /// Returns: (loop_var_name, loop_var_id, carrier_info)
    ///
    /// This consolidates the initialization that was previously duplicated
    /// across the primary loop-route lowerers.
    ///
    /// # Arguments
    ///
    /// - `builder`: MirBuilder instance for extracting loop variable
    /// - `condition`: Loop condition AST node (e.g., `i < 10`)
    /// - `variable_map`: Current variable mappings (host ValueIds)
    /// - `exclude_carriers`: Optional list of variable names to exclude from carriers
    ///
    /// # Example
    ///
    /// ```rust
    /// // loop_simple_while / if_phi_join / loop_continue_only: no exclusions
    /// let (loop_var_name, loop_var_id, carrier_info) =
    ///     CommonPatternInitializer::initialize_pattern(
    ///         builder,
    ///         condition,
    ///         &builder.variable_map,
    ///         None,
    ///     )?;
    ///
    /// // loop_break: exclude break-triggered variables
    /// let (loop_var_name, loop_var_id, carrier_info) =
    ///     CommonPatternInitializer::initialize_pattern(
    ///         builder,
    ///         condition,
    ///         &builder.variable_map,
    ///         Some(&["break_flag"]),
    ///     )?;
    /// ```
    pub fn initialize_pattern(
        builder: &MirBuilder,
        condition: &ASTNode,
        variable_map: &BTreeMap<String, ValueId>,
        exclude_carriers: Option<&[&str]>,
    ) -> Result<(String, ValueId, CarrierInfo), String> {
        // Step 1: Extract loop variable from condition
        let loop_var_name = builder.extract_loop_variable_from_condition(condition)?;
        let loop_var_id = variable_map.get(&loop_var_name).copied().ok_or_else(|| {
            format!(
                "[common_init] Loop variable '{}' not found in variable_map",
                loop_var_name
            )
        })?;

        // Phase 183-2: Delegate to CarrierInfo::from_variable_map for consistency
        // Phase 222.5-D: Direct BTreeMap usage (no conversion needed)

        // Step 2: Use CarrierInfo::from_variable_map as primary initialization method
        let mut carrier_info = CarrierInfo::from_variable_map(loop_var_name.clone(), variable_map)?;

        // Step 3: Apply exclusions if provided (loop_break-specific)
        if let Some(excluded) = exclude_carriers {
            carrier_info
                .carriers
                .retain(|c| !excluded.contains(&c.name.as_str()));
        }

        Ok((loop_var_name, loop_var_id, carrier_info))
    }
}
