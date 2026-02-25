//! JoinIR Debug Assertions (Phase 286C-4.3)
//!
//! Debug-only verification functions that panic on contract violations.
//! These are excluded from release builds.
//!
//! # Split from contract_checks.rs
//!
//! This file was extracted from contract_checks.rs to separate debug-only
//! panic-based assertions from production Fail-Fast contract checks.
//!
//! All functions here are `#[cfg(debug_assertions)]` and panic on violations.

#[cfg(debug_assertions)]
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
#[cfg(debug_assertions)]
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
#[cfg(debug_assertions)]
use std::collections::HashMap;

#[cfg(debug_assertions)]
use super::LoopHeaderPhiInfo;
#[cfg(debug_assertions)]
use crate::mir::join_ir::lowering::join_value_space::{LOCAL_MAX, PARAM_MAX, PARAM_MIN};

/// Verify loop header PHIs match boundary expectations (debug assertion)
///
/// # Contract
///
/// - If boundary has loop_var_name, header block must have corresponding PHI
/// - Each carrier_phi entry must have a PHI instruction in header block
///
/// # Panics
///
/// Panics if PHI structure doesn't match boundary expectations.
#[cfg(debug_assertions)]
pub(super) fn verify_loop_header_phis(
    func: &MirFunction,
    header_block: BasicBlockId,
    loop_info: &LoopHeaderPhiInfo,
    boundary: &JoinInlineBoundary,
) {
    if let Some(ref loop_var_name) = boundary.loop_var_name {
        let header_block_data = func.blocks.get(&header_block).unwrap_or_else(|| {
            panic!(
                "[JoinIRVerifier] Header block {} not found ({} blocks in func)",
                header_block,
                func.blocks.len()
            )
        });
        let has_loop_var_phi = header_block_data
            .instructions
            .iter()
            .any(|instr| matches!(instr, MirInstruction::Phi { .. }));

        if !has_loop_var_phi && !loop_info.carrier_phis.is_empty() {
            panic!(
                "[JoinIRVerifier] Loop variable '{}' in boundary but no PHI in header block {} (has {} carrier PHIs)",
                loop_var_name, header_block.0, loop_info.carrier_phis.len()
            );
        }
    }

    if !loop_info.carrier_phis.is_empty() {
        let header_block_data = func.blocks.get(&header_block).unwrap_or_else(|| {
            panic!(
                "[JoinIRVerifier] Header block {} not found ({} blocks in func)",
                header_block,
                func.blocks.len()
            )
        });
        let phi_count = header_block_data
            .instructions
            .iter()
            .filter(|instr| matches!(instr, MirInstruction::Phi { .. }))
            .count();

        if phi_count == 0 {
            panic!(
                "[JoinIRVerifier] LoopHeaderPhiInfo has {} PHIs but header block {} has none",
                loop_info.carrier_phis.len(),
                header_block.0
            );
        }

        for (carrier_name, entry) in &loop_info.carrier_phis {
            let phi_exists = header_block_data.instructions.iter().any(|instr| {
                if let MirInstruction::Phi { dst, .. } = instr {
                    *dst == entry.phi_dst
                } else {
                    false
                }
            });

            if !phi_exists {
                panic!(
                    "[JoinIRVerifier] Carrier '{}' has PHI dst {:?} but PHI not found in header block {}",
                    carrier_name, entry.phi_dst, header_block.0
                );
            }
        }
    }
}

/// Verify exit line structure (debug assertion)
///
/// # Contract
///
/// - Exit block must exist in function
/// - Exit binding host_slots must be reasonable (< 1_000_000)
/// - Exit PHI ValueIds must not collide with other instructions
///
/// # Panics
///
/// Panics if exit line structure is invalid.
#[cfg(debug_assertions)]
pub(super) fn verify_exit_line(
    func: &MirFunction,
    exit_block: BasicBlockId,
    boundary: &JoinInlineBoundary,
) {
    if !func.blocks.contains_key(&exit_block) {
        panic!(
            "[JoinIRVerifier] Exit block {} out of range (func has {} blocks)",
            exit_block.0,
            func.blocks.len()
        );
    }

    if !boundary.exit_bindings.is_empty() {
        for binding in &boundary.exit_bindings {
            if binding.host_slot.0 >= 1_000_000 {
                panic!(
                    "[JoinIRVerifier] Exit binding '{}' has suspiciously large host_slot {:?}",
                    binding.carrier_name, binding.host_slot
                );
            }
        }
    }

    // Phase 132-P2: Verify exit PHI ValueIds don't collide with other instructions
    verify_exit_phi_no_collision(func, exit_block);
}

/// Phase 132-P2: Verify exit PHI dst ValueIds don't collide with other instructions
///
/// # Problem
///
/// If exit_phi_builder uses builder.value_gen.next() (module-level) instead of
/// func.next_value_id() (function-level), it can allocate ValueIds that collide
/// with existing instructions in the function.
///
/// Example collision:
/// - bb0: %1 = const 0   (counter init)
/// - bb3: %1 = phi ...   (exit PHI - collision!)
///
/// This causes LLVM backend errors:
/// "Cannot overwrite PHI dst=1. ValueId namespace collision detected."
///
/// # Contract
///
/// All exit PHI dst ValueIds must be unique within the function and not
/// overwrite any existing instruction dst.
///
/// # Panics
///
/// Panics if any exit PHI dst collides with an existing instruction dst.
#[cfg(debug_assertions)]
fn verify_exit_phi_no_collision(func: &MirFunction, exit_block: BasicBlockId) {
    let exit_block_data = match func.blocks.get(&exit_block) {
        Some(block) => block,
        None => return, // Block not found, other verification will catch this
    };

    // Collect all exit PHI dsts
    let mut exit_phi_dsts = std::collections::HashSet::new();
    for instr in &exit_block_data.instructions {
        if let MirInstruction::Phi { dst, .. } = instr {
            exit_phi_dsts.insert(*dst);
        }
    }

    if exit_phi_dsts.is_empty() {
        return; // No exit PHIs, nothing to verify
    }

    // Collect all instruction dsts in the entire function (excluding PHIs)
    let mut all_non_phi_dsts = std::collections::HashSet::new();
    for (block_id, block) in &func.blocks {
        if *block_id == exit_block {
            // For exit block, only check non-PHI instructions
            for instr in &block.instructions {
                if !matches!(instr, MirInstruction::Phi { .. }) {
                    if let Some(dst) = get_instruction_dst(instr) {
                        all_non_phi_dsts.insert(dst);
                    }
                }
            }
        } else {
            // For other blocks, check all instructions
            for instr in &block.instructions {
                if let Some(dst) = get_instruction_dst(instr) {
                    all_non_phi_dsts.insert(dst);
                }
            }
        }
    }

    // Check for collisions
    for phi_dst in &exit_phi_dsts {
        if all_non_phi_dsts.contains(phi_dst) {
            // Find which instruction collides
            for (block_id, block) in &func.blocks {
                for instr in &block.instructions {
                    if matches!(instr, MirInstruction::Phi { .. }) && *block_id == exit_block {
                        continue; // Skip exit PHIs themselves
                    }
                    if let Some(dst) = get_instruction_dst(instr) {
                        if dst == *phi_dst {
                            panic!(
                                "[JoinIRVerifier/Phase132-P2] Exit PHI dst {:?} collides with instruction in block {}: {:?}\n\
                                 This indicates exit_phi_builder used module-level value_gen.next() instead of function-level next_value_id().\n\
                                 Fix: Use func.next_value_id() in exit_phi_builder.rs",
                                phi_dst, block_id.0, instr
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Helper: Extract dst ValueId from MirInstruction
#[cfg(debug_assertions)]
fn get_instruction_dst(instr: &MirInstruction) -> Option<ValueId> {
    use MirInstruction;
    match instr {
        MirInstruction::Const { dst, .. }
        | MirInstruction::Load { dst, .. }
        | MirInstruction::UnaryOp { dst, .. }
        | MirInstruction::BinOp { dst, .. }
        | MirInstruction::Compare { dst, .. }
        | MirInstruction::TypeOp { dst, .. }
        | MirInstruction::NewBox { dst, .. }
        | MirInstruction::NewClosure { dst, .. }
        | MirInstruction::Copy { dst, .. }
        | MirInstruction::Phi { dst, .. }
        | MirInstruction::RefNew { dst, .. }
        | MirInstruction::WeakRef { dst, .. }
        | MirInstruction::FutureNew { dst, .. }
        | MirInstruction::Await { dst, .. } => Some(*dst),
        MirInstruction::Call { dst, .. } => *dst,
        _ => None,
    }
}

/// Verify ValueId regions are correct (debug assertion)
///
/// # Contract
///
/// - join_inputs must be in Param region (100-999)
/// - carrier PHI dsts must be in Local region (<= LOCAL_MAX)
/// - condition_bindings join_values must be in Param region
/// - exit_bindings join_exit_values must be in Param region
///
/// # Panics
///
/// Panics if any ValueId is outside its expected region.
#[cfg(debug_assertions)]
pub(super) fn verify_valueid_regions(loop_info: &LoopHeaderPhiInfo, boundary: &JoinInlineBoundary) {
    fn region_name(id: ValueId) -> &'static str {
        if id.0 < PARAM_MIN {
            "PHI Reserved"
        } else if id.0 <= PARAM_MAX {
            "Param"
        } else if id.0 <= LOCAL_MAX {
            "Local"
        } else {
            "Invalid (> LOCAL_MAX)"
        }
    }

    for join_id in &boundary.join_inputs {
        if !(PARAM_MIN..=PARAM_MAX).contains(&join_id.0) {
            panic!(
                "[JoinIRVerifier] join_input {:?} not in Param region ({})",
                join_id,
                region_name(*join_id)
            );
        }
    }

    for (_, entry) in &loop_info.carrier_phis {
        if entry.phi_dst.0 > LOCAL_MAX {
            panic!(
                "[JoinIRVerifier] Carrier PHI dst {:?} outside Local region ({})",
                entry.phi_dst,
                region_name(entry.phi_dst)
            );
        }
    }

    for binding in &boundary.condition_bindings {
        if !(PARAM_MIN..=PARAM_MAX).contains(&binding.join_value.0) {
            panic!(
                "[JoinIRVerifier] Condition binding '{}' join_value {:?} not in Param region ({})",
                binding.name,
                binding.join_value,
                region_name(binding.join_value)
            );
        }
    }

    for binding in &boundary.exit_bindings {
        if !(PARAM_MIN..=PARAM_MAX).contains(&binding.join_exit_value.0) {
            panic!(
                "[JoinIRVerifier] Exit binding '{}' join_exit_value {:?} not in Param region ({})",
                binding.carrier_name,
                binding.join_exit_value,
                region_name(binding.join_exit_value)
            );
        }
    }
}

/// Phase 135 P1 Step 1: Verify condition_bindings consistency (alias allowed, conflict fails)
///
/// # Contract
///
/// condition_bindings can have multiple names (aliases) pointing to the same join_value,
/// but if the same join_value appears with different host_value, it's a contract violation.
///
/// This catches merge-time inconsistencies before BoundaryInjector tries to inject Copy
/// instructions, preventing MIR SSA breakage.
///
/// # Example Valid (alias):
/// ```text
/// condition_bindings: [
///   { name: "is_char_match", join_value: ValueId(104), host_value: ValueId(12) },
///   { name: "char", join_value: ValueId(104), host_value: ValueId(12) }  // Same host_value - OK
/// ]
/// ```
///
/// # Example Invalid (conflict):
/// ```text
/// condition_bindings: [
///   { name: "is_char_match", join_value: ValueId(104), host_value: ValueId(12) },
///   { name: "char", join_value: ValueId(104), host_value: ValueId(18) }  // Different host_value - FAIL
/// ]
/// ```
///
/// # Panics
///
/// Panics if the same join_value has conflicting host_value mappings.
#[cfg(debug_assertions)]
pub(super) fn verify_condition_bindings_consistent(boundary: &JoinInlineBoundary) {
    let mut join_to_host: HashMap<ValueId, ValueId> = HashMap::new();

    for binding in &boundary.condition_bindings {
        if let Some(&existing_host) = join_to_host.get(&binding.join_value) {
            if existing_host != binding.host_value {
                panic!(
                    "[JoinIRVerifier/Phase135-P1] condition_bindings conflict: join_value {:?} mapped to both {:?} and {:?}\n\
                     Binding names with conflict: check all bindings with join_value={:?}\n\
                     Contract: Same join_value can have multiple names (alias) but must have same host_value.\n\
                     Fix: Ensure ConditionLoweringBox uses SSOT allocator (ConditionContext.alloc_value).",
                    binding.join_value, existing_host, binding.host_value, binding.join_value
                );
            }
        } else {
            join_to_host.insert(binding.join_value, binding.host_value);
        }
    }
}

/// Phase 135 P1 Step 2: Verify header PHI dsts are not redefined by non-PHI instructions
///
/// # Contract
///
/// Loop header PHI dst ValueIds must not be reused as dst in non-PHI instructions.
/// This prevents "PHI dst overwrite" where a Copy/BinOp/etc. instruction redefines
/// the PHI result, breaking MIR SSA.
///
/// # Example Invalid:
/// ```text
/// bb3 (header):
///   %14 = phi [%2, bb1], [%28, bb8]  // Header PHI
///   %16 = copy %0
///   %14 = call %16.length()          // INVALID: Redefines PHI dst %14
/// ```
///
/// This typically happens when:
/// - ConditionLoweringBox bypasses SSOT allocator and reuses PHI dst ValueIds
/// - JoinIR merge incorrectly remaps values to PHI dst range
///
/// # Panics
///
/// Panics if any header PHI dst is redefined by a non-PHI instruction in the function.
#[cfg(debug_assertions)]
pub(super) fn verify_header_phi_dsts_not_redefined(
    func: &MirFunction,
    header_block: BasicBlockId,
    phi_dsts: &std::collections::HashSet<ValueId>,
) {
    if phi_dsts.is_empty() {
        return; // No PHI dsts to protect
    }

    // Check all blocks for non-PHI instructions that redefine PHI dsts
    for (block_id, block) in &func.blocks {
        for instr in &block.instructions {
            // Skip PHIs in header block (they're the definitions we're protecting)
            if *block_id == header_block && matches!(instr, MirInstruction::Phi { .. }) {
                continue;
            }

            // Check if this instruction redefines a PHI dst
            if let Some(dst) = get_instruction_dst(instr) {
                if phi_dsts.contains(&dst) {
                    panic!(
                        "[JoinIRVerifier/Phase135-P1] Header PHI dst {:?} redefined by non-PHI instruction in block {}:\n\
                         Instruction: {:?}\n\
                         Contract: Header PHI dsts must not be reused as dst in other instructions.\n\
                         Fix: Ensure ConditionLoweringBox uses SSOT allocator (ConditionContext.alloc_value) to avoid ValueId collisions.",
                        dst, block_id.0, instr
                    );
                }
            }
        }
    }
}

/// Phase 204-2: Verify PHI dst ValueIds are not overwritten by subsequent instructions in header block
///
/// # Contract
///
/// PHI instructions must appear first in a basic block, and their dst ValueIds
/// must not be overwritten by any subsequent non-PHI instructions in the same block.
///
/// # Panics
///
/// Panics if:
/// - PHI instruction appears after non-PHI instructions
/// - Non-PHI instruction overwrites a PHI dst in the header block
#[cfg(debug_assertions)]
pub(super) fn verify_no_phi_dst_overwrite(
    func: &MirFunction,
    header_block: BasicBlockId,
    loop_info: &LoopHeaderPhiInfo,
) {
    if loop_info.carrier_phis.is_empty() {
        return; // No PHIs to verify
    }

    let header_block_data = func.blocks.get(&header_block).unwrap_or_else(|| {
        panic!(
            "[JoinIRVerifier] Header block {} not found ({} blocks in func)",
            header_block,
            func.blocks.len()
        )
    });

    // 1. Collect all PHI dsts
    let phi_dsts: std::collections::HashSet<ValueId> = loop_info
        .carrier_phis
        .values()
        .map(|entry| entry.phi_dst)
        .collect();

    // 2. Check instructions after PHI definitions
    let mut after_phis = false;
    for instr in &header_block_data.instructions {
        match instr {
            MirInstruction::Phi { dst, .. } => {
                // PHI instructions come first in basic block
                if after_phis {
                    panic!(
                        "[JoinIRVerifier] PHI instruction {:?} appears after non-PHI instructions in block {}",
                        dst, header_block.0
                    );
                }
            }
            _ => {
                after_phis = true;
                // Check if this instruction writes to a PHI dst
                if let Some(dst) = get_instruction_dst(instr) {
                    if phi_dsts.contains(&dst) {
                        panic!(
                            "[JoinIRVerifier/Phase204] PHI dst {:?} is overwritten by instruction in header block {}: {:?}",
                            dst, header_block.0, instr
                        );
                    }
                }
            }
        }
    }
}

/// Verify PHI inputs are defined (Phase 204-3 - Conservative sanity checks)
///
/// # Checks
///
/// 1. PHI inputs have reasonable ValueId values (< threshold)
/// 2. No obviously undefined values (e.g., suspiciously large IDs)
///
/// # Note
///
/// Full data-flow analysis (DFA) verification is deferred to Phase 205+.
/// This function only performs conservative sanity checks.
///
/// # Panics
///
/// Panics in debug mode if suspicious PHI inputs are detected.
#[cfg(debug_assertions)]
pub(super) fn verify_phi_inputs_defined(func: &MirFunction, header_block: BasicBlockId) {
    let header_block_data = func.blocks.get(&header_block).unwrap_or_else(|| {
        panic!(
            "[JoinIRVerifier] Header block {} not found ({} blocks in func)",
            header_block,
            func.blocks.len()
        )
    });

    for instr in &header_block_data.instructions {
        if let MirInstruction::Phi {
            dst,
            inputs,
            type_hint: _,
        } = instr
        {
            for (value_id, pred_block) in inputs {
                // Conservative sanity check: ValueId should not be suspiciously large
                // Phase 201 JoinValueSpace uses regions:
                // - PHI Reserved: 0-99
                // - Param: 100-999
                // - Local: 1000+
                // - Reasonable max: 100000 (arbitrary but catches obvious bugs)
                if value_id.0 >= 100000 {
                    panic!(
                        "[JoinIRVerifier/Phase204-3] PHI {:?} has suspiciously large input {:?} from predecessor block {:?}",
                        dst, value_id, pred_block
                    );
                }
            }
        }
    }
}

/// Verify all loop contracts for a merged JoinIR function
///
/// This is the main entry point for verification. It runs all checks
/// and panics if any contract violation is found.
///
/// # Panics
///
/// Panics in debug mode if any contract violation is detected.
#[cfg(debug_assertions)]
pub(super) fn verify_joinir_contracts(
    func: &MirFunction,
    header_block: BasicBlockId,
    exit_block: BasicBlockId,
    loop_info: &LoopHeaderPhiInfo,
    boundary: &JoinInlineBoundary,
) {
    // Phase 135 P1 Step 1: Verify condition_bindings consistency (before merge)
    verify_condition_bindings_consistent(boundary);

    verify_loop_header_phis(func, header_block, loop_info, boundary);
    verify_no_phi_dst_overwrite(func, header_block, loop_info); // Phase 204-2
    verify_phi_inputs_defined(func, header_block); // Phase 204-3
    verify_exit_line(func, exit_block, boundary);
    verify_valueid_regions(loop_info, boundary); // Phase 205-4

    // Phase 135 P1 Step 2: Verify header PHI dsts not redefined (after merge)
    let phi_dsts: std::collections::HashSet<_> = loop_info
        .carrier_phis
        .values()
        .map(|entry| entry.phi_dst)
        .collect();
    verify_header_phi_dsts_not_redefined(func, header_block, &phi_dsts);
}
