//! Phase 118: ExitArgsCollectorBox - Collects exit values from JoinIR jump_args
//!
//! This module provides a box-based abstraction for collecting exit PHI inputs
//! and carrier inputs from JoinIR jump_args during the merge phase.
//!
//! # Design Philosophy
//!
//! - **Single Responsibility**: Only collects and classifies jump_args into exit/carrier values
//! - **SSOT**: offset calculation logic centralized here
//! - **Fail-Fast**: Validates jump_args length against exit_bindings
//! - **Box Theory**: Encapsulates the complex jump_args → PHI inputs mapping
//!
//! # Usage
//!
//! ```ignore
//! let collector = ExitArgsCollectorBox::new();
//! let result = collector.collect(
//!     &boundary.exit_bindings,
//!     &remapped_args,
//!     block_id,
//!     strict_mode,
//!     boundary.jump_args_layout,
//! )?;
//! exit_phi_inputs.push((result.block_id, result.expr_result_value));
//! carrier_inputs.extend(result.carrier_values);
//! ```

use crate::mir::join_ir::lowering::carrier_info::CarrierRole;
use crate::mir::join_ir::lowering::inline_boundary::{JumpArgsLayout, LoopExitBinding};
use crate::mir::{BasicBlockId, ValueId};
#[cfg(test)]
use std::collections::BTreeMap;

/// Phase 118: Result of exit args collection
///
/// This structure separates the expr result (first jump arg) from carrier values,
/// following the JoinFragmentMeta philosophy.
#[derive(Debug, Clone)]
pub struct ExitArgsCollectionResult {
    /// The source block for these values
    pub block_id: BasicBlockId,
    /// The expression result value (jump_args[0], optional)
    pub expr_result_value: Option<ValueId>,
    /// Carrier values: (carrier_name, (block_id, exit_value))
    pub carrier_values: Vec<(String, (BasicBlockId, ValueId))>,
}

/// Phase 118: Box for collecting exit values from jump_args
///
/// This box encapsulates the logic for mapping JoinIR jump_args to exit PHI inputs
/// and carrier PHI inputs, based on the exit_bindings contract.
///
/// # Phase 118 P2 Contract
///
/// - Every LoopState carrier in exit_bindings must have an exit PHI input
/// - jump_args order is assumed to match exit_bindings order
/// - Layout is enforced via JumpArgsLayout (no inference)
/// - Extra trailing args are treated as invariants and ignored
/// - This avoids if_phi_join-specific assumptions (legacy Pattern 3, traceability-only), such as
///   "jump_args[0] is loop_var"
///
/// # Fail-Fast Guarantee
///
/// - Returns error if jump_args is too short (missing required carriers)
/// - In strict mode, returns error for length mismatches
/// - In non-strict mode, logs warnings and continues with best effort
pub struct ExitArgsCollectorBox;

impl ExitArgsCollectorBox {
    /// Create a new ExitArgsCollectorBox
    pub fn new() -> Self {
        Self
    }

    /// Collect exit values from jump_args
    ///
    /// # Arguments
    ///
    /// * `exit_bindings` - Exit bindings from JoinInlineBoundary (SSOT)
    /// * `remapped_args` - Remapped jump_args from JoinIR block (already in host value space)
    /// * `block_id` - Source block ID for PHI inputs
    /// * `strict_exit` - If true, Fail-Fast on any validation error
    /// * `layout` - jump_args layout policy (SSOT from boundary)
    ///
    /// # Returns
    ///
    /// * `Ok(ExitArgsCollectionResult)` - Successfully collected exit values
    /// * `Err(String)` - Validation failed (Fail-Fast in strict mode)
    ///
    /// # Phase 118 P2: SSOT Offset Calculation
    ///
    /// The offset is determined by JumpArgsLayout:
    /// - `CarriersOnly`: offset = 0
    /// - `ExprResultPlusCarriers`: offset = 1
    /// - Length mismatches are errors (or warnings in non-strict mode)
    pub fn collect(
        &self,
        exit_bindings: &[LoopExitBinding],
        remapped_args: &[ValueId],
        block_id: BasicBlockId,
        strict_exit: bool,
        layout: JumpArgsLayout,
    ) -> Result<ExitArgsCollectionResult, String> {
        // Filter exit bindings to get only LoopState carriers (skip ConditionOnly)
        let exit_phi_bindings: Vec<_> = exit_bindings
            .iter()
            .filter(|eb| eb.role != CarrierRole::ConditionOnly)
            .collect();

        // Early return if no exit PHI bindings
        if exit_phi_bindings.is_empty() {
            return Ok(ExitArgsCollectionResult {
                block_id,
                expr_result_value: None,
                carrier_values: Vec::new(),
            });
        }

        // Phase 118 P2: Calculate offset (SSOT)
        let offset =
            self.calculate_offset(remapped_args.len(), exit_phi_bindings.len(), block_id, strict_exit, layout)?;

        // Collect expr result (first jump arg if offset > 0)
        let expr_result_value = if layout == JumpArgsLayout::ExprResultPlusCarriers && offset > 0 {
            remapped_args.first().copied()
        } else {
            None
        };

        // Collect carrier values
        let mut carrier_values = Vec::new();
        for (binding_idx, binding) in exit_phi_bindings.iter().enumerate() {
            let jump_args_idx = offset + binding_idx;
            if let Some(&carrier_exit) = remapped_args.get(jump_args_idx) {
                carrier_values.push((
                    binding.carrier_name.clone(),
                    (block_id, carrier_exit),
                ));
            } else {
                // Missing carrier value - Fail-Fast in strict mode
                let msg = format!(
                    "[joinir/exit-line] Missing jump_args entry for exit_binding carrier '{}' at index {} in block {:?}",
                    binding.carrier_name, jump_args_idx, block_id
                );
                if strict_exit {
                    return Err(msg);
                } else {
                    if crate::config::env::is_joinir_debug() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!("[DEBUG-177] {}", msg));
                    }
                }
            }
        }

        Ok(ExitArgsCollectionResult {
            block_id,
            expr_result_value,
            carrier_values,
        })
    }

    /// Phase 118 P2: Calculate offset for jump_args indexing (SSOT)
    ///
    /// # Returns
    ///
    /// * `Ok(0)` - Direct mapping (no offset)
    /// * `Ok(1)` - Legacy layout (one extra slot)
    /// * `Err(String)` - Invalid length (Fail-Fast in strict mode)
    fn calculate_offset(
        &self,
        jump_args_len: usize,
        exit_phi_bindings_len: usize,
        block_id: BasicBlockId,
        strict_exit: bool,
        layout: JumpArgsLayout,
    ) -> Result<usize, String> {
        if jump_args_len < exit_phi_bindings_len {
            // Too short - missing carriers
            let msg = format!(
                "[joinir/exit-line] jump_args too short: need {} carrier args (from exit_bindings) but got {} in block {:?}",
                exit_phi_bindings_len, jump_args_len, block_id
            );
            if strict_exit {
                Err(msg)
            } else {
                if crate::config::env::is_joinir_debug() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!("[DEBUG-177] {}", msg));
                }
                Ok(0) // Best effort: try direct mapping
            }
        } else {
            match layout {
                JumpArgsLayout::CarriersOnly => {
                    if jump_args_len > exit_phi_bindings_len {
                        #[cfg(debug_assertions)]
                        {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug(&format!(
                                "[joinir/exit-line] jump_args has {} extra args (block {:?}), ignoring invariants",
                                jump_args_len - exit_phi_bindings_len,
                                block_id
                            ));
                        }
                    }
                    Ok(0)
                }
                JumpArgsLayout::ExprResultPlusCarriers => {
                    if jump_args_len >= exit_phi_bindings_len + 1 {
                        if jump_args_len > exit_phi_bindings_len + 1 {
                            #[cfg(debug_assertions)]
                            {
                                let ring0 = crate::runtime::get_global_ring0();
                                ring0.log.debug(&format!(
                                    "[joinir/exit-line] jump_args has {} extra args (block {:?}), ignoring invariants after expr_result",
                                    jump_args_len - (exit_phi_bindings_len + 1),
                                    block_id
                                ));
                            }
                        }
                        Ok(1)
                    } else {
                        let msg = format!(
                            "[joinir/exit-line] expr_result layout requires leading slot: carriers={} args={} in block {:?}",
                            exit_phi_bindings_len, jump_args_len, block_id
                        );
                        if strict_exit {
                            Err(msg)
                        } else {
                            if crate::config::env::is_joinir_debug() {
                                let ring0 = crate::runtime::get_global_ring0();
                                ring0.log.debug(&format!("[DEBUG-177] {}", msg));
                            }
                            Ok(0)
                        }
                    }
                }
            }
        }
    }

    /// Convenience method: Convert collected carrier values to BTreeMap
    ///
    /// This matches the format expected by instruction_rewriter.rs's carrier_inputs.
    #[cfg(test)]
    pub fn to_carrier_map(
        carrier_values: Vec<(String, (BasicBlockId, ValueId))>,
    ) -> BTreeMap<String, Vec<(BasicBlockId, ValueId)>> {
        let mut carrier_map: BTreeMap<String, Vec<(BasicBlockId, ValueId)>> = BTreeMap::new();
        for (carrier_name, (block_id, value_id)) in carrier_values {
            carrier_map
                .entry(carrier_name)
                .or_insert_with(Vec::new)
                .push((block_id, value_id));
        }
        carrier_map
    }
}

impl Default for ExitArgsCollectorBox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_binding(name: &str, role: CarrierRole) -> LoopExitBinding {
        use crate::mir::ValueId;
        LoopExitBinding {
            carrier_name: name.to_string(),
            join_exit_value: ValueId(0), // Dummy for tests
            host_slot: ValueId(0),       // Dummy for tests
            role,
        }
    }

    #[test]
    fn test_collect_direct_mapping() {
        let collector = ExitArgsCollectorBox::new();
        let bindings = vec![
            make_binding("sum", CarrierRole::LoopState),
            make_binding("count", CarrierRole::LoopState),
        ];
        let args = vec![ValueId(10), ValueId(20)];
        let result = collector.collect(
            &bindings,
            &args,
            BasicBlockId(1),
            true,
            JumpArgsLayout::CarriersOnly,
        );

        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.block_id, BasicBlockId(1));
        assert_eq!(res.expr_result_value, None); // offset = 0, no expr result
        assert_eq!(res.carrier_values.len(), 2);
        assert_eq!(res.carrier_values[0].0, "sum");
        assert_eq!(res.carrier_values[0].1, (BasicBlockId(1), ValueId(10)));
    }

    #[test]
    fn test_collect_legacy_layout() {
        let collector = ExitArgsCollectorBox::new();
        let bindings = vec![make_binding("sum", CarrierRole::LoopState)];
        let args = vec![ValueId(5), ValueId(10)]; // [expr_result, carrier]
        let result = collector.collect(
            &bindings,
            &args,
            BasicBlockId(1),
            true,
            JumpArgsLayout::ExprResultPlusCarriers,
        );

        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.expr_result_value, Some(ValueId(5))); // offset = 1
        assert_eq!(res.carrier_values.len(), 1);
        assert_eq!(res.carrier_values[0].1, (BasicBlockId(1), ValueId(10)));
    }

    #[test]
    fn test_collect_skip_condition_only() {
        let collector = ExitArgsCollectorBox::new();
        let bindings = vec![
            make_binding("sum", CarrierRole::LoopState),
            make_binding("is_digit", CarrierRole::ConditionOnly), // Skip
        ];
        let args = vec![ValueId(10)]; // Only one arg for LoopState carrier
        let result = collector.collect(
            &bindings,
            &args,
            BasicBlockId(1),
            true,
            JumpArgsLayout::CarriersOnly,
        );

        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.carrier_values.len(), 1); // ConditionOnly skipped
    }

    #[test]
    fn test_collect_too_short_strict_fails() {
        let collector = ExitArgsCollectorBox::new();
        let bindings = vec![
            make_binding("sum", CarrierRole::LoopState),
            make_binding("count", CarrierRole::LoopState),
        ];
        let args = vec![ValueId(10)]; // Missing one carrier
        let result = collector.collect(
            &bindings,
            &args,
            BasicBlockId(1),
            true,
            JumpArgsLayout::CarriersOnly,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too short"));
    }

    #[test]
    fn test_collect_too_short_non_strict_warns() {
        let collector = ExitArgsCollectorBox::new();
        let bindings = vec![
            make_binding("sum", CarrierRole::LoopState),
            make_binding("count", CarrierRole::LoopState),
        ];
        let args = vec![ValueId(10)]; // Missing one carrier
        let result = collector.collect(
            &bindings,
            &args,
            BasicBlockId(1),
            false,
            JumpArgsLayout::CarriersOnly,
        );

        // Non-strict mode: succeeds with warning
        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.carrier_values.len(), 1); // Only one carrier collected
    }

    #[test]
    fn test_to_carrier_map() {
        let carrier_values = vec![
            ("sum".to_string(), (BasicBlockId(1), ValueId(10))),
            ("sum".to_string(), (BasicBlockId(2), ValueId(20))),
            ("count".to_string(), (BasicBlockId(1), ValueId(15))),
        ];
        let map = ExitArgsCollectorBox::to_carrier_map(carrier_values);

        assert_eq!(map.len(), 2);
        assert_eq!(map["sum"].len(), 2);
        assert_eq!(map["count"].len(), 1);
    }

    #[test]
    fn test_collect_extra_invariants_no_expr_result() {
        let collector = ExitArgsCollectorBox::new();
        let bindings = vec![
            make_binding("i", CarrierRole::LoopState),
            make_binding("start", CarrierRole::LoopState),
            make_binding("result", CarrierRole::LoopState),
        ];
        let args = vec![ValueId(1), ValueId(2), ValueId(3), ValueId(99)]; // extra invariant
        let result = collector.collect(
            &bindings,
            &args,
            BasicBlockId(1),
            true,
            JumpArgsLayout::CarriersOnly,
        );

        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.expr_result_value, None);
        assert_eq!(res.carrier_values.len(), 3);
        assert_eq!(res.carrier_values[0].1, (BasicBlockId(1), ValueId(1)));
        assert_eq!(res.carrier_values[1].1, (BasicBlockId(1), ValueId(2)));
        assert_eq!(res.carrier_values[2].1, (BasicBlockId(1), ValueId(3)));
    }
}
