//! ExprResultResolver: expr_result ValueId を解決する Box
//!
//! Phase 221-R: ExprResult 処理を箱化
//!
//! # 責任
//! JoinIR-local expr_result ValueId をホスト側 ValueId に変換する
//!
//! # Algorithm
//! 1. expr_result が存在するか確認
//! 2. exit_bindings で expr_result が carrier に対応しているか検査
//! 3. 対応する場合 → carrier_phis[carrier] (PHI dst) を返す
//! 4. 非対応の場合 → remapper.get_value(expr_result) を返す
//! 5. expr_result が None → None を返す

use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::join_ir::lowering::inline_boundary::LoopExitBinding;
use crate::mir::ValueId;
use std::collections::BTreeMap;

pub struct ExprResultResolver;

impl ExprResultResolver {
    /// expr_result を解決する
    ///
    /// # Arguments
    /// * `expr_result` - JoinIR-local expr_result ValueId (boundary.expr_result)
    /// * `exit_bindings` - Exit bindings from boundary
    /// * `carrier_phis` - Carrier name → PHI dst ValueId mapping
    /// * `remapper` - JoinIR ValueId → Host ValueId remapper
    /// * `debug` - Debug logging enabled
    ///
    /// # Returns
    /// * `Ok(Some(ValueId))` - Resolved host ValueId
    /// * `Ok(None)` - No expr_result to resolve
    /// * `Err(String)` - Resolution error
    ///
    /// # Phase 221 Logic
    /// This preserves the exact logic from merge/mod.rs L613-676:
    /// - If expr_result matches a carrier exit binding → use carrier PHI dst
    /// - Otherwise → use remapper to translate JoinIR ValueId to host ValueId
    pub fn resolve(
        expr_result: Option<ValueId>,
        exit_bindings: &[LoopExitBinding],
        carrier_phis: &BTreeMap<String, ValueId>,
        remapper: &JoinIrIdRemapper,
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();

        // Step 1: Check if expr_result exists
        let expr_result_id = match expr_result {
            Some(id) => id,
            None => {
                if debug {
                    trace.stderr_if(
                        "[cf_loop/joinir] Phase 221: expr_result is None, returning None",
                        true,
                    );
                }
                return Ok(None);
            }
        };

        if debug {
            trace.stderr_if(
                &format!(
                    "[cf_loop/joinir] Phase 221: Resolving expr_result {:?}, exit_bindings={:?}",
                    expr_result_id,
                    exit_bindings
                        .iter()
                        .map(|b| (b.carrier_name.as_str(), b.join_exit_value))
                        .collect::<Vec<_>>()
                ),
                true,
            );
        }

        // Step 2: Check if expr_result corresponds to a carrier exit binding
        // If so, use the carrier PHI dst instead of remapped value
        for binding in exit_bindings {
            if binding.join_exit_value == expr_result_id {
                // expr_result is a carrier! Use the carrier PHI dst
                if let Some(&carrier_phi_dst) = carrier_phis.get(&binding.carrier_name) {
                    if debug {
                        trace.stderr_if(
                            &format!(
                                "[cf_loop/joinir] Phase 221: expr_result {:?} is carrier '{}', returning PHI dst {:?}",
                                expr_result_id, binding.carrier_name, carrier_phi_dst
                            ),
                            true,
                        );
                    }
                    return Ok(Some(carrier_phi_dst));
                } else {
                    return Err(format!(
                        "[cf_loop/joinir] Phase 221: Carrier '{}' not found in carrier_phis",
                        binding.carrier_name
                    ));
                }
            }
        }

        // Step 3: expr_result is NOT a carrier - use remapped value
        if let Some(remapped_expr) = remapper.get_value(expr_result_id) {
            if debug {
                trace.stderr_if(
                    &format!(
                        "[cf_loop/joinir] Phase 221: Returning non-carrier expr_result: JoinIR {:?} → Host {:?}",
                        expr_result_id, remapped_expr
                    ),
                    true,
                );
            }
            Ok(Some(remapped_expr))
        } else {
            // expr_result was not remapped - this is an error
            Err(format!(
                "[cf_loop/joinir] Phase 221: expr_result {:?} was not found in remapper",
                expr_result_id
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_result_none() {
        let carrier_phis = BTreeMap::new();
        let remapper = JoinIrIdRemapper::new();

        let result = ExprResultResolver::resolve(None, &[], &carrier_phis, &remapper, false);

        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_expr_result_carrier() {
        let mut carrier_phis = BTreeMap::new();
        carrier_phis.insert("sum".to_string(), ValueId(100));

        let binding = LoopExitBinding {
            carrier_name: "sum".to_string(),
            join_exit_value: ValueId(18),
            host_slot: ValueId(5),
            role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
        };

        let remapper = JoinIrIdRemapper::new();

        let result = ExprResultResolver::resolve(
            Some(ValueId(18)),
            &[binding],
            &carrier_phis,
            &remapper,
            false,
        );

        assert_eq!(result.unwrap(), Some(ValueId(100)));
    }

    #[test]
    fn test_expr_result_non_carrier() {
        let carrier_phis = BTreeMap::new();
        let mut remapper = JoinIrIdRemapper::new();
        remapper.set_value(ValueId(42), ValueId(200));

        let result =
            ExprResultResolver::resolve(Some(ValueId(42)), &[], &carrier_phis, &remapper, false);

        assert_eq!(result.unwrap(), Some(ValueId(200)));
    }

    #[test]
    fn test_expr_result_not_found() {
        let carrier_phis = BTreeMap::new();
        let remapper = JoinIrIdRemapper::new();

        let result =
            ExprResultResolver::resolve(Some(ValueId(999)), &[], &carrier_phis, &remapper, false);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found in remapper"));
    }
}
