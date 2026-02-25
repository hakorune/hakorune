//! Phase 188-Impl-3: JoinInlineBoundary Copy Instruction Injector
//!
//! 責務:
//! - JoinInlineBoundary で指定された入出力の Copy instruction 生成
//! - Entry block への Copy instruction 挿入
//! - SSA 値空間の接続

use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap; // Phase 222.5-E: HashMap → BTreeMap for determinism

pub struct BoundaryInjector;

impl BoundaryInjector {
    /// JoinInlineBoundary で指定された入力を entry block に Copy instruction として挿入
    ///
    /// # Arguments
    ///
    /// * `func` - 対象の MirFunction
    /// * `entry_block_id` - entry block の ID
    /// * `boundary` - JoinInlineBoundary（入力マッピング情報）
    /// * `value_map` - ValueId リマッピング情報
    /// * `debug` - デバッグログ出力
    ///
    /// # Returns
    ///
    /// * `Ok(())` - 成功
    /// * `Err(String)` - エラー（ブロックが見つからない等）
    ///
    /// # Phase 33-16: Loop Variable Header PHI Support
    ///
    /// When `boundary.loop_var_name` is set, the first join_input (loop variable)
    /// is handled by the header PHI instead of a Copy instruction. We skip
    /// emitting the Copy for join_inputs[0] in this case to avoid overwriting
    /// the PHI result with the initial value.
    ///
    /// # Phase 33-20: All Carriers Header PHI Support
    ///
    /// When `boundary.loop_var_name` is set, ALL carriers (loop var + carrier_info carriers)
    /// are handled by header PHIs. We skip ALL join_inputs
    /// Copy instructions to avoid overwriting the PHI results.
    ///
    /// # Phase 177-3: PHI Collision Avoidance (Option B)
    ///
    /// When `phi_dst_ids` contains ValueIds of existing PHI dsts in the header block,
    /// we allocate a NEW ValueId for condition_bindings instead of skipping the copy.
    /// This ensures the condition variable is available even when its remapped ValueId
    /// collides with a PHI dst.
    ///
    /// # Returns
    ///
    /// Returns a BTreeMap mapping original ValueIds to their reallocated ValueIds
    /// (only for condition_bindings that had collisions).
    /// Phase 222.5-E: HashMap → BTreeMap for determinism
    pub fn inject_boundary_copies(
        func: &mut MirFunction,
        entry_block_id: BasicBlockId,
        boundary: &JoinInlineBoundary,
        value_map: &BTreeMap<ValueId, ValueId>, // Phase 222.5-E: HashMap → BTreeMap for determinism
        phi_dst_ids: &std::collections::HashSet<ValueId>,
        debug: bool,
    ) -> Result<BTreeMap<ValueId, ValueId>, String> {
        // Phase 222.5-E: HashMap → BTreeMap for determinism
        // Phase 33-20: When loop_var_name is set, ALL join_inputs are handled by header PHIs
        // This includes the loop variable AND all other carriers from carrier_info.
        // We skip ALL join_inputs Copy instructions, only condition_bindings remain.
        let skip_all_join_inputs = boundary.loop_var_name.is_some();

        // Phase 171-fix: Check both join_inputs and condition_bindings
        let effective_join_inputs = if skip_all_join_inputs {
            0 // Phase 33-20: All join_inputs are handled by header PHIs
        } else {
            boundary.join_inputs.len()
        };
        let total_inputs = effective_join_inputs + boundary.condition_bindings.len();
        if total_inputs == 0 {
            // No inputs to process, return empty reallocations map
            // Phase 222.5-E: HashMap → BTreeMap for determinism
            return Ok(BTreeMap::new());
        }

        if debug {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[BoundaryInjector] Phase 33-20: Injecting {} Copy instructions ({} join_inputs, {} condition_bindings) at entry block {:?}{}",
                total_inputs,
                effective_join_inputs,
                boundary.condition_bindings.len(),
                entry_block_id,
                if skip_all_join_inputs { " (skipping ALL join_inputs - handled by header PHIs)" } else { "" }
            ));
        }

        // Phase 177-3: Use PHI dst IDs passed from caller
        if debug && !phi_dst_ids.is_empty() {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[BoundaryInjector] Phase 177-3: Received {} PHI dst IDs to avoid: {:?}",
                phi_dst_ids.len(),
                phi_dst_ids
            ));
        }

        // Phase 177-3 Option B: First pass - allocate all new ValueIds for PHI collisions
        // We need to do this BEFORE acquiring the entry_block reference to avoid borrow conflicts
        // Phase 222.5-E: HashMap → BTreeMap for determinism
        let mut reallocations = BTreeMap::new();

        for binding in &boundary.condition_bindings {
            let remapped_join = value_map
                .get(&binding.join_value)
                .copied()
                .unwrap_or(binding.join_value);

            if phi_dst_ids.contains(&remapped_join) {
                // Collision detected! Allocate a fresh ValueId
                let fresh_dst = func.next_value_id();

                if debug {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[BoundaryInjector] Phase 177-3 Option B: PHI collision for condition binding '{}': {:?} → reallocated to {:?}",
                        binding.name, remapped_join, fresh_dst
                    ));
                }

                reallocations.insert(binding.join_value, fresh_dst);
            }
        }

        // Now get entry block reference (after all ValueId allocations are done)
        let entry_block = func
            .get_block_mut(entry_block_id)
            .ok_or(format!("Entry block {:?} not found", entry_block_id))?;

        // Copy instructions を生成して挿入
        let mut copy_instructions = Vec::new();

        // Phase 171: Inject Copy instructions for join_inputs (loop parameters)
        // Phase 33-20: Skip ALL join_inputs when loop_var_name is set (header PHIs handle them)
        if !skip_all_join_inputs {
            for (join_input, host_input) in
                boundary.join_inputs.iter().zip(boundary.host_inputs.iter())
            {
                // リマップ後の ValueId を取得
                let remapped_join = value_map.get(join_input).copied().unwrap_or(*join_input);
                let remapped_host = *host_input; // host_input is already in host space

                // Copy instruction: remapped_join = Copy remapped_host
                let copy_inst = MirInstruction::Copy {
                    dst: remapped_join,
                    src: remapped_host,
                };

                copy_instructions.push(copy_inst);

                if debug {
                    crate::mir::builder::control_flow::joinir::trace::trace().stderr_if(
                        &format!(
                            "[BoundaryInjector]   Join input: Copy {:?} = Copy {:?}",
                            remapped_join, remapped_host
                        ),
                        true,
                    );
                }
            }
        }

        // Phase 177-3 DEBUG: Check value_map in BoundaryInjector
        if debug {
            crate::mir::builder::control_flow::joinir::trace::trace().stderr_if(
                "[DEBUG-177] === BoundaryInjector value_map ===",
                true,
            );
            for binding in &boundary.condition_bindings {
                let lookup = value_map.get(&binding.join_value);
                crate::mir::builder::control_flow::joinir::trace::trace().stderr_if(
                    &format!(
                        "[DEBUG-177]   '{}': JoinIR {:?} → {:?}",
                        binding.name, binding.join_value, lookup
                    ),
                    true,
                );
            }
        }

        // Phase 171-fix: Inject Copy instructions for condition_bindings (condition-only variables)
        // These variables are read-only and used ONLY in the loop condition.
        // Each binding explicitly specifies HOST ValueId → JoinIR ValueId mapping.
        // We inject Copy: remapped_join_value = Copy host_value
        //
        // Phase 177-3 Option B: Use pre-allocated reallocations for PHI collision cases
        let mut seen_dst_to_src: BTreeMap<ValueId, ValueId> = BTreeMap::new();
        for binding in &boundary.condition_bindings {
            // Look up the remapped JoinIR ValueId from value_map
            let remapped_join = value_map
                .get(&binding.join_value)
                .copied()
                .unwrap_or(binding.join_value);

            // Phase 177-3 Option B: Check if this binding was reallocated (PHI collision case)
            let final_dst = reallocations
                .get(&binding.join_value)
                .copied()
                .unwrap_or(remapped_join);

            // Phase 135-P0: Deduplicate condition bindings that alias the same JoinIR ValueId.
            //
            // Promoters may produce multiple names that map to the same JoinIR ValueId
            // (e.g. 'char' and 'is_char_match' sharing a promoted carrier slot).
            // Emitting multiple `Copy` instructions to the same `dst` violates MIR SSA and
            // fails `--verify` (ValueId defined multiple times).
            if let Some(prev_src) = seen_dst_to_src.get(&final_dst).copied() {
                if prev_src != binding.host_value {
                    return Err(format!(
                        "[BoundaryInjector] condition_bindings conflict: dst {:?} would be assigned from two different sources: {:?} and {:?} (JoinIR {:?}, name '{}')",
                        final_dst, prev_src, binding.host_value, binding.join_value, binding.name
                    ));
                }
                continue;
            }
            seen_dst_to_src.insert(final_dst, binding.host_value);

            // Copy instruction: final_dst = Copy host_value
            let copy_inst = MirInstruction::Copy {
                dst: final_dst,
                src: binding.host_value,
            };

            copy_instructions.push(copy_inst);

            if debug {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[BoundaryInjector]   Condition binding '{}': Copy {:?} = Copy {:?} (JoinIR {:?} → remapped {:?}{})",
                    binding.name, final_dst, binding.host_value, binding.join_value, remapped_join,
                    if final_dst != remapped_join { format!(" → reallocated {:?}", final_dst) } else { String::new() }
                ));
            }
        }

        // Entry block の先頭に Copy instructions を挿入
        // Reverse order to preserve original order when inserting at position 0
        // Phase 189 FIX: Also insert corresponding spans
        let default_span = entry_block
            .instruction_spans
            .first()
            .copied()
            .unwrap_or_else(crate::ast::Span::unknown);
        for inst in copy_instructions.into_iter().rev() {
            entry_block.instructions.insert(0, inst);
            entry_block.instruction_spans.insert(0, default_span);
        }

        // Return reallocations map for condition_bindings that had PHI collisions
        Ok(reallocations)
    }
}

// TODO: These tests need to be updated to use the new MirModule API
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::mir::{BasicBlock, MirModule};
//
//     #[test]
//     fn test_injector_empty_boundary() {
//         // 空の boundary で何もしない
//         let boundary = JoinInlineBoundary::new_inputs_only(vec![], vec![]);
//         let mut module = MirModule::new();
//         let mut func = module.define_function("test".to_string(), vec![]);
//         let entry_block = func.create_block();
//         let value_map = HashMap::new();
//
//         let result = BoundaryInjector::inject_boundary_copies(
//             &mut func,
//             entry_block,
//             &boundary,
//             &value_map,
//             false,
//         );
//
//         assert!(result.is_ok());
//     }
//
//     #[test]
//     fn test_injector_single_copy() {
//         // 単一の Copy instruction を挿入
//         let boundary = JoinInlineBoundary::new_inputs_only(
//             vec![ValueId(0)],
//             vec![ValueId(10)],
//         );
//
//         let mut module = MirModule::new();
//         let mut func = module.define_function("test".to_string(), vec![]);
//         let entry_block = func.create_block();
//
//         let mut value_map = HashMap::new();
//         value_map.insert(ValueId(0), ValueId(100)); // JoinIR ValueId(0) remapped to ValueId(100)
//
//         let result = BoundaryInjector::inject_boundary_copies(
//             &mut func,
//             entry_block,
//             &boundary,
//             &value_map,
//             false,
//         );
//
//         assert!(result.is_ok());
//
//         // Copy instruction が挿入されたことを確認
//         let block = func.get_block(entry_block).unwrap();
//         assert!(!block.instructions.is_empty());
//
//         // First instruction should be Copy
//         match &block.instructions[0] {
//             MirInstruction::Copy { dst, src } => {
//                 assert_eq!(*dst, ValueId(100)); // Remapped join input
//                 assert_eq!(*src, ValueId(10));  // Host input
//             }
//             _ => panic!("Expected Copy instruction"),
//         }
//     }
//
//     #[test]
//     fn test_injector_multiple_copies() {
//         // 複数の Copy instruction を挿入
//         let boundary = JoinInlineBoundary::new_inputs_only(
//             vec![ValueId(0), ValueId(1)],
//             vec![ValueId(10), ValueId(20)],
//         );
//
//         let mut module = MirModule::new();
//         let mut func = module.define_function("test".to_string(), vec![]);
//         let entry_block = func.create_block();
//
//         let mut value_map = HashMap::new();
//         value_map.insert(ValueId(0), ValueId(100));
//         value_map.insert(ValueId(1), ValueId(101));
//
//         let result = BoundaryInjector::inject_boundary_copies(
//             &mut func,
//             entry_block,
//             &boundary,
//             &value_map,
//             false,
//         );
//
//         assert!(result.is_ok());
//
//         let block = func.get_block(entry_block).unwrap();
//         assert_eq!(block.instructions.len(), 2);
//
//         // Check both copy instructions
//         match &block.instructions[0] {
//             MirInstruction::Copy { dst, src } => {
//                 assert_eq!(*dst, ValueId(100));
//                 assert_eq!(*src, ValueId(10));
//             }
//             _ => panic!("Expected Copy instruction"),
//         }
//
//         match &block.instructions[1] {
//             MirInstruction::Copy { dst, src } => {
//                 assert_eq!(*dst, ValueId(101));
//                 assert_eq!(*src, ValueId(20));
//             }
//             _ => panic!("Expected Copy instruction"),
//         }
//     }
// }
