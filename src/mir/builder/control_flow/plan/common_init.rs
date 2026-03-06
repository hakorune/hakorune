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

    /// Check if carrier updates are allowed in current route shape
    ///
    /// Phase 188: Validates that all carrier updates use simple expressions.
    ///
    /// Accepts:
    /// - Const (numeric) - e.g., `i = i + 1`
    /// - Variable (numeric/string) - e.g., `sum = sum + i`
    /// - StringLiteral (Phase 188) - e.g., `s = s + "x"`
    ///
    /// Rejects:
    /// - Other (method calls, complex expressions) - e.g., `x = x + foo.bar()`
    ///
    /// # Arguments
    ///
    /// - `body`: Loop body AST nodes to analyze
    /// - `loop_var_name`: Loop variable name (for creating dummy carriers)
    /// - `variable_map`: Current variable mappings (for creating dummy carriers)
    ///
    /// # Returns
    ///
    /// - `true` if all carrier updates are allowed
    /// - `false` if any update uses complex expressions (UpdateRhs::Other)
    ///
    /// # Example
    ///
    /// ```rust
    /// if !CommonPatternInitializer::check_carrier_updates_allowed(
    ///     body,
    ///     &loop_var_name,
    ///     &builder.variable_map,
    /// ) {
    ///     return false; // Route cannot lower - reject
    /// }
    /// ```
    pub fn check_carrier_updates_allowed(
        body: &[ASTNode],
        _loop_var_name: &str,
        _variable_map: &BTreeMap<String, ValueId>,
    ) -> bool {
        use crate::mir::join_ir::lowering::carrier_info::CarrierVar;
        use crate::mir::join_ir::lowering::loop_update_analyzer::{
            LoopUpdateAnalyzer, UpdateExpr, UpdateRhs,
        };

        // Create dummy carriers from body assignment targets for analysis
        let dummy_carriers: Vec<CarrierVar> = body
            .iter()
            .filter_map(|node| {
                match node {
                    ASTNode::Assignment { target, .. } => {
                        if let ASTNode::Variable { name, .. } = target.as_ref() {
                            Some(CarrierVar {
                            name: name.clone(),
                            host_id: ValueId(0),  // Dummy
                            join_id: None,
                            role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState, // Phase 227: Default
                            init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228: Default
                        })
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            })
            .collect();

        let updates = LoopUpdateAnalyzer::analyze_carrier_updates(body, &dummy_carriers);

        // Phase 188: Check if any update is complex (reject only UpdateRhs::Other)
        // Phase 190: Allow NumberAccumulation pattern
        // Allow: Const (numeric), Variable (numeric/string), StringLiteral, NumberAccumulation
        // Reject: Other (method calls, nested BinOp)
        for update in updates.values() {
            if let UpdateExpr::BinOp { rhs, .. } = update {
                match rhs {
                    UpdateRhs::Const(_) => {
                        // Numeric: i = i + 1 (allowed)
                    }
                    UpdateRhs::Variable(_) => {
                        // Phase 188: StringAppendChar: s = s + ch (allowed)
                        // Or numeric: sum = sum + i (allowed)
                    }
                    UpdateRhs::StringLiteral(_) => {
                        // Phase 188: StringAppendLiteral: s = s + "x" (allowed)
                    }
                    UpdateRhs::NumberAccumulation { .. } => {
                        // Phase 190: Number accumulation: result = result * 10 + digit (allowed)
                    }
                    UpdateRhs::Other => {
                        // Phase 188: Complex update (method call, nested BinOp) - reject
                        crate::mir::builder::control_flow::joinir::trace::trace().dev(
                            "common_init/check_carriers",
                            "Phase 188: Complex update detected (UpdateRhs::Other), rejecting pattern",
                        );
                        return false;
                    }
                }
            }
        }

        true
    }
}
